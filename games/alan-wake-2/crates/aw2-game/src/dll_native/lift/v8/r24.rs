#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008b9094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b9094); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b9098); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b9099); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b909a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008b909f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b90a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b90ad); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b90bc);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b90bd); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b90c3); };
            c.w64(1, t5);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b90ca); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2134, 0x70008b90d2, 0x70008b90cd);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b90d2); };
            c.w64(6, t6);
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b90d6); };
            c.w32(1, t7);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008b90e5);
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b90e6); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            return c.call(0x70008ba474, 0x70008b90f7, 0x70008b90f2);
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b90f7); };
            c.w64(6, t10);
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b90fb); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b9101); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9107); };
            c.w32(8, t15);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008b9111); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b9115); }
            c.cmp(c.r64(8), 0x79u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008b911f);
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b9120); };
            c.w32(8, t16);
            c.w32(9, 0x8u64);
            c.w64(11, c.r64(9));
            let r17 = c.add(c.r64(11), 0xcu64, 64);
            c.w64(11, r17);
            if c.cc_o() { blk = 67; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b913a); };
            c.w64(12, t18);
            let Some(t19) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b9141); };
            c.w64(15, t19);
            c.w64(3, c.r64(15).wrapping_add(c.r64(11)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b9149); };
            c.w64(2, t20);
            let Some(t21) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b9150); };
            c.cmp(t21, c.r64(3), 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 70; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008b9166); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b916e); }
            c.w32(8, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b9178); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        14 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b9181); }
            let r22 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r22);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(9), 0x14u64, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9190); };
            c.w32(9, t23);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x70008b91a0);
        }
        18 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b91a1); };
            c.w64(15, t24);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008b91a8); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008b91b2);
        }
        20 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b91b3); };
            c.w64(15, t25);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b91ba); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x70008b91c4);
        }
        22 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b91c5); };
            c.w64(9, t26);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b91c9); }
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b91cd); };
            c.w32(9, t27);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008b91e0);
        }
        24 => {
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b91e1); };
            c.w32(9, t28);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x10e0)) else { return RegfileExit::fault(0x70008b91e8); };
            c.w64(15, t29);
            let Some(t30) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b91ef); };
            c.w32(3, t30);
            let r31 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r31);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(13).wrapping_add(0x63f0));
            let Some(t32) = c.ld64(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b9205); };
            c.w64(3, t32);
            blk = 45; continue;
        }
        26 => {
            let Some(t33) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b920e); };
            c.w32(7, t33);
            let r34 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r34);
            let Some(t35) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9214); };
            c.w32(7, t35);
            c.cmp(c.r32(7), 0x5cu64, 32);
            if c.cc_l() { blk = 33; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(7), 0x85u64, 32);
            if c.cc_l() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(7), 0x85u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(7), 0x9du64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(7), 0x9eu64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(7), 0x5cu64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(7), 0x5eu64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(7), 0x4au64, 32);
            if c.cc_l() { blk = 36; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(7), 0x4au64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(7), 0x53u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(7), 0x48u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(7), 0x49u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9260); };
            c.w32(3, t36);
            blk = 45; continue;
        }
        39 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b9265); };
            c.w64(3, t37);
            blk = 45; continue;
        }
        40 => {
            let Some(t38) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b926e); };
            c.w32(3, t38);
            blk = 45; continue;
        }
        41 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b9273); };
            c.w64(3, t39);
            blk = 45; continue;
        }
        42 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b927c); };
            c.w64(3, t40);
            blk = 45; continue;
        }
        43 => {
            let Some(t41) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b9285); };
            c.w32(3, t41);
            blk = 45; continue;
        }
        44 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b928a); };
            c.w64(3, t42);
            blk = 45; continue;
        }
        45 => {
            let Some(t43) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b9291); };
            c.w64(7, t43);
            c.w64(6, c.r64(7).wrapping_add(0x1c));
            let Some(t44) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b9299); };
            c.cmp(t44, c.r64(6), 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(12), c.r64(6)) { return RegfileExit::fault(0x70008b92a2); }
            c.w64(12, c.r64(7).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008b92aa); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(6, 0x1u64);
            blk = 50; continue;
        }
        49 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        50 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b92bb); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b92c0); };
            c.w64(9, t45);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b92c7); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b92cc); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b92d1); };
            c.w64(9, t46);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b92d8); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b92dd); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b92e2); }
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70008b92e7); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b92f3); };
            c.w32(9, t47);
            let r48 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r48);
            let Some(t49) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b92fa); };
            c.cmp(t49, 0x83du64, 16);
            if c.cc_ne() { blk = 57; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9303); };
            c.w32(11, t50);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t51) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b930a); };
            c.w32(9, t51);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x97u64, 64);
            if c.cc_a() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            return RegfileExit::fault(0x70008b931a);
        }
        54 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b931b); };
            c.cmp(t52, c.r32(9), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008b9324); };
            c.w64(9, t53);
            let Some(t54) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b932b); };
            c.cmp(t54, c.r32(8), 32);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            blk = 57; continue;
        }
        56 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b933e); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9345); };
            c.w64(6, t56);
            return c.call(0x7000870914, 0x70008b934e, 0x70008b9349);
        }
        59 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b934e); };
            if !c.push64(t57) { return RegfileExit::fault(0x70008b934e); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9351); };
            if !c.push64(t58) { return RegfileExit::fault(0x70008b9351); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b935c); };
            c.w64(6, t59);
            return c.call(0x70007f2274, 0x70008b9365, 0x70008b9360);
        }
        60 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b9365); };
            c.w64(1, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b936c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return c.ret(0x10, 0x70008b9373);
        }
        62 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b9376); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b937d); }
            return c.ret(0x0, 0x70008b937f);
        }
        63 => {
            c.w64(6, c.r64(1));
            let Some(t61) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9383); };
            c.w32(1, t61);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xeu64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x70008b9392);
        }
        65 => {
            let Some(t62) = c.ld32(c.r64(6).wrapping_add(0x3f)) else { return RegfileExit::fault(0x70008b9393); };
            c.w32(7, t62);
            let r63 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r63);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b9399); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b939b); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x70008b93a6, 0x70008b93a1);
        }
        66 => {
            blk = 60; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008b93a8);
        }
        68 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b93a9); };
            c.w64(3, t64);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b93b7, 0x70008b93b2);
        }
        69 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b93b7); };
            c.w64(6, t65);
            blk = 1; continue;
        }
        70 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008b93c0); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008b93cc, 0x70008b93c7);
        }
        71 => {
            c.w64(11, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b93cf); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b93d3); };
            c.w64(1, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b93d7); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b93db); };
            c.w64(12, t69);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b93e2); };
            c.w64(2, t70);
            c.w32(9, 0x8u64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b93ef); };
            c.w64(8, t71);
            blk = 13; continue;
        }
        72 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008b93fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008b9401); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008b9405); }
            return c.call(0x70007febf4, 0x70008b940e, 0x70008b9409);
        }
        73 => {
            c.w64(12, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9411); };
            c.w64(0, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b9415); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b9419); };
            c.w64(3, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b941d); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x10e0)) else { return RegfileExit::fault(0x70008b9421); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b9428); };
            c.w64(9, t77);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        74 => {
            return RegfileExit::jump(0x70008b9438);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b9454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b9454); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b9458); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b9459); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b945a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b945f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b9463); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 121; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b946d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b9479); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9486); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b948d); };
            c.w32(9, t5);
            c.w32(11, c.r64(9).wrapping_sub(0xba));
            c.cmp(c.r32(11), 0x78eu64, 32);
            if c.cc_a() { blk = 119; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b94a6); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b94ad); };
            c.cmp(t8, 0x90u64, 16);
            if c.cc_ne() { blk = 116; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b94ba); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b94c1); };
            c.w32(12, t11);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008b94d4);
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b94d5); };
            c.w32(11, t12);
            c.w32(9, c.r16(9));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008b94e0); }
            c.cmp(c.r32(9), 0x83du64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b94ed); };
            c.w32(9, t13);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x97u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008b9500);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b9501); };
            c.w32(9, t14);
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b9508); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x70008b9511); };
            c.w64(9, t16);
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b9518); };
            c.cmp(t17, c.r32(8), 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            blk = 12; continue;
        }
        11 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008b9536); };
            c.w64(3, t18);
            return c.call(0x7000870914, 0x70008b9542, 0x70008b953d);
        }
        14 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b9542); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9557); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            let Some(t22) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b955d); };
            c.cmp(t22, 0xbau64, 16);
            if c.cc_b() { blk = 123; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008b9569); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9570); };
            c.w64(6, t24);
            return c.call(0x7000870914, 0x70008b9579, 0x70008b9574);
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b9579); };
            c.cmp(t25, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b9586); };
            c.cmp(t26, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r27 = c.r32(1) | c.r32(8);
            c.w32(1, r27);
            c.logic(r27, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 19; continue;
        }
        19 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t28) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b959e); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b95a4); };
            c.logic(t30 & 0x40u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008b95aa); }
            blk = 24; continue;
        }
        22 => {
            c.w32(1, 0x26eu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b95b5); }
            c.w32(0, 0x1u64);
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b95bb); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b95c2); };
            c.w64(6, t32);
            return c.call(0x700086ed74, 0x70008b95cb, 0x70008b95c6);
        }
        23 => {
            return RegfileExit::fault(0x70008b95cb);
        }
        24 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b95cc); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b95d0); };
            c.w64(6, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b95d4); };
            c.w64(0, t35);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b95db); };
            c.w64(1, t36);
            blk = 25; continue;
        }
        25 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b95df); };
            c.w64(9, t37);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b95ed); };
            c.w32(12, t38);
            let r39 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r39);
            let Some(t40) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b95f4); };
            c.logic(t40 & 0x2u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t41) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9600); };
            c.w32(12, t41);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x79u64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x70008b9610);
        }
        29 => {
            let Some(t42) = c.ld32(c.r64(6).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b9611); };
            c.w32(12, t42);
            c.w32(3, 0x8u64);
            c.w64(15, c.r64(3));
            let r43 = c.add(c.r64(3), 0x10u64, 64);
            c.w64(3, r43);
            if c.cc_o() { blk = 93; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b962a); };
            c.w64(2, t44);
            let Some(t45) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b9631); };
            c.w64(7, t45);
            c.w64(0, c.r64(7).wrapping_add(c.r64(3)));
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b9638); };
            c.w64(1, t46);
            let Some(t47) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b963f); };
            c.cmp(t47, c.r64(0), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 125; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(2), c.r64(0)) { return RegfileExit::fault(0x70008b9655); }
            c.w64(0, c.r64(7).wrapping_add(0x1));
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008b965c); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008b9660); }
            blk = 35; continue;
        }
        34 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b9666); }
            let r48 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r48);
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(15), 0x18u64, 64);
            if c.cc_ne() { blk = 34; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9675); };
            c.w32(12, t49);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008b9685);
        }
        38 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b9686); };
            c.w64(15, t50);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008b968d); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008b9697);
        }
        40 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b9698); };
            c.w64(15, t51);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b969f); }
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x70008b96a9);
        }
        42 => {
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b96aa); }
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x70008b96b4);
        }
        44 => {
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70008b96b5); }
            let Some(t52) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b96b9); };
            c.w32(9, t52);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x70008b96cc);
        }
        46 => {
            let Some(t53) = c.ld32(c.r64(6).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b96cd); };
            c.w32(9, t53);
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x10d0)) else { return RegfileExit::fault(0x70008b96d4); };
            c.w64(11, t54);
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b96db); };
            c.w32(12, t55);
            let r56 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r56);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(13).wrapping_add(0x63f0));
            let Some(t57) = c.ld64(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b96f2); };
            c.w64(12, t57);
            blk = 67; continue;
        }
        48 => {
            let Some(t58) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b96fb); };
            c.w32(15, t58);
            let r59 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r59);
            let Some(t60) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9703); };
            c.w32(15, t60);
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_l() { blk = 55; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_l() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(15), 0x9du64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(15), 0x9eu64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(15), 0x5eu64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_l() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(15), 0x53u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(15), 0x48u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(15), 0x49u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t61) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b975c); };
            c.w32(12, t61);
            blk = 67; continue;
        }
        61 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b9763); };
            c.w64(12, t62);
            blk = 67; continue;
        }
        62 => {
            let Some(t63) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b976c); };
            c.w32(12, t63);
            blk = 67; continue;
        }
        63 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b9773); };
            c.w64(12, t64);
            blk = 67; continue;
        }
        64 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b977c); };
            c.w64(12, t65);
            blk = 67; continue;
        }
        65 => {
            let Some(t66) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b9785); };
            c.w32(12, t66);
            blk = 67; continue;
        }
        66 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b978c); };
            c.w64(12, t67);
            blk = 67; continue;
        }
        67 => {
            let Some(t68) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b9793); };
            c.w64(15, t68);
            c.w64(3, c.r64(15).wrapping_add(0x1c));
            let Some(t69) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b979a); };
            c.cmp(t69, c.r64(3), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.st64(c.r64(2), c.r64(3)) { return RegfileExit::fault(0x70008b97a3); }
            let r70 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r70);
            blk = 69; continue;
        }
        69 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b97aa); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b97ae); };
            c.w64(3, t71);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008b97b5); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008b97b9); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b97bd); };
            c.w64(7, t72);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008b97c4); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b97c8); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008b97cc); }
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008b97d0); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x10c0)) else { return RegfileExit::fault(0x70008b97d4); };
            c.w64(11, t73);
            let Some(t74) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b97db); };
            c.w32(12, t74);
            let r75 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r75);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(13).wrapping_add(0x63f0));
            let Some(t76) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b97f2); };
            c.w64(8, t76);
            blk = 90; continue;
        }
        71 => {
            let Some(t77) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b97fb); };
            c.w32(8, t77);
            let r78 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r78);
            let Some(t79) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9803); };
            c.w32(8, t79);
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_l() { blk = 78; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(8), 0x85u64, 32);
            if c.cc_l() { blk = 76; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(8), 0x85u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(8), 0x9du64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r32(8), 0x9eu64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(8), 0x5eu64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(8), 0x4au64, 32);
            if c.cc_l() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(8), 0x4au64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(8), 0x53u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b985c); };
            c.w32(8, t80);
            blk = 90; continue;
        }
        84 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b9863); };
            c.w64(8, t81);
            blk = 90; continue;
        }
        85 => {
            let Some(t82) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b986c); };
            c.w32(8, t82);
            blk = 90; continue;
        }
        86 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b9873); };
            c.w64(8, t83);
            blk = 90; continue;
        }
        87 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b987c); };
            c.w64(8, t84);
            blk = 90; continue;
        }
        88 => {
            let Some(t85) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b9885); };
            c.w32(8, t85);
            blk = 90; continue;
        }
        89 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b988c); };
            c.w64(8, t86);
            blk = 90; continue;
        }
        90 => {
            let Some(t87) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b9893); };
            c.w64(12, t87);
            c.w64(6, c.r64(12).wrapping_add(0x1c));
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b989b); };
            c.cmp(t88, c.r64(6), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st64(c.r64(2), c.r64(6)) { return RegfileExit::fault(0x70008b98a4); }
            c.w64(1, c.r64(12).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b98ac); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008b98b0); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008b98b3); }
            if !c.st32(c.r64(1).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008b98b6); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b98b9); }
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008b98bd); }
            if !c.st32(c.r64(1).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008b98c0); }
            c.w64(11, c.r64(1));
            c.w64(9, c.r64(15));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b98ca); };
            c.w64(6, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b98ce); };
            c.w64(0, t90);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b98d5); };
            c.w64(1, t91);
            blk = 95; continue;
        }
        93 => {
            return RegfileExit::fault(0x70008b98db);
        }
        94 => {
            c.w64(11, c.r64(9));
            blk = 95; continue;
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008b98df); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008b98e3); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w32(1, 0x1u64);
            blk = 98; continue;
        }
        97 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t92) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b98fa); };
            c.w32(1, t92);
            let r93 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r93);
            let Some(t94) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9900); };
            c.cmp(t94, 0x83du64, 16);
            if c.cc_ne() { blk = 105; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t95) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9908); };
            c.w32(12, t95);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t96) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b990f); };
            c.w32(1, t96);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x97u64, 64);
            if c.cc_a() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            return RegfileExit::fault(0x70008b991e);
        }
        102 => {
            let Some(t97) = c.ld32(c.r64(6).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b991f); };
            c.cmp(t97, c.r32(1), 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008b9927); };
            c.w64(1, t98);
            let Some(t99) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b992e); };
            c.cmp(t99, c.r32(8), 32);
            c.w8(1, (c.cc_ne() as u64));
            c.w32(1, c.r8(1));
            blk = 106; continue;
        }
        104 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 106; continue;
        }
        105 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b9945); };
            c.w64(3, t100);
            return c.call(0x7000870914, 0x70008b9951, 0x70008b994c);
        }
        108 => {
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9951); };
            if !c.push64(t101) { return RegfileExit::fault(0x70008b9951); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9954); };
            if !c.push64(t102) { return RegfileExit::fault(0x70008b9954); }
            let Some(t103) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b9957); };
            if !c.push64(t103) { return RegfileExit::fault(0x70008b9957); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b995a); };
            c.w64(6, t104);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008b996e, 0x70008b9969);
        }
        109 => {
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b996e); };
            c.w64(1, t105);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b9975); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return c.ret(0x10, 0x70008b997c);
        }
        111 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b997f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b9986); }
            return c.ret(0x0, 0x70008b9988);
        }
        112 => {
            let Some(t106) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9989); };
            c.w32(1, t106);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xeu64, 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return RegfileExit::fault(0x70008b9998);
        }
        114 => {
            let Some(t107) = c.ld32(c.r64(6).wrapping_add(0x3f)) else { return RegfileExit::fault(0x70008b9999); };
            c.w32(7, t107);
            let r108 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r108);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b999f); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b99a1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b99a3); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2274, 0x70008b99ae, 0x70008b99a9);
        }
        115 => {
            blk = 109; continue;
        }
        116 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b99b0); };
            c.w64(2, t109);
            let Some(t110) = c.ld32(c.r64(2).wrapping_add(0x25b)) else { return RegfileExit::fault(0x70008b99b7); };
            c.w32(2, t110);
            let r111 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r111);
            return c.call(0x7000824234, 0x70008b99c5, 0x70008b99c0);
        }
        117 => {
            return RegfileExit::fault(0x70008b99c5);
        }
        118 => {
            return RegfileExit::fault(0x70008b99c6);
        }
        119 => {
            c.w32(1, 0x34u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b99cc); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b99cd); };
            c.w64(1, t112);
            let Some(t113) = c.ld32(c.r64(1).wrapping_add(0x25f)) else { return RegfileExit::fault(0x70008b99d4); };
            c.w32(1, t113);
            let r114 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r114);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b99dd); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b99de); };
            c.w64(3, t115);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b99ef, 0x70008b99ea);
        }
        120 => {
            return RegfileExit::fault(0x70008b99ef);
        }
        121 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b99f2); };
            c.w64(3, t116);
            return c.call(0x700086ed74, 0x70008b99fe, 0x70008b99f9);
        }
        122 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b99fe); };
            c.w64(6, t117);
            blk = 1; continue;
        }
        123 => {
            c.w32(1, 0x54u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b9a0c); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b9a0d); };
            c.w64(1, t118);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b9a14); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b9a15); }
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b9a16); };
            c.w64(3, t119);
            c.w32(0, 0x3u64);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9a22); };
            c.w64(6, t120);
            return c.call(0x700086ed74, 0x70008b9a2b, 0x70008b9a26);
        }
        124 => {
            return RegfileExit::fault(0x70008b9a2b);
        }
        125 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008b9a2c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008b9a30); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008b9a3c, 0x70008b9a37);
        }
        126 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9a3c); };
            c.w64(12, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9a40); };
            c.w64(6, t122);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t123) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b9a47); };
            c.w64(9, t123);
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b9a4b); };
            c.w64(2, t124);
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b9a52); };
            c.w64(1, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9a59); };
            c.w64(11, t126);
            c.w32(15, 0x8u64);
            blk = 33; continue;
        }
        127 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008b9a6d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008b9a71); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008b9a75); }
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x70008b9a81, 0x70008b9a7c);
        }
        128 => {
            c.w64(15, c.r64(0));
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9a84); };
            c.w64(6, t127);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b9a8b); };
            c.w64(0, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b9a8f); };
            c.w64(9, t129);
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b9a93); };
            c.w64(2, t130);
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b9a9a); };
            c.w64(1, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9aa1); };
            c.w64(12, t132);
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x10d0)) else { return RegfileExit::fault(0x70008b9aa5); };
            c.w64(11, t133);
            blk = 69; continue;
        }
        129 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008b9ab1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008b9ab5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008b9ab9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008b9abd); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008b9acb, 0x70008b9ac6);
        }
        130 => {
            c.w64(1, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9ace); };
            c.w64(15, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9ad2); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b9ad6); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x10c0)) else { return RegfileExit::fault(0x70008b9ada); };
            c.w64(11, t137);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b9ae1); };
            c.w64(7, t138);
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b9ae8); };
            c.w64(3, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b9aef); };
            c.w64(9, t140);
            blk = 92; continue;
        }
        131 => {
            return RegfileExit::jump(0x70008b9afc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b9b14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b9b14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b9b18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b9b19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b9b1a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b9b1f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b9b23); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 79; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b9b2d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9b39); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9b40); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 77; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9b59); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9b60); };
            c.cmp(t9, 0x90u64, 16);
            if c.cc_ne() { blk = 74; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b9b6d); };
            c.w64(3, t10);
            return c.call(0x70008b23d4, 0x70008b9b79, 0x70008b9b74);
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9b79); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b9b80); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9b87); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008b9b8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b9b91); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b9b95); };
            c.w64(11, t17);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008b9b9c); }
            let Some(t18) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008b9ba0); };
            c.cmp(t18, 0x0u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9baa); };
            c.w64(6, t19);
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9bae); };
            c.w32(12, t20);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xeau64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008b9bc1);
        }
        8 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b9bc2); };
            c.w64(0, t21);
            let Some(t22) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b9bc6); };
            c.cmp(t22, c.r32(0), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        10 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x1000)) else { return RegfileExit::fault(0x70008b9bd3); };
            c.w64(12, t23);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b9bdd); };
            c.cmp(t24, c.r32(15), 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b9bf0); };
            c.w64(12, t25);
            blk = 17; continue;
        }
        13 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b9bf9); };
            c.w64(3, t26);
            return c.call(0x7000870914, 0x70008b9c05, 0x70008b9c00);
        }
        14 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9c0d); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b9c14); };
            c.logic(t29 & 0x2u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(12, c.r64(0));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9c22); };
            c.w64(6, t30);
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008b9c26); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008b9c2a); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b9c31); };
            c.w64(0, t32);
            return c.call(0x7000870914, 0x70008b9c3a, 0x70008b9c35);
        }
        18 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t33) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9c42); };
            c.w32(8, t33);
            let r34 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r34);
            let Some(t35) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b9c49); };
            c.logic(t35 & 0x2u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b9c54); };
            if !c.push64(t36) { return RegfileExit::fault(0x70008b9c54); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9c5c); };
            c.w64(6, t37);
            return c.call(0x70007f2274, 0x70008b9c65, 0x70008b9c60);
        }
        21 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008b9c65); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 102; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t38) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9c71); };
            c.w32(8, t38);
            let r39 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r39);
            let Some(t40) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9c78); };
            c.cmp(t40, 0xbau64, 16);
            if c.cc_b() { blk = 102; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008b9c85); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9c8c); };
            c.w64(6, t42);
            return c.call(0x7000870914, 0x70008b9c95, 0x70008b9c90);
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(0)) { return RegfileExit::fault(0x70008b9c95); }
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9c99); };
            c.w64(6, t43);
            let Some(t44) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9c9d); };
            c.w32(8, t44);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x68u64, 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x70008b9cad);
        }
        26 => {
            let Some(t45) = c.ld32(c.r64(6).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008b9cae); };
            c.w32(8, t45);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008b9cb5); }
            blk = 28; continue;
        }
        27 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9cbb); };
            c.w64(6, t46);
            c.w64(11, c.r64(8));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b9cc2); };
            c.w64(0, t47);
            blk = 28; continue;
        }
        28 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b9cc6); };
            if !c.push64(t48) { return RegfileExit::fault(0x70008b9cc6); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f21d4, 0x70008b9cd3, 0x70008b9cce);
        }
        29 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008b9cd3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9cdf); };
            c.w32(8, t49);
            let r50 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r50);
            let Some(t51) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b9ce6); };
            c.cmp(c.r32(8), t51, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t52) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9cf0); };
            c.cmp(t52, 0xbau64, 16);
            if c.cc_b() { blk = 100; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x70008b9cfd); };
            c.w64(3, t53);
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9d04); };
            c.w64(6, t54);
            return c.call(0x7000870914, 0x70008b9d0d, 0x70008b9d08);
        }
        33 => {
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b9d0d); };
            c.cmp(t55, c.r32(0), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t56) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b9d1d); };
            c.cmp(t56, c.r32(0), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9d26); };
            c.w32(8, t57);
            let r58 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r58);
            let Some(t59) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b9d2d); };
            c.logic(t59 & 0x10u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b9d34); };
            c.cmp(t60, c.r32(8), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9d41); };
            c.cmp(t61, 0x41u64, 16);
            if c.cc_e() { blk = 83; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 68; continue;
        }
        40 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9d52); };
            c.w64(0, t62);
            blk = 53; continue;
        }
        41 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9d5b); };
            c.w64(0, t63);
            blk = 53; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9d6e); };
            c.w64(0, t64);
            blk = 53; continue;
        }
        44 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9d74); };
            c.w64(0, t65);
            blk = 53; continue;
        }
        45 => {
            let Some(t66) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b9d7a); };
            c.w32(8, t66);
            let r67 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r67);
            let Some(t68) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b9d81); };
            c.cmp(t68, c.r32(8), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b9d93); };
            c.cmp(t69, c.r32(8), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t70) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9d9c); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r71);
            let Some(t72) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b9da3); };
            c.logic(t72 & 0x10u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b9daa); };
            c.cmp(t73, c.r32(9), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t74) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9db7); };
            c.cmp(t74, 0x41u64, 16);
            if c.cc_e() { blk = 87; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 68; continue;
        }
        52 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b9dd2); };
            c.w64(8, t75);
            let Some(t76) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9dd6); };
            c.cmp(t76, c.r32(8), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008b9ddc); };
            c.w64(3, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9de3); };
            c.w64(6, t78);
            return c.call(0x7000870914, 0x70008b9dec, 0x70008b9de7);
        }
        55 => {
            blk = 57; continue;
        }
        56 => {
            let Some(t79) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b9dee); };
            c.w32(0, t79);
            let r80 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r80);
            blk = 57; continue;
        }
        57 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b9df4); };
            c.w64(7, t81);
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b9df8); };
            c.cmp(t82, c.r32(7), 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(3, c.r64(0));
            let Some(t83) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b9e08); };
            c.w64(0, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9e0c); };
            c.w64(6, t84);
            return c.call(0x70008ba474, 0x70008b9e15, 0x70008b9e10);
        }
        59 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008b9e15); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b9e19); };
            c.w64(3, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9e20); };
            c.w64(6, t86);
            return c.call(0x7000870914, 0x70008b9e29, 0x70008b9e24);
        }
        60 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b9e29); };
            if !c.push64(t87) { return RegfileExit::fault(0x70008b9e29); }
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b9e2c); };
            if !c.push64(t88) { return RegfileExit::fault(0x70008b9e2c); }
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9e2f); };
            if !c.push64(t89) { return RegfileExit::fault(0x70008b9e2f); }
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9e32); };
            c.w64(6, t90);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008b9e46, 0x70008b9e41);
        }
        61 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b9e46); };
            c.w64(8, t91);
            let Some(t92) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x70008b9e4d); };
            c.cmp(t92, 0x0u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9e53); };
            c.w64(1, t93);
            blk = 27; continue;
        }
        63 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9e5c); };
            c.w64(1, t94);
            let Some(t95) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b9e60); };
            c.w32(9, t95);
            let r96 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r96);
            let Some(t97) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b9e67); };
            c.cmp(t97, 0x83du64, 16);
            if c.cc_ne() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(9, 0x1u64);
            blk = 66; continue;
        }
        65 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 67; continue;
        }
        67 => {
            blk = 27; continue;
        }
        68 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9e8a); };
            c.w64(0, t98);
            blk = 69; continue;
        }
        69 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b9e8e); };
            c.w64(1, t99);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b9e95); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            return c.ret(0x10, 0x70008b9e9c);
        }
        71 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b9e9f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b9ea6); }
            return c.ret(0x0, 0x70008b9ea8);
        }
        72 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b9eaf); }
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b9eb1); };
            c.w64(8, t100);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b9eb8); }
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b9eba); };
            c.w64(3, t101);
            c.w32(0, 0x2u64);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9ec6); };
            c.w64(6, t102);
            return c.call(0x700086ed74, 0x70008b9ecf, 0x70008b9eca);
        }
        73 => {
            return RegfileExit::fault(0x70008b9ecf);
        }
        74 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b9ed0); };
            c.w64(2, t103);
            let Some(t104) = c.ld32(c.r64(2).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b9ed7); };
            c.w32(2, t104);
            let r105 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r105);
            return c.call(0x7000824234, 0x70008b9ee5, 0x70008b9ee0);
        }
        75 => {
            return RegfileExit::fault(0x70008b9ee5);
        }
        76 => {
            return RegfileExit::fault(0x70008b9ee6);
        }
        77 => {
            c.w32(8, 0x34u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b9eed); }
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b9eef); };
            c.w64(8, t106);
            let Some(t107) = c.ld32(c.r64(8).wrapping_add(0x267)) else { return RegfileExit::fault(0x70008b9ef6); };
            c.w32(8, t107);
            let r108 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r108);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b9f00); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b9f02); };
            c.w64(3, t109);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b9f13, 0x70008b9f0e);
        }
        78 => {
            return RegfileExit::fault(0x70008b9f13);
        }
        79 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b9f16); };
            c.w64(3, t110);
            return c.call(0x700086ed74, 0x70008b9f22, 0x70008b9f1d);
        }
        80 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9f22); };
            c.w64(6, t111);
            blk = 1; continue;
        }
        81 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc38)) else { return RegfileExit::fault(0x70008b9f2b); };
            c.w64(3, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9f32); };
            c.w64(6, t113);
            c.w64(0, c.r64(9));
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b9f39); };
            c.w64(1, t114);
            c.w64(12, c.r64(1));
            return c.call(0x7000871cb4, 0x70008b9f48, 0x70008b9f43);
        }
        82 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9f48); };
            c.w64(1, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b9f4c); };
            c.w64(9, t116);
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b9f50); };
            c.w64(11, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b9f57); };
            c.w64(8, t118);
            blk = 6; continue;
        }
        83 => {
            let Some(t119) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9f60); };
            c.logic(t119 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9f6d); };
            c.w64(0, t120);
            blk = 53; continue;
        }
        85 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b9f76); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 68; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9f89); };
            c.w64(0, t121);
            blk = 53; continue;
        }
        87 => {
            let Some(t122) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b9f92); };
            c.logic(t122 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 88; continue;
        }
        88 => {
            blk = 68; continue;
        }
        89 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b9fa5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 68; continue; }
            blk = 90; continue;
        }
        90 => {
            blk = 53; continue;
        }
        91 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b9fbe); }
            let Some(t123) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b9fbf); };
            if !c.push64(t123) { return RegfileExit::fault(0x70008b9fbf); }
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9fc2); };
            c.w64(6, t124);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x70008b9fd0, 0x70008b9fcb);
        }
        92 => {
            blk = 59; continue;
        }
        93 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0xc40)) else { return RegfileExit::fault(0x70008b9fd5); };
            c.w64(3, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9fdc); };
            c.w64(6, t126);
            return c.call(0x7000871cb4, 0x70008b9fe5, 0x70008b9fe0);
        }
        94 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b9fe5); };
            c.w64(1, t127);
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b9fe9); };
            c.w64(8, t128);
            blk = 27; continue;
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008b9ff5); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x70008b9ff9); };
            c.w64(3, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ba000); };
            c.w64(0, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba004); };
            c.w64(6, t131);
            return c.call(0x7000870914, 0x70008ba00d, 0x70008ba008);
        }
        96 => {
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba00d); };
            c.cmp(t132, c.r32(0), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ba016); };
            c.cmp(t133, c.r32(0), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ba01f); };
            if !c.push64(t134) { return RegfileExit::fault(0x70008ba01f); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba027); };
            c.w64(6, t135);
            return c.call(0x70007f2274, 0x70008ba030, 0x70008ba02b);
        }
        99 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ba030); };
            c.w64(0, t136);
            blk = 106; continue;
        }
        100 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba036); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008ba037); };
            c.w64(3, t137);
            c.w32(0, 0x1u64);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba043); };
            c.w64(6, t138);
            return c.call(0x700086ed74, 0x70008ba04c, 0x70008ba047);
        }
        101 => {
            return RegfileExit::fault(0x70008ba04c);
        }
        102 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x70008ba04d); };
            c.w64(3, t139);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba056); };
            c.w64(6, t140);
            return c.call(0x700086ed74, 0x70008ba05f, 0x70008ba05a);
        }
        103 => {
            return RegfileExit::fault(0x70008ba05f);
        }
        104 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ba060); };
            if !c.push64(t141) { return RegfileExit::fault(0x70008ba060); }
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x70008ba063); };
            c.w64(3, t142);
            c.w32(0, 0x1u64);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba06f); };
            c.w64(6, t143);
            return c.call(0x700086ed74, 0x70008ba078, 0x70008ba073);
        }
        105 => {
            return RegfileExit::fault(0x70008ba078);
        }
        106 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba079); }
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba07a); };
            c.w64(8, t144);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ba081); }
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ba083); };
            c.w64(7, t145);
            c.w32(0, 0x1u64);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba08c); };
            c.w64(6, t146);
            return c.call(0x70007f2134, 0x70008ba095, 0x70008ba090);
        }
        107 => {
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ba095); };
            c.w64(0, t147);
            blk = 69; continue;
        }
        108 => {
            return RegfileExit::jump(0x70008ba0a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ba0b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba0b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba0b8); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008ba0be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008ba0c2); }
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba0c6); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba0d3); }
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba0d4); };
            c.w64(0, t2);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba0db); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(3));
            c.w64(2, c.r64(0));
            return c.call(0x70007f2134, 0x70008ba0ec, 0x70008ba0e7);
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba0ec); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba0f0); };
            c.cmp(t4, c.r32(2), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba0f9); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba100); };
            c.cmp(t7, 0x83du64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba10f); };
            c.w64(6, t8);
            return c.call(0x70008ba734, 0x70008ba118, 0x70008ba113);
        }
        5 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ba11b); }
            return c.ret(0x0, 0x70008ba11c);
        }
        6 => {
            c.w64(1, c.r64(2));
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba120); };
            c.cmp(t9, c.r32(1), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba129); };
            c.w64(0, t10);
            blk = 5; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba132); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba138); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba139); };
            c.w64(3, t13);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008ba140); }
            c.w32(0, 0x1u64);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba146); };
            c.w64(6, t14);
            return c.call(0x70007f2134, 0x70008ba14f, 0x70008ba14a);
        }
        9 => {
            blk = 5; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba151); };
            c.w64(0, t15);
            blk = 5; continue;
        }
        11 => {
            c.w64(2, c.r64(1));
            let Some(t16) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba15d); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            let Some(t18) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba164); };
            c.cmp(t18, 0x83du64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            return c.call(0x70008ba734, 0x70008ba178, 0x70008ba173);
        }
        13 => {
            blk = 5; continue;
        }
        14 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba17a); };
            c.cmp(t19, c.r32(2), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba183); };
            c.w64(0, t20);
            blk = 5; continue;
        }
        16 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba18c); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba192); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba193); };
            c.w64(3, t23);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008ba19a); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008ba1a5, 0x70008ba1a0);
        }
        17 => {
            blk = 5; continue;
        }
        18 => {
            blk = 21; continue;
        }
        19 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba1ac); };
            c.w64(6, t24);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba1b0); };
            c.w64(3, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba1b7); };
            c.w64(1, t26);
            return c.call(0x70008ba214, 0x70008ba1c0, 0x70008ba1bb);
        }
        20 => {
            blk = 5; continue;
        }
        21 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba1c5); };
            c.w64(6, t27);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba1c9); };
            c.w64(3, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba1d0); };
            c.w64(1, t29);
            return c.call(0x70008ba214, 0x70008ba1d9, 0x70008ba1d4);
        }
        22 => {
            blk = 5; continue;
        }
        23 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba1de); };
            c.w64(6, t30);
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba1e2); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba1e9); };
            c.w64(1, t32);
            return c.call(0x70008ba214, 0x70008ba1f2, 0x70008ba1ed);
        }
        24 => {
            blk = 5; continue;
        }
        25 => {
            return RegfileExit::jump(0x70008ba1f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ba214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba214); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba218); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008ba21e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008ba222); }
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba226); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba233); }
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba234); };
            c.w64(0, t2);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba23b); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(3));
            c.w64(2, c.r64(0));
            return c.call(0x70007f2134, 0x70008ba24c, 0x70008ba247);
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba24c); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba250); };
            c.cmp(t4, c.r32(2), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba259); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            let Some(t7) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba260); };
            c.cmp(t7, 0x83du64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba26f); };
            c.w64(6, t8);
            return c.call(0x70008ba734, 0x70008ba278, 0x70008ba273);
        }
        5 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ba27b); }
            return c.ret(0x0, 0x70008ba27c);
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba27d); };
            c.cmp(t9, c.r32(2), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba286); };
            c.w64(0, t10);
            blk = 5; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba28f); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba295); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba296); };
            c.w64(0, t13);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba29d); }
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba29e); };
            c.w64(6, t14);
            c.w32(0, 0x1u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x70008ba2af, 0x70008ba2aa);
        }
        9 => {
            blk = 5; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba2b1); };
            c.w64(0, t15);
            blk = 5; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba2ba); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            let Some(t18) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba2c0); };
            c.cmp(t18, 0x83du64, 16);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ba2ce); };
            c.w64(1, t19);
            c.w64(2, c.r64(1));
            return c.call(0x70008b1fb4, 0x70008ba2dd, 0x70008ba2d8);
        }
        13 => {
            blk = 5; continue;
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba2df); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba2e8); };
            c.w64(0, t21);
            blk = 5; continue;
        }
        16 => {
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ba2f1); };
            c.w32(7, t22);
            let r23 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r23);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba2f7); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba2f8); };
            c.w64(2, t24);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ba2ff); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008ba30a, 0x70008ba305);
        }
        17 => {
            blk = 5; continue;
        }
        18 => {
            blk = 24; continue;
        }
        19 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba311); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba315); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba31c); };
            c.cmp(t28, 0x83du64, 16);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ba325); };
            c.w64(1, t29);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba332); };
            c.w64(6, t30);
            return c.call(0x70008b1fb4, 0x70008ba33b, 0x70008ba336);
        }
        21 => {
            blk = 5; continue;
        }
        22 => {
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ba340); };
            c.w32(7, t31);
            let r32 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba346); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba347); };
            c.w64(0, t33);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba34e); }
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba34f); };
            c.w64(6, t34);
            c.w32(0, 0x1u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x70008ba360, 0x70008ba35b);
        }
        23 => {
            blk = 5; continue;
        }
        24 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba365); };
            c.w64(2, t35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba369); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba370); };
            c.cmp(t38, 0x83du64, 16);
            if c.cc_ne() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ba379); };
            c.w64(1, t39);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(2));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba386); };
            c.w64(6, t40);
            return c.call(0x70008b1fb4, 0x70008ba38f, 0x70008ba38a);
        }
        26 => {
            blk = 5; continue;
        }
        27 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba394); };
            c.cmp(t41, c.r32(2), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba39d); };
            c.w64(0, t42);
            blk = 5; continue;
        }
        29 => {
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ba3a9); };
            c.w32(7, t43);
            let r44 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r44);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba3af); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba3b0); };
            c.w64(8, t45);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ba3b7); }
            c.w32(0, 0x1u64);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba3be); };
            c.w64(6, t46);
            return c.call(0x70007f2134, 0x70008ba3c7, 0x70008ba3c2);
        }
        30 => {
            blk = 5; continue;
        }
        31 => {
            return RegfileExit::jump(0x70008ba3d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ba3d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba3d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ba3d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ba3d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba3da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ba3df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ba3e5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba3ed); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba3f3); };
            c.w32(2, t5);
            let r6 = c.sub(c.r32(2), 0xbau64, 32);
            c.w32(2, r6);
            c.cmp(c.r32(2), 0x78eu64, 32);
            if c.cc_a() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ba405); };
            c.w64(3, t7);
            return c.call(0x70008ba474, 0x70008ba40e, 0x70008ba409);
        }
        4 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba40e); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ba415); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x10, 0x70008ba41c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ba41f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ba426); }
            return c.ret(0x0, 0x70008ba428);
        }
        7 => {
            c.w32(2, 0x34u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ba42e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ba42f); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x26b)) else { return RegfileExit::fault(0x70008ba436); };
            c.w32(2, t10);
            let r11 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r11);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ba43f); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008ba440); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ba451, 0x70008ba44c);
        }
        8 => {
            return RegfileExit::fault(0x70008ba451);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ba452); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ba456); };
            c.w64(3, t13);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ba464, 0x70008ba45f);
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba464); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008ba470);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ba474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba474); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba478); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba47e); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ba484); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ba48a); };
            c.w32(8, t5);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ba494); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ba498); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008ba4a5);
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008ba4a6); };
            c.w32(9, t6);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba4b2); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba4b9); };
            c.cmp(t9, 0x83du64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r64(8), 0x97u64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008ba4cb);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008ba4cc); };
            c.w32(1, t10);
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ba4d2); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x70008ba4dc); };
            c.w64(1, t12);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ba4e6); };
            c.cmp(t13, c.r32(8), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(3));
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ba4ff); }
            return c.ret(0x0, 0x70008ba500);
        }
        11 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba50a); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ba510); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ba516); };
            c.w32(8, t18);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x70008ba529);
        }
        14 => {
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008ba52a); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008ba533); };
            c.w32(1, t21);
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ba536); };
            c.w64(8, t22);
            let Some(t23) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ba53d); };
            c.w64(9, t23);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ba544); };
            c.w64(12, t24);
            let Some(t25) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008ba54b); };
            c.cmp(t25, c.r64(11), 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008ba555); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 16; continue;
        }
        16 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ba55c); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ba560); };
            c.w64(1, t26);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008ba567); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008ba56b); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008ba571); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008ba575); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008ba579); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008ba57d); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008ba581); };
            c.w64(1, t27);
            let Some(t28) = c.ld32(c.r64(1)) else { return RegfileExit::fault(0x70008ba588); };
            c.w32(1, t28);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008ba58a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008ba58e); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba5a4); };
            c.w32(9, t29);
            let r30 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r30);
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ba5ab); };
            c.w32(9, t31);
            let r32 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r32);
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ba5b2); };
            c.w32(11, t33);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008ba5c5);
        }
        20 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008ba5c6); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            let Some(t36) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba5d0); };
            c.w32(9, t36);
            let r37 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r37);
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ba5d7); };
            c.logic(t38 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba5de); };
            c.w64(9, t39);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t40) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba5eb); };
            c.w32(11, t40);
            let r41 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r41);
            let Some(t42) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba5f2); };
            c.w32(11, t42);
            c.cmp(c.r16(11), 0x83du64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(11, c.r16(11));
            c.cmp(c.r32(11), 0x83du64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::fault(0x70008ba60c);
        }
        25 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ba60d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ba60f); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ba611); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x70008ba61d, 0x70008ba618);
        }
        26 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba61d); };
            c.w64(8, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba621); };
            c.w64(3, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba625); };
            c.w64(6, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ba629); };
            c.w64(1, t46);
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(1) & 0x6u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ba632); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba634); };
            c.w64(1, t47);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ba63b); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x70008ba63c); };
            c.w64(1, t48);
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x70008ba650, 0x70008ba64b);
        }
        29 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba650); };
            c.w64(8, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba654); };
            c.w64(3, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba658); };
            c.w64(6, t51);
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008ba734, 0x70008ba664, 0x70008ba65f);
        }
        31 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba664); };
            c.w64(0, t52);
            blk = 10; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ba66d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ba671); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008ba675); };
            c.w64(1, t53);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(1));
            return c.call(0x7000870914, 0x70008ba687, 0x70008ba682);
        }
        33 => {
            let Some(t54) = c.ld32(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba687); };
            c.cmp(c.r32(0), t54, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba68c); };
            c.w64(0, t55);
            blk = 10; continue;
        }
        35 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba695); };
            c.w64(3, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba699); };
            c.w64(6, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba69d); };
            c.w64(0, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ba6a1); };
            c.w64(9, t59);
            blk = 11; continue;
        }
        36 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ba6aa); };
            c.w64(1, t60);
            c.w64(3, c.r64(1));
            return c.call(0x70008b23d4, 0x70008ba6b9, 0x70008ba6b4);
        }
        37 => {
            let Some(t61) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba6b9); };
            c.w32(7, t61);
            let r62 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba6bf); };
            if !c.push64(t63) { return RegfileExit::fault(0x70008ba6bf); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ba6c2); };
            c.w64(1, t64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ba6c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ba6ca); }
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba6ce); };
            c.w64(6, t65);
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2134, 0x70008ba6df, 0x70008ba6da);
        }
        38 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba6df); };
            c.w64(1, t66);
            let Some(t67) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ba6e3); };
            c.w32(0, t67);
            let r68 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r68);
            blk = 10; continue;
        }
        39 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008ba6f3); }
            return c.call(0x70007febf4, 0x70008ba6fc, 0x70008ba6f7);
        }
        40 => {
            c.w64(8, c.r64(0));
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba6ff); };
            c.w64(3, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba703); };
            c.w64(6, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ba707); };
            c.w64(1, t71);
            blk = 16; continue;
        }
        41 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba710); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008ba711); };
            c.w64(3, t72);
            c.w32(0, 0x1u64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba71d); };
            c.w64(6, t73);
            return c.call(0x700086ed74, 0x70008ba726, 0x70008ba721);
        }
        42 => {
            blk = 26; continue;
        }
        43 => {
            return RegfileExit::jump(0x70008ba72c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ba734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008ba734); };
            c.w64(7, t0);
            let Some(t1) = c.ld8(c.r64(7)) else { return RegfileExit::fault(0x70008ba73b); };
            c.logic(t1 & 0xeu64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba755); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba75b); };
            c.w32(8, t4);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008ba76c); };
            c.w64(9, t5);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ba776); };
            c.cmp(t6, c.r32(11), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba780); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            c.w32(8, c.r16(8));
            let Some(t9) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ba78b); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            c.cmp(c.r32(8), 0x83du64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ba79b); };
            c.w32(8, t11);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x97u64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008ba7ae);
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(9).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008ba7af); };
            c.w32(11, t12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ba7b6); };
            c.cmp(t13, c.r32(11), 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(8), 0xeu64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008ba7c6);
        }
        11 => {
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0x3f)) else { return RegfileExit::fault(0x70008ba7c7); };
            c.w32(7, t14);
            let r15 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r15);
            blk = 17; continue;
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ba7d0); };
            c.w32(8, t16);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x68u64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x70008ba7e0);
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba7e1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba7e5); }
            let r17 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r17);
            let Some(t18) = c.ld32(c.r64(9).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008ba7eb); };
            c.cmp(t18, c.r32(7), 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 15; continue;
        }
        15 => {
            return c.call(0x70008b1bf4, 0x70008ba7fd, 0x70008ba7f8);
        }
        16 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ba800); }
            return c.ret(0x0, 0x70008ba801);
        }
        17 => {
            c.w64(8, c.r64(7));
            blk = 18; continue;
        }
        18 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba817); };
            c.w32(11, t19);
            let r20 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r20);
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba81e); };
            c.cmp(t21, 0x80bu64, 16);
            if c.cc_ne() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t22) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ba827); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            blk = 18; continue;
        }
        21 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 23; continue;
        }
        22 => {
            c.w32(11, 0x1u64);
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t24) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba841); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            let Some(t26) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba848); };
            c.cmp(t26, 0xbcu64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba851); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            blk = 18; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba85a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba85e); }
            let r29 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r29);
            blk = 28; continue;
        }
        27 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba866); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba86a); }
            let r30 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r30);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba876); };
            c.w32(9, t31);
            let r32 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r32);
            let Some(t33) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ba87d); };
            c.w32(9, t33);
            let r34 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r34);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ba88f); };
            c.w32(6, t35);
            let r36 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r36);
            blk = 31; continue;
        }
        31 => {
            let Some(t37) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba896); };
            c.w32(8, t37);
            let r38 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r38);
            let Some(t39) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ba89d); };
            c.w32(6, t39);
            let r40 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r40);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ba8a4); };
            c.w64(8, t41);
            let Some(t42) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ba8ab); };
            c.w64(9, t42);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ba8b2); };
            c.w64(12, t43);
            let Some(t44) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008ba8b9); };
            c.cmp(t44, c.r64(11), 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008ba8c3); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 33; continue;
        }
        33 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xd40)) else { return RegfileExit::fault(0x70008ba8ca); };
            c.w64(9, t45);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008ba8d1); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008ba8d5); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008ba8d9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008ba8dd); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008ba8e1); }
            c.w64(0, c.r64(8));
            return c.call(0x700081f374, 0x70008ba8ed, 0x70008ba8e8);
        }
        34 => {
            blk = 16; continue;
        }
        35 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba8f2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba8f6); }
            let r46 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r46);
            return c.call(0x70008b1bf4, 0x70008ba901, 0x70008ba8fc);
        }
        36 => {
            blk = 16; continue;
        }
        37 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba906); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba90a); }
            let r47 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r47);
            return c.call(0x70008b1bf4, 0x70008ba915, 0x70008ba910);
        }
        38 => {
            blk = 16; continue;
        }
        39 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba91a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba91e); }
            let r48 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r48);
            blk = 40; continue;
        }
        40 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ba924); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008ba925); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x25d0)) else { return RegfileExit::fault(0x70008ba926); };
            c.w64(3, t49);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ba937, 0x70008ba932);
        }
        41 => {
            blk = 16; continue;
        }
        42 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba93c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba940); }
            let r50 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r50);
            blk = 44; continue;
        }
        43 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ba948); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ba94c); }
            let r51 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r51);
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ba952); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ba956); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ba95a); }
            c.w64(0, c.r64(3));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008ba961); };
            c.w64(3, t52);
            c.w64(7, c.r64(3));
            return c.call(0x7000870914, 0x70008ba970, 0x70008ba96b);
        }
        45 => {
            blk = 48; continue;
        }
        46 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ba972); };
            c.w64(1, t53);
            c.w64(3, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba97c); };
            c.w64(0, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba980); };
            c.w64(6, t55);
            return c.call(0x70008b1fb4, 0x70008ba989, 0x70008ba984);
        }
        47 => {
            blk = 16; continue;
        }
        48 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t56) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ba992); };
            c.w32(7, t56);
            let r57 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r57);
            let Some(t58) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ba998); };
            c.logic(t58 & 0x2u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(8, 0x1u64);
            blk = 52; continue;
        }
        51 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 52; continue;
        }
        52 => {
            c.w32(7, 0x1u64);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(7, c.r64(0));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba9b7); };
            c.w64(0, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba9bb); };
            c.w64(3, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba9bf); };
            c.w64(6, t61);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ba9c6); }
            blk = 17; continue;
        }
        54 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ba9d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008ba9d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ba9d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ba9dd); }
            return c.call(0x70007febb4, 0x70008ba9e6, 0x70008ba9e1);
        }
        55 => {
            c.w64(8, c.r64(0));
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba9e9); };
            c.w64(0, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ba9ed); };
            c.w64(3, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ba9f1); };
            c.w64(6, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ba9f5); };
            c.w64(7, t65);
            blk = 33; continue;
        }
        56 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ba9fe); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008baa02); };
            c.w64(3, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008baa06); };
            c.w64(6, t68);
            return c.call(0x70008b1bf4, 0x70008baa0f, 0x70008baa0a);
        }
        57 => {
            blk = 16; continue;
        }
        58 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008baa14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008baa18); }
            let r69 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r69);
            blk = 40; continue;
        }
        59 => {
            return RegfileExit::jump(0x70008baa24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008baa34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008baa34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008baa38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008baa39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008baa3a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008baa3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008baa43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 117; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008baa4d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baa59); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baa5f); };
            c.cmp(t5, 0x83du64, 16);
            if c.cc_ne() { blk = 115; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008baa6b); };
            c.w32(8, t6);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008baa7e);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008baa7f); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008baa89); }
            c.cmp(c.r64(8), 0x97u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008baa96);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008baa97); };
            c.w32(8, t9);
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008baa9e); };
            c.cmp(t10, c.r32(8), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x70008baaa6); };
            c.w64(8, t11);
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008baaad); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008baab7); };
            c.w64(3, t13);
            return c.call(0x7000870914, 0x70008baac3, 0x70008baabe);
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008baac3); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baad8); };
            c.w32(1, t15);
            let r16 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r16);
            let Some(t17) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baade); };
            c.cmp(t17, 0xbau64, 16);
            if c.cc_b() { blk = 119; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008baaea); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008baaf1); };
            c.w64(6, t19);
            return c.call(0x7000870914, 0x70008baafa, 0x70008baaf5);
        }
        14 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008baafa); };
            c.cmp(t20, c.r32(0), 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bab07); };
            c.cmp(t21, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let r22 = c.r32(1) | c.r32(8);
            c.w32(1, r22);
            c.logic(r22, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 15; continue;
        }
        15 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bab1f); };
            c.w32(1, t23);
            let r24 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r24);
            let Some(t25) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bab25); };
            c.logic(t25 & 0x40u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(1, 0x26eu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bab30); }
            c.w32(0, 0x1u64);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bab36); };
            c.w64(3, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bab3d); };
            c.w64(6, t27);
            return c.call(0x700086ed74, 0x70008bab46, 0x70008bab41);
        }
        18 => {
            return RegfileExit::fault(0x70008bab46);
        }
        19 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bab47); };
            c.w64(0, t28);
            blk = 21; continue;
        }
        20 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bab4d); };
            c.w64(0, t29);
            blk = 21; continue;
        }
        21 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bab51); };
            c.w64(1, t30);
            c.cmp(c.r32(1), c.r32(0), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bab59); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bab60); };
            c.w64(6, t32);
            return c.call(0x70008b23d4, 0x70008bab69, 0x70008bab64);
        }
        23 => {
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bab69); };
            c.w32(1, t33);
            let r34 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r34);
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bab75); };
            c.w64(6, t35);
            blk = 44; continue;
        }
        24 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bab7e); };
            c.w64(6, t36);
            let Some(t37) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bab82); };
            c.w32(1, t37);
            let r38 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r38);
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bab88); };
            c.w32(1, t39);
            let r40 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r40);
            let Some(t41) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bab8e); };
            c.w32(8, t41);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x70008baba1);
        }
        26 => {
            let Some(t42) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008baba2); };
            c.w32(1, t42);
            let r43 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r43);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008babab); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008babb2); };
            c.w64(9, t45);
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008babb5); };
            c.w32(1, t46);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008babbc); };
            c.w64(12, t47);
            let Some(t48) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008babc3); };
            c.cmp(t48, c.r64(11), 64);
            if c.cc_be() { blk = 121; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008babcd); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008babd4); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008babd8); };
            c.w64(1, t49);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008babdf); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008babe3); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008babe9); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008babed); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008babf1); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008babf5); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008babf9); };
            c.w64(1, t50);
            let Some(t51) = c.ld32(c.r64(1)) else { return RegfileExit::fault(0x70008bac00); };
            c.w32(1, t51);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008bac02); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008bac06); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 29; continue;
        }
        29 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t52) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bac1c); };
            c.w32(9, t52);
            let r53 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r53);
            let Some(t54) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bac23); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bac2a); };
            c.w32(11, t56);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8eu64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008bac3d);
        }
        32 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008bac3e); };
            c.w32(7, t57);
            let r58 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r58);
            let Some(t59) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bac48); };
            c.w32(9, t59);
            let r60 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r60);
            let Some(t61) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bac4f); };
            c.logic(t61 & 0x2u64, 8);
            if c.cc_e() { blk = 40; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bac56); };
            c.w64(9, t62);
            let Some(t63) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bac5a); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            let Some(t65) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bac61); };
            c.w32(11, t65);
            c.cmp(c.r16(11), 0x83du64, 16);
            if c.cc_ne() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(11, c.r16(11));
            c.cmp(c.r32(11), 0x83du64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(11, c.r64(9));
            blk = 38; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008bac80);
        }
        37 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bac81); };
            c.w64(11, t66);
            blk = 38; continue;
        }
        38 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008bac88); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bac8a); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bac8c); };
            c.w64(11, t67);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008bac93); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x70008bac9f, 0x70008bac9a);
        }
        39 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bac9f); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008baca3); };
            c.w64(8, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008baca7); };
            c.w64(1, t70);
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(1) & 0x6u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bacb0); }
            let Some(t71) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bacb2); };
            if !c.push64(t71) { return RegfileExit::fault(0x70008bacb2); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x70008bacb5); };
            c.w64(3, t72);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bacc6, 0x70008bacc1);
        }
        42 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bacc6); };
            c.w64(6, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008bacca); };
            c.w64(8, t74);
            blk = 43; continue;
        }
        43 => {
            c.w64(1, c.r64(8));
            blk = 44; continue;
        }
        44 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bacd1); };
            c.w64(9, t75);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008bacd5); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t76) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bacdf); };
            c.w32(11, t76);
            let r77 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r77);
            let Some(t78) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bace6); };
            c.logic(t78 & 0x2u64, 8);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008baced); };
            c.w64(9, t79);
            blk = 47; continue;
        }
        47 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bacf4); };
            c.w64(11, t80);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t81) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bacfe); };
            c.w32(12, t81);
            let r82 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r82);
            let Some(t83) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bad05); };
            c.logic(t83 & 0x2u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bad0d); };
            c.w64(11, t84);
            blk = 50; continue;
        }
        50 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bad14); };
            c.w64(12, t85);
            let Some(t86) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bad18); };
            c.w32(15, t86);
            c.w32(15, c.r32(15));
            let r87 = c.r32(15) & 0x6u64;
            c.w32(15, r87);
            c.logic(r87, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t88) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bad2a); };
            c.w32(0, t88);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(15, c.r64(9));
            blk = 53; continue;
        }
        53 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t89) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bad4b); };
            c.w32(2, t89);
            let r90 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r90);
            let Some(t91) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bad52); };
            c.cmp(t91, 0x80bu64, 16);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t92) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bad5a); };
            c.w32(15, t92);
            let r93 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r93);
            blk = 53; continue;
        }
        56 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 58; continue;
        }
        57 => {
            c.w32(2, 0x1u64);
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t94) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bad71); };
            c.w32(2, t94);
            let r95 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r95);
            let Some(t96) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bad78); };
            c.cmp(t96, 0xbcu64, 16);
            if c.cc_ne() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t97) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bad80); };
            c.w32(15, t97);
            let r98 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r98);
            blk = 53; continue;
        }
        61 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t99) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bad8e); };
            c.w32(3, t99);
            let r100 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r100);
            let Some(t101) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bad95); };
            c.w32(3, t101);
            let r102 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r102);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t103) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bada4); };
            c.w32(11, t103);
            let r104 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r104);
            c.w64(6, c.r64(11));
            blk = 75; continue;
        }
        64 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t105) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008badc5); };
            c.w32(3, t105);
            let r106 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r106);
            let Some(t107) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008badcc); };
            c.cmp(t107, 0x80bu64, 16);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t108) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008badd4); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            blk = 64; continue;
        }
        67 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 69; continue;
        }
        68 => {
            c.w32(3, 0x1u64);
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t110) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008badeb); };
            c.w32(3, t110);
            let r111 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r111);
            let Some(t112) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008badf2); };
            c.cmp(t112, 0xbcu64, 16);
            if c.cc_ne() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t113) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008badfa); };
            c.w32(11, t113);
            let r114 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r114);
            blk = 64; continue;
        }
        72 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t115) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bae09); };
            c.w32(15, t115);
            let r116 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r116);
            let Some(t117) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bae10); };
            c.w32(15, t117);
            let r118 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r118);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bae22); };
            c.w32(11, t119);
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            c.w64(6, c.r64(11));
            blk = 75; continue;
        }
        75 => {
            let Some(t121) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bae2c); };
            c.w32(11, t121);
            let r122 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r122);
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bae33); };
            c.w32(11, t123);
            let r124 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r124);
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bae3a); };
            c.w32(15, t125);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x5u64, 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x70008bae4a);
        }
        77 => {
            let Some(t126) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bae4b); };
            c.w32(11, t126);
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bae4f); };
            c.w64(15, t127);
            let Some(t128) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bae56); };
            c.w64(3, t128);
            c.w64(2, c.r64(3).wrapping_add(0x18));
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bae5d); };
            c.w64(7, t129);
            let Some(t130) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008bae64); };
            c.cmp(t130, c.r64(2), 64);
            if c.cc_be() { blk = 159; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008bae6d); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 79; continue;
        }
        79 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0xd20)) else { return RegfileExit::fault(0x70008bae74); };
            c.w64(3, t131);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008bae7b); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008bae7f); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70008bae83); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008bae87); }
            c.w64(0, c.r64(1));
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008bae8e); }
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70008bae92); }
            blk = 80; continue;
        }
        80 => {
            c.w64(0, c.r64(15));
            return c.call(0x700081f374, 0x70008bae9e, 0x70008bae99);
        }
        81 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bae9e); };
            c.w64(12, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008baea2); };
            c.w64(8, t133);
            blk = 112; continue;
        }
        82 => {
            c.w64(15, c.r64(9));
            blk = 83; continue;
        }
        83 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t134) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baebd); };
            c.w32(3, t134);
            let r135 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r135);
            let Some(t136) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baec4); };
            c.cmp(t136, 0x80bu64, 16);
            if c.cc_ne() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t137) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008baecc); };
            c.w32(15, t137);
            let r138 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r138);
            blk = 83; continue;
        }
        86 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 88; continue;
        }
        87 => {
            c.w32(3, 0x1u64);
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t139) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baee3); };
            c.w32(3, t139);
            let r140 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r140);
            let Some(t141) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baeea); };
            c.cmp(t141, 0xbcu64, 16);
            if c.cc_ne() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t142) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baef2); };
            c.w32(15, t142);
            let r143 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r143);
            blk = 83; continue;
        }
        91 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t144) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baf00); };
            c.w32(0, t144);
            let r145 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r145);
            let Some(t146) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baf07); };
            c.w32(0, t146);
            let r147 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r147);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t148) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008baf15); };
            c.w32(6, t148);
            let r149 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r149);
            blk = 106; continue;
        }
        94 => {
            c.w64(15, c.r64(11));
            blk = 95; continue;
        }
        95 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t150) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baf33); };
            c.w32(3, t150);
            let r151 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r151);
            let Some(t152) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baf3a); };
            c.cmp(t152, 0x80bu64, 16);
            if c.cc_ne() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t153) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008baf42); };
            c.w32(15, t153);
            let r154 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r154);
            blk = 95; continue;
        }
        98 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 100; continue;
        }
        99 => {
            c.w32(3, 0x1u64);
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t155) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baf59); };
            c.w32(3, t155);
            let r156 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r156);
            let Some(t157) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baf60); };
            c.cmp(t157, 0xbcu64, 16);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t158) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baf68); };
            c.w32(15, t158);
            let r159 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r159);
            blk = 95; continue;
        }
        103 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t160) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baf76); };
            c.w32(0, t160);
            let r161 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r161);
            let Some(t162) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008baf7d); };
            c.w32(0, t162);
            let r163 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r163);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t164) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008baf8b); };
            c.w32(6, t164);
            let r165 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r165);
            blk = 106; continue;
        }
        106 => {
            let Some(t166) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008baf92); };
            c.w32(15, t166);
            let r167 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r167);
            let Some(t168) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008baf99); };
            c.w32(15, t168);
            let r169 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r169);
            let Some(t170) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bafa0); };
            c.w32(0, t170);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t171) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bafa7); };
            c.w32(3, t171);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x5u64, 64);
            if c.cc_a() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            return RegfileExit::fault(0x70008bafb5);
        }
        108 => {
            let Some(t172) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bafb6); };
            c.w32(15, t172);
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bafba); };
            c.w64(0, t173);
            let Some(t174) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008bafc1); };
            c.w64(2, t174);
            c.w64(6, c.r64(2).wrapping_add(0x18));
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bafc8); };
            c.w64(7, t175);
            let Some(t176) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008bafcf); };
            c.cmp(t176, c.r64(6), 64);
            if c.cc_be() { blk = 161; continue; }
            blk = 109; continue;
        }
        109 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008bafd8); }
            c.w64(0, c.r64(2).wrapping_add(0x1));
            blk = 110; continue;
        }
        110 => {
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0xdf8)) else { return RegfileExit::fault(0x70008bafdf); };
            c.w64(2, t177);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008bafe6); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008bafe9); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008bafec); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008baff0); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008baff4); }
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008baff7); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008baffb); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r178 = c.r64(1) & c.r64(12);
            c.w64(1, r178);
            c.logic(r178, 64);
            let Some(t179) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008bb00e); };
            c.logic(t179 & 0x4u64, 8);
            if c.cc_ne() { blk = 163; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t180) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bb018); };
            c.w32(1, t180);
            let r181 = c.r64(1) | 0x8u64;
            c.w64(1, r181);
            c.logic(r181, 64);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008bb021); }
            c.w64(0, c.r64(8));
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bb029); };
            c.w64(1, t182);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bb030); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return c.ret(0x18, 0x70008bb037);
        }
        114 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bb03a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bb041); }
            return c.ret(0x0, 0x70008bb043);
        }
        115 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bb049); }
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008bb04a); };
            c.w64(1, t183);
            let Some(t184) = c.ld32(c.r64(1).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008bb051); };
            c.w32(1, t184);
            let r185 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r185);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bb05a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bb05b); }
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bb05c); };
            c.w64(3, t186);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008bb06d, 0x70008bb068);
        }
        116 => {
            return RegfileExit::fault(0x70008bb06d);
        }
        117 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bb070); };
            c.w64(3, t187);
            return c.call(0x700086ed74, 0x70008bb07c, 0x70008bb077);
        }
        118 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb07c); };
            c.w64(6, t188);
            blk = 1; continue;
        }
        119 => {
            c.w32(1, 0x54u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bb08a); }
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008bb08b); };
            c.w64(1, t189);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bb092); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bb093); }
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bb094); };
            c.w64(3, t190);
            c.w32(0, 0x3u64);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb0a0); };
            c.w64(6, t191);
            return c.call(0x700086ed74, 0x70008bb0a9, 0x70008bb0a4);
        }
        120 => {
            return RegfileExit::fault(0x70008bb0a9);
        }
        121 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008bb0af); }
            return c.call(0x70007febf4, 0x70008bb0b8, 0x70008bb0b3);
        }
        122 => {
            c.w64(8, c.r64(0));
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb0bb); };
            c.w64(6, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb0bf); };
            c.w64(1, t193);
            blk = 28; continue;
        }
        123 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bb0c8); }
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008bb0c9); };
            c.w64(3, t194);
            c.w32(0, 0x1u64);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb0d5); };
            c.w64(6, t195);
            return c.call(0x700086ed74, 0x70008bb0de, 0x70008bb0d9);
        }
        124 => {
            blk = 39; continue;
        }
        125 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008bb0e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008bb0e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008bb0eb); }
            c.w64(15, c.r64(11));
            blk = 126; continue;
        }
        126 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 130; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t196) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb101); };
            c.w32(2, t196);
            let r197 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r197);
            let Some(t198) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb108); };
            c.cmp(t198, 0x80bu64, 16);
            if c.cc_ne() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t199) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bb110); };
            c.w32(15, t199);
            let r200 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r200);
            blk = 126; continue;
        }
        129 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 131; continue;
        }
        130 => {
            c.w32(2, 0x1u64);
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t201) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb127); };
            c.w32(2, t201);
            let r202 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r202);
            let Some(t203) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb12e); };
            c.cmp(t203, 0xbcu64, 16);
            if c.cc_ne() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t204) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb136); };
            c.w32(15, t204);
            let r205 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r205);
            blk = 126; continue;
        }
        134 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t206) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb144); };
            c.w32(3, t206);
            let r207 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r207);
            let Some(t208) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb14b); };
            c.w32(3, t208);
            let r209 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r209);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t210) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bb15a); };
            c.w32(9, t210);
            let r211 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r211);
            blk = 149; continue;
        }
        137 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 141; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t212) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb178); };
            c.w32(3, t212);
            let r213 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r213);
            let Some(t214) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb17f); };
            c.cmp(t214, 0x80bu64, 16);
            if c.cc_ne() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t215) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bb187); };
            c.w32(9, t215);
            let r216 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r216);
            blk = 137; continue;
        }
        140 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 142; continue;
        }
        141 => {
            c.w32(3, 0x1u64);
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 145; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t217) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb19e); };
            c.w32(3, t217);
            let r218 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r218);
            let Some(t219) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb1a5); };
            c.cmp(t219, 0xbcu64, 16);
            if c.cc_ne() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t220) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb1ad); };
            c.w32(9, t220);
            let r221 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r221);
            blk = 137; continue;
        }
        145 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 148; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t222) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb1bc); };
            c.w32(15, t222);
            let r223 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r223);
            let Some(t224) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb1c3); };
            c.w32(15, t224);
            let r225 = c.sub(c.r32(15), 0x80cu64, 32);
            c.w32(15, r225);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_a() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t226) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bb1d5); };
            c.w32(9, t226);
            let r227 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r227);
            blk = 149; continue;
        }
        148 => {
            c.w64(9, c.r64(6));
            blk = 149; continue;
        }
        149 => {
            let Some(t228) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb1e1); };
            c.w32(15, t228);
            let r229 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r229);
            let Some(t230) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bb1e8); };
            c.w32(15, t230);
            let r231 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r231);
            let Some(t232) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb1ef); };
            c.w32(3, t232);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008bb1f9); }
            c.cmp(c.r64(3), 0x5u64, 64);
            if c.cc_a() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            return RegfileExit::fault(0x70008bb203);
        }
        151 => {
            let Some(t233) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bb204); };
            c.w32(15, t233);
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bb208); };
            c.w64(3, t234);
            let Some(t235) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bb20f); };
            c.w64(2, t235);
            c.w64(7, c.r64(2).wrapping_add(0x18));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008bb216); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008bb21a); }
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bb21e); };
            c.w64(2, t236);
            let Some(t237) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008bb225); };
            c.cmp(t237, c.r64(7), 64);
            if c.cc_a() { blk = 154; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w32(2, 0x18u64);
            return c.call(0x70007febb4, 0x70008bb234, 0x70008bb22f);
        }
        153 => {
            c.w64(3, c.r64(0));
            let Some(t238) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bb237); };
            c.w64(12, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb23b); };
            c.w64(6, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb23f); };
            c.w64(8, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008bb243); };
            c.w64(1, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bb247); };
            c.w64(11, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008bb24b); };
            c.w64(9, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bb24f); };
            c.w64(0, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bb253); };
            c.w64(15, t245);
            blk = 155; continue;
        }
        154 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008bb259); }
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bb25c); };
            c.w64(3, t246);
            let r247 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r247);
            blk = 155; continue;
        }
        155 => {
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0xd28)) else { return RegfileExit::fault(0x70008bb264); };
            c.w64(2, t248);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008bb26b); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008bb26e); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008bb271); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008bb275); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008bb279); }
            c.w64(1, c.r64(15));
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008bb27f); }
            let Some(t249) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bb282); };
            c.w32(1, t249);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008bb287); }
            c.logic(c.r8(1) & 0x8u64, 8);
            if c.cc_ne() { blk = 158; continue; }
            blk = 156; continue;
        }
        156 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008bb290); }
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x25b8)) else { return RegfileExit::fault(0x70008bb292); };
            c.w64(1, t250);
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x70008bb2a6, 0x70008bb2a1);
        }
        157 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bb2a6); };
            c.w64(12, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb2aa); };
            c.w64(8, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008bb2ae); };
            c.w64(9, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bb2b2); };
            c.w64(3, t254);
            blk = 158; continue;
        }
        158 => {
            c.w64(6, c.r64(9));
            c.w64(15, c.r64(3));
            blk = 80; continue;
        }
        159 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008bb2c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008bb2ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008bb2ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008bb2d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008bb2d6); }
            return c.call(0x70007febb4, 0x70008bb2df, 0x70008bb2da);
        }
        160 => {
            c.w64(15, c.r64(0));
            let Some(t255) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bb2e2); };
            c.w64(12, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb2e6); };
            c.w64(8, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008bb2ea); };
            c.w64(1, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bb2ee); };
            c.w64(9, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008bb2f2); };
            c.w64(6, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb2f6); };
            c.w64(11, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bb2fa); };
            c.w64(0, t261);
            blk = 79; continue;
        }
        161 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008bb308); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008bb30c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008bb310); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008bb314); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008bb318); }
            return c.call(0x70007febb4, 0x70008bb321, 0x70008bb31c);
        }
        162 => {
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008bb321); };
            c.w64(3, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bb325); };
            c.w64(12, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb329); };
            c.w64(8, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb32d); };
            c.w64(15, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008bb331); };
            c.w64(1, t266);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bb335); };
            c.w64(9, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bb339); };
            c.w64(11, t268);
            blk = 110; continue;
        }
        163 => {
            c.w32(0, c.r32(0));
            let r269 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r269);
            c.w64(1, 0xfffffffffffc0000u64);
            let r270 = c.r64(1) & c.r64(0);
            c.w64(1, r270);
            c.logic(r270, 64);
            let Some(t271) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008bb351); };
            c.logic(t271 & 0x2u64, 8);
            if c.cc_e() { blk = 112; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(9, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bb360); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bb361); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008bb36e, 0x70008bb369);
        }
        165 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008bb36e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008bb370); }
            blk = 112; continue;
        }
        166 => {
            return RegfileExit::jump(0x70008bb378);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bb394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bb394); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bb398); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb39e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bb3a5); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb3ac); };
            c.w32(9, t5);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bb3b6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008bb3ba); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008bb3be); }
            c.cmp(c.r64(9), 0xeu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008bb3c8);
        }
        2 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb3c9); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0x3f)) else { return RegfileExit::fault(0x70008bb3d0); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb3d6); };
            c.cmp(t9, 0x83du64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008bb3df); };
            c.w64(12, t10);
            let Some(t11) = c.ld32(c.r64(12)) else { return RegfileExit::fault(0x70008bb3e6); };
            c.cmp(t11, 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(9), 0x97u64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008bb3f6);
        }
        7 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008bb3f7); };
            c.w32(12, t12);
            let Some(t13) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bb3fe); };
            c.cmp(t13, c.r32(12), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x70008bb404); };
            c.w64(11, t14);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t15) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bb40e); };
            c.cmp(t15, c.r32(12), 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            blk = 10; continue;
        }
        9 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(9), 0x79u64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008bb42f);
        }
        13 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008bb430); };
            c.w32(9, t16);
            c.w32(11, 0x8u64);
            c.w64(12, c.r64(11));
            let r17 = c.add(c.r64(12), 0x14u64, 64);
            c.w64(12, r17);
            if c.cc_o() { blk = 80; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bb44a); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bb451); };
            c.w64(2, t19);
            c.w64(7, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bb458); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bb45f); };
            c.cmp(t21, c.r64(7), 64);
            if c.cc_be() { blk = 83; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 83; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.st64(c.r64(15), c.r64(7)) { return RegfileExit::fault(0x70008bb475); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 17; continue;
        }
        17 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008bb47c); }
            c.w32(9, 0xau64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008bb487); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        18 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008bb491); }
            let r22 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r22);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(11), 0x1cu64, 64);
            if c.cc_ne() { blk = 18; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb4a0); };
            c.w32(9, t23);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x70008bb4b1);
        }
        22 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008bb4b2); };
            c.w64(11, t24);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008bb4b9); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008bb4c4);
        }
        24 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bb4c5); };
            c.w64(11, t25);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008bb4cc); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x70008bb4d7);
        }
        26 => {
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008bb4d8); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x70008bb4e3);
        }
        28 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bb4e4); };
            c.w64(11, t26);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70008bb4eb); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008bb4f6);
        }
        30 => {
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008bb4f7); }
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb4fc); };
            c.w32(9, t27);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008bb50f);
        }
        32 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008bb510); };
            c.w32(8, t28);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x10b8)) else { return RegfileExit::fault(0x70008bb517); };
            c.w64(9, t29);
            let Some(t30) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb51e); };
            c.w32(11, t30);
            let r31 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r31);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(13).wrapping_add(0x63f0));
            let Some(t32) = c.ld64(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x70008bb535); };
            c.w64(11, t32);
            blk = 53; continue;
        }
        34 => {
            let Some(t33) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb53e); };
            c.w32(0, t33);
            let r34 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r34);
            let Some(t35) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb545); };
            c.w32(0, t35);
            c.cmp(c.r32(0), 0x5cu64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(0), 0x85u64, 32);
            if c.cc_l() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(0), 0x85u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(0), 0x9du64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(0), 0x9eu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(0), 0x5cu64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(0), 0x5eu64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(0), 0x4au64, 32);
            if c.cc_l() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(0), 0x4au64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(0), 0x53u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(0), 0x48u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(0), 0x49u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb58d); };
            c.w32(11, t36);
            blk = 53; continue;
        }
        47 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008bb593); };
            c.w64(11, t37);
            blk = 53; continue;
        }
        48 => {
            let Some(t38) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb59c); };
            c.w32(11, t38);
            blk = 53; continue;
        }
        49 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008bb5a2); };
            c.w64(11, t39);
            blk = 53; continue;
        }
        50 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008bb5ab); };
            c.w64(11, t40);
            blk = 53; continue;
        }
        51 => {
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bb5b4); };
            c.w32(11, t41);
            blk = 53; continue;
        }
        52 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008bb5ba); };
            c.w64(11, t42);
            blk = 53; continue;
        }
        53 => {
            let Some(t43) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bb5c1); };
            c.w64(0, t43);
            c.w64(2, c.r64(0).wrapping_add(0x1c));
            let Some(t44) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bb5c8); };
            c.cmp(t44, c.r64(2), 64);
            if c.cc_be() { blk = 85; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008bb5d1); }
            let r45 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r45);
            blk = 55; continue;
        }
        55 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008bb5d8); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bb5dc); };
            c.w64(2, t46);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008bb5e3); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008bb5e6); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008bb5e9); };
            c.w64(7, t47);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008bb5f0); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008bb5f3); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008bb5f7); }
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008bb5fb); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x10b0)) else { return RegfileExit::fault(0x70008bb5ff); };
            c.w64(9, t48);
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb606); };
            c.w32(11, t49);
            let r50 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r50);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(1, c.r64(13).wrapping_add(0x63f0));
            let Some(t51) = c.ld64(c.r64(1).wrapping_add(c.r64(11).wrapping_mul(4))) else { return RegfileExit::fault(0x70008bb61d); };
            c.w64(11, t51);
            blk = 76; continue;
        }
        57 => {
            let Some(t52) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb626); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            let Some(t54) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb62d); };
            c.w32(1, t54);
            c.cmp(c.r32(1), 0x5cu64, 32);
            if c.cc_l() { blk = 64; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(1), 0x85u64, 32);
            if c.cc_l() { blk = 62; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(1), 0x85u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(1), 0x9du64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(1), 0x9eu64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(1), 0x5cu64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(1), 0x5eu64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(1), 0x4au64, 32);
            if c.cc_l() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(1), 0x4au64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(1), 0x53u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(1), 0x48u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(1), 0x49u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb679); };
            c.w32(11, t55);
            blk = 76; continue;
        }
        70 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008bb67f); };
            c.w64(11, t56);
            blk = 76; continue;
        }
        71 => {
            let Some(t57) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb688); };
            c.w32(11, t57);
            blk = 76; continue;
        }
        72 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008bb68e); };
            c.w64(11, t58);
            blk = 76; continue;
        }
        73 => {
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008bb697); };
            c.w64(11, t59);
            blk = 76; continue;
        }
        74 => {
            let Some(t60) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bb6a0); };
            c.w32(11, t60);
            blk = 76; continue;
        }
        75 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008bb6a6); };
            c.w64(11, t61);
            blk = 76; continue;
        }
        76 => {
            let Some(t62) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bb6ad); };
            c.w64(1, t62);
            c.w64(6, c.r64(1).wrapping_add(0x1c));
            let Some(t63) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bb6b4); };
            c.cmp(t63, c.r64(6), 64);
            if c.cc_be() { blk = 87; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.st64(c.r64(15), c.r64(6)) { return RegfileExit::fault(0x70008bb6bd); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 78; continue;
        }
        78 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008bb6c4); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008bb6c8); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008bb6cc); }
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008bb6d0); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008bb6d4); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008bb6d8); }
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008bb6dc); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008bb6e0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bb6e2); }
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bb6e3); };
            if !c.push64(t64) { return RegfileExit::fault(0x70008bb6e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008bb6e6); }
            c.w32(0, 0x2u64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb6ef); };
            c.w64(7, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb6f3); };
            c.w64(6, t66);
            return c.call(0x70007f2274, 0x70008bb6fc, 0x70008bb6f7);
        }
        79 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bb6ff); }
            return c.ret(0x0, 0x70008bb700);
        }
        80 => {
            return RegfileExit::fault(0x70008bb701);
        }
        81 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bb702); };
            c.w64(1, t67);
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(1));
            return c.call(0x70008b2ef4, 0x70008bb717, 0x70008bb712);
        }
        82 => {
            blk = 79; continue;
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bb719); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008bb71d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008bb721); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008bb72d, 0x70008bb728);
        }
        84 => {
            c.w64(12, c.r64(0));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bb730); };
            c.w64(0, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb734); };
            c.w64(6, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb738); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bb73c); };
            c.w64(15, t71);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bb743); };
            c.w64(3, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bb74a); };
            c.w64(8, t73);
            c.w32(11, 0x8u64);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bb754); };
            c.w64(9, t74);
            blk = 17; continue;
        }
        85 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008bb762); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008bb766); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008bb76a); }
            return c.call(0x70007febf4, 0x70008bb773, 0x70008bb76e);
        }
        86 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x10b8)) else { return RegfileExit::fault(0x70008bb773); };
            c.w64(9, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb77a); };
            c.w64(6, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb77e); };
            c.w64(1, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008bb782); };
            c.w64(12, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bb786); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bb78a); };
            c.w64(15, t80);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bb791); };
            c.w64(3, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bb798); };
            c.w64(11, t82);
            blk = 55; continue;
        }
        87 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008bb7a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008bb7a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008bb7a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008bb7ad); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008bb7bb, 0x70008bb7b6);
        }
        88 => {
            c.w64(15, c.r64(0));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bb7be); };
            c.w64(0, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bb7c2); };
            c.w64(11, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008bb7c6); };
            c.w64(12, t85);
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x10b0)) else { return RegfileExit::fault(0x70008bb7ca); };
            c.w64(9, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008bb7d1); };
            c.w64(7, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bb7d8); };
            c.w64(2, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bb7df); };
            c.w64(8, t89);
            blk = 78; continue;
        }
        89 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bb7e8); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bb7e9); };
            c.w64(8, t90);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bb7f0); }
            c.w32(0, 0x1u64);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bb7f7); };
            c.w64(7, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb7fb); };
            c.w64(6, t92);
            return c.call(0x70007f2134, 0x70008bb804, 0x70008bb7ff);
        }
        90 => {
            blk = 79; continue;
        }
        91 => {
            return RegfileExit::jump(0x70008bb80c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bb814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bb814); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bb818); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bb819); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bb81a); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bb81f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 24; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bb829); };
            c.cmp(t2, c.r32(2), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bb836); };
            c.w64(1, t3);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb843); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb849); };
            c.w32(8, t6);
            let r7 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r7);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 28; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bb862); };
            c.w64(8, t8);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb870); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bb877); };
            c.w32(9, t11);
            let r12 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r12);
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 28; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bb890); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            let Some(t15) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bb897); };
            c.w32(7, t15);
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bb89b); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            c.logic(c.r8(7) & 0x2u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t18) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb8a8); };
            c.w32(7, t18);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x95u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008bb8bb);
        }
        9 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0x25b)) else { return RegfileExit::fault(0x70008bb8bc); };
            c.w32(7, t19);
            blk = 17; continue;
        }
        10 => {
            c.logic(c.r8(7) & 0x40u64, 8);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb8cb); };
            c.w32(7, t20);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x92u64, 64);
            if c.cc_a() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008bb8de);
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x24f)) else { return RegfileExit::fault(0x70008bb8df); };
            c.w32(7, t21);
            blk = 17; continue;
        }
        14 => {
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bb8e8); };
            c.w32(7, t22);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x93u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x70008bb8fb);
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x253)) else { return RegfileExit::fault(0x70008bb8fc); };
            c.w32(7, t23);
            blk = 17; continue;
        }
        17 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bb903); };
            c.w64(9, t24);
            let Some(t25) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008bb90a); };
            c.w64(11, t25);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bb911); };
            c.w64(15, t26);
            let Some(t27) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bb918); };
            c.cmp(t27, c.r64(12), 64);
            if c.cc_be() { blk = 26; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008bb91d); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 19; continue;
        }
        19 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008bb924); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bb928); };
            c.w64(7, t28);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008bb92f); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008bb933); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008bb937); }
            c.w64(0, c.r64(9));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bb93e); };
            c.w64(1, t29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bb945); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return c.ret(0x18, 0x70008bb94c);
        }
        21 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bb94f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bb956); }
            return c.ret(0x0, 0x70008bb958);
        }
        22 => {
            c.w32(1, 0x52u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bb95e); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x990)) else { return RegfileExit::fault(0x70008bb95f); };
            c.w64(1, t30);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bb966); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bb967); };
            c.w64(3, t31);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bb978, 0x70008bb973);
        }
        23 => {
            return RegfileExit::fault(0x70008bb978);
        }
        24 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bb979); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008bb97d); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bb981); };
            c.w64(3, t32);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bb98f, 0x70008bb98a);
        }
        25 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb98f); };
            c.w64(6, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bb993); };
            c.w64(2, t34);
            blk = 1; continue;
        }
        26 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008bb9a1); }
            return c.call(0x70007febf4, 0x70008bb9aa, 0x70008bb9a5);
        }
        27 => {
            c.w64(9, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bb9ad); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bb9b1); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bb9b5); };
            c.w64(7, t37);
            blk = 19; continue;
        }
        28 => {
            c.w32(1, 0x124u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bb9c3); }
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bb9c4); };
            c.w64(3, t38);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008bb9d5, 0x70008bb9d0);
        }
        29 => {
            return RegfileExit::fault(0x70008bb9d5);
        }
        30 => {
            return RegfileExit::jump(0x70008bb9d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bb9f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bb9f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bb9f8); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008bb9fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008bba02); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008bba06); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bba0a); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bba14); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bba1b); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bba22); };
            c.w32(9, t6);
            let r7 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008bba2e); }
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 71; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bba3f); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bba45); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x638)) else { return RegfileExit::fault(0x70008bba49); };
            c.w64(3, t10);
            c.w64(0, c.r64(8));
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70008bba5b, 0x70008bba56);
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bba5b); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bba68); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bba7d); };
            c.w32(8, t13);
            let r14 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r14);
            let Some(t15) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bba84); };
            c.logic(t15 & 0x2u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bba8f); };
            if !c.push64(t16) { return RegfileExit::fault(0x70008bba8f); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bba92); };
            if !c.push64(t17) { return RegfileExit::fault(0x70008bba92); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bba95); };
            if !c.push64(t18) { return RegfileExit::fault(0x70008bba95); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bba98); };
            c.w64(6, t19);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008bbaac, 0x70008bbaa7);
        }
        8 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bbaac); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008bbabc); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bbac5); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            let Some(t24) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bbacc); };
            c.logic(t24 & 0x10u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bbad3); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t26) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bbae0); };
            c.cmp(t26, 0x41u64, 16);
            if c.cc_e() { blk = 75; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(9, 0x1u64);
            blk = 18; continue;
        }
        17 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        18 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bbb0c); };
            c.w64(3, t27);
            let Some(t28) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bbb10); };
            c.w32(9, t28);
            let r29 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbb17); };
            c.w64(0, t30);
            let Some(t31) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bbb1b); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bbb22); };
            c.w32(9, t33);
            c.logic(c.r32(9) & 0xffffffe0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 123; continue;
        }
        22 => {
            let Some(t34) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbb3c); };
            c.logic(t34 & 0x2u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bbb46); };
            c.w32(9, t35);
            c.cmp(c.r16(9), 0x410u64, 16);
            if c.cc_be() { blk = 50; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bbb57); };
            c.w32(9, t36);
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t37) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bbb68); };
            c.w32(12, t37);
            let r38 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r38);
            c.logic(c.r32(9) & 0xffc00u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 26; continue;
        }
        26 => {
            let r39 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r39);
            let r40 = c.r32(9) & 0x3ffu64;
            c.w32(9, r40);
            c.logic(r40, 32);
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_be() { blk = 39; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t41) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008bbb91); };
            c.w32(15, (t41 as u16 as i16 as i64 as u64));
            let Some(t42) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbb97); };
            c.w32(2, t42);
            let r43 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r43);
            let r44 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r44);
            c.w64(1, c.r64(15));
            blk = 29; continue;
        }
        28 => {
            c.w64(1, c.r64(6));
            blk = 29; continue;
        }
        29 => {
            c.w64(6, c.r64(1));
            let r45 = c.sub(c.r32(6), c.r32(8), 32);
            c.w32(6, r45);
            let r46 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r46);
            let r47 = c.add(c.r32(6), c.r32(8), 32);
            c.w32(6, r47);
            c.w32(7, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t48) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bbbba); };
            c.w32(7, t48);
            let r49 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r49);
            let r50 = c.shr(c.r32(7), 0x9u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r50);
            let r51 = c.r32(7) & 0x3ffu64;
            c.w32(7, r51);
            c.logic(r51, 32);
            c.w32(7, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t52) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bbbd0); };
            c.w32(7, t52);
            let r53 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r53);
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbbd8); };
            c.w32(7, t54);
            let r55 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r55);
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_be() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(8, c.r64(6).wrapping_add(0x1));
            c.w64(6, c.r64(1));
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(8), c.r32(6), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 34; continue;
        }
        33 => {
            let r56 = c.add(c.r32(8), 0x1u64, 32);
            c.w32(8, r56);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(15), c.r32(8), 32);
            if c.cc_l() { blk = 61; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(1, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t57) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bbc04); };
            c.w32(1, t57);
            let r58 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r58);
            let r59 = c.shr(c.r32(1), 0x9u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r59);
            let r60 = c.r32(1) & 0x3ffu64;
            c.w32(1, r60);
            c.logic(r60, 32);
            c.w32(6, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t61) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bbc1a); };
            c.w32(7, t61);
            let r62 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r62);
            let Some(t63) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbc22); };
            c.w32(0, t63);
            let r64 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r64);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 61; continue; }
            blk = 38; continue;
        }
        38 => {
            blk = 79; continue;
        }
        39 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            let r65 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(6, r65);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            let r66 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r66);
            let Some(t67) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bbc57); };
            c.cmp(t67, c.r32(3), 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 40; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 61; continue;
        }
        43 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbc6d); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bbc7a); };
            c.w64(8, t70);
            blk = 45; continue;
        }
        45 => {
            let Some(t71) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bbc81); };
            c.w32(9, t71);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r72 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r72);
            let Some(t73) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbc8f); };
            c.w32(12, t73);
            let r74 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r74);
            let r75 = c.r64(12) & c.r64(9);
            c.w64(12, r75);
            c.logic(r75, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        46 => {
            let r76 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r76);
            let r77 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r77);
            c.w64(2, c.r64(9));
            let r78 = c.r64(2) & c.r64(12);
            c.w64(2, r78);
            c.logic(r78, 64);
            c.w64(12, c.r64(2));
            blk = 47; continue;
        }
        47 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(2).wrapping_mul(4));
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bbcbc); };
            c.w32(6, t79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bbcc1); };
            c.cmp(t80, c.r32(6), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(6), c.r32(3), 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 46; continue;
        }
        50 => {
            c.cmp(c.r16(9), 0xbdu64, 16);
            if c.cc_ne() { blk = 123; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t81) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bbce4); };
            c.logic(t81 & 0x24u64, 8);
            if c.cc_ne() { blk = 123; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbcef); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bbcfc); };
            c.w64(8, t84);
            blk = 54; continue;
        }
        54 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bbd03); };
            c.w32(9, t85);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r86 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r86);
            let Some(t87) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbd11); };
            c.w32(12, t87);
            let r88 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r88);
            let r89 = c.r64(12) & c.r64(9);
            c.w64(12, r89);
            c.logic(r89, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r90 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r90);
            let r91 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r91);
            c.w64(2, c.r64(9));
            let r92 = c.r64(2) & c.r64(12);
            c.w64(2, r92);
            c.logic(r92, 64);
            c.w64(12, c.r64(2));
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(12).wrapping_add(0x5));
            let Some(t93) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bbd36); };
            c.w32(1, t93);
            let r94 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r94);
            let Some(t95) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bbd3e); };
            c.cmp(t95, c.r32(1), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bbd47); };
            c.cmp(t96, c.r32(1), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbd50); };
            c.cmp(t97, c.r32(3), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t98) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbd55); };
            c.w32(8, t98);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ae() { blk = 98; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t99) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bbd68); };
            c.w32(8, t99);
            let r100 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r100);
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bbd6f); };
            c.cmp(t101, c.r32(8), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bbd7c); };
            c.w64(0, t102);
            blk = 62; continue;
        }
        62 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bbd86); }
            return c.ret(0x0, 0x70008bbd87);
        }
        63 => {
            let Some(t103) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bbd88); };
            c.cmp(t103, 0x2u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bbd8e); };
            c.w64(0, t104);
            blk = 62; continue;
        }
        65 => {
            c.w32(8, 0x13au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bbd9d); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x638)) else { return RegfileExit::fault(0x70008bbd9f); };
            c.w64(8, t105);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bbda6); }
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bbda8); };
            if !c.push64(t106) { return RegfileExit::fault(0x70008bbda8); }
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bbdab); };
            c.w64(3, t107);
            c.w32(0, 0x3u64);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bbdb7); };
            c.w64(6, t108);
            return c.call(0x700086ed74, 0x70008bbdc0, 0x70008bbdbb);
        }
        66 => {
            return RegfileExit::fault(0x70008bbdc0);
        }
        67 => {
            c.w32(8, 0xf6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bbdc7); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bbdc9); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x638)) else { return RegfileExit::fault(0x70008bbdca); };
            c.w64(8, t109);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bbdd1); }
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bbdd3); };
            if !c.push64(t110) { return RegfileExit::fault(0x70008bbdd3); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bbdd6); };
            c.w64(3, t111);
            c.w32(0, 0x4u64);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bbde2); };
            c.w64(6, t112);
            return c.call(0x700086ed74, 0x70008bbdeb, 0x70008bbde6);
        }
        68 => {
            return RegfileExit::fault(0x70008bbdeb);
        }
        69 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbdec); };
            c.w64(0, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bbdf0); };
            c.w64(3, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bbdf4); };
            c.w64(1, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bbdf8); };
            c.w64(6, t116);
            return c.call(0x7000820e54, 0x70008bbe01, 0x70008bbdfc);
        }
        70 => {
            blk = 62; continue;
        }
        71 => {
            c.w32(8, 0x130u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bbe09); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x638)) else { return RegfileExit::fault(0x70008bbe0b); };
            c.w64(8, t117);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bbe12); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bbe14); };
            c.w64(3, t118);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bbe25, 0x70008bbe20);
        }
        72 => {
            return RegfileExit::fault(0x70008bbe25);
        }
        73 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bbe26); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bbe2c); };
            c.w64(3, t119);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70008bbe3e, 0x70008bbe39);
        }
        74 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bbe3e); };
            c.w64(6, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bbe42); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bbe46); };
            c.w64(1, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbe4a); };
            c.w64(0, t123);
            blk = 1; continue;
        }
        75 => {
            let Some(t124) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbe53); };
            c.logic(t124 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 16; continue;
        }
        77 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bbe65); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 16; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 17; continue;
        }
        79 => {
            c.w64(8, c.r64(6).wrapping_mul(4));
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bbe85); };
            c.w32(9, t125);
            c.w32(15, c.r32(9));
            c.w64(0, c.r64(9));
            let r126 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r126);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008bbe92); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008bbe96); }
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t127) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bbea0); };
            c.w32(8, t127);
            let r128 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbea8); };
            c.w64(15, t129);
            blk = 94; continue;
        }
        81 => {
            c.w64(8, c.r64(0));
            if let Some(sr) = c.shr(c.r64(8), 0x13u64, 64) { c.w64(8, sr); }
            let Some(t130) = c.ld8(c.r64(11).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008bbeb8); };
            c.w32(9, t130);
            let r131 = c.r64(8) & 0x3ffu64;
            c.w64(8, r131);
            c.logic(r131, 64);
            let r132 = c.r32(15) & 0x380u64;
            c.w32(15, r132);
            c.logic(r132, 32);
            let r133 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r133);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t134) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbedd); };
            c.w32(12, t134);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 86; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbee7); };
            c.w64(15, t135);
            let Some(t136) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbeeb); };
            c.w32(2, t136);
            let r137 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r137);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bbef7); };
            c.w64(2, t138);
            blk = 84; continue;
        }
        84 => {
            let r139 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r139);
            let Some(t140) = c.ld32(c.r64(2).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bbf01); };
            c.w32(8, t140);
            let r141 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r141);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 85; continue;
        }
        85 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bbf13); }
            blk = 88; continue;
        }
        86 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbf21); };
            c.w64(9, t142);
            let Some(t143) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bbf25); };
            c.w32(8, t143);
            let r144 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r144);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bbf2d); }
            c.w64(15, c.r64(9));
            blk = 88; continue;
        }
        88 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bbf36); };
            c.w64(8, t145);
            let Some(t146) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008bbf3d); };
            c.w64(9, t146);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008bbf44); }
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bbf49); };
            c.w64(2, t147);
            let Some(t148) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008bbf50); };
            c.cmp(t148, c.r64(12), 64);
            if c.cc_a() { blk = 91; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008bbf5f, 0x70008bbf5a);
        }
        90 => {
            c.w64(8, c.r64(0));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbf62); };
            c.w64(15, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bbf66); };
            c.w64(3, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bbf6a); };
            c.w64(11, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bbf6e); };
            c.w64(0, t152);
            blk = 92; continue;
        }
        91 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008bbf74); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bbf7b); };
            c.w64(9, t153);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008bbf82); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008bbf86); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008bbf8b); }
            blk = 94; continue;
        }
        93 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bbf93); };
            c.w64(12, t154);
            let Some(t155) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bbf97); };
            c.w32(8, t155);
            let r156 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r156);
            c.w64(15, c.r64(12));
            blk = 94; continue;
        }
        94 => {
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(15));
            blk = 100; continue;
        }
        95 => {
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bbfad); };
            c.w32(9, t157);
            let Some(t158) = c.ld32(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bbfb1); };
            c.w32(12, t158);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            return RegfileExit::fault(0x70008bbfc5);
        }
        97 => {
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bbfc6); };
            c.w32(8, t159);
            let r160 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r160);
            let r161 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r161);
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(12));
            blk = 100; continue;
        }
        98 => {
            return RegfileExit::fault(0x70008bbfd9);
        }
        99 => {
            let Some(t162) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bbfda); };
            c.w32(9, t162);
            let r163 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r163);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 100; continue;
        }
        100 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t164) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bbff4); };
            c.w32(12, t164);
            let Some(t165) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008bbff8); };
            c.cmp(t165, c.r32(12), 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t166) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc005); };
            c.w32(12, t166);
            let r167 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r167);
            let Some(t168) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc00c); };
            c.w32(15, t168);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 115; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w32(2, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 109; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_ne() { blk = 123; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t169) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc036); };
            c.w32(9, t169);
            let Some(t170) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008bc03a); };
            c.cmp(t170, c.r32(9), 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t171) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc047); };
            c.w32(9, t171);
            let r172 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r172);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t173) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc058); };
            c.w32(9, t173);
            let r174 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r174);
            let Some(t175) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc05f); };
            c.cmp(t175, 0x40u64, 16);
            if c.cc_b() { blk = 116; continue; }
            blk = 108; continue;
        }
        108 => {
            blk = 123; continue;
        }
        109 => {
            let Some(t176) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc070); };
            c.w32(9, t176);
            let Some(t177) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008bc074); };
            c.cmp(t177, c.r32(9), 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t178) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bc081); };
            c.w32(9, t178);
            c.w64(12, c.r64(9));
            let r179 = c.r32(12) & 0xc0u64;
            c.w32(12, r179);
            c.logic(r179, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t180) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc09a); };
            c.w32(12, t180);
            let r181 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r181);
            let Some(t182) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bc0a1); };
            c.w32(12, t182);
            let r183 = c.r32(12) & 0x1fu64;
            c.w32(12, r183);
            c.logic(r183, 32);
            let r184 = c.sub(c.r32(12), 0x10u64, 32);
            c.w32(12, r184);
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_a() { blk = 123; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 123; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t185) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bc0c2); };
            c.w32(9, t185);
            let r186 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r186);
            let Some(t187) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bc0c9); };
            c.cmp(t187, c.r32(9), 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc0d2); };
            c.w32(9, t188);
            let Some(t189) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008bc0d6); };
            c.cmp(t189, c.r32(9), 32);
            blk = 116; continue;
        }
        115 => {
            let Some(t190) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc0df); };
            c.w32(9, t190);
            let Some(t191) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008bc0e3); };
            c.cmp(t191, c.r32(9), 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 116; continue;
        }
        116 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 121; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t192) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc0f2); };
            c.logic(t192 & 0x8000000u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w32(8, 0x108u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc106); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bc108); }
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bc109); };
            c.w64(3, t193);
            c.w32(0, 0x2u64);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc115); };
            c.w64(6, t194);
            return c.call(0x700086ed74, 0x70008bc11e, 0x70008bc119);
        }
        119 => {
            return RegfileExit::fault(0x70008bc11e);
        }
        120 => {
            blk = 61; continue;
        }
        121 => {
            c.w32(8, 0x106u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc12a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bc12c); }
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bc12d); };
            c.w64(3, t195);
            c.w32(0, 0x2u64);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc139); };
            c.w64(6, t196);
            return c.call(0x700086ed74, 0x70008bc142, 0x70008bc13d);
        }
        122 => {
            return RegfileExit::fault(0x70008bc142);
        }
        123 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bc143); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bc144); }
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0x2608)) else { return RegfileExit::fault(0x70008bc145); };
            c.w64(3, t197);
            c.w32(0, 0x2u64);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc151); };
            c.w64(6, t198);
            return c.call(0x700086ed74, 0x70008bc15a, 0x70008bc155);
        }
        124 => {
            blk = 61; continue;
        }
        125 => {
            return RegfileExit::jump(0x70008bc160);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bc174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bc174); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bc178); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008bc17e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008bc182); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008bc186); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008bc18a); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bc18e); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 136; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc198); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008bc19f); }
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bc1a3); };
            c.cmp(t4, c.r32(8), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(8, 0x130u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc1b2); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x750)) else { return RegfileExit::fault(0x70008bc1b4); };
            c.w64(8, t5);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc1bb); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bc1bd); };
            c.w64(3, t6);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bc1ce, 0x70008bc1c9);
        }
        3 => {
            return RegfileExit::fault(0x70008bc1ce);
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc1cf); };
            c.w32(0, t7);
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc1d5); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc1dc); };
            c.w32(9, t11);
            let r12 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008bc1e8); }
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 135; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x750)) else { return RegfileExit::fault(0x70008bc1f9); };
            c.w64(3, t13);
            c.w64(0, c.r64(8));
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70008bc20b, 0x70008bc206);
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bc20b); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bc218); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc22d); };
            c.w32(8, t16);
            let r17 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r17);
            let Some(t18) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bc234); };
            c.logic(t18 & 0x2u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc23f); };
            if !c.push64(t19) { return RegfileExit::fault(0x70008bc23f); }
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bc242); };
            if !c.push64(t20) { return RegfileExit::fault(0x70008bc242); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc245); };
            if !c.push64(t21) { return RegfileExit::fault(0x70008bc245); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bc248); };
            if !c.push64(t22) { return RegfileExit::fault(0x70008bc248); }
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bc24b); };
            c.w64(6, t23);
            c.w64(7, c.r64(0));
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008bc25f, 0x70008bc25a);
        }
        11 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bc25f); };
            c.w64(3, t24);
            let Some(t25) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc263); };
            c.w32(8, t25);
            let r26 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc26a); };
            c.w64(9, t27);
            let Some(t28) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc26e); };
            c.w32(11, t28);
            let r29 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r29);
            let Some(t30) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc275); };
            c.w32(8, t30);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008bc27a); }
            c.logic(c.r32(8) & 0xffffffe0u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 151; continue;
        }
        14 => {
            let Some(t31) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc293); };
            c.logic(t31 & 0x2u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t32) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc29d); };
            c.w32(8, t32);
            c.cmp(c.r16(8), 0x410u64, 16);
            if c.cc_be() { blk = 59; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc2ae); };
            c.w32(8, t33);
            c.logic(c.r32(8) & 0x200000u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bc2bf); };
            c.w32(12, t34);
            let r35 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r35);
            c.logic(c.r32(8) & 0xffc00u64, 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 18; continue;
        }
        18 => {
            let r36 = c.shr(c.r32(8), 0xau64, 32).unwrap_or(c.r32(8));
            c.w32(8, r36);
            let r37 = c.r32(8) & 0x3ffu64;
            c.w32(8, r37);
            c.logic(r37, 32);
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_be() { blk = 33; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t38) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008bc2e8); };
            c.w32(15, (t38 as u16 as i16 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc2ee); };
            c.w32(2, t39);
            let r40 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r40);
            let r41 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r41);
            c.w64(6, c.r64(15));
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 21; continue;
        }
        20 => {
            c.w64(6, c.r64(7));
            blk = 21; continue;
        }
        21 => {
            c.w64(7, c.r64(6));
            let r42 = c.sub(c.r32(7), c.r32(1), 32);
            c.w32(7, r42);
            let r43 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r43);
            let r44 = c.add(c.r32(7), c.r32(1), 32);
            c.w32(7, r44);
            c.w32(0, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t45) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bc311); };
            c.w32(0, t45);
            let r46 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r46);
            let r47 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r47);
            let r48 = c.r32(0) & 0x3ffu64;
            c.w32(0, r48);
            c.logic(r48, 32);
            c.w32(0, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bc326); };
            c.w32(0, t49);
            let r50 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r50);
            let Some(t51) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc32e); };
            c.w32(0, t51);
            let r52 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r52);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_be() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w32(0, c.r64(7).wrapping_add(0x1));
            c.w64(1, c.r64(0));
            c.w64(7, c.r64(6));
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 26; continue;
        }
        25 => {
            let r53 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r53);
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(15), c.r32(1), 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(12).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bc355); };
            c.w32(0, t54);
            let r55 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r55);
            let r56 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r56);
            let r57 = c.r32(0) & 0x3ffu64;
            c.w32(0, r57);
            c.logic(r57, 32);
            c.w32(6, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t58) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bc36a); };
            c.w32(7, t58);
            let r59 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc372); };
            c.w32(9, t60);
            let r61 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r61);
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_a() { blk = 37; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc388); };
            c.w64(0, t62);
            blk = 130; continue;
        }
        31 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc391); };
            c.w64(0, t63);
            blk = 130; continue;
        }
        32 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc39a); };
            c.w64(0, t64);
            blk = 130; continue;
        }
        33 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let r65 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(6, r65);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 130; continue; }
            blk = 34; continue;
        }
        34 => {
            let r66 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r66);
            let Some(t67) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bc3b8); };
            c.cmp(t67, c.r32(3), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 34; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 130; continue;
        }
        37 => {
            c.w64(8, c.r64(6).wrapping_mul(4));
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bc3d2); };
            c.w32(9, t68);
            c.w32(15, c.r32(9));
            c.w64(0, c.r64(9));
            let r69 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r69);
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bc3e5); };
            c.w32(8, t70);
            let r71 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc3ed); };
            c.w64(12, t72);
            blk = 50; continue;
        }
        39 => {
            c.w64(8, c.r64(0));
            if let Some(sr) = c.shr(c.r64(8), 0x13u64, 64) { c.w64(8, sr); }
            let Some(t73) = c.ld8(c.r64(11).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008bc3fd); };
            c.w32(9, t73);
            let r74 = c.r64(8) & 0x3ffu64;
            c.w64(8, r74);
            c.logic(r74, 64);
            let r75 = c.r32(15) & 0x380u64;
            c.w32(15, r75);
            c.logic(r75, 32);
            let r76 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r76);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t77) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc422); };
            c.w32(11, t77);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 44; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc42c); };
            c.w64(12, t78);
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc430); };
            c.w32(15, t79);
            let r80 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r80);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bc43e); };
            c.w64(15, t81);
            blk = 42; continue;
        }
        42 => {
            let r82 = c.sub(c.r64(8), c.r64(11), 64);
            c.w64(8, r82);
            let Some(t83) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc448); };
            c.w32(8, t83);
            let r84 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r84);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bc456); }
            blk = 46; continue;
        }
        44 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc464); };
            c.w64(9, t85);
            let Some(t86) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc468); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bc470); }
            c.w64(12, c.r64(9));
            blk = 46; continue;
        }
        46 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bc479); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008bc480); };
            c.w64(9, t89);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bc487); };
            c.w64(15, t90);
            let Some(t91) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bc48e); };
            c.cmp(t91, c.r64(11), 64);
            if c.cc_be() { blk = 138; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008bc497); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 48; continue;
        }
        48 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bc49e); };
            c.w64(9, t92);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008bc4a5); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008bc4a9); }
            blk = 50; continue;
        }
        49 => {
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc4b1); };
            c.w64(12, t93);
            let Some(t94) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc4b5); };
            c.w32(8, t94);
            let r95 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r95);
            blk = 50; continue;
        }
        50 => {
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(12));
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc4c6); };
            c.w64(0, t96);
            blk = 71; continue;
        }
        51 => {
            let Some(t97) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc4cf); };
            c.w32(8, t97);
            let r98 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r98);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bc4dc); };
            c.w64(8, t99);
            blk = 53; continue;
        }
        53 => {
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bc4e3); };
            c.w32(11, t100);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r101 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r101);
            let Some(t102) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc4f1); };
            c.w32(12, t102);
            let r103 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r103);
            let r104 = c.r64(12) & c.r64(11);
            c.w64(12, r104);
            c.logic(r104, 64);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(6));
            blk = 55; continue;
        }
        54 => {
            let r105 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r105);
            let r106 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r106);
            c.w64(2, c.r64(11));
            let r107 = c.r64(2) & c.r64(12);
            c.w64(2, r107);
            c.logic(r107, 64);
            c.w64(12, c.r64(2));
            blk = 55; continue;
        }
        55 => {
            c.w64(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(2).wrapping_mul(4));
            let Some(t108) = c.ld32(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc520); };
            c.w32(7, t108);
            let Some(t109) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bc525); };
            c.cmp(t109, c.r32(7), 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t110) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc536); };
            c.w32(11, t110);
            let Some(t111) = c.ld32(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bc53a); };
            c.w32(12, t111);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ae() { blk = 140; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t112) = c.ld32(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc552); };
            c.w32(8, t112);
            let r113 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r113);
            let r114 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r114);
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(12));
            blk = 71; continue;
        }
        59 => {
            c.cmp(c.r16(8), 0xbdu64, 16);
            if c.cc_ne() { blk = 151; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t115) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bc574); };
            c.logic(t115 & 0x24u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc57f); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bc58c); };
            c.w64(8, t118);
            blk = 63; continue;
        }
        63 => {
            let Some(t119) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bc593); };
            c.w32(11, t119);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r120 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r120);
            let Some(t121) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc5a1); };
            c.w32(12, t121);
            let r122 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r122);
            let r123 = c.r64(12) & c.r64(11);
            c.w64(12, r123);
            c.logic(r123, 64);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        64 => {
            let r124 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r124);
            let r125 = c.add(c.r64(12), c.r64(6), 64);
            c.w64(12, r125);
            c.w64(15, c.r64(11));
            let r126 = c.r64(15) & c.r64(12);
            c.w64(15, r126);
            c.logic(r126, 64);
            c.w64(12, c.r64(15));
            blk = 65; continue;
        }
        65 => {
            c.w64(15, c.r64(12).wrapping_add(0x5));
            let Some(t127) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc5c5); };
            c.w32(2, t127);
            let r128 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bc5cd); };
            c.cmp(t129, c.r32(2), 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bc5da); };
            c.cmp(t130, c.r32(2), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t131) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc5e3); };
            c.cmp(t131, c.r32(3), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t132) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc5e8); };
            c.w32(8, t132);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_ae() { blk = 142; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t133) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc5fb); };
            c.w32(8, t133);
            let r134 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r134);
            let Some(t135) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bc602); };
            c.cmp(t135, c.r32(8), 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t136) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc60f); };
            c.w32(11, t136);
            let r137 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r137);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(10));
            blk = 71; continue;
        }
        71 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 91; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t138) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc629); };
            c.w32(12, t138);
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008bc62d); };
            c.cmp(t139, c.r32(12), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(9, c.r64(11));
            blk = 92; continue;
        }
        74 => {
            let Some(t140) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc63e); };
            c.w32(12, t140);
            let r141 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r141);
            let Some(t142) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc645); };
            c.w32(15, t142);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 89; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 81; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_ne() { blk = 151; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t143) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc66f); };
            c.w32(12, t143);
            let Some(t144) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008bc673); };
            c.cmp(t144, c.r32(12), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t145) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc680); };
            c.w32(12, t145);
            let r146 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r146);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t147) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc691); };
            c.w32(15, t147);
            let r148 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r148);
            let Some(t149) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc699); };
            c.cmp(t149, 0x40u64, 16);
            if c.cc_ae() { blk = 151; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t150) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc6a5); };
            c.w32(9, t150);
            let r151 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r151);
            blk = 92; continue;
        }
        81 => {
            let Some(t152) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc6b2); };
            c.w32(15, t152);
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008bc6b6); };
            c.cmp(t153, c.r32(15), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t154) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bc6c3); };
            c.w32(12, t154);
            c.w64(15, c.r64(12));
            let r155 = c.r32(15) & 0xc0u64;
            c.w32(15, r155);
            c.logic(r155, 32);
            c.cmp(c.r32(15), 0xc0u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t156) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc6dc); };
            c.w32(15, t156);
            let r157 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r157);
            let Some(t158) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bc6e3); };
            c.w32(15, t158);
            let r159 = c.r32(15) & 0x1fu64;
            c.w32(15, r159);
            c.logic(r159, 32);
            let r160 = c.sub(c.r32(15), 0x10u64, 32);
            c.w32(15, r160);
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_a() { blk = 151; continue; }
            blk = 84; continue;
        }
        84 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t161) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bc703); };
            c.w32(12, t161);
            let r162 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r162);
            let Some(t163) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bc70a); };
            c.cmp(t163, c.r32(12), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t164) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc717); };
            c.w32(9, t164);
            let Some(t165) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008bc71c); };
            c.cmp(t165, c.r32(9), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t166) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bc725); };
            c.w32(9, t166);
            let r167 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r167);
            c.w64(12, c.r64(9));
            blk = 88; continue;
        }
        88 => {
            c.w64(9, c.r64(12));
            blk = 92; continue;
        }
        89 => {
            let Some(t168) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc735); };
            c.w32(12, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008bc739); };
            c.cmp(t169, c.r32(12), 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t170) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bc746); };
            c.w32(9, t170);
            let r171 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r171);
            blk = 92; continue;
        }
        91 => {
            c.w64(9, c.r64(11));
            blk = 92; continue;
        }
        92 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008bc752); }
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_e() { blk = 130; continue; }
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t172) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc766); };
            c.w32(12, t172);
            let Some(t173) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008bc76a); };
            c.cmp(t173, c.r32(12), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t174) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bc773); };
            c.w32(8, t174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bc777); };
            c.cmp(t175, c.r32(8), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bc784); };
            c.cmp(t176, c.r32(8), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 97; continue;
        }
        97 => {
            blk = 130; continue;
        }
        98 => {
            c.logic(c.r8(8) & 0x8u64, 8);
            if c.cc_e() { blk = 130; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 100; continue;
        }
        100 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t177) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc7c7); };
            c.w32(8, t177);
            let r178 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r178);
            let Some(t179) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bc7ce); };
            c.cmp(t179, c.r32(8), 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t180) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc7db); };
            c.w32(8, t180);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 109; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_ne() { blk = 128; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t181) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc7f2); };
            c.w32(8, t181);
            let r182 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r182);
            let Some(t183) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc7f9); };
            c.cmp(t183, 0x41u64, 16);
            if c.cc_ne() { blk = 128; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bc805); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008bc806); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008bc80f); };
            c.w64(3, t184);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008bc821, 0x70008bc81c);
        }
        107 => {
            let Some(t185) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bc821); };
            c.cmp(t185, c.r32(0), 32);
            if c.cc_ne() { blk = 148; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc82e); };
            c.w64(0, t186);
            blk = 130; continue;
        }
        109 => {
            c.w64(8, c.r64(9));
            let Some(t187) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc83a); };
            c.w32(9, t187);
            let r188 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r188);
            let Some(t189) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bc841); };
            c.cmp(t189, 0x40u64, 16);
            if c.cc_ae() { blk = 149; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(8));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x70008bc85a, 0x70008bc855);
        }
        111 => {
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bc85a); };
            c.cmp(t190, c.r32(0), 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc867); };
            c.w64(0, t191);
            blk = 130; continue;
        }
        113 => {
            let Some(t192) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc870); };
            c.w32(8, t192);
            let Some(t193) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bc874); };
            c.cmp(t193, c.r32(8), 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 114; continue;
        }
        114 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bc881); }
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bc886); }
            blk = 120; continue;
        }
        115 => {
            let Some(t194) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc88e); };
            c.w32(8, t194);
            let Some(t195) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bc892); };
            c.cmp(t195, c.r32(8), 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 116; continue;
        }
        116 => {
            c.w64(8, c.r64(9));
            let r196 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r196);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bc8a5); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            blk = 120; continue;
        }
        117 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t197) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bc8bb); };
            c.w32(8, t197);
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bc8bf); };
            c.cmp(t198, c.r32(8), 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w64(8, c.r64(0));
            let r199 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r199);
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bc8ce); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 120; continue;
        }
        120 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            if c.cc_e() { blk = 127; continue; }
            blk = 122; continue;
        }
        122 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            if c.cc_e() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 130; continue; }
            blk = 125; continue;
        }
        125 => {
            if c.cc_e() { blk = 128; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 130; continue;
        }
        127 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(8), c.r32(11), 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w32(8, 0x10au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc914); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bc916); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008bc917); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bc919); }
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bc91a); };
            c.w64(3, t200);
            c.w32(0, 0x4u64);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bc926); };
            c.w64(6, t201);
            return c.call(0x700086ed74, 0x70008bc92f, 0x70008bc92a);
        }
        129 => {
            return RegfileExit::fault(0x70008bc92f);
        }
        130 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bc933); }
            return c.ret(0x0, 0x70008bc934);
        }
        131 => {
            c.w32(8, 0xf6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc93b); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bc93d); }
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x750)) else { return RegfileExit::fault(0x70008bc93e); };
            c.w64(8, t202);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc945); }
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bc947); };
            if !c.push64(t203) { return RegfileExit::fault(0x70008bc947); }
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bc94a); };
            c.w64(3, t204);
            c.w32(0, 0x4u64);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bc956); };
            c.w64(6, t205);
            return c.call(0x700086ed74, 0x70008bc95f, 0x70008bc95a);
        }
        132 => {
            return RegfileExit::fault(0x70008bc95f);
        }
        133 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc960); };
            c.w64(0, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bc964); };
            c.w64(3, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc968); };
            c.w64(1, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bc96c); };
            c.w64(2, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bc970); };
            c.w64(6, t210);
            return c.call(0x70008712b4, 0x70008bc979, 0x70008bc974);
        }
        134 => {
            blk = 130; continue;
        }
        135 => {
            return RegfileExit::fault(0x70008bc97b);
        }
        136 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008bc97c); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bc982); };
            c.w64(3, t211);
            c.w64(8, c.r64(0));
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x70008bc994, 0x70008bc98f);
        }
        137 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bc994); };
            c.w64(3, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bc998); };
            c.w64(1, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bc99c); };
            c.w64(2, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bc9a0); };
            c.w64(6, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc9a4); };
            c.w64(0, t216);
            blk = 1; continue;
        }
        138 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008bc9b2); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008bc9b6); }
            return c.call(0x70007febf4, 0x70008bc9c0, 0x70008bc9bb);
        }
        139 => {
            c.w64(8, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bc9c3); };
            c.w64(12, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bc9c7); };
            c.w64(3, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bc9cb); };
            c.w64(0, t219);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008bc9cf); }
            blk = 48; continue;
        }
        140 => {
            return RegfileExit::fault(0x70008bc9d9);
        }
        141 => {
            blk = 151; continue;
        }
        142 => {
            return RegfileExit::fault(0x70008bc9dc);
        }
        143 => {
            blk = 151; continue;
        }
        144 => {
            c.w64(11, c.r64(0));
            let Some(t220) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bc9e2); };
            c.cmp(t220, c.r32(11), 32);
            if c.cc_e() { blk = 147; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w32(8, 0x10cu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bc9f1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bc9f3); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008bc9f4); }
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bc9f6); };
            c.w64(3, t221);
            c.w32(0, 0x3u64);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bca02); };
            c.w64(6, t222);
            return c.call(0x700086ed74, 0x70008bca0b, 0x70008bca06);
        }
        146 => {
            return RegfileExit::fault(0x70008bca0b);
        }
        147 => {
            c.w64(0, c.r64(11));
            blk = 130; continue;
        }
        148 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bca14); };
            c.w64(3, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bca18); };
            c.w64(0, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bca1c); };
            c.w64(9, t225);
            blk = 128; continue;
        }
        149 => {
            c.w64(9, c.r64(8));
            blk = 128; continue;
        }
        150 => {
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bca2d); };
            c.w64(3, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bca31); };
            c.w64(0, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bca35); };
            c.w64(9, t228);
            blk = 128; continue;
        }
        151 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bca3e); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008bca3f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bca41); }
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bca45); }
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x25f8)) else { return RegfileExit::fault(0x70008bca47); };
            c.w64(3, t229);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bca4e); };
            c.w64(6, t230);
            c.w32(0, 0x4u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008bca5f, 0x70008bca5a);
        }
        152 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bca5f); };
            c.w64(0, t231);
            blk = 130; continue;
        }
        153 => {
            return RegfileExit::jump(0x70008bca6c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bca74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bca74); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bca78); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008bca7e); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bca82); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 77; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bca8c); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bca92); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bca99); };
            c.w32(8, t6);
            let r7 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008bcaa5); }
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 75; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcab6); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x760)) else { return RegfileExit::fault(0x70008bcabc); };
            c.w64(3, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bcac3); }
            c.w64(0, c.r64(1));
            return c.call(0x7000870914, 0x70008bcacf, 0x70008bcaca);
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bcacf); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bcadc); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcaf1); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bcaf7); };
            c.logic(t15 & 0x2u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcb01); };
            if !c.push64(t16) { return RegfileExit::fault(0x70008bcb01); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcb04); };
            if !c.push64(t17) { return RegfileExit::fault(0x70008bcb04); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcb07); };
            c.w64(6, t18);
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008bcb1b, 0x70008bcb16);
        }
        8 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008bcb23); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008bcb27); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcb30); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            let Some(t21) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcb37); };
            c.w32(8, t21);
            let r22 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r22);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(8, 0x1u64);
            blk = 12; continue;
        }
        11 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bcb5d); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t24) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcb6a); };
            c.logic(t24 & 0x1u64, 8);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcb70); };
            c.w64(8, t25);
            let Some(t26) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcb74); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            let Some(t28) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcb7b); };
            c.w32(9, t28);
            c.w32(11, c.r64(9).wrapping_sub(0xba));
            c.cmp(c.r32(11), 0x78eu64, 32);
            if c.cc_a() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0xbcu64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(0, c.r64(8));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcba0); };
            c.w64(6, t29);
            return c.call(0x70008bd5f4, 0x70008bcba9, 0x70008bcba4);
        }
        18 => {
            blk = 23; continue;
        }
        19 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bcbab); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x2440)) else { return RegfileExit::fault(0x70008bcbad); };
            c.w64(3, t30);
            c.w32(0, 0x1u64);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcbb9); };
            c.w64(6, t31);
            return c.call(0x700086ed74, 0x70008bcbc2, 0x70008bcbbd);
        }
        20 => {
            blk = 23; continue;
        }
        21 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcbc4); };
            c.w64(8, t32);
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bcbc8); };
            c.w64(0, t33);
            c.w64(9, c.r64(0));
            blk = 23; continue;
        }
        23 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bcbd2); };
            c.cmp(t34, c.r32(0), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcbdf); };
            c.logic(t35 & 0x1u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcbe9); };
            c.w64(0, t36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcbed); };
            c.w32(1, t37);
            let r38 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r38);
            let Some(t39) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcbf3); };
            c.cmp(t39, 0xbau64, 16);
            if c.cc_b() { blk = 80; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcbff); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            let Some(t42) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcc05); };
            c.cmp(t42, 0xbcu64, 16);
            if c.cc_ne() { blk = 29; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcc0d); };
            c.w64(6, t43);
            return c.call(0x70008bca74, 0x70008bcc16, 0x70008bcc11);
        }
        28 => {
            blk = 30; continue;
        }
        29 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bcc18); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x23e0)) else { return RegfileExit::fault(0x70008bcc19); };
            c.w64(3, t44);
            c.w32(0, 0x1u64);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcc25); };
            c.w64(6, t45);
            return c.call(0x700086ed74, 0x70008bcc2e, 0x70008bcc29);
        }
        30 => {
            let Some(t46) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcc2e); };
            c.cmp(c.r32(0), t46, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 48; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t47) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bcc3f); };
            c.cmp(t47, 0x0u64, 32);
            if c.cc_ne() { blk = 46; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t48) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcc49); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bcc4f); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t51) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcc5c); };
            c.w32(1, t51);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 59; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcc70); };
            c.w64(1, t52);
            let Some(t53) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcc74); };
            c.w32(8, t53);
            let r54 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r54);
            let Some(t55) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcc7b); };
            c.cmp(t55, 0x41u64, 16);
            if c.cc_ne() { blk = 59; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bcc87); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bcc88); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008bcc8b); };
            c.w64(3, t56);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bcc9c, 0x70008bcc97);
        }
        38 => {
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bcc9c); };
            c.cmp(t57, c.r32(0), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 59; continue;
        }
        40 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bccae); };
            c.w64(1, t58);
            let Some(t59) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bccb2); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bccb9); };
            c.cmp(t61, 0x40u64, 16);
            if c.cc_ae() { blk = 59; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(1));
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x70008bccd2, 0x70008bcccd);
        }
        42 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bccd2); };
            c.cmp(t62, c.r32(0), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 59; continue;
        }
        44 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcce4); };
            c.w64(1, t63);
            let Some(t64) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcce8); };
            c.w32(8, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bccec); };
            c.cmp(t65, c.r32(8), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 45; continue;
        }
        45 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bccf9); }
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bccfe); }
            blk = 51; continue;
        }
        46 => {
            let Some(t66) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcd05); };
            c.w32(1, t66);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bcd08); };
            c.cmp(t67, c.r32(1), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcd15); };
            c.w64(1, t68);
            let r69 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r69);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bcd1b); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcd28); };
            c.w64(1, t70);
            blk = 51; continue;
        }
        48 => {
            let Some(t71) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bcd2e); };
            c.cmp(t71, 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcd34); };
            c.w64(1, t72);
            let Some(t73) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcd38); };
            c.w32(8, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bcd3c); };
            c.cmp(t74, c.r32(8), 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 50; continue;
        }
        50 => {
            let r75 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r75);
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bcd47); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 51; continue;
        }
        51 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            if c.cc_e() { blk = 58; continue; }
            blk = 53; continue;
        }
        53 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            if c.cc_e() { blk = 59; continue; }
            blk = 55; continue;
        }
        55 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 61; continue; }
            blk = 56; continue;
        }
        56 => {
            if c.cc_e() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 61; continue;
        }
        58 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(1, 0x11cu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bcd8a); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bcd8b); };
            c.w64(3, t76);
            c.w32(0, 0x1u64);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcd97); };
            c.w64(6, t77);
            return c.call(0x700086ed74, 0x70008bcda0, 0x70008bcd9b);
        }
        60 => {
            return RegfileExit::fault(0x70008bcda0);
        }
        61 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcda1); };
            c.w64(0, t78);
            blk = 62; continue;
        }
        62 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bcda8); }
            return c.ret(0x0, 0x70008bcda9);
        }
        63 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcdaa); };
            c.w64(0, t79);
            blk = 62; continue;
        }
        64 => {
            c.w32(1, 0x11au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bcdb5); }
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bcdb6); };
            c.w64(3, t80);
            c.w32(0, 0x1u64);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcdc2); };
            c.w64(6, t81);
            return c.call(0x700086ed74, 0x70008bcdcb, 0x70008bcdc6);
        }
        65 => {
            return RegfileExit::fault(0x70008bcdcb);
        }
        66 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bcdd1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bcdd2); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x760)) else { return RegfileExit::fault(0x70008bcdd3); };
            c.w64(1, t82);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bcdda); }
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcddb); };
            if !c.push64(t83) { return RegfileExit::fault(0x70008bcddb); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bcdde); };
            c.w64(3, t84);
            c.w32(0, 0x4u64);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcdea); };
            c.w64(6, t85);
            return c.call(0x700086ed74, 0x70008bcdf3, 0x70008bcdee);
        }
        67 => {
            return RegfileExit::fault(0x70008bcdf3);
        }
        68 => {
            let Some(t86) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcdf4); };
            c.logic(t86 & 0x1u64, 8);
            if c.cc_e() { blk = 82; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcdfe); };
            c.w64(0, t87);
            let Some(t88) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bce02); };
            c.w32(1, t88);
            let r89 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r89);
            let Some(t90) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bce08); };
            c.cmp(t90, 0xbau64, 16);
            if c.cc_b() { blk = 83; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t91) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bce14); };
            c.w32(1, t91);
            let r92 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r92);
            let Some(t93) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bce1a); };
            c.cmp(t93, 0xbcu64, 16);
            if c.cc_ne() { blk = 73; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bce22); };
            c.w64(6, t94);
            return c.call(0x70008bca74, 0x70008bce2b, 0x70008bce26);
        }
        72 => {
            blk = 62; continue;
        }
        73 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bce30); }
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x23e0)) else { return RegfileExit::fault(0x70008bce31); };
            c.w64(3, t95);
            c.w32(0, 0x1u64);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bce3d); };
            c.w64(6, t96);
            return c.call(0x700086ed74, 0x70008bce46, 0x70008bce41);
        }
        74 => {
            blk = 62; continue;
        }
        75 => {
            c.w32(1, 0x130u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bce50); }
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x760)) else { return RegfileExit::fault(0x70008bce51); };
            c.w64(1, t97);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bce58); }
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bce59); };
            c.w64(3, t98);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bce6a, 0x70008bce65);
        }
        76 => {
            return RegfileExit::fault(0x70008bce6a);
        }
        77 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bce6b); }
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bce6f); };
            c.w64(3, t99);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008bce80, 0x70008bce7b);
        }
        78 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bce80); };
            c.w64(6, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bce84); };
            c.w64(0, t101);
            blk = 1; continue;
        }
        79 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bce8d); };
            c.w64(0, t102);
            blk = 80; continue;
        }
        80 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bce91); };
            c.w64(6, t103);
            return c.call(0x70008a2054, 0x70008bce9a, 0x70008bce95);
        }
        81 => {
            blk = 26; continue;
        }
        82 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bce9f); };
            c.w64(0, t104);
            blk = 83; continue;
        }
        83 => {
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcea3); };
            c.w64(6, t105);
            return c.call(0x70008a2054, 0x70008bceac, 0x70008bcea7);
        }
        84 => {
            blk = 70; continue;
        }
        85 => {
            return RegfileExit::jump(0x70008bceb4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bced4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bced4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bced8); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008bcede); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008bcee2); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bcee6); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bcef0); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcef6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcefd); };
            c.w32(8, t6);
            let r7 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008bcf09); }
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 66; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcf1a); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcf20); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            let Some(t12) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcf27); };
            c.w32(8, t12);
            let r13 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r13);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bcf33); }
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 65; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x788)) else { return RegfileExit::fault(0x70008bcf44); };
            c.w64(3, t14);
            c.w64(0, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x7000870914, 0x70008bcf56, 0x70008bcf51);
        }
        4 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bcf56); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bcf63); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcf78); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bcf7e); };
            c.logic(t19 & 0x2u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcf88); };
            if !c.push64(t20) { return RegfileExit::fault(0x70008bcf88); }
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcf8b); };
            if !c.push64(t21) { return RegfileExit::fault(0x70008bcf8b); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bcf8e); };
            if !c.push64(t22) { return RegfileExit::fault(0x70008bcf8e); }
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bcf91); };
            c.w64(6, t23);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008bcfa5, 0x70008bcfa0);
        }
        9 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bcfa5); };
            c.cmp(t24, c.r32(0), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008bcfb4); };
            c.cmp(t25, c.r32(0), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t26) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcfbd); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            let Some(t28) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bcfc3); };
            c.logic(t28 & 0x10u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bcfc9); };
            c.cmp(t29, c.r32(1), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t30) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bcfd6); };
            c.cmp(t30, 0x41u64, 16);
            if c.cc_e() { blk = 70; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 17; continue;
        }
        16 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bcfe8); };
            c.w64(0, t31);
            blk = 18; continue;
        }
        18 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bcff2); }
            return c.ret(0x0, 0x70008bcff3);
        }
        19 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bcff4); };
            c.w64(3, t32);
            let Some(t33) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bcff8); };
            c.w32(1, t33);
            let r34 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bcffe); };
            c.w64(0, t35);
            let Some(t36) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd002); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd009); };
            c.w32(1, t38);
            c.logic(c.r32(1) & 0xffffffe0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 117; continue;
        }
        22 => {
            let Some(t39) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd020); };
            c.logic(t39 & 0x2u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t40) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd02a); };
            c.w32(1, t40);
            c.cmp(c.r16(1), 0x410u64, 16);
            if c.cc_be() { blk = 50; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t41) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd03a); };
            c.w32(1, t41);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bd04a); };
            c.w32(9, t42);
            let r43 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r43);
            c.logic(c.r32(1) & 0xffc00u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 26; continue;
        }
        26 => {
            let r44 = c.shr(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r44);
            let r45 = c.r32(1) & 0x3ffu64;
            c.w32(1, r45);
            c.logic(r45, 32);
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_be() { blk = 39; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t46) = c.ld16(c.r64(9).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008bd06f); };
            c.w32(11, (t46 as u16 as i16 as i64 as u64));
            let Some(t47) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd074); };
            c.w32(12, t47);
            let r48 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r48);
            let r49 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r49);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(11));
            blk = 29; continue;
        }
        28 => {
            c.w64(2, c.r64(6));
            blk = 29; continue;
        }
        29 => {
            c.w64(6, c.r64(2));
            let r50 = c.sub(c.r32(6), c.r32(15), 32);
            c.w32(6, r50);
            let r51 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r51);
            let r52 = c.add(c.r32(6), c.r32(15), 32);
            c.w32(6, r52);
            c.w32(7, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t53) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bd09c); };
            c.w32(7, t53);
            let r54 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r54);
            let r55 = c.shr(c.r32(7), 0x9u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r55);
            let r56 = c.r32(7) & 0x3ffu64;
            c.w32(7, r56);
            c.logic(r56, 32);
            c.w32(7, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bd0b2); };
            c.w32(7, t57);
            let r58 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r58);
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd0ba); };
            c.w32(7, t59);
            let r60 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r60);
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_be() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(15, c.r64(6).wrapping_add(0x1));
            c.w64(6, c.r64(2));
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(15), c.r32(6), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 34; continue;
        }
        33 => {
            let r61 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r61);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_l() { blk = 61; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(2, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t62) = c.ld32(c.r64(9).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bd0e7); };
            c.w32(2, t62);
            let r63 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r63);
            let r64 = c.shr(c.r32(2), 0x9u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r64);
            let r65 = c.r32(2) & 0x3ffu64;
            c.w32(2, r65);
            c.logic(r65, 32);
            c.w32(6, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t66) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bd0fd); };
            c.w32(7, t66);
            let r67 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r67);
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd105); };
            c.w32(0, t68);
            let r69 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r69);
            c.cmp(c.r32(12), c.r32(0), 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_be() { blk = 61; continue; }
            blk = 38; continue;
        }
        38 => {
            blk = 74; continue;
        }
        39 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let r70 = c.imul2(c.r64(1), 0x3u64, 64);
            c.w64(6, r70);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            let r71 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r71);
            let Some(t72) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bd13a); };
            c.cmp(t72, c.r32(3), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 40; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 61; continue;
        }
        43 => {
            let Some(t73) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd150); };
            c.w32(1, t73);
            let r74 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r74);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bd15b); };
            c.w64(1, t75);
            blk = 45; continue;
        }
        45 => {
            let Some(t76) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bd162); };
            c.w32(9, t76);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r77 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r77);
            let Some(t78) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd170); };
            c.w32(11, t78);
            let r79 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r79);
            let r80 = c.r64(11) & c.r64(9);
            c.w64(11, r80);
            c.logic(r80, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        46 => {
            let r81 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r81);
            let r82 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r82);
            c.w64(15, c.r64(9));
            let r83 = c.r64(15) & c.r64(11);
            c.w64(15, r83);
            c.logic(r83, 64);
            c.w64(11, c.r64(15));
            blk = 47; continue;
        }
        47 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(15).wrapping_mul(4));
            let Some(t84) = c.ld32(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd19d); };
            c.w32(6, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bd1a1); };
            c.cmp(t85, c.r32(6), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(6), c.r32(3), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 46; continue;
        }
        50 => {
            c.cmp(c.r16(1), 0xbdu64, 16);
            if c.cc_ne() { blk = 117; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t86) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bd1c3); };
            c.logic(t86 & 0x24u64, 8);
            if c.cc_ne() { blk = 117; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t87) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd1ce); };
            c.w32(1, t87);
            let r88 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r88);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bd1d9); };
            c.w64(1, t89);
            blk = 54; continue;
        }
        54 => {
            let Some(t90) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bd1e0); };
            c.w32(9, t90);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r91 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r91);
            let Some(t92) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd1ee); };
            c.w32(11, t92);
            let r93 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r93);
            let r94 = c.r64(11) & c.r64(9);
            c.w64(11, r94);
            c.logic(r94, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r95 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r95);
            let r96 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r96);
            c.w64(15, c.r64(9));
            let r97 = c.r64(15) & c.r64(11);
            c.w64(15, r97);
            c.logic(r97, 64);
            c.w64(11, c.r64(15));
            blk = 56; continue;
        }
        56 => {
            c.w64(15, c.r64(11).wrapping_add(0x5));
            let Some(t98) = c.ld32(c.r64(1).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd212); };
            c.w32(2, t98);
            let r99 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r99);
            let Some(t100) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bd21a); };
            c.cmp(t100, c.r32(2), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bd223); };
            c.cmp(t101, c.r32(2), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t102) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd22c); };
            c.cmp(t102, c.r32(3), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t103) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd231); };
            c.w32(1, t103);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_ae() { blk = 92; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t104) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd243); };
            c.w32(1, t104);
            let r105 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r105);
            let Some(t106) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bd249); };
            c.cmp(t106, c.r32(1), 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd256); };
            c.w64(0, t107);
            blk = 18; continue;
        }
        62 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bd267); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bd268); }
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x788)) else { return RegfileExit::fault(0x70008bd269); };
            c.w64(1, t108);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bd270); }
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bd271); };
            if !c.push64(t109) { return RegfileExit::fault(0x70008bd271); }
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bd274); };
            c.w64(3, t110);
            c.w32(0, 0x4u64);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd280); };
            c.w64(6, t111);
            return c.call(0x700086ed74, 0x70008bd289, 0x70008bd284);
        }
        63 => {
            return RegfileExit::fault(0x70008bd289);
        }
        64 => {
            let Some(t112) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008bd28a); };
            c.w64(5, t112);
            let Some(t113) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bd28e); };
            c.w64(0, t113);
            let Some(t114) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bd293); };
            c.w64(3, t114);
            let Some(t115) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008bd298); };
            c.w64(6, t115);
            let r116 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r116);
            return RegfileExit::jump(0x7000820354);
        }
        65 => {
            return RegfileExit::fault(0x70008bd2a6);
        }
        66 => {
            c.w32(1, 0x130u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bd2ac); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x788)) else { return RegfileExit::fault(0x70008bd2ad); };
            c.w64(1, t117);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bd2b4); }
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bd2b5); };
            c.w64(3, t118);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bd2c6, 0x70008bd2c1);
        }
        67 => {
            return RegfileExit::fault(0x70008bd2c6);
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bd2c7); }
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bd2cb); };
            c.w64(1, t119);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008bd2df, 0x70008bd2da);
        }
        69 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd2df); };
            c.w64(6, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bd2e3); };
            c.w64(3, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd2e7); };
            c.w64(0, t122);
            blk = 1; continue;
        }
        70 => {
            let Some(t123) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd2f0); };
            c.logic(t123 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 17; continue;
        }
        72 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bd302); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 17; continue; }
            blk = 73; continue;
        }
        73 => {
            blk = 19; continue;
        }
        74 => {
            c.w64(1, c.r64(6).wrapping_mul(4));
            let Some(t124) = c.ld32(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bd322); };
            c.w32(11, t124);
            c.w32(12, c.r32(11));
            c.w64(15, c.r64(11));
            let r125 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r125);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008bd330); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008bd334); }
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t126) = c.ld32(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bd33e); };
            c.w32(1, t126);
            let r127 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd346); };
            c.w64(0, t128);
            blk = 94; continue;
        }
        76 => {
            c.w64(1, c.r64(15));
            if let Some(sr) = c.shr(c.r64(1), 0x13u64, 64) { c.w64(1, sr); }
            let Some(t129) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008bd356); };
            c.w32(9, t129);
            let r130 = c.r64(1) & 0x3ffu64;
            c.w64(1, r130);
            c.logic(r130, 64);
            let r131 = c.r32(12) & 0x380u64;
            c.w32(12, r131);
            c.logic(r131, 32);
            let r132 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r132);
            c.cmp(c.r32(12), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t133) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd37b); };
            c.w32(11, t133);
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_b() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd385); };
            c.w64(0, t134);
            let Some(t135) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd389); };
            c.w32(12, t135);
            let r136 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r136);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bd396); };
            c.w64(12, t137);
            blk = 79; continue;
        }
        79 => {
            let r138 = c.sub(c.r64(1), c.r64(11), 64);
            c.w64(1, r138);
            let Some(t139) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd3a0); };
            c.w32(1, t139);
            let r140 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r140);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 80; continue;
        }
        80 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bd3b2); }
            blk = 83; continue;
        }
        81 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd3bf); };
            c.w64(9, t141);
            let Some(t142) = c.ld32(c.r64(9).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd3c3); };
            c.w32(1, t142);
            let r143 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r143);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bd3cb); }
            c.w64(0, c.r64(9));
            blk = 83; continue;
        }
        83 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bd3d3); };
            c.w64(1, t144);
            let Some(t145) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008bd3da); };
            c.w64(9, t145);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008bd3e1); }
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bd3e6); };
            c.w64(12, t146);
            let Some(t147) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008bd3ed); };
            c.cmp(t147, c.r64(11), 64);
            if c.cc_a() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008bd3fd, 0x70008bd3f8);
        }
        85 => {
            c.w64(1, c.r64(0));
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd400); };
            c.w64(0, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bd404); };
            c.w64(3, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bd408); };
            c.w64(8, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bd40c); };
            c.w64(15, t151);
            blk = 87; continue;
        }
        86 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x70008bd412); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 87; continue;
        }
        87 => {
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bd419); };
            c.w64(9, t152);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008bd420); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008bd424); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008bd429); }
            blk = 94; continue;
        }
        88 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd430); };
            c.w64(0, t153);
            let Some(t154) = c.ld32(c.r64(0).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd434); };
            c.w32(1, t154);
            let r155 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r155);
            blk = 94; continue;
        }
        89 => {
            let Some(t156) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd43d); };
            c.w32(9, t156);
            let Some(t157) = c.ld32(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bd441); };
            c.w32(11, t157);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            return RegfileExit::fault(0x70008bd455);
        }
        91 => {
            let Some(t158) = c.ld32(c.r64(1).wrapping_add(c.r64(2)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd456); };
            c.w32(1, t158);
            let r159 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r159);
            let r160 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r160);
            c.w64(15, c.r64(11));
            blk = 94; continue;
        }
        92 => {
            return RegfileExit::fault(0x70008bd465);
        }
        93 => {
            let Some(t161) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd466); };
            c.w32(9, t161);
            let r162 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r162);
            c.w64(15, c.r64(9));
            blk = 94; continue;
        }
        94 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 110; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t163) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd47a); };
            c.w32(9, t163);
            let Some(t164) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008bd47e); };
            c.cmp(t164, c.r32(9), 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t165) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd48b); };
            c.w32(9, t165);
            let r166 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r166);
            let Some(t167) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd492); };
            c.w32(11, t167);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 109; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 103; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 117; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t168) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd4bc); };
            c.w32(1, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008bd4bf); };
            c.cmp(t169, c.r32(1), 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t170) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd4cc); };
            c.w32(1, t170);
            let r171 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r171);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 117; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t172) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd4db); };
            c.w32(1, t172);
            let r173 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r173);
            let Some(t174) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd4e1); };
            c.cmp(t174, 0x40u64, 16);
            if c.cc_b() { blk = 110; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 117; continue;
        }
        103 => {
            let Some(t175) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd4f1); };
            c.w32(1, t175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008bd4f4); };
            c.cmp(t176, c.r32(1), 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t177) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bd501); };
            c.w32(1, t177);
            c.w64(9, c.r64(1));
            let r178 = c.r32(9) & 0xc0u64;
            c.w32(9, r178);
            c.logic(r178, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t179) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd519); };
            c.w32(9, t179);
            let r180 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r180);
            let Some(t181) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bd520); };
            c.w32(9, t181);
            let r182 = c.r32(9) & 0x1fu64;
            c.w32(9, r182);
            c.logic(r182, 32);
            let r183 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r183);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_a() { blk = 117; continue; }
            blk = 106; continue;
        }
        106 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 117; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t184) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008bd53f); };
            c.w32(1, t184);
            let r185 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008bd545); };
            c.cmp(t186, c.r32(1), 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t187) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd54e); };
            c.w32(1, t187);
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008bd551); };
            c.cmp(t188, c.r32(1), 32);
            blk = 110; continue;
        }
        109 => {
            let Some(t189) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd55a); };
            c.w32(1, t189);
            let Some(t190) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008bd55d); };
            c.cmp(t190, c.r32(1), 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 110; continue;
        }
        110 => {
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 115; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t191) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd56c); };
            c.logic(t191 & 0x8000000u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(1, 0x120u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bd57f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bd580); }
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bd581); };
            c.w64(3, t192);
            c.w32(0, 0x2u64);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd58d); };
            c.w64(6, t193);
            return c.call(0x700086ed74, 0x70008bd596, 0x70008bd591);
        }
        113 => {
            return RegfileExit::fault(0x70008bd596);
        }
        114 => {
            blk = 61; continue;
        }
        115 => {
            c.w32(1, 0x11eu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bd5a1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bd5a2); }
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bd5a3); };
            c.w64(3, t194);
            c.w32(0, 0x2u64);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd5af); };
            c.w64(6, t195);
            return c.call(0x700086ed74, 0x70008bd5b8, 0x70008bd5b3);
        }
        116 => {
            return RegfileExit::fault(0x70008bd5b8);
        }
        117 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bd5b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bd5ba); }
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0x2600)) else { return RegfileExit::fault(0x70008bd5bb); };
            c.w64(3, t196);
            c.w32(0, 0x2u64);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd5c7); };
            c.w64(6, t197);
            return c.call(0x700086ed74, 0x70008bd5d0, 0x70008bd5cb);
        }
        118 => {
            blk = 61; continue;
        }
        119 => {
            return RegfileExit::jump(0x70008bd5d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bd5f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bd5f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bd5f8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008bd5fe); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bd602); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 58; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd60c); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd612); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd618); };
            c.w32(1, t6);
            let r7 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008bd622); }
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 56; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd632); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x7f0)) else { return RegfileExit::fault(0x70008bd638); };
            c.w64(3, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bd63f); }
            c.w64(0, c.r64(2));
            return c.call(0x7000870914, 0x70008bd64b, 0x70008bd646);
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bd64b); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bd658); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd66d); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            let Some(t15) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bd673); };
            c.logic(t15 & 0x2u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd67d); };
            if !c.push64(t16) { return RegfileExit::fault(0x70008bd67d); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bd680); };
            if !c.push64(t17) { return RegfileExit::fault(0x70008bd680); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd683); };
            c.w64(6, t18);
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008bd697, 0x70008bd692);
        }
        8 => {
            let Some(t19) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd697); };
            c.logic(t19 & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd6a1); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008bd6b0); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd6b9); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            let Some(t24) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bd6bf); };
            c.logic(t24 & 0x10u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bd6c5); };
            c.cmp(t25, c.r32(1), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t26) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd6d2); };
            c.cmp(t26, 0x41u64, 16);
            if c.cc_e() { blk = 60; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(8, 0x1u64);
            blk = 18; continue;
        }
        17 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        18 => {
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008bd6f1); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd6fa); };
            c.w64(0, t27);
            let Some(t28) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd6fe); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd704); };
            c.w32(2, t30);
            c.w32(9, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 25; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd723); };
            c.w64(6, t31);
            return c.call(0x70008bd5f4, 0x70008bd72c, 0x70008bd727);
        }
        22 => {
            blk = 26; continue;
        }
        23 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bd72e); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2440)) else { return RegfileExit::fault(0x70008bd72f); };
            c.w64(3, t32);
            c.w32(0, 0x1u64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd73b); };
            c.w64(6, t33);
            return c.call(0x700086ed74, 0x70008bd744, 0x70008bd73f);
        }
        24 => {
            blk = 26; continue;
        }
        25 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd746); };
            c.w64(0, t34);
            blk = 26; continue;
        }
        26 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd74d); };
            c.cmp(t35, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008bd75c); };
            c.cmp(t36, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd765); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            let Some(t39) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bd76b); };
            c.logic(t39 & 0x10u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bd771); };
            c.cmp(t40, c.r32(2), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t41) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd77e); };
            c.cmp(t41, 0x41u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 34; continue;
        }
        33 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w32(2, 0x1u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 36; continue;
        }
        36 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bd799); };
            c.w64(8, t42);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd7a8); };
            c.w64(0, t43);
            blk = 39; continue;
        }
        39 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bd7b2); }
            return c.ret(0x0, 0x70008bd7b3);
        }
        40 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bd7b4); };
            c.w64(0, t44);
            blk = 39; continue;
        }
        41 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd7c2); };
            c.w64(2, t45);
            blk = 44; continue;
        }
        43 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bd7cb); };
            c.w64(2, t46);
            blk = 44; continue;
        }
        44 => {
            c.w32(1, 0x122u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bd7d7); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bd7d8); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bd7d9); };
            c.w64(3, t47);
            c.w32(0, 0x2u64);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd7e5); };
            c.w64(6, t48);
            return c.call(0x700086ed74, 0x70008bd7ee, 0x70008bd7e9);
        }
        45 => {
            return RegfileExit::fault(0x70008bd7ee);
        }
        46 => {
            c.w32(2, 0xf6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bd7f4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bd7f5); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x7f0)) else { return RegfileExit::fault(0x70008bd7f6); };
            c.w64(2, t49);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bd7fd); }
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bd7fe); };
            if !c.push64(t50) { return RegfileExit::fault(0x70008bd7fe); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bd801); };
            c.w64(3, t51);
            c.w32(0, 0x4u64);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd80d); };
            c.w64(6, t52);
            return c.call(0x700086ed74, 0x70008bd816, 0x70008bd811);
        }
        47 => {
            return RegfileExit::fault(0x70008bd816);
        }
        48 => {
            let Some(t53) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd817); };
            c.logic(t53 & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd81d); };
            c.w64(0, t54);
            let Some(t55) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd821); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd827); };
            c.w32(2, t57);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd844); };
            c.w64(6, t58);
            return c.call(0x70008bd5f4, 0x70008bd84d, 0x70008bd848);
        }
        52 => {
            blk = 39; continue;
        }
        53 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bd852); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x2440)) else { return RegfileExit::fault(0x70008bd853); };
            c.w64(3, t59);
            c.w32(0, 0x1u64);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd85f); };
            c.w64(6, t60);
            return c.call(0x700086ed74, 0x70008bd868, 0x70008bd863);
        }
        54 => {
            blk = 39; continue;
        }
        55 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd86d); };
            c.w64(0, t61);
            blk = 39; continue;
        }
        56 => {
            c.w32(2, 0x130u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bd87e); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x7f0)) else { return RegfileExit::fault(0x70008bd87f); };
            c.w64(2, t62);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bd886); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bd887); };
            c.w64(3, t63);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bd898, 0x70008bd893);
        }
        57 => {
            return RegfileExit::fault(0x70008bd898);
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bd899); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bd89d); };
            c.w64(3, t64);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008bd8ae, 0x70008bd8a9);
        }
        59 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd8ae); };
            c.w64(6, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd8b2); };
            c.w64(0, t66);
            blk = 1; continue;
        }
        60 => {
            let Some(t67) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd8bb); };
            c.logic(t67 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 16; continue;
        }
        62 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bd8cd); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 16; continue; }
            blk = 63; continue;
        }
        63 => {
            blk = 17; continue;
        }
        64 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bd8e5); };
            c.logic(t68 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 65; continue;
        }
        65 => {
            blk = 34; continue;
        }
        66 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bd8f7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 34; continue; }
            blk = 67; continue;
        }
        67 => {
            blk = 35; continue;
        }
        68 => {
            return RegfileExit::jump(0x70008bd910);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bd914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bd914); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bd918); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008bd91e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008bd922); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bd926); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bd930); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd936); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd93c); };
            c.w32(1, t6);
            let r7 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008bd946); }
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 53; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd956); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bd95c); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x960)) else { return RegfileExit::fault(0x70008bd960); };
            c.w64(1, t10);
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(1));
            return c.call(0x7000870914, 0x70008bd972, 0x70008bd96d);
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bd972); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bd97f); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd994); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            let Some(t15) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bd99a); };
            c.logic(t15 & 0x2u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bd9a4); };
            if !c.push64(t16) { return RegfileExit::fault(0x70008bd9a4); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bd9a7); };
            if !c.push64(t17) { return RegfileExit::fault(0x70008bd9a7); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bd9aa); };
            c.w64(6, t18);
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008bd9be, 0x70008bd9b9);
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bd9be); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008bd9d1); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bd9de); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            let Some(t23) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bd9e4); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 29; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bd9ee); };
            c.cmp(t24, c.r32(2), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t25) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bd9fb); };
            c.cmp(t25, 0x41u64, 16);
            if c.cc_e() { blk = 57; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t26) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bda11); };
            c.logic(t26 & 0x1u64, 8);
            if c.cc_e() { blk = 23; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bda17); };
            c.w64(0, t27);
            let Some(t28) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bda1b); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bda21); };
            c.w32(2, t30);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bda3e); };
            c.w64(6, t31);
            return c.call(0x70008bd5f4, 0x70008bda47, 0x70008bda42);
        }
        20 => {
            blk = 24; continue;
        }
        21 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bda49); }
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2440)) else { return RegfileExit::fault(0x70008bda4a); };
            c.w64(3, t32);
            c.w32(0, 0x1u64);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bda56); };
            c.w64(6, t33);
            return c.call(0x700086ed74, 0x70008bda5f, 0x70008bda5a);
        }
        22 => {
            blk = 24; continue;
        }
        23 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bda61); };
            c.w64(0, t34);
            blk = 24; continue;
        }
        24 => {
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bda68); };
            c.cmp(t35, c.r32(0), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bda71); };
            c.w64(0, t36);
            blk = 26; continue;
        }
        26 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bda7b); }
            return c.ret(0x0, 0x70008bda7c);
        }
        27 => {
            c.w32(2, 0x12cu64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bda82); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bda83); };
            c.w64(3, t37);
            c.w32(0, 0x1u64);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bda8f); };
            c.w64(6, t38);
            return c.call(0x700086ed74, 0x70008bda98, 0x70008bda93);
        }
        28 => {
            return RegfileExit::fault(0x70008bda98);
        }
        29 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bda99); };
            c.w64(2, t39);
            let Some(t40) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bda9d); };
            c.cmp(t40, c.r32(2), 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bdaa6); };
            c.w64(0, t41);
            blk = 26; continue;
        }
        31 => {
            c.w32(2, 0x138u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bdab4); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x960)) else { return RegfileExit::fault(0x70008bdab5); };
            c.w64(2, t42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bdabc); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bdabd); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bdac9); };
            c.w64(6, t44);
            return c.call(0x700086ed74, 0x70008bdad2, 0x70008bdacd);
        }
        32 => {
            return RegfileExit::fault(0x70008bdad2);
        }
        33 => {
            c.w32(2, 0xf6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bdad8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bdad9); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x960)) else { return RegfileExit::fault(0x70008bdada); };
            c.w64(2, t45);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bdae1); }
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bdae2); };
            if !c.push64(t46) { return RegfileExit::fault(0x70008bdae2); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bdae5); };
            c.w64(3, t47);
            c.w32(0, 0x4u64);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bdaf1); };
            c.w64(6, t48);
            return c.call(0x700086ed74, 0x70008bdafa, 0x70008bdaf5);
        }
        34 => {
            return RegfileExit::fault(0x70008bdafa);
        }
        35 => {
            let Some(t49) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bdafb); };
            c.logic(t49 & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bdb05); };
            c.w64(2, t50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bdb09); };
            c.cmp(t51, c.r32(2), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bdb17); };
            c.w64(0, t52);
            let Some(t53) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bdb1b); };
            c.w32(2, t53);
            let r54 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r54);
            let Some(t55) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bdb21); };
            c.w32(2, t55);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 43; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bdb3e); };
            c.w64(3, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bdb45); };
            c.w64(6, t57);
            return c.call(0x70008bd914, 0x70008bdb4e, 0x70008bdb49);
        }
        40 => {
            blk = 26; continue;
        }
        41 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bdb53); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x23d0)) else { return RegfileExit::fault(0x70008bdb54); };
            c.w64(3, t58);
            c.w32(0, 0x1u64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bdb60); };
            c.w64(6, t59);
            return c.call(0x700086ed74, 0x70008bdb69, 0x70008bdb64);
        }
        42 => {
            blk = 26; continue;
        }
        43 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bdb6e); };
            c.w64(0, t60);
            blk = 26; continue;
        }
        44 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bdb7f); };
            c.w64(0, t61);
            let Some(t62) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bdb83); };
            c.w32(2, t62);
            let r63 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r63);
            let Some(t64) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bdb89); };
            c.w32(2, t64);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bdba6); };
            c.w64(3, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bdbad); };
            c.w64(6, t66);
            return c.call(0x70008bd914, 0x70008bdbb6, 0x70008bdbb1);
        }
        48 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bdbb6); };
            c.w64(0, t67);
            blk = 26; continue;
        }
        49 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bdbbf); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x23d8)) else { return RegfileExit::fault(0x70008bdbc0); };
            c.w64(3, t68);
            c.w32(0, 0x1u64);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bdbcc); };
            c.w64(6, t69);
            return c.call(0x700086ed74, 0x70008bdbd5, 0x70008bdbd0);
        }
        50 => {
            blk = 26; continue;
        }
        51 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bdbda); };
            c.w64(0, t70);
            blk = 52; continue;
        }
        52 => {
            blk = 26; continue;
        }
        53 => {
            c.w32(2, 0x130u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bdbe8); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x960)) else { return RegfileExit::fault(0x70008bdbe9); };
            c.w64(2, t71);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bdbf0); }
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bdbf1); };
            c.w64(3, t72);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bdc02, 0x70008bdbfd);
        }
        54 => {
            return RegfileExit::fault(0x70008bdc02);
        }
        55 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008bdc03); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bdc07); };
            c.w64(2, t73);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008bdc1b, 0x70008bdc16);
        }
        56 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bdc1b); };
            c.w64(6, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bdc1f); };
            c.w64(3, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bdc23); };
            c.w64(0, t76);
            blk = 1; continue;
        }
        57 => {
            let Some(t77) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bdc2c); };
            c.logic(t77 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 16; continue;
        }
        59 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bdc3e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 16; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 29; continue;
        }
        61 => {
            return RegfileExit::jump(0x70008bdc58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bdc74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bdc74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bdc78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bdc79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bdc7a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bdc7f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 64; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bdc89); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bdc96); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bdc9c); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 74; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bdcb5); };
            c.w64(8, t7);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bdcc3); };
            c.w32(9, t8);
            let r9 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r9);
            let Some(t10) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bdcca); };
            c.w32(9, t10);
            let r11 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r11);
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 74; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bdce3); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bdcea); };
            c.w32(7, t14);
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bdcee); };
            c.w32(9, t15);
            let r16 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r16);
            c.logic(c.r8(7) & 0x2u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bdcfb); };
            c.w32(7, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x95u64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008bdd0e);
        }
        8 => {
            let Some(t18) = c.ld32(c.r64(9).wrapping_add(0x25b)) else { return RegfileExit::fault(0x70008bdd0f); };
            c.w32(7, t18);
            blk = 16; continue;
        }
        9 => {
            c.logic(c.r8(7) & 0x40u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bdd1e); };
            c.w32(7, t19);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x92u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008bdd31);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0x24f)) else { return RegfileExit::fault(0x70008bdd32); };
            c.w32(7, t20);
            blk = 16; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bdd3b); };
            c.w32(7, t21);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x93u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008bdd4e);
        }
        15 => {
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x253)) else { return RegfileExit::fault(0x70008bdd4f); };
            c.w32(7, t22);
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bdd56); };
            c.w64(9, t23);
            let Some(t24) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008bdd5d); };
            c.w64(11, t24);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bdd64); };
            c.w64(15, t25);
            let Some(t26) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bdd6b); };
            c.cmp(t26, c.r64(12), 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008bdd74); }
            let r27 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r27);
            blk = 18; continue;
        }
        18 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008bdd7b); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008bdd7f); };
            c.w64(7, t28);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008bdd86); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008bdd8a); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008bdd8e); }
            let Some(t29) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bdd92); };
            c.w32(1, t29);
            let r30 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r30);
            let Some(t31) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bdd98); };
            c.w32(1, t31);
            let r32 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r32);
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bdd9e); };
            c.w32(7, t33);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x79u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008bddad);
        }
        20 => {
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008bddae); };
            c.w32(7, t34);
            c.w32(8, 0x8u64);
            c.w64(12, c.r64(8));
            let r35 = c.add(c.r64(12), 0xcu64, 64);
            c.w64(12, r35);
            if c.cc_o() { blk = 63; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t36) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008bddc7); };
            c.w64(0, t36);
            c.w64(3, c.r64(0).wrapping_add(c.r64(12)));
            let Some(t37) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bddce); };
            c.cmp(t37, c.r64(3), 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 68; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x70008bdde4); }
            c.w64(12, c.r64(0).wrapping_add(0x1));
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008bddeb); }
            c.w32(7, 0x6u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008bddf5); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        25 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008bddfe); }
            let r38 = c.add(c.r64(8), 0x4u64, 64);
            c.w64(8, r38);
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(8), 0x14u64, 64);
            if c.cc_ne() { blk = 25; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t39) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bde0d); };
            c.w32(7, t39);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            return RegfileExit::fault(0x70008bde1e);
        }
        29 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008bde1f); };
            c.w64(8, t40);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008bde26); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x70008bde31);
        }
        31 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bde32); };
            c.w64(7, t41);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008bde39); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008bde3e); }
            let Some(t42) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bde43); };
            c.w32(7, t42);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xbeu64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x70008bde55);
        }
        33 => {
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008bde56); };
            c.w32(1, t43);
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x10e8)) else { return RegfileExit::fault(0x70008bde5c); };
            c.w64(7, t44);
            let Some(t45) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bde63); };
            c.w32(8, t45);
            let r46 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r46);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(13).wrapping_add(0x63f0));
            let Some(t47) = c.ld64(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4))) else { return RegfileExit::fault(0x70008bde7a); };
            c.w64(8, t47);
            blk = 54; continue;
        }
        35 => {
            let Some(t48) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bde83); };
            c.w32(0, t48);
            let r49 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r49);
            let Some(t50) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bde8a); };
            c.w32(0, t50);
            c.cmp(c.r32(0), 0x5cu64, 32);
            if c.cc_l() { blk = 42; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(0), 0x85u64, 32);
            if c.cc_l() { blk = 40; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(0), 0x85u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(0), 0x9du64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(0), 0x9eu64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(0), 0x5cu64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(0), 0x5eu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(0), 0x4au64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(0), 0x4au64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(0), 0x53u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(0), 0x48u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(0), 0x49u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bded2); };
            c.w32(8, t51);
            blk = 54; continue;
        }
        48 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008bded8); };
            c.w64(8, t52);
            blk = 54; continue;
        }
        49 => {
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bdee1); };
            c.w32(8, t53);
            blk = 54; continue;
        }
        50 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008bdee7); };
            c.w64(8, t54);
            blk = 54; continue;
        }
        51 => {
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008bdef0); };
            c.w64(8, t55);
            blk = 54; continue;
        }
        52 => {
            let Some(t56) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bdef9); };
            c.w32(8, t56);
            blk = 54; continue;
        }
        53 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008bdeff); };
            c.w64(8, t57);
            blk = 54; continue;
        }
        54 => {
            let Some(t58) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008bdf06); };
            c.w64(0, t58);
            c.w64(3, c.r64(0).wrapping_add(0x1c));
            let Some(t59) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bdf0d); };
            c.cmp(t59, c.r64(3), 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 55; continue;
        }
        55 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x70008bdf16); }
            let r60 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r60);
            blk = 56; continue;
        }
        56 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008bdf1d); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bdf20); };
            c.w64(1, t61);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008bdf27); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008bdf2a); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008bdf2d); };
            c.w64(3, t62);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x70008bdf34); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008bdf37); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008bdf3a); }
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008bdf3e); }
            let Some(t63) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bdf42); };
            c.w32(7, t63);
            let r64 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r64);
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bdf48); };
            c.w32(7, t65);
            let r66 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r66);
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bdf4e); };
            c.w32(8, t67);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x96u64, 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            return RegfileExit::fault(0x70008bdf61);
        }
        58 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0x25f)) else { return RegfileExit::fault(0x70008bdf62); };
            c.w32(7, t68);
            let Some(t69) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008bdf68); };
            c.w64(8, t69);
            c.w64(12, c.r64(8).wrapping_add(0x14));
            let Some(t70) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bdf6f); };
            c.cmp(t70, c.r64(12), 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008bdf78); }
            let r71 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r71);
            blk = 60; continue;
        }
        60 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008bdf7f); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008bdf83); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008bdf87); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008bdf8b); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008bdf8f); }
            c.w64(0, c.r64(8));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bdf96); };
            c.w64(1, t72);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bdf9d); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return c.ret(0x18, 0x70008bdfa4);
        }
        62 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bdfa7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bdfae); }
            return c.ret(0x0, 0x70008bdfb0);
        }
        63 => {
            return RegfileExit::fault(0x70008bdfb1);
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bdfb2); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bdfb6); };
            c.w64(3, t73);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bdfc4, 0x70008bdfbf);
        }
        65 => {
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bdfc4); };
            c.w64(6, t74);
            blk = 1; continue;
        }
        66 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bdfd2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008bdfd6); }
            return c.call(0x70007febf4, 0x70008bdfdf, 0x70008bdfda);
        }
        67 => {
            c.w64(11, c.r64(0));
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bdfe2); };
            c.w64(6, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bdfe6); };
            c.w64(9, t76);
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bdfed); };
            c.w64(15, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bdff4); };
            c.w64(8, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bdff8); };
            c.w64(1, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bdffc); };
            c.w64(7, t80);
            blk = 18; continue;
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008be005); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008be009); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008be00d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008be011); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008be01d, 0x70008be018);
        }
        69 => {
            c.w64(12, c.r64(0));
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be020); };
            c.w64(6, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008be024); };
            c.w64(11, t82);
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008be028); };
            c.w64(9, t83);
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008be02f); };
            c.w64(15, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008be036); };
            c.w64(1, t85);
            c.w32(8, 0x8u64);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008be040); };
            c.w64(7, t86);
            blk = 24; continue;
        }
        70 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008be04e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008be052); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008be056); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008be05a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008be05e); }
            return c.call(0x70007febf4, 0x70008be067, 0x70008be062);
        }
        71 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008be067); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be06b); };
            c.w64(6, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008be06f); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008be073); };
            c.w64(9, t90);
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008be07a); };
            c.w64(15, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008be081); };
            c.w64(8, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008be085); };
            c.w64(12, t93);
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x10e8)) else { return RegfileExit::fault(0x70008be089); };
            c.w64(7, t94);
            blk = 56; continue;
        }
        72 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008be09a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008be09e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008be0a2); }
            return c.call(0x70007febb4, 0x70008be0ab, 0x70008be0a6);
        }
        73 => {
            c.w64(8, c.r64(0));
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008be0ae); };
            c.w64(0, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008be0b2); };
            c.w64(11, t96);
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008be0b6); };
            c.w64(1, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be0bd); };
            c.w64(7, t98);
            blk = 60; continue;
        }
        74 => {
            c.w32(1, 0x124u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be0cb); }
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008be0cc); };
            c.w64(1, t99);
            let Some(t100) = c.ld32(c.r64(1).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008be0d3); };
            c.w32(1, t100);
            let r101 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r101);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be0dc); }
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008be0dd); };
            c.w64(3, t102);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008be0ee, 0x70008be0e9);
        }
        75 => {
            return RegfileExit::fault(0x70008be0ee);
        }
        76 => {
            return RegfileExit::jump(0x70008be0f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008be0f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008be0f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008be0f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008be0f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008be0fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008be0ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be105); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008be114);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be115); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008be11b); };
            c.cmp(t5, c.r32(2), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008be124); };
            c.w64(0, t6);
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008be12b); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008be132); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008be139);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008be13c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008be143); }
            return c.ret(0x0, 0x70008be145);
        }
        8 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008be146); };
            c.w64(1, t8);
            if !c.st32(c.r64(6).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008be14d); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008be150); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008be153); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008be156); };
            c.w64(0, t9);
            blk = 5; continue;
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008be15f); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008be163); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008be171, 0x70008be16c);
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be171); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008be178);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008be194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008be194); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008be198); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be19e); };
            c.w32(8, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008be1a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008be1a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008be1aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008be1ae); }
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008be1b2); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld8(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be1bb); };
            c.logic(t3 & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(1, 0x12eu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be1c6); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008be1c7); };
            c.w64(1, t4);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be1ce); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be1cf); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2258)) else { return RegfileExit::fault(0x70008be1d0); };
            c.w64(3, t5);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008be1e1, 0x70008be1dc);
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008be1e1); };
            c.w64(0, t6);
            blk = 4; continue;
        }
        4 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008be1eb); }
            return c.ret(0x0, 0x70008be1ec);
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008be1ed); };
            c.w32(8, t7);
            let r8 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be1f4); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be1fb); };
            c.w32(9, t11);
            let r12 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r12);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008be207); }
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 144; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be218); };
            c.w32(9, t13);
            let r14 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r14);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008be21f); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x70008be223); };
            c.w64(3, t15);
            c.w64(0, c.r64(8));
            c.w64(11, c.r64(3));
            return c.call(0x7000870914, 0x70008be235, 0x70008be230);
        }
        7 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008be235); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008be242); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 9; continue;
        }
        9 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be257); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            let Some(t20) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008be25d); };
            c.logic(t20 & 0x2u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008be267); };
            if !c.push64(t21) { return RegfileExit::fault(0x70008be267); }
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008be26a); };
            if !c.push64(t22) { return RegfileExit::fault(0x70008be26a); }
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008be26d); };
            if !c.push64(t23) { return RegfileExit::fault(0x70008be26d); }
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be270); };
            if !c.push64(t24) { return RegfileExit::fault(0x70008be270); }
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008be273); };
            if !c.push64(t25) { return RegfileExit::fault(0x70008be273); }
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008be276); };
            c.w64(6, t26);
            c.w64(7, c.r64(0));
            c.w32(0, 0x4u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008be28a, 0x70008be285);
        }
        12 => {
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008be28a); };
            c.cmp(t27, c.r32(0), 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008be29d); };
            c.cmp(t28, c.r32(0), 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be2aa); };
            c.w32(1, t29);
            let r30 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r30);
            let Some(t31) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008be2b0); };
            c.logic(t31 & 0x10u64, 8);
            if c.cc_ne() { blk = 138; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008be2ba); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t33) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be2c7); };
            c.cmp(t33, 0x41u64, 16);
            if c.cc_e() { blk = 146; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 20; continue;
        }
        19 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008be2dd); };
            c.w64(1, t34);
            let Some(t35) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be2e1); };
            c.w32(8, t35);
            let r36 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be2e8); };
            c.w64(9, t37);
            let Some(t38) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be2ec); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            let Some(t40) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be2f3); };
            c.w32(8, t40);
            c.logic(c.r32(8) & 0xffffffe0u64, 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 161; continue;
        }
        23 => {
            let Some(t41) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be30d); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t42) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be317); };
            c.w32(8, t42);
            c.cmp(c.r16(8), 0x410u64, 16);
            if c.cc_be() { blk = 66; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008be328); };
            c.w32(8, t43);
            c.logic(c.r32(8) & 0x200000u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008be339); };
            c.w32(12, t44);
            let r45 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r45);
            c.logic(c.r32(8) & 0xffc00u64, 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 27; continue;
        }
        27 => {
            let r46 = c.shr(c.r32(8), 0xau64, 32).unwrap_or(c.r32(8));
            c.w32(8, r46);
            let r47 = c.r32(8) & 0x3ffu64;
            c.w32(8, r47);
            c.logic(r47, 32);
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_be() { blk = 40; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t48) = c.ld16(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008be362); };
            c.w32(15, (t48 as u16 as i16 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be368); };
            c.w32(0, t49);
            let r50 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r50);
            let r51 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r51);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(15));
            blk = 30; continue;
        }
        29 => {
            c.w64(2, c.r64(6));
            blk = 30; continue;
        }
        30 => {
            c.w64(6, c.r64(2));
            let r52 = c.sub(c.r32(6), c.r32(3), 32);
            c.w32(6, r52);
            let r53 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r53);
            let r54 = c.add(c.r32(6), c.r32(3), 32);
            c.w32(6, r54);
            c.w32(7, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008be38b); };
            c.w32(7, t55);
            let r56 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r56);
            let r57 = c.shr(c.r32(7), 0x9u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r57);
            let r58 = c.r32(7) & 0x3ffu64;
            c.w32(7, r58);
            c.logic(r58, 32);
            c.w32(7, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be3a1); };
            c.w32(7, t59);
            let r60 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r60);
            let Some(t61) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be3a9); };
            c.w32(7, t61);
            let r62 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r62);
            c.cmp(c.r32(0), c.r32(7), 32);
            if c.cc_be() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(3, c.r64(6).wrapping_add(0x1));
            c.w64(6, c.r64(2));
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            let r63 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r63);
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(15), c.r32(3), 32);
            if c.cc_l() { blk = 137; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w32(2, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t64) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008be3d1); };
            c.w32(2, t64);
            let r65 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r65);
            let r66 = c.shr(c.r32(2), 0x9u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r66);
            let r67 = c.r32(2) & 0x3ffu64;
            c.w32(2, r67);
            c.logic(r67, 32);
            c.w32(6, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be3e7); };
            c.w32(7, t68);
            let r69 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r69);
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be3ef); };
            c.w32(9, t70);
            let r71 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r71);
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 137; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_be() { blk = 137; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 44; continue;
        }
        40 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let r72 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(6, r72);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 137; continue; }
            blk = 41; continue;
        }
        41 => {
            let r73 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r73);
            let Some(t74) = c.ld32(c.r64(12).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be424); };
            c.cmp(t74, c.r32(1), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 41; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 137; continue;
        }
        44 => {
            c.w64(8, c.r64(6).wrapping_mul(4));
            let Some(t75) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008be43e); };
            c.w32(9, t75);
            c.w32(15, c.r32(9));
            c.w64(0, c.r64(9));
            let r76 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r76);
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t77) = c.ld32(c.r64(12).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008be451); };
            c.w32(8, t77);
            let r78 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be459); };
            c.w64(12, t79);
            blk = 57; continue;
        }
        46 => {
            c.w64(8, c.r64(0));
            if let Some(sr) = c.shr(c.r64(8), 0x13u64, 64) { c.w64(8, sr); }
            let Some(t80) = c.ld8(c.r64(11).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008be469); };
            c.w32(9, t80);
            let r81 = c.r64(8) & 0x3ffu64;
            c.w64(8, r81);
            c.logic(r81, 64);
            let r82 = c.r32(15) & 0x380u64;
            c.w32(15, r82);
            c.logic(r82, 32);
            let r83 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r83);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t84) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be48e); };
            c.w32(11, t84);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 51; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be498); };
            c.w64(12, t85);
            let Some(t86) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be49c); };
            c.w32(15, t86);
            let r87 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r87);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008be4aa); };
            c.w64(15, t88);
            blk = 49; continue;
        }
        49 => {
            let r89 = c.sub(c.r64(8), c.r64(11), 64);
            c.w64(8, r89);
            let Some(t90) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be4b4); };
            c.w32(8, t90);
            let r91 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r91);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 57; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008be4c2); }
            blk = 53; continue;
        }
        51 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be4d0); };
            c.w64(9, t92);
            let Some(t93) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be4d4); };
            c.w32(8, t93);
            let r94 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r94);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008be4dc); }
            c.w64(12, c.r64(9));
            blk = 53; continue;
        }
        53 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008be4e5); };
            c.w64(8, t95);
            let Some(t96) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008be4ec); };
            c.w64(9, t96);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008be4f3); };
            c.w64(15, t97);
            let Some(t98) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008be4fa); };
            c.cmp(t98, c.r64(11), 64);
            if c.cc_be() { blk = 150; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008be503); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 55; continue;
        }
        55 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008be50a); };
            c.w64(9, t99);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008be511); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008be515); }
            blk = 57; continue;
        }
        56 => {
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be51d); };
            c.w64(12, t100);
            let Some(t101) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be521); };
            c.w32(8, t101);
            let r102 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r102);
            blk = 57; continue;
        }
        57 => {
            c.w64(9, c.r64(12));
            blk = 78; continue;
        }
        58 => {
            let Some(t103) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be531); };
            c.w32(8, t103);
            let r104 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r104);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008be53e); };
            c.w64(8, t105);
            blk = 60; continue;
        }
        60 => {
            let Some(t106) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be545); };
            c.w32(11, t106);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r107 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r107);
            let Some(t108) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be553); };
            c.w32(12, t108);
            let r109 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r109);
            let r110 = c.r64(12) & c.r64(11);
            c.w64(12, r110);
            c.logic(r110, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 62; continue;
        }
        61 => {
            let r111 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r111);
            let r112 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r112);
            c.w64(0, c.r64(11));
            let r113 = c.r64(0) & c.r64(12);
            c.w64(0, r113);
            c.logic(r113, 64);
            c.w64(12, c.r64(0));
            blk = 62; continue;
        }
        62 => {
            c.w64(0, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(0).wrapping_mul(4));
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be580); };
            c.w32(2, t114);
            let Some(t115) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008be585); };
            c.cmp(t115, c.r32(2), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be596); };
            c.w32(11, t116);
            let Some(t117) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be59a); };
            c.w32(12, t117);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ae() { blk = 152; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008be5b2); };
            c.w32(8, t118);
            let r119 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r119);
            let r120 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r120);
            c.w64(0, c.r64(12));
            blk = 78; continue;
        }
        66 => {
            c.cmp(c.r16(8), 0xbdu64, 16);
            if c.cc_ne() { blk = 161; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t121) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008be5d1); };
            c.logic(t121 & 0x24u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t122) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be5dc); };
            c.w32(8, t122);
            let r123 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r123);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008be5e9); };
            c.w64(8, t124);
            blk = 70; continue;
        }
        70 => {
            let Some(t125) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be5f0); };
            c.w32(11, t125);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r126 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r126);
            let Some(t127) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be5fe); };
            c.w32(12, t127);
            let r128 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r128);
            let r129 = c.r64(12) & c.r64(11);
            c.w64(12, r129);
            c.logic(r129, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 72; continue;
        }
        71 => {
            let r130 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r130);
            let r131 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r131);
            c.w64(0, c.r64(11));
            let r132 = c.r64(0) & c.r64(12);
            c.w64(0, r132);
            c.logic(r132, 64);
            c.w64(12, c.r64(0));
            blk = 72; continue;
        }
        72 => {
            c.w64(0, c.r64(12).wrapping_add(0x5));
            let Some(t133) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be623); };
            c.w32(3, t133);
            let r134 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r134);
            let Some(t135) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008be62b); };
            c.cmp(t135, c.r32(3), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t136) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008be638); };
            c.cmp(t136, c.r32(3), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t137) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be641); };
            c.cmp(t137, c.r32(1), 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be646); };
            c.w32(8, t138);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_ae() { blk = 154; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t139) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008be659); };
            c.w32(8, t139);
            let r140 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r140);
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008be660); };
            c.cmp(t141, c.r32(8), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t142) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be66d); };
            c.w32(11, t142);
            let r143 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r143);
            c.w64(0, c.r64(11));
            blk = 78; continue;
        }
        78 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t144) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be67f); };
            c.w32(11, t144);
            let Some(t145) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008be683); };
            c.cmp(t145, c.r32(11), 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(9, c.r64(8));
            blk = 99; continue;
        }
        81 => {
            let Some(t146) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be694); };
            c.w32(11, t146);
            let r147 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r147);
            let Some(t148) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be69b); };
            c.w32(12, t148);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 96; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 88; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 161; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be6c6); };
            c.w32(11, t149);
            let Some(t150) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008be6ca); };
            c.cmp(t150, c.r32(11), 32);
            if c.cc_ne() { blk = 161; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t151) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008be6d7); };
            c.w32(11, t151);
            let r152 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r152);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t153) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be6e8); };
            c.w32(12, t153);
            let r154 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r154);
            let Some(t155) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be6ef); };
            c.cmp(t155, 0x40u64, 16);
            if c.cc_ae() { blk = 161; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t156) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be6fc); };
            c.w32(9, t156);
            let r157 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r157);
            blk = 99; continue;
        }
        88 => {
            let Some(t158) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be708); };
            c.w32(12, t158);
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008be70c); };
            c.cmp(t159, c.r32(12), 32);
            if c.cc_ne() { blk = 161; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t160) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008be719); };
            c.w32(11, t160);
            c.w64(12, c.r64(11));
            let r161 = c.r32(12) & 0xc0u64;
            c.w32(12, r161);
            c.logic(r161, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t162) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008be731); };
            c.w32(12, t162);
            let r163 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r163);
            let Some(t164) = c.ld32(c.r64(12).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008be738); };
            c.w32(12, t164);
            let r165 = c.r32(12) & 0x1fu64;
            c.w32(12, r165);
            c.logic(r165, 32);
            let r166 = c.sub(c.r32(12), 0x10u64, 32);
            c.w32(12, r166);
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_a() { blk = 161; continue; }
            blk = 91; continue;
        }
        91 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 161; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t167) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008be759); };
            c.w32(11, t167);
            let r168 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008be760); };
            c.cmp(t169, c.r32(11), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t170) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be76d); };
            c.w32(9, t170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008be771); };
            c.cmp(t171, c.r32(9), 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t172) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008be77a); };
            c.w32(9, t172);
            let r173 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r173);
            c.w64(11, c.r64(9));
            blk = 95; continue;
        }
        95 => {
            c.w64(9, c.r64(11));
            blk = 99; continue;
        }
        96 => {
            let Some(t174) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008be789); };
            c.w32(11, t174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008be78d); };
            c.cmp(t175, c.r32(11), 32);
            if c.cc_ne() { blk = 161; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t176) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008be79a); };
            c.w32(9, t176);
            let r177 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r177);
            blk = 99; continue;
        }
        98 => {
            c.w64(9, c.r64(8));
            blk = 99; continue;
        }
        99 => {
            c.logic(c.r8(0) & 0x20u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 100; continue;
        }
        100 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t178) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be7b4); };
            c.w32(11, t178);
            let Some(t179) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008be7b8); };
            c.cmp(t179, c.r32(11), 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t180) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be7c1); };
            c.w32(8, t180);
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008be7c5); };
            c.cmp(t181, c.r32(8), 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008be7d2); };
            c.cmp(t182, c.r32(8), 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 104; continue;
        }
        104 => {
            blk = 137; continue;
        }
        105 => {
            c.logic(c.r8(0) & 0x8u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008be7ec); };
            c.w64(2, t183);
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 107; continue;
        }
        107 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 124; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 122; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t184) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be818); };
            c.w32(8, t184);
            let r185 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r185);
            let Some(t186) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008be81f); };
            c.cmp(t186, c.r32(8), 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t187) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be82c); };
            c.w32(8, t187);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 116; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_ne() { blk = 135; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be843); };
            c.w32(8, t188);
            let r189 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r189);
            let Some(t190) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be84a); };
            c.cmp(t190, 0x41u64, 16);
            if c.cc_ne() { blk = 135; continue; }
            blk = 113; continue;
        }
        113 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008be856); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008be857); }
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008be859); };
            c.w64(3, t191);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008be86c, 0x70008be867);
        }
        114 => {
            let Some(t192) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008be86c); };
            c.cmp(t192, c.r32(0), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 159; continue;
        }
        116 => {
            let Some(t193) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be87e); };
            c.w32(8, t193);
            let r194 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r194);
            let Some(t195) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008be885); };
            c.cmp(t195, 0x40u64, 16);
            if c.cc_ae() { blk = 135; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(9));
            return c.call(0x70007fb534, 0x70008be89b, 0x70008be896);
        }
        118 => {
            let Some(t196) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008be89b); };
            c.cmp(t196, c.r32(0), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 160; continue;
        }
        120 => {
            let Some(t197) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be8ad); };
            c.w32(8, t197);
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008be8b1); };
            c.cmp(t198, c.r32(8), 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(0, c.r64(2));
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008be8c1); }
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008be8c6); }
            c.w64(2, c.r64(0));
            blk = 127; continue;
        }
        122 => {
            let Some(t199) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be8d1); };
            c.w32(8, t199);
            let Some(t200) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008be8d5); };
            c.cmp(t200, c.r32(8), 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 123; continue;
        }
        123 => {
            let r201 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r201);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008be8e5); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            blk = 127; continue;
        }
        124 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t202) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008be8fb); };
            c.w32(8, t202);
            let Some(t203) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008be8ff); };
            c.cmp(t203, c.r32(8), 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(8, c.r64(2));
            let r204 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r204);
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008be90e); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 127; continue;
        }
        127 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            if c.cc_e() { blk = 134; continue; }
            blk = 129; continue;
        }
        129 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            if c.cc_e() { blk = 135; continue; }
            blk = 131; continue;
        }
        131 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 137; continue; }
            blk = 132; continue;
        }
        132 => {
            if c.cc_e() { blk = 135; continue; }
            blk = 133; continue;
        }
        133 => {
            blk = 137; continue;
        }
        134 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w32(8, 0x132u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008be954); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be956); }
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008be957); };
            c.w64(3, t205);
            c.w32(0, 0x2u64);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008be963); };
            c.w64(6, t206);
            return c.call(0x700086ed74, 0x70008be96c, 0x70008be967);
        }
        136 => {
            return RegfileExit::fault(0x70008be96c);
        }
        137 => {
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008be96d); };
            c.w64(0, t207);
            blk = 4; continue;
        }
        138 => {
            c.w32(1, 0x13au64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be97b); }
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x70008be97c); };
            c.w64(1, t208);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be983); }
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008be984); };
            if !c.push64(t209) { return RegfileExit::fault(0x70008be984); }
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x2258)) else { return RegfileExit::fault(0x70008be987); };
            c.w64(3, t210);
            c.w32(0, 0x3u64);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008be993); };
            c.w64(6, t211);
            return c.call(0x700086ed74, 0x70008be99c, 0x70008be997);
        }
        139 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008be99c); };
            c.w64(0, t212);
            blk = 4; continue;
        }
        140 => {
            c.w32(1, 0xf6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be9aa); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008be9ab); }
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x70008be9ac); };
            c.w64(1, t213);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be9b3); }
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008be9b4); };
            if !c.push64(t214) { return RegfileExit::fault(0x70008be9b4); }
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008be9b7); };
            c.w64(3, t215);
            c.w32(0, 0x4u64);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008be9c3); };
            c.w64(6, t216);
            return c.call(0x700086ed74, 0x70008be9cc, 0x70008be9c7);
        }
        141 => {
            return RegfileExit::fault(0x70008be9cc);
        }
        142 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008be9cd); };
            if !c.push64(t217) { return RegfileExit::fault(0x70008be9cd); }
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008be9d0); };
            if !c.push64(t218) { return RegfileExit::fault(0x70008be9d0); }
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008be9d3); };
            if !c.push64(t219) { return RegfileExit::fault(0x70008be9d3); }
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008be9d6); };
            if !c.push64(t220) { return RegfileExit::fault(0x70008be9d6); }
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2630)) else { return RegfileExit::fault(0x70008be9d9); };
            c.w64(3, t221);
            c.w32(0, 0x4u64);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008be9e5); };
            c.w64(6, t222);
            return c.call(0x700086ed74, 0x70008be9ee, 0x70008be9e9);
        }
        143 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008be9ee); };
            c.w64(0, t223);
            blk = 4; continue;
        }
        144 => {
            c.w32(1, 0x130u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008be9fc); }
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0xa38)) else { return RegfileExit::fault(0x70008be9fd); };
            c.w64(1, t224);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bea04); }
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bea05); };
            c.w64(3, t225);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bea16, 0x70008bea11);
        }
        145 => {
            return RegfileExit::fault(0x70008bea16);
        }
        146 => {
            let Some(t226) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bea17); };
            c.logic(t226 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 138; continue; }
            blk = 147; continue;
        }
        147 => {
            blk = 20; continue;
        }
        148 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bea29); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 20; continue; }
            blk = 149; continue;
        }
        149 => {
            blk = 138; continue;
        }
        150 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008bea46); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008bea4a); }
            return c.call(0x70007febf4, 0x70008bea54, 0x70008bea4f);
        }
        151 => {
            c.w64(8, c.r64(0));
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bea57); };
            c.w64(12, t227);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bea5b); };
            c.w64(1, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bea5f); };
            c.w64(0, t229);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008bea63); }
            blk = 55; continue;
        }
        152 => {
            return RegfileExit::fault(0x70008bea6d);
        }
        153 => {
            blk = 161; continue;
        }
        154 => {
            return RegfileExit::fault(0x70008bea70);
        }
        155 => {
            blk = 161; continue;
        }
        156 => {
            c.w32(8, 0x134u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bea79); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008bea7b); }
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bea7c); };
            c.w64(3, t230);
            c.w32(0, 0x2u64);
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bea88); };
            c.w64(6, t231);
            return c.call(0x700086ed74, 0x70008bea91, 0x70008bea8c);
        }
        157 => {
            return RegfileExit::fault(0x70008bea91);
        }
        158 => {
            blk = 135; continue;
        }
        159 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bea97); };
            c.w64(1, t232);
            blk = 135; continue;
        }
        160 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008beaa0); };
            c.w64(1, t233);
            blk = 135; continue;
        }
        161 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008beaa9); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008beaaa); }
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008beaac); };
            if !c.push64(t234) { return RegfileExit::fault(0x70008beaac); }
            c.w32(1, 0x2u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008beab4); }
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x25f8)) else { return RegfileExit::fault(0x70008beab5); };
            c.w64(3, t235);
            c.w32(0, 0x4u64);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008beac1); };
            c.w64(6, t236);
            return c.call(0x700086ed74, 0x70008beaca, 0x70008beac5);
        }
        162 => {
            blk = 137; continue;
        }
        163 => {
            return RegfileExit::jump(0x70008bead0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bead4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bead4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bead8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008beade); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x70008beae2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008beae6); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008beaea); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008beaf4); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008beafb); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008beb02); };
            c.w32(9, t6);
            let r7 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008beb0e); }
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 93; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008beb1f); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa48)) else { return RegfileExit::fault(0x70008beb25); };
            c.w64(3, t10);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008beb2c); }
            c.w64(0, c.r64(8));
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70008beb3b, 0x70008beb36);
        }
        3 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008beb3b); };
            c.cmp(t11, c.r32(0), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008beb48); };
            c.cmp(t12, c.r32(0), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008beb5d); };
            c.w32(8, t13);
            let r14 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r14);
            let Some(t15) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008beb64); };
            c.logic(t15 & 0x2u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008beb6f); };
            if !c.push64(t16) { return RegfileExit::fault(0x70008beb6f); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008beb72); };
            if !c.push64(t17) { return RegfileExit::fault(0x70008beb72); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008beb75); };
            if !c.push64(t18) { return RegfileExit::fault(0x70008beb75); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008beb78); };
            c.w64(6, t19);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008beb8c, 0x70008beb87);
        }
        8 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008beb8c); };
            c.cmp(t20, c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008beb9c); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008beba5); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            let Some(t24) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008bebac); };
            c.logic(t24 & 0x10u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bebb3); };
            c.cmp(t25, c.r32(8), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t26) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bebc0); };
            c.cmp(t26, 0x41u64, 16);
            if c.cc_e() { blk = 97; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w32(0, 0x1u64);
            blk = 18; continue;
        }
        17 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t27) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bebe9); };
            c.logic(t27 & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bebef); };
            c.w64(0, t28);
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bebf3); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bebfa); };
            c.w32(8, t31);
            c.w32(9, c.r64(8).wrapping_sub(0xba));
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0xbcu64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bec1c); };
            c.w64(6, t32);
            return c.call(0x70008bd5f4, 0x70008bec25, 0x70008bec20);
        }
        23 => {
            blk = 28; continue;
        }
        24 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bec27); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2440)) else { return RegfileExit::fault(0x70008bec28); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bec34); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x70008bec3d, 0x70008bec38);
        }
        25 => {
            blk = 28; continue;
        }
        26 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bec3f); };
            c.w64(0, t35);
            blk = 27; continue;
        }
        27 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bec43); };
            c.w64(0, t36);
            c.w64(8, c.r64(0));
            blk = 28; continue;
        }
        28 => {
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bec4d); };
            c.cmp(t37, c.r32(0), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t38) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bec5a); };
            c.logic(t38 & 0x1u64, 8);
            if c.cc_e() { blk = 101; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bec64); };
            c.w64(0, t39);
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bec68); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            let Some(t42) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bec6f); };
            c.cmp(t42, 0xbau64, 16);
            if c.cc_b() { blk = 102; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bec7c); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            let Some(t45) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bec83); };
            c.cmp(t45, 0xbcu64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bec8c); };
            c.w64(6, t46);
            return c.call(0x70008bca74, 0x70008bec95, 0x70008bec90);
        }
        33 => {
            blk = 35; continue;
        }
        34 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bec97); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x23e0)) else { return RegfileExit::fault(0x70008bec98); };
            c.w64(3, t47);
            c.w32(0, 0x1u64);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008beca4); };
            c.w64(6, t48);
            return c.call(0x700086ed74, 0x70008becad, 0x70008beca8);
        }
        35 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008becad); };
            c.w64(2, t49);
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 36; continue;
        }
        36 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t50) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008becd6); };
            c.w32(8, t50);
            let r51 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r51);
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008becdd); };
            c.cmp(t52, c.r32(8), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t53) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008becea); };
            c.w32(8, t53);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 45; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_ne() { blk = 64; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bed01); };
            c.w32(8, t54);
            let r55 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r55);
            let Some(t56) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bed08); };
            c.cmp(t56, 0x41u64, 16);
            if c.cc_ne() { blk = 64; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bed14); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bed15); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008bed18); };
            c.w64(3, t57);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bed29, 0x70008bed24);
        }
        43 => {
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bed29); };
            c.cmp(t58, c.r32(0), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 64; continue;
        }
        45 => {
            let Some(t59) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bed3b); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bed42); };
            c.cmp(t61, 0x40u64, 16);
            if c.cc_ae() { blk = 64; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x70007fb534, 0x70008bed55, 0x70008bed50);
        }
        47 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bed55); };
            c.cmp(t62, c.r32(0), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 64; continue;
        }
        49 => {
            let Some(t63) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bed67); };
            c.w32(8, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bed6b); };
            c.cmp(t64, c.r32(8), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bed78); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bed7d); }
            blk = 56; continue;
        }
        51 => {
            let Some(t65) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bed84); };
            c.w32(8, t65);
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bed88); };
            c.cmp(t66, c.r32(8), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 52; continue;
        }
        52 => {
            let r67 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r67);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bed97); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 56; continue;
        }
        53 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bedac); };
            c.w32(8, t68);
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008bedb0); };
            c.cmp(t69, c.r32(8), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 55; continue;
        }
        55 => {
            let r70 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r70);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bedbb); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 56; continue;
        }
        56 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            if c.cc_e() { blk = 63; continue; }
            blk = 58; continue;
        }
        58 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            if c.cc_e() { blk = 64; continue; }
            blk = 60; continue;
        }
        60 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 66; continue; }
            blk = 61; continue;
        }
        61 => {
            if c.cc_e() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 66; continue;
        }
        63 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(8, 0x136u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bedff); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bee01); };
            c.w64(3, t71);
            c.w32(0, 0x1u64);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bee0d); };
            c.w64(6, t72);
            return c.call(0x700086ed74, 0x70008bee16, 0x70008bee11);
        }
        65 => {
            return RegfileExit::fault(0x70008bee16);
        }
        66 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bee17); };
            c.w64(0, t73);
            blk = 67; continue;
        }
        67 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bee21); }
            return c.ret(0x0, 0x70008bee22);
        }
        68 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bee23); };
            c.w64(0, t74);
            blk = 67; continue;
        }
        69 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bee2c); };
            c.w64(8, t75);
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bee30); };
            c.cmp(t76, c.r32(8), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bee39); };
            c.w64(0, t77);
            blk = 67; continue;
        }
        71 => {
            c.w32(8, 0x13au64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bee48); }
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0xa48)) else { return RegfileExit::fault(0x70008bee4a); };
            c.w64(8, t78);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bee51); }
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bee53); };
            c.w64(3, t79);
            c.w32(0, 0x2u64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bee5f); };
            c.w64(6, t80);
            return c.call(0x700086ed74, 0x70008bee68, 0x70008bee63);
        }
        72 => {
            return RegfileExit::fault(0x70008bee68);
        }
        73 => {
            c.w32(8, 0xf6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bee6f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bee71); }
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xa48)) else { return RegfileExit::fault(0x70008bee72); };
            c.w64(8, t81);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008bee79); }
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bee7b); };
            if !c.push64(t82) { return RegfileExit::fault(0x70008bee7b); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bee7e); };
            c.w64(3, t83);
            c.w32(0, 0x4u64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bee8a); };
            c.w64(6, t84);
            return c.call(0x700086ed74, 0x70008bee93, 0x70008bee8e);
        }
        74 => {
            return RegfileExit::fault(0x70008bee93);
        }
        75 => {
            let Some(t85) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bee94); };
            c.logic(t85 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008beea0); };
            c.w64(8, t86);
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008beea4); };
            c.cmp(t87, c.r32(8), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008beeb7); };
            c.w64(0, t88);
            let Some(t89) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008beebb); };
            c.w32(8, t89);
            let r90 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r90);
            let Some(t91) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008beec2); };
            c.w32(8, t91);
            c.w32(9, c.r64(8).wrapping_sub(0xba));
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 83; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0xbcu64, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008beee4); };
            c.w64(1, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008beeeb); };
            c.w64(3, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008beeef); };
            c.w64(6, t94);
            return c.call(0x70008bead4, 0x70008beef8, 0x70008beef3);
        }
        80 => {
            blk = 67; continue;
        }
        81 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008beefd); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008beefe); };
            if !c.push64(t95) { return RegfileExit::fault(0x70008beefe); }
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x23e8)) else { return RegfileExit::fault(0x70008bef01); };
            c.w64(3, t96);
            c.w32(0, 0x2u64);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bef0d); };
            c.w64(6, t97);
            return c.call(0x700086ed74, 0x70008bef16, 0x70008bef11);
        }
        82 => {
            blk = 67; continue;
        }
        83 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bef1b); };
            c.w64(0, t98);
            blk = 67; continue;
        }
        84 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bef2d); };
            c.w64(0, t99);
            let Some(t100) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bef31); };
            c.w32(8, t100);
            let r101 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r101);
            let Some(t102) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bef38); };
            c.w32(8, t102);
            c.w32(9, c.r64(8).wrapping_sub(0xba));
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 92; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0xbcu64, 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008bef5a); };
            c.w64(1, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bef61); };
            c.w64(3, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bef65); };
            c.w64(6, t105);
            return c.call(0x70008bead4, 0x70008bef6e, 0x70008bef69);
        }
        88 => {
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bef6e); };
            c.w64(0, t106);
            blk = 67; continue;
        }
        89 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bef77); }
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bef78); };
            if !c.push64(t107) { return RegfileExit::fault(0x70008bef78); }
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x23f0)) else { return RegfileExit::fault(0x70008bef7b); };
            c.w64(3, t108);
            c.w32(0, 0x2u64);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bef87); };
            c.w64(6, t109);
            return c.call(0x700086ed74, 0x70008bef90, 0x70008bef8b);
        }
        90 => {
            blk = 67; continue;
        }
        91 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bef95); };
            c.w64(0, t110);
            blk = 92; continue;
        }
        92 => {
            blk = 67; continue;
        }
        93 => {
            c.w32(8, 0x130u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008befa4); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa48)) else { return RegfileExit::fault(0x70008befa6); };
            c.w64(8, t111);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008befad); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008befaf); };
            c.w64(3, t112);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008befc0, 0x70008befbb);
        }
        94 => {
            return RegfileExit::fault(0x70008befc0);
        }
        95 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008befc1); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008befc7); };
            c.w64(3, t113);
            c.w64(8, c.r64(3));
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70008befd9, 0x70008befd4);
        }
        96 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008befd9); };
            c.w64(6, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008befdd); };
            c.w64(3, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008befe1); };
            c.w64(1, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008befe5); };
            c.w64(0, t117);
            blk = 1; continue;
        }
        97 => {
            let Some(t118) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008befee); };
            c.logic(t118 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 98; continue;
        }
        98 => {
            blk = 16; continue;
        }
        99 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bf000); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 16; continue; }
            blk = 100; continue;
        }
        100 => {
            blk = 17; continue;
        }
        101 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf018); };
            c.w64(0, t119);
            blk = 102; continue;
        }
        102 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bf01c); };
            c.w64(6, t120);
            return c.call(0x70008a2054, 0x70008bf025, 0x70008bf020);
        }
        103 => {
            blk = 31; continue;
        }
        104 => {
            return RegfileExit::jump(0x70008bf02c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf034); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bf038); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf039); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf03a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bf03f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bf049); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf051); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf057); };
            c.w32(2, t5);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, c.r16(2));
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.call(0x70008bd5f4, 0x70008bf087, 0x70008bf082);
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf087); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bf08e); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x70008bf095);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bf098); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bf09f); }
            return c.ret(0x0, 0x70008bf0a1);
        }
        9 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf0a2); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2440)) else { return RegfileExit::fault(0x70008bf0a3); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008bf0b4, 0x70008bf0af);
        }
        10 => {
            blk = 6; continue;
        }
        11 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bf0b6); };
            c.w64(0, t8);
            blk = 6; continue;
        }
        12 => {
            c.w32(2, 0x34u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf0c4); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008bf0c5); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x277)) else { return RegfileExit::fault(0x70008bf0cc); };
            c.w32(2, t10);
            let r11 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r11);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf0d5); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf0d6); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf0e7, 0x70008bf0e2);
        }
        13 => {
            return RegfileExit::fault(0x70008bf0e7);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bf0e8); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bf0ec); };
            c.w64(3, t13);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bf0fa, 0x70008bf0f5);
        }
        15 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf0fa); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x70008bf104);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf114); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bf118); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf119); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf11a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bf11f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bf129); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf135); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf13b); };
            c.w32(2, t5);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, c.r16(2));
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bf166); };
            c.w64(3, t6);
            return c.call(0x70008bd914, 0x70008bf172, 0x70008bf16d);
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf172); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bf179); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x70008bf180);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bf183); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bf18a); }
            return c.ret(0x0, 0x70008bf18c);
        }
        9 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf18d); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x23d0)) else { return RegfileExit::fault(0x70008bf18e); };
            c.w64(3, t8);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008bf19f, 0x70008bf19a);
        }
        10 => {
            blk = 6; continue;
        }
        11 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bf1a1); };
            c.w64(0, t9);
            blk = 6; continue;
        }
        12 => {
            c.w32(2, 0x34u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf1af); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008bf1b0); };
            c.w64(2, t10);
            let Some(t11) = c.ld32(c.r64(2).wrapping_add(0x27b)) else { return RegfileExit::fault(0x70008bf1b7); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf1c0); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf1c1); };
            c.w64(3, t13);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf1d2, 0x70008bf1cd);
        }
        13 => {
            return RegfileExit::fault(0x70008bf1d2);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bf1d3); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bf1d7); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bf1e5, 0x70008bf1e0);
        }
        15 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf1e5); };
            c.w64(6, t15);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x70008bf1f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf1f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf1f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bf1f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf1f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf1fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bf1ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bf209); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf211); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf217); };
            c.w32(2, t5);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.call(0x70008bca74, 0x70008bf239, 0x70008bf234);
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf239); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bf240); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x10, 0x70008bf247);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bf24a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bf251); }
            return c.ret(0x0, 0x70008bf253);
        }
        8 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf254); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x23e0)) else { return RegfileExit::fault(0x70008bf255); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008bf266, 0x70008bf261);
        }
        9 => {
            blk = 5; continue;
        }
        10 => {
            c.w32(2, 0x34u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf26d); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008bf26e); };
            c.w64(2, t8);
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x27f)) else { return RegfileExit::fault(0x70008bf275); };
            c.w32(2, t9);
            let r10 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r10);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf27e); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf27f); };
            c.w64(3, t11);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf290, 0x70008bf28b);
        }
        11 => {
            return RegfileExit::fault(0x70008bf290);
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bf291); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bf295); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bf2a3, 0x70008bf29e);
        }
        13 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf2a3); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008bf2b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf2b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf2b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bf2b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf2b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf2ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bf2bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bf2c9); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf2d5); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf2db); };
            c.w32(2, t5);
            c.w32(7, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 17; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bf2f1); };
            c.w64(3, t6);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bf2fe); };
            c.cmp(t7, c.r32(3), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf307); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf30d); };
            c.w32(7, t10);
            let r11 = c.sub(c.r32(7), 0xbau64, 32);
            c.w32(7, r11);
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(2, c.r16(2));
            c.w32(7, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bf338); };
            c.w64(1, t12);
            return c.call(0x70008bead4, 0x70008bf344, 0x70008bf33f);
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf344); };
            c.w64(1, t13);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bf34b); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x18, 0x70008bf352);
        }
        11 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bf355); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bf35c); }
            return c.ret(0x0, 0x70008bf35e);
        }
        12 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf35f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bf360); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x23e8)) else { return RegfileExit::fault(0x70008bf361); };
            c.w64(3, t14);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf372, 0x70008bf36d);
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008bf374); };
            c.w64(0, t15);
            blk = 9; continue;
        }
        15 => {
            c.w32(2, 0xf8u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf382); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008bf383); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf384); };
            c.w64(3, t16);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf395, 0x70008bf390);
        }
        16 => {
            return RegfileExit::fault(0x70008bf395);
        }
        17 => {
            c.w32(2, 0x34u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf39b); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008bf39c); };
            c.w64(2, t17);
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008bf3a3); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf3ac); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf3ad); };
            c.w64(3, t20);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf3be, 0x70008bf3b9);
        }
        18 => {
            return RegfileExit::fault(0x70008bf3be);
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bf3bf); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bf3c3); };
            c.w64(3, t21);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bf3d1, 0x70008bf3cc);
        }
        20 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf3d1); };
            c.w64(6, t22);
            blk = 1; continue;
        }
        21 => {
            return RegfileExit::jump(0x70008bf3dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf3f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf3f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bf3f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf3f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf3fa); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bf3ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bf403); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008bf410); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(8, c.r64(5));
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bf41d); };
            c.w64(8, t2);
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bf423); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008bf42a); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf438); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf43f); };
            c.w32(9, t6);
            let r7 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r7);
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(9, c.r64(5));
            let r8 = c.add(c.r64(9), 0x18u64, 64);
            c.w64(9, r8);
            let Some(t9) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008bf465); };
            c.w64(9, t9);
            blk = 9; continue;
        }
        8 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bf46b); };
            c.w64(9, t10);
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a1af4, 0x70008bf47a, 0x70008bf475);
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bf47a); };
            c.cmp(t11, 0x2u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bf481); };
            c.w64(7, t12);
            blk = 15; continue;
        }
        12 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bf487); };
            c.cmp(t13, 0x2u64, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(7, c.r64(5));
            let r14 = c.add(c.r64(7), 0x18u64, 64);
            c.w64(7, r14);
            let Some(t15) = c.ld64(c.r64(7).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008bf495); };
            c.w64(7, t15);
            blk = 15; continue;
        }
        14 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008bf49b); };
            c.w64(7, t16);
            blk = 15; continue;
        }
        15 => {
            c.w32(2, 0x2u64);
            c.w64(3, c.r64(0));
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bf4aa); };
            c.w64(0, t17);
            c.w64(1, c.r64(7));
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf4b1); };
            c.w64(6, t18);
            return c.call(0x70008712b4, 0x70008bf4ba, 0x70008bf4b5);
        }
        16 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bf4ba); };
            c.w64(7, t19);
            let r20 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r20);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bf4c5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bf4c6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bf4cc); }
            return c.ret(0x0, 0x70008bf4ce);
        }
        17 => {
            c.w32(7, 0x34u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf4d4); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x9b8)) else { return RegfileExit::fault(0x70008bf4d5); };
            c.w64(7, t21);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf4dc); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf4dd); };
            c.w64(3, t22);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf4ee, 0x70008bf4e9);
        }
        18 => {
            return RegfileExit::fault(0x70008bf4ee);
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008bf4ef); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bf4f3); };
            c.w64(3, t23);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008bf504, 0x70008bf4ff);
        }
        20 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf504); };
            c.w64(6, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bf508); };
            c.w64(0, t25);
            blk = 1; continue;
        }
        21 => {
            return RegfileExit::jump(0x70008bf514);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf534); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bf538); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf539); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf53a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bf53f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bf545); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf54d); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf553); };
            c.w32(2, t5);
            let r6 = c.sub(c.r32(2), 0xbau64, 32);
            c.w32(2, r6);
            c.cmp(c.r32(2), 0x78eu64, 32);
            if c.cc_a() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bf567); };
            c.w64(3, t7);
            return c.call(0x7000820e54, 0x70008bf570, 0x70008bf56b);
        }
        4 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf570); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bf577); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x18, 0x70008bf57e);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bf581); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bf588); }
            return c.ret(0x0, 0x70008bf58a);
        }
        7 => {
            c.w32(2, 0x34u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf590); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008bf591); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x287)) else { return RegfileExit::fault(0x70008bf598); };
            c.w32(2, t10);
            let r11 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r11);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf5a1); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf5a2); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf5b3, 0x70008bf5ae);
        }
        8 => {
            return RegfileExit::fault(0x70008bf5b3);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bf5b4); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bf5b8); };
            c.w64(3, t13);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bf5c6, 0x70008bf5c1);
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf5c6); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008bf5d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf5d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf5d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008bf5d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf5d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008bf5da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008bf5df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008bf5e5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf5ed); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf5f3); };
            c.w32(2, t5);
            let r6 = c.sub(c.r32(2), 0xbau64, 32);
            c.w32(2, r6);
            c.cmp(c.r32(2), 0x78eu64, 32);
            if c.cc_a() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bf605); };
            c.w64(3, t7);
            return c.call(0x7000820354, 0x70008bf60e, 0x70008bf609);
        }
        4 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf60e); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008bf615); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return c.ret(0x18, 0x70008bf61c);
        }
        6 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008bf61f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008bf626); }
            return c.ret(0x0, 0x70008bf628);
        }
        7 => {
            c.w32(2, 0x34u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf62e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x9c0)) else { return RegfileExit::fault(0x70008bf62f); };
            c.w64(2, t9);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008bf636); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008bf637); };
            c.w64(3, t10);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008bf648, 0x70008bf643);
        }
        8 => {
            return RegfileExit::fault(0x70008bf648);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008bf649); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008bf64d); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008bf65b, 0x70008bf656);
        }
        10 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bf65b); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008bf664);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008bf674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008bf674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008bf678); }
            let r0 = c.sub(c.r64(4), 0x70u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf67e); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008bf684); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008bf688); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008bf68c); }
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf690); };
            c.cmp(t3, 0x83eu64, 16);
            if c.cc_ne() { blk = 220; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008bf69c); };
            c.w64(3, t4);
            c.w64(7, c.r64(3));
            return c.call(0x7000870914, 0x70008bf6ab, 0x70008bf6a6);
        }
        2 => {
            c.logic(c.r32(0) & 0x80000001u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf6b6); };
            c.w64(6, t5);
            return c.call(0x70008a17f4, 0x70008bf6bf, 0x70008bf6ba);
        }
        5 => {
            blk = 10; continue;
        }
        6 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(1, c.r64(0));
            blk = 9; continue;
        }
        8 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(1));
            blk = 10; continue;
        }
        10 => {
            c.w32(7, 0x9eu64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008bf6d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008bf6d6); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf6da); };
            c.w64(6, t6);
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008bf6de); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008bf6f2, 0x70008bf6ed);
        }
        11 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bf6f2); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bf6f6); };
            c.w32(7, t9);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.logic(c.r8(7) & 0x9u64, 8);
            c.w8(7, (c.cc_ne() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008bf70b); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bf717); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf71b); };
            c.w64(11, t11);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        13 => {
            let Some(t12) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf727); };
            c.logic(t12 & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bf73d); };
            c.w64(3, t13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf741); };
            c.w32(12, t14);
            let r15 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r15);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bf74e); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf75b); };
            c.w64(15, t16);
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bf75f); }
            c.w64(11, c.r64(15));
            blk = 20; continue;
        }
        17 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf76a); };
            c.w64(15, t17);
            let r18 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r18);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf77a); };
            c.w64(11, t19);
            blk = 20; continue;
        }
        18 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            let r20 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf789); };
            c.w64(11, t21);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008bf78d); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 20; continue;
        }
        20 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 24; continue;
        }
        22 => {
            let Some(t22) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf7a4); };
            c.cmp(c.r32(12), t22, 32);
            if c.cc_l() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf7aa); };
            c.w64(11, t23);
            blk = 24; continue;
        }
        24 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        25 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bf7b3); };
            c.w64(11, t24);
            blk = 26; continue;
        }
        26 => {
            c.w32(15, 0x1u64);
            blk = 27; continue;
        }
        27 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 218; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(15, c.r64(11));
            blk = 29; continue;
        }
        29 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bf7cd); };
            c.w64(6, t25);
            let Some(t26) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf7d1); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            let Some(t28) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bf7d7); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bf7dd); };
            c.w32(1, t30);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008bf7e6); }
            c.cmp(c.r64(1), 0x9bu64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x70008bf7f3);
        }
        31 => {
            let Some(t31) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf7f4); };
            c.w32(1, t31);
            let r32 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r32);
            let Some(t33) = c.ld32(c.r64(2).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008bf7fa); };
            c.w32(2, t33);
            let r34 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r34);
            let Some(t35) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf803); };
            c.w32(1, t35);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x70008bf807); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf818); };
            c.w32(9, t36);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 81; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t37) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bf828); };
            c.w32(15, t37);
            let r38 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r38);
            let Some(t39) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf82f); };
            c.w32(2, t39);
            let r40 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r40);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008bf835); }
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 222; continue;
        }
        37 => {
            blk = 40; continue;
        }
        38 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008bf84f); };
            c.w64(2, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bf853); };
            c.w64(1, t42);
            return c.call(0x7000859a54, 0x70008bf85c, 0x70008bf857);
        }
        39 => {
            c.w64(2, c.r64(0));
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bf85f); };
            c.w64(0, t43);
            blk = 82; continue;
        }
        40 => {
            let Some(t44) = c.ld32(c.r64(15).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008bf868); };
            c.w32(2, t44);
            c.cmp(c.r32(2), 0x7eu64, 32);
            if c.cc_a() { blk = 235; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(1) & 0x7u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008bf87e); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008bf889); }
            blk = 44; continue;
        }
        44 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bf88d); };
            c.w64(11, t45);
            let r46 = c.r32(11) & 0x7u64;
            c.w32(11, r46);
            c.logic(r46, 32);
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 235; continue;
        }
        49 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 235; continue;
        }
        52 => {
            let Some(t47) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bf8c7); };
            c.w32(11, t47);
            let Some(t48) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008bf8cb); };
            c.cmp(t48, c.r32(11), 32);
            if c.cc_ne() { blk = 235; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t49) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bf8d8); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf8de); };
            c.w32(11, t51);
            let r52 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r52);
            let Some(t53) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf8e5); };
            c.w32(11, t53);
            blk = 43; continue;
        }
        54 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008bf8ec); };
            c.w32(11, t54);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r55 = c.add(c.r64(1), c.r64(11), 64);
            c.w64(1, r55);
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bf8f9); };
            c.w32(7, t56);
            let r57 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r57);
            let Some(t58) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf8ff); };
            c.w32(11, t58);
            let r59 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r59);
            let Some(t60) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf906); };
            c.w32(11, t60);
            blk = 43; continue;
        }
        55 => {
            let Some(t61) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008bf910); };
            c.w32(7, t61);
            let r62 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r62);
            let Some(t63) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bf916); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            let Some(t65) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bf91d); };
            c.w32(11, t65);
            blk = 43; continue;
        }
        56 => {
            c.w64(11, c.r64(12));
            blk = 59; continue;
        }
        57 => {
            c.w32(11, 0x1u64);
            blk = 59; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008bf934); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(12));
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 224; continue; }
            blk = 60; continue;
        }
        60 => {
            let r66 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r66);
            blk = 61; continue;
        }
        61 => {
            c.w64(11, c.r64(1).wrapping_add(c.r64(9)));
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bf952); };
            c.w64(9, t67);
            let r68 = c.r32(9) & 0x8u64;
            c.w32(9, r68);
            c.logic(r68, 32);
            let r69 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r69);
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_ne() { blk = 226; continue; }
            blk = 62; continue;
        }
        62 => {
            let r70 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r70);
            let r71 = c.add(c.r64(7), c.r64(1), 64);
            c.w64(7, r71);
            let Some(t72) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bf96d); };
            c.w32(1, t72);
            let r73 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r73);
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008bf974); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008bf978); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 235; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t74) = c.ld32(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008bf985); };
            c.w32(9, t74);
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008bf989); };
            c.w64(7, t75);
            let Some(t76) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008bf990); };
            c.w64(7, t76);
            c.cmp(c.r32(9), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w32(1, c.r64(9).wrapping_mul(8));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(13));
            let Some(t77) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008bf9a7); };
            c.w64(1, t77);
            blk = 67; continue;
        }
        66 => {
            let r78 = c.add(c.r64(1), 0x3fu64, 64);
            c.w64(1, r78);
            c.w64(9, c.r64(13));
            blk = 67; continue;
        }
        67 => {
            let r79 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r79);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(8, c.r32(8));
            c.w64(10, c.r64(4));
            let r80 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r80);
            let r81 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r81);
            c.logic(r81, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008bf9cf); }
            let r82 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r82);
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008bf9d6); };
            c.w64(11, t83);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008bf9dd); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x70008bf9e2); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(7)) { return RegfileExit::fault(0x70008bf9e7); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008bf9ec); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(9)) { return RegfileExit::fault(0x70008bf9f5); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x70008bf9fa); }
            c.w64(7, c.r64(1));
            c.w64(1, c.r64(3));
            c.w64(2, c.r64(8));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bfa08); };
            c.w64(8, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008bfa0c); };
            c.w64(9, t85);
            c.w64(10, 0x70008bfa17u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008bfa17); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008bfa1b); }
            return c.call(c.r64(7), 0x70008bfa21, 0x70008bfa1f);
        }
        68 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008bfa21); }
            let Some(t86) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008bfa29); };
            c.w64(4, t86);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 227; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bfa41); };
            c.w64(0, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bfa45); };
            c.w64(6, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bfa49); };
            c.w64(3, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008bfa4d); };
            c.w64(11, t90);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bfa54); };
            c.w64(7, t91);
            blk = 81; continue;
        }
        71 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008bfa5d); };
            c.w64(2, t92);
            let Some(t93) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bfa61); };
            c.w32(7, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bfa64); };
            c.w64(11, t94);
            let Some(t95) = c.ld32(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008bfa68); };
            c.w32(11, t95);
            let r96 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r96);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r97 = c.sub(c.r32(7), 0x6u64, 32);
            c.w32(7, r97);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_l() { blk = 234; continue; }
            blk = 72; continue;
        }
        72 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008bfa82); }
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bfa86); };
            c.w64(0, t98);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008bfa8a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r99 = c.r64(7) & c.r64(2);
            c.w64(7, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008bfa9b); };
            c.logic(t100 & 0x4u64, 8);
            if c.cc_ne() { blk = 310; continue; }
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008bfaa5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r101 = c.r64(7) & c.r64(2);
            c.w64(7, r101);
            c.logic(r101, 64);
            let Some(t102) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008bfab6); };
            c.logic(t102 & 0x4u64, 8);
            if c.cc_ne() { blk = 307; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if c.cc_e() { blk = 82; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(11, 0x13u64);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008bfad1); };
            c.w64(15, t103);
            blk = 79; continue;
        }
        78 => {
            let r104 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r104);
            blk = 79; continue;
        }
        79 => {
            let Some(t105) = c.ld32(c.r64(15).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008bfade); };
            c.w32(3, t105);
            let r106 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r106);
            if !c.st32(c.r64(2).wrapping_add(c.r64(11)), c.r32(3)) { return RegfileExit::fault(0x70008bfae4); }
            let r107 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r107);
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ne() { blk = 78; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 82; continue;
        }
        81 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bfaf3); };
            c.w64(2, t108);
            c.w64(0, c.r64(3));
            blk = 82; continue;
        }
        82 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x70008bfafd); }
            let Some(t109) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008bfb01); };
            c.cmp(t109, c.r32(2), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t110) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bfb0a); };
            c.cmp(t110, 0x0u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bfb14); };
            c.w64(0, t111);
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008bfb18); };
            c.w64(3, t112);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bfb21); };
            c.w64(6, t113);
            return c.call(0x7000871cb4, 0x70008bfb2a, 0x70008bfb25);
        }
        85 => {
            blk = 219; continue;
        }
        86 => {
            let Some(t114) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bfb2f); };
            c.cmp(t114, 0x0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t115) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bfb35); };
            c.w32(7, t115);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_a() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            return RegfileExit::fault(0x70008bfb44);
        }
        89 => {
            let Some(t116) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bfb45); };
            c.w32(1, t116);
            let r117 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bfb4b); };
            c.w64(0, t118);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008bfb4f); };
            c.w64(3, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bfb56); };
            c.w64(6, t120);
            return c.call(0x7000871cb4, 0x70008bfb5f, 0x70008bfb5a);
        }
        90 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008bfb5f); };
            c.w64(2, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008bfb63); };
            c.w64(0, t122);
            blk = 91; continue;
        }
        91 => {
            let Some(t123) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008bfb67); };
            c.w32(7, t123);
            let Some(t124) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bfb6a); };
            c.w32(3, t124);
            let r125 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r125);
            let Some(t126) = c.ld32(c.r64(2).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008bfb70); };
            c.w32(1, t126);
            let r127 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r127);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008bfb76); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008bfb7a); }
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bfb7e); };
            c.w64(6, t128);
            return c.call(0x700086fe54, 0x70008bfb87, 0x70008bfb82);
        }
        92 => {
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bfb87); };
            c.w64(7, t129);
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bfb8b); };
            c.w32(11, t130);
            let r131 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r131);
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008bfba0); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008bfba4); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bfbb2); };
            c.w64(6, t132);
            let Some(t133) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bfbb6); };
            c.w32(12, t133);
            let r134 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r134);
            let Some(t135) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bfbbd); };
            c.w32(12, t135);
            let r136 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r136);
            let Some(t137) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bfbc5); };
            c.w32(15, t137);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa1u64, 64);
            if c.cc_a() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            return RegfileExit::fault(0x70008bfbd9);
        }
        95 => {
            let Some(t138) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bfbda); };
            c.w64(15, (t138 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let Some(t139) = c.ld32(c.r64(12).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008bfbe1); };
            c.w32(12, t139);
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r140 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r140);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008bfbf8); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 109; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(15, c.r64(3).wrapping_mul(2).wrapping_add(0x30));
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r64(2), 0x2000000u64, 64);
            if c.cc_a() { blk = 237; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 242; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t141) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bfc35); };
            c.w64(15, (t141 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r142 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r142);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bfc49); };
            c.w64(3, t143);
            let Some(t144) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bfc50); };
            c.w64(1, t144);
            c.w64(9, c.r64(8).wrapping_add(0x8));
            c.w64(7, c.r64(9).wrapping_add(c.r64(1)));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x70008bfc5b); }
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bfc5f); };
            c.w64(11, t145);
            let Some(t146) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008bfc66); };
            c.cmp(t146, c.r64(7), 64);
            if c.cc_be() { blk = 238; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 238; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008bfc7c); }
            c.w64(7, c.r64(1).wrapping_add(0x1));
            blk = 102; continue;
        }
        102 => {
            let r147 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r147);
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008bfc85); };
            c.w64(1, t148);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008bfc8c); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008bfc8f); }
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r149 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r149);
            let r150 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r150);
            c.logic(r150, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008bfca1); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x70008bfca6); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008bfcac); };
            c.w64(0, t151);
            c.w64(10, 0x70008bfcbau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008bfcba); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008bfcbe); }
            return c.call(c.r64(0), 0x70008bfcc4, 0x70008bfcc2);
        }
        103 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008bfcc4); }
            let Some(t152) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bfccc); };
            c.w64(4, t152);
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bfcd1); };
            c.w64(7, t153);
            let Some(t154) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008bfcd8); };
            c.w64(11, t154);
            c.w64(12, c.r64(11).wrapping_add(0x28));
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bfcdf); };
            c.w64(15, t155);
            let Some(t156) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bfce6); };
            c.cmp(t156, c.r64(12), 64);
            if c.cc_be() { blk = 240; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008bfcef); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 105; continue;
        }
        105 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bfcf6); };
            c.w64(11, t157);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008bfcfa); }
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008bfcfe); };
            c.w64(11, t158);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008bfd02); }
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bfd06); };
            c.w64(11, t159);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008bfd0d); }
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008bfd11); };
            c.w64(0, t160);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008bfd15); }
            c.w64(3, c.r64(7));
            c.w64(12, c.r64(0));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bfd1e); };
            c.w64(7, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bfd22); };
            c.w64(6, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bfd26); };
            c.w64(11, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bfd2a); };
            c.w64(0, t164);
            blk = 131; continue;
        }
        106 => {
            c.w32(3, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bfd36); };
            c.w64(2, t165);
            let Some(t166) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008bfd3d); };
            c.w64(1, t166);
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bfd44); };
            c.w64(9, t167);
            let Some(t168) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008bfd4b); };
            c.cmp(t168, c.r64(8), 64);
            if c.cc_be() { blk = 244; continue; }
            blk = 107; continue;
        }
        107 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008bfd54); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 108; continue;
        }
        108 => {
            let Some(t169) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bfd5b); };
            c.w64(2, (t169 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r170 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r170);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008bfd67); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008bfd6b); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bfd6f); };
            c.w64(12, t171);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008bfd76); }
            c.w64(12, c.r64(15).wrapping_add(0x28));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008bfd7e); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008bfd82); };
            c.w64(2, t172);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008bfd89); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008bfd8e); }
            c.w64(3, c.r64(15));
            blk = 131; continue;
        }
        109 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bfd9b); };
            c.w64(3, t173);
            let Some(t174) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bfda2); };
            c.w64(2, t174);
            c.w64(1, c.r64(2).wrapping_add(0x28));
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bfda9); };
            c.w64(8, t175);
            let Some(t176) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008bfdb0); };
            c.cmp(t176, c.r64(1), 64);
            if c.cc_be() { blk = 246; continue; }
            blk = 110; continue;
        }
        110 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008bfdb9); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 111; continue;
        }
        111 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008bfdc0); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008bfdc4); }
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bfdc8); };
            c.w64(12, t177);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008bfdcf); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008bfdd3); }
            blk = 131; continue;
        }
        112 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bfddc); };
            c.w64(6, t178);
            let Some(t179) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008bfde0); };
            c.w32(12, t179);
            let r180 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r180);
            let Some(t181) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008bfde7); };
            c.w32(12, t181);
            let r182 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r182);
            let Some(t183) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008bfdef); };
            c.w32(15, t183);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa2u64, 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return RegfileExit::fault(0x70008bfe03);
        }
        114 => {
            let Some(t184) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bfe04); };
            c.w64(15, (t184 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r185 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r185);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t186) = c.ld32(c.r64(12).wrapping_add(0x28f)) else { return RegfileExit::fault(0x70008bfe14); };
            c.w32(12, t186);
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008bfe22); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 128; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(15, c.r64(3).wrapping_mul(2).wrapping_add(0x34));
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r64(2), 0x2000000u64, 64);
            if c.cc_a() { blk = 248; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 253; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t187) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bfe5f); };
            c.w64(15, (t187 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r188 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r188);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let r189 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r189);
            c.w64(8, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(8).wrapping_add(0x8));
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bfe79); };
            c.w64(1, t190);
            let Some(t191) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008bfe80); };
            c.w64(9, t191);
            c.w64(7, c.r64(3).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x70008bfe87); }
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bfe8b); };
            c.w64(11, t192);
            let Some(t193) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008bfe92); };
            c.cmp(t193, c.r64(7), 64);
            if c.cc_be() { blk = 249; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 249; continue; }
            blk = 120; continue;
        }
        120 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008bfea8); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 121; continue;
        }
        121 => {
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008bfeaf); };
            c.w64(3, t194);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008bfeb6); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008bfeb9); }
            c.w64(3, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r195 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r195);
            let r196 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r196);
            c.logic(r196, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008bfecb); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x70008bfed0); }
            c.w64(1, c.r64(3));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008bfed9); };
            c.w64(0, t197);
            c.w64(10, 0x70008bfee7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008bfee7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008bfeeb); }
            return c.call(c.r64(0), 0x70008bfef1, 0x70008bfeef);
        }
        122 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008bfef1); }
            let Some(t198) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008bfef9); };
            c.w64(4, t198);
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bfefe); };
            c.w64(7, t199);
            let Some(t200) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008bff05); };
            c.w64(11, t200);
            c.w64(12, c.r64(11).wrapping_add(0x2c));
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bff0c); };
            c.w64(15, t201);
            let Some(t202) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008bff13); };
            c.cmp(t202, c.r64(12), 64);
            if c.cc_be() { blk = 251; continue; }
            blk = 123; continue;
        }
        123 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008bff1c); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008bff23); };
            c.w64(11, t203);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008bff27); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008bff2b); };
            c.w64(11, t204);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008bff2f); }
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bff33); };
            c.w64(11, t205);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008bff3a); }
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008bff3e); };
            c.w64(0, t206);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008bff42); }
            c.w64(3, c.r64(7));
            c.w64(12, c.r64(0));
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008bff4b); };
            c.w64(7, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008bff4f); };
            c.w64(6, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008bff53); };
            c.w64(11, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008bff57); };
            c.w64(0, t210);
            blk = 131; continue;
        }
        125 => {
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bff60); };
            c.w64(3, t211);
            let Some(t212) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bff67); };
            c.w64(1, t212);
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bff6e); };
            c.w64(9, t213);
            let Some(t214) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008bff75); };
            c.cmp(t214, c.r64(8), 64);
            if c.cc_be() { blk = 255; continue; }
            blk = 126; continue;
        }
        126 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008bff7e); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 127; continue;
        }
        127 => {
            let Some(t215) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008bff85); };
            c.w64(3, (t215 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r216 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r216);
            let r217 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r217);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008bff93); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008bff97); }
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bff9b); };
            c.w64(12, t218);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008bffa2); }
            c.w64(12, c.r64(15).wrapping_add(0x2c));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008bffaa); }
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008bffae); };
            c.w64(3, t219);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008bffb5); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008bffba); }
            c.w64(3, c.r64(15));
            blk = 131; continue;
        }
        128 => {
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008bffc4); };
            c.w64(3, t220);
            let Some(t221) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008bffcb); };
            c.w64(2, t221);
            c.w64(1, c.r64(2).wrapping_add(0x2c));
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008bffd2); };
            c.w64(8, t222);
            let Some(t223) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008bffd9); };
            c.cmp(t223, c.r64(1), 64);
            if c.cc_be() { blk = 257; continue; }
            blk = 129; continue;
        }
        129 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008bffe2); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 130; continue;
        }
        130 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008bffe9); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008bffed); }
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008bfff1); };
            c.w64(12, t224);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008bfff8); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008bfffc); }
            blk = 131; continue;
        }
        131 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008c0000); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x70008c0004); }
            let Some(t225) = c.ld8(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c0008); };
            c.logic(t225 & 0x1u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 134; continue;
        }
        133 => {
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c0013); };
            c.w64(15, t226);
            blk = 134; continue;
        }
        134 => {
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c0017); };
            c.w64(2, t227);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x70008c001b); }
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c001e); };
            c.w64(2, t228);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008c0022); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r229 = c.r64(1) & c.r64(3);
            c.w64(1, r229);
            c.logic(r229, 64);
            let Some(t230) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0034); };
            c.logic(t230 & 0x4u64, 8);
            if c.cc_ne() { blk = 304; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c003e); };
            c.w64(1, t231);
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70008c0045); }
            if !c.st32(c.r64(3).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x70008c0048); }
            if !c.st32(c.r64(3).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70008c004b); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r232 = c.r64(8) & c.r64(3);
            c.w64(8, r232);
            c.logic(r232, 64);
            let Some(t233) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c005d); };
            c.logic(t233 & 0x4u64, 8);
            if c.cc_ne() { blk = 301; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.st32(c.r64(3).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70008c0068); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.st32(c.r64(3).wrapping_add(0x27), c.r32(1)) { return RegfileExit::fault(0x70008c0072); }
            blk = 140; continue;
        }
        140 => {
            let Some(t234) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c0075); };
            c.w64(15, (t234 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r235 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r235);
            c.w64(8, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 142; continue; }
            blk = 141; continue;
        }
        141 => {
            let r236 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r236);
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x70008c0097); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 141; continue; }
            blk = 142; continue;
        }
        142 => {
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c00a1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r237 = c.r64(8) & c.r64(12);
            c.w64(8, r237);
            c.logic(r237, 64);
            let Some(t238) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c00b4); };
            c.logic(t238 & 0x4u64, 8);
            if c.cc_ne() { blk = 298; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t239) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c00c9); };
            c.w64(15, (t239 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r240 = c.add(c.r64(15), 0x3u64, 64);
            c.w64(15, r240);
            c.w32(0, 0x1u64);
            c.w32(8, 0x5u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008c00df); }
            blk = 147; continue;
        }
        146 => {
            c.w64(0, c.r64(7).wrapping_add(0x1));
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c00e9); };
            c.w64(2, t241);
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c00ed); };
            c.w64(1, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c00f4); };
            c.w64(6, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c00f8); };
            c.w64(3, t244);
            blk = 147; continue;
        }
        147 => {
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c00fc); };
            c.w64(9, t245);
            let Some(t246) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0100); };
            c.w32(7, t246);
            let r247 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r247);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008c0108); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008c010c); }
            c.cmp(c.r32(7), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(7, c.r64(8).wrapping_add(0x1));
            let Some(t248) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0119); };
            c.w32(7, t248);
            let r249 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r249);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008c0121); }
            c.w64(0, c.r64(2));
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c0128); };
            c.w64(3, t250);
            c.w64(1, c.r64(7));
            return c.call(0x700086fe54, 0x70008c0134, 0x70008c012f);
        }
        149 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c0134); };
            c.w64(7, t251);
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c0140); };
            c.w64(12, t252);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008c0144); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r253 = c.r64(15) & c.r64(12);
            c.w64(15, r253);
            c.logic(r253, 64);
            let Some(t254) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0156); };
            c.logic(t254 & 0x4u64, 8);
            if c.cc_ne() { blk = 295; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 153; continue;
        }
        152 => {
            c.w64(7, c.r64(0));
            blk = 153; continue;
        }
        153 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c0166); };
            c.w64(8, t255);
            let r256 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c016e); };
            c.cmp(c.r64(8), t257, 64);
            if c.cc_b() { blk = 146; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c0178); };
            c.w64(0, t258);
            let Some(t259) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c017c); };
            c.w32(7, t259);
            let r260 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r260);
            let Some(t261) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0182); };
            c.w32(11, t261);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x9u64, 64);
            if c.cc_be() { blk = 259; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t262) = c.ld32(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008c0196); };
            c.w32(7, t262);
            let r263 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r263);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008c019c); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t264) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c01aa); };
            c.w32(11, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c01ae); };
            c.w64(15, t265);
            if !c.st32(c.r64(15).wrapping_add(0x1b), c.r32(7)) { return RegfileExit::fault(0x70008c01b2); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r266 = c.r64(3) & c.r64(15);
            c.w64(3, r266);
            c.logic(r266, 64);
            let Some(t267) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c01c6); };
            c.logic(t267 & 0x4u64, 8);
            if c.cc_ne() { blk = 292; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c01d0); };
            c.w64(6, t268);
            let Some(t269) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c01d4); };
            c.w32(3, t269);
            let r270 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r270);
            let Some(t271) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c01da); };
            c.w32(3, t271);
            let r272 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r272);
            let Some(t273) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c01e0); };
            c.w32(2, t273);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xb1u64, 64);
            if c.cc_a() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            return RegfileExit::fault(0x70008c01f2);
        }
        160 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(2, c.r64(11));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w64(8, c.r64(2));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(2, c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x1));
            c.w64(8, c.r64(2));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r274 = c.r64(8) | c.r64(2);
            c.w64(8, r274);
            c.logic(r274, 64);
            c.w64(2, c.r64(8));
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r275 = c.r64(2) | c.r64(8);
            c.w64(2, r275);
            c.logic(r275, 64);
            c.w64(8, c.r64(2));
            if let Some(sr) = c.shr(c.r64(8), 0x4u64, 64) { c.w64(8, sr); }
            let r276 = c.r64(8) | c.r64(2);
            c.w64(8, r276);
            c.logic(r276, 64);
            c.w64(2, c.r64(8));
            if let Some(sr) = c.shr(c.r64(2), 0x8u64, 64) { c.w64(2, sr); }
            let r277 = c.r64(2) | c.r64(8);
            c.w64(2, r277);
            c.logic(r277, 64);
            c.w64(8, c.r64(2));
            if let Some(sr) = c.shr(c.r64(8), 0x10u64, 64) { c.w64(8, sr); }
            let r278 = c.r64(8) | c.r64(2);
            c.w64(8, r278);
            c.logic(r278, 64);
            c.w64(2, c.r64(8).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c023f); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_ge() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w32(2, 0x4u64);
            blk = 162; continue;
        }
        162 => {
            let Some(t279) = c.ld32(c.r64(3).wrapping_add(0x2cb)) else { return RegfileExit::fault(0x70008c024e); };
            c.w32(3, t279);
            let r280 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r280);
            c.w64(8, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c025c); };
            c.w64(9, t281);
            let Some(t282) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c0263); };
            c.w64(9, t282);
            c.w64(6, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(9)));
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c0272); };
            c.w64(11, t283);
            let Some(t284) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c0279); };
            c.cmp(t284, c.r64(0), 64);
            if c.cc_be() { blk = 260; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_ae() { blk = 260; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c028f); };
            c.w64(11, t285);
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008c0296); }
            c.w64(11, c.r64(9).wrapping_add(0x1));
            blk = 165; continue;
        }
        165 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008c02a1); };
            c.w64(8, t286);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008c02a8); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008c02ac); }
            let Some(t287) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c02b0); };
            c.w32(0, t287);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 262; continue; }
            blk = 166; continue;
        }
        166 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008c02c4); }
            let Some(t288) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c02c8); };
            c.w32(0, t288);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 263; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008c02dc); }
            let Some(t289) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c02e0); };
            c.w32(0, t289);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x2u64, 64);
            if c.cc_be() { blk = 264; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w32(0, c.r64(2).wrapping_add(c.r64(2)));
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008c02f7); }
            let Some(t290) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c02fb); };
            c.w32(0, t290);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_be() { blk = 265; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w32(0, 0x2u64);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x70008c0314); }
            let Some(t291) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0318); };
            c.w32(0, t291);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x4u64, 64);
            if c.cc_be() { blk = 266; continue; }
            blk = 170; continue;
        }
        170 => {
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70008c032c); }
            c.w64(0, c.r64(11).wrapping_add(c.r64(6)).wrapping_sub(0x1));
            c.w64(2, c.r64(11).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_e() { blk = 174; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c033e); };
            c.w64(6, t292);
            blk = 172; continue;
        }
        172 => {
            if !c.st32(c.r64(2), c.r32(6)) { return RegfileExit::fault(0x70008c0345); }
            let r293 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r293);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 172; continue; }
            blk = 173; continue;
        }
        173 => {
            blk = 175; continue;
        }
        174 => {
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c0352); };
            c.w64(6, t294);
            blk = 175; continue;
        }
        175 => {
            let Some(t295) = c.ld8(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0359); };
            c.w32(0, t295);
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c035d); };
            c.w64(2, t296);
            let Some(t297) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c0364); };
            c.w64(8, t297);
            if let Some(sr) = c.shl(c.r64(0), 0x2u64, 64) { c.w64(0, sr); }
            c.w64(9, c.r64(0).wrapping_add(c.r64(8)));
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c036f); };
            c.w64(7, t298);
            let Some(t299) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c0376); };
            c.cmp(t299, c.r64(9), 64);
            if c.cc_be() { blk = 267; continue; }
            blk = 176; continue;
        }
        176 => {
            if !c.st64(c.r64(2), c.r64(9)) { return RegfileExit::fault(0x70008c037f); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 177; continue;
        }
        177 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008c0386); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c0389); }
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c038d); };
            c.w64(3, t300);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008c0394); }
            let r301 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r301);
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let r302 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r302);
            if !c.st32(c.r64(0).wrapping_add(c.r64(7)), c.r32(6)) { return RegfileExit::fault(0x70008c03a5); }
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_ne() { blk = 178; continue; }
            blk = 179; continue;
        }
        179 => {
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(7)) { return RegfileExit::fault(0x70008c03ae); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r303 = c.r64(0) & c.r64(15);
            c.w64(0, r303);
            c.logic(r303, 64);
            let Some(t304) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c03c2); };
            c.logic(t304 & 0x4u64, 8);
            if c.cc_ne() { blk = 289; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(0, c.r64(1));
            blk = 183; continue;
        }
        182 => {
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c03d1); };
            c.w64(12, t305);
            blk = 183; continue;
        }
        183 => {
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c03d5); };
            c.w64(7, t306);
            let Some(t307) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c03d9); };
            c.w32(2, t307);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ae() { blk = 269; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(6, c.r64(0).wrapping_add(0x1));
            let Some(t308) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c03ef); };
            c.w32(0, t308);
            let r309 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r309);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008c03f6); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_ae() { blk = 270; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t310) = c.ld32(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0403); };
            c.w32(2, t310);
            let Some(t311) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0407); };
            c.w32(8, t311);
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_ae() { blk = 271; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t312) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0415); };
            c.w32(8, t312);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t313) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c041f); };
            c.w32(9, t313);
            let Some(t314) = c.ld32(c.r64(12).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0423); };
            c.w32(2, t314);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 272; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t315) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0435); };
            c.w32(15, t315);
            let r316 = c.add(c.r32(15), 0x2u64, 32);
            c.w32(15, r316);
            c.w64(7, c.r64(15));
            let r317 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r317);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r318 = c.r64(7) & 0xfffffffffffffffeu64;
            c.w64(7, r318);
            c.logic(r318, 64);
            let r319 = c.add(c.r32(7), c.r32(15), 32);
            c.w32(7, r319);
            c.w32(12, c.r32(9));
            c.cmp(c.r32(9), c.r32(7), 32);
            if c.cc_b() { blk = 279; continue; }
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 273; continue; }
            blk = 189; continue;
        }
        189 => {
            let r320 = c.sub(c.r32(12), c.r32(15), 32);
            c.w32(12, r320);
            let r321 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r321);
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t322) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c046b); };
            c.w32(12, t322);
            let r323 = c.r64(7) & 0xfffffffffffffffeu64;
            c.w64(7, r323);
            c.logic(r323, 64);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_b() { blk = 279; continue; }
            blk = 190; continue;
        }
        190 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 274; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t324) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c0486); };
            c.w32(7, t324);
            let r325 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r325);
            c.w32(12, c.r64(7).wrapping_add(0x2));
            c.cmp(c.r32(12), 0xfffffeu64, 32);
            if c.cc_a() { blk = 279; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008c04a1); }
            let Some(t326) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c04a5); };
            c.w32(12, t326);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 275; continue; }
            blk = 193; continue;
        }
        193 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c04bc); }
            let Some(t327) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c04c0); };
            c.w32(12, t327);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r328 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r328);
            let Some(t329) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c04ce); };
            c.w32(15, t329);
            let r330 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r330);
            let r331 = c.r64(15) & c.r64(12);
            c.w64(15, r331);
            c.logic(r331, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 195; continue;
        }
        194 => {
            let r332 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r332);
            let r333 = c.add(c.r64(15), c.r64(8), 64);
            c.w64(15, r333);
            c.w64(3, c.r64(12));
            let r334 = c.r64(3) & c.r64(15);
            c.w64(3, r334);
            c.logic(r334, 64);
            c.w64(15, c.r64(3));
            blk = 195; continue;
        }
        195 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(9).wrapping_mul(4));
            c.w64(3, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t335) = c.ld32(c.r64(11).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0503); };
            c.w32(6, t335);
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c0508); };
            c.cmp(t336, c.r32(6), 32);
            if c.cc_e() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t337) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c0511); };
            c.cmp(t337, c.r32(6), 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t338) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c051a); };
            c.w32(12, t338);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 276; continue; }
            blk = 198; continue;
        }
        198 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(3)), c.r32(0)) { return RegfileExit::fault(0x70008c052d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r339 = c.r64(12) & c.r64(11);
            c.w64(12, r339);
            c.logic(r339, 64);
            let Some(t340) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c053f); };
            c.logic(t340 & 0x4u64, 8);
            if c.cc_ne() { blk = 286; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t341) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c054b); };
            c.w32(12, t341);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(9).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ae() { blk = 277; continue; }
            blk = 201; continue;
        }
        201 => {
            if let Some(sr) = c.shl(c.r64(7), 0x8u64, 64) { c.w64(7, sr); }
            c.w32(12, c.r32(7));
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(11).wrapping_add(c.r64(15)), c.r32(2)) { return RegfileExit::fault(0x70008c0571); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            c.w64(3, 0xfffffffffffc0000u64);
            let r342 = c.r64(3) & c.r64(11);
            c.w64(3, r342);
            c.logic(r342, 64);
            let Some(t343) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0584); };
            c.logic(t343 & 0x4u64, 8);
            if c.cc_ne() { blk = 283; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t344) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c058e); };
            c.w32(15, t344);
            let Some(t345) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008c0592); };
            c.cmp(t345, c.r32(15), 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t346) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c059b); };
            c.logic(t346 & 0x1u64, 8);
            if c.cc_e() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            let r347 = c.r32(12) | 0x20u64;
            c.w32(12, r347);
            c.logic(r347, 32);
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 207; continue;
        }
        206 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 207; continue;
        }
        207 => {
            let Some(t348) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c05ad); };
            c.w32(12, t348);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ae() { blk = 278; continue; }
            blk = 208; continue;
        }
        208 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008c05c4); }
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c05c9); };
            c.w64(7, t349);
            c.w64(0, c.r64(7).wrapping_add(0x1));
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c05d1); };
            c.w64(7, t350);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_g() { blk = 182; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c05de); };
            c.w64(0, t351);
            blk = 211; continue;
        }
        210 => {
            c.w64(0, c.r64(7));
            blk = 211; continue;
        }
        211 => {
            let Some(t352) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c05e7); };
            c.cmp(t352, 0x0u64, 32);
            if c.cc_ne() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c05ed); };
            c.w64(7, t353);
            blk = 216; continue;
        }
        213 => {
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c05f3); };
            c.w64(7, t354);
            let Some(t355) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c05f7); };
            c.w32(11, t355);
            let r356 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r356);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c05fe); }
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c05ff); };
            if !c.push64(t357) { return RegfileExit::fault(0x70008c05ff); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c0602); }
            let Some(t358) = c.ld64(c.r64(13).wrapping_add(0x2640)) else { return RegfileExit::fault(0x70008c0604); };
            c.w64(3, t358);
            c.w32(0, 0x3u64);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c0610); };
            c.w64(6, t359);
            return c.call(0x700086ed74, 0x70008c0619, 0x70008c0614);
        }
        214 => {
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c0619); };
            c.w64(7, t360);
            if !c.st32(c.r64(7).wrapping_add(0x27), c.r32(0)) { return RegfileExit::fault(0x70008c061d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 216; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r361 = c.r64(11) & c.r64(7);
            c.w64(11, r361);
            c.logic(r361, 64);
            let Some(t362) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c062e); };
            c.logic(t362 & 0x4u64, 8);
            if c.cc_ne() { blk = 280; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(0, c.r64(7));
            blk = 217; continue;
        }
        217 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c063f); }
            return c.ret(0x0, 0x70008c0640);
        }
        218 => {
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c0641); };
            c.w64(6, t363);
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0x810)) else { return RegfileExit::fault(0x70008c0645); };
            c.w64(3, t364);
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x7000871cb4, 0x70008c0653, 0x70008c064e);
        }
        219 => {
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c0653); };
            c.w64(0, t365);
            blk = 217; continue;
        }
        220 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0661); }
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c0662); };
            c.w64(7, t366);
            let Some(t367) = c.ld32(c.r64(7).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c0669); };
            c.w32(7, t367);
            let r368 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r368);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0672); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c0673); }
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c0674); };
            c.w64(3, t369);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c0685, 0x70008c0680);
        }
        221 => {
            return RegfileExit::fault(0x70008c0685);
        }
        222 => {
            return RegfileExit::fault(0x70008c0686);
        }
        223 => {
            blk = 235; continue;
        }
        224 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c068c); };
            c.w64(11, t370);
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 235; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t371) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c069a); };
            c.w64(7, t371);
            blk = 61; continue;
        }
        226 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(7, c.r64(7).wrapping_add(c.r64(1).wrapping_mul(2)));
            let Some(t372) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c06ab); };
            c.w32(1, t372);
            let r373 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r373);
            blk = 63; continue;
        }
        227 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 232; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c06c3); };
            c.w64(0, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c06c7); };
            c.w64(6, t375);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c06cb); };
            c.w64(3, t376);
            blk = 235; continue;
        }
        230 => {
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c06d1); };
            if !c.push64(t377) { return RegfileExit::fault(0x70008c06d1); }
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c06d4); };
            if !c.push64(t378) { return RegfileExit::fault(0x70008c06d4); }
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c06d7); };
            if !c.push64(t379) { return RegfileExit::fault(0x70008c06d7); }
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c06da); };
            if !c.push64(t380) { return RegfileExit::fault(0x70008c06da); }
            let Some(t381) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008c06dd); };
            c.w64(3, t381);
            c.w32(0, 0x4u64);
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c06e9); };
            c.w64(6, t382);
            return c.call(0x700086ed74, 0x70008c06f2, 0x70008c06ed);
        }
        231 => {
            c.w64(2, c.r64(0));
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c06f5); };
            c.w64(0, t383);
            blk = 82; continue;
        }
        232 => {
            let Some(t384) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008c06fe); };
            c.w64(3, t384);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c0707); };
            c.w64(6, t385);
            return c.call(0x700086ed74, 0x70008c0710, 0x70008c070b);
        }
        233 => {
            return RegfileExit::fault(0x70008c0710);
        }
        234 => {
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c0711); };
            c.w64(0, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c0715); };
            c.w64(6, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c0719); };
            c.w64(3, t388);
            blk = 235; continue;
        }
        235 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c071d); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008c071e); }
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c071f); };
            if !c.push64(t389) { return RegfileExit::fault(0x70008c071f); }
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c0722); };
            if !c.push64(t390) { return RegfileExit::fault(0x70008c0722); }
            c.w32(0, 0x4u64);
            let Some(t391) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008c072a); };
            c.w64(3, t391);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008c073c, 0x70008c0737);
        }
        236 => {
            c.w64(2, c.r64(0));
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c073f); };
            c.w64(0, t392);
            blk = 82; continue;
        }
        237 => {
            return RegfileExit::fault(0x70008c0748);
        }
        238 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c0749); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008c074d); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008c0759, 0x70008c0754);
        }
        239 => {
            c.w64(7, c.r64(0));
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c075c); };
            c.w64(2, t393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c0760); };
            c.w64(8, t394);
            blk = 102; continue;
        }
        240 => {
            c.w32(2, 0x28u64);
            return c.call(0x70007febf4, 0x70008c0773, 0x70008c076e);
        }
        241 => {
            c.w64(7, c.r64(0));
            blk = 105; continue;
        }
        242 => {
            let Some(t395) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c077b); };
            c.w64(3, t395);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c078b, 0x70008c0786);
        }
        243 => {
            return RegfileExit::fault(0x70008c078b);
        }
        244 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008c078c); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008c0798, 0x70008c0793);
        }
        245 => {
            c.w64(15, c.r64(0));
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c079b); };
            c.w64(7, t396);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c079f); };
            c.w64(6, t397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c07a3); };
            c.w64(11, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c07a7); };
            c.w64(0, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c07ab); };
            c.w64(12, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c07af); };
            c.w64(3, t401);
            blk = 108; continue;
        }
        246 => {
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008c07bd); }
            return c.call(0x70007febf4, 0x70008c07c6, 0x70008c07c1);
        }
        247 => {
            c.w64(3, c.r64(0));
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c07c9); };
            c.w64(7, t402);
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c07cd); };
            c.w64(6, t403);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c07d1); };
            c.w64(11, t404);
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c07d5); };
            c.w64(0, t405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c07d9); };
            c.w64(12, t406);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c07dd); };
            c.w64(15, t407);
            blk = 111; continue;
        }
        248 => {
            return RegfileExit::fault(0x70008c07e6);
        }
        249 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008c07e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008c07eb); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008c07f7, 0x70008c07f2);
        }
        250 => {
            c.w64(7, c.r64(0));
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c07fa); };
            c.w64(8, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c07fe); };
            c.w64(2, t409);
            blk = 121; continue;
        }
        251 => {
            c.w32(2, 0x2cu64);
            return c.call(0x70007febf4, 0x70008c0811, 0x70008c080c);
        }
        252 => {
            c.w64(7, c.r64(0));
            blk = 124; continue;
        }
        253 => {
            let Some(t410) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c0819); };
            c.w64(3, t410);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c0829, 0x70008c0824);
        }
        254 => {
            return RegfileExit::fault(0x70008c0829);
        }
        255 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c082a); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008c0836, 0x70008c0831);
        }
        256 => {
            c.w64(15, c.r64(0));
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c0839); };
            c.w64(7, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c083d); };
            c.w64(6, t412);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c0841); };
            c.w64(11, t413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c0845); };
            c.w64(0, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c0849); };
            c.w64(12, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c084d); };
            c.w64(2, t416);
            blk = 127; continue;
        }
        257 => {
            c.w32(2, 0x2cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008c085b); }
            return c.call(0x70007febf4, 0x70008c0864, 0x70008c085f);
        }
        258 => {
            c.w64(3, c.r64(0));
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008c0867); };
            c.w64(7, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c086b); };
            c.w64(6, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c086f); };
            c.w64(11, t419);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c0873); };
            c.w64(0, t420);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c0877); };
            c.w64(12, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c087b); };
            c.w64(15, t422);
            blk = 130; continue;
        }
        259 => {
            return RegfileExit::fault(0x70008c0884);
        }
        260 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008c0885); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008c0889); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c088d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008c0891); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008c089d, 0x70008c0898);
        }
        261 => {
            c.w64(11, c.r64(0));
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c08a2); };
            c.w64(15, t423);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c08a6); };
            c.w64(12, t424);
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c08aa); };
            c.w64(7, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c08ae); };
            c.w64(3, t426);
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c08b2); };
            c.w64(6, t427);
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c08b6); };
            c.w64(2, t428);
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c08ba); };
            c.w64(8, t429);
            blk = 165; continue;
        }
        262 => {
            return RegfileExit::fault(0x70008c08c3);
        }
        263 => {
            return RegfileExit::fault(0x70008c08c4);
        }
        264 => {
            return RegfileExit::fault(0x70008c08c5);
        }
        265 => {
            return RegfileExit::fault(0x70008c08c6);
        }
        266 => {
            return RegfileExit::fault(0x70008c08c7);
        }
        267 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008c08c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c08cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008c08d0); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008c08dc, 0x70008c08d7);
        }
        268 => {
            c.w64(7, c.r64(0));
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c08e1); };
            c.w64(15, t430);
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c08e5); };
            c.w64(11, t431);
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c08e9); };
            c.w64(12, t432);
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c08ed); };
            c.w64(6, t433);
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c08f4); };
            c.w64(0, t434);
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c08f8); };
            c.w64(3, t435);
            blk = 177; continue;
        }
        269 => {
            return RegfileExit::fault(0x70008c0901);
        }
        270 => {
            return RegfileExit::fault(0x70008c0902);
        }
        271 => {
            return RegfileExit::fault(0x70008c0903);
        }
        272 => {
            return RegfileExit::fault(0x70008c0904);
        }
        273 => {
            return RegfileExit::fault(0x70008c0905);
        }
        274 => {
            return RegfileExit::fault(0x70008c0906);
        }
        275 => {
            return RegfileExit::fault(0x70008c0907);
        }
        276 => {
            return RegfileExit::fault(0x70008c0908);
        }
        277 => {
            return RegfileExit::fault(0x70008c0909);
        }
        278 => {
            return RegfileExit::fault(0x70008c090a);
        }
        279 => {
            return RegfileExit::fault(0x70008c090b);
        }
        280 => {
            c.w32(0, c.r32(0));
            let r436 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r436);
            c.w64(11, 0xfffffffffffc0000u64);
            let r437 = c.r64(11) & c.r64(0);
            c.w64(11, r437);
            c.logic(r437, 64);
            let Some(t438) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c091b); };
            c.logic(t438 & 0x2u64, 8);
            if c.cc_e() { blk = 216; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w64(12, c.r64(7).wrapping_add(0x27));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c092a); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c0934, 0x70008c092f);
        }
        282 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0934); }
            blk = 216; continue;
        }
        283 => {
            c.w32(2, c.r32(2));
            let r439 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r439);
            c.w64(3, 0xfffffffffffc0000u64);
            let r440 = c.r64(3) & c.r64(2);
            c.w64(3, r440);
            c.logic(r440, 64);
            let Some(t441) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c094a); };
            c.logic(t441 & 0x2u64, 8);
            if c.cc_e() { blk = 203; continue; }
            blk = 284; continue;
        }
        284 => {
            c.w64(6, c.r64(11).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0958); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c0959); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008c0966, 0x70008c0961);
        }
        285 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0966); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0968); }
            blk = 203; continue;
        }
        286 => {
            c.w32(0, c.r32(0));
            let r442 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r442);
            c.w64(12, 0xfffffffffffc0000u64);
            let r443 = c.r64(12) & c.r64(0);
            c.w64(12, r443);
            c.logic(r443, 64);
            let Some(t444) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c097d); };
            c.logic(t444 & 0x2u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 287; continue;
        }
        287 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c098d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c098e); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c099b, 0x70008c0996);
        }
        288 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c099b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c099d); }
            blk = 200; continue;
        }
        289 => {
            c.w32(7, c.r32(7));
            let r445 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r445);
            c.w64(0, 0xfffffffffffc0000u64);
            let r446 = c.r64(0) & c.r64(7);
            c.w64(0, r446);
            c.logic(r446, 64);
            let Some(t447) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c09b2); };
            c.logic(t447 & 0x2u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 290; continue;
        }
        290 => {
            c.w64(3, c.r64(15).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c09c0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c09c1); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c09ce, 0x70008c09c9);
        }
        291 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c09ce); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c09d0); }
            blk = 181; continue;
        }
        292 => {
            c.w32(7, c.r32(7));
            let r448 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r448);
            c.w64(3, 0xfffffffffffc0000u64);
            let r449 = c.r64(3) & c.r64(7);
            c.w64(3, r449);
            c.logic(r449, 64);
            let Some(t450) = c.ld8(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c09e5); };
            c.logic(t450 & 0x2u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 293; continue;
        }
        293 => {
            c.w64(2, c.r64(15).wrapping_add(0x1b));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c09f3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c09f4); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c0a01, 0x70008c09fc);
        }
        294 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0a01); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0a03); }
            blk = 158; continue;
        }
        295 => {
            c.w32(0, c.r32(0));
            let r451 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r451);
            c.w64(15, 0xfffffffffffc0000u64);
            let r452 = c.r64(15) & c.r64(0);
            c.w64(15, r452);
            c.logic(r452, 64);
            let Some(t453) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0a18); };
            c.logic(t453 & 0x2u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0a27); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c0a28); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c0a35, 0x70008c0a30);
        }
        297 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0a35); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0a37); }
            blk = 151; continue;
        }
        298 => {
            c.w32(0, c.r32(0));
            let r454 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r454);
            c.w64(8, 0xfffffffffffc0000u64);
            let r455 = c.r64(8) & c.r64(0);
            c.w64(8, r455);
            c.logic(r455, 64);
            let Some(t456) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0a4c); };
            c.logic(t456 & 0x2u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 299; continue;
        }
        299 => {
            c.w64(9, c.r64(12).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0a5c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c0a5d); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008c0a6a, 0x70008c0a65);
        }
        300 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0a6a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0a6c); }
            blk = 144; continue;
        }
        301 => {
            c.w32(2, c.r32(2));
            let r457 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r457);
            c.w64(8, 0xfffffffffffc0000u64);
            let r458 = c.r64(8) & c.r64(2);
            c.w64(8, r458);
            c.logic(r458, 64);
            let Some(t459) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0a81); };
            c.logic(t459 & 0x2u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(9, c.r64(3).wrapping_add(0x1f));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0a90); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c0a91); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008c0a9e, 0x70008c0a99);
        }
        303 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0a9e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0aa0); }
            blk = 138; continue;
        }
        304 => {
            c.w32(2, c.r32(2));
            let r460 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r460);
            c.w64(1, 0xfffffffffffc0000u64);
            let r461 = c.r64(1) & c.r64(2);
            c.w64(1, r461);
            c.logic(r461, 64);
            let Some(t462) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0ab5); };
            c.logic(t462 & 0x2u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w64(8, c.r64(3).wrapping_add(0x13));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0ac3); }
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c0acc, 0x70008c0ac7);
        }
        306 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0acc); }
            blk = 136; continue;
        }
        307 => {
            c.w32(0, c.r32(0));
            let r463 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r463);
            c.w64(7, 0xfffffffffffc0000u64);
            let r464 = c.r64(7) & c.r64(0);
            c.w64(7, r464);
            c.logic(r464, 64);
            let Some(t465) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0ae1); };
            c.logic(t465 & 0x2u64, 8);
            if c.cc_e() { blk = 76; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(12, c.r64(2).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0aef); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c0af0); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c0afd, 0x70008c0af8);
        }
        309 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0afd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0aff); }
            blk = 76; continue;
        }
        310 => {
            c.w32(0, c.r32(0));
            let r466 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r466);
            c.w64(7, 0xfffffffffffc0000u64);
            let r467 = c.r64(7) & c.r64(0);
            c.w64(7, r467);
            c.logic(r467, 64);
            let Some(t468) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0b14); };
            c.logic(t468 & 0x2u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(12, c.r64(2).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0b22); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c0b23); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c0b30, 0x70008c0b2b);
        }
        312 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c0b30); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c0b32); }
            blk = 74; continue;
        }
        313 => {
            return RegfileExit::jump(0x70008c0b3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008c0b54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008c0b54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008c0b58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c0b59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c0b5a); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008c0b5f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008c0b63); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 217; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c0b6d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0b79); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0b7f); };
            c.cmp(t5, 0x83eu64, 16);
            if c.cc_ne() { blk = 215; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008c0b8b); };
            c.w64(7, t6);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0b99); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0ba0); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 219; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0bac); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c0bb3); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0bba); };
            c.w32(12, t14);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0bc1); };
            c.w32(15, t15);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008c0bc8); }
            c.cmp(c.r64(12), 0x9au64, 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008c0bd5);
        }
        7 => {
            let Some(t16) = c.ld32(c.r64(11).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008c0bd6); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c0be0); };
            c.cmp(t18, c.r32(15), 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c0be6); };
            c.w32(15, t19);
            c.logic(c.r32(15) & 0x80000001u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(3, c.r64(7));
            return c.call(0x70008bf674, 0x70008c0bfb, 0x70008c0bf6);
        }
        10 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008c0bfb); };
            c.w64(1, t20);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008c0c02); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x10, 0x70008c0c09);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008c0c0c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008c0c13); }
            return c.ret(0x0, 0x70008c0c15);
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c0c16); };
            c.w32(15, t21);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let Some(t22) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c0c20); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            c.logic(c.r8(15) & 0x9u64, 8);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008c0c32); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008c0c36); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(2, 0x1u64);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        15 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0c5e); };
            c.w32(1, t24);
            let r25 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r25);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c0c68); }
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c0c72); }
            blk = 22; continue;
        }
        19 => {
            c.w64(2, c.r64(3));
            let r26 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r26);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 22; continue;
        }
        20 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            let r27 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r27);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008c0c8f); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
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
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_l() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            c.w32(1, 0x1u64);
            blk = 27; continue;
        }
        27 => {
            c.w32(2, 0x1u64);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 213; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(1, c.r64(3));
            blk = 29; continue;
        }
        29 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008c0cc2); }
            c.cmp(c.r64(12), 0x9bu64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x70008c0ccf);
        }
        31 => {
            let Some(t28) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0cd0); };
            c.w32(12, t28);
            let r29 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r29);
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0x273)) else { return RegfileExit::fault(0x70008c0cd7); };
            c.w32(11, t30);
            let r31 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r31);
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0ce1); };
            c.w32(12, t32);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(11)) { return RegfileExit::fault(0x70008c0ce7); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0cf7); };
            c.w32(9, t33);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 80; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c0d07); };
            c.w32(11, t34);
            let r35 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0d0e); };
            c.w32(1, t36);
            let r37 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r37);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008c0d14); }
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 221; continue;
        }
        37 => {
            blk = 40; continue;
        }
        38 => {
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c0d2e); };
            c.w64(2, t38);
            c.w64(3, c.r64(7));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c0d35); };
            c.w64(1, t39);
            return c.call(0x7000859a54, 0x70008c0d3e, 0x70008c0d39);
        }
        39 => {
            c.w64(11, c.r64(0));
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c0d41); };
            c.w64(0, t40);
            blk = 81; continue;
        }
        40 => {
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c0d4a); };
            c.w32(1, t41);
            c.cmp(c.r32(1), 0x7eu64, 32);
            if c.cc_a() { blk = 234; continue; }
            blk = 41; continue;
        }
        41 => {
            c.logic(c.r8(12) & 0x7u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 42; continue;
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c0d61); }
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c0d6a); }
            blk = 44; continue;
        }
        44 => {
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c0d6e); };
            c.w64(15, t42);
            let r43 = c.r32(15) & 0x7u64;
            c.w32(15, r43);
            c.logic(r43, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 49; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 48; continue;
        }
        48 => {
            blk = 234; continue;
        }
        49 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 234; continue;
        }
        52 => {
            let Some(t44) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c0da8); };
            c.w32(15, t44);
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008c0dac); };
            c.cmp(t45, c.r32(15), 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t46) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c0db9); };
            c.w32(7, t46);
            let r47 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r47);
            let Some(t48) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0dbf); };
            c.w32(15, t48);
            let r49 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r49);
            let Some(t50) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0dc6); };
            c.w32(15, t50);
            blk = 43; continue;
        }
        54 => {
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c0dcd); };
            c.w32(15, t51);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r52 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c0dda); };
            c.w32(7, t53);
            let r54 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r54);
            let Some(t55) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0de0); };
            c.w32(15, t55);
            let r56 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r56);
            let Some(t57) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0de7); };
            c.w32(15, t57);
            blk = 43; continue;
        }
        55 => {
            let Some(t58) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c0df1); };
            c.w32(7, t58);
            let r59 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c0df7); };
            c.w32(15, t60);
            let r61 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r61);
            let Some(t62) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c0dfe); };
            c.w32(15, t62);
            blk = 43; continue;
        }
        56 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 58; continue;
        }
        57 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008c0e0c); }
            c.w32(12, 0);
            c.logic(0, 32);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 223; continue; }
            blk = 59; continue;
        }
        59 => {
            let r63 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r63);
            blk = 60; continue;
        }
        60 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c0e26); };
            c.w64(2, t64);
            let r65 = c.r32(2) & 0x8u64;
            c.w32(2, r65);
            c.logic(r65, 32);
            let r66 = c.add(c.r64(12), c.r64(8), 64);
            c.w64(12, r66);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 61; continue;
        }
        61 => {
            let r67 = c.add(c.r64(15), c.r64(7), 64);
            c.w64(15, r67);
            let r68 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r68);
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c0e3f); };
            c.w32(12, t69);
            let r70 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r70);
            blk = 62; continue;
        }
        62 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008c0e46); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 234; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008c0e54); };
            c.w32(2, t71);
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2e70)) else { return RegfileExit::fault(0x70008c0e59); };
            c.w64(9, t72);
            let Some(t73) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c0e60); };
            c.w64(9, t73);
            c.cmp(c.r32(2), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(12, c.r64(2).wrapping_mul(8));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13));
            let Some(t74) = c.ld64(c.r64(2).wrapping_add(c.r64(12)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008c0e76); };
            c.w64(12, t74);
            blk = 66; continue;
        }
        65 => {
            let r75 = c.add(c.r64(12), 0x3fu64, 64);
            c.w64(12, r75);
            c.w64(2, c.r64(13));
            blk = 66; continue;
        }
        66 => {
            let r76 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r76);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w32(8, c.r32(8));
            c.w64(10, c.r64(4));
            let r77 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r77);
            let r78 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r78);
            c.logic(r78, 64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008c0e9e); }
            let r79 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r79);
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c0ea5); };
            c.w64(7, t80);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(7)) { return RegfileExit::fault(0x70008c0eac); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x70008c0eb1); }
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(9)) { return RegfileExit::fault(0x70008c0eb6); }
            if !c.st64(c.r64(4).wrapping_add(0x38), 0x1u64) { return RegfileExit::fault(0x70008c0ebb); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(2)) { return RegfileExit::fault(0x70008c0ec4); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(0)) { return RegfileExit::fault(0x70008c0ec9); }
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c0ece); };
            c.w64(1, t81);
            c.w64(2, c.r64(8));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c0ed5); };
            c.w64(8, t82);
            c.w64(9, c.r64(15));
            c.w64(10, 0x70008c0ee3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c0ee3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c0ee7); }
            return c.call(c.r64(12), 0x70008c0eee, 0x70008c0eeb);
        }
        67 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c0eee); }
            let Some(t83) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008c0ef6); };
            c.w64(4, t83);
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_e() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ne() { blk = 226; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c0f0e); };
            c.w64(0, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c0f12); };
            c.w64(6, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c0f16); };
            c.w64(7, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c0f1a); };
            c.w64(3, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c0f1e); };
            c.w64(15, t88);
            blk = 80; continue;
        }
        70 => {
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c0f27); };
            c.w64(11, t89);
            let Some(t90) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0f2b); };
            c.w32(7, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c0f2f); };
            c.w64(12, t91);
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008c0f33); };
            c.w32(12, t92);
            let r93 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r93);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r94 = c.sub(c.r32(7), 0x6u64, 32);
            c.w32(7, r94);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_l() { blk = 233; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c0f4e); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c0f52); };
            c.w64(0, t95);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008c0f56); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r96 = c.r64(7) & c.r64(11);
            c.w64(7, r96);
            c.logic(r96, 64);
            let Some(t97) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0f68); };
            c.logic(t97 & 0x4u64, 8);
            if c.cc_ne() { blk = 309; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008c0f72); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r98 = c.r64(7) & c.r64(11);
            c.w64(7, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c0f84); };
            c.logic(t99 & 0x4u64, 8);
            if c.cc_ne() { blk = 306; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if c.cc_e() { blk = 81; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(12, 0x13u64);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2e78)) else { return RegfileExit::fault(0x70008c0f9f); };
            c.w64(3, t100);
            blk = 78; continue;
        }
        77 => {
            let r101 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r101);
            blk = 78; continue;
        }
        78 => {
            let Some(t102) = c.ld32(c.r64(3).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008c0fac); };
            c.w32(2, t102);
            let r103 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r103);
            if !c.st32(c.r64(11).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70008c0fb2); }
            let r104 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r104);
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_ne() { blk = 77; continue; }
            blk = 79; continue;
        }
        79 => {
            blk = 81; continue;
        }
        80 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c0fc1); };
            c.w64(11, t105);
            c.w64(0, c.r64(7));
            blk = 81; continue;
        }
        81 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(11)) { return RegfileExit::fault(0x70008c0fcb); }
            let Some(t106) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c0fcf); };
            c.cmp(t106, c.r32(11), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t107) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c0fd8); };
            c.cmp(t107, 0x0u64, 32);
            if c.cc_e() { blk = 214; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t108) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c0fe2); };
            c.w64(7, t108);
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x70008c0fe8); }
            blk = 214; continue;
        }
        84 => {
            let Some(t109) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c0ff0); };
            c.cmp(t109, 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t110) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c0ff6); };
            c.w32(7, t110);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4u64, 64);
            if c.cc_a() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            return RegfileExit::fault(0x70008c1006);
        }
        87 => {
            let Some(t111) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c1007); };
            c.w32(12, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c100b); };
            c.w64(7, t112);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008c100f); }
            blk = 88; continue;
        }
        88 => {
            let Some(t113) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c1013); };
            c.w32(12, t113);
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c1017); };
            c.w32(3, t114);
            let r115 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c101e); };
            c.w32(1, t116);
            let r117 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r117);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008c1025); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008c1029); }
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c102d); };
            c.w64(6, t118);
            return c.call(0x700086fe54, 0x70008c1036, 0x70008c1031);
        }
        89 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1036); };
            c.w64(7, t119);
            let Some(t120) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c103a); };
            c.w32(11, t120);
            let r121 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r121);
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008c104f); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008c1053); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1061); };
            c.w64(6, t122);
            let Some(t123) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c1065); };
            c.w32(12, t123);
            let r124 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r124);
            let Some(t125) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c106c); };
            c.w32(12, t125);
            let r126 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r126);
            let Some(t127) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1074); };
            c.w32(15, t127);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa1u64, 64);
            if c.cc_a() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            return RegfileExit::fault(0x70008c1088);
        }
        92 => {
            let Some(t128) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c1089); };
            c.w64(15, (t128 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let Some(t129) = c.ld32(c.r64(12).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c1090); };
            c.w32(12, t129);
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r130 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r130);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c10a7); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 106; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(15, c.r64(3).wrapping_mul(2).wrapping_add(0x30));
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 103; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r64(2), 0x2000000u64, 64);
            if c.cc_a() { blk = 236; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 241; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t131) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c10e4); };
            c.w64(15, (t131 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r132 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r132);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c10f8); };
            c.w64(3, t133);
            let Some(t134) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c10ff); };
            c.w64(1, t134);
            c.w64(9, c.r64(8).wrapping_add(0x8));
            c.w64(7, c.r64(9).wrapping_add(c.r64(1)));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008c110a); }
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c110e); };
            c.w64(11, t135);
            let Some(t136) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c1115); };
            c.cmp(t136, c.r64(7), 64);
            if c.cc_be() { blk = 237; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r64(9), 0x20000u64, 64);
            if c.cc_ae() { blk = 237; continue; }
            blk = 98; continue;
        }
        98 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008c112b); }
            c.w64(7, c.r64(1).wrapping_add(0x1));
            blk = 99; continue;
        }
        99 => {
            let r137 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r137);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c1134); };
            c.w64(1, t138);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008c113b); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008c113e); }
            c.w64(1, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r139 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r139);
            let r140 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r140);
            c.logic(r140, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008c1150); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(7)) { return RegfileExit::fault(0x70008c1155); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008c115b); };
            c.w64(0, t141);
            c.w64(10, 0x70008c1169u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c1169); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c116d); }
            return c.call(c.r64(0), 0x70008c1173, 0x70008c1171);
        }
        100 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c1173); }
            let Some(t142) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008c117b); };
            c.w64(4, t142);
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c1180); };
            c.w64(7, t143);
            let Some(t144) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c1187); };
            c.w64(11, t144);
            c.w64(12, c.r64(11).wrapping_add(0x28));
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c118e); };
            c.w64(15, t145);
            let Some(t146) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c1195); };
            c.cmp(t146, c.r64(12), 64);
            if c.cc_be() { blk = 239; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008c119e); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 102; continue;
        }
        102 => {
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c11a5); };
            c.w64(11, t147);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c11a9); }
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c11ad); };
            c.w64(11, t148);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c11b1); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c11b5); };
            c.w64(11, t149);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c11bc); }
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c11c0); };
            c.w64(0, t150);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c11c4); }
            c.w64(3, c.r64(7));
            c.w64(12, c.r64(0));
            let Some(t151) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c11cd); };
            c.w64(7, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c11d1); };
            c.w64(6, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c11d5); };
            c.w64(11, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c11d9); };
            c.w64(0, t154);
            blk = 128; continue;
        }
        103 => {
            c.w32(3, c.r64(2).wrapping_add(c.r64(2)));
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c11e5); };
            c.w64(2, t155);
            let Some(t156) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008c11ec); };
            c.w64(1, t156);
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c11f3); };
            c.w64(9, t157);
            let Some(t158) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c11fa); };
            c.cmp(t158, c.r64(8), 64);
            if c.cc_be() { blk = 243; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008c1203); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 105; continue;
        }
        105 => {
            let Some(t159) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c120a); };
            c.w64(2, (t159 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if let Some(sr) = c.shr(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r160 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r160);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c1216); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008c121a); }
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c121e); };
            c.w64(12, t161);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c1225); }
            c.w64(12, c.r64(15).wrapping_add(0x28));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c122d); }
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c1231); };
            c.w64(2, t162);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c1238); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008c123d); }
            c.w64(3, c.r64(15));
            blk = 128; continue;
        }
        106 => {
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c124a); };
            c.w64(3, t163);
            let Some(t164) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c1251); };
            c.w64(2, t164);
            c.w64(1, c.r64(2).wrapping_add(0x28));
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c1258); };
            c.w64(8, t165);
            let Some(t166) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c125f); };
            c.cmp(t166, c.r64(1), 64);
            if c.cc_be() { blk = 245; continue; }
            blk = 107; continue;
        }
        107 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008c1268); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 108; continue;
        }
        108 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c126f); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008c1273); }
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c1277); };
            c.w64(12, t167);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c127e); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c1282); }
            blk = 128; continue;
        }
        109 => {
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c128b); };
            c.w64(6, t168);
            let Some(t169) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c128f); };
            c.w32(12, t169);
            let r170 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r170);
            let Some(t171) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c1296); };
            c.w32(12, t171);
            let r172 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r172);
            let Some(t173) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c129e); };
            c.w32(15, t173);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa2u64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return RegfileExit::fault(0x70008c12b2);
        }
        111 => {
            let Some(t174) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c12b3); };
            c.w64(15, (t174 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r175 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r175);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t176) = c.ld32(c.r64(12).wrapping_add(0x28f)) else { return RegfileExit::fault(0x70008c12c3); };
            c.w32(12, t176);
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008c12d1); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 125; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(15, c.r64(3).wrapping_mul(2).wrapping_add(0x34));
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_be() { blk = 122; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r64(2), 0x2000000u64, 64);
            if c.cc_a() { blk = 247; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r64(2), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 252; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t177) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c130e); };
            c.w64(15, (t177 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r178 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r178);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let r179 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r179);
            c.w64(8, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(8).wrapping_add(0x8));
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c1328); };
            c.w64(1, t180);
            let Some(t181) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008c132f); };
            c.w64(9, t181);
            c.w64(7, c.r64(3).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008c1336); }
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c133a); };
            c.w64(11, t182);
            let Some(t183) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c1341); };
            c.cmp(t183, c.r64(7), 64);
            if c.cc_be() { blk = 248; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 248; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008c1357); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 118; continue;
        }
        118 => {
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c135e); };
            c.w64(3, t184);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008c1365); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008c1368); }
            c.w64(3, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r185 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r185);
            let r186 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r186);
            c.logic(r186, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008c137a); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(7)) { return RegfileExit::fault(0x70008c137f); }
            c.w64(1, c.r64(3));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008c1388); };
            c.w64(0, t187);
            c.w64(10, 0x70008c1396u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008c1396); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008c139a); }
            return c.call(c.r64(0), 0x70008c13a0, 0x70008c139e);
        }
        119 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008c13a0); }
            let Some(t188) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008c13a8); };
            c.w64(4, t188);
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c13ad); };
            c.w64(7, t189);
            let Some(t190) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008c13b4); };
            c.w64(11, t190);
            c.w64(12, c.r64(11).wrapping_add(0x2c));
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c13bb); };
            c.w64(15, t191);
            let Some(t192) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008c13c2); };
            c.cmp(t192, c.r64(12), 64);
            if c.cc_be() { blk = 250; continue; }
            blk = 120; continue;
        }
        120 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008c13cb); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 121; continue;
        }
        121 => {
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c13d2); };
            c.w64(11, t193);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008c13d6); }
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008c13da); };
            c.w64(11, t194);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008c13de); }
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c13e2); };
            c.w64(11, t195);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008c13e9); }
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c13ed); };
            c.w64(0, t196);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c13f1); }
            c.w64(3, c.r64(7));
            c.w64(12, c.r64(0));
            let Some(t197) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c13fa); };
            c.w64(7, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c13fe); };
            c.w64(6, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1402); };
            c.w64(11, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1406); };
            c.w64(0, t200);
            blk = 128; continue;
        }
        122 => {
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c140f); };
            c.w64(3, t201);
            let Some(t202) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c1416); };
            c.w64(1, t202);
            c.w64(8, c.r64(15).wrapping_add(c.r64(1)));
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c141d); };
            c.w64(9, t203);
            let Some(t204) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c1424); };
            c.cmp(t204, c.r64(8), 64);
            if c.cc_be() { blk = 254; continue; }
            blk = 123; continue;
        }
        123 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008c142d); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 124; continue;
        }
        124 => {
            let Some(t205) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c1434); };
            c.w64(3, (t205 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r206 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r206);
            let r207 = c.shl(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r207);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c1442); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008c1446); }
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c144a); };
            c.w64(12, t208);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c1451); }
            c.w64(12, c.r64(15).wrapping_add(0x2c));
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c1459); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008c145d); };
            c.w64(3, t209);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008c1464); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008c1469); }
            c.w64(3, c.r64(15));
            blk = 128; continue;
        }
        125 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c1473); };
            c.w64(3, t210);
            let Some(t211) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008c147a); };
            c.w64(2, t211);
            c.w64(1, c.r64(2).wrapping_add(0x2c));
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c1481); };
            c.w64(8, t212);
            let Some(t213) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008c1488); };
            c.cmp(t213, c.r64(1), 64);
            if c.cc_be() { blk = 256; continue; }
            blk = 126; continue;
        }
        126 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008c1491); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 127; continue;
        }
        127 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008c1498); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008c149c); }
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c14a0); };
            c.w64(12, t214);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008c14a7); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c14ab); }
            blk = 128; continue;
        }
        128 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008c14af); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x70008c14b3); }
            let Some(t215) = c.ld8(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c14b7); };
            c.logic(t215 & 0x1u64, 8);
            if c.cc_e() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 131; continue;
        }
        130 => {
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c14c2); };
            c.w64(15, t216);
            blk = 131; continue;
        }
        131 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c14c6); };
            c.w64(2, t217);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x70008c14ca); }
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c14cd); };
            c.w64(2, t218);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008c14d1); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r219 = c.r64(1) & c.r64(3);
            c.w64(1, r219);
            c.logic(r219, 64);
            let Some(t220) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c14e3); };
            c.logic(t220 & 0x4u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c14ed); };
            c.w64(1, t221);
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70008c14f4); }
            if !c.st32(c.r64(3).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x70008c14f7); }
            if !c.st32(c.r64(3).wrapping_add(0x1f), c.r32(2)) { return RegfileExit::fault(0x70008c14fa); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r222 = c.r64(8) & c.r64(3);
            c.w64(8, r222);
            c.logic(r222, 64);
            let Some(t223) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c150c); };
            c.logic(t223 & 0x4u64, 8);
            if c.cc_ne() { blk = 300; continue; }
            blk = 135; continue;
        }
        135 => {
            if !c.st32(c.r64(3).wrapping_add(0x23), c.r32(15)) { return RegfileExit::fault(0x70008c1517); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.st32(c.r64(3).wrapping_add(0x27), c.r32(1)) { return RegfileExit::fault(0x70008c1521); }
            blk = 137; continue;
        }
        137 => {
            let Some(t224) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c1524); };
            c.w64(15, (t224 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r225 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r225);
            c.w64(8, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            let r226 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r226);
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x70008c1546); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 138; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008c1550); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r227 = c.r64(8) & c.r64(12);
            c.w64(8, r227);
            c.logic(r227, 64);
            let Some(t228) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1563); };
            c.logic(t228 & 0x4u64, 8);
            if c.cc_ne() { blk = 297; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t229) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c1578); };
            c.w64(15, (t229 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r230 = c.add(c.r64(15), 0x3u64, 64);
            c.w64(15, r230);
            c.w32(0, 0x1u64);
            c.w32(8, 0x5u64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008c158e); }
            blk = 144; continue;
        }
        143 => {
            c.w64(0, c.r64(7).wrapping_add(0x1));
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c1598); };
            c.w64(2, t231);
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c159c); };
            c.w64(1, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c15a3); };
            c.w64(6, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c15a7); };
            c.w64(3, t234);
            blk = 144; continue;
        }
        144 => {
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c15ab); };
            c.w64(9, t235);
            let Some(t236) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c15af); };
            c.w32(7, t236);
            let r237 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r237);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008c15b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008c15bb); }
            c.cmp(c.r32(7), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 149; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(7, c.r64(8).wrapping_add(0x1));
            let Some(t238) = c.ld32(c.r64(9).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c15c8); };
            c.w32(7, t238);
            let r239 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r239);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c15d0); }
            c.w64(0, c.r64(2));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c15d7); };
            c.w64(3, t240);
            c.w64(1, c.r64(7));
            return c.call(0x700086fe54, 0x70008c15e3, 0x70008c15de);
        }
        146 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c15e3); };
            c.w64(7, t241);
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c15ef); };
            c.w64(12, t242);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008c15f3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r243 = c.r64(15) & c.r64(12);
            c.w64(15, r243);
            c.logic(r243, 64);
            let Some(t244) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1605); };
            c.logic(t244 & 0x4u64, 8);
            if c.cc_ne() { blk = 294; continue; }
            blk = 148; continue;
        }
        148 => {
            blk = 150; continue;
        }
        149 => {
            c.w64(7, c.r64(0));
            blk = 150; continue;
        }
        150 => {
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c1615); };
            c.w64(8, t245);
            let r246 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c161d); };
            c.cmp(c.r64(8), t247, 64);
            if c.cc_b() { blk = 143; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1627); };
            c.w64(7, t248);
            let Some(t249) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c162b); };
            c.w32(11, t249);
            let r250 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r250);
            let Some(t251) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1632); };
            c.w32(15, t251);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x9u64, 64);
            if c.cc_be() { blk = 258; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t252) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008c1646); };
            c.w32(11, t252);
            let r253 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r253);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008c164d); }
            c.w32(15, 0);
            c.logic(0, 32);
            c.cmp(c.r32(15), c.r32(11), 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t254) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c165d); };
            c.w32(0, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1661); };
            c.w64(3, t255);
            if !c.st32(c.r64(3).wrapping_add(0x1b), c.r32(11)) { return RegfileExit::fault(0x70008c1665); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r256 = c.r64(2) & c.r64(3);
            c.w64(2, r256);
            c.logic(r256, 64);
            let Some(t257) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1679); };
            c.logic(t257 & 0x4u64, 8);
            if c.cc_ne() { blk = 291; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1683); };
            c.w64(6, t258);
            let Some(t259) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c1687); };
            c.w32(2, t259);
            let r260 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r260);
            let Some(t261) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c168d); };
            c.w32(2, t261);
            let r262 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r262);
            let Some(t263) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1693); };
            c.w32(1, t263);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xb1u64, 64);
            if c.cc_a() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            return RegfileExit::fault(0x70008c16a5);
        }
        157 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(1, c.r64(0));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(1, c.r64(8).wrapping_add(c.r64(1)).wrapping_sub(0x1));
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r264 = c.r64(8) | c.r64(1);
            c.w64(8, r264);
            c.logic(r264, 64);
            c.w64(1, c.r64(8));
            if let Some(sr) = c.shr(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            let r265 = c.r64(1) | c.r64(8);
            c.w64(1, r265);
            c.logic(r265, 64);
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 0x4u64, 64) { c.w64(8, sr); }
            let r266 = c.r64(8) | c.r64(1);
            c.w64(8, r266);
            c.logic(r266, 64);
            c.w64(1, c.r64(8));
            if let Some(sr) = c.shr(c.r64(1), 0x8u64, 64) { c.w64(1, sr); }
            let r267 = c.r64(1) | c.r64(8);
            c.w64(1, r267);
            c.logic(r267, 64);
            c.w64(8, c.r64(1));
            if let Some(sr) = c.shr(c.r64(8), 0x10u64, 64) { c.w64(8, sr); }
            let r268 = c.r64(8) | c.r64(1);
            c.w64(8, r268);
            c.logic(r268, 64);
            c.w64(1, c.r64(8).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008c16f2); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_ge() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w32(1, 0x4u64);
            blk = 159; continue;
        }
        159 => {
            let Some(t269) = c.ld32(c.r64(2).wrapping_add(0x2cb)) else { return RegfileExit::fault(0x70008c1701); };
            c.w32(2, t269);
            let r270 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r270);
            c.w64(8, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c170f); };
            c.w64(9, t271);
            let Some(t272) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008c1716); };
            c.w64(9, t272);
            c.w64(6, c.r64(8).wrapping_mul(4).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(9)));
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c1725); };
            c.w64(0, t273);
            let Some(t274) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008c172c); };
            c.cmp(t274, c.r64(7), 64);
            if c.cc_be() { blk = 259; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r64(6), 0x20000u64, 64);
            if c.cc_ae() { blk = 259; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c1742); };
            c.w64(0, t275);
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x70008c1749); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 162; continue;
        }
        162 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008c1754); };
            c.w64(8, t276);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008c175b); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008c175f); }
            let Some(t277) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1762); };
            c.w32(0, t277);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_be() { blk = 261; continue; }
            blk = 163; continue;
        }
        163 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008c1775); }
            let Some(t278) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1779); };
            c.w32(0, t278);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 262; continue; }
            blk = 164; continue;
        }
        164 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008c178c); }
            let Some(t279) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1790); };
            c.w32(0, t279);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x2u64, 64);
            if c.cc_be() { blk = 263; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008c17a6); }
            let Some(t280) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c17a9); };
            c.w32(0, t280);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_be() { blk = 264; continue; }
            blk = 166; continue;
        }
        166 => {
            c.w32(0, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x70008c17c1); }
            let Some(t281) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c17c4); };
            c.w32(0, t281);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x4u64, 64);
            if c.cc_be() { blk = 265; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008c17d7); }
            c.w64(0, c.r64(7).wrapping_add(c.r64(6)).wrapping_sub(0x1));
            c.w64(1, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(1), c.r64(0), 64);
            if c.cc_e() { blk = 171; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c17e9); };
            c.w64(6, t282);
            blk = 169; continue;
        }
        169 => {
            if !c.st32(c.r64(1), c.r32(6)) { return RegfileExit::fault(0x70008c17f0); }
            let r283 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r283);
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_ne() { blk = 169; continue; }
            blk = 170; continue;
        }
        170 => {
            blk = 172; continue;
        }
        171 => {
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c17fd); };
            c.w64(6, t284);
            blk = 172; continue;
        }
        172 => {
            let Some(t285) = c.ld8(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1804); };
            c.w32(0, t285);
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008c1808); };
            c.w64(1, t286);
            let Some(t287) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008c180f); };
            c.w64(8, t287);
            if let Some(sr) = c.shl(c.r64(0), 0x2u64, 64) { c.w64(0, sr); }
            c.w64(9, c.r64(0).wrapping_add(c.r64(8)));
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008c181a); };
            c.w64(11, t288);
            let Some(t289) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008c1821); };
            c.cmp(t289, c.r64(9), 64);
            if c.cc_be() { blk = 266; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008c182a); }
            c.w64(11, c.r64(8).wrapping_add(0x1));
            blk = 174; continue;
        }
        174 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008c1831); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008c1835); }
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008c1839); };
            c.w64(2, t290);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008c1840); }
            let r291 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r291);
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let r292 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r292);
            if !c.st32(c.r64(0).wrapping_add(c.r64(11)), c.r32(6)) { return RegfileExit::fault(0x70008c1852); }
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_ne() { blk = 175; continue; }
            blk = 176; continue;
        }
        176 => {
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008c185c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r293 = c.r64(0) & c.r64(3);
            c.w64(0, r293);
            c.logic(r293, 64);
            let Some(t294) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1870); };
            c.logic(t294 & 0x4u64, 8);
            if c.cc_ne() { blk = 288; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(0, c.r64(15));
            blk = 180; continue;
        }
        179 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c187f); };
            c.w64(12, t295);
            blk = 180; continue;
        }
        180 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c1883); };
            c.w64(11, t296);
            let Some(t297) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1887); };
            c.w32(1, t297);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(0), c.r64(1), 64);
            if c.cc_ae() { blk = 268; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(6, c.r64(0).wrapping_add(0x1));
            let Some(t298) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c189e); };
            c.w32(0, t298);
            let r299 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r299);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008c18a6); }
            c.cmp(c.r64(6), c.r64(1), 64);
            if c.cc_ae() { blk = 269; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t300) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c18b3); };
            c.w32(1, t300);
            let Some(t301) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c18b8); };
            c.w32(8, t301);
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_ae() { blk = 270; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t302) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c18c6); };
            c.w32(8, t302);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t303) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c18d0); };
            c.w32(9, t303);
            let Some(t304) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c18d4); };
            c.w32(1, t304);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 271; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t305) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c18e6); };
            c.w32(3, t305);
            let r306 = c.add(c.r32(3), 0x2u64, 32);
            c.w32(3, r306);
            c.w64(11, c.r64(3));
            let r307 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r307);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r308 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r308);
            c.logic(r308, 64);
            let r309 = c.add(c.r32(11), c.r32(3), 32);
            c.w32(11, r309);
            c.w32(12, c.r32(9));
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_b() { blk = 278; continue; }
            blk = 185; continue;
        }
        185 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 272; continue; }
            blk = 186; continue;
        }
        186 => {
            let r310 = c.sub(c.r32(12), c.r32(3), 32);
            c.w32(12, r310);
            let r311 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r311);
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t312) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008c191b); };
            c.w32(12, t312);
            let r313 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r313);
            c.logic(r313, 64);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_b() { blk = 278; continue; }
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 273; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t314) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c1936); };
            c.w32(11, t314);
            let r315 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r315);
            c.w32(12, c.r64(11).wrapping_add(0x2));
            c.cmp(c.r32(12), 0xfffffeu64, 32);
            if c.cc_a() { blk = 278; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008c1951); }
            let Some(t316) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1955); };
            c.w32(12, t316);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 274; continue; }
            blk = 190; continue;
        }
        190 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008c196c); }
            let Some(t317) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008c1970); };
            c.w32(12, t317);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r318 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r318);
            let Some(t319) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c197e); };
            c.w32(3, t319);
            let r320 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r320);
            let r321 = c.r64(3) & c.r64(12);
            c.w64(3, r321);
            c.logic(r321, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 192; continue;
        }
        191 => {
            let r322 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r322);
            c.w64(15, c.r64(8).wrapping_add(c.r64(3)));
            c.w64(3, c.r64(12));
            let r323 = c.r64(3) & c.r64(15);
            c.w64(3, r323);
            c.logic(r323, 64);
            blk = 192; continue;
        }
        192 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(15, c.r64(9).wrapping_mul(4));
            c.w64(2, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t324) = c.ld32(c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c19af); };
            c.w32(6, t324);
            let Some(t325) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c19b4); };
            c.cmp(t325, c.r32(6), 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t326) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008c19bd); };
            c.cmp(t326, c.r32(6), 32);
            if c.cc_ne() { blk = 191; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t327) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c19c6); };
            c.w32(12, t327);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 275; continue; }
            blk = 195; continue;
        }
        195 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(2)), c.r32(0)) { return RegfileExit::fault(0x70008c19d9); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r328 = c.r64(12) & c.r64(7);
            c.w64(12, r328);
            c.logic(r328, 64);
            let Some(t329) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c19ea); };
            c.logic(t329 & 0x4u64, 8);
            if c.cc_ne() { blk = 285; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t330) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c19f6); };
            c.w32(12, t330);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(3, c.r64(9).wrapping_add(0x1));
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_ae() { blk = 276; continue; }
            blk = 198; continue;
        }
        198 => {
            if let Some(sr) = c.shl(c.r64(11), 0x8u64, 64) { c.w64(11, sr); }
            c.w32(12, c.r32(11));
            c.w64(3, c.r64(9).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)), c.r32(1)) { return RegfileExit::fault(0x70008c1a1c); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r331 = c.r64(2) & c.r64(7);
            c.w64(2, r331);
            c.logic(r331, 64);
            let Some(t332) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1a2e); };
            c.logic(t332 & 0x4u64, 8);
            if c.cc_ne() { blk = 282; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t333) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008c1a38); };
            c.w32(3, t333);
            let Some(t334) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008c1a3b); };
            c.cmp(t334, c.r32(3), 32);
            if c.cc_ne() { blk = 203; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t335) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008c1a44); };
            c.logic(t335 & 0x1u64, 8);
            if c.cc_e() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let r336 = c.r32(12) | 0x20u64;
            c.w32(12, r336);
            c.logic(r336, 32);
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 204; continue;
        }
        203 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 204; continue;
        }
        204 => {
            let Some(t337) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008c1a56); };
            c.w32(12, t337);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(0, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r64(0), c.r64(12), 64);
            if c.cc_ae() { blk = 277; continue; }
            blk = 205; continue;
        }
        205 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008c1a6d); }
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1a72); };
            c.w64(11, t338);
            c.w64(0, c.r64(11).wrapping_add(0x1));
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008c1a7a); };
            c.w64(11, t339);
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_g() { blk = 179; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t340) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1a87); };
            c.w64(7, t340);
            blk = 207; continue;
        }
        207 => {
            let Some(t341) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1a8b); };
            c.cmp(t341, 0x0u64, 32);
            if c.cc_ne() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1a91); };
            c.w64(7, t342);
            blk = 212; continue;
        }
        209 => {
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1a97); };
            c.w64(0, t343);
            let Some(t344) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008c1a9b); };
            c.w32(11, t344);
            let r345 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r345);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1aa2); }
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c1aa3); };
            if !c.push64(t346) { return RegfileExit::fault(0x70008c1aa3); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008c1aa6); }
            let Some(t347) = c.ld64(c.r64(13).wrapping_add(0x2640)) else { return RegfileExit::fault(0x70008c1aa8); };
            c.w64(3, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1aaf); };
            c.w64(6, t348);
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008c1ac0, 0x70008c1abb);
        }
        210 => {
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1ac0); };
            c.w64(7, t349);
            if !c.st32(c.r64(7).wrapping_add(0x27), c.r32(0)) { return RegfileExit::fault(0x70008c1ac4); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r350 = c.r64(11) & c.r64(7);
            c.w64(11, r350);
            c.logic(r350, 64);
            let Some(t351) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1ad5); };
            c.logic(t351 & 0x4u64, 8);
            if c.cc_ne() { blk = 279; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(0, c.r64(7));
            blk = 10; continue;
        }
        213 => {
            c.w64(7, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008c1aee); }
            blk = 214; continue;
        }
        214 => {
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008c1af2); };
            c.w64(0, t352);
            blk = 10; continue;
        }
        215 => {
            c.w32(7, 0x80u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1b03); }
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008c1b04); };
            c.w64(7, t353);
            let Some(t354) = c.ld32(c.r64(7).wrapping_add(0x28b)) else { return RegfileExit::fault(0x70008c1b0b); };
            c.w32(7, t354);
            let r355 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r355);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1b14); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c1b15); }
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008c1b16); };
            c.w64(3, t356);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008c1b27, 0x70008c1b22);
        }
        216 => {
            return RegfileExit::fault(0x70008c1b27);
        }
        217 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008c1b2a); };
            c.w64(3, t357);
            return c.call(0x700086ed74, 0x70008c1b36, 0x70008c1b31);
        }
        218 => {
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1b36); };
            c.w64(6, t358);
            blk = 1; continue;
        }
        219 => {
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008c1b47, 0x70008c1b42);
        }
        220 => {
            c.w64(7, c.r64(0));
            let Some(t359) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1b4a); };
            c.w64(0, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1b4e); };
            c.w64(6, t360);
            blk = 5; continue;
        }
        221 => {
            return RegfileExit::fault(0x70008c1b57);
        }
        222 => {
            blk = 234; continue;
        }
        223 => {
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1b5d); };
            c.w64(15, t361);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 234; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t362) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008c1b6b); };
            c.w64(7, t362);
            blk = 60; continue;
        }
        225 => {
            c.w64(15, c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(7, c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)));
            let Some(t363) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008c1b7c); };
            c.w32(12, t363);
            let r364 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r364);
            blk = 62; continue;
        }
        226 => {
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 231; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r64(7), 0xfffffffffffffffdu64, 64);
            if c.cc_e() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t365) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1b94); };
            c.w64(0, t365);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1b98); };
            c.w64(6, t366);
            blk = 234; continue;
        }
        229 => {
            let Some(t367) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1b9e); };
            if !c.push64(t367) { return RegfileExit::fault(0x70008c1b9e); }
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c1ba1); };
            if !c.push64(t368) { return RegfileExit::fault(0x70008c1ba1); }
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1ba4); };
            if !c.push64(t369) { return RegfileExit::fault(0x70008c1ba4); }
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c1ba7); };
            if !c.push64(t370) { return RegfileExit::fault(0x70008c1ba7); }
            let Some(t371) = c.ld64(c.r64(13).wrapping_add(0x2658)) else { return RegfileExit::fault(0x70008c1baa); };
            c.w64(3, t371);
            c.w32(0, 0x4u64);
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1bb6); };
            c.w64(6, t372);
            return c.call(0x700086ed74, 0x70008c1bbf, 0x70008c1bba);
        }
        230 => {
            c.w64(11, c.r64(0));
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c1bc2); };
            c.w64(0, t373);
            blk = 81; continue;
        }
        231 => {
            let Some(t374) = c.ld64(c.r64(13).wrapping_add(0x2230)) else { return RegfileExit::fault(0x70008c1bcb); };
            c.w64(3, t374);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1bd4); };
            c.w64(6, t375);
            return c.call(0x700086ed74, 0x70008c1bdd, 0x70008c1bd8);
        }
        232 => {
            return RegfileExit::fault(0x70008c1bdd);
        }
        233 => {
            let Some(t376) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1bde); };
            c.w64(0, t376);
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1be2); };
            c.w64(6, t377);
            blk = 234; continue;
        }
        234 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008c1be6); }
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c1be7); };
            if !c.push64(t378) { return RegfileExit::fault(0x70008c1be7); }
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1bea); };
            if !c.push64(t379) { return RegfileExit::fault(0x70008c1bea); }
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008c1bed); };
            if !c.push64(t380) { return RegfileExit::fault(0x70008c1bed); }
            c.w32(0, 0x4u64);
            let Some(t381) = c.ld64(c.r64(13).wrapping_add(0x2648)) else { return RegfileExit::fault(0x70008c1bf5); };
            c.w64(3, t381);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008c1c07, 0x70008c1c02);
        }
        235 => {
            c.w64(11, c.r64(0));
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c1c0a); };
            c.w64(0, t382);
            blk = 81; continue;
        }
        236 => {
            return RegfileExit::fault(0x70008c1c13);
        }
        237 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008c1c14); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008c1c18); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febb4, 0x70008c1c24, 0x70008c1c1f);
        }
        238 => {
            c.w64(7, c.r64(0));
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c1c27); };
            c.w64(2, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c1c2b); };
            c.w64(8, t384);
            blk = 99; continue;
        }
        239 => {
            c.w32(2, 0x28u64);
            return c.call(0x70007febf4, 0x70008c1c3e, 0x70008c1c39);
        }
        240 => {
            c.w64(7, c.r64(0));
            blk = 102; continue;
        }
        241 => {
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c1c46); };
            c.w64(3, t385);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c1c56, 0x70008c1c51);
        }
        242 => {
            return RegfileExit::fault(0x70008c1c56);
        }
        243 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x70008c1c57); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008c1c63, 0x70008c1c5e);
        }
        244 => {
            c.w64(15, c.r64(0));
            let Some(t386) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1c66); };
            c.w64(7, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1c6a); };
            c.w64(6, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1c6e); };
            c.w64(11, t388);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1c72); };
            c.w64(0, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1c76); };
            c.w64(12, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c1c7a); };
            c.w64(3, t391);
            blk = 105; continue;
        }
        245 => {
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x70008c1c88); }
            return c.call(0x70007febf4, 0x70008c1c91, 0x70008c1c8c);
        }
        246 => {
            c.w64(3, c.r64(0));
            let Some(t392) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1c94); };
            c.w64(7, t392);
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1c98); };
            c.w64(6, t393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1c9c); };
            c.w64(11, t394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1ca0); };
            c.w64(0, t395);
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1ca4); };
            c.w64(12, t396);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c1ca8); };
            c.w64(15, t397);
            blk = 108; continue;
        }
        247 => {
            return RegfileExit::fault(0x70008c1cb1);
        }
        248 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008c1cb2); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(2)) { return RegfileExit::fault(0x70008c1cb6); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008c1cc2, 0x70008c1cbd);
        }
        249 => {
            c.w64(7, c.r64(0));
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c1cc5); };
            c.w64(8, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c1cc9); };
            c.w64(2, t399);
            blk = 118; continue;
        }
        250 => {
            c.w32(2, 0x2cu64);
            return c.call(0x70007febf4, 0x70008c1cdc, 0x70008c1cd7);
        }
        251 => {
            c.w64(7, c.r64(0));
            blk = 121; continue;
        }
        252 => {
            let Some(t400) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008c1ce4); };
            c.w64(3, t400);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008c1cf4, 0x70008c1cef);
        }
        253 => {
            return RegfileExit::fault(0x70008c1cf4);
        }
        254 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(2)) { return RegfileExit::fault(0x70008c1cf5); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008c1d01, 0x70008c1cfc);
        }
        255 => {
            c.w64(15, c.r64(0));
            let Some(t401) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1d04); };
            c.w64(7, t401);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1d08); };
            c.w64(6, t402);
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1d0c); };
            c.w64(11, t403);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1d10); };
            c.w64(0, t404);
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1d14); };
            c.w64(12, t405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c1d18); };
            c.w64(2, t406);
            blk = 124; continue;
        }
        256 => {
            c.w32(2, 0x2cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x70008c1d26); }
            return c.call(0x70007febf4, 0x70008c1d2f, 0x70008c1d2a);
        }
        257 => {
            c.w64(3, c.r64(0));
            let Some(t407) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008c1d32); };
            c.w64(7, t407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008c1d36); };
            c.w64(6, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008c1d3a); };
            c.w64(11, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1d3e); };
            c.w64(0, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1d42); };
            c.w64(12, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c1d46); };
            c.w64(15, t412);
            blk = 127; continue;
        }
        258 => {
            return RegfileExit::fault(0x70008c1d4f);
        }
        259 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c1d50); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008c1d54); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008c1d58); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008c1d5c); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febb4, 0x70008c1d68, 0x70008c1d63);
        }
        260 => {
            c.w64(7, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1d6e); };
            c.w64(3, t413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c1d72); };
            c.w64(12, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008c1d76); };
            c.w64(11, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c1d7a); };
            c.w64(2, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1d7e); };
            c.w64(6, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008c1d82); };
            c.w64(1, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008c1d86); };
            c.w64(8, t419);
            blk = 162; continue;
        }
        261 => {
            return RegfileExit::fault(0x70008c1d8f);
        }
        262 => {
            return RegfileExit::fault(0x70008c1d90);
        }
        263 => {
            return RegfileExit::fault(0x70008c1d91);
        }
        264 => {
            return RegfileExit::fault(0x70008c1d92);
        }
        265 => {
            return RegfileExit::fault(0x70008c1d93);
        }
        266 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008c1d94); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008c1d98); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008c1d9c); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008c1da8, 0x70008c1da3);
        }
        267 => {
            c.w64(11, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008c1dae); };
            c.w64(3, t420);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008c1db2); };
            c.w64(7, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008c1db6); };
            c.w64(12, t422);
            let Some(t423) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008c1dba); };
            c.w64(6, t423);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008c1dc1); };
            c.w64(0, t424);
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008c1dc5); };
            c.w64(2, t425);
            blk = 174; continue;
        }
        268 => {
            return RegfileExit::fault(0x70008c1dce);
        }
        269 => {
            return RegfileExit::fault(0x70008c1dcf);
        }
        270 => {
            return RegfileExit::fault(0x70008c1dd0);
        }
        271 => {
            return RegfileExit::fault(0x70008c1dd1);
        }
        272 => {
            return RegfileExit::fault(0x70008c1dd2);
        }
        273 => {
            return RegfileExit::fault(0x70008c1dd3);
        }
        274 => {
            return RegfileExit::fault(0x70008c1dd4);
        }
        275 => {
            return RegfileExit::fault(0x70008c1dd5);
        }
        276 => {
            return RegfileExit::fault(0x70008c1dd6);
        }
        277 => {
            return RegfileExit::fault(0x70008c1dd7);
        }
        278 => {
            return RegfileExit::fault(0x70008c1dd8);
        }
        279 => {
            c.w32(0, c.r32(0));
            let r426 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r426);
            c.w64(11, 0xfffffffffffc0000u64);
            let r427 = c.r64(11) & c.r64(0);
            c.w64(11, r427);
            c.logic(r427, 64);
            let Some(t428) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1de8); };
            c.logic(t428 & 0x2u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 280; continue;
        }
        280 => {
            c.w64(12, c.r64(7).wrapping_add(0x27));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1df7); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008c1e01, 0x70008c1dfc);
        }
        281 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1e01); }
            blk = 212; continue;
        }
        282 => {
            c.w32(1, c.r32(1));
            let r429 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r429);
            c.w64(2, 0xfffffffffffc0000u64);
            let r430 = c.r64(2) & c.r64(1);
            c.w64(2, r430);
            c.logic(r430, 64);
            let Some(t431) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1e17); };
            c.logic(t431 & 0x2u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(6, c.r64(7).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1e25); }
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008c1e2f, 0x70008c1e2a);
        }
        284 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1e2f); }
            blk = 200; continue;
        }
        285 => {
            c.w32(0, c.r32(0));
            let r432 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r432);
            c.w64(12, 0xfffffffffffc0000u64);
            let r433 = c.r64(12) & c.r64(0);
            c.w64(12, r433);
            c.logic(r433, 64);
            let Some(t434) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1e45); };
            c.logic(t434 & 0x2u64, 8);
            if c.cc_e() { blk = 197; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w64(3, c.r64(7).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1e55); }
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c1e5f, 0x70008c1e5a);
        }
        287 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1e5f); }
            blk = 197; continue;
        }
        288 => {
            c.w32(11, c.r32(11));
            let r435 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r435);
            c.w64(0, 0xfffffffffffc0000u64);
            let r436 = c.r64(0) & c.r64(11);
            c.w64(0, r436);
            c.logic(r436, 64);
            let Some(t437) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1e76); };
            c.logic(t437 & 0x2u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w64(2, c.r64(3).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1e84); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1e85); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(2));
            return c.call(0x70007f0e54, 0x70008c1e92, 0x70008c1e8d);
        }
        290 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1e92); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1e94); }
            blk = 178; continue;
        }
        291 => {
            c.w32(11, c.r32(11));
            let r438 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r438);
            c.w64(2, 0xfffffffffffc0000u64);
            let r439 = c.r64(2) & c.r64(11);
            c.w64(2, r439);
            c.logic(r439, 64);
            let Some(t440) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1eaa); };
            c.logic(t440 & 0x2u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w64(1, c.r64(3).wrapping_add(0x1b));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1eb8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1eb9); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x70008c1ec6, 0x70008c1ec1);
        }
        293 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1ec6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1ec8); }
            blk = 155; continue;
        }
        294 => {
            c.w32(0, c.r32(0));
            let r441 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r441);
            c.w64(15, 0xfffffffffffc0000u64);
            let r442 = c.r64(15) & c.r64(0);
            c.w64(15, r442);
            c.logic(r442, 64);
            let Some(t443) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1edd); };
            c.logic(t443 & 0x2u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1eec); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1eed); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c1efa, 0x70008c1ef5);
        }
        296 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1efa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1efc); }
            blk = 148; continue;
        }
        297 => {
            c.w32(0, c.r32(0));
            let r444 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r444);
            c.w64(8, 0xfffffffffffc0000u64);
            let r445 = c.r64(8) & c.r64(0);
            c.w64(8, r445);
            c.logic(r445, 64);
            let Some(t446) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1f11); };
            c.logic(t446 & 0x2u64, 8);
            if c.cc_e() { blk = 141; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(9, c.r64(12).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1f21); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1f22); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008c1f2f, 0x70008c1f2a);
        }
        299 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1f2f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1f31); }
            blk = 141; continue;
        }
        300 => {
            c.w32(2, c.r32(2));
            let r447 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r447);
            c.w64(8, 0xfffffffffffc0000u64);
            let r448 = c.r64(8) & c.r64(2);
            c.w64(8, r448);
            c.logic(r448, 64);
            let Some(t449) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1f46); };
            c.logic(t449 & 0x2u64, 8);
            if c.cc_e() { blk = 135; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(9, c.r64(3).wrapping_add(0x1f));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1f55); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1f56); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008c1f63, 0x70008c1f5e);
        }
        302 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1f63); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1f65); }
            blk = 135; continue;
        }
        303 => {
            c.w32(2, c.r32(2));
            let r450 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r450);
            c.w64(1, 0xfffffffffffc0000u64);
            let r451 = c.r64(1) & c.r64(2);
            c.w64(1, r451);
            c.logic(r451, 64);
            let Some(t452) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1f7a); };
            c.logic(t452 & 0x2u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 304; continue;
        }
        304 => {
            c.w64(8, c.r64(3).wrapping_add(0x13));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1f88); }
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x70008c1f91, 0x70008c1f8c);
        }
        305 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1f91); }
            blk = 133; continue;
        }
        306 => {
            c.w32(0, c.r32(0));
            let r453 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r453);
            c.w64(7, 0xfffffffffffc0000u64);
            let r454 = c.r64(7) & c.r64(0);
            c.w64(7, r454);
            c.logic(r454, 64);
            let Some(t455) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1fa6); };
            c.logic(t455 & 0x2u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 307; continue;
        }
        307 => {
            c.w64(15, c.r64(11).wrapping_add(0xf));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1fb4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1fb5); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c1fc2, 0x70008c1fbd);
        }
        308 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1fc2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1fc4); }
            blk = 75; continue;
        }
        309 => {
            c.w32(0, c.r32(0));
            let r456 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r456);
            c.w64(7, 0xfffffffffffc0000u64);
            let r457 = c.r64(7) & c.r64(0);
            c.w64(7, r457);
            c.logic(r457, 64);
            let Some(t458) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008c1fd9); };
            c.logic(t458 & 0x2u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 310; continue;
        }
        310 => {
            c.w64(15, c.r64(11).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008c1fe7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008c1fe8); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008c1ff5, 0x70008c1ff0);
        }
        311 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008c1ff5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008c1ff7); }
            blk = 73; continue;
        }
        312 => {
            return RegfileExit::jump(0x70008c2000);
        }
            _ => unreachable!(),
        }
    }
}
