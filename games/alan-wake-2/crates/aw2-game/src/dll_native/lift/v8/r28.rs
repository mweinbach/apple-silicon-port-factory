#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008da354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008da354); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008da358); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008da359); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008da35a); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008da35f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 356; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008da373); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 363; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008da382); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da389); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 358; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da395); };
            c.w32(8, t6);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let r7 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008da3a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008da3a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008da3ab); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008da3b1); }
            c.w64(12, c.r64(7));
            c.w64(15, c.r64(6));
            blk = 5; continue;
        }
        4 => {
            c.w64(12, c.r64(0));
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008da3c0); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da3c8); };
            c.w32(0, t10);
            c.w64(3, c.r64(0));
            let r11 = c.r32(3) & 0xfu64;
            c.w32(3, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(2, c.r64(0));
            let r12 = c.r32(2) & 0x7u64;
            c.w32(2, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008da3ef); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            c.w64(0, c.r64(12));
            blk = 4; continue;
        }
        9 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008da401); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da409); };
            c.cmp(t17, 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008da40f); }
            c.w64(0, c.r64(12));
            return c.call(0x70008e88b4, 0x70008da41b, 0x70008da416);
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da41b); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da41f); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da423); };
            c.w64(7, t20);
            blk = 4; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008da429); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            c.w64(0, c.r64(12));
            blk = 4; continue;
        }
        14 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008da43f); };
            c.w32(8, t23);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r24 = c.add(c.r64(15), c.r64(8), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008da44d); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w64(0, c.r64(12));
            blk = 4; continue;
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da45d); };
            c.w32(8, t27);
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008da462); };
            c.cmp(c.r32(8), t28, 32);
            if c.cc_b() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da46b); };
            let r30 = c.sub(c.r64(8), t29, 64);
            c.w64(8, r30);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(15).wrapping_add(0xc));
            blk = 32; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008da47d);
        }
        20 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da47e); };
            c.w32(8, t31);
            let Some(t32) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008da483); };
            c.cmp(c.r32(8), t32, 32);
            if c.cc_b() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da48c); };
            let r34 = c.sub(c.r64(8), t33, 64);
            c.w64(8, r34);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(15).wrapping_mul(2).wrapping_add(0xc));
            c.w64(15, c.r64(8));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008da4a0); };
            c.w64(11, t35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008da4a7); };
            c.w64(2, t36);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008da4ae); };
            c.w64(8, t37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008da4b5); };
            c.w64(6, t38);
            c.w32(1, 0xbu64);
            blk = 198; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008da4c6);
        }
        24 => {
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_ne() { blk = 190; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t39) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008da4d4); };
            c.w64(8, t39);
            c.w64(0, c.r64(8));
            blk = 29; continue;
        }
        27 => {
            c.w64(10, c.r64(4));
            let r40 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r40);
            let r41 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r41);
            c.logic(r41, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008da4e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008da4ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008da4f2); }
            c.w64(1, c.r64(12));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008da4f9); };
            c.w64(0, t42);
            c.w64(10, 0x70008da507u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008da507); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008da50b); }
            return c.call(c.r64(0), 0x70008da511, 0x70008da50f);
        }
        28 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008da511); }
            let Some(t43) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008da519); };
            c.w64(4, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da51e); };
            c.w64(15, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da522); };
            c.w64(9, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da526); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008da52a); };
            c.w64(12, t47);
            blk = 29; continue;
        }
        29 => {
            c.w64(1, c.r64(12));
            let Some(t48) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da531); };
            c.w32(8, t48);
            let Some(t49) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008da535); };
            c.cmp(c.r32(8), t49, 32);
            if c.cc_b() { blk = 189; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da542); };
            let r51 = c.sub(c.r64(8), t50, 64);
            c.w64(8, r51);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 189; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(8, c.r64(15).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        32 => {
            let Some(t52) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da557); };
            c.w32(11, t52);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w32(2, 0x1u64);
            blk = 34; continue;
        }
        33 => {
            let r53 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r53);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da572); };
            c.cmp(c.r64(15), t54, 64);
            if c.cc_ae() { blk = 67; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(15)));
            let Some(t55) = c.ld8(c.r64(12).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008da580); };
            c.w32(0, t55);
            c.cmp(c.r8(0), 0x20u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(3, c.r64(0).wrapping_sub(0xe));
            c.cmp(c.r32(3), 0x92u64, 32);
            if c.cc_b() { blk = 63; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r8(0), 0x9u64, 8);
            if c.cc_b() { blk = 62; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r8(0), 0xdu64, 8);
            if c.cc_be() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r8(0), 0xffffffffffffffa0u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(0), 0x1680u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(0), 0x2000u64, 32);
            if c.cc_b() { blk = 58; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(0), 0x200au64, 32);
            if c.cc_be() { blk = 57; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(0), 0x2028u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(0), 0x2029u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(0), 0x202fu64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(0), 0x205fu64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(0), 0xfeffu64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(0), 0x3000u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        51 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        52 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        53 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        54 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        55 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        56 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        57 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        58 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        59 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        60 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        61 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        62 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        63 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        64 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            blk = 33; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008da64f);
        }
        68 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 70; continue;
        }
        69 => {
            c.w64(8, c.r64(2));
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008da662); };
            c.w32(8, t56);
            let r57 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r57);
            c.w64(12, c.r64(11));
            let r58 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r58);
            let Some(t59) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da66f); };
            c.w32(8, t59);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 184; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 183; continue; }
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(8) & 0x7u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 158; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(3, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(7));
            blk = 76; continue;
        }
        76 => {
            c.w64(1, c.r64(3));
            let r60 = c.r32(1) & 0x7u64;
            c.w32(1, r60);
            c.logic(r60, 32);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_l() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 186; continue;
        }
        81 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 186; continue;
        }
        84 => {
            let Some(t61) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008da6de); };
            c.w32(3, t61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008da6e1); };
            c.cmp(t62, c.r32(3), 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t63) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008da6ee); };
            c.w32(0, t63);
            let r64 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r64);
            let Some(t65) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008da6f4); };
            c.w32(3, t65);
            let r66 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r66);
            let Some(t67) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da6fa); };
            c.w32(3, t67);
            blk = 76; continue;
        }
        86 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008da700); };
            c.w32(3, t68);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r69 = c.add(c.r64(8), c.r64(3), 64);
            c.w64(8, r69);
            let Some(t70) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008da70c); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            let Some(t72) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008da712); };
            c.w32(3, t72);
            let r73 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r73);
            let Some(t74) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da718); };
            c.w32(3, t74);
            blk = 76; continue;
        }
        87 => {
            let Some(t75) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008da71e); };
            c.w32(0, t75);
            let r76 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r76);
            let Some(t77) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008da724); };
            c.w32(3, t77);
            let r78 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r78);
            let Some(t79) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008da72a); };
            c.w32(3, t79);
            blk = 76; continue;
        }
        88 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 91; continue;
        }
        89 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 91; continue;
        }
        90 => {
            c.w64(0, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(7));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r64(12), 0xdu64, 64);
            if c.cc_l() { blk = 99; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(8)));
            let r80 = c.r32(0) & 0x8u64;
            c.w32(0, r80);
            c.logic(r80, 32);
            let r81 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r81);
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da76e); };
            c.w64(6, t82);
            let Some(t83) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008da775); };
            c.w64(8, t83);
            c.w64(11, c.r64(8).wrapping_add(0x14));
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008da77c); };
            c.w64(1, t84);
            let Some(t85) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008da783); };
            c.cmp(t85, c.r64(11), 64);
            if c.cc_be() { blk = 365; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x70008da78c); }
            let r86 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r86);
            blk = 95; continue;
        }
        95 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008da793); };
            c.w64(11, t87);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008da79a); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008da79e); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008da7a6); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008da7aa); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008da7ae); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008da7b8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008da7b9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008da7bf); }
            return c.ret(0x0, 0x70008da7c1);
        }
        96 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da7c2); };
            c.w64(6, t88);
            let Some(t89) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008da7c9); };
            c.w64(8, t89);
            c.w64(11, c.r64(8).wrapping_add(0x14));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008da7d0); };
            c.w64(1, t90);
            let Some(t91) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008da7d7); };
            c.cmp(t91, c.r64(11), 64);
            if c.cc_be() { blk = 367; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x70008da7e0); }
            let r92 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r92);
            blk = 98; continue;
        }
        98 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008da7e7); };
            c.w64(11, t93);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008da7ee); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008da7f2); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008da7fa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008da7fe); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008da802); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008da80c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008da80d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008da813); }
            return c.ret(0x0, 0x70008da815);
        }
        99 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w32(7, c.r32(12));
            let r94 = c.r32(0) & 0x8u64;
            c.w32(0, r94);
            c.logic(r94, 32);
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w32(7, c.r32(12));
            c.w64(11, c.r64(7));
            let r95 = c.add(c.r32(11), c.r32(7), 32);
            c.w32(11, r95);
            if c.cc_o() { blk = 109; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r96 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r96);
            if c.cc_o() { blk = 108; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da856); };
            c.w64(1, t97);
            let Some(t98) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008da85d); };
            c.w64(11, t98);
            let r99 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r99);
            let r100 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r100);
            c.logic(r100, 64);
            c.w64(0, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008da86c); };
            c.w64(2, t101);
            let Some(t102) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008da873); };
            c.cmp(t102, c.r64(0), 64);
            if c.cc_be() { blk = 369; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 369; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008da889); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 107; continue;
        }
        107 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008da890); };
            c.w64(11, t103);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008da897); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008da89b); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008da8a2); }
            c.w64(0, c.r64(7));
            blk = 111; continue;
        }
        108 => {
            return RegfileExit::fault(0x70008da8ab);
        }
        109 => {
            return RegfileExit::fault(0x70008da8ac);
        }
        110 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008da8ad); };
            c.w64(0, t104);
            blk = 111; continue;
        }
        111 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(8)));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(11, 0xbu64);
            blk = 114; continue;
        }
        113 => {
            let r105 = c.add(c.r64(11), 0x2u64, 64);
            c.w64(11, r105);
            c.w64(7, c.r64(12));
            blk = 114; continue;
        }
        114 => {
            c.w64(12, c.r64(7).wrapping_add(0x2));
            let Some(t106) = c.ld16(c.r64(3).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008da8dc); };
            c.w32(7, t106);
            if !c.st16(c.r64(0).wrapping_add(c.r64(11)), c.r16(7)) { return RegfileExit::fault(0x70008da8e0); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 113; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008da8ed); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008da8ee); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008da8f4); }
            return c.ret(0x0, 0x70008da8f6);
        }
        116 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r107 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r107);
            if c.cc_o() { blk = 122; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da909); };
            c.w64(1, t108);
            let Some(t109) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008da910); };
            c.w64(11, t109);
            let r110 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r110);
            let r111 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r111);
            c.logic(r111, 64);
            c.w64(0, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008da91f); };
            c.w64(2, t112);
            let Some(t113) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008da926); };
            c.cmp(t113, c.r64(0), 64);
            if c.cc_be() { blk = 371; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 371; continue; }
            blk = 120; continue;
        }
        120 => {
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008da93c); }
            c.w64(7, c.r64(11).wrapping_add(0x1));
            blk = 121; continue;
        }
        121 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008da943); };
            c.w64(11, t114);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008da94a); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008da94e); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008da955); }
            c.w64(0, c.r64(7));
            blk = 124; continue;
        }
        122 => {
            return RegfileExit::fault(0x70008da95e);
        }
        123 => {
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008da95f); };
            c.w64(0, t115);
            blk = 124; continue;
        }
        124 => {
            c.w64(7, c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(12)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 128; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(11, 0xbu64);
            blk = 127; continue;
        }
        126 => {
            let r116 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r116);
            c.w64(7, c.r64(12));
            blk = 127; continue;
        }
        127 => {
            c.w64(12, c.r64(7).wrapping_add(0x1));
            let Some(t117) = c.ld8(c.r64(3).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008da987); };
            c.w32(7, t117);
            if !c.st8(c.r64(0).wrapping_add(c.r64(11)), c.r8(7)) { return RegfileExit::fault(0x70008da98b); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 126; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008da997); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008da998); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008da99e); }
            return c.ret(0x0, 0x70008da9a0);
        }
        129 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008da9a1); };
            c.w64(2, t118);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008da9a8); };
            c.w64(7, t119);
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008da9af); };
            c.w64(6, t120);
            c.w32(1, 0xbu64);
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 157; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w32(11, c.r32(12));
            let Some(t121) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008da9c6); };
            c.w64(3, t121);
            let r122 = c.r32(0) & 0x8u64;
            c.w32(0, r122);
            c.logic(r122, 32);
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 140; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(7, c.r32(12));
            c.w64(11, c.r64(7));
            let r123 = c.add(c.r32(11), c.r32(7), 32);
            c.w32(11, r123);
            if c.cc_o() { blk = 139; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(7, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w32(0, 0xcu64);
            let r124 = c.add(c.r64(7), c.r64(0), 64);
            c.w64(7, r124);
            if c.cc_o() { blk = 138; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008daa00); };
            c.w64(11, t125);
            let Some(t126) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008daa07); };
            c.w64(0, t126);
            let r127 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r127);
            let r128 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r128);
            c.logic(r128, 64);
            c.w64(9, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008daa16); };
            c.w64(2, t129);
            let Some(t130) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008daa1d); };
            c.cmp(t130, c.r64(9), 64);
            if c.cc_be() { blk = 373; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 373; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.st64(c.r64(11), c.r64(9)) { return RegfileExit::fault(0x70008daa33); }
            c.w64(7, c.r64(0).wrapping_add(0x1));
            blk = 137; continue;
        }
        137 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008daa3a); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008daa3d); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008daa44); }
            c.w64(2, c.r64(7));
            blk = 140; continue;
        }
        138 => {
            return RegfileExit::fault(0x70008daa4d);
        }
        139 => {
            return RegfileExit::fault(0x70008daa4e);
        }
        140 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(8)));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 143; continue;
        }
        142 => {
            let r131 = c.add(c.r64(1), 0x2u64, 64);
            c.w64(1, r131);
            c.w64(7, c.r64(9));
            blk = 143; continue;
        }
        143 => {
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t132) = c.ld16(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008daa71); };
            c.w32(7, t132);
            if !c.st16(c.r64(2).wrapping_add(c.r64(1)), c.r16(7)) { return RegfileExit::fault(0x70008daa76); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 142; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(0, c.r64(2));
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008daa82); };
            c.w64(9, t133);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008daa89); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008daa8a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008daa90); }
            return c.ret(0x0, 0x70008daa92);
        }
        145 => {
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008daa93); };
            c.w64(0, t134);
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008daa9a); };
            c.w64(6, t135);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(2, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(11, 0xcu64);
            let r136 = c.add(c.r64(2), c.r64(11), 64);
            c.w64(2, r136);
            if c.cc_o() { blk = 151; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t137) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008daab5); };
            c.w64(11, t137);
            let r138 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r138);
            let r139 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r139);
            c.logic(r139, 64);
            c.w64(9, c.r64(11).wrapping_add(c.r64(2)));
            let Some(t140) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008daac4); };
            c.cmp(t140, c.r64(9), 64);
            if c.cc_be() { blk = 375; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 375; continue; }
            blk = 149; continue;
        }
        149 => {
            if !c.st64(c.r64(0), c.r64(9)) { return RegfileExit::fault(0x70008daada); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 150; continue;
        }
        150 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008daae1); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008daae5); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008daaed); }
            c.w64(2, c.r64(9));
            blk = 152; continue;
        }
        151 => {
            return RegfileExit::fault(0x70008daaf6);
        }
        152 => {
            c.w64(7, c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(12)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 156; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 155; continue;
        }
        154 => {
            let r141 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r141);
            c.w64(7, c.r64(9));
            blk = 155; continue;
        }
        155 => {
            c.w64(9, c.r64(7).wrapping_add(0x1));
            let Some(t142) = c.ld8(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008dab12); };
            c.w32(7, t142);
            if !c.st8(c.r64(2).wrapping_add(c.r64(1)), c.r8(7)) { return RegfileExit::fault(0x70008dab17); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 154; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w64(0, c.r64(2));
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dab23); };
            c.w64(9, t143);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dab2a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dab2b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dab31); }
            return c.ret(0x0, 0x70008dab33);
        }
        157 => {
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dab34); };
            c.w64(7, t144);
            blk = 186; continue;
        }
        158 => {
            c.w32(8, c.r16(8));
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dab41); };
            c.w64(11, t145);
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dab48); };
            c.w64(12, t146);
            c.w32(6, 0xcu64);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 174; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w64(3, c.r64(8));
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(7));
            blk = 160; continue;
        }
        160 => {
            c.w64(1, c.r64(3));
            let r147 = c.r32(1) & 0x7u64;
            c.w32(1, r147);
            c.logic(r147, 32);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_l() { blk = 165; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 164; continue;
        }
        164 => {
            blk = 377; continue;
        }
        165 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            blk = 377; continue;
        }
        168 => {
            let Some(t148) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dab98); };
            c.w32(3, t148);
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dab9b); };
            c.cmp(t149, c.r32(3), 32);
            if c.cc_ne() { blk = 378; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t150) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008daba8); };
            c.w32(0, t150);
            let r151 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r151);
            let Some(t152) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dabae); };
            c.w32(3, t152);
            let r153 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r153);
            let Some(t154) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dabb4); };
            c.w32(3, t154);
            blk = 160; continue;
        }
        170 => {
            let Some(t155) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dabba); };
            c.w32(3, t155);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r156 = c.add(c.r64(8), c.r64(3), 64);
            c.w64(8, r156);
            let Some(t157) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dabc6); };
            c.w32(0, t157);
            let r158 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r158);
            let Some(t159) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dabcc); };
            c.w32(3, t159);
            let r160 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r160);
            let Some(t161) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dabd2); };
            c.w32(3, t161);
            blk = 160; continue;
        }
        171 => {
            let Some(t162) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dabd8); };
            c.w32(0, t162);
            let r163 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r163);
            let Some(t164) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dabde); };
            c.w32(3, t164);
            let r165 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r165);
            let Some(t166) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dabe4); };
            c.w32(3, t166);
            blk = 160; continue;
        }
        172 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(8));
            c.w64(8, c.r64(10));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 175; continue;
        }
        173 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(8));
            c.w64(8, c.r64(10));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 175; continue;
        }
        174 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(7));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 379; continue; }
            blk = 176; continue;
        }
        176 => {
            c.w64(7, c.r64(3).wrapping_add(0xb));
            blk = 177; continue;
        }
        177 => {
            let r167 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r167);
            let r168 = c.r32(8) & 0x8u64;
            c.w32(8, r168);
            c.logic(r168, 32);
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t169) = c.ld16(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008dac38); };
            c.w32(7, t169);
            blk = 180; continue;
        }
        179 => {
            let Some(t170) = c.ld8(c.r64(15).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008dac3f); };
            c.w32(7, t170);
            blk = 180; continue;
        }
        180 => {
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 394; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(8, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008dac58); };
            c.w64(15, t171);
            let Some(t172) = c.ld32(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dac5f); };
            c.w32(0, t172);
            let r173 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r173);
            let Some(t174) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dac67); };
            c.cmp(t174, c.r32(0), 32);
            if c.cc_e() { blk = 399; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dac77); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dac78); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dac7e); }
            return c.ret(0x0, 0x70008dac80);
        }
        183 => {
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dac81); };
            c.w64(0, t175);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dac8b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dac8c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dac92); }
            return c.ret(0x0, 0x70008dac94);
        }
        184 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008daca1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008daca2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008daca8); }
            return c.ret(0x0, 0x70008dacaa);
        }
        186 => {
            c.w32(8, c.r64(15).wrapping_add(c.r64(15)));
            let r176 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r176);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dacb2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008dacb3); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008dacb5); }
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008dacb7); };
            c.w64(3, t177);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008dacca, 0x70008dacc5);
        }
        187 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dacca); };
            c.w64(7, t178);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dacd1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dacd2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dacd8); }
            return c.ret(0x0, 0x70008dacda);
        }
        188 => {
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dacdb); };
            c.w64(0, t179);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dace5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dace6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dacec); }
            return c.ret(0x0, 0x70008dacee);
        }
        189 => {
            return RegfileExit::fault(0x70008dacef);
        }
        190 => {
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dacf0); };
            c.w64(2, t180);
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dacf7); };
            c.w64(8, t181);
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008dacfe); };
            c.w64(6, t182);
            c.w32(1, 0xbu64);
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008dad0a); };
            c.w64(11, t183);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_ne() { blk = 355; continue; }
            blk = 191; continue;
        }
        191 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t184) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dad1e); };
            c.w64(0, t184);
            blk = 195; continue;
        }
        193 => {
            c.w64(10, c.r64(4));
            let r185 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r185);
            let r186 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r186);
            c.logic(r186, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008dad30); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008dad35); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dad39); }
            c.w64(1, c.r64(12));
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008dad40); };
            c.w64(0, t187);
            c.w64(10, 0x70008dad4eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008dad4e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008dad52); }
            return c.call(c.r64(0), 0x70008dad58, 0x70008dad56);
        }
        194 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008dad58); }
            let Some(t188) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008dad60); };
            c.w64(4, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dad65); };
            c.w64(15, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dad69); };
            c.w64(9, t190);
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dad6d); };
            c.w64(2, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dad74); };
            c.w64(7, t192);
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dad78); };
            c.w64(8, t193);
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008dad7f); };
            c.w64(6, t194);
            c.w32(1, 0xbu64);
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008dad8b); };
            c.w64(11, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dad92); };
            c.w64(12, t196);
            blk = 195; continue;
        }
        195 => {
            let Some(t197) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dad96); };
            c.w32(12, t197);
            let Some(t198) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dad9b); };
            c.cmp(c.r32(12), t198, 32);
            if c.cc_b() { blk = 354; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dada8); };
            let r200 = c.sub(c.r64(12), t199, 64);
            c.w64(12, r200);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 354; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w64(12, c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x1));
            c.w64(15, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 198; continue;
        }
        198 => {
            c.w64(0, c.r64(7));
            let Some(t201) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dadc3); };
            c.w32(7, t201);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008dadc6); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 200; continue;
        }
        199 => {
            let r202 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r202);
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r64(3), c.r64(7), 64);
            if c.cc_e() { blk = 235; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008daddb); };
            c.cmp(c.r64(3), t203, 64);
            if c.cc_ae() { blk = 233; continue; }
            blk = 202; continue;
        }
        202 => {
            c.w64(6, c.r64(15).wrapping_add(c.r64(3).wrapping_mul(2)));
            let Some(t204) = c.ld16(c.r64(12).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dade9); };
            c.w32(6, t204);
            c.cmp(c.r16(6), 0x20u64, 16);
            if c.cc_e() { blk = 230; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w32(1, c.r64(6).wrapping_sub(0xe));
            c.cmp(c.r32(1), 0x92u64, 32);
            if c.cc_b() { blk = 229; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r16(6), 0x9u64, 16);
            if c.cc_b() { blk = 228; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r16(6), 0xdu64, 16);
            if c.cc_be() { blk = 227; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r16(6), 0xa0u64, 16);
            if c.cc_e() { blk = 226; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r16(6), 0x1680u64, 16);
            if c.cc_e() { blk = 225; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r16(6), 0x2000u64, 16);
            if c.cc_b() { blk = 224; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r16(6), 0x200au64, 16);
            if c.cc_be() { blk = 223; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r16(6), 0x2028u64, 16);
            if c.cc_e() { blk = 222; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r16(6), 0x2029u64, 16);
            if c.cc_e() { blk = 221; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r16(6), 0x202fu64, 16);
            if c.cc_e() { blk = 220; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r16(6), 0x205fu64, 16);
            if c.cc_e() { blk = 219; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r16(6), 0xfeffu64, 16);
            if c.cc_e() { blk = 218; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r16(6), 0x3000u64, 16);
            if c.cc_e() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 231; continue;
        }
        217 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        218 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        219 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        220 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        221 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        222 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        223 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        224 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 231; continue;
        }
        225 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        226 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        227 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        228 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 231; continue;
        }
        229 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 231; continue;
        }
        230 => {
            c.w32(1, 0x1u64);
            blk = 231; continue;
        }
        231 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 232; continue;
        }
        232 => {
            blk = 199; continue;
        }
        233 => {
            return RegfileExit::fault(0x70008daed7);
        }
        234 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 236; continue;
        }
        235 => {
            c.w32(12, 0x1u64);
            blk = 236; continue;
        }
        236 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 353; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t205) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008daeed); };
            c.w32(12, t205);
            let r206 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r206);
            c.w64(15, c.r64(7));
            let r207 = c.sub(c.r64(15), c.r64(3), 64);
            c.w64(15, r207);
            let Some(t208) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008daefa); };
            c.w32(12, t208);
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_be() { blk = 349; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 348; continue; }
            blk = 239; continue;
        }
        239 => {
            c.logic(c.r8(12) & 0x7u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_e() { blk = 323; continue; }
            blk = 240; continue;
        }
        240 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 257; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w64(6, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            blk = 243; continue;
        }
        242 => {
            c.w64(6, c.r64(0));
            blk = 243; continue;
        }
        243 => {
            c.w64(0, c.r64(6));
            let r209 = c.r32(0) & 0x7u64;
            c.w32(0, r209);
            c.logic(r209, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 248; continue; }
            blk = 244; continue;
        }
        244 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 256; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 246; continue;
        }
        246 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 247; continue;
        }
        247 => {
            blk = 351; continue;
        }
        248 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 255; continue; }
            blk = 249; continue;
        }
        249 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            blk = 351; continue;
        }
        251 => {
            let Some(t210) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008daf6f); };
            c.w32(0, t210);
            let Some(t211) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008daf72); };
            c.cmp(t211, c.r32(0), 32);
            if c.cc_ne() { blk = 351; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t212) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008daf7f); };
            c.w32(1, t212);
            let r213 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r213);
            let Some(t214) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008daf85); };
            c.w32(0, t214);
            let r215 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r215);
            let Some(t216) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008daf8b); };
            c.w32(0, t216);
            blk = 242; continue;
        }
        253 => {
            let Some(t217) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008daf91); };
            c.w32(0, t217);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r218 = c.add(c.r64(12), c.r64(0), 64);
            c.w64(12, r218);
            let Some(t219) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008daf9d); };
            c.w32(1, t219);
            let r220 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r220);
            let Some(t221) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dafa3); };
            c.w32(0, t221);
            let r222 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r222);
            let Some(t223) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dafa9); };
            c.w32(0, t223);
            blk = 242; continue;
        }
        254 => {
            let Some(t224) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dafaf); };
            c.w32(1, t224);
            let r225 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r225);
            let Some(t226) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dafb5); };
            c.w32(0, t226);
            let r227 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r227);
            let Some(t228) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dafbb); };
            c.w32(0, t228);
            blk = 242; continue;
        }
        255 => {
            c.w64(0, c.r64(6));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 258; continue;
        }
        256 => {
            c.w64(0, c.r64(6));
            c.w32(6, 0x1u64);
            blk = 258; continue;
        }
        257 => {
            c.w64(1, c.r64(0));
            c.w64(0, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 258; continue;
        }
        258 => {
            c.cmp(c.r64(15), 0xdu64, 64);
            if c.cc_l() { blk = 266; continue; }
            blk = 259; continue;
        }
        259 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(12)));
            let r229 = c.r32(0) & 0x8u64;
            c.w32(0, r229);
            c.logic(r229, 32);
            let r230 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r230);
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008daff8); };
            c.w64(12, t231);
            let Some(t232) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008dafff); };
            c.w64(11, t232);
            c.w64(0, c.r64(11).wrapping_add(0x14));
            let Some(t233) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008db007); };
            c.cmp(t233, c.r64(0), 64);
            if c.cc_be() { blk = 408; continue; }
            blk = 261; continue;
        }
        261 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008db010); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 262; continue;
        }
        262 => {
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008db018); };
            c.w64(11, t234);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008db01f); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db023); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008db02b); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008db02f); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008db033); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db03d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db03e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db044); }
            return c.ret(0x0, 0x70008db046);
        }
        263 => {
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008db047); };
            c.w64(12, t235);
            let Some(t236) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008db04e); };
            c.w64(0, t236);
            c.w64(3, c.r64(0).wrapping_add(0x14));
            let Some(t237) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008db056); };
            c.cmp(t237, c.r64(3), 64);
            if c.cc_be() { blk = 410; continue; }
            blk = 264; continue;
        }
        264 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008db05f); }
            c.w64(8, c.r64(0).wrapping_add(0x1));
            blk = 265; continue;
        }
        265 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008db067); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db06b); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008db073); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008db077); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008db07b); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db085); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db086); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db08c); }
            return c.ret(0x0, 0x70008db08e);
        }
        266 => {
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008db08f); };
            c.w64(11, t238);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 294; continue; }
            blk = 267; continue;
        }
        267 => {
            c.w32(7, c.r32(15));
            let r239 = c.r32(0) & 0x8u64;
            c.w32(0, r239);
            c.logic(r239, 32);
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 282; continue; }
            blk = 268; continue;
        }
        268 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 277; continue; }
            blk = 269; continue;
        }
        269 => {
            c.w32(7, c.r32(15));
            c.w64(0, c.r64(7));
            let r240 = c.add(c.r32(0), c.r32(7), 32);
            c.w32(0, r240);
            if c.cc_o() { blk = 276; continue; }
            blk = 270; continue;
        }
        270 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r241 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r241);
            if c.cc_o() { blk = 275; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t242) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008db0d5); };
            c.w64(0, t242);
            let r243 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r243);
            let r244 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r244);
            c.logic(r244, 64);
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t245) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008db0e4); };
            c.cmp(t245, c.r64(2), 64);
            if c.cc_be() { blk = 412; continue; }
            blk = 272; continue;
        }
        272 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 412; continue; }
            blk = 273; continue;
        }
        273 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008db0fa); }
            c.w64(7, c.r64(0).wrapping_add(0x1));
            blk = 274; continue;
        }
        274 => {
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008db101); };
            c.w64(8, t246);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008db108); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db10c); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008db113); }
            c.w64(2, c.r64(7));
            blk = 277; continue;
        }
        275 => {
            return RegfileExit::fault(0x70008db11c);
        }
        276 => {
            return RegfileExit::fault(0x70008db11d);
        }
        277 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(12)));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 281; continue; }
            blk = 278; continue;
        }
        278 => {
            c.w64(11, c.r64(7));
            c.w32(7, 0xbu64);
            blk = 280; continue;
        }
        279 => {
            let r247 = c.add(c.r64(7), 0x2u64, 64);
            c.w64(7, r247);
            c.w64(11, c.r64(12));
            blk = 280; continue;
        }
        280 => {
            c.w64(12, c.r64(11).wrapping_add(0x2));
            let Some(t248) = c.ld16(c.r64(1).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008db148); };
            c.w32(11, t248);
            if !c.st16(c.r64(2).wrapping_add(c.r64(7)), c.r16(11)) { return RegfileExit::fault(0x70008db14d); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 279; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db15d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db15e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db164); }
            return c.ret(0x0, 0x70008db166);
        }
        282 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 289; continue; }
            blk = 283; continue;
        }
        283 => {
            c.w64(7, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r249 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r249);
            if c.cc_o() { blk = 288; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t250) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008db179); };
            c.w64(0, t250);
            let r251 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r251);
            let r252 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r252);
            c.logic(r252, 64);
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t253) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008db188); };
            c.cmp(t253, c.r64(2), 64);
            if c.cc_be() { blk = 414; continue; }
            blk = 285; continue;
        }
        285 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 414; continue; }
            blk = 286; continue;
        }
        286 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008db19e); }
            c.w64(7, c.r64(0).wrapping_add(0x1));
            blk = 287; continue;
        }
        287 => {
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008db1a5); };
            c.w64(8, t254);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008db1ac); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db1b0); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008db1b7); }
            c.w64(2, c.r64(7));
            blk = 289; continue;
        }
        288 => {
            return RegfileExit::fault(0x70008db1c0);
        }
        289 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(15)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 293; continue; }
            blk = 290; continue;
        }
        290 => {
            c.w64(11, c.r64(7));
            c.w32(7, 0xbu64);
            blk = 292; continue;
        }
        291 => {
            let r255 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r255);
            c.w64(11, c.r64(12));
            blk = 292; continue;
        }
        292 => {
            c.w64(12, c.r64(11).wrapping_add(0x1));
            let Some(t256) = c.ld8(c.r64(1).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008db1e4); };
            c.w32(11, t256);
            if !c.st8(c.r64(2).wrapping_add(c.r64(7)), c.r8(11)) { return RegfileExit::fault(0x70008db1e9); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 291; continue; }
            blk = 293; continue;
        }
        293 => {
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db1f8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db1f9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db1ff); }
            return c.ret(0x0, 0x70008db201);
        }
        294 => {
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008db202); };
            c.w64(7, t257);
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008db209); };
            c.w64(6, t258);
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 322; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w32(7, c.r32(15));
            let Some(t259) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008db21b); };
            c.w64(1, t259);
            let r260 = c.r32(0) & 0x8u64;
            c.w32(0, r260);
            c.logic(r260, 32);
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(0), 0x8u64, 32);
            if c.cc_e() { blk = 310; continue; }
            blk = 296; continue;
        }
        296 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 305; continue; }
            blk = 297; continue;
        }
        297 => {
            c.w32(7, c.r32(15));
            c.w64(0, c.r64(7));
            let r261 = c.add(c.r32(0), c.r32(7), 32);
            c.w32(0, r261);
            if c.cc_o() { blk = 304; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r262 = c.add(c.r64(7), c.r64(2), 64);
            c.w64(7, r262);
            if c.cc_o() { blk = 303; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t263) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008db252); };
            c.w64(0, t263);
            let r264 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r264);
            let r265 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r265);
            c.logic(r265, 64);
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t266) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008db261); };
            c.cmp(t266, c.r64(2), 64);
            if c.cc_be() { blk = 416; continue; }
            blk = 300; continue;
        }
        300 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 416; continue; }
            blk = 301; continue;
        }
        301 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008db277); }
            c.w64(7, c.r64(0).wrapping_add(0x1));
            blk = 302; continue;
        }
        302 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008db27e); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db281); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008db288); }
            c.w64(2, c.r64(7));
            blk = 305; continue;
        }
        303 => {
            return RegfileExit::fault(0x70008db291);
        }
        304 => {
            return RegfileExit::fault(0x70008db292);
        }
        305 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(12)));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 309; continue; }
            blk = 306; continue;
        }
        306 => {
            c.w64(11, c.r64(7));
            c.w32(7, 0xbu64);
            blk = 308; continue;
        }
        307 => {
            let r267 = c.add(c.r64(7), 0x2u64, 64);
            c.w64(7, r267);
            c.w64(11, c.r64(12));
            blk = 308; continue;
        }
        308 => {
            c.w64(12, c.r64(11).wrapping_add(0x2));
            let Some(t268) = c.ld16(c.r64(11).wrapping_add(c.r64(1)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008db2bd); };
            c.w32(11, t268);
            if !c.st16(c.r64(2).wrapping_add(c.r64(7)), c.r16(11)) { return RegfileExit::fault(0x70008db2c3); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 307; continue; }
            blk = 309; continue;
        }
        309 => {
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db2d3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db2d4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db2da); }
            return c.ret(0x0, 0x70008db2dc);
        }
        310 => {
            c.w32(0, 0xcu64);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 317; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(7, (c.r32(15) as u32 as i32 as i64 as u64));
            let r269 = c.add(c.r64(7), c.r64(0), 64);
            c.w64(7, r269);
            if c.cc_o() { blk = 316; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t270) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008db2ef); };
            c.w64(0, t270);
            let r271 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r271);
            let r272 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r272);
            c.logic(r272, 64);
            c.w64(2, c.r64(0).wrapping_add(c.r64(7)));
            let Some(t273) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008db2fe); };
            c.cmp(t273, c.r64(2), 64);
            if c.cc_be() { blk = 418; continue; }
            blk = 313; continue;
        }
        313 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 418; continue; }
            blk = 314; continue;
        }
        314 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008db314); }
            c.w64(7, c.r64(0).wrapping_add(0x1));
            blk = 315; continue;
        }
        315 => {
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008db31b); };
            c.w64(2, t274);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008db322); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db325); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008db32c); }
            c.w64(2, c.r64(7));
            blk = 317; continue;
        }
        316 => {
            return RegfileExit::fault(0x70008db335);
        }
        317 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(15)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 321; continue; }
            blk = 318; continue;
        }
        318 => {
            c.w64(11, c.r64(7));
            c.w32(7, 0xbu64);
            blk = 320; continue;
        }
        319 => {
            let r275 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r275);
            c.w64(11, c.r64(12));
            blk = 320; continue;
        }
        320 => {
            c.w64(12, c.r64(11).wrapping_add(0x1));
            let Some(t276) = c.ld8(c.r64(11).wrapping_add(c.r64(1)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008db359); };
            c.w32(11, t276);
            if !c.st8(c.r64(2).wrapping_add(c.r64(7)), c.r8(11)) { return RegfileExit::fault(0x70008db35f); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 319; continue; }
            blk = 321; continue;
        }
        321 => {
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db36e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db36f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db375); }
            return c.ret(0x0, 0x70008db377);
        }
        322 => {
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008db378); };
            c.w64(7, t277);
            blk = 351; continue;
        }
        323 => {
            c.w32(7, c.r16(12));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 339; continue; }
            blk = 324; continue;
        }
        324 => {
            c.w64(12, c.r64(7));
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(0));
            blk = 325; continue;
        }
        325 => {
            c.w64(15, c.r64(12));
            let r278 = c.r32(15) & 0x7u64;
            c.w32(15, r278);
            c.logic(r278, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 330; continue; }
            blk = 326; continue;
        }
        326 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 338; continue; }
            blk = 327; continue;
        }
        327 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 335; continue; }
            blk = 328; continue;
        }
        328 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 336; continue; }
            blk = 329; continue;
        }
        329 => {
            blk = 420; continue;
        }
        330 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 337; continue; }
            blk = 331; continue;
        }
        331 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            blk = 420; continue;
        }
        333 => {
            let Some(t279) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008db3cf); };
            c.w32(12, t279);
            let Some(t280) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008db3d3); };
            c.cmp(t280, c.r32(12), 32);
            if c.cc_ne() { blk = 421; continue; }
            blk = 334; continue;
        }
        334 => {
            let Some(t281) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008db3e0); };
            c.w32(11, t281);
            let r282 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r282);
            let Some(t283) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008db3e7); };
            c.w32(12, t283);
            let r284 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r284);
            let Some(t285) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008db3ee); };
            c.w32(12, t285);
            blk = 325; continue;
        }
        335 => {
            let Some(t286) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008db3f6); };
            c.w32(12, t286);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r287 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r287);
            let Some(t288) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008db403); };
            c.w32(11, t288);
            let r289 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r289);
            let Some(t290) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008db40a); };
            c.w32(12, t290);
            let r291 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r291);
            let Some(t292) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008db411); };
            c.w32(12, t292);
            blk = 325; continue;
        }
        336 => {
            let Some(t293) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008db41c); };
            c.w32(11, t293);
            let r294 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r294);
            let Some(t295) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008db423); };
            c.w32(12, t295);
            let r296 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r296);
            let Some(t297) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008db42a); };
            c.w32(12, t297);
            blk = 325; continue;
        }
        337 => {
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(12));
            c.w64(12, c.r64(10));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 340; continue;
        }
        338 => {
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(7));
            c.w64(7, c.r64(10));
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(12));
            c.w64(12, c.r64(10));
            c.w32(15, 0x1u64);
            blk = 340; continue;
        }
        339 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 340; continue;
        }
        340 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 422; continue; }
            blk = 341; continue;
        }
        341 => {
            let r298 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r298);
            blk = 342; continue;
        }
        342 => {
            let r299 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r299);
            let r300 = c.r32(7) & 0x8u64;
            c.w32(7, r300);
            c.logic(r300, 32);
            c.cmp(c.r32(7), 0x8u64, 32);
            if c.cc_e() { blk = 344; continue; }
            blk = 343; continue;
        }
        343 => {
            let Some(t301) = c.ld16(c.r64(12).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x70008db488); };
            c.w32(7, t301);
            blk = 345; continue;
        }
        344 => {
            let Some(t302) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008db48f); };
            c.w32(7, t302);
            blk = 345; continue;
        }
        345 => {
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 437; continue; }
            blk = 346; continue;
        }
        346 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008db4a8); };
            c.w64(12, t303);
            let Some(t304) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008db4af); };
            c.w32(0, t304);
            let r305 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r305);
            let Some(t306) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008db4b7); };
            c.cmp(t306, c.r32(0), 32);
            if c.cc_e() { blk = 442; continue; }
            blk = 347; continue;
        }
        347 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db4c7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db4c8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db4ce); }
            return c.ret(0x0, 0x70008db4d0);
        }
        348 => {
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db4d7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db4d8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db4de); }
            return c.ret(0x0, 0x70008db4e0);
        }
        349 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_a() { blk = 351; continue; }
            blk = 350; continue;
        }
        350 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db4ea); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db4eb); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db4f1); }
            return c.ret(0x0, 0x70008db4f3);
        }
        351 => {
            c.w32(8, c.r64(3).wrapping_add(c.r64(3)));
            let r307 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db4fa); };
            if !c.push64(t308) { return RegfileExit::fault(0x70008db4fa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008db4fd); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008db4ff); }
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008db500); };
            c.w64(3, t309);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008db513, 0x70008db50e);
        }
        352 => {
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db513); };
            c.w64(7, t310);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db51a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db51b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db521); }
            return c.ret(0x0, 0x70008db523);
        }
        353 => {
            c.w64(0, c.r64(2));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db52a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db52b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db531); }
            return c.ret(0x0, 0x70008db533);
        }
        354 => {
            return RegfileExit::fault(0x70008db534);
        }
        355 => {
            return RegfileExit::fault(0x70008db535);
        }
        356 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008db536); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008db53a); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008db540); };
            c.w64(3, t311);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008db54f, 0x70008db54a);
        }
        357 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db54f); };
            c.w64(0, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db553); };
            c.w64(6, t313);
            blk = 1; continue;
        }
        358 => {
            let Some(t314) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008db55c); };
            c.cmp(t314, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t315) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008db56b); };
            c.cmp(t315, c.r32(7), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008db57a); }
            let r316 = c.r32(8) | c.r32(9);
            c.w32(8, r316);
            c.logic(r316, 32);
            if c.cc_ne() { blk = 361; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008db58b, 0x70008db586);
        }
        360 => {
            c.w64(7, c.r64(0));
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db58e); };
            c.w64(0, t317);
            blk = 3; continue;
        }
        361 => {
            c.w32(7, 0x36u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008db59c); }
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008db59d); };
            c.w64(7, t318);
            let Some(t319) = c.ld32(c.r64(7).wrapping_add(0x397)) else { return RegfileExit::fault(0x70008db5a4); };
            c.w32(7, t319);
            let r320 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r320);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008db5ad); }
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008db5ae); };
            c.w64(3, t321);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008db5bf, 0x70008db5ba);
        }
        362 => {
            return RegfileExit::fault(0x70008db5bf);
        }
        363 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008db5c0); }
            c.w64(0, c.r64(7));
            return c.call(0x70008a1214, 0x70008db5cc, 0x70008db5c7);
        }
        364 => {
            c.w64(7, c.r64(0));
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db5cf); };
            c.w64(0, t322);
            blk = 3; continue;
        }
        365 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008db5dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008db5e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008db5e5); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008db5f1, 0x70008db5ec);
        }
        366 => {
            c.w64(8, c.r64(0));
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db5f4); };
            c.w64(9, t323);
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db5f8); };
            c.w64(7, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db5fc); };
            c.w64(12, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db600); };
            c.w64(3, t326);
            blk = 95; continue;
        }
        367 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008db609); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008db60d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008db611); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008db61f, 0x70008db61a);
        }
        368 => {
            c.w64(8, c.r64(0));
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db622); };
            c.w64(9, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db626); };
            c.w64(7, t328);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db62a); };
            c.w64(12, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db62e); };
            c.w64(3, t330);
            blk = 98; continue;
        }
        369 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008db637); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008db63b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008db63f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008db643); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008db64f, 0x70008db64a);
        }
        370 => {
            c.w64(7, c.r64(0));
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db652); };
            c.w64(9, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db656); };
            c.w64(3, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db65a); };
            c.w64(15, t333);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db65e); };
            c.w64(12, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008db662); };
            c.w64(8, t335);
            blk = 107; continue;
        }
        371 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008db66b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008db66f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008db673); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008db677); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008db683, 0x70008db67e);
        }
        372 => {
            c.w64(7, c.r64(0));
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db686); };
            c.w64(9, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db68a); };
            c.w64(3, t337);
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db68e); };
            c.w64(15, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db692); };
            c.w64(12, t339);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008db696); };
            c.w64(8, t340);
            blk = 121; continue;
        }
        373 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008db69f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008db6a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008db6a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008db6ab); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008db6b7, 0x70008db6b2);
        }
        374 => {
            c.w64(7, c.r64(0));
            let Some(t341) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008db6ba); };
            c.w64(6, t341);
            c.w32(1, 0xbu64);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db6c6); };
            c.w64(15, t342);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db6ca); };
            c.w64(3, t343);
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db6ce); };
            c.w64(12, t344);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008db6d2); };
            c.w64(8, t345);
            blk = 137; continue;
        }
        375 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008db6db); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008db6df); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008db6e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008db6e7); }
            return c.call(0x70007febb4, 0x70008db6f0, 0x70008db6eb);
        }
        376 => {
            c.w64(9, c.r64(0));
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008db6f3); };
            c.w64(7, t346);
            c.w32(1, 0xbu64);
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db6ff); };
            c.w64(15, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db703); };
            c.w64(3, t348);
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db707); };
            c.w64(12, t349);
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008db70b); };
            c.w64(8, t350);
            blk = 150; continue;
        }
        377 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008db714); }
            blk = 381; continue;
        }
        378 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008db71a); }
            blk = 381; continue;
        }
        379 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008db720); }
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 381; continue; }
            blk = 380; continue;
        }
        380 => {
            let Some(t351) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008db72a); };
            c.w64(7, t351);
            blk = 177; continue;
        }
        381 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 383; continue; }
            blk = 382; continue;
        }
        382 => {
            c.w32(8, c.r64(15).wrapping_add(c.r64(15)));
            blk = 392; continue;
        }
        383 => {
            let Some(t352) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008db742); };
            c.w64(8, t352);
            c.w64(0, c.r64(8).wrapping_add(0xc));
            let Some(t353) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008db749); };
            c.cmp(t353, c.r64(0), 64);
            if c.cc_a() { blk = 386; continue; }
            blk = 384; continue;
        }
        384 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x70008db757, 0x70008db752);
        }
        385 => {
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db757); };
            c.w64(15, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db75b); };
            c.w64(9, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db75f); };
            c.w64(7, t356);
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008db763); };
            c.w64(11, t357);
            let Some(t358) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008db76a); };
            c.w64(12, t358);
            blk = 387; continue;
        }
        386 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008db773); }
            let r359 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r359);
            c.w64(0, c.r64(8));
            blk = 387; continue;
        }
        387 => {
            let Some(t360) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008db77d); };
            c.w64(8, t360);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008db784); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 391; continue; }
            blk = 388; continue;
        }
        388 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 390; continue; }
            blk = 389; continue;
        }
        389 => {
            let r361 = c.r64(10) | 0x1u64;
            c.w64(10, r361);
            c.logic(r361, 64);
            blk = 390; continue;
        }
        390 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 391; continue;
        }
        391 => {
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008db7af); }
            c.w64(8, c.r64(0));
            blk = 392; continue;
        }
        392 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008db7b7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008db7b8); }
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008db7ba); };
            c.w64(3, t362);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008db7cd, 0x70008db7c8);
        }
        393 => {
            let r363 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r363);
            c.w64(7, c.r64(0));
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db7d2); };
            c.w64(9, t364);
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008db7d6); };
            c.w64(11, t365);
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008db7dd); };
            c.w64(12, t366);
            blk = 180; continue;
        }
        394 => {
            c.w64(8, c.r64(11));
            let Some(t367) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008db7ec); };
            c.w64(11, t367);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008db7f3); }
            let Some(t368) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008db7f7); };
            c.cmp(t368, c.r64(15), 64);
            if c.cc_a() { blk = 397; continue; }
            blk = 395; continue;
        }
        395 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008db807, 0x70008db802);
        }
        396 => {
            blk = 398; continue;
        }
        397 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x70008db809); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 398; continue;
        }
        398 => {
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008db813); };
            c.w64(9, t369);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008db81a); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008db81e); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db825); }
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db82c); };
            c.w64(7, t370);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008db830); }
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db834); };
            c.w64(9, t371);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db83b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db83c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db842); }
            return c.ret(0x0, 0x70008db844);
        }
        399 => {
            let Some(t372) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008db845); };
            c.w64(0, t372);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008db84c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008db850); }
            let Some(t373) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008db854); };
            c.cmp(t373, c.r64(3), 64);
            if c.cc_a() { blk = 402; continue; }
            blk = 400; continue;
        }
        400 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008db864, 0x70008db85f);
        }
        401 => {
            blk = 403; continue;
        }
        402 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008db866); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 403; continue;
        }
        403 => {
            let Some(t374) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008db870); };
            c.w64(12, t374);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008db877); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008db87b); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008db882); }
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db889); };
            c.w64(7, t375);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x70008db88d); }
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008db891); };
            c.w64(15, t376);
            let Some(t377) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008db898); };
            c.w32(8, t377);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 405; continue; }
            blk = 404; continue;
        }
        404 => {
            return RegfileExit::fault(0x70008db8a7);
        }
        405 => {
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db8a8); };
            c.w64(8, t378);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008db8ac); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 407; continue; }
            blk = 406; continue;
        }
        406 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r379 = c.r64(7) & c.r64(15);
            c.w64(7, r379);
            c.logic(r379, 64);
            let Some(t380) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008db8be); };
            c.logic(t380 & 0x4u64, 8);
            if c.cc_ne() { blk = 454; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db8c8); };
            c.w64(8, t381);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008db8cf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008db8d0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008db8d6); }
            return c.ret(0x0, 0x70008db8d8);
        }
        408 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008db8d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008db8dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008db8e1); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008db8ef, 0x70008db8ea);
        }
        409 => {
            c.w64(8, c.r64(0));
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db8f2); };
            c.w64(9, t382);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db8f6); };
            c.w64(15, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db8fa); };
            c.w64(1, t384);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db8fe); };
            c.w64(7, t385);
            blk = 262; continue;
        }
        410 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008db907); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008db90b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008db90f); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008db91d, 0x70008db918);
        }
        411 => {
            c.w64(8, c.r64(0));
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db920); };
            c.w64(9, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db924); };
            c.w64(15, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db928); };
            c.w64(1, t388);
            let Some(t389) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008db92c); };
            c.w64(11, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db933); };
            c.w64(7, t390);
            blk = 265; continue;
        }
        412 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008db93c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008db940); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008db944); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008db948); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008db954, 0x70008db94f);
        }
        413 => {
            c.w64(7, c.r64(0));
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db957); };
            c.w64(9, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db95b); };
            c.w64(3, t392);
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db95f); };
            c.w64(15, t393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008db963); };
            c.w64(12, t394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db967); };
            c.w64(1, t395);
            blk = 274; continue;
        }
        414 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008db970); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008db974); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008db978); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008db97c); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008db988, 0x70008db983);
        }
        415 => {
            c.w64(7, c.r64(0));
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db98b); };
            c.w64(9, t396);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db98f); };
            c.w64(3, t397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db993); };
            c.w64(15, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008db997); };
            c.w64(12, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008db99b); };
            c.w64(1, t400);
            blk = 287; continue;
        }
        416 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008db9a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008db9a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008db9ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008db9b0); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008db9bc, 0x70008db9b7);
        }
        417 => {
            c.w64(7, c.r64(0));
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db9bf); };
            c.w64(9, t401);
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db9c3); };
            c.w64(3, t402);
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008db9c7); };
            c.w64(6, t403);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008db9ce); };
            c.w64(1, t404);
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008db9d2); };
            c.w64(15, t405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008db9d6); };
            c.w64(12, t406);
            blk = 302; continue;
        }
        418 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008db9df); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008db9e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008db9e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008db9eb); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008db9f7, 0x70008db9f2);
        }
        419 => {
            c.w64(7, c.r64(0));
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008db9fa); };
            c.w64(9, t407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008db9fe); };
            c.w64(3, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dba02); };
            c.w64(1, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dba06); };
            c.w64(15, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dba0a); };
            c.w64(12, t411);
            blk = 315; continue;
        }
        420 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008dba13); }
            blk = 424; continue;
        }
        421 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008dba19); }
            blk = 424; continue;
        }
        422 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008dba1f); }
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 424; continue; }
            blk = 423; continue;
        }
        423 => {
            let Some(t412) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dba29); };
            c.w64(12, t412);
            blk = 342; continue;
        }
        424 => {
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 426; continue; }
            blk = 425; continue;
        }
        425 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(11, c.r64(7));
            blk = 435; continue;
        }
        426 => {
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dba47); };
            c.w64(7, t413);
            let Some(t414) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008dba4e); };
            c.w64(11, t414);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t415) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dba55); };
            c.cmp(t415, c.r64(12), 64);
            if c.cc_a() { blk = 429; continue; }
            blk = 427; continue;
        }
        427 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008dba64, 0x70008dba5f);
        }
        428 => {
            c.w64(11, c.r64(0));
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dba67); };
            c.w64(9, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dba6b); };
            c.w64(0, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dba6f); };
            c.w64(3, t418);
            let Some(t419) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dba73); };
            c.w64(7, t419);
            let Some(t420) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dba7a); };
            c.w64(8, t420);
            blk = 430; continue;
        }
        429 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008dba83); }
            let r421 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r421);
            blk = 430; continue;
        }
        430 => {
            let Some(t422) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dba8a); };
            c.w64(12, t422);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008dba91); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 434; continue; }
            blk = 431; continue;
        }
        431 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 433; continue; }
            blk = 432; continue;
        }
        432 => {
            let r423 = c.r64(10) | 0x1u64;
            c.w64(10, r423);
            c.logic(r423, 64);
            blk = 433; continue;
        }
        433 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 434; continue;
        }
        434 => {
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dbabc); }
            blk = 435; continue;
        }
        435 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dbac2); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008dbac3); }
            c.w32(0, 0x2u64);
            let Some(t424) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008dbaca); };
            c.w64(3, t424);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008dbad8, 0x70008dbad3);
        }
        436 => {
            let r425 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r425);
            c.w64(7, c.r64(0));
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dbadd); };
            c.w64(9, t426);
            let Some(t427) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dbae1); };
            c.w64(8, t427);
            blk = 345; continue;
        }
        437 => {
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dbaed); };
            c.w64(11, t428);
            let Some(t429) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dbaf4); };
            c.w64(12, t429);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008dbafc); }
            let Some(t430) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dbb00); };
            c.cmp(t430, c.r64(15), 64);
            if c.cc_a() { blk = 440; continue; }
            blk = 438; continue;
        }
        438 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008dbb0f, 0x70008dbb0a);
        }
        439 => {
            blk = 441; continue;
        }
        440 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008dbb11); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 441; continue;
        }
        441 => {
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dbb1c); };
            c.w64(7, t431);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008dbb23); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008dbb26); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dbb2d); }
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dbb34); };
            c.w64(7, t432);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008dbb38); }
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dbb3c); };
            c.w64(9, t433);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dbb43); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dbb44); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dbb4a); }
            return c.ret(0x0, 0x70008dbb4c);
        }
        442 => {
            let Some(t434) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dbb4d); };
            c.w64(15, t434);
            let Some(t435) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008dbb54); };
            c.w64(0, t435);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008dbb5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008dbb5f); }
            let Some(t436) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dbb63); };
            c.cmp(t436, c.r64(3), 64);
            if c.cc_a() { blk = 445; continue; }
            blk = 443; continue;
        }
        443 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008dbb72, 0x70008dbb6d);
        }
        444 => {
            blk = 446; continue;
        }
        445 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008dbb74); }
            c.w64(8, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 446; continue;
        }
        446 => {
            let Some(t437) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dbb7e); };
            c.w64(7, t437);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008dbb85); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008dbb88); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dbb8f); }
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dbb96); };
            c.w64(7, t438);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x70008dbb9a); }
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008dbb9e); };
            c.w64(12, t439);
            let Some(t440) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dbba5); };
            c.w32(8, t440);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 448; continue; }
            blk = 447; continue;
        }
        447 => {
            return RegfileExit::fault(0x70008dbbb5);
        }
        448 => {
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dbbb6); };
            c.w64(11, t441);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008dbbba); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 450; continue; }
            blk = 449; continue;
        }
        449 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r442 = c.r64(7) & c.r64(12);
            c.w64(7, r442);
            c.logic(r442, 64);
            let Some(t443) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008dbbcc); };
            c.logic(t443 & 0x4u64, 8);
            if c.cc_ne() { blk = 451; continue; }
            blk = 450; continue;
        }
        450 => {
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dbbd2); };
            c.w64(7, t444);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dbbd9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dbbda); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dbbe0); }
            return c.ret(0x0, 0x70008dbbe2);
        }
        451 => {
            c.w32(0, c.r32(0));
            let r445 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r445);
            c.w64(7, 0xfffffffffffc0000u64);
            let r446 = c.r64(7) & c.r64(0);
            c.w64(7, r446);
            c.logic(r446, 64);
            let Some(t447) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008dbbf2); };
            c.logic(t447 & 0x2u64, 8);
            if c.cc_e() { blk = 450; continue; }
            blk = 452; continue;
        }
        452 => {
            c.w64(8, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dbbfc); }
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x70008dbc05, 0x70008dbc00);
        }
        453 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008dbc05); }
            blk = 450; continue;
        }
        454 => {
            c.w32(0, c.r32(0));
            let r448 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r448);
            c.w64(7, 0xfffffffffffc0000u64);
            let r449 = c.r64(7) & c.r64(0);
            c.w64(7, r449);
            c.logic(r449, 64);
            let Some(t450) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008dbc17); };
            c.logic(t450 & 0x2u64, 8);
            if c.cc_e() { blk = 407; continue; }
            blk = 455; continue;
        }
        455 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dbc25); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008dbc26); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008dbc33, 0x70008dbc2e);
        }
        456 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008dbc33); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008dbc35); }
            blk = 407; continue;
        }
        457 => {
            return RegfileExit::jump(0x70008dbc3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008dbc54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008dbc54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dbc58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dbc59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dbc5a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008dbc5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 355; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008dbc73); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 362; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dbc82); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbc89); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 357; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbc95); };
            c.w32(8, t6);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let r7 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008dbca3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008dbca7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008dbcab); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008dbcb1); }
            c.w64(12, c.r64(7));
            c.w64(15, c.r64(6));
            blk = 5; continue;
        }
        4 => {
            c.w64(12, c.r64(0));
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dbcc0); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbcc8); };
            c.w32(0, t10);
            c.w64(3, c.r64(0));
            let r11 = c.r32(3) & 0xfu64;
            c.w32(3, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(2, c.r64(0));
            let r12 = c.r32(2) & 0x7u64;
            c.w32(2, r12);
            c.logic(r12, 32);
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dbcef); };
            c.w32(12, t13);
            let r14 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r14);
            c.w64(0, c.r64(12));
            blk = 4; continue;
        }
        9 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dbd01); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbd09); };
            c.cmp(t17, 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dbd0f); }
            c.w64(0, c.r64(12));
            return c.call(0x70008e88b4, 0x70008dbd1b, 0x70008dbd16);
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dbd1b); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dbd1f); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dbd23); };
            c.w64(7, t20);
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 4; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dbd2b); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            c.w64(0, c.r64(12));
            blk = 4; continue;
        }
        14 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dbd41); };
            c.w32(8, t23);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r24 = c.add(c.r64(15), c.r64(8), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dbd4f); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w64(0, c.r64(12));
            blk = 4; continue;
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbd5f); };
            c.w32(8, t27);
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dbd64); };
            c.cmp(c.r32(8), t28, 32);
            if c.cc_b() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dbd6d); };
            let r30 = c.sub(c.r64(8), t29, 64);
            c.w64(8, r30);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(15).wrapping_add(0xc));
            blk = 32; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008dbd7f);
        }
        20 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbd80); };
            c.w32(8, t31);
            let Some(t32) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dbd85); };
            c.cmp(c.r32(8), t32, 32);
            if c.cc_b() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dbd8e); };
            let r34 = c.sub(c.r64(8), t33, 64);
            c.w64(8, r34);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(15).wrapping_mul(2).wrapping_add(0xc));
            c.w64(15, c.r64(8));
            c.w32(2, 0xcu64);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dbda7); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dbdae); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dbdb5); };
            c.w64(1, t37);
            blk = 196; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008dbdc1);
        }
        24 => {
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dbdcf); };
            c.w64(8, t38);
            c.w64(0, c.r64(8));
            blk = 29; continue;
        }
        27 => {
            c.w64(10, c.r64(4));
            let r39 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r39);
            let r40 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r40);
            c.logic(r40, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008dbde4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008dbde9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dbded); }
            c.w64(1, c.r64(12));
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008dbdf4); };
            c.w64(0, t41);
            c.w64(10, 0x70008dbe02u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008dbe02); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008dbe06); }
            return c.call(c.r64(0), 0x70008dbe0c, 0x70008dbe0a);
        }
        28 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008dbe0c); }
            let Some(t42) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008dbe14); };
            c.w64(4, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dbe19); };
            c.w64(15, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dbe1d); };
            c.w64(9, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dbe21); };
            c.w64(7, t45);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dbe27); };
            c.w64(12, t46);
            blk = 29; continue;
        }
        29 => {
            c.w64(1, c.r64(12));
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbe2e); };
            c.w32(8, t47);
            let Some(t48) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dbe32); };
            c.cmp(c.r32(8), t48, 32);
            if c.cc_b() { blk = 187; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dbe3f); };
            let r50 = c.sub(c.r64(8), t49, 64);
            c.w64(8, r50);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 187; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(8, c.r64(15).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w64(12, c.r64(6));
            blk = 32; continue;
        }
        32 => {
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbe54); };
            c.w32(11, t51);
            c.w64(15, c.r64(11).wrapping_sub(0x1));
            c.w32(2, 0x1u64);
            blk = 34; continue;
        }
        33 => {
            let r52 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r52);
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r64(15), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dbe71); };
            c.cmp(c.r64(15), t53, 64);
            if c.cc_ae() { blk = 67; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(15)));
            let Some(t54) = c.ld8(c.r64(12).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dbe7f); };
            c.w32(0, t54);
            c.cmp(c.r8(0), 0x20u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(3, c.r64(0).wrapping_sub(0xe));
            c.cmp(c.r32(3), 0x92u64, 32);
            if c.cc_b() { blk = 63; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r8(0), 0x9u64, 8);
            if c.cc_b() { blk = 62; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r8(0), 0xdu64, 8);
            if c.cc_be() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r8(0), 0xffffffffffffffa0u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(0), 0x1680u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(0), 0x2000u64, 32);
            if c.cc_b() { blk = 58; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(0), 0x200au64, 32);
            if c.cc_be() { blk = 57; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(0), 0x2028u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(0), 0x2029u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(0), 0x202fu64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(0), 0x205fu64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(0), 0xfeffu64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(0), 0x3000u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        51 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        52 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        53 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        54 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        55 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        56 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        57 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        58 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        59 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        60 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        61 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        62 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        63 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        64 => {
            c.w64(0, c.r64(2));
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            blk = 33; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008dbf4e);
        }
        68 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 70; continue;
        }
        69 => {
            c.w64(8, c.r64(2));
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(8, c.r64(15).wrapping_add(0x1));
            let Some(t55) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dbf65); };
            c.w32(12, t55);
            let r56 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r56);
            let Some(t57) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dbf6c); };
            c.w32(12, t57);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_be() { blk = 185; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 184; continue; }
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(12) & 0x7u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_e() { blk = 159; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 90; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(11, c.r64(6));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(7));
            blk = 76; continue;
        }
        76 => {
            c.w64(0, c.r64(15));
            let r58 = c.r32(0) & 0x7u64;
            c.w32(0, r58);
            c.logic(r58, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 157; continue;
        }
        81 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 157; continue;
        }
        84 => {
            let Some(t59) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dbfe0); };
            c.w32(15, t59);
            let Some(t60) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dbfe5); };
            c.cmp(t60, c.r32(15), 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t61) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dbff2); };
            c.w32(12, t61);
            let r62 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r62);
            let Some(t63) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dbffa); };
            c.w32(15, t63);
            let r64 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r64);
            let Some(t65) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc002); };
            c.w32(15, t65);
            blk = 76; continue;
        }
        86 => {
            let Some(t66) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dc009); };
            c.w32(15, t66);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r67 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r67);
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc017); };
            c.w32(12, t68);
            let r69 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r69);
            let Some(t70) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc01f); };
            c.w32(15, t70);
            let r71 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r71);
            let Some(t72) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc027); };
            c.w32(15, t72);
            blk = 76; continue;
        }
        87 => {
            let Some(t73) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc02e); };
            c.w32(12, t73);
            let r74 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r74);
            let Some(t75) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc036); };
            c.w32(15, t75);
            let r76 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r76);
            let Some(t77) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc03e); };
            c.w32(15, t77);
            blk = 76; continue;
        }
        88 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 91; continue;
        }
        89 => {
            c.w64(0, c.r64(2));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 91; continue;
        }
        90 => {
            c.w64(11, c.r64(6));
            c.w64(15, c.r64(7));
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r64(8), 0xdu64, 64);
            if c.cc_l() { blk = 99; continue; }
            blk = 92; continue;
        }
        92 => {
            let r78 = c.r32(12) & 0x8u64;
            c.w32(12, r78);
            c.logic(r78, 32);
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc083); };
            c.w64(3, t79);
            let Some(t80) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008dc08a); };
            c.w64(11, t80);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc091); };
            c.w64(1, t81);
            let Some(t82) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dc098); };
            c.cmp(t82, c.r64(12), 64);
            if c.cc_be() { blk = 364; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70008dc0a1); }
            let r83 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r83);
            blk = 95; continue;
        }
        95 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008dc0a8); };
            c.w64(12, t84);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008dc0af); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc0b3); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008dc0bb); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008dc0bf); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008dc0c3); }
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc0cd); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc0ce); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc0d4); }
            return c.ret(0x0, 0x70008dc0d6);
        }
        96 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc0d7); };
            c.w64(3, t85);
            let Some(t86) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008dc0de); };
            c.w64(11, t86);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc0e5); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dc0ec); };
            c.cmp(t88, c.r64(12), 64);
            if c.cc_be() { blk = 366; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x70008dc0f5); }
            let r89 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r89);
            blk = 98; continue;
        }
        98 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008dc0fc); };
            c.w64(6, t90);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008dc103); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc107); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008dc10f); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008dc113); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008dc117); }
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc121); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc122); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc128); }
            return c.ret(0x0, 0x70008dc12a);
        }
        99 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w32(7, c.r32(8));
            let r91 = c.r32(12) & 0x8u64;
            c.w32(12, r91);
            c.logic(r91, 32);
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w32(7, c.r32(8));
            c.w64(12, c.r64(7));
            let r92 = c.add(c.r32(12), c.r32(7), 32);
            c.w32(12, r92);
            if c.cc_o() { blk = 109; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(0, 0xcu64);
            let r93 = c.add(c.r64(7), c.r64(0), 64);
            c.w64(7, r93);
            if c.cc_o() { blk = 108; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc16d); };
            c.w64(3, t94);
            let Some(t95) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008dc174); };
            c.w64(12, t95);
            let r96 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r96);
            let r97 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r97);
            c.logic(r97, 64);
            c.w64(0, c.r64(12).wrapping_add(c.r64(7)));
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc183); };
            c.w64(1, t98);
            let Some(t99) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dc18a); };
            c.cmp(t99, c.r64(0), 64);
            if c.cc_be() { blk = 368; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 368; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x70008dc1a0); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 107; continue;
        }
        107 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dc1a8); };
            c.w64(2, t100);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008dc1af); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc1b2); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008dc1b9); }
            c.w64(12, c.r64(7));
            blk = 111; continue;
        }
        108 => {
            return RegfileExit::fault(0x70008dc1c2);
        }
        109 => {
            return RegfileExit::fault(0x70008dc1c3);
        }
        110 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc1c4); };
            c.w64(12, t101);
            blk = 111; continue;
        }
        111 => {
            c.w64(7, c.r64(11).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w32(2, 0xbu64);
            blk = 114; continue;
        }
        113 => {
            let r102 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r102);
            c.w64(7, c.r64(11));
            blk = 114; continue;
        }
        114 => {
            c.w64(11, c.r64(7).wrapping_add(0x2));
            let Some(t103) = c.ld16(c.r64(15).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008dc1ee); };
            c.w32(7, t103);
            if !c.st16(c.r64(12).wrapping_add(c.r64(2)), c.r16(7)) { return RegfileExit::fault(0x70008dc1f3); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 113; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc203); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc204); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc20a); }
            return c.ret(0x0, 0x70008dc20c);
        }
        116 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 123; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(0, 0xcu64);
            let r104 = c.add(c.r64(7), c.r64(0), 64);
            c.w64(7, r104);
            if c.cc_o() { blk = 122; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc21f); };
            c.w64(3, t105);
            let Some(t106) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008dc226); };
            c.w64(12, t106);
            let r107 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r107);
            let r108 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r108);
            c.logic(r108, 64);
            c.w64(0, c.r64(12).wrapping_add(c.r64(7)));
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc235); };
            c.w64(1, t109);
            let Some(t110) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dc23c); };
            c.cmp(t110, c.r64(0), 64);
            if c.cc_be() { blk = 370; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 370; continue; }
            blk = 120; continue;
        }
        120 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x70008dc252); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 121; continue;
        }
        121 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dc25a); };
            c.w64(12, t111);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008dc261); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc265); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008dc26c); }
            c.w64(12, c.r64(7));
            blk = 124; continue;
        }
        122 => {
            return RegfileExit::fault(0x70008dc275);
        }
        123 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc276); };
            c.w64(12, t112);
            blk = 124; continue;
        }
        124 => {
            c.w64(7, c.r64(11).wrapping_add(0xb));
            let r113 = c.add(c.r64(8), c.r64(7), 64);
            c.w64(8, r113);
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 128; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(2, 0xbu64);
            blk = 127; continue;
        }
        126 => {
            let r114 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r114);
            c.w64(7, c.r64(11));
            blk = 127; continue;
        }
        127 => {
            c.w64(11, c.r64(7).wrapping_add(0x1));
            let Some(t115) = c.ld8(c.r64(15).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008dc29b); };
            c.w32(7, t115);
            if !c.st8(c.r64(12).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008dc2a0); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 126; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc2af); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc2b0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc2b6); }
            return c.ret(0x0, 0x70008dc2b8);
        }
        129 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc2b9); };
            c.w64(0, t116);
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dc2c0); };
            c.w64(3, t117);
            c.w32(2, 0xbu64);
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 157; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w32(7, c.r32(8));
            let Some(t118) = c.ld64(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dc2d9); };
            c.w64(15, t118);
            let r119 = c.r32(12) & 0x8u64;
            c.w32(12, r119);
            c.logic(r119, 32);
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 140; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w32(7, c.r32(8));
            c.w64(12, c.r64(7));
            let r120 = c.add(c.r32(12), c.r32(7), 32);
            c.w32(12, r120);
            if c.cc_o() { blk = 139; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(7, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(1, 0xcu64);
            let r121 = c.add(c.r64(7), c.r64(1), 64);
            c.w64(7, r121);
            if c.cc_o() { blk = 138; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc313); };
            c.w64(6, t122);
            let Some(t123) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008dc31a); };
            c.w64(12, t123);
            let r124 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r124);
            let r125 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r125);
            c.logic(r125, 64);
            c.w64(3, c.r64(12).wrapping_add(c.r64(7)));
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc329); };
            c.w64(0, t126);
            let Some(t127) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008dc330); };
            c.cmp(t127, c.r64(3), 64);
            if c.cc_be() { blk = 372; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 372; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.st64(c.r64(6), c.r64(3)) { return RegfileExit::fault(0x70008dc346); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 137; continue;
        }
        137 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dc34e); };
            c.w64(12, t128);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008dc355); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc359); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008dc360); }
            c.w64(0, c.r64(7));
            blk = 140; continue;
        }
        138 => {
            return RegfileExit::fault(0x70008dc369);
        }
        139 => {
            return RegfileExit::fault(0x70008dc36a);
        }
        140 => {
            c.w64(7, c.r64(11).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 143; continue;
        }
        142 => {
            let r129 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r129);
            c.w64(7, c.r64(11));
            blk = 143; continue;
        }
        143 => {
            c.w64(11, c.r64(7).wrapping_add(0x2));
            let Some(t130) = c.ld16(c.r64(7).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008dc389); };
            c.w32(7, t130);
            if !c.st16(c.r64(0).wrapping_add(c.r64(2)), c.r16(7)) { return RegfileExit::fault(0x70008dc38f); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 142; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc39b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc39c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc3a2); }
            return c.ret(0x0, 0x70008dc3a4);
        }
        145 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc3a5); };
            c.w64(1, t131);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(7, 0xcu64);
            let r132 = c.add(c.r64(12), c.r64(7), 64);
            c.w64(12, r132);
            if c.cc_o() { blk = 151; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc3be); };
            c.w64(0, t133);
            let Some(t134) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008dc3c5); };
            c.w64(7, t134);
            let r135 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r135);
            let r136 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r136);
            c.logic(r136, 64);
            c.w64(6, c.r64(7).wrapping_add(c.r64(12)));
            let Some(t137) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dc3d4); };
            c.cmp(t137, c.r64(6), 64);
            if c.cc_be() { blk = 374; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 374; continue; }
            blk = 149; continue;
        }
        149 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008dc3ea); }
            let r138 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r138);
            blk = 150; continue;
        }
        150 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008dc3f1); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc3f4); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008dc3fb); }
            c.w64(0, c.r64(7));
            blk = 152; continue;
        }
        151 => {
            return RegfileExit::fault(0x70008dc404);
        }
        152 => {
            c.w64(7, c.r64(11).wrapping_add(0xb));
            let r139 = c.add(c.r64(8), c.r64(7), 64);
            c.w64(8, r139);
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 156; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 155; continue;
        }
        154 => {
            let r140 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r140);
            c.w64(7, c.r64(11));
            blk = 155; continue;
        }
        155 => {
            c.w64(11, c.r64(7).wrapping_add(0x1));
            let Some(t141) = c.ld8(c.r64(7).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008dc41e); };
            c.w32(7, t141);
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008dc424); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 154; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc430); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc431); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc437); }
            return c.ret(0x0, 0x70008dc439);
        }
        157 => {
            let r142 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r142);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dc43d); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dc43e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008dc43f); }
            c.w32(0, 0x3u64);
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008dc446); };
            c.w64(3, t143);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008dc458, 0x70008dc453);
        }
        158 => {
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dc458); };
            c.w64(9, t144);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc45f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc460); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc466); }
            return c.ret(0x0, 0x70008dc468);
        }
        159 => {
            c.w32(12, c.r16(12));
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dc46d); };
            c.w64(8, t145);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 175; continue; }
            blk = 160; continue;
        }
        160 => {
            c.w64(11, c.r64(6));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(7));
            blk = 161; continue;
        }
        161 => {
            c.w64(0, c.r64(15));
            let r146 = c.r32(0) & 0x7u64;
            c.w32(0, r146);
            c.logic(r146, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 166; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 378; continue;
        }
        166 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 378; continue;
        }
        169 => {
            let Some(t147) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dc4b9); };
            c.w32(15, t147);
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc4be); };
            c.cmp(t148, c.r32(15), 32);
            if c.cc_ne() { blk = 378; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t149) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc4cb); };
            c.w32(12, t149);
            let r150 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r150);
            let Some(t151) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc4d3); };
            c.w32(15, t151);
            let r152 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r152);
            let Some(t153) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc4db); };
            c.w32(15, t153);
            blk = 161; continue;
        }
        171 => {
            let Some(t154) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dc4e2); };
            c.w32(15, t154);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r155 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r155);
            let Some(t156) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc4f0); };
            c.w32(12, t156);
            let r157 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r157);
            let Some(t158) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc4f8); };
            c.w32(15, t158);
            let r159 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r159);
            let Some(t160) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc500); };
            c.w32(15, t160);
            blk = 161; continue;
        }
        172 => {
            let Some(t161) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc507); };
            c.w32(12, t161);
            let r162 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r162);
            let Some(t163) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc50f); };
            c.w32(15, t163);
            let r164 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r164);
            let Some(t165) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc517); };
            c.w32(15, t165);
            blk = 161; continue;
        }
        173 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 176; continue;
        }
        174 => {
            c.w64(0, c.r64(2));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 176; continue;
        }
        175 => {
            c.w64(11, c.r64(6));
            c.w64(15, c.r64(7));
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 376; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(7, c.r64(15).wrapping_add(0xb));
            blk = 178; continue;
        }
        178 => {
            let r166 = c.r32(12) & 0x8u64;
            c.w32(12, r166);
            c.logic(r166, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t167) = c.ld16(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x70008dc55b); };
            c.w32(7, t167);
            blk = 181; continue;
        }
        180 => {
            let Some(t168) = c.ld8(c.r64(7).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008dc562); };
            c.w32(7, t168);
            blk = 181; continue;
        }
        181 => {
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 380; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(8, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008dc57b); };
            c.w64(11, t169);
            let Some(t170) = c.ld32(c.r64(11).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc582); };
            c.w32(0, t170);
            let r171 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dc58a); };
            c.cmp(t172, c.r32(0), 32);
            if c.cc_e() { blk = 385; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc59a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc59b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc5a1); }
            return c.ret(0x0, 0x70008dc5a3);
        }
        184 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc5a4); };
            c.w64(0, t173);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc5ae); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc5af); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc5b5); }
            return c.ret(0x0, 0x70008dc5b7);
        }
        185 => {
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc5be); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc5bf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc5c5); }
            return c.ret(0x0, 0x70008dc5c7);
        }
        186 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc5c8); };
            c.w64(0, t174);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc5d2); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc5d3); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc5d9); }
            return c.ret(0x0, 0x70008dc5db);
        }
        187 => {
            return RegfileExit::fault(0x70008dc5dc);
        }
        188 => {
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc5dd); };
            c.w64(8, t175);
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc5e4); };
            c.w64(11, t176);
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dc5eb); };
            c.w64(1, t177);
            c.w32(2, 0xcu64);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_ne() { blk = 354; continue; }
            blk = 189; continue;
        }
        189 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t178) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dc604); };
            c.w64(0, t178);
            blk = 193; continue;
        }
        191 => {
            c.w64(10, c.r64(4));
            let r179 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r179);
            let r180 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r180);
            c.logic(r180, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008dc616); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008dc61b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dc61f); }
            c.w64(1, c.r64(12));
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008dc626); };
            c.w64(0, t181);
            c.w64(10, 0x70008dc634u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008dc634); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008dc638); }
            return c.call(c.r64(0), 0x70008dc63e, 0x70008dc63c);
        }
        192 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008dc63e); }
            let Some(t182) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008dc646); };
            c.w64(4, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dc64b); };
            c.w64(15, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dc64f); };
            c.w64(9, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dc653); };
            c.w64(7, t185);
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dc657); };
            c.w64(8, t186);
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dc65e); };
            c.w64(11, t187);
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dc665); };
            c.w64(1, t188);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(2, 0xcu64);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dc673); };
            c.w64(12, t189);
            blk = 193; continue;
        }
        193 => {
            let Some(t190) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc677); };
            c.w32(12, t190);
            let Some(t191) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dc67c); };
            c.cmp(c.r32(12), t191, 32);
            if c.cc_b() { blk = 353; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dc689); };
            let r193 = c.sub(c.r64(12), t192, 64);
            c.w64(12, r193);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 353; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(12, c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x1));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(6));
            blk = 196; continue;
        }
        196 => {
            c.w64(0, c.r64(7));
            let Some(t194) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc6a4); };
            c.w32(7, t194);
            c.w64(3, c.r64(7).wrapping_sub(0x1));
            blk = 198; continue;
        }
        197 => {
            let r195 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r195);
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r64(3), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 233; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dc6bb); };
            c.cmp(c.r64(3), t196, 64);
            if c.cc_ae() { blk = 231; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w64(1, c.r64(15).wrapping_add(c.r64(3).wrapping_mul(2)));
            let Some(t197) = c.ld16(c.r64(12).wrapping_add(c.r64(1)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc6c9); };
            c.w32(1, t197);
            c.cmp(c.r16(1), 0x20u64, 16);
            if c.cc_e() { blk = 228; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w32(2, c.r64(1).wrapping_sub(0xe));
            c.cmp(c.r32(2), 0x92u64, 32);
            if c.cc_b() { blk = 227; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r16(1), 0x9u64, 16);
            if c.cc_b() { blk = 226; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r16(1), 0xdu64, 16);
            if c.cc_be() { blk = 225; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r16(1), 0xa0u64, 16);
            if c.cc_e() { blk = 224; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r16(1), 0x1680u64, 16);
            if c.cc_e() { blk = 223; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r16(1), 0x2000u64, 16);
            if c.cc_b() { blk = 222; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r16(1), 0x200au64, 16);
            if c.cc_be() { blk = 221; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r16(1), 0x2028u64, 16);
            if c.cc_e() { blk = 220; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r16(1), 0x2029u64, 16);
            if c.cc_e() { blk = 219; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r16(1), 0x202fu64, 16);
            if c.cc_e() { blk = 218; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r16(1), 0x205fu64, 16);
            if c.cc_e() { blk = 217; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r16(1), 0xfeffu64, 16);
            if c.cc_e() { blk = 216; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r16(1), 0x3000u64, 16);
            if c.cc_e() { blk = 215; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 229; continue;
        }
        215 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        216 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        217 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        218 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        219 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        220 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        221 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        222 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 229; continue;
        }
        223 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        224 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        225 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        226 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 229; continue;
        }
        227 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 229; continue;
        }
        228 => {
            c.w32(2, 0x1u64);
            blk = 229; continue;
        }
        229 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            blk = 197; continue;
        }
        231 => {
            return RegfileExit::fault(0x70008dc7b7);
        }
        232 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 234; continue;
        }
        233 => {
            c.w32(12, 0x1u64);
            blk = 234; continue;
        }
        234 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 352; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(12, c.r64(3).wrapping_add(0x1));
            let Some(t198) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc7d1); };
            c.w32(15, t198);
            let r199 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r199);
            let Some(t200) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc7d8); };
            c.w32(15, t200);
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_be() { blk = 351; continue; }
            blk = 236; continue;
        }
        236 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 350; continue; }
            blk = 237; continue;
        }
        237 => {
            c.logic(c.r8(15) & 0x7u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 325; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 254; continue; }
            blk = 239; continue;
        }
        239 => {
            c.w64(7, c.r64(6));
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(0));
            blk = 240; continue;
        }
        240 => {
            c.w64(2, c.r64(3));
            let r201 = c.r32(2) & 0x7u64;
            c.w32(2, r201);
            c.logic(r201, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 245; continue; }
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 242; continue;
        }
        242 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 243; continue;
        }
        243 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 244; continue;
        }
        244 => {
            blk = 323; continue;
        }
        245 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 246; continue;
        }
        246 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            blk = 323; continue;
        }
        248 => {
            let Some(t202) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dc84a); };
            c.w32(3, t202);
            let Some(t203) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc84e); };
            c.cmp(t203, c.r32(3), 32);
            if c.cc_ne() { blk = 323; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t204) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc85b); };
            c.w32(15, t204);
            let r205 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r205);
            let Some(t206) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc862); };
            c.w32(3, t206);
            let r207 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r207);
            let Some(t208) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc869); };
            c.w32(3, t208);
            blk = 240; continue;
        }
        250 => {
            let Some(t209) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dc86f); };
            c.w32(3, t209);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r210 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r210);
            let Some(t211) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc87c); };
            c.w32(15, t211);
            let r212 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r212);
            let Some(t213) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc883); };
            c.w32(3, t213);
            let r214 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r214);
            let Some(t215) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc88a); };
            c.w32(3, t215);
            blk = 240; continue;
        }
        251 => {
            let Some(t216) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dc890); };
            c.w32(15, t216);
            let r217 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r217);
            let Some(t218) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dc897); };
            c.w32(3, t218);
            let r219 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r219);
            let Some(t220) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dc89e); };
            c.w32(3, t220);
            blk = 240; continue;
        }
        252 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 255; continue;
        }
        253 => {
            c.w32(2, 0x1u64);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 255; continue;
        }
        254 => {
            c.w64(7, c.r64(6));
            c.w64(3, c.r64(0));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r64(12), 0xdu64, 64);
            if c.cc_l() { blk = 263; continue; }
            blk = 256; continue;
        }
        256 => {
            let r221 = c.r32(15) & 0x8u64;
            c.w32(15, r221);
            c.logic(r221, 32);
            let r222 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r222);
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t223) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dc8e2); };
            c.w64(15, t223);
            c.w64(0, c.r64(15).wrapping_add(0x14));
            let Some(t224) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dc8e9); };
            c.cmp(t224, c.r64(0), 64);
            if c.cc_be() { blk = 394; continue; }
            blk = 258; continue;
        }
        258 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008dc8f2); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 259; continue;
        }
        259 => {
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008dc8f9); };
            c.w64(11, t225);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008dc900); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc904); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008dc90c); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008dc910); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008dc914); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc91e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc91f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc925); }
            return c.ret(0x0, 0x70008dc927);
        }
        260 => {
            let Some(t226) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dc928); };
            c.w64(15, t226);
            c.w64(0, c.r64(15).wrapping_add(0x14));
            let Some(t227) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dc92f); };
            c.cmp(t227, c.r64(0), 64);
            if c.cc_be() { blk = 396; continue; }
            blk = 261; continue;
        }
        261 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008dc938); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 262; continue;
        }
        262 => {
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008dc93f); };
            c.w64(11, t228);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008dc946); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc94a); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008dc952); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008dc956); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008dc95a); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dc964); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dc965); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dc96b); }
            return c.ret(0x0, 0x70008dc96d);
        }
        263 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 293; continue; }
            blk = 264; continue;
        }
        264 => {
            c.w32(0, c.r32(12));
            let r229 = c.r32(15) & 0x8u64;
            c.w32(15, r229);
            c.logic(r229, 32);
            c.cmp(c.r32(0), 0x0u64, 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 280; continue; }
            blk = 265; continue;
        }
        265 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w32(15, c.r32(12));
            c.w64(0, c.r64(15));
            let r230 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r230);
            if c.cc_o() { blk = 273; continue; }
            blk = 267; continue;
        }
        267 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r231 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r231);
            if c.cc_o() { blk = 272; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t232) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dc9ae); };
            c.w64(0, t232);
            let r233 = c.add(c.r64(15), 0x3u64, 64);
            c.w64(15, r233);
            let r234 = c.r64(15) & 0xfffffffffffffffcu64;
            c.w64(15, r234);
            c.logic(r234, 64);
            c.w64(2, c.r64(0).wrapping_add(c.r64(15)));
            let Some(t235) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dc9bd); };
            c.cmp(t235, c.r64(2), 64);
            if c.cc_be() { blk = 398; continue; }
            blk = 269; continue;
        }
        269 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 398; continue; }
            blk = 270; continue;
        }
        270 => {
            if !c.st64(c.r64(8), c.r64(2)) { return RegfileExit::fault(0x70008dc9d3); }
            c.w64(8, c.r64(0).wrapping_add(0x1));
            blk = 271; continue;
        }
        271 => {
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dc9da); };
            c.w64(1, t236);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008dc9e1); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dc9e5); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008dc9ed); }
            c.w64(0, c.r64(8));
            blk = 275; continue;
        }
        272 => {
            return RegfileExit::fault(0x70008dc9f6);
        }
        273 => {
            return RegfileExit::fault(0x70008dc9f7);
        }
        274 => {
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dc9f8); };
            c.w64(0, t237);
            blk = 275; continue;
        }
        275 => {
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 279; continue; }
            blk = 276; continue;
        }
        276 => {
            c.w32(2, 0xbu64);
            blk = 278; continue;
        }
        277 => {
            let r238 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r238);
            c.w64(7, c.r64(11));
            blk = 278; continue;
        }
        278 => {
            c.w64(11, c.r64(7).wrapping_add(0x2));
            let Some(t239) = c.ld16(c.r64(3).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008dca22); };
            c.w32(7, t239);
            if !c.st16(c.r64(0).wrapping_add(c.r64(2)), c.r16(7)) { return RegfileExit::fault(0x70008dca26); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 277; continue; }
            blk = 279; continue;
        }
        279 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dca32); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dca33); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dca39); }
            return c.ret(0x0, 0x70008dca3b);
        }
        280 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 287; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(2, 0xcu64);
            let r240 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r240);
            if c.cc_o() { blk = 286; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t241) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dca4e); };
            c.w64(0, t241);
            let r242 = c.add(c.r64(15), 0x3u64, 64);
            c.w64(15, r242);
            let r243 = c.r64(15) & 0xfffffffffffffffcu64;
            c.w64(15, r243);
            c.logic(r243, 64);
            c.w64(2, c.r64(0).wrapping_add(c.r64(15)));
            let Some(t244) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dca5d); };
            c.cmp(t244, c.r64(2), 64);
            if c.cc_be() { blk = 400; continue; }
            blk = 283; continue;
        }
        283 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 400; continue; }
            blk = 284; continue;
        }
        284 => {
            if !c.st64(c.r64(8), c.r64(2)) { return RegfileExit::fault(0x70008dca73); }
            c.w64(8, c.r64(0).wrapping_add(0x1));
            blk = 285; continue;
        }
        285 => {
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dca7a); };
            c.w64(11, t245);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008dca81); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dca85); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008dca8d); }
            c.w64(0, c.r64(8));
            blk = 288; continue;
        }
        286 => {
            return RegfileExit::fault(0x70008dca96);
        }
        287 => {
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dca97); };
            c.w64(0, t246);
            blk = 288; continue;
        }
        288 => {
            let r247 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r247);
            c.w64(8, c.r64(7).wrapping_add(c.r64(12)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 292; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w32(2, 0xbu64);
            blk = 291; continue;
        }
        290 => {
            let r248 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r248);
            c.w64(7, c.r64(11));
            blk = 291; continue;
        }
        291 => {
            c.w64(11, c.r64(7).wrapping_add(0x1));
            let Some(t249) = c.ld8(c.r64(3).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008dcabd); };
            c.w32(7, t249);
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008dcac1); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 290; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcacd); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcace); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcad4); }
            return c.ret(0x0, 0x70008dcad6);
        }
        293 => {
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dcad7); };
            c.w64(2, t250);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 323; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w32(0, c.r32(12));
            let Some(t251) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dcaeb); };
            c.w64(3, t251);
            let r252 = c.r32(15) & 0x8u64;
            c.w32(15, r252);
            c.logic(r252, 32);
            c.cmp(c.r32(0), 0x0u64, 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 310; continue; }
            blk = 295; continue;
        }
        295 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 304; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w32(15, c.r32(12));
            c.w64(0, c.r64(15));
            let r253 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r253);
            if c.cc_o() { blk = 303; continue; }
            blk = 297; continue;
        }
        297 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r254 = c.add(c.r64(0), c.r64(15), 64);
            c.w64(0, r254);
            if c.cc_o() { blk = 302; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t255) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dcb24); };
            c.w64(15, t255);
            let r256 = c.add(c.r64(0), 0x3u64, 64);
            c.w64(0, r256);
            let r257 = c.r64(0) & 0xfffffffffffffffcu64;
            c.w64(0, r257);
            c.logic(r257, 64);
            c.w64(2, c.r64(15).wrapping_add(c.r64(0)));
            let Some(t258) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dcb33); };
            c.cmp(t258, c.r64(2), 64);
            if c.cc_be() { blk = 402; continue; }
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 402; continue; }
            blk = 300; continue;
        }
        300 => {
            if !c.st64(c.r64(8), c.r64(2)) { return RegfileExit::fault(0x70008dcb48); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 301; continue;
        }
        301 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dcb4f); };
            c.w64(1, t259);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008dcb56); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dcb5a); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008dcb62); }
            c.w64(0, c.r64(8));
            blk = 305; continue;
        }
        302 => {
            return RegfileExit::fault(0x70008dcb6b);
        }
        303 => {
            return RegfileExit::fault(0x70008dcb6c);
        }
        304 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dcb6d); };
            c.w64(0, t260);
            blk = 305; continue;
        }
        305 => {
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 309; continue; }
            blk = 306; continue;
        }
        306 => {
            c.w32(2, 0xbu64);
            blk = 308; continue;
        }
        307 => {
            let r261 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r261);
            c.w64(7, c.r64(11));
            blk = 308; continue;
        }
        308 => {
            c.w64(11, c.r64(7).wrapping_add(0x2));
            let Some(t262) = c.ld16(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008dcb97); };
            c.w32(7, t262);
            if !c.st16(c.r64(0).wrapping_add(c.r64(2)), c.r16(7)) { return RegfileExit::fault(0x70008dcb9c); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 307; continue; }
            blk = 309; continue;
        }
        309 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcba8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcba9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcbaf); }
            return c.ret(0x0, 0x70008dcbb1);
        }
        310 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 317; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(0, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r263 = c.add(c.r64(0), c.r64(15), 64);
            c.w64(0, r263);
            if c.cc_o() { blk = 316; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t264) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dcbc5); };
            c.w64(15, t264);
            let r265 = c.add(c.r64(0), 0x3u64, 64);
            c.w64(0, r265);
            let r266 = c.r64(0) & 0xfffffffffffffffcu64;
            c.w64(0, r266);
            c.logic(r266, 64);
            c.w64(1, c.r64(15).wrapping_add(c.r64(0)));
            let Some(t267) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dcbd4); };
            c.cmp(t267, c.r64(1), 64);
            if c.cc_be() { blk = 404; continue; }
            blk = 313; continue;
        }
        313 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_ae() { blk = 404; continue; }
            blk = 314; continue;
        }
        314 => {
            if !c.st64(c.r64(8), c.r64(1)) { return RegfileExit::fault(0x70008dcbe9); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 315; continue;
        }
        315 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008dcbf0); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dcbf4); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008dcbfc); }
            c.w64(0, c.r64(8));
            blk = 318; continue;
        }
        316 => {
            return RegfileExit::fault(0x70008dcc05);
        }
        317 => {
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dcc06); };
            c.w64(0, t268);
            blk = 318; continue;
        }
        318 => {
            let r269 = c.add(c.r64(7), 0xbu64, 64);
            c.w64(7, r269);
            c.w64(8, c.r64(7).wrapping_add(c.r64(12)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 322; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w32(2, 0xbu64);
            blk = 321; continue;
        }
        320 => {
            let r270 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r270);
            c.w64(7, c.r64(11));
            blk = 321; continue;
        }
        321 => {
            c.w64(11, c.r64(7).wrapping_add(0x1));
            let Some(t271) = c.ld8(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008dcc2c); };
            c.w32(7, t271);
            if !c.st8(c.r64(0).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008dcc31); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ne() { blk = 320; continue; }
            blk = 322; continue;
        }
        322 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcc3d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcc3e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcc44); }
            return c.ret(0x0, 0x70008dcc46);
        }
        323 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dcc4b); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dcc4c); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dcc4d); }
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008dcc4e); };
            c.w64(3, t272);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008dcc5f, 0x70008dcc5a);
        }
        324 => {
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcc5f); };
            c.w64(9, t273);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcc66); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcc67); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcc6d); }
            return c.ret(0x0, 0x70008dcc6f);
        }
        325 => {
            c.w32(12, c.r16(15));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 341; continue; }
            blk = 326; continue;
        }
        326 => {
            c.w64(7, c.r64(6));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(0));
            blk = 327; continue;
        }
        327 => {
            c.w64(3, c.r64(15));
            let r274 = c.r32(3) & 0x7u64;
            c.w32(3, r274);
            c.logic(r274, 32);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 332; continue; }
            blk = 328; continue;
        }
        328 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 340; continue; }
            blk = 329; continue;
        }
        329 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 337; continue; }
            blk = 330; continue;
        }
        330 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 338; continue; }
            blk = 331; continue;
        }
        331 => {
            blk = 408; continue;
        }
        332 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 339; continue; }
            blk = 333; continue;
        }
        333 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 335; continue; }
            blk = 334; continue;
        }
        334 => {
            blk = 408; continue;
        }
        335 => {
            let Some(t275) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dccb8); };
            c.w32(15, t275);
            let Some(t276) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dccbd); };
            c.cmp(t276, c.r32(15), 32);
            if c.cc_ne() { blk = 408; continue; }
            blk = 336; continue;
        }
        336 => {
            let Some(t277) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dccca); };
            c.w32(12, t277);
            let r278 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r278);
            let Some(t279) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dccd2); };
            c.w32(15, t279);
            let r280 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r280);
            let Some(t281) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dccda); };
            c.w32(15, t281);
            blk = 327; continue;
        }
        337 => {
            let Some(t282) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008dcce1); };
            c.w32(15, t282);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r283 = c.add(c.r64(7), c.r64(15), 64);
            c.w64(7, r283);
            let Some(t284) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dccef); };
            c.w32(12, t284);
            let r285 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r285);
            let Some(t286) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dccf7); };
            c.w32(15, t286);
            let r287 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r287);
            let Some(t288) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dccff); };
            c.w32(15, t288);
            blk = 327; continue;
        }
        338 => {
            let Some(t289) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dcd06); };
            c.w32(12, t289);
            let r290 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r290);
            let Some(t291) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dcd0e); };
            c.w32(15, t291);
            let r292 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r292);
            let Some(t293) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dcd16); };
            c.w32(15, t293);
            blk = 327; continue;
        }
        339 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 342; continue;
        }
        340 => {
            c.w32(3, 0x1u64);
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 342; continue;
        }
        341 => {
            c.w64(7, c.r64(6));
            c.w64(15, c.r64(0));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 342; continue;
        }
        342 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 406; continue; }
            blk = 343; continue;
        }
        343 => {
            let r294 = c.add(c.r64(15), 0xbu64, 64);
            c.w64(15, r294);
            blk = 344; continue;
        }
        344 => {
            let r295 = c.r32(12) & 0x8u64;
            c.w32(12, r295);
            c.logic(r295, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t296) = c.ld16(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x70008dcd5c); };
            c.w32(7, t296);
            blk = 347; continue;
        }
        346 => {
            let Some(t297) = c.ld8(c.r64(15).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008dcd63); };
            c.w32(7, t297);
            blk = 347; continue;
        }
        347 => {
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 410; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w64(12, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008dcd7c); };
            c.w64(15, t298);
            let Some(t299) = c.ld32(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dcd83); };
            c.w32(0, t299);
            let r300 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r300);
            let Some(t301) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dcd8b); };
            c.cmp(t301, c.r32(0), 32);
            if c.cc_e() { blk = 415; continue; }
            blk = 349; continue;
        }
        349 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcd9b); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcd9c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcda2); }
            return c.ret(0x0, 0x70008dcda4);
        }
        350 => {
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dcda5); };
            c.w64(0, t302);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcdaf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcdb0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcdb6); }
            return c.ret(0x0, 0x70008dcdb8);
        }
        351 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcdbc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcdbd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcdc3); }
            return c.ret(0x0, 0x70008dcdc5);
        }
        352 => {
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dcdc6); };
            c.w64(0, t303);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dcdd0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dcdd1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dcdd7); }
            return c.ret(0x0, 0x70008dcdd9);
        }
        353 => {
            return RegfileExit::fault(0x70008dcdda);
        }
        354 => {
            return RegfileExit::fault(0x70008dcddb);
        }
        355 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008dcddc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008dcde0); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t304) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008dcde6); };
            c.w64(3, t304);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008dcdf5, 0x70008dcdf0);
        }
        356 => {
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcdf5); };
            c.w64(0, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dcdf9); };
            c.w64(6, t306);
            blk = 1; continue;
        }
        357 => {
            let Some(t307) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008dce02); };
            c.cmp(t307, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t308) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dce11); };
            c.cmp(t308, c.r32(7), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008dce20); }
            let r309 = c.r32(8) | c.r32(9);
            c.w32(8, r309);
            c.logic(r309, 32);
            if c.cc_ne() { blk = 360; continue; }
            blk = 358; continue;
        }
        358 => {
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008dce31, 0x70008dce2c);
        }
        359 => {
            c.w64(7, c.r64(0));
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dce34); };
            c.w64(0, t310);
            blk = 3; continue;
        }
        360 => {
            c.w32(7, 0x36u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dce42); }
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dce43); };
            c.w64(7, t311);
            let Some(t312) = c.ld32(c.r64(7).wrapping_add(0x39b)) else { return RegfileExit::fault(0x70008dce4a); };
            c.w32(7, t312);
            let r313 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r313);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dce53); }
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dce54); };
            c.w64(3, t314);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008dce65, 0x70008dce60);
        }
        361 => {
            return RegfileExit::fault(0x70008dce65);
        }
        362 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008dce66); }
            c.w64(0, c.r64(7));
            return c.call(0x70008a1214, 0x70008dce72, 0x70008dce6d);
        }
        363 => {
            c.w64(7, c.r64(0));
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dce75); };
            c.w64(0, t315);
            blk = 3; continue;
        }
        364 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008dce83); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dce87); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008dce8b); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008dce97, 0x70008dce92);
        }
        365 => {
            c.w64(11, c.r64(0));
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dce9a); };
            c.w64(9, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dce9e); };
            c.w64(7, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dcea2); };
            c.w64(8, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dcea6); };
            c.w64(15, t319);
            blk = 95; continue;
        }
        366 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008dceaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dceb3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008dceb7); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008dcec5, 0x70008dcec0);
        }
        367 => {
            c.w64(11, c.r64(0));
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcec8); };
            c.w64(9, t320);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dcecc); };
            c.w64(7, t321);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dced0); };
            c.w64(8, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dced4); };
            c.w64(15, t323);
            blk = 98; continue;
        }
        368 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008dcedd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008dcee1); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dcee5); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008dcef1, 0x70008dceec);
        }
        369 => {
            c.w64(7, c.r64(0));
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcef4); };
            c.w64(9, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dcef8); };
            c.w64(15, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dcefc); };
            c.w64(8, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dcf00); };
            c.w64(11, t327);
            blk = 107; continue;
        }
        370 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008dcf09); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008dcf0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008dcf11); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008dcf1d, 0x70008dcf18);
        }
        371 => {
            c.w64(7, c.r64(0));
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcf20); };
            c.w64(9, t328);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dcf24); };
            c.w64(15, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dcf28); };
            c.w64(8, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dcf2c); };
            c.w64(11, t331);
            blk = 121; continue;
        }
        372 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008dcf35); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008dcf39); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008dcf3d); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008dcf49, 0x70008dcf44);
        }
        373 => {
            c.w64(7, c.r64(0));
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcf4c); };
            c.w64(9, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dcf50); };
            c.w64(11, t333);
            c.w32(2, 0xbu64);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dcf59); };
            c.w64(8, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dcf5d); };
            c.w64(15, t335);
            blk = 137; continue;
        }
        374 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008dcf66); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008dcf6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008dcf6e); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008dcf7a, 0x70008dcf75);
        }
        375 => {
            c.w64(7, c.r64(0));
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcf7d); };
            c.w64(9, t336);
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dcf81); };
            c.w64(3, t337);
            c.w32(2, 0xbu64);
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dcf8d); };
            c.w64(8, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dcf91); };
            c.w64(15, t339);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dcf95); };
            c.w64(11, t340);
            blk = 150; continue;
        }
        376 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 378; continue; }
            blk = 377; continue;
        }
        377 => {
            let Some(t341) = c.ld64(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dcfa4); };
            c.w64(7, t341);
            blk = 178; continue;
        }
        378 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dcfad); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dcfae); }
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008dcfaf); };
            c.w64(3, t342);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008dcfc0, 0x70008dcfbb);
        }
        379 => {
            let r343 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r343);
            c.w64(7, c.r64(0));
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dcfc5); };
            c.w64(9, t344);
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dcfc9); };
            c.w64(8, t345);
            blk = 181; continue;
        }
        380 => {
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dcfd5); };
            c.w64(11, t346);
            let Some(t347) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dcfdc); };
            c.w64(12, t347);
            c.w64(0, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008dcfe4); }
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dcfe8); };
            c.w64(15, t348);
            let Some(t349) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008dcfef); };
            c.cmp(t349, c.r64(0), 64);
            if c.cc_a() { blk = 383; continue; }
            blk = 381; continue;
        }
        381 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008dcffe, 0x70008dcff9);
        }
        382 => {
            blk = 384; continue;
        }
        383 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008dd000); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 384; continue;
        }
        384 => {
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dd00b); };
            c.w64(8, t350);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008dd012); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008dd016); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dd01d); }
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd024); };
            c.w64(7, t351);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008dd028); }
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd02c); };
            c.w64(9, t352);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd033); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd034); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd03a); }
            return c.ret(0x0, 0x70008dd03c);
        }
        385 => {
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dd03d); };
            c.w64(12, t353);
            let Some(t354) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008dd044); };
            c.w64(15, t354);
            c.w64(3, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008dd04c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008dd050); }
            let Some(t355) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dd054); };
            c.w64(0, t355);
            let Some(t356) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008dd05b); };
            c.cmp(t356, c.r64(3), 64);
            if c.cc_a() { blk = 388; continue; }
            blk = 386; continue;
        }
        386 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008dd06a, 0x70008dd065);
        }
        387 => {
            blk = 389; continue;
        }
        388 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008dd06c); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 389; continue;
        }
        389 => {
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dd077); };
            c.w64(7, t357);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008dd07e); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008dd081); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dd088); }
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd08f); };
            c.w64(7, t358);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x70008dd093); }
            let Some(t359) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008dd097); };
            c.w64(11, t359);
            let Some(t360) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dd09e); };
            c.w32(8, t360);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 391; continue; }
            blk = 390; continue;
        }
        390 => {
            return RegfileExit::fault(0x70008dd0ad);
        }
        391 => {
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd0ae); };
            c.w64(8, t361);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008dd0b2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 393; continue; }
            blk = 392; continue;
        }
        392 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r362 = c.r64(7) & c.r64(11);
            c.w64(7, r362);
            c.logic(r362, 64);
            let Some(t363) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008dd0c4); };
            c.logic(t363 & 0x4u64, 8);
            if c.cc_ne() { blk = 427; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd0ce); };
            c.w64(9, t364);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd0d5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd0d6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd0dc); }
            return c.ret(0x0, 0x70008dd0de);
        }
        394 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008dd0df); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008dd0e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008dd0e7); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008dd0f5, 0x70008dd0f0);
        }
        395 => {
            c.w64(8, c.r64(0));
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd0f8); };
            c.w64(9, t365);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd0fc); };
            c.w64(12, t366);
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dd100); };
            c.w64(3, t367);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd104); };
            c.w64(7, t368);
            blk = 259; continue;
        }
        396 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008dd10d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008dd111); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008dd115); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008dd123, 0x70008dd11e);
        }
        397 => {
            c.w64(8, c.r64(0));
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd126); };
            c.w64(9, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd12a); };
            c.w64(12, t370);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dd12e); };
            c.w64(3, t371);
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd132); };
            c.w64(7, t372);
            blk = 262; continue;
        }
        398 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008dd13b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008dd13f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008dd143); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008dd14f, 0x70008dd14a);
        }
        399 => {
            c.w64(8, c.r64(0));
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd152); };
            c.w64(9, t373);
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd156); };
            c.w64(12, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dd15a); };
            c.w64(7, t375);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dd15e); };
            c.w64(3, t376);
            blk = 271; continue;
        }
        400 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008dd167); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008dd16b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008dd16f); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008dd17b, 0x70008dd176);
        }
        401 => {
            c.w64(8, c.r64(0));
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd17e); };
            c.w64(9, t377);
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd182); };
            c.w64(12, t378);
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dd186); };
            c.w64(7, t379);
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dd18a); };
            c.w64(3, t380);
            blk = 285; continue;
        }
        402 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008dd193); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008dd197); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008dd19b); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70008dd1a7, 0x70008dd1a2);
        }
        403 => {
            c.w64(8, c.r64(0));
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd1aa); };
            c.w64(9, t381);
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dd1ae); };
            c.w64(7, t382);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd1b2); };
            c.w64(12, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd1b6); };
            c.w64(3, t384);
            blk = 301; continue;
        }
        404 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008dd1bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008dd1c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008dd1c7); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70008dd1d3, 0x70008dd1ce);
        }
        405 => {
            c.w64(8, c.r64(0));
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd1d6); };
            c.w64(9, t385);
            let Some(t386) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dd1da); };
            c.w64(2, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd1e1); };
            c.w64(12, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd1e5); };
            c.w64(3, t388);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dd1e9); };
            c.w64(7, t389);
            blk = 315; continue;
        }
        406 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 408; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t390) = c.ld64(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dd1f8); };
            c.w64(15, t390);
            blk = 344; continue;
        }
        408 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dd201); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dd202); }
            c.w32(0, 0x2u64);
            let Some(t391) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008dd208); };
            c.w64(3, t391);
            return c.call(0x700086ed74, 0x70008dd214, 0x70008dd20f);
        }
        409 => {
            let r392 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r392);
            c.w64(7, c.r64(0));
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd219); };
            c.w64(9, t393);
            let Some(t394) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dd21d); };
            c.w64(8, t394);
            let Some(t395) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dd224); };
            c.w64(11, t395);
            blk = 347; continue;
        }
        410 => {
            let Some(t396) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dd230); };
            c.w64(12, t396);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008dd238); }
            let Some(t397) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dd23c); };
            c.cmp(t397, c.r64(15), 64);
            if c.cc_a() { blk = 413; continue; }
            blk = 411; continue;
        }
        411 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008dd24b, 0x70008dd246);
        }
        412 => {
            blk = 414; continue;
        }
        413 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x70008dd24d); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 414; continue;
        }
        414 => {
            let Some(t398) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008dd258); };
            c.w64(7, t398);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008dd25f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008dd262); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dd269); }
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd270); };
            c.w64(7, t399);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008dd274); }
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd278); };
            c.w64(9, t400);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd27f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd280); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd286); }
            return c.ret(0x0, 0x70008dd288);
        }
        415 => {
            let Some(t401) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008dd289); };
            c.w64(0, t401);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008dd290); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008dd294); }
            let Some(t402) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dd298); };
            c.cmp(t402, c.r64(3), 64);
            if c.cc_a() { blk = 418; continue; }
            blk = 416; continue;
        }
        416 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008dd2a7, 0x70008dd2a2);
        }
        417 => {
            blk = 419; continue;
        }
        418 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008dd2a9); }
            c.w64(8, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 419; continue;
        }
        419 => {
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008dd2b3); };
            c.w64(7, t403);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008dd2ba); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008dd2bd); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008dd2c4); }
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dd2cb); };
            c.w64(7, t404);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x70008dd2cf); }
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008dd2d3); };
            c.w64(15, t405);
            let Some(t406) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dd2da); };
            c.w32(8, t406);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 421; continue; }
            blk = 420; continue;
        }
        420 => {
            return RegfileExit::fault(0x70008dd2e9);
        }
        421 => {
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd2ea); };
            c.w64(12, t407);
            if !c.st32(c.r64(15).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x70008dd2ee); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 423; continue; }
            blk = 422; continue;
        }
        422 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r408 = c.r64(7) & c.r64(15);
            c.w64(7, r408);
            c.logic(r408, 64);
            let Some(t409) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008dd300); };
            c.logic(t409 & 0x4u64, 8);
            if c.cc_ne() { blk = 424; continue; }
            blk = 423; continue;
        }
        423 => {
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd306); };
            c.w64(7, t410);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd30d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd30e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd314); }
            return c.ret(0x0, 0x70008dd316);
        }
        424 => {
            c.w32(0, c.r32(0));
            let r411 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r411);
            c.w64(7, 0xfffffffffffc0000u64);
            let r412 = c.r64(7) & c.r64(0);
            c.w64(7, r412);
            c.logic(r412, 64);
            let Some(t413) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008dd326); };
            c.logic(t413 & 0x2u64, 8);
            if c.cc_e() { blk = 423; continue; }
            blk = 425; continue;
        }
        425 => {
            c.w64(8, c.r64(15).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dd330); }
            c.w64(7, c.r64(15));
            return c.call(0x70007f0e54, 0x70008dd339, 0x70008dd334);
        }
        426 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008dd339); }
            blk = 423; continue;
        }
        427 => {
            c.w32(0, c.r32(0));
            let r414 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r414);
            c.w64(7, 0xfffffffffffc0000u64);
            let r415 = c.r64(7) & c.r64(0);
            c.w64(7, r415);
            c.logic(r415, 64);
            let Some(t416) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008dd34b); };
            c.logic(t416 & 0x2u64, 8);
            if c.cc_e() { blk = 393; continue; }
            blk = 428; continue;
        }
        428 => {
            c.w64(9, c.r64(11).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dd359); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008dd35a); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008dd367, 0x70008dd362);
        }
        429 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008dd367); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008dd369); }
            blk = 393; continue;
        }
        430 => {
            return RegfileExit::jump(0x70008dd370);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008dd374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008dd374); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dd378); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dd379); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dd37a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008dd37f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd385); };
            c.w64(2, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dd38b); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dd396); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dd39c); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008dd3a4); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dd3ad); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dd3b3); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd3ba); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x70008dd3c1);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd3c4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd3cb); }
            return c.ret(0x0, 0x70008dd3cd);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008dd3ce); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008dd3d2); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008dd3e0, 0x70008dd3db);
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd3e0); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd3eb); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dd3ec); };
            c.w64(2, t14);
            let Some(t15) = c.ld32(c.r64(2).wrapping_add(0x39f)) else { return RegfileExit::fault(0x70008dd3f3); };
            c.w32(2, t15);
            let r16 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r16);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd3fc); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xaa0)) else { return RegfileExit::fault(0x70008dd3fd); };
            c.w64(2, t17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd404); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dd405); };
            c.w64(3, t18);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008dd416, 0x70008dd411);
        }
        12 => {
            return RegfileExit::fault(0x70008dd416);
        }
        13 => {
            return RegfileExit::jump(0x70008dd418);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008dd434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008dd434); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dd438); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dd439); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dd43a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008dd43f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd445); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dd44b); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dd455); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dd45b); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008dd463); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dd46c); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd473); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x70008dd47a);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd47d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd484); }
            return c.ret(0x0, 0x70008dd486);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008dd487); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008dd48b); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008dd499, 0x70008dd494);
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd499); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd4a4); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dd4a5); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_add(0x3a3)) else { return RegfileExit::fault(0x70008dd4ac); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd4b5); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xaa0)) else { return RegfileExit::fault(0x70008dd4b6); };
            c.w64(2, t15);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd4bd); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dd4be); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008dd4cf, 0x70008dd4ca);
        }
        12 => {
            return RegfileExit::fault(0x70008dd4cf);
        }
        13 => {
            return RegfileExit::jump(0x70008dd4d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008dd4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008dd4f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dd4f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dd4f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dd4fa); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008dd4ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd505); };
            c.w64(2, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dd50b); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dd516); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dd51c); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008dd524); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd52d); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x27e8)) else { return RegfileExit::fault(0x70008dd52e); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008dd53f, 0x70008dd53a);
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dd53f); };
            c.w64(1, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd546); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x8, 0x70008dd54d);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd550); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd557); }
            return c.ret(0x0, 0x70008dd559);
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008dd55a); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008dd55e); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008dd56c, 0x70008dd567);
        }
        11 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd56c); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        12 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd577); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dd578); };
            c.w64(2, t13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x3a7)) else { return RegfileExit::fault(0x70008dd57f); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd588); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xaa0)) else { return RegfileExit::fault(0x70008dd589); };
            c.w64(2, t16);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd590); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dd591); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008dd5a2, 0x70008dd59d);
        }
        13 => {
            return RegfileExit::fault(0x70008dd5a2);
        }
        14 => {
            return RegfileExit::jump(0x70008dd5a4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008dd5b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008dd5b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dd5b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dd5b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dd5ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008dd5bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd5c5); };
            c.w64(0, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dd5cb); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dd5d5); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dd5db); };
            c.cmp(t7, 0x411u64, 16);
            if c.cc_e() { blk = 2; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008dd5e3); };
            c.cmp(t8, c.r32(2), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dd5ec); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd5f3); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x8, 0x70008dd5fa);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd5fd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd604); }
            return c.ret(0x0, 0x70008dd606);
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008dd607); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008dd60b); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008dd619, 0x70008dd614);
        }
        10 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd619); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        11 => {
            c.w32(2, 0xc6u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd624); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dd625); };
            c.w64(2, t12);
            let Some(t13) = c.ld32(c.r64(2).wrapping_add(0x3ab)) else { return RegfileExit::fault(0x70008dd62c); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd635); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xaa0)) else { return RegfileExit::fault(0x70008dd636); };
            c.w64(2, t15);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008dd63d); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dd63e); };
            c.w64(3, t16);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008dd64f, 0x70008dd64a);
        }
        12 => {
            return RegfileExit::fault(0x70008dd64f);
        }
        13 => {
            return RegfileExit::jump(0x70008dd654);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008dd674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008dd674); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008dd678); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008dd679); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dd67a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008dd67f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008dd683); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd68d); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dd699); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dd69f); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 68; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dd6ab); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008dd6b1); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 66; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008dd6bb); };
            c.w64(1, t9);
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 5; continue;
        }
        5 => {
            let r10 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r10);
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008dd6ce); };
            c.w64(7, t11);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008dd6d2); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x70008dd6e4, 0x70008dd6df);
        }
        8 => {
            c.w64(7, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd6e7); };
            c.w64(0, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd6eb); };
            c.w64(6, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd6ef); };
            c.w64(1, t14);
            blk = 9; continue;
        }
        9 => {
            c.w32(8, c.r32(7));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dd708); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 13; continue;
        }
        12 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ge() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(12, c.r32(1));
            let r15 = c.add(c.r32(8), c.r32(12), 32);
            c.w32(8, r15);
            if c.cc_o() { blk = 49; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dd75e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 37; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 21; continue;
        }
        20 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 37; continue; }
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dd781); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dd78c); }
            blk = 27; continue;
        }
        24 => {
            c.w64(1, c.r64(7));
            let r16 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r16);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(1) as u32 as i32 as i64));
            blk = 27; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            let r17 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r17);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dd7aa); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 27; continue;
        }
        27 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 30; continue;
        }
        29 => {
            c.cmp(c.r32(1), c.r32(7), 32);
            if c.cc_le() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        31 => {
            c.w32(1, 0x1u64);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(3, c.r64(7));
            return c.call(0x7000870914, 0x70008dd7d9, 0x70008dd7d4);
        }
        34 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dd7d9); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008dd7e0); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            return c.ret(0x10, 0x70008dd7e7);
        }
        36 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008dd7ea); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008dd7f1); }
            return c.ret(0x0, 0x70008dd7f3);
        }
        37 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dd7f4); };
            c.w64(0, t19);
            blk = 34; continue;
        }
        38 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008dd7ff); };
            c.w64(3, t20);
            return c.call(0x700086ed74, 0x70008dd80b, 0x70008dd806);
        }
        39 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd80b); };
            c.w64(6, t21);
            blk = 1; continue;
        }
        40 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dd814); };
            c.w64(7, t22);
            let Some(t23) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008dd81b); };
            c.w64(8, t23);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008dd822); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dd826); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dd82d); };
            c.cmp(t25, c.r64(9), 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008dd83c, 0x70008dd837);
        }
        42 => {
            c.w64(7, c.r64(0));
            let Some(t26) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd83f); };
            c.w64(0, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd843); };
            c.w64(6, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd847); };
            c.w64(1, t28);
            blk = 44; continue;
        }
        43 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008dd84d); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 44; continue;
        }
        44 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let r29 = c.r64(10) | 0x1u64;
            c.w64(10, r29);
            c.logic(r29, 64);
            blk = 47; continue;
        }
        47 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 48; continue;
        }
        48 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dd87b); };
            c.w64(1, t30);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008dd882); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dd885); }
            c.w64(1, c.r64(7));
            blk = 6; continue;
        }
        49 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dd898); }
            blk = 52; continue;
        }
        51 => {
            let r31 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r31);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.movsd_rm(1, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dd8af); }
            blk = 55; continue;
        }
        54 => {
            c.w64(7, c.r64(1));
            let r32 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r32);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            blk = 55; continue;
        }
        55 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x70008dd8d3); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 61; continue; }
            blk = 56; continue;
        }
        56 => {
            if c.cc_ne() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(8, c.r64(7));
            let r33 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r33);
            if c.cc_o() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 17; continue;
        }
        61 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dd904); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008dd90b); };
            c.w64(8, t35);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dd912); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dd919); };
            c.cmp(t37, c.r64(9), 64);
            if c.cc_a() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008dd928, 0x70008dd923);
        }
        63 => {
            c.w64(7, c.r64(0));
            let Some(t38) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008dd92b); };
            c.w64(0, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dd92f); };
            c.w64(6, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dd933); };
            c.w64(1, t40);
            blk = 65; continue;
        }
        64 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008dd939); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 65; continue;
        }
        65 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dd940); };
            c.w64(8, t41);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008dd947); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x70008dd94b); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dd950); }
            blk = 17; continue;
        }
        66 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008dd95f); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dd960); };
            c.w64(1, t42);
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008dd967); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008dd970); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dd971); };
            c.w64(3, t45);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008dd982, 0x70008dd97d);
        }
        67 => {
            return RegfileExit::fault(0x70008dd982);
        }
        68 => {
            c.w32(1, 0x80u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008dd988); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dd989); };
            c.w64(1, t46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008dd990); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008dd999); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dd99a); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dd99b); };
            c.w64(3, t49);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008dd9ac, 0x70008dd9a7);
        }
        69 => {
            return RegfileExit::fault(0x70008dd9ac);
        }
        70 => {
            return RegfileExit::jump(0x70008dd9b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008dd9b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008dd9b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008dd9b8); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008dd9c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008dd9cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008dd9d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008dd9d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008dd9d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008dd9dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008dd9e1); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 399; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dd9ef); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dd9f6); };
            c.w32(11, t3);
            c.cmp(c.r16(11), 0x835u64, 16);
            if c.cc_ne() { blk = 172; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008dda07); };
            c.w32(9, t4);
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(11));
            blk = 5; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dda19); };
            c.cmp(t5, c.r32(7), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w32(15, 0x1u64);
            blk = 5; continue;
        }
        5 => {
            c.w32(12, 0x1u64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008dda34); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(9, c.r64(11));
            blk = 9; continue;
        }
        8 => {
            c.w64(9, c.r64(12));
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dda52); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008dda59); };
            c.logic(t8 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008dda60); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            let Some(t11) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dda67); };
            c.w32(15, t11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008dda6b); };
            c.cmp(t12, c.r32(15), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008dda74); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(9));
            blk = 18; continue;
        }
        14 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dda7f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008dda80); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008dda81); };
            c.w64(9, t14);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008dda88); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x70008dda8a); };
            c.w64(3, t15);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008dda9b, 0x70008dda96);
        }
        15 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008dda9b); };
            c.w64(7, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008dda9f); };
            c.w64(1, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ddaa3); };
            c.w64(6, t18);
            c.w32(12, 0x1u64);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ddaad); };
            c.w64(2, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ddab1); };
            c.w64(11, t20);
            blk = 18; continue;
        }
        16 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ddab7); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008ddab8); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ddab9); };
            c.w64(9, t21);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ddac0); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x70008ddac2); };
            c.w64(3, t22);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008ddad3, 0x70008ddace);
        }
        17 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ddad3); };
            c.w64(7, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ddad7); };
            c.w64(1, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ddadb); };
            c.w64(6, t25);
            c.w32(12, 0x1u64);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ddae5); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ddae9); };
            c.w64(11, t27);
            blk = 18; continue;
        }
        18 => {
            let Some(t28) = c.ld8(c.r64(0).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ddaed); };
            c.w32(9, t28);
            let r29 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r29);
            let r30 = c.r32(9) & 0x3fu64;
            c.w32(9, r30);
            c.logic(r30, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008ddafa); }
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_l() { blk = 37; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_l() { blk = 25; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 566; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 566; continue;
        }
        29 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 566; continue;
        }
        34 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 566; continue;
        }
        37 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_l() { blk = 47; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 566; continue;
        }
        43 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 46; continue;
        }
        46 => {
            blk = 566; continue;
        }
        47 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_l() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 566; continue;
        }
        52 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            blk = 566; continue;
        }
        55 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 77; continue;
        }
        56 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 77; continue;
        }
        57 => {
            c.w32(9, 0x1u64);
            blk = 77; continue;
        }
        58 => {
            c.w32(9, 0x1u64);
            blk = 77; continue;
        }
        59 => {
            c.w32(9, 0x2u64);
            blk = 77; continue;
        }
        60 => {
            c.w32(9, 0x2u64);
            blk = 77; continue;
        }
        61 => {
            c.w32(9, 0x2u64);
            blk = 77; continue;
        }
        62 => {
            c.w32(9, 0x3u64);
            blk = 77; continue;
        }
        63 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 77; continue;
        }
        64 => {
            c.w32(9, 0x3u64);
            blk = 77; continue;
        }
        65 => {
            c.w32(9, 0x3u64);
            blk = 77; continue;
        }
        66 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 77; continue;
        }
        67 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 77; continue;
        }
        68 => {
            c.w32(9, 0x1u64);
            blk = 77; continue;
        }
        69 => {
            c.w32(9, 0x1u64);
            blk = 77; continue;
        }
        70 => {
            c.w32(9, 0x2u64);
            blk = 77; continue;
        }
        71 => {
            c.w32(9, 0x2u64);
            blk = 77; continue;
        }
        72 => {
            c.w32(9, 0x2u64);
            blk = 77; continue;
        }
        73 => {
            c.w32(9, 0x3u64);
            blk = 77; continue;
        }
        74 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 77; continue;
        }
        75 => {
            c.w32(9, 0x3u64);
            blk = 77; continue;
        }
        76 => {
            c.w32(9, 0x3u64);
            blk = 77; continue;
        }
        77 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008ddcd3); }
            c.w64(15, c.r64(2));
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ddcda); };
            c.cmp(t31, c.r32(15), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 78; continue;
        }
        78 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(3, c.r64(15));
            blk = 82; continue;
        }
        80 => {
            c.w64(0, c.r64(15));
            return c.call(0x7000899a14, 0x70008ddcfa, 0x70008ddcf5);
        }
        81 => {
            c.w64(3, c.r64(0));
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ddcfd); };
            c.w64(1, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ddd01); };
            c.w64(6, t33);
            c.w32(12, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ddd0b); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ddd0f); };
            c.w64(15, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ddd13); };
            c.w64(11, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ddd17); };
            c.w64(9, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ddd1b); };
            c.w64(7, t38);
            blk = 82; continue;
        }
        82 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_l() { blk = 581; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            blk = 92; continue;
        }
        85 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ddd35); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 581; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(10, 0x433fffffffffffffu64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 581; continue; }
            blk = 87; continue;
        }
        87 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 90; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(3, cv);
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_s() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(10, 0x8000000000000000u64);
            let r39 = c.r64(3) | c.r64(10);
            c.w64(3, r39);
            c.logic(r39, 64);
            blk = 90; continue;
        }
        90 => {
            blk = 92; continue;
        }
        91 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 92; continue;
        }
        92 => {
            c.w64(8, c.r64(1));
            c.w32(2, 0x1u64);
            c.w64(1, c.r64(9));
            if let Some(sr) = c.shl(c.r64(2), c.r8(1), 64) { c.w64(2, sr); }
            let r40 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r40);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008dddac); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008dddb0); }
            c.logic(c.r64(2) & c.r64(3), 64);
            if c.cc_ne() { blk = 584; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(1, c.r64(7));
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dddc0); };
            c.cmp(t41, c.r32(1), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 94; continue;
        }
        94 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(7, c.r64(1));
            blk = 98; continue;
        }
        96 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008ddddf, 0x70008dddda);
        }
        97 => {
            c.w64(7, c.r64(0));
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ddde2); };
            c.w64(8, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ddde6); };
            c.w64(6, t43);
            c.w32(12, 0x1u64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dddf0); };
            c.w64(0, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dddf4); };
            c.w64(15, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dddf8); };
            c.w64(3, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dddfc); };
            c.w64(11, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dde00); };
            c.w64(9, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dde04); };
            c.w64(2, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008dde08); };
            c.w64(1, t50);
            blk = 98; continue;
        }
        98 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 101; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 568; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 108; continue;
        }
        101 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008dde23); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 568; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t51) = c.ld64(0x70008ddd4au64) else { return RegfileExit::fault(0x70008dde36); };
            c.w64(10, t51);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 568; continue; }
            blk = 103; continue;
        }
        103 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_ns() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t52) = c.ld64(0x70008ddd6du64) else { return RegfileExit::fault(0x70008dde56); };
            c.w64(10, t52);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(7, cv);
            c.logic(c.r64(7) & c.r64(7), 64);
            if c.cc_s() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t53) = c.ld64(0x70008ddd8bu64) else { return RegfileExit::fault(0x70008dde71); };
            c.w64(10, t53);
            let r54 = c.r64(7) | c.r64(10);
            c.w64(7, r54);
            c.logic(r54, 64);
            blk = 106; continue;
        }
        106 => {
            blk = 108; continue;
        }
        107 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 108; continue;
        }
        108 => {
            let Some(t55) = c.ld32(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008dde7f); };
            c.w32(9, t55);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008dde83); }
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_ne() { blk = 170; continue; }
            blk = 109; continue;
        }
        109 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_e() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t56) = c.ld64(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dde9d); };
            c.w64(9, t56);
            blk = 128; continue;
        }
        112 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ddea6); }
            c.w64(0, c.r64(12));
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2b18)) else { return RegfileExit::fault(0x70008ddeab); };
            c.w64(3, t57);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x70008ddeba, 0x70008ddeb5);
        }
        113 => {
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ddeba); };
            c.cmp(t58, c.r32(0), 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 114; continue;
        }
        114 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ddecb); };
            c.w64(6, t59);
            return c.call(0x7000899a14, 0x70008dded4, 0x70008ddecf);
        }
        116 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 119; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 125; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 127; continue;
        }
        119 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ddee5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 125; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t60) = c.ld64(0x70008ddd4au64) else { return RegfileExit::fault(0x70008ddef4); };
            c.w64(10, t60);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 124; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t61) = c.ld64(0x70008ddd6du64) else { return RegfileExit::fault(0x70008ddf10); };
            c.w64(10, t61);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_s() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t62) = c.ld64(0x70008ddd8bu64) else { return RegfileExit::fault(0x70008ddf2b); };
            c.w64(10, t62);
            let r63 = c.r64(9) | c.r64(10);
            c.w64(9, r63);
            c.logic(r63, 64);
            blk = 124; continue;
        }
        124 => {
            blk = 127; continue;
        }
        125 => {
            return RegfileExit::fault(0x70008ddf37);
        }
        126 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 127; continue;
        }
        127 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ddf3b); };
            c.w64(1, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ddf3f); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ddf43); };
            c.w64(6, t66);
            c.w32(12, 0x1u64);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ddf4d); };
            c.w64(0, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ddf51); };
            c.w64(15, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ddf55); };
            c.w64(3, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ddf59); };
            c.w64(11, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ddf5d); };
            c.w64(7, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ddf61); };
            c.w64(2, t72);
            blk = 128; continue;
        }
        128 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ddf65); };
            c.cmp(t73, c.r32(1), 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(15, 0x100000000u64);
            c.cmp(c.r64(15), c.r64(7), 64);
            if c.cc_b() { blk = 569; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(15, 0x1fffffffffffffu64);
            c.w64(2, c.r64(1));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ddf8e); };
            c.w64(1, t74);
            if let Some(sr) = c.shr(c.r64(15), c.r8(1), 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(7), 64);
            if c.cc_b() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ddf9a); };
            c.w64(1, t75);
            c.w64(15, c.r64(7));
            if let Some(sr) = c.shl(c.r64(15), c.r8(1), 64) { c.w64(15, sr); }
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            let r76 = c.sub(c.r64(9), c.r64(15), 64);
            c.w64(9, r76);
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_b() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(9, c.r64(15));
            c.w64(2, c.r64(7));
            blk = 139; continue;
        }
        134 => {
            c.w32(9, 0x1b4u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ddfbf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ddfc1); }
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008ddfc2); };
            c.w64(3, t77);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ddfd3, 0x70008ddfce);
        }
        135 => {
            return RegfileExit::fault(0x70008ddfd3);
        }
        136 => {
            c.logic(c.r64(9) & c.r64(2), 64);
            if c.cc_ne() { blk = 583; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_b() { blk = 581; continue; }
            blk = 138; continue;
        }
        138 => {
            let r78 = c.sub(c.r64(9), c.r64(3), 64);
            c.w64(9, r78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ddfe9); };
            c.w64(1, t79);
            c.w64(2, c.r64(9));
            if let Some(sr) = c.shr(c.r64(2), c.r8(1), 64) { c.w64(2, sr); }
            let Some(t80) = c.ld64(0x70008ddf70u64) else { return RegfileExit::fault(0x70008ddff3); };
            c.w64(1, t80);
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_b() { blk = 581; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008de003); }
            c.w64(15, c.r64(0));
            let Some(t81) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008de00a); };
            c.logic(t81 & 0x200000u64, 32);
            if c.cc_ne() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008de014); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008de01b); };
            c.w64(1, t83);
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008de022); };
            c.w64(0, t84);
            blk = 151; continue;
        }
        141 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008de02e); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008de035); };
            c.w64(1, t86);
            c.w64(6, c.r64(1).wrapping_add(0x4c));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008de03c); };
            c.w64(7, t87);
            let Some(t88) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008de043); };
            c.cmp(t88, c.r64(6), 64);
            if c.cc_be() { blk = 570; continue; }
            blk = 142; continue;
        }
        142 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008de04c); }
            let r89 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r89);
            blk = 143; continue;
        }
        143 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008de053); };
            c.w64(6, t90);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008de05a); }
            c.w32(6, 0x22u64);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008de062); }
            let Some(t91) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de065); };
            c.w32(6, t91);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_be() { blk = 572; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70008de07a); }
            let Some(t92) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de07d); };
            c.w32(11, t92);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 573; continue; }
            blk = 145; continue;
        }
        145 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x70008de091); }
            let Some(t93) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de094); };
            c.w32(11, t93);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_be() { blk = 574; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w32(11, 0x8u64);
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008de0ae); }
            let Some(t94) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de0b2); };
            c.w32(11, t94);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_be() { blk = 575; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w32(11, 0x2u64);
            if !c.st32(c.r64(1).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x70008de0cc); }
            let Some(t95) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de0d0); };
            c.w32(11, t95);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 576; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008de0e7); }
            c.w64(11, c.r64(1).wrapping_add(0x4b));
            c.w64(6, c.r64(1).wrapping_add(0x1b));
            c.cmp(c.r64(6), c.r64(11), 64);
            if c.cc_e() { blk = 151; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008de0f8); };
            c.w64(2, t96);
            blk = 150; continue;
        }
        150 => {
            if !c.st32(c.r64(6), c.r32(2)) { return RegfileExit::fault(0x70008de0ff); }
            let r97 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r97);
            c.cmp(c.r64(11), c.r64(6), 64);
            if c.cc_ne() { blk = 150; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t98) = c.ld8(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de10a); };
            c.w32(11, t98);
            c.w64(2, c.r64(11).wrapping_mul(4));
            let Some(t99) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008de117); };
            c.w64(11, t99);
            c.w64(6, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t100) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008de11e); };
            c.cmp(t100, c.r64(6), 64);
            if c.cc_be() { blk = 577; continue; }
            blk = 152; continue;
        }
        152 => {
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x70008de127); }
            let r101 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r101);
            blk = 153; continue;
        }
        153 => {
            c.w64(0, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008de132); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008de136); }
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008de13a); };
            c.w64(2, t102);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008de141); }
            let Some(t103) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008de145); };
            c.w32(2, t103);
            c.w64(1, c.r64(2));
            let r104 = c.r32(1) & 0xe0000000u64;
            c.w32(1, r104);
            c.logic(r104, 32);
            if c.cc_ne() { blk = 158; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_e() { blk = 165; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008de15a); };
            c.w64(12, t105);
            blk = 156; continue;
        }
        156 => {
            let r106 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r106);
            if !c.st32(c.r64(0).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008de165); }
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_ne() { blk = 156; continue; }
            blk = 157; continue;
        }
        157 => {
            blk = 165; continue;
        }
        158 => {
            let r107 = c.sub(c.r32(2), 0x20000000u64, 32);
            c.w32(2, r107);
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008de177); }
            let Some(t108) = c.ld8(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008de17b); };
            c.w32(2, t108);
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_e() { blk = 161; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008de18d); };
            c.w64(6, t109);
            blk = 160; continue;
        }
        160 => {
            let r110 = c.sub(c.r64(0), 0x4u64, 64);
            c.w64(0, r110);
            if !c.st32(c.r64(0).wrapping_add(c.r64(11)), c.r32(6)) { return RegfileExit::fault(0x70008de198); }
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 160; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_e() { blk = 164; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008de1a7); };
            c.w64(0, t111);
            blk = 163; continue;
        }
        163 => {
            let r112 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r112);
            if !c.st32(c.r64(2).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008de1b2); }
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_ne() { blk = 163; continue; }
            blk = 164; continue;
        }
        164 => {
            let r113 = c.add(c.r32(1), 0xe0000000u64, 32);
            c.w32(1, r113);
            if c.cc_e() { blk = 579; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x3d8)) else { return RegfileExit::fault(0x70008de1c8); };
            c.w64(12, t114);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008de1cf); }
            c.w64(1, c.r64(8));
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008de1d6); }
            if !c.st64(c.r64(11).wrapping_add(0xf), c.r64(3)) { return RegfileExit::fault(0x70008de1da); }
            if !c.st64(c.r64(11).wrapping_add(0x17), c.r64(9)) { return RegfileExit::fault(0x70008de1de); }
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008de1e2); };
            c.w64(2, t115);
            if !c.st64(c.r64(11).wrapping_add(0x1f), c.r64(2)) { return RegfileExit::fault(0x70008de1e6); }
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008de1ea); };
            c.w32(9, t116);
            let r117 = c.r32(9) & 0xfffffffffffffffeu64;
            c.w32(9, r117);
            c.logic(r117, 32);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008de1f2); };
            c.w64(12, t118);
            let r119 = c.r32(12) | c.r32(9);
            c.w32(12, r119);
            c.logic(r119, 32);
            if !c.st32(c.r64(11).wrapping_add(0x33), c.r32(12)) { return RegfileExit::fault(0x70008de1f9); }
            let Some(t120) = c.ld32(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008de1fd); };
            c.w32(9, t120);
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 168; continue;
        }
        167 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            blk = 168; continue;
        }
        168 => {
            let r121 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r121);
            let r122 = c.r32(12) & 0xfffffffffffffffdu64;
            c.w32(12, r122);
            c.logic(r122, 32);
            let r123 = c.r32(9) | c.r32(12);
            c.w32(9, r123);
            c.logic(r123, 32);
            if !c.st32(c.r64(11).wrapping_add(0x33), c.r32(9)) { return RegfileExit::fault(0x70008de222); }
            let Some(t124) = c.ld64(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008de226); };
            c.w64(9, t124);
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x2f), c.r32(12)) { return RegfileExit::fault(0x70008de22d); }
            let r125 = c.add(c.r64(9), c.r64(3), 64);
            c.w64(9, r125);
            if !c.st64(c.r64(11).wrapping_add(0x27), c.r64(9)) { return RegfileExit::fault(0x70008de234); }
            if !c.st32(c.r64(11).wrapping_add(0x37), c.r32(12)) { return RegfileExit::fault(0x70008de238); }
            if !c.st32(c.r64(11).wrapping_add(0x3b), c.r32(12)) { return RegfileExit::fault(0x70008de23c); }
            if !c.st32(c.r64(11).wrapping_add(0x3f), c.r32(12)) { return RegfileExit::fault(0x70008de240); }
            if !c.st32(c.r64(11).wrapping_add(0x43), c.r32(12)) { return RegfileExit::fault(0x70008de244); }
            c.w64(0, c.r64(11));
            blk = 169; continue;
        }
        169 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008de24e); }
            return c.ret(0x0, 0x70008de24f);
        }
        170 => {
            c.w32(9, 0x66u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008de256); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008de258); };
            c.w64(9, t126);
            let Some(t127) = c.ld32(c.r64(9).wrapping_add(0x3b3)) else { return RegfileExit::fault(0x70008de25f); };
            c.w32(9, t127);
            let r128 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r128);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008de269); }
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008de26b); };
            c.w64(3, t129);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008de27c, 0x70008de277);
        }
        171 => {
            return RegfileExit::fault(0x70008de27c);
        }
        172 => {
            c.w32(11, c.r16(11));
            c.cmp(c.r32(11), 0x82eu64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t130) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de28e); };
            c.w32(9, t130);
            let r131 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r131);
            let Some(t132) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008de295); };
            c.w32(9, t132);
            let r133 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r133);
            let Some(t134) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de29c); };
            c.w32(11, t134);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x28u64, 64);
            if c.cc_a() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            return RegfileExit::fault(0x70008de2ac);
        }
        175 => {
            let Some(t135) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008de2ad); };
            c.w32(11, t135);
            let r136 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r136);
            let Some(t137) = c.ld32(c.r64(9).wrapping_add(0xa7)) else { return RegfileExit::fault(0x70008de2b4); };
            c.w32(9, t137);
            let r138 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r138);
            let Some(t139) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008de2be); };
            c.w32(12, t139);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008de2c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008de2c6); }
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 177; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t140) = c.ld64(c.r64(1).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008de2d0); };
            c.w64(15, t140);
            blk = 178; continue;
        }
        177 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 178; continue;
        }
        178 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008de2d9); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 228; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008de2e7); };
            c.w64(3, t141);
            c.w64(0, c.r64(11));
            c.w64(12, c.r64(3));
            return c.call(0x7000870914, 0x70008de2f9, 0x70008de2f4);
        }
        180 => {
            let Some(t142) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008de2f9); };
            c.cmp(t142, c.r32(0), 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 181; continue;
        }
        181 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 587; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t143) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de30e); };
            c.w32(9, t143);
            let r144 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r144);
            let Some(t145) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008de315); };
            c.cmp(t145, 0xbau64, 16);
            if c.cc_b() { blk = 587; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008de322); };
            c.w64(3, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de329); };
            c.w64(6, t147);
            return c.call(0x7000870914, 0x70008de332, 0x70008de32d);
        }
        184 => {
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008de332); };
            c.cmp(t148, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t149) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008de341); };
            c.cmp(t149, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let r150 = c.r32(9) | c.r32(11);
            c.w32(9, r150);
            c.logic(r150, 32);
            if c.cc_ne() { blk = 190; continue; }
            blk = 185; continue;
        }
        185 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t151) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de359); };
            c.w32(9, t151);
            let r152 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r152);
            let Some(t153) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008de360); };
            c.logic(t153 & 0x40u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008de367); }
            blk = 190; continue;
        }
        188 => {
            c.w32(9, 0x26eu64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008de373); }
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008de375); };
            c.w64(3, t154);
            c.w32(0, 0x1u64);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de381); };
            c.w64(6, t155);
            return c.call(0x700086ed74, 0x70008de38a, 0x70008de385);
        }
        189 => {
            return RegfileExit::fault(0x70008de38a);
        }
        190 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008de38b); };
            c.w64(9, t156);
            let Some(t157) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008de38f); };
            c.logic(t157 & 0x4u64, 8);
            if c.cc_ne() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008de396); };
            c.w64(15, t158);
            blk = 193; continue;
        }
        192 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 193; continue;
        }
        193 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008de39f); };
            c.w64(9, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008de3a3); };
            c.w64(3, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008de3a7); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de3ab); };
            c.w64(6, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008de3af); };
            c.w64(0, t163);
            blk = 228; continue;
        }
        194 => {
            let r164 = c.sub(c.r32(11), 0xbau64, 32);
            c.w32(11, r164);
            c.cmp(c.r32(11), 0x78eu64, 32);
            if c.cc_a() { blk = 399; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008de3cc); };
            c.w64(3, t165);
            c.w64(0, c.r64(1));
            c.w64(9, c.r64(3));
            return c.call(0x7000870914, 0x70008de3de, 0x70008de3d9);
        }
        196 => {
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008de3de); };
            c.cmp(t166, c.r32(0), 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t167) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008de3eb); };
            c.cmp(t167, c.r32(0), 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 198; continue;
        }
        198 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t168) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de400); };
            c.w32(9, t168);
            let r169 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r169);
            let Some(t170) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008de407); };
            c.logic(t170 & 0x2u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w64(3, c.r64(0));
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008de415); };
            c.w64(0, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de419); };
            c.w64(6, t172);
            return c.call(0x7000845bd4, 0x70008de422, 0x70008de41d);
        }
        201 => {
            let Some(t173) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008de422); };
            c.w32(9, t173);
            let r174 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r174);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 207; continue;
        }
        203 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008de437); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 206; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t175) = c.ld64(0x70008ddd6du64) else { return RegfileExit::fault(0x70008de447); };
            c.w64(10, t175);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_s() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t176) = c.ld64(0x70008ddd8bu64) else { return RegfileExit::fault(0x70008de462); };
            c.w64(10, t176);
            let r177 = c.r64(15) | c.r64(10);
            c.w64(15, r177);
            c.logic(r177, 64);
            blk = 206; continue;
        }
        206 => {
            c.w64(9, c.r64(15));
            blk = 207; continue;
        }
        207 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de46f); };
            c.w64(6, t178);
            let Some(t179) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de473); };
            c.w32(11, t179);
            let r180 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r180);
            let Some(t181) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008de47a); };
            c.w32(11, t181);
            let r182 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r182);
            let Some(t183) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de481); };
            c.w32(12, t183);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x28u64, 64);
            if c.cc_a() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            return RegfileExit::fault(0x70008de491);
        }
        209 => {
            let Some(t184) = c.ld32(c.r64(11).wrapping_add(0xa7)) else { return RegfileExit::fault(0x70008de492); };
            c.w32(11, t184);
            let r185 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r185);
            blk = 227; continue;
        }
        210 => {
            c.w32(9, 0x90u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008de4a7); }
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008de4a9); };
            c.w64(3, t186);
            c.w32(0, 0x1u64);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de4b5); };
            c.w64(6, t187);
            return c.call(0x700086ed74, 0x70008de4be, 0x70008de4b9);
        }
        211 => {
            return RegfileExit::fault(0x70008de4be);
        }
        212 => {
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008de4bf); };
            c.w64(3, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008de4c6); };
            c.w64(0, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de4ca); };
            c.w64(6, t190);
            return c.call(0x7000870914, 0x70008de4d3, 0x70008de4ce);
        }
        213 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de4d7); };
            c.w64(6, t191);
            return c.call(0x70008a17f4, 0x70008de4e0, 0x70008de4db);
        }
        215 => {
            blk = 218; continue;
        }
        216 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ge() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 218; continue;
        }
        218 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 224; continue;
        }
        220 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008de4f5); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 223; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t192) = c.ld64(0x70008ddd6du64) else { return RegfileExit::fault(0x70008de504); };
            c.w64(10, t192);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(15, cv);
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_s() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t193) = c.ld64(0x70008ddd8bu64) else { return RegfileExit::fault(0x70008de51f); };
            c.w64(10, t193);
            let r194 = c.r64(15) | c.r64(10);
            c.w64(15, r194);
            c.logic(r194, 64);
            blk = 223; continue;
        }
        223 => {
            c.w64(9, c.r64(15));
            blk = 224; continue;
        }
        224 => {
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de52c); };
            c.w64(6, t195);
            let Some(t196) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de530); };
            c.w32(11, t196);
            let r197 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r197);
            let Some(t198) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008de537); };
            c.w32(11, t198);
            let r199 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r199);
            let Some(t200) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de53e); };
            c.w32(12, t200);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x28u64, 64);
            if c.cc_a() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            return RegfileExit::fault(0x70008de54e);
        }
        226 => {
            let Some(t201) = c.ld32(c.r64(11).wrapping_add(0xa7)) else { return RegfileExit::fault(0x70008de54f); };
            c.w32(11, t201);
            let r202 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008de559); };
            c.w64(0, t203);
            blk = 227; continue;
        }
        227 => {
            c.w64(15, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(1, c.r64(0));
            blk = 228; continue;
        }
        228 => {
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008de566); };
            c.w64(11, t204);
            let Some(t205) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de56a); };
            c.w32(12, t205);
            let r206 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r206);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x70008de571); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008de575); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008de579); }
            let Some(t207) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008de57d); };
            c.logic(t207 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 232; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t208) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008de585); };
            c.w32(12, t208);
            let r209 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r209);
            let Some(t210) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de58c); };
            c.w32(0, t210);
            let Some(t211) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008de591); };
            c.cmp(t211, c.r32(0), 32);
            if c.cc_ne() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008de59a); };
            c.w64(0, t212);
            let Some(t213) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008de59e); };
            c.cmp(t213, c.r32(0), 32);
            if c.cc_ne() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            c.w64(0, c.r64(12));
            blk = 234; continue;
        }
        232 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008de5aa); };
            if !c.push64(t214) { return RegfileExit::fault(0x70008de5aa); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008de5ad); }
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008de5af); };
            c.w64(11, t215);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008de5b6); }
            let Some(t216) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x70008de5b8); };
            c.w64(3, t216);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008de5c9, 0x70008de5c4);
        }
        233 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008de5c9); };
            c.w64(9, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008de5cd); };
            c.w64(6, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008de5d1); };
            c.w64(15, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008de5d5); };
            c.w64(1, t220);
            blk = 234; continue;
        }
        234 => {
            let Some(t221) = c.ld8(c.r64(0).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008de5d9); };
            c.w32(11, t221);
            let r222 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r222);
            let r223 = c.r32(11) & 0x3fu64;
            c.w32(11, r223);
            c.logic(r223, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008de5e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008de5ea); }
            c.cmp(c.r32(11), 0x1cu64, 32);
            if c.cc_l() { blk = 253; continue; }
            blk = 235; continue;
        }
        235 => {
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_l() { blk = 245; continue; }
            blk = 236; continue;
        }
        236 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_l() { blk = 241; continue; }
            blk = 237; continue;
        }
        237 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_e() { blk = 290; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(11), 0x25u64, 32);
            if c.cc_e() { blk = 291; continue; }
            blk = 239; continue;
        }
        239 => {
            c.cmp(c.r32(11), 0x26u64, 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 240; continue;
        }
        240 => {
            blk = 589; continue;
        }
        241 => {
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_e() { blk = 287; continue; }
            blk = 242; continue;
        }
        242 => {
            c.cmp(c.r32(11), 0x22u64, 32);
            if c.cc_e() { blk = 288; continue; }
            blk = 243; continue;
        }
        243 => {
            c.cmp(c.r32(11), 0x23u64, 32);
            if c.cc_e() { blk = 289; continue; }
            blk = 244; continue;
        }
        244 => {
            blk = 589; continue;
        }
        245 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_l() { blk = 250; continue; }
            blk = 246; continue;
        }
        246 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 247; continue;
        }
        247 => {
            c.cmp(c.r32(11), 0x1fu64, 32);
            if c.cc_e() { blk = 285; continue; }
            blk = 248; continue;
        }
        248 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_e() { blk = 286; continue; }
            blk = 249; continue;
        }
        249 => {
            blk = 589; continue;
        }
        250 => {
            c.cmp(c.r32(11), 0x1cu64, 32);
            if c.cc_e() { blk = 282; continue; }
            blk = 251; continue;
        }
        251 => {
            c.cmp(c.r32(11), 0x1du64, 32);
            if c.cc_e() { blk = 283; continue; }
            blk = 252; continue;
        }
        252 => {
            blk = 589; continue;
        }
        253 => {
            c.cmp(c.r32(11), 0x16u64, 32);
            if c.cc_l() { blk = 263; continue; }
            blk = 254; continue;
        }
        254 => {
            c.cmp(c.r32(11), 0x19u64, 32);
            if c.cc_l() { blk = 259; continue; }
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r32(11), 0x19u64, 32);
            if c.cc_e() { blk = 279; continue; }
            blk = 256; continue;
        }
        256 => {
            c.cmp(c.r32(11), 0x1au64, 32);
            if c.cc_e() { blk = 280; continue; }
            blk = 257; continue;
        }
        257 => {
            c.cmp(c.r32(11), 0x1bu64, 32);
            if c.cc_e() { blk = 281; continue; }
            blk = 258; continue;
        }
        258 => {
            blk = 589; continue;
        }
        259 => {
            c.cmp(c.r32(11), 0x16u64, 32);
            if c.cc_e() { blk = 276; continue; }
            blk = 260; continue;
        }
        260 => {
            c.cmp(c.r32(11), 0x17u64, 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 261; continue;
        }
        261 => {
            c.cmp(c.r32(11), 0x18u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 262; continue;
        }
        262 => {
            blk = 589; continue;
        }
        263 => {
            c.cmp(c.r32(11), 0x13u64, 32);
            if c.cc_l() { blk = 268; continue; }
            blk = 264; continue;
        }
        264 => {
            c.cmp(c.r32(11), 0x13u64, 32);
            if c.cc_e() { blk = 273; continue; }
            blk = 265; continue;
        }
        265 => {
            c.cmp(c.r32(11), 0x14u64, 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 266; continue;
        }
        266 => {
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_e() { blk = 275; continue; }
            blk = 267; continue;
        }
        267 => {
            blk = 589; continue;
        }
        268 => {
            c.cmp(c.r32(11), 0x11u64, 32);
            if c.cc_e() { blk = 271; continue; }
            blk = 269; continue;
        }
        269 => {
            c.cmp(c.r32(11), 0x12u64, 32);
            if c.cc_e() { blk = 272; continue; }
            blk = 270; continue;
        }
        270 => {
            blk = 589; continue;
        }
        271 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 293; continue;
        }
        272 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 293; continue;
        }
        273 => {
            c.w32(12, 0x1u64);
            blk = 293; continue;
        }
        274 => {
            c.w32(12, 0x1u64);
            blk = 293; continue;
        }
        275 => {
            c.w32(12, 0x2u64);
            blk = 293; continue;
        }
        276 => {
            c.w32(12, 0x2u64);
            blk = 293; continue;
        }
        277 => {
            c.w32(12, 0x2u64);
            blk = 293; continue;
        }
        278 => {
            c.w32(12, 0x3u64);
            blk = 293; continue;
        }
        279 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 293; continue;
        }
        280 => {
            c.w32(12, 0x3u64);
            blk = 293; continue;
        }
        281 => {
            c.w32(12, 0x3u64);
            blk = 293; continue;
        }
        282 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 293; continue;
        }
        283 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 293; continue;
        }
        284 => {
            c.w32(12, 0x1u64);
            blk = 293; continue;
        }
        285 => {
            c.w32(12, 0x1u64);
            blk = 293; continue;
        }
        286 => {
            c.w32(12, 0x2u64);
            blk = 293; continue;
        }
        287 => {
            c.w32(12, 0x2u64);
            blk = 293; continue;
        }
        288 => {
            c.w32(12, 0x2u64);
            blk = 293; continue;
        }
        289 => {
            c.w32(12, 0x3u64);
            blk = 293; continue;
        }
        290 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 293; continue;
        }
        291 => {
            c.w32(12, 0x3u64);
            blk = 293; continue;
        }
        292 => {
            c.w32(12, 0x3u64);
            blk = 293; continue;
        }
        293 => {
            let Some(t224) = c.ld64(0x70008ddf70u64) else { return RegfileExit::fault(0x70008de7c3); };
            c.w64(3, t224);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 395; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t225) = c.ld64(0x70008ddf83u64) else { return RegfileExit::fault(0x70008de7d3); };
            c.w64(3, t225);
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(12));
            if let Some(sr) = c.shr(c.r64(3), c.r8(1), 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_b() { blk = 395; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(1, c.r64(12));
            c.w64(12, c.r64(15));
            if let Some(sr) = c.shl(c.r64(12), c.r8(1), 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            c.w8(3, (c.cc_a() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008de802); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 590; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w32(1, c.r64(12).wrapping_add(c.r64(12)));
            blk = 297; continue;
        }
        297 => {
            let Some(t226) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008de813); };
            c.w32(7, t226);
            let r227 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r227);
            let Some(t228) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008de819); };
            c.w32(7, t228);
            let r229 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r229);
            let Some(t230) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de81f); };
            c.w32(8, t230);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008de829); }
            c.cmp(c.r64(8), 0x28u64, 64);
            if c.cc_a() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            return RegfileExit::fault(0x70008de833);
        }
        299 => {
            let Some(t231) = c.ld32(c.r64(7).wrapping_add(0xa7)) else { return RegfileExit::fault(0x70008de834); };
            c.w32(1, t231);
            let r232 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r232);
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_ne() { blk = 345; continue; }
            blk = 300; continue;
        }
        300 => {
            c.cmp(c.r64(12), 0x40u64, 64);
            if c.cc_a() { blk = 341; continue; }
            blk = 301; continue;
        }
        301 => {
            c.cmp(c.r64(8), 0x29u64, 64);
            if c.cc_a() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            return RegfileExit::fault(0x70008de856);
        }
        303 => {
            let Some(t233) = c.ld32(c.r64(7).wrapping_add(0xab)) else { return RegfileExit::fault(0x70008de857); };
            c.w32(9, t233);
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008de85e); };
            c.w64(1, t234);
            let Some(t235) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008de865); };
            c.w64(7, t235);
            c.w64(8, c.r64(7).wrapping_add(0x40));
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008de86c); };
            c.w64(6, t236);
            let Some(t237) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008de873); };
            c.cmp(t237, c.r64(8), 64);
            if c.cc_be() { blk = 599; continue; }
            blk = 304; continue;
        }
        304 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008de87c); }
            let r238 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r238);
            blk = 305; continue;
        }
        305 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008de883); }
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008de887); };
            c.w64(9, t239);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008de88e); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008de892); }
            if !c.st32(c.r64(7).wrapping_add(0x2b), 0x3u64) { return RegfileExit::fault(0x70008de896); }
            if !c.st64(c.r64(7).wrapping_add(0xb), c.r64(12)) { return RegfileExit::fault(0x70008de89d); }
            if !c.st64(c.r64(7).wrapping_add(0x1b), 0x0u64) { return RegfileExit::fault(0x70008de8a1); }
            if !c.st64(c.r64(7).wrapping_add(0x23), 0x0u64) { return RegfileExit::fault(0x70008de8a9); }
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x2f), c.r32(9)) { return RegfileExit::fault(0x70008de8b4); }
            if !c.st32(c.r64(7).wrapping_add(0x33), c.r32(9)) { return RegfileExit::fault(0x70008de8b8); }
            if !c.st32(c.r64(7).wrapping_add(0x37), c.r32(9)) { return RegfileExit::fault(0x70008de8bc); }
            if !c.st32(c.r64(7).wrapping_add(0x3b), c.r32(9)) { return RegfileExit::fault(0x70008de8c0); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 310; continue; }
            blk = 306; continue;
        }
        306 => {
            c.w64(8, c.r64(12).wrapping_add(0xb));
            let r240 = c.r64(8) & 0xfffffffffffffffcu64;
            c.w64(8, r240);
            c.logic(r240, 64);
            c.cmp(c.r64(8), 0x20000u64, 64);
            if c.cc_g() { blk = 601; continue; }
            blk = 307; continue;
        }
        307 => {
            c.w64(3, c.r64(1));
            let Some(t241) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008de8e3); };
            c.w64(1, t241);
            c.w64(11, c.r64(1).wrapping_add(c.r64(8)));
            c.w32(2, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t242) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008de8ee); };
            c.cmp(t242, c.r64(11), 64);
            if c.cc_be() { blk = 614; continue; }
            blk = 308; continue;
        }
        308 => {
            if !c.st64(c.r64(3), c.r64(11)) { return RegfileExit::fault(0x70008de8f7); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 309; continue;
        }
        309 => {
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0xd8)) else { return RegfileExit::fault(0x70008de8fe); };
            c.w64(1, t243);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008de905); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008de909); }
            c.w64(1, c.r64(3));
            blk = 311; continue;
        }
        310 => {
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x3d8)) else { return RegfileExit::fault(0x70008de912); };
            c.w64(3, t244);
            c.w64(11, c.r64(3));
            blk = 311; continue;
        }
        311 => {
            let Some(t245) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008de91c); };
            c.logic(t245 & 0x200000u64, 32);
            if c.cc_ne() { blk = 313; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008de925); };
            c.w64(3, t246);
            c.w64(9, c.r64(3));
            blk = 323; continue;
        }
        313 => {
            let Some(t247) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008de934); };
            c.w64(3, t247);
            c.w64(2, c.r64(3).wrapping_add(0x4c));
            let Some(t248) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008de93b); };
            c.cmp(t248, c.r64(2), 64);
            if c.cc_be() { blk = 616; continue; }
            blk = 314; continue;
        }
        314 => {
            if !c.st64(c.r64(1), c.r64(2)) { return RegfileExit::fault(0x70008de944); }
            let r249 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r249);
            blk = 315; continue;
        }
        315 => {
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008de94b); };
            c.w64(2, t250);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008de952); }
            c.w32(2, 0x22u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008de95a); }
            let Some(t251) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de95d); };
            c.w32(2, t251);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 618; continue; }
            blk = 316; continue;
        }
        316 => {
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008de970); }
            let Some(t252) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de974); };
            c.w32(2, t252);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 619; continue; }
            blk = 317; continue;
        }
        317 => {
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008de987); }
            let Some(t253) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de98b); };
            c.w32(9, t253);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 620; continue; }
            blk = 318; continue;
        }
        318 => {
            c.w32(9, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008de9a5); }
            let Some(t254) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de9a9); };
            c.w32(9, t254);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_be() { blk = 621; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w32(9, 0x2u64);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008de9c3); }
            let Some(t255) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008de9c7); };
            c.w32(9, t255);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 622; continue; }
            blk = 320; continue;
        }
        320 => {
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008de9de); }
            c.w64(9, c.r64(3).wrapping_add(0x4b));
            c.w64(2, c.r64(3).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_e() { blk = 323; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008de9ef); };
            c.w64(8, t256);
            blk = 322; continue;
        }
        322 => {
            if !c.st32(c.r64(2), c.r32(8)) { return RegfileExit::fault(0x70008de9f6); }
            let r257 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r257);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_ne() { blk = 322; continue; }
            blk = 323; continue;
        }
        323 => {
            let Some(t258) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dea02); };
            c.w32(9, t258);
            c.w64(2, c.r64(9).wrapping_mul(4));
            let Some(t259) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dea0f); };
            c.w64(9, t259);
            c.w64(8, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t260) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008dea16); };
            c.cmp(t260, c.r64(8), 64);
            if c.cc_be() { blk = 623; continue; }
            blk = 324; continue;
        }
        324 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008dea1f); }
            let r261 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r261);
            blk = 325; continue;
        }
        325 => {
            let r262 = c.sub(c.r64(2), 0x1u64, 64);
            c.w64(2, r262);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008dea2a); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008dea2e); }
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008dea32); };
            c.w64(3, t263);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008dea39); }
            let Some(t264) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dea3d); };
            c.w32(3, t264);
            c.w64(1, c.r64(3));
            let r265 = c.r32(1) & 0xe0000000u64;
            c.w32(1, r265);
            c.logic(r265, 32);
            if c.cc_ne() { blk = 330; continue; }
            blk = 326; continue;
        }
        326 => {
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_e() { blk = 337; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t266) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dea51); };
            c.w64(0, t266);
            blk = 328; continue;
        }
        328 => {
            let r267 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r267);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x70008dea5c); }
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_ne() { blk = 328; continue; }
            blk = 329; continue;
        }
        329 => {
            blk = 337; continue;
        }
        330 => {
            let r268 = c.sub(c.r32(3), 0x20000000u64, 32);
            c.w32(3, r268);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008dea6e); }
            let Some(t269) = c.ld8(c.r64(0).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008dea71); };
            c.w32(3, t269);
            c.w64(3, c.r64(3).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 333; continue; }
            blk = 331; continue;
        }
        331 => {
            let Some(t270) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008dea82); };
            c.w64(6, t270);
            blk = 332; continue;
        }
        332 => {
            let r271 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r271);
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(6)) { return RegfileExit::fault(0x70008dea8d); }
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_ne() { blk = 332; continue; }
            blk = 333; continue;
        }
        333 => {
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 336; continue; }
            blk = 334; continue;
        }
        334 => {
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dea9c); };
            c.w64(2, t272);
            blk = 335; continue;
        }
        335 => {
            let r273 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r273);
            if !c.st32(c.r64(3).wrapping_add(c.r64(9)), c.r32(2)) { return RegfileExit::fault(0x70008deaa7); }
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_ne() { blk = 335; continue; }
            blk = 336; continue;
        }
        336 => {
            let r274 = c.add(c.r32(1), 0xe0000000u64, 32);
            c.w32(1, r274);
            if c.cc_e() { blk = 625; continue; }
            blk = 337; continue;
        }
        337 => {
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008deabd); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008deac1); }
            if !c.st64(c.r64(9).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70008deac5); }
            if !c.st64(c.r64(9).wrapping_add(0x17), c.r64(12)) { return RegfileExit::fault(0x70008deacd); }
            if !c.st64(c.r64(9).wrapping_add(0x1f), c.r64(15)) { return RegfileExit::fault(0x70008dead1); }
            let Some(t275) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008dead5); };
            c.w32(12, t275);
            c.w64(0, c.r64(12));
            let r276 = c.r32(0) & 0xfffffffffffffffeu64;
            c.w32(0, r276);
            c.logic(r276, 32);
            if !c.st32(c.r64(9).wrapping_add(0x33), c.r32(0)) { return RegfileExit::fault(0x70008deadf); }
            let Some(t277) = c.ld32(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008deae3); };
            c.w32(0, t277);
            c.logic(c.r8(0) & 0x20u64, 8);
            if c.cc_ne() { blk = 339; continue; }
            blk = 338; continue;
        }
        338 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 340; continue;
        }
        339 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            blk = 340; continue;
        }
        340 => {
            let r278 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r278);
            let r279 = c.r32(12) & 0xfffffffffffffffcu64;
            c.w32(12, r279);
            c.logic(r279, 32);
            let r280 = c.r32(0) | c.r32(12);
            c.w32(0, r280);
            c.logic(r280, 32);
            if !c.st32(c.r64(9).wrapping_add(0x33), c.r32(0)) { return RegfileExit::fault(0x70008deaff); }
            c.w32(12, c.r32(11));
            c.w64(0, c.r64(11));
            let r281 = c.sub(c.r64(0), c.r64(12), 64);
            c.w64(0, r281);
            if !c.st32(c.r64(9).wrapping_add(0x2f), c.r32(11)) { return RegfileExit::fault(0x70008deb0c); }
            c.w64(11, c.r64(0).wrapping_add(0x7));
            if !c.st64(c.r64(9).wrapping_add(0x27), c.r64(11)) { return RegfileExit::fault(0x70008deb14); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0x37), c.r32(11)) { return RegfileExit::fault(0x70008deb1b); }
            if !c.st32(c.r64(9).wrapping_add(0x3b), c.r32(11)) { return RegfileExit::fault(0x70008deb1f); }
            if !c.st32(c.r64(9).wrapping_add(0x3f), c.r32(11)) { return RegfileExit::fault(0x70008deb23); }
            if !c.st32(c.r64(9).wrapping_add(0x43), c.r32(11)) { return RegfileExit::fault(0x70008deb27); }
            blk = 378; continue;
        }
        341 => {
            c.cmp(c.r64(8), 0x2au64, 64);
            if c.cc_a() { blk = 343; continue; }
            blk = 342; continue;
        }
        342 => {
            return RegfileExit::fault(0x70008deb36);
        }
        343 => {
            let Some(t282) = c.ld32(c.r64(7).wrapping_add(0xaf)) else { return RegfileExit::fault(0x70008deb37); };
            c.w32(7, t282);
            let r283 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008deb40); };
            if !c.push64(t284) { return RegfileExit::fault(0x70008deb40); }
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008deb43); };
            c.w64(9, t285);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008deb4a); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008deb56, 0x70008deb51);
        }
        344 => {
            blk = 346; continue;
        }
        345 => {
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008deb58); };
            if !c.push64(t286) { return RegfileExit::fault(0x70008deb58); }
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008deb5b); };
            c.w64(3, t287);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008deb62); }
            c.w64(7, c.r64(1));
            c.w64(2, c.r64(9));
            c.w32(0, 0x1u64);
            return c.call(0x70007f7214, 0x70008deb73, 0x70008deb6e);
        }
        346 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 394; continue; }
            blk = 347; continue;
        }
        347 => {
            let Some(t288) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008deb7b); };
            c.w32(9, t288);
            let r289 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r289);
            let Some(t290) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008deb82); };
            c.cmp(t290, 0x835u64, 16);
            if c.cc_ne() { blk = 394; continue; }
            blk = 348; continue;
        }
        348 => {
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008deb8f); };
            c.w64(12, t291);
            let Some(t292) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008deb93); };
            c.logic(t292 & 0x200000u64, 32);
            if c.cc_ne() { blk = 350; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008deb9e); };
            c.w64(3, t293);
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008deba5); };
            c.w64(11, t294);
            let Some(t295) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008debac); };
            c.w64(9, t295);
            blk = 360; continue;
        }
        350 => {
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008debb8); };
            c.w64(9, t296);
            let Some(t297) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008debbf); };
            c.w64(11, t297);
            c.w64(15, c.r64(11).wrapping_add(0x4c));
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008debc6); };
            c.w64(3, t298);
            let Some(t299) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008debcd); };
            c.cmp(t299, c.r64(15), 64);
            if c.cc_be() { blk = 627; continue; }
            blk = 351; continue;
        }
        351 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008debd6); }
            let r300 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r300);
            blk = 352; continue;
        }
        352 => {
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008debdd); };
            c.w64(15, t301);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008debe4); }
            c.w32(15, 0x22u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008debee); }
            let Some(t302) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008debf2); };
            c.w32(15, t302);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 629; continue; }
            blk = 353; continue;
        }
        353 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008dec09); }
            let Some(t303) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dec0d); };
            c.w32(2, t303);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 630; continue; }
            blk = 354; continue;
        }
        354 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008dec21); }
            let Some(t304) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dec25); };
            c.w32(15, t304);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_be() { blk = 631; continue; }
            blk = 355; continue;
        }
        355 => {
            c.w32(15, 0x8u64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008dec3f); }
            let Some(t305) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dec43); };
            c.w32(15, t305);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_be() { blk = 632; continue; }
            blk = 356; continue;
        }
        356 => {
            c.w32(15, 0x2u64);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008dec5d); }
            let Some(t306) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dec61); };
            c.w32(15, t306);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_be() { blk = 633; continue; }
            blk = 357; continue;
        }
        357 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008dec78); }
            c.w64(15, c.r64(11).wrapping_add(0x4b));
            c.w64(2, c.r64(11).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_e() { blk = 360; continue; }
            blk = 358; continue;
        }
        358 => {
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008dec89); };
            c.w64(1, t307);
            blk = 359; continue;
        }
        359 => {
            if !c.st32(c.r64(2), c.r32(1)) { return RegfileExit::fault(0x70008dec90); }
            let r308 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r308);
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_ne() { blk = 359; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t309) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008dec9b); };
            c.w32(15, t309);
            c.w64(2, c.r64(15).wrapping_mul(4));
            let Some(t310) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008deca9); };
            c.w64(15, t310);
            c.w64(1, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t311) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008decb0); };
            c.cmp(t311, c.r64(1), 64);
            if c.cc_be() { blk = 634; continue; }
            blk = 361; continue;
        }
        361 => {
            if !c.st64(c.r64(9), c.r64(1)) { return RegfileExit::fault(0x70008decb9); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            blk = 362; continue;
        }
        362 => {
            c.w64(15, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008decc4); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008decc8); }
            let Some(t312) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008deccc); };
            c.w64(11, t312);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008decd3); }
            let Some(t313) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008decd7); };
            c.w32(11, t313);
            c.w64(3, c.r64(11));
            let r314 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r314);
            c.logic(r314, 32);
            if c.cc_ne() { blk = 367; continue; }
            blk = 363; continue;
        }
        363 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 374; continue; }
            blk = 364; continue;
        }
        364 => {
            let Some(t315) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008deced); };
            c.w64(11, t315);
            blk = 365; continue;
        }
        365 => {
            let r316 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r316);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x70008decf8); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 365; continue; }
            blk = 366; continue;
        }
        366 => {
            blk = 374; continue;
        }
        367 => {
            let r317 = c.sub(c.r32(11), 0x20000000u64, 32);
            c.w32(11, r317);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008ded0b); }
            let Some(t318) = c.ld8(c.r64(12).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008ded10); };
            c.w32(11, t318);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 370; continue; }
            blk = 368; continue;
        }
        368 => {
            let Some(t319) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008ded23); };
            c.w64(2, t319);
            blk = 369; continue;
        }
        369 => {
            let r320 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r320);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(2)) { return RegfileExit::fault(0x70008ded2e); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ne() { blk = 369; continue; }
            blk = 370; continue;
        }
        370 => {
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 373; continue; }
            blk = 371; continue;
        }
        371 => {
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ded3d); };
            c.w64(15, t321);
            blk = 372; continue;
        }
        372 => {
            let r322 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r322);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x70008ded48); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 372; continue; }
            blk = 373; continue;
        }
        373 => {
            let r323 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r323);
            if c.cc_e() { blk = 636; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t324) = c.ld64(c.r64(13).wrapping_add(0x3d8)) else { return RegfileExit::fault(0x70008ded5e); };
            c.w64(11, t324);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008ded65); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008ded69); }
            if !c.st64(c.r64(9).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70008ded6d); }
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ded75); };
            c.w64(11, t325);
            if !c.st64(c.r64(9).wrapping_add(0x17), c.r64(11)) { return RegfileExit::fault(0x70008ded79); }
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ded7d); };
            c.w64(15, t326);
            if !c.st64(c.r64(9).wrapping_add(0x1f), c.r64(15)) { return RegfileExit::fault(0x70008ded81); }
            let Some(t327) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008ded85); };
            c.w32(11, t327);
            c.w64(12, c.r64(11));
            let r328 = c.r32(12) & 0xfffffffffffffffeu64;
            c.w32(12, r328);
            c.logic(r328, 32);
            if !c.st32(c.r64(9).wrapping_add(0x33), c.r32(12)) { return RegfileExit::fault(0x70008ded90); }
            let Some(t329) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008ded94); };
            c.w32(12, t329);
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 376; continue; }
            blk = 375; continue;
        }
        375 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 377; continue;
        }
        376 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            blk = 377; continue;
        }
        377 => {
            let r330 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r330);
            let r331 = c.r32(11) & 0xfffffffffffffffcu64;
            c.w32(11, r331);
            c.logic(r331, 32);
            let r332 = c.r32(12) | c.r32(11);
            c.w32(12, r332);
            c.logic(r332, 32);
            if !c.st32(c.r64(9).wrapping_add(0x33), c.r32(12)) { return RegfileExit::fault(0x70008dedb9); }
            let Some(t333) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008dedbd); };
            c.w64(11, t333);
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0x2f), c.r32(12)) { return RegfileExit::fault(0x70008dedc4); }
            if !c.st64(c.r64(9).wrapping_add(0x27), c.r64(11)) { return RegfileExit::fault(0x70008dedc8); }
            if !c.st32(c.r64(9).wrapping_add(0x37), c.r32(12)) { return RegfileExit::fault(0x70008dedcc); }
            if !c.st32(c.r64(9).wrapping_add(0x3b), c.r32(12)) { return RegfileExit::fault(0x70008dedd0); }
            if !c.st32(c.r64(9).wrapping_add(0x3f), c.r32(12)) { return RegfileExit::fault(0x70008dedd4); }
            if !c.st32(c.r64(9).wrapping_add(0x43), c.r32(12)) { return RegfileExit::fault(0x70008dedd8); }
            blk = 378; continue;
        }
        378 => {
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008deddc); };
            c.w64(1, t334);
            let Some(t335) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008dede0); };
            c.w32(11, t335);
            let r336 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r336);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008dede7); }
            let Some(t337) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008dedeb); };
            c.cmp(t337, 0x82eu64, 16);
            if c.cc_ne() { blk = 638; continue; }
            blk = 379; continue;
        }
        379 => {
            let Some(t338) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008dedf8); };
            c.w32(12, t338);
            let r339 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r339);
            let Some(t340) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008dedff); };
            c.w32(12, t340);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 391; continue; }
            blk = 380; continue;
        }
        380 => {
            let Some(t341) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008dee0e); };
            c.w32(11, t341);
            c.w64(0, c.r64(11));
            let r342 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r342);
            let r343 = c.r32(0) & 0x3fu64;
            c.w32(0, r343);
            c.logic(r343, 32);
            let Some(t344) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dee1c); };
            c.cmp(c.r32(0), t344, 32);
            if c.cc_ne() { blk = 388; continue; }
            blk = 381; continue;
        }
        381 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 393; continue; }
            blk = 382; continue;
        }
        382 => {
            let Some(t345) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008dee2f); };
            c.w64(8, t345);
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 385; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t346) = c.ld64(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008dee39); };
            c.w64(11, t346);
            let Some(t347) = c.ld32(c.r64(9).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008dee3d); };
            c.w32(12, t347);
            let Some(t348) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008dee41); };
            c.w64(15, t348);
            let Some(t349) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008dee45); };
            c.w32(0, t349);
            c.w64(2, c.r64(15).wrapping_add(c.r64(0)));
            c.w64(1, c.r64(11).wrapping_add(c.r64(12)));
            c.w64(10, c.r64(4));
            let r350 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r350);
            let r351 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r351);
            c.logic(r351, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008dee5b); }
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008dee60); };
            c.w64(0, t352);
            c.w64(10, 0x70008dee6eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008dee6e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008dee72); }
            return c.call(c.r64(0), 0x70008dee78, 0x70008dee76);
        }
        384 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008dee78); }
            let Some(t353) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008dee80); };
            c.w64(4, t353);
            blk = 387; continue;
        }
        385 => {
            c.w64(0, c.r64(9));
            let Some(t354) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008dee8a); };
            c.w64(9, t354);
            let Some(t355) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008dee8e); };
            c.w32(11, t355);
            let Some(t356) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008dee92); };
            c.w64(12, t356);
            let Some(t357) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008dee96); };
            c.w32(15, t357);
            c.w64(2, c.r64(12).wrapping_add(c.r64(15)));
            c.w64(1, c.r64(9).wrapping_add(c.r64(11)));
            c.w64(10, c.r64(4));
            let r358 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r358);
            let r359 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r359);
            c.logic(r359, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008deead); }
            let Some(t360) = c.ld64(c.r64(13).wrapping_add(0x13e0)) else { return RegfileExit::fault(0x70008deeb2); };
            c.w64(0, t360);
            c.w64(10, 0x70008deec0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008deec0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008deec4); }
            return c.call(c.r64(0), 0x70008deeca, 0x70008deec8);
        }
        386 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008deeca); }
            let Some(t361) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008deed2); };
            c.w64(4, t361);
            blk = 387; continue;
        }
        387 => {
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008deed7); };
            c.w64(9, t362);
            blk = 393; continue;
        }
        388 => {
            c.w32(11, c.r8(11));
            let r363 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r363);
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008deee8); };
            c.w64(12, t364);
            let r365 = c.sub(c.r32(12), 0x1au64, 32);
            c.w32(12, r365);
            let r366 = c.r32(11) & 0x3fu64;
            c.w32(11, r366);
            c.logic(r366, 32);
            let r367 = c.sub(c.r32(11), 0x1au64, 32);
            c.w32(11, r367);
            c.cmp(c.r32(12), 0x1u64, 32);
            c.w8(12, (c.cc_be() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(11), 0x1u64, 32);
            c.w8(11, (c.cc_be() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_e() { blk = 638; continue; }
            blk = 389; continue;
        }
        389 => {
            c.w32(9, 0x2au64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008def1f); }
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008def21); };
            c.w64(3, t368);
            c.w32(0, 0x1u64);
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008def2d); };
            c.w64(6, t369);
            return c.call(0x700086ed74, 0x70008def36, 0x70008def31);
        }
        390 => {
            return RegfileExit::fault(0x70008def36);
        }
        391 => {
            c.w32(9, 0x66u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008def3d); }
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008def3f); };
            c.w64(9, t370);
            let Some(t371) = c.ld32(c.r64(9).wrapping_add(0x3b3)) else { return RegfileExit::fault(0x70008def46); };
            c.w32(9, t371);
            let r372 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r372);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008def50); }
            let Some(t373) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008def52); };
            c.w64(3, t373);
            c.w32(0, 0x2u64);
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008def5e); };
            c.w64(6, t374);
            return c.call(0x700086ed74, 0x70008def67, 0x70008def62);
        }
        392 => {
            return RegfileExit::fault(0x70008def67);
        }
        393 => {
            c.w64(0, c.r64(9));
            blk = 169; continue;
        }
        394 => {
            return RegfileExit::fault(0x70008def70);
        }
        395 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 652; continue; }
            blk = 396; continue;
        }
        396 => {
            c.w32(9, c.r64(15).wrapping_add(c.r64(15)));
            blk = 397; continue;
        }
        397 => {
            c.w32(11, 0x1b4u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008def88); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008def8a); }
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008def8c); };
            c.w64(3, t375);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008def9d, 0x70008def98);
        }
        398 => {
            return RegfileExit::fault(0x70008def9d);
        }
        399 => {
            let Some(t376) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008def9e); };
            c.w32(11, t376);
            let r377 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r377);
            let Some(t378) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008defa5); };
            c.logic(t378 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 403; continue; }
            blk = 400; continue;
        }
        400 => {
            let Some(t379) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008defac); };
            c.w32(11, t379);
            let r380 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008defb3); };
            c.w32(12, t381);
            let Some(t382) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008defb7); };
            c.cmp(t382, c.r32(12), 32);
            if c.cc_ne() { blk = 403; continue; }
            blk = 401; continue;
        }
        401 => {
            let Some(t383) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008defc0); };
            c.cmp(t383, c.r32(0), 32);
            if c.cc_ne() { blk = 403; continue; }
            blk = 402; continue;
        }
        402 => {
            c.w64(0, c.r64(11));
            blk = 405; continue;
        }
        403 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008defcb); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008defcc); }
            let Some(t384) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008defcd); };
            c.w64(11, t384);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008defd4); }
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x2380)) else { return RegfileExit::fault(0x70008defd6); };
            c.w64(3, t385);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008defe7, 0x70008defe2);
        }
        404 => {
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008defe7); };
            c.w64(9, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008defeb); };
            c.w64(1, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008defef); };
            c.w64(6, t388);
            blk = 405; continue;
        }
        405 => {
            let Some(t389) = c.ld8(c.r64(0).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008deff3); };
            c.w32(11, t389);
            let r390 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r390);
            let r391 = c.r32(11) & 0x3fu64;
            c.w32(11, r391);
            c.logic(r391, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008df000); }
            c.cmp(c.r32(11), 0x1cu64, 32);
            if c.cc_l() { blk = 424; continue; }
            blk = 406; continue;
        }
        406 => {
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_l() { blk = 416; continue; }
            blk = 407; continue;
        }
        407 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_l() { blk = 412; continue; }
            blk = 408; continue;
        }
        408 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_e() { blk = 461; continue; }
            blk = 409; continue;
        }
        409 => {
            c.cmp(c.r32(11), 0x25u64, 32);
            if c.cc_e() { blk = 462; continue; }
            blk = 410; continue;
        }
        410 => {
            c.cmp(c.r32(11), 0x26u64, 32);
            if c.cc_e() { blk = 463; continue; }
            blk = 411; continue;
        }
        411 => {
            blk = 661; continue;
        }
        412 => {
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_e() { blk = 458; continue; }
            blk = 413; continue;
        }
        413 => {
            c.cmp(c.r32(11), 0x22u64, 32);
            if c.cc_e() { blk = 459; continue; }
            blk = 414; continue;
        }
        414 => {
            c.cmp(c.r32(11), 0x23u64, 32);
            if c.cc_e() { blk = 460; continue; }
            blk = 415; continue;
        }
        415 => {
            blk = 661; continue;
        }
        416 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_l() { blk = 421; continue; }
            blk = 417; continue;
        }
        417 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_e() { blk = 455; continue; }
            blk = 418; continue;
        }
        418 => {
            c.cmp(c.r32(11), 0x1fu64, 32);
            if c.cc_e() { blk = 456; continue; }
            blk = 419; continue;
        }
        419 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_e() { blk = 457; continue; }
            blk = 420; continue;
        }
        420 => {
            blk = 661; continue;
        }
        421 => {
            c.cmp(c.r32(11), 0x1cu64, 32);
            if c.cc_e() { blk = 453; continue; }
            blk = 422; continue;
        }
        422 => {
            c.cmp(c.r32(11), 0x1du64, 32);
            if c.cc_e() { blk = 454; continue; }
            blk = 423; continue;
        }
        423 => {
            blk = 661; continue;
        }
        424 => {
            c.cmp(c.r32(11), 0x16u64, 32);
            if c.cc_l() { blk = 434; continue; }
            blk = 425; continue;
        }
        425 => {
            c.cmp(c.r32(11), 0x19u64, 32);
            if c.cc_l() { blk = 430; continue; }
            blk = 426; continue;
        }
        426 => {
            c.cmp(c.r32(11), 0x19u64, 32);
            if c.cc_e() { blk = 450; continue; }
            blk = 427; continue;
        }
        427 => {
            c.cmp(c.r32(11), 0x1au64, 32);
            if c.cc_e() { blk = 451; continue; }
            blk = 428; continue;
        }
        428 => {
            c.cmp(c.r32(11), 0x1bu64, 32);
            if c.cc_e() { blk = 452; continue; }
            blk = 429; continue;
        }
        429 => {
            blk = 661; continue;
        }
        430 => {
            c.cmp(c.r32(11), 0x16u64, 32);
            if c.cc_e() { blk = 447; continue; }
            blk = 431; continue;
        }
        431 => {
            c.cmp(c.r32(11), 0x17u64, 32);
            if c.cc_e() { blk = 448; continue; }
            blk = 432; continue;
        }
        432 => {
            c.cmp(c.r32(11), 0x18u64, 32);
            if c.cc_e() { blk = 449; continue; }
            blk = 433; continue;
        }
        433 => {
            blk = 661; continue;
        }
        434 => {
            c.cmp(c.r32(11), 0x13u64, 32);
            if c.cc_l() { blk = 439; continue; }
            blk = 435; continue;
        }
        435 => {
            c.cmp(c.r32(11), 0x13u64, 32);
            if c.cc_e() { blk = 444; continue; }
            blk = 436; continue;
        }
        436 => {
            c.cmp(c.r32(11), 0x14u64, 32);
            if c.cc_e() { blk = 445; continue; }
            blk = 437; continue;
        }
        437 => {
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_e() { blk = 446; continue; }
            blk = 438; continue;
        }
        438 => {
            blk = 661; continue;
        }
        439 => {
            c.cmp(c.r32(11), 0x11u64, 32);
            if c.cc_e() { blk = 442; continue; }
            blk = 440; continue;
        }
        440 => {
            c.cmp(c.r32(11), 0x12u64, 32);
            if c.cc_e() { blk = 443; continue; }
            blk = 441; continue;
        }
        441 => {
            blk = 661; continue;
        }
        442 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 464; continue;
        }
        443 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 464; continue;
        }
        444 => {
            c.w32(11, 0x1u64);
            blk = 464; continue;
        }
        445 => {
            c.w32(11, 0x1u64);
            blk = 464; continue;
        }
        446 => {
            c.w32(11, 0x2u64);
            blk = 464; continue;
        }
        447 => {
            c.w32(11, 0x2u64);
            blk = 464; continue;
        }
        448 => {
            c.w32(11, 0x2u64);
            blk = 464; continue;
        }
        449 => {
            c.w32(11, 0x3u64);
            blk = 464; continue;
        }
        450 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 464; continue;
        }
        451 => {
            c.w32(11, 0x3u64);
            blk = 464; continue;
        }
        452 => {
            c.w32(11, 0x3u64);
            blk = 464; continue;
        }
        453 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 464; continue;
        }
        454 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 464; continue;
        }
        455 => {
            c.w32(11, 0x1u64);
            blk = 464; continue;
        }
        456 => {
            c.w32(11, 0x1u64);
            blk = 464; continue;
        }
        457 => {
            c.w32(11, 0x2u64);
            blk = 464; continue;
        }
        458 => {
            c.w32(11, 0x2u64);
            blk = 464; continue;
        }
        459 => {
            c.w32(11, 0x2u64);
            blk = 464; continue;
        }
        460 => {
            c.w32(11, 0x3u64);
            blk = 464; continue;
        }
        461 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 464; continue;
        }
        462 => {
            c.w32(11, 0x3u64);
            blk = 464; continue;
        }
        463 => {
            c.w32(11, 0x3u64);
            blk = 464; continue;
        }
        464 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008df1d9); }
            c.w64(12, c.r64(1));
            let Some(t392) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df1e0); };
            c.cmp(t392, c.r32(12), 32);
            if c.cc_e() { blk = 478; continue; }
            blk = 465; continue;
        }
        465 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 467; continue; }
            blk = 466; continue;
        }
        466 => {
            c.w64(9, c.r64(12));
            blk = 469; continue;
        }
        467 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x70008df200, 0x70008df1fb);
        }
        468 => {
            c.w64(9, c.r64(0));
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008df203); };
            c.w64(12, t393);
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008df207); };
            c.w64(6, t394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008df20b); };
            c.w64(0, t395);
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008df20f); };
            c.w64(11, t396);
            blk = 469; continue;
        }
        469 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 472; continue; }
            blk = 470; continue;
        }
        470 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 711; continue; }
            blk = 471; continue;
        }
        471 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 479; continue;
        }
        472 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008df22b); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 711; continue; }
            blk = 473; continue;
        }
        473 => {
            let Some(t397) = c.ld64(0x70008ddd4au64) else { return RegfileExit::fault(0x70008df23f); };
            c.w64(10, t397);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 711; continue; }
            blk = 474; continue;
        }
        474 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 477; continue; }
            blk = 475; continue;
        }
        475 => {
            let Some(t398) = c.ld64(0x70008ddd6du64) else { return RegfileExit::fault(0x70008df25f); };
            c.w64(10, t398);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_s() { blk = 477; continue; }
            blk = 476; continue;
        }
        476 => {
            let Some(t399) = c.ld64(0x70008ddd8bu64) else { return RegfileExit::fault(0x70008df27a); };
            c.w64(10, t399);
            let r400 = c.r64(9) | c.r64(10);
            c.w64(9, r400);
            c.logic(r400, 64);
            blk = 477; continue;
        }
        477 => {
            blk = 479; continue;
        }
        478 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 479; continue;
        }
        479 => {
            let Some(t401) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008df289); };
            c.w32(15, t401);
            let r402 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r402);
            let Some(t403) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008df290); };
            c.w32(15, t403);
            let r404 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r404);
            let Some(t405) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df297); };
            c.w32(3, t405);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008df2a1); }
            c.cmp(c.r64(3), 0x28u64, 64);
            if c.cc_a() { blk = 481; continue; }
            blk = 480; continue;
        }
        480 => {
            return RegfileExit::fault(0x70008df2ab);
        }
        481 => {
            let Some(t406) = c.ld32(c.r64(15).wrapping_add(0xa7)) else { return RegfileExit::fault(0x70008df2ac); };
            c.w32(2, t406);
            let r407 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r407);
            let Some(t408) = c.ld64(0x70008ddf70u64) else { return RegfileExit::fault(0x70008df2b6); };
            c.w64(15, t408);
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 711; continue; }
            blk = 482; continue;
        }
        482 => {
            let Some(t409) = c.ld64(0x70008ddf83u64) else { return RegfileExit::fault(0x70008df2c6); };
            c.w64(15, t409);
            c.w64(1, c.r64(11));
            if let Some(sr) = c.shr(c.r64(15), c.r8(1), 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 711; continue; }
            blk = 483; continue;
        }
        483 => {
            c.w64(1, c.r64(11));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.shl(c.r64(11), c.r8(1), 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            c.w8(12, (c.cc_a() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008df2f4); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 663; continue; }
            blk = 484; continue;
        }
        484 => {
            c.w32(15, c.r64(11).wrapping_add(c.r64(11)));
            blk = 485; continue;
        }
        485 => {
            let Some(t410) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008df306); };
            c.w32(3, t410);
            let r411 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r411);
            let Some(t412) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008df30c); };
            c.w32(3, t412);
            let r413 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r413);
            let Some(t414) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df312); };
            c.w32(1, t414);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x28u64, 64);
            if c.cc_a() { blk = 487; continue; }
            blk = 486; continue;
        }
        486 => {
            return RegfileExit::fault(0x70008df321);
        }
        487 => {
            let Some(t415) = c.ld32(c.r64(3).wrapping_add(0xa7)) else { return RegfileExit::fault(0x70008df322); };
            c.w32(7, t415);
            let r416 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r416);
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_ne() { blk = 532; continue; }
            blk = 488; continue;
        }
        488 => {
            c.cmp(c.r64(11), 0x40u64, 64);
            if c.cc_a() { blk = 530; continue; }
            blk = 489; continue;
        }
        489 => {
            c.cmp(c.r64(1), 0x29u64, 64);
            if c.cc_a() { blk = 491; continue; }
            blk = 490; continue;
        }
        490 => {
            return RegfileExit::fault(0x70008df343);
        }
        491 => {
            let Some(t417) = c.ld32(c.r64(3).wrapping_add(0xab)) else { return RegfileExit::fault(0x70008df344); };
            c.w32(15, t417);
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008df34b); };
            c.w64(3, t418);
            let Some(t419) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008df352); };
            c.w64(2, t419);
            c.w64(1, c.r64(2).wrapping_add(0x40));
            let Some(t420) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008df359); };
            c.w64(6, t420);
            let Some(t421) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008df360); };
            c.cmp(t421, c.r64(1), 64);
            if c.cc_be() { blk = 672; continue; }
            blk = 492; continue;
        }
        492 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008df369); }
            let r422 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r422);
            blk = 493; continue;
        }
        493 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008df370); }
            let Some(t423) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008df374); };
            c.w64(15, t423);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008df37b); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008df37f); }
            if !c.st32(c.r64(2).wrapping_add(0x2b), 0x3u64) { return RegfileExit::fault(0x70008df383); }
            if !c.st64(c.r64(2).wrapping_add(0xb), c.r64(11)) { return RegfileExit::fault(0x70008df38a); }
            if !c.st64(c.r64(2).wrapping_add(0x1b), 0x0u64) { return RegfileExit::fault(0x70008df38e); }
            if !c.st64(c.r64(2).wrapping_add(0x23), 0x0u64) { return RegfileExit::fault(0x70008df396); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(2).wrapping_add(0x2f), c.r32(15)) { return RegfileExit::fault(0x70008df3a1); }
            if !c.st32(c.r64(2).wrapping_add(0x33), c.r32(15)) { return RegfileExit::fault(0x70008df3a5); }
            if !c.st32(c.r64(2).wrapping_add(0x37), c.r32(15)) { return RegfileExit::fault(0x70008df3a9); }
            if !c.st32(c.r64(2).wrapping_add(0x3b), c.r32(15)) { return RegfileExit::fault(0x70008df3ad); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 498; continue; }
            blk = 494; continue;
        }
        494 => {
            c.w64(1, c.r64(11).wrapping_add(0xb));
            let r424 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r424);
            c.logic(r424, 64);
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_g() { blk = 674; continue; }
            blk = 495; continue;
        }
        495 => {
            c.w64(12, c.r64(3));
            let Some(t425) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008df3cf); };
            c.w64(3, t425);
            c.w64(7, c.r64(3).wrapping_add(c.r64(1)));
            c.w32(8, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t426) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008df3db); };
            c.cmp(t426, c.r64(7), 64);
            if c.cc_be() { blk = 687; continue; }
            blk = 496; continue;
        }
        496 => {
            if !c.st64(c.r64(12), c.r64(7)) { return RegfileExit::fault(0x70008df3e4); }
            let r427 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r427);
            blk = 497; continue;
        }
        497 => {
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0xd8)) else { return RegfileExit::fault(0x70008df3ec); };
            c.w64(1, t428);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008df3f3); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008df3f6); }
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 499; continue;
        }
        498 => {
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0x3d8)) else { return RegfileExit::fault(0x70008df405); };
            c.w64(12, t429);
            blk = 499; continue;
        }
        499 => {
            let Some(t430) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008df40c); };
            c.logic(t430 & 0x200000u64, 32);
            if c.cc_ne() { blk = 501; continue; }
            blk = 500; continue;
        }
        500 => {
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008df415); };
            c.w64(1, t431);
            c.w64(15, c.r64(1));
            blk = 511; continue;
        }
        501 => {
            let Some(t432) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008df424); };
            c.w64(1, t432);
            c.w64(7, c.r64(1).wrapping_add(0x4c));
            let Some(t433) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008df42b); };
            c.cmp(t433, c.r64(7), 64);
            if c.cc_be() { blk = 689; continue; }
            blk = 502; continue;
        }
        502 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008df434); }
            let r434 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r434);
            blk = 503; continue;
        }
        503 => {
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008df43b); };
            c.w64(7, t435);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008df442); }
            c.w32(7, 0x22u64);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008df44a); }
            let Some(t436) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df44d); };
            c.w32(7, t436);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_be() { blk = 691; continue; }
            blk = 504; continue;
        }
        504 => {
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008df460); }
            let Some(t437) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df464); };
            c.w32(7, t437);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 692; continue; }
            blk = 505; continue;
        }
        505 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008df477); }
            let Some(t438) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df47b); };
            c.w32(15, t438);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_be() { blk = 693; continue; }
            blk = 506; continue;
        }
        506 => {
            c.w32(15, 0x8u64);
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008df495); }
            let Some(t439) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df499); };
            c.w32(15, t439);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_be() { blk = 694; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w32(15, 0x2u64);
            if !c.st32(c.r64(1).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008df4b3); }
            let Some(t440) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df4b7); };
            c.w32(15, t440);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_be() { blk = 695; continue; }
            blk = 508; continue;
        }
        508 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008df4ce); }
            c.w64(15, c.r64(1).wrapping_add(0x4b));
            c.w64(7, c.r64(1).wrapping_add(0x1b));
            c.cmp(c.r64(7), c.r64(15), 64);
            if c.cc_e() { blk = 511; continue; }
            blk = 509; continue;
        }
        509 => {
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df4df); };
            c.w64(8, t441);
            blk = 510; continue;
        }
        510 => {
            if !c.st32(c.r64(7), c.r32(8)) { return RegfileExit::fault(0x70008df4e6); }
            let r442 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r442);
            c.cmp(c.r64(15), c.r64(7), 64);
            if c.cc_ne() { blk = 510; continue; }
            blk = 511; continue;
        }
        511 => {
            let Some(t443) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df4f2); };
            c.w32(15, t443);
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let Some(t444) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008df4fb); };
            c.w64(7, t444);
            c.w64(8, c.r64(15).wrapping_add(c.r64(7)));
            let Some(t445) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008df502); };
            c.cmp(t445, c.r64(8), 64);
            if c.cc_be() { blk = 696; continue; }
            blk = 512; continue;
        }
        512 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008df50b); }
            c.w64(3, c.r64(7).wrapping_add(0x1));
            blk = 513; continue;
        }
        513 => {
            let r446 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r446);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008df516); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008df519); }
            let Some(t447) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008df51c); };
            c.w64(1, t447);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008df523); }
            let Some(t448) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008df526); };
            c.w32(1, t448);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008df529); }
            c.w64(6, c.r64(1));
            let r449 = c.r32(6) & 0xe0000000u64;
            c.w32(6, r449);
            c.logic(r449, 32);
            if c.cc_ne() { blk = 518; continue; }
            blk = 514; continue;
        }
        514 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 525; continue; }
            blk = 515; continue;
        }
        515 => {
            let Some(t450) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df53e); };
            c.w64(0, t450);
            blk = 516; continue;
        }
        516 => {
            let r451 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r451);
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)), c.r32(0)) { return RegfileExit::fault(0x70008df549); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 516; continue; }
            blk = 517; continue;
        }
        517 => {
            blk = 525; continue;
        }
        518 => {
            let r452 = c.sub(c.r32(1), 0x20000000u64, 32);
            c.w32(1, r452);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008df55b); }
            let Some(t453) = c.ld8(c.r64(0).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008df55e); };
            c.w32(1, t453);
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_e() { blk = 521; continue; }
            blk = 519; continue;
        }
        519 => {
            let Some(t454) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008df56f); };
            c.w64(7, t454);
            blk = 520; continue;
        }
        520 => {
            let r455 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r455);
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)), c.r32(7)) { return RegfileExit::fault(0x70008df57a); }
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_ne() { blk = 520; continue; }
            blk = 521; continue;
        }
        521 => {
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_e() { blk = 524; continue; }
            blk = 522; continue;
        }
        522 => {
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df589); };
            c.w64(15, t456);
            blk = 523; continue;
        }
        523 => {
            let r457 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r457);
            if !c.st32(c.r64(1).wrapping_add(c.r64(3)), c.r32(15)) { return RegfileExit::fault(0x70008df594); }
            c.cmp(c.r64(1), 0xbu64, 64);
            if c.cc_ne() { blk = 523; continue; }
            blk = 524; continue;
        }
        524 => {
            let r458 = c.add(c.r32(6), 0xe0000000u64, 32);
            c.w32(6, r458);
            if c.cc_e() { blk = 698; continue; }
            blk = 525; continue;
        }
        525 => {
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008df5aa); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008df5ae); }
            if !c.st64(c.r64(3).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70008df5b1); }
            if !c.st64(c.r64(3).wrapping_add(0x17), c.r64(11)) { return RegfileExit::fault(0x70008df5b9); }
            if !c.st64(c.r64(3).wrapping_add(0x1f), c.r64(9)) { return RegfileExit::fault(0x70008df5bd); }
            let Some(t459) = c.ld32(c.r64(3).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008df5c1); };
            c.w32(9, t459);
            c.w64(15, c.r64(9));
            let r460 = c.r32(15) & 0xfffffffffffffffeu64;
            c.w32(15, r460);
            c.logic(r460, 32);
            if !c.st32(c.r64(3).wrapping_add(0x33), c.r32(15)) { return RegfileExit::fault(0x70008df5cc); }
            let Some(t461) = c.ld32(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008df5d0); };
            c.w32(15, t461);
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 527; continue; }
            blk = 526; continue;
        }
        526 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 528; continue;
        }
        527 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            blk = 528; continue;
        }
        528 => {
            let r462 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r462);
            let r463 = c.r32(9) & 0xfffffffffffffffcu64;
            c.w32(9, r463);
            c.logic(r463, 32);
            let r464 = c.r32(15) | c.r32(9);
            c.w32(15, r464);
            c.logic(r464, 32);
            if !c.st32(c.r64(3).wrapping_add(0x33), c.r32(15)) { return RegfileExit::fault(0x70008df5f5); }
            c.w32(9, c.r32(12));
            c.w64(15, c.r64(12));
            let r465 = c.sub(c.r64(15), c.r64(9), 64);
            c.w64(15, r465);
            if !c.st32(c.r64(3).wrapping_add(0x2f), c.r32(12)) { return RegfileExit::fault(0x70008df602); }
            c.w64(9, c.r64(15).wrapping_add(0x7));
            if !c.st64(c.r64(3).wrapping_add(0x27), c.r64(9)) { return RegfileExit::fault(0x70008df60a); }
            c.w32(2, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(3).wrapping_add(0x37), c.r32(2)) { return RegfileExit::fault(0x70008df610); }
            if !c.st32(c.r64(3).wrapping_add(0x3b), c.r32(2)) { return RegfileExit::fault(0x70008df613); }
            if !c.st32(c.r64(3).wrapping_add(0x3f), c.r32(2)) { return RegfileExit::fault(0x70008df616); }
            if !c.st32(c.r64(3).wrapping_add(0x43), c.r32(2)) { return RegfileExit::fault(0x70008df619); }
            let Some(t466) = c.ld64(c.r64(3).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008df61c); };
            c.w64(9, t466);
            let Some(t467) = c.ld32(c.r64(3).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008df620); };
            c.w32(12, t467);
            c.w64(1, c.r64(9).wrapping_add(c.r64(12)));
            c.w64(10, c.r64(4));
            let r468 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r468);
            let r469 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r469);
            c.logic(r469, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008df633); }
            c.w64(8, c.r64(11));
            let Some(t470) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008df63b); };
            c.w64(0, t470);
            c.w64(10, 0x70008df649u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008df649); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008df64d); }
            return c.call(c.r64(0), 0x70008df653, 0x70008df651);
        }
        529 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008df653); }
            let Some(t471) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008df65b); };
            c.w64(4, t471);
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008df660); };
            c.w64(0, t472);
            blk = 169; continue;
        }
        530 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008df669); }
            let Some(t473) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df66b); };
            c.w64(12, t473);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008df672); }
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(7));
            return c.call(0x70007f7214, 0x70008df681, 0x70008df67c);
        }
        531 => {
            blk = 533; continue;
        }
        532 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008df683); }
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df685); };
            c.w64(12, t474);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008df68c); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f7214, 0x70008df698, 0x70008df693);
        }
        533 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 565; continue; }
            blk = 534; continue;
        }
        534 => {
            let Some(t475) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008df6a0); };
            c.w32(9, t475);
            let r476 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r476);
            let Some(t477) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008df6a7); };
            c.cmp(t477, 0x835u64, 16);
            if c.cc_ne() { blk = 565; continue; }
            blk = 535; continue;
        }
        535 => {
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008df6b4); };
            c.w64(11, t478);
            let Some(t479) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008df6b8); };
            c.logic(t479 & 0x200000u64, 32);
            if c.cc_ne() { blk = 537; continue; }
            blk = 536; continue;
        }
        536 => {
            let Some(t480) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008df6c2); };
            c.w64(3, t480);
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008df6c9); };
            c.w64(12, t481);
            let Some(t482) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008df6d0); };
            c.w64(9, t482);
            blk = 547; continue;
        }
        537 => {
            let Some(t483) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008df6dc); };
            c.w64(9, t483);
            let Some(t484) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008df6e3); };
            c.w64(12, t484);
            c.w64(15, c.r64(12).wrapping_add(0x4c));
            let Some(t485) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008df6eb); };
            c.w64(3, t485);
            let Some(t486) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008df6f2); };
            c.cmp(t486, c.r64(15), 64);
            if c.cc_be() { blk = 700; continue; }
            blk = 538; continue;
        }
        538 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008df6fb); }
            let r487 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r487);
            blk = 539; continue;
        }
        539 => {
            let Some(t488) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008df702); };
            c.w64(15, t488);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008df709); }
            c.w32(15, 0x22u64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008df714); }
            let Some(t489) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df719); };
            c.w32(15, t489);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 702; continue; }
            blk = 540; continue;
        }
        540 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008df731); }
            let Some(t490) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df736); };
            c.w32(2, t490);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_be() { blk = 703; continue; }
            blk = 541; continue;
        }
        541 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008df74b); }
            let Some(t491) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df750); };
            c.w32(15, t491);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_be() { blk = 704; continue; }
            blk = 542; continue;
        }
        542 => {
            c.w32(15, 0x8u64);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008df76b); }
            let Some(t492) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df770); };
            c.w32(15, t492);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_be() { blk = 705; continue; }
            blk = 543; continue;
        }
        543 => {
            c.w32(15, 0x2u64);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008df78b); }
            let Some(t493) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df790); };
            c.w32(15, t493);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_be() { blk = 706; continue; }
            blk = 544; continue;
        }
        544 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008df7a8); }
            c.w64(15, c.r64(12).wrapping_add(0x4b));
            c.w64(2, c.r64(12).wrapping_add(0x1b));
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_e() { blk = 547; continue; }
            blk = 545; continue;
        }
        545 => {
            let Some(t494) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df7bc); };
            c.w64(1, t494);
            blk = 546; continue;
        }
        546 => {
            if !c.st32(c.r64(2), c.r32(1)) { return RegfileExit::fault(0x70008df7c3); }
            let r495 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r495);
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_ne() { blk = 546; continue; }
            blk = 547; continue;
        }
        547 => {
            let Some(t496) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008df7ce); };
            c.w32(15, t496);
            c.w64(2, c.r64(15).wrapping_mul(4));
            let Some(t497) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008df7db); };
            c.w64(15, t497);
            c.w64(1, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t498) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008df7e2); };
            c.cmp(t498, c.r64(1), 64);
            if c.cc_be() { blk = 707; continue; }
            blk = 548; continue;
        }
        548 => {
            if !c.st64(c.r64(9), c.r64(1)) { return RegfileExit::fault(0x70008df7eb); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            blk = 549; continue;
        }
        549 => {
            c.w64(15, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008df7f6); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008df7fa); }
            let Some(t499) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008df7fe); };
            c.w64(12, t499);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008df805); }
            let Some(t500) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008df809); };
            c.w32(12, t500);
            c.w64(3, c.r64(12));
            let r501 = c.r32(3) & 0xe0000000u64;
            c.w32(3, r501);
            c.logic(r501, 32);
            if c.cc_ne() { blk = 554; continue; }
            blk = 550; continue;
        }
        550 => {
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_e() { blk = 561; continue; }
            blk = 551; continue;
        }
        551 => {
            let Some(t502) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df81e); };
            c.w64(11, t502);
            blk = 552; continue;
        }
        552 => {
            let r503 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r503);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x70008df829); }
            c.cmp(c.r64(15), 0xbu64, 64);
            if c.cc_ne() { blk = 552; continue; }
            blk = 553; continue;
        }
        553 => {
            blk = 561; continue;
        }
        554 => {
            let r504 = c.sub(c.r32(12), 0x20000000u64, 32);
            c.w32(12, r504);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008df83c); }
            let Some(t505) = c.ld8(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008df840); };
            c.w32(12, t505);
            c.w64(12, c.r64(12).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_e() { blk = 557; continue; }
            blk = 555; continue;
        }
        555 => {
            let Some(t506) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008df852); };
            c.w64(2, t506);
            blk = 556; continue;
        }
        556 => {
            let r507 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r507);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(2)) { return RegfileExit::fault(0x70008df85d); }
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ne() { blk = 556; continue; }
            blk = 557; continue;
        }
        557 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 560; continue; }
            blk = 558; continue;
        }
        558 => {
            let Some(t508) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008df86c); };
            c.w64(15, t508);
            blk = 559; continue;
        }
        559 => {
            let r509 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r509);
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x70008df877); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_ne() { blk = 559; continue; }
            blk = 560; continue;
        }
        560 => {
            let r510 = c.add(c.r32(3), 0xe0000000u64, 32);
            c.w32(3, r510);
            if c.cc_e() { blk = 709; continue; }
            blk = 561; continue;
        }
        561 => {
            let Some(t511) = c.ld64(c.r64(13).wrapping_add(0x3d8)) else { return RegfileExit::fault(0x70008df88d); };
            c.w64(11, t511);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008df894); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008df898); }
            if !c.st64(c.r64(9).wrapping_add(0xf), 0x0u64) { return RegfileExit::fault(0x70008df89c); }
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008df8a4); };
            c.w64(11, t512);
            if !c.st64(c.r64(9).wrapping_add(0x17), c.r64(11)) { return RegfileExit::fault(0x70008df8a8); }
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008df8ac); };
            c.w64(11, t513);
            if !c.st64(c.r64(9).wrapping_add(0x1f), c.r64(11)) { return RegfileExit::fault(0x70008df8b0); }
            let Some(t514) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008df8b4); };
            c.w32(11, t514);
            c.w64(12, c.r64(11));
            let r515 = c.r32(12) & 0xfffffffffffffffeu64;
            c.w32(12, r515);
            c.logic(r515, 32);
            if !c.st32(c.r64(9).wrapping_add(0x33), c.r32(12)) { return RegfileExit::fault(0x70008df8bf); }
            let Some(t516) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008df8c3); };
            c.w32(12, t516);
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 563; continue; }
            blk = 562; continue;
        }
        562 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 564; continue;
        }
        563 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            blk = 564; continue;
        }
        564 => {
            let r517 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r517);
            let r518 = c.r32(11) & 0xfffffffffffffffcu64;
            c.w32(11, r518);
            c.logic(r518, 32);
            let r519 = c.r32(12) | c.r32(11);
            c.w32(12, r519);
            c.logic(r519, 32);
            if !c.st32(c.r64(9).wrapping_add(0x33), c.r32(12)) { return RegfileExit::fault(0x70008df8e8); }
            let Some(t520) = c.ld64(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008df8ec); };
            c.w64(11, t520);
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0x2f), c.r32(12)) { return RegfileExit::fault(0x70008df8f3); }
            if !c.st64(c.r64(9).wrapping_add(0x27), c.r64(11)) { return RegfileExit::fault(0x70008df8f7); }
            if !c.st32(c.r64(9).wrapping_add(0x37), c.r32(12)) { return RegfileExit::fault(0x70008df8fb); }
            if !c.st32(c.r64(9).wrapping_add(0x3b), c.r32(12)) { return RegfileExit::fault(0x70008df8ff); }
            if !c.st32(c.r64(9).wrapping_add(0x3f), c.r32(12)) { return RegfileExit::fault(0x70008df903); }
            if !c.st32(c.r64(9).wrapping_add(0x43), c.r32(12)) { return RegfileExit::fault(0x70008df907); }
            c.w64(0, c.r64(9));
            blk = 169; continue;
        }
        565 => {
            return RegfileExit::fault(0x70008df913);
        }
        566 => {
            return RegfileExit::fault(0x70008df914);
        }
        567 => {
            blk = 581; continue;
        }
        568 => {
            c.w64(2, c.r64(1));
            blk = 134; continue;
        }
        569 => {
            c.w64(2, c.r64(1));
            blk = 134; continue;
        }
        570 => {
            c.w32(1, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008df92f); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008df93b, 0x70008df936);
        }
        571 => {
            c.w64(1, c.r64(0));
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008df93e); };
            c.w64(8, t521);
            c.w32(12, 0x1u64);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008df948); };
            c.w64(15, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008df94c); };
            c.w64(3, t523);
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008df950); };
            c.w64(11, t524);
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008df954); };
            c.w64(2, t525);
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008df958); };
            c.w64(9, t526);
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008df95c); };
            c.w64(0, t527);
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008df963); };
            c.w64(7, t528);
            blk = 143; continue;
        }
        572 => {
            return RegfileExit::fault(0x70008df96f);
        }
        573 => {
            return RegfileExit::fault(0x70008df970);
        }
        574 => {
            return RegfileExit::fault(0x70008df971);
        }
        575 => {
            return RegfileExit::fault(0x70008df972);
        }
        576 => {
            return RegfileExit::fault(0x70008df973);
        }
        577 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008df974); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008df978); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008df97c); }
            return c.call(0x70007febf4, 0x70008df985, 0x70008df980);
        }
        578 => {
            c.w64(11, c.r64(0));
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008df988); };
            c.w64(8, t529);
            c.w32(12, 0x1u64);
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008df992); };
            c.w64(15, t530);
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008df996); };
            c.w64(3, t531);
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008df99a); };
            c.w64(9, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008df99e); };
            c.w64(1, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008df9a2); };
            c.w64(2, t534);
            blk = 153; continue;
        }
        579 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008df9ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008df9ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008df9b1); }
            c.w64(0, c.r64(12));
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t535) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008df9ba); };
            c.w64(3, t535);
            c.w64(15, c.r64(3));
            return c.call(0x700086ed74, 0x70008df9c9, 0x70008df9c4);
        }
        580 => {
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008df9c9); };
            c.w64(8, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008df9cd); };
            c.w64(11, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008df9d1); };
            c.w64(3, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008df9d5); };
            c.w64(9, t539);
            blk = 165; continue;
        }
        581 => {
            c.w32(9, 0x19eu64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008df9e4); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008df9e6); }
            let Some(t540) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008df9e8); };
            c.w64(3, t540);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008df9f9, 0x70008df9f4);
        }
        582 => {
            return RegfileExit::fault(0x70008df9f9);
        }
        583 => {
            let Some(t541) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008df9fa); };
            c.w64(9, t541);
            let Some(t542) = c.ld32(c.r64(9).wrapping_add(0x3b7)) else { return RegfileExit::fault(0x70008dfa01); };
            c.w32(9, t542);
            let r543 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r543);
            blk = 585; continue;
        }
        584 => {
            let Some(t544) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008dfa0d); };
            c.w64(9, t544);
            let Some(t545) = c.ld32(c.r64(9).wrapping_add(0x3bb)) else { return RegfileExit::fault(0x70008dfa14); };
            c.w32(9, t545);
            let r546 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r546);
            blk = 585; continue;
        }
        585 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dfa1e); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008dfa1f); }
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0x21e8)) else { return RegfileExit::fault(0x70008dfa21); };
            c.w64(3, t547);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008dfa32, 0x70008dfa2d);
        }
        586 => {
            return RegfileExit::fault(0x70008dfa32);
        }
        587 => {
            c.w32(9, 0x54u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008dfa39); }
            let Some(t548) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008dfa3b); };
            c.w64(9, t548);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008dfa42); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dfa44); }
            let Some(t549) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008dfa45); };
            c.w64(3, t549);
            c.w32(0, 0x3u64);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dfa51); };
            c.w64(6, t550);
            return c.call(0x700086ed74, 0x70008dfa5a, 0x70008dfa55);
        }
        588 => {
            return RegfileExit::fault(0x70008dfa5a);
        }
        589 => {
            return RegfileExit::fault(0x70008dfa5b);
        }
        590 => {
            let Some(t551) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfa5c); };
            c.w64(1, t551);
            let Some(t552) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dfa63); };
            c.w64(7, t552);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008dfa6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008dfa6e); }
            let Some(t553) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfa72); };
            c.w64(7, t553);
            let Some(t554) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008dfa79); };
            c.cmp(t554, c.r64(8), 64);
            if c.cc_a() { blk = 593; continue; }
            blk = 591; continue;
        }
        591 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008dfa8b, 0x70008dfa86);
        }
        592 => {
            c.w64(1, c.r64(0));
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dfa8e); };
            c.w64(6, t555);
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfa92); };
            c.w64(15, t556);
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008dfa96); };
            c.w64(2, t557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dfa9a); };
            c.w64(11, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfa9e); };
            c.w64(12, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfaa2); };
            c.w64(0, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfaa6); };
            c.w64(9, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfaaa); };
            c.w64(3, t562);
            blk = 594; continue;
        }
        593 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008dfab0); }
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfab3); };
            c.w64(1, t563);
            let r564 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r564);
            blk = 594; continue;
        }
        594 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 598; continue; }
            blk = 595; continue;
        }
        595 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 597; continue; }
            blk = 596; continue;
        }
        596 => {
            let r565 = c.r64(10) | 0x1u64;
            c.w64(10, r565);
            c.logic(r565, 64);
            blk = 597; continue;
        }
        597 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 598; continue;
        }
        598 => {
            let Some(t566) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dfae2); };
            c.w64(7, t566);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008dfae9); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dfaec); }
            blk = 297; continue;
        }
        599 => {
            c.w32(2, 0x40u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008dfafb); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008dfaff); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x70008dfb0b, 0x70008dfb06);
        }
        600 => {
            c.w64(7, c.r64(0));
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfb0e); };
            c.w64(15, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008dfb12); };
            c.w64(2, t568);
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dfb16); };
            c.w64(11, t569);
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfb1a); };
            c.w64(12, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfb1e); };
            c.w64(0, t571);
            let Some(t572) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfb22); };
            c.w64(1, t572);
            let Some(t573) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfb29); };
            c.w64(6, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfb30); };
            c.w64(3, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfb34); };
            c.w64(9, t575);
            blk = 305; continue;
        }
        601 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008dfb3d); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 603; continue; }
            blk = 602; continue;
        }
        602 => {
            c.w32(3, c.r64(12).wrapping_add(c.r64(12)));
            blk = 612; continue;
        }
        603 => {
            let Some(t576) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008dfb4f); };
            c.w64(3, t576);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 607; continue; }
            blk = 604; continue;
        }
        604 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 606; continue; }
            blk = 605; continue;
        }
        605 => {
            let r577 = c.r64(10) | 0x1u64;
            c.w64(10, r577);
            c.logic(r577, 64);
            blk = 606; continue;
        }
        606 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 607; continue;
        }
        607 => {
            c.w64(8, c.r64(3).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008dfb7d); }
            let Some(t578) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008dfb82); };
            c.cmp(t578, c.r64(8), 64);
            if c.cc_a() { blk = 610; continue; }
            blk = 608; continue;
        }
        608 => {
            c.w32(2, 0xcu64);
            c.w64(3, c.r64(2));
            return c.call(0x70007febf4, 0x70008dfb94, 0x70008dfb8f);
        }
        609 => {
            c.w64(3, c.r64(0));
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfb97); };
            c.w64(15, t579);
            let Some(t580) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008dfb9b); };
            c.w64(2, t580);
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dfb9f); };
            c.w64(11, t581);
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfba3); };
            c.w64(12, t582);
            let Some(t583) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfba7); };
            c.w64(0, t583);
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfbab); };
            c.w64(7, t584);
            let Some(t585) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfbaf); };
            c.w64(1, t585);
            let Some(t586) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfbb6); };
            c.w64(6, t586);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 611; continue;
        }
        610 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008dfbc2); }
            let r587 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r587);
            blk = 611; continue;
        }
        611 => {
            let Some(t588) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dfbc9); };
            c.w64(8, t588);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008dfbd0); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008dfbd4); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dfbd9); }
            blk = 612; continue;
        }
        612 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008dfbde); }
            let Some(t589) = c.ld64(c.r64(13).wrapping_add(0x20d0)) else { return RegfileExit::fault(0x70008dfbdf); };
            c.w64(3, t589);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x70008dfbf3, 0x70008dfbee);
        }
        613 => {
            c.w64(11, c.r64(0));
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfbf6); };
            c.w64(15, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfbfa); };
            c.w64(12, t591);
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfbfe); };
            c.w64(0, t592);
            let Some(t593) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfc02); };
            c.w64(7, t593);
            let Some(t594) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfc06); };
            c.w64(1, t594);
            let Some(t595) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfc0d); };
            c.w64(6, t595);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 311; continue;
        }
        614 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008dfc1c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008dfc20); }
            c.w64(2, c.r64(8));
            return c.call(0x70007febf4, 0x70008dfc2c, 0x70008dfc27);
        }
        615 => {
            c.w64(11, c.r64(0));
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfc2f); };
            c.w64(15, t596);
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfc33); };
            c.w64(12, t597);
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfc37); };
            c.w64(0, t598);
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfc3b); };
            c.w64(7, t599);
            let Some(t600) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfc3f); };
            c.w64(3, t600);
            let Some(t601) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfc46); };
            c.w64(6, t601);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfc50); };
            c.w64(2, t602);
            blk = 309; continue;
        }
        616 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008dfc5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008dfc62); }
            return c.call(0x70007febf4, 0x70008dfc6b, 0x70008dfc66);
        }
        617 => {
            c.w64(3, c.r64(0));
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfc6e); };
            c.w64(15, t603);
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfc72); };
            c.w64(12, t604);
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfc76); };
            c.w64(0, t605);
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfc7a); };
            c.w64(11, t606);
            let Some(t607) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfc7e); };
            c.w64(7, t607);
            let Some(t608) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfc82); };
            c.w64(1, t608);
            let Some(t609) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfc89); };
            c.w64(6, t609);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 315; continue;
        }
        618 => {
            return RegfileExit::fault(0x70008dfc98);
        }
        619 => {
            return RegfileExit::fault(0x70008dfc99);
        }
        620 => {
            return RegfileExit::fault(0x70008dfc9a);
        }
        621 => {
            return RegfileExit::fault(0x70008dfc9b);
        }
        622 => {
            return RegfileExit::fault(0x70008dfc9c);
        }
        623 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008dfc9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008dfca1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008dfca5); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(2)) { return RegfileExit::fault(0x70008dfca9); }
            return c.call(0x70007febf4, 0x70008dfcb2, 0x70008dfcad);
        }
        624 => {
            c.w64(9, c.r64(0));
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfcb5); };
            c.w64(15, t610);
            let Some(t611) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfcb9); };
            c.w64(12, t611);
            let Some(t612) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfcbd); };
            c.w64(0, t612);
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfcc1); };
            c.w64(11, t613);
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfcc5); };
            c.w64(7, t614);
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008dfcc9); };
            c.w64(3, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008dfccd); };
            c.w64(2, t616);
            blk = 325; continue;
        }
        625 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008dfcd6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008dfcd7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008dfcdb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008dfcdf); }
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008dfce3); };
            c.w64(3, t617);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008dfcf6, 0x70008dfcf1);
        }
        626 => {
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfcf6); };
            c.w64(15, t618);
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfcfa); };
            c.w64(9, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dfcfe); };
            c.w64(12, t620);
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfd02); };
            c.w64(11, t621);
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfd06); };
            c.w64(7, t622);
            blk = 337; continue;
        }
        627 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008dfd14); }
            return c.call(0x70007febf4, 0x70008dfd1d, 0x70008dfd18);
        }
        628 => {
            c.w64(11, c.r64(0));
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfd20); };
            c.w64(0, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfd24); };
            c.w64(12, t624);
            let Some(t625) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfd28); };
            c.w64(9, t625);
            let Some(t626) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfd2f); };
            c.w64(3, t626);
            blk = 352; continue;
        }
        629 => {
            return RegfileExit::fault(0x70008dfd3b);
        }
        630 => {
            return RegfileExit::fault(0x70008dfd3c);
        }
        631 => {
            return RegfileExit::fault(0x70008dfd3d);
        }
        632 => {
            return RegfileExit::fault(0x70008dfd3e);
        }
        633 => {
            return RegfileExit::fault(0x70008dfd3f);
        }
        634 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008dfd40); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008dfd44); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008dfd48); }
            return c.call(0x70007febf4, 0x70008dfd51, 0x70008dfd4c);
        }
        635 => {
            c.w64(9, c.r64(0));
            let Some(t627) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfd54); };
            c.w64(0, t627);
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008dfd58); };
            c.w64(12, t628);
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008dfd5c); };
            c.w64(11, t629);
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008dfd60); };
            c.w64(2, t630);
            blk = 362; continue;
        }
        636 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008dfd69); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008dfd6b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008dfd6f); }
            let Some(t631) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008dfd73); };
            c.w64(3, t631);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008dfd86, 0x70008dfd81);
        }
        637 => {
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfd86); };
            c.w64(9, t632);
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfd8a); };
            c.w64(0, t633);
            blk = 374; continue;
        }
        638 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 393; continue; }
            blk = 639; continue;
        }
        639 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 641; continue; }
            blk = 640; continue;
        }
        640 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15)));
            blk = 650; continue;
        }
        641 => {
            let Some(t634) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfdaf); };
            c.w64(11, t634);
            let Some(t635) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008dfdb6); };
            c.w64(12, t635);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 645; continue; }
            blk = 642; continue;
        }
        642 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 644; continue; }
            blk = 643; continue;
        }
        643 => {
            let r636 = c.r64(10) | 0x1u64;
            c.w64(10, r636);
            c.logic(r636, 64);
            blk = 644; continue;
        }
        644 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 645; continue;
        }
        645 => {
            c.w64(15, c.r64(12).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008dfde5); }
            let Some(t637) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfdea); };
            c.w64(0, t637);
            let Some(t638) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008dfdf1); };
            c.cmp(t638, c.r64(15), 64);
            if c.cc_a() { blk = 648; continue; }
            blk = 646; continue;
        }
        646 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008dfe00, 0x70008dfdfb);
        }
        647 => {
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008dfe00); };
            c.w64(1, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfe04); };
            c.w64(9, t640);
            blk = 649; continue;
        }
        648 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008dfe0a); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 649; continue;
        }
        649 => {
            let Some(t641) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dfe15); };
            c.w64(11, t641);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008dfe1c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008dfe20); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dfe25); }
            c.w64(11, c.r64(0));
            blk = 650; continue;
        }
        650 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008dfe2d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008dfe2f); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008dfe30); }
            let Some(t642) = c.ld64(c.r64(13).wrapping_add(0x2b20)) else { return RegfileExit::fault(0x70008dfe32); };
            c.w64(3, t642);
            c.w32(0, 0x3u64);
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dfe3e); };
            c.w64(6, t643);
            return c.call(0x700086ed74, 0x70008dfe47, 0x70008dfe42);
        }
        651 => {
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfe47); };
            c.w64(9, t644);
            blk = 393; continue;
        }
        652 => {
            let Some(t645) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfe50); };
            c.w64(9, t645);
            let Some(t646) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008dfe57); };
            c.w64(11, t646);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t647) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfe5e); };
            c.w64(0, t647);
            let Some(t648) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008dfe65); };
            c.cmp(t648, c.r64(12), 64);
            if c.cc_a() { blk = 655; continue; }
            blk = 653; continue;
        }
        653 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008dfe74, 0x70008dfe6f);
        }
        654 => {
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008dfe74); };
            c.w64(15, t649);
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dfe78); };
            c.w64(6, t650);
            blk = 656; continue;
        }
        655 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008dfe7e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 656; continue;
        }
        656 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 660; continue; }
            blk = 657; continue;
        }
        657 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 659; continue; }
            blk = 658; continue;
        }
        658 => {
            let r651 = c.r64(10) | 0x1u64;
            c.w64(10, r651);
            c.logic(r651, 64);
            blk = 659; continue;
        }
        659 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 660; continue;
        }
        660 => {
            let Some(t652) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dfeaf); };
            c.w64(9, t652);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008dfeb6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dfeba); }
            c.w64(9, c.r64(0));
            blk = 397; continue;
        }
        661 => {
            return RegfileExit::fault(0x70008dfec7);
        }
        662 => {
            blk = 711; continue;
        }
        663 => {
            let Some(t653) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dfecd); };
            c.w64(15, t653);
            let Some(t654) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008dfed4); };
            c.w64(3, t654);
            c.w64(1, c.r64(3).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008dfedb); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x70008dfedf); }
            let Some(t655) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dfee3); };
            c.w64(7, t655);
            let Some(t656) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008dfeea); };
            c.cmp(t656, c.r64(1), 64);
            if c.cc_a() { blk = 666; continue; }
            blk = 664; continue;
        }
        664 => {
            c.w32(2, 0xcu64);
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x70008dfefc, 0x70008dfef7);
        }
        665 => {
            c.w64(15, c.r64(0));
            let Some(t657) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dfeff); };
            c.w64(6, t657);
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dff03); };
            c.w64(9, t658);
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dff07); };
            c.w64(11, t659);
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dff0b); };
            c.w64(0, t660);
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008dff0f); };
            c.w64(2, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008dff13); };
            c.w64(12, t662);
            blk = 667; continue;
        }
        666 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x70008dff19); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 667; continue;
        }
        667 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 671; continue; }
            blk = 668; continue;
        }
        668 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 670; continue; }
            blk = 669; continue;
        }
        669 => {
            let r663 = c.r64(10) | 0x1u64;
            c.w64(10, r663);
            c.logic(r663, 64);
            blk = 670; continue;
        }
        670 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 671; continue;
        }
        671 => {
            let Some(t664) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008dff47); };
            c.w64(3, t664);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008dff4e); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008dff52); }
            blk = 485; continue;
        }
        672 => {
            c.w32(2, 0x40u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(12)) { return RegfileExit::fault(0x70008dff62); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008dff66); }
            return c.call(0x70007febf4, 0x70008dff6f, 0x70008dff6a);
        }
        673 => {
            c.w64(2, c.r64(0));
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dff72); };
            c.w64(9, t665);
            let Some(t666) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dff76); };
            c.w64(11, t666);
            let Some(t667) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dff7a); };
            c.w64(0, t667);
            let Some(t668) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008dff7e); };
            c.w64(3, t668);
            let Some(t669) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008dff85); };
            c.w64(6, t669);
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008dff8c); };
            c.w64(12, t670);
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008dff90); };
            c.w64(15, t671);
            blk = 493; continue;
        }
        674 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008dff99); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 676; continue; }
            blk = 675; continue;
        }
        675 => {
            c.w32(12, c.r64(11).wrapping_add(c.r64(11)));
            blk = 685; continue;
        }
        676 => {
            let Some(t672) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008dffac); };
            c.w64(12, t672);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 680; continue; }
            blk = 677; continue;
        }
        677 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 679; continue; }
            blk = 678; continue;
        }
        678 => {
            let r673 = c.r64(10) | 0x1u64;
            c.w64(10, r673);
            c.logic(r673, 64);
            blk = 679; continue;
        }
        679 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 680; continue;
        }
        680 => {
            c.w64(1, c.r64(12).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008dffdb); }
            let Some(t674) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008dffe0); };
            c.cmp(t674, c.r64(1), 64);
            if c.cc_a() { blk = 683; continue; }
            blk = 681; continue;
        }
        681 => {
            c.w32(2, 0xcu64);
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x70008dfff2, 0x70008dffed);
        }
        682 => {
            c.w64(12, c.r64(0));
            let Some(t675) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008dfff5); };
            c.w64(9, t675);
            let Some(t676) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008dfff9); };
            c.w64(11, t676);
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008dfffd); };
            c.w64(0, t677);
            let Some(t678) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0001); };
            c.w64(2, t678);
            let Some(t679) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0005); };
            c.w64(3, t679);
            let Some(t680) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e000c); };
            c.w64(6, t680);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 684; continue;
        }
        683 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008e0018); }
            let r681 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r681);
            blk = 684; continue;
        }
        684 => {
            let Some(t682) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e001f); };
            c.w64(1, t682);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008e0026); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008e002b); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e0030); }
            blk = 685; continue;
        }
        685 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008e0037); }
            c.w64(6, c.r64(15));
            let Some(t683) = c.ld64(c.r64(13).wrapping_add(0x20d0)) else { return RegfileExit::fault(0x70008e003c); };
            c.w64(3, t683);
            c.w32(0, 0x1u64);
            c.w64(12, c.r64(3));
            return c.call(0x700086ed74, 0x70008e0050, 0x70008e004b);
        }
        686 => {
            c.w64(12, c.r64(0));
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0053); };
            c.w64(9, t684);
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0057); };
            c.w64(11, t685);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e005b); };
            c.w64(0, t686);
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e005f); };
            c.w64(2, t687);
            let Some(t688) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0063); };
            c.w64(3, t688);
            let Some(t689) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e006a); };
            c.w64(6, t689);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 499; continue;
        }
        687 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008e0079); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008e007d); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008e0089, 0x70008e0084);
        }
        688 => {
            c.w64(3, c.r64(0));
            let Some(t690) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e008c); };
            c.w64(9, t690);
            let Some(t691) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0090); };
            c.w64(11, t691);
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e0094); };
            c.w64(0, t692);
            let Some(t693) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0098); };
            c.w64(2, t693);
            let Some(t694) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e009c); };
            c.w64(12, t694);
            let Some(t695) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e00a3); };
            c.w64(6, t695);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t696) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e00ad); };
            c.w64(8, t696);
            blk = 497; continue;
        }
        689 => {
            c.w32(1, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70008e00bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008e00bf); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008e00cb, 0x70008e00c6);
        }
        690 => {
            c.w64(1, c.r64(0));
            let Some(t697) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e00ce); };
            c.w64(9, t697);
            let Some(t698) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e00d2); };
            c.w64(11, t698);
            let Some(t699) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e00d6); };
            c.w64(0, t699);
            let Some(t700) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e00da); };
            c.w64(12, t700);
            let Some(t701) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e00de); };
            c.w64(2, t701);
            let Some(t702) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e00e2); };
            c.w64(3, t702);
            let Some(t703) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e00e9); };
            c.w64(6, t703);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 503; continue;
        }
        691 => {
            return RegfileExit::fault(0x70008e00f8);
        }
        692 => {
            return RegfileExit::fault(0x70008e00f9);
        }
        693 => {
            return RegfileExit::fault(0x70008e00fa);
        }
        694 => {
            return RegfileExit::fault(0x70008e00fb);
        }
        695 => {
            return RegfileExit::fault(0x70008e00fc);
        }
        696 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70008e00fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008e0101); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008e0105); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008e0109); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008e0115, 0x70008e0110);
        }
        697 => {
            c.w64(3, c.r64(0));
            let Some(t704) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0118); };
            c.w64(9, t704);
            let Some(t705) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e011c); };
            c.w64(11, t705);
            let Some(t706) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e0120); };
            c.w64(0, t706);
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e0124); };
            c.w64(12, t707);
            let Some(t708) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0128); };
            c.w64(2, t708);
            let Some(t709) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e012c); };
            c.w64(1, t709);
            let Some(t710) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e0130); };
            c.w64(15, t710);
            blk = 513; continue;
        }
        698 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e0139); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(12)) { return RegfileExit::fault(0x70008e013a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008e013e); }
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t711) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008e0149); };
            c.w64(3, t711);
            c.w64(15, c.r64(3));
            return c.call(0x700086ed74, 0x70008e0158, 0x70008e0153);
        }
        699 => {
            let Some(t712) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0158); };
            c.w64(9, t712);
            let Some(t713) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e015c); };
            c.w64(11, t713);
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e0160); };
            c.w64(3, t714);
            let Some(t715) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e0164); };
            c.w64(12, t715);
            let Some(t716) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0168); };
            c.w64(2, t716);
            blk = 525; continue;
        }
        700 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008e0176); }
            return c.call(0x70007febf4, 0x70008e017f, 0x70008e017a);
        }
        701 => {
            c.w64(12, c.r64(0));
            let Some(t717) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e0182); };
            c.w64(0, t717);
            let Some(t718) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e0186); };
            c.w64(11, t718);
            let Some(t719) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e018a); };
            c.w64(9, t719);
            let Some(t720) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0191); };
            c.w64(3, t720);
            blk = 539; continue;
        }
        702 => {
            return RegfileExit::fault(0x70008e019d);
        }
        703 => {
            return RegfileExit::fault(0x70008e019e);
        }
        704 => {
            return RegfileExit::fault(0x70008e019f);
        }
        705 => {
            return RegfileExit::fault(0x70008e01a0);
        }
        706 => {
            return RegfileExit::fault(0x70008e01a1);
        }
        707 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008e01a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008e01a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008e01aa); }
            return c.call(0x70007febf4, 0x70008e01b3, 0x70008e01ae);
        }
        708 => {
            c.w64(9, c.r64(0));
            let Some(t721) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e01b6); };
            c.w64(0, t721);
            let Some(t722) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e01ba); };
            c.w64(11, t722);
            let Some(t723) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e01be); };
            c.w64(12, t723);
            let Some(t724) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e01c2); };
            c.w64(2, t724);
            blk = 549; continue;
        }
        709 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e01cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008e01cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008e01d1); }
            let Some(t725) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008e01d5); };
            c.w64(3, t725);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008e01e8, 0x70008e01e3);
        }
        710 => {
            let Some(t726) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e01e8); };
            c.w64(9, t726);
            let Some(t727) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e01ec); };
            c.w64(0, t727);
            blk = 561; continue;
        }
        711 => {
            c.w32(9, 0x1b4u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e01fb); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008e01fd); }
            let Some(t728) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008e01ff); };
            c.w64(3, t728);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e0210, 0x70008e020b);
        }
        712 => {
            return RegfileExit::fault(0x70008e0210);
        }
        713 => {
            return RegfileExit::jump(0x70008e0214);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e0234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e0234); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e0238); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e0239); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e023a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e023f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e0243); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 66; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e025b); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008e0260); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e026e); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e0275); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 64; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e0282); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e0289); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e02aa); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e02b0); };
            c.w64(0, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008e02b7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e02c3); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e02c9); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e02d9); };
            c.w64(1, t14);
            blk = 12; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e02df); };
            c.w64(1, t15);
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e02e6); };
            c.w32(8, t16);
            c.w64(11, c.r64(8));
            let r17 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            let r18 = c.r32(8) & 0xfcu64;
            c.w32(8, r18);
            c.logic(r18, 32);
            let r19 = c.r32(11) & 0x3fu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008e02fd); }
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(8, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(8, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(8, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(8, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(8, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e037d);
        }
        30 => {
            c.w32(8, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(8, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(8, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(8, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(8, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(8, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            let Some(t20) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e03ac); };
            c.w64(11, t20);
            let r21 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008e03b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008e03b8); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008e03bf); }
            blk = 38; continue;
        }
        37 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e03c5); };
            c.w64(15, t22);
            let r23 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e03cd); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e03d1); };
            c.w64(12, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e03d5); };
            c.w64(9, t26);
            blk = 38; continue;
        }
        38 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e03d9); };
            c.cmp(c.r64(15), t27, 64);
            if c.cc_ae() { blk = 56; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e03e3); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld8(c.r64(2).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e03ea); };
            c.logic(t30 & 0x4u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e03f4); }
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(15));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e03fe); };
            return c.call(t31, 0x70008e0406, 0x70008e03fe);
        }
        41 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e0406); };
            c.cmp(t32, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 68; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e0414); };
            c.w64(15, t33);
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            blk = 43; continue;
        }
        43 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e041c); };
            if !c.push64(t34) { return RegfileExit::fault(0x70008e041c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e041f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e0420); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e0421); };
            if !c.push64(t35) { return RegfileExit::fault(0x70008e0421); }
            c.w32(0, 0x3u64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0429); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e042d); };
            c.w64(6, t37);
            return c.call(0x70007f2274, 0x70008e0436, 0x70008e0431);
        }
        44 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e0436); };
            c.cmp(t38, c.r32(0), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e0445); };
            c.cmp(t39, c.r32(0), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e044e); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            let Some(t42) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e0454); };
            c.logic(t42 & 0x10u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e045a); };
            c.cmp(t43, c.r32(1), 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t44) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e0467); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 77; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 52; continue;
        }
        51 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w32(1, 0x1u64);
            blk = 54; continue;
        }
        53 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e048b); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0492); };
            c.w64(1, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e0499); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e049a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e04a0); }
            return c.ret(0x0, 0x70008e04a2);
        }
        56 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008e04a3); };
            c.w64(0, t47);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e04ad); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e04ae); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e04b4); }
            return c.ret(0x0, 0x70008e04b6);
        }
        57 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t48) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e04bd); };
            c.w64(1, t48);
            blk = 60; continue;
        }
        59 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e04c3); };
            c.w64(1, t49);
            blk = 60; continue;
        }
        60 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e04d0); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e04d2); }
            c.w32(0, 0x2u64);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e04d8); };
            c.w64(3, t50);
            return c.call(0x700086ed74, 0x70008e04e4, 0x70008e04df);
        }
        61 => {
            return RegfileExit::fault(0x70008e04e4);
        }
        62 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e04ea); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e04eb); };
            c.w64(1, t51);
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0x3bf)) else { return RegfileExit::fault(0x70008e04f2); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e04fb); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e04fc); };
            c.w64(3, t54);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e050d, 0x70008e0508);
        }
        63 => {
            return RegfileExit::fault(0x70008e050d);
        }
        64 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e0513); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e0514); };
            c.w64(1, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0x3bf)) else { return RegfileExit::fault(0x70008e051b); };
            c.w32(1, t56);
            let r57 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r57);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e0524); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e0525); };
            c.w64(3, t58);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e0536, 0x70008e0531);
        }
        65 => {
            return RegfileExit::fault(0x70008e0536);
        }
        66 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e0537); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e053d); };
            c.w64(3, t59);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e054c, 0x70008e0547);
        }
        67 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e054c); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0550); };
            c.w64(0, t61);
            blk = 1; continue;
        }
        68 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0559); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e0560); };
            c.w64(8, t63);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e0567); }
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e056b); };
            c.w64(11, t64);
            let Some(t65) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e0572); };
            c.cmp(t65, c.r64(9), 64);
            if c.cc_a() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e0581, 0x70008e057c);
        }
        70 => {
            c.w64(1, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e0584); };
            c.w64(0, t66);
            blk = 72; continue;
        }
        71 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e058a); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 72; continue;
        }
        72 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e0591); };
            c.w64(8, t67);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e0598); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e059c); };
            c.w64(10, t68);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 76; continue; }
            blk = 73; continue;
        }
        73 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let r69 = c.r64(10) | 0x1u64;
            c.w64(10, r69);
            c.logic(r69, 64);
            blk = 75; continue;
        }
        75 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 76; continue;
        }
        76 => {
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e05c4); }
            blk = 43; continue;
        }
        77 => {
            let Some(t70) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e05ce); };
            c.logic(t70 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 52; continue;
        }
        79 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e05e0); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 52; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 53; continue;
        }
        81 => {
            return RegfileExit::jump(0x70008e05fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e0614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e0614); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e0618); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e0619); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e061a); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e061f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 13; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(7).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e0633); };
            c.w64(1, t2);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0641); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e0647); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e0653); };
            c.w32(7, t6);
            let r7 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r7);
            let Some(t8) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e0659); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e065f); };
            c.w32(7, t9);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4au64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008e066e);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(6).wrapping_add(0x12f)) else { return RegfileExit::fault(0x70008e066f); };
            c.w32(7, t10);
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0675); };
            c.w64(8, t11);
            let Some(t12) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e067c); };
            c.w64(9, t12);
            c.w64(11, c.r64(9).wrapping_add(0x18));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0683); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e068a); };
            c.cmp(t14, c.r64(11), 64);
            if c.cc_be() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e0694); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008e069b); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e069f); };
            c.w64(7, t15);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008e06a6); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008e06aa); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008e06ae); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008e06b4); }
            c.w32(1, 0x4u64);
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008e06bd); }
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let r16 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r16);
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e06ce); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e06cf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e06d5); }
            return c.ret(0x0, 0x70008e06d7);
        }
        9 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e06dd); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e06de); };
            c.w64(1, t17);
            let Some(t18) = c.ld32(c.r64(1).wrapping_add(0x3c3)) else { return RegfileExit::fault(0x70008e06e5); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e06ee); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e06ef); };
            c.w64(3, t20);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e0700, 0x70008e06fb);
        }
        10 => {
            return RegfileExit::fault(0x70008e0700);
        }
        11 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e0706); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e0707); };
            c.w64(1, t21);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0x3c3)) else { return RegfileExit::fault(0x70008e070e); };
            c.w32(1, t22);
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e0717); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e0718); };
            c.w64(3, t24);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e0729, 0x70008e0724);
        }
        12 => {
            return RegfileExit::fault(0x70008e0729);
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e072a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e072e); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e0732); };
            c.w64(3, t25);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e0743, 0x70008e073e);
        }
        14 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0743); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0747); };
            c.w64(0, t27);
            blk = 1; continue;
        }
        15 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e0755); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e0759); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008e075d); }
            return c.call(0x70007febb4, 0x70008e0766, 0x70008e0761);
        }
        16 => {
            c.w64(8, c.r64(0));
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0769); };
            c.w64(0, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e076d); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0771); };
            c.w64(7, t30);
            blk = 8; continue;
        }
        17 => {
            return RegfileExit::jump(0x70008e077c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e0794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e0794); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e0798); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e0799); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e079a); }
            let r0 = c.sub(c.r64(4), 0x70u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e079f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e07a3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 190; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e07bb); };
            c.w64(11, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008e07c0); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e07ce); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e07d5); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 188; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e07e3); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let Some(t8) = c.ld8(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e07ea); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 265; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t9) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e07f8); };
            c.w64(0, t9);
            c.cmp(c.r64(15), 0x0u64, 64);
            c.w8(3, (c.cc_be() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e0806); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(0)) { return RegfileExit::fault(0x70008e080a); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e0816); };
            c.w64(2, t10);
            blk = 7; continue;
        }
        6 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e081c); };
            c.w64(2, t11);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008e0823); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 183; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t12) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0830); };
            c.w32(1, t12);
            let r13 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r13);
            let Some(t14) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e0836); };
            c.logic(t14 & 0x2u64, 8);
            if c.cc_e() { blk = 183; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t15) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e0846); };
            c.w64(8, t15);
            blk = 12; continue;
        }
        11 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e084c); };
            c.w64(8, t16);
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e0853); };
            c.w32(9, t17);
            c.w64(12, c.r64(9));
            let r18 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r18);
            let r19 = c.r32(9) & 0xfcu64;
            c.w32(9, r19);
            c.logic(r19, 32);
            let r20 = c.r32(12) & 0x3fu64;
            c.w32(12, r20);
            c.logic(r20, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(8)) { return RegfileExit::fault(0x70008e086b); }
            c.cmp(c.r32(12), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(9), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(9, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(9, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(9, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(9, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(9, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(9), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(9), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e08eb);
        }
        30 => {
            c.w32(9, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(9, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(9, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(9, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(9, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(9, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x70008e091a); }
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e0920); };
            c.w64(7, t21);
            c.w64(8, c.r64(1));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008e092a); }
            blk = 38; continue;
        }
        37 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0930); };
            c.w64(11, t22);
            let r23 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e0938); }
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008e093c); };
            c.w64(9, t24);
            c.w64(8, c.r64(11));
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e0943); };
            c.w64(11, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0947); };
            c.w64(6, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e094b); };
            c.w64(15, t27);
            blk = 38; continue;
        }
        38 => {
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e094f); };
            c.w64(12, t28);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008e0958); };
            c.cmp(c.r64(8), t29, 64);
            if c.cc_ae() { blk = 76; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008e0965); }
            let Some(t30) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e0969); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008e0970); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008e0974); }
            let Some(t32) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e0978); };
            c.logic(t32 & 0x4u64, 8);
            if c.cc_ne() { blk = 265; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(11));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0986); };
            c.w64(3, t33);
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e098a); };
            return c.call(t34, 0x70008e0992, 0x70008e098a);
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008e0992); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0996); };
            c.cmp(t35, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 192; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e09a4); };
            c.w64(8, t36);
            c.w32(9, c.r64(8).wrapping_add(c.r64(8)));
            blk = 43; continue;
        }
        43 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e09ac); };
            if !c.push64(t37) { return RegfileExit::fault(0x70008e09ac); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e09af); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e09b1); }
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008e09b2); };
            if !c.push64(t38) { return RegfileExit::fault(0x70008e09b2); }
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e09b5); };
            c.w64(7, t39);
            c.w32(0, 0x3u64);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e09be); };
            c.w64(6, t40);
            return c.call(0x70007f2274, 0x70008e09c7, 0x70008e09c2);
        }
        44 => {
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e09c7); };
            c.cmp(t41, c.r32(0), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e09db); };
            c.cmp(t42, c.r32(0), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t43) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e09e8); };
            c.w32(8, t43);
            let r44 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r44);
            let Some(t45) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e09ef); };
            c.logic(t45 & 0x10u64, 8);
            if c.cc_ne() { blk = 75; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t46) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e09fa); };
            c.cmp(t46, c.r32(8), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t47) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e0a07); };
            c.cmp(t47, 0x41u64, 16);
            if c.cc_e() { blk = 201; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 52; continue;
        }
        51 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0a1f); };
            c.w64(8, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e0a23); };
            c.cmp(c.r64(8), t49, 64);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e0a29); };
            c.w64(1, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e0a2d); };
            c.w64(7, t51);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0a31); };
            c.w64(0, t52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0a38); };
            c.w64(6, t53);
            blk = 71; continue;
        }
        54 => {
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e0a44); };
            c.w64(7, t54);
            let Some(t55) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e0a48); };
            c.w32(9, t55);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e0a52); };
            c.w64(1, t56);
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_a() { blk = 177; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_b() { blk = 177; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(9, c.r64(8));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r57 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r57);
            let r58 = c.add(c.r64(8), 0x10u64, 64);
            c.w64(8, r58);
            if c.cc_e() { blk = 70; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 263; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w32(9, c.r64(8).wrapping_add(c.r64(8)));
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w32(12, c.r32(12));
            let r59 = c.imul2(c.r32(12), 0x4u64, 32);
            c.w32(12, r59);
            if c.cc_o() { blk = 181; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0x8u64);
            let r60 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r60);
            if c.cc_o() { blk = 180; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0ab5); };
            c.w64(0, t61);
            let Some(t62) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008e0abc); };
            c.w64(3, t62);
            c.w64(2, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0ac3); };
            c.w64(6, t63);
            let Some(t64) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008e0aca); };
            c.cmp(t64, c.r64(2), 64);
            if c.cc_be() { blk = 205; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 205; continue; }
            blk = 62; continue;
        }
        62 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008e0ae0); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            blk = 63; continue;
        }
        63 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e0ae7); };
            c.w64(3, t65);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008e0aee); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008e0af3); }
            c.w64(9, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.w64(11, c.r64(15));
            blk = 65; continue;
        }
        64 => {
            c.w64(3, c.r64(15).wrapping_add(0x4));
            if !c.st32(c.r64(12).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008e0b09); }
            c.w64(15, c.r64(3));
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(2, c.r64(1).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0b23); };
            c.w32(2, t66);
            let r67 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r67);
            blk = 64; continue;
        }
        68 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e0b2e); };
            c.w64(2, t68);
            blk = 64; continue;
        }
        69 => {
            c.w64(7, c.r64(12));
            blk = 71; continue;
        }
        70 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e0b3c); };
            c.w64(7, t69);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0b43); };
            c.w64(0, t70);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0b4a); };
            c.w64(6, t71);
            blk = 71; continue;
        }
        71 => {
            let Some(t72) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e0b51); };
            c.w32(9, t72);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_ae() { blk = 182; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(9, 0xffffffffffffffffu64);
            c.w64(11, c.r64(9).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x8));
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e0b70); };
            c.w64(12, t73);
            if !c.st32(c.r64(7).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008e0b74); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r74 = c.r64(15) & c.r64(7);
            c.w64(15, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e0b88); };
            c.logic(t75 & 0x4u64, 8);
            if c.cc_ne() { blk = 267; continue; }
            blk = 74; continue;
        }
        74 => {
            let r76 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r76);
            blk = 37; continue;
        }
        75 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0b9c); };
            c.w64(8, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e0ba0); };
            c.w64(7, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e0ba4); };
            c.w64(1, t79);
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0ba8); };
            c.w64(0, t80);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0baf); };
            c.w64(6, t81);
            c.w64(9, 0xffffffffffffffffu64);
            blk = 37; continue;
        }
        76 => {
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            c.w8(8, (c.cc_a() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e0bd1); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(9, c.r64(1).wrapping_add(c.r64(1)));
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(11));
            let Some(t82) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0be6); };
            c.w32(11, t82);
            let r83 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r83);
            let Some(t84) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e0bed); };
            c.w32(2, t84);
            let r85 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r85);
            let r86 = c.r32(2) & 0x3fu64;
            c.w32(2, r86);
            c.logic(r86, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008e0bf8); }
            c.cmp(c.r32(2), 0x1cu64, 32);
            if c.cc_l() { blk = 97; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(2), 0x21u64, 32);
            if c.cc_l() { blk = 89; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(2), 0x24u64, 32);
            if c.cc_l() { blk = 85; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(2), 0x24u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(2), 0x25u64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(2), 0x26u64, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 216; continue;
        }
        85 => {
            c.cmp(c.r32(2), 0x21u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(2), 0x22u64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(2), 0x23u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 88; continue;
        }
        88 => {
            blk = 216; continue;
        }
        89 => {
            c.cmp(c.r32(2), 0x1eu64, 32);
            if c.cc_l() { blk = 94; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(2), 0x1eu64, 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(2), 0x1fu64, 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 92; continue;
        }
        92 => {
            c.cmp(c.r32(2), 0x20u64, 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 93; continue;
        }
        93 => {
            blk = 216; continue;
        }
        94 => {
            c.cmp(c.r32(2), 0x1cu64, 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r32(2), 0x1du64, 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 96; continue;
        }
        96 => {
            blk = 216; continue;
        }
        97 => {
            c.cmp(c.r32(2), 0x16u64, 32);
            if c.cc_l() { blk = 107; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(2), 0x19u64, 32);
            if c.cc_l() { blk = 103; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(2), 0x19u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(2), 0x1au64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(2), 0x1bu64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 216; continue;
        }
        103 => {
            c.cmp(c.r32(2), 0x16u64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(2), 0x17u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(2), 0x18u64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 106; continue;
        }
        106 => {
            blk = 216; continue;
        }
        107 => {
            c.cmp(c.r32(2), 0x13u64, 32);
            if c.cc_l() { blk = 112; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(2), 0x13u64, 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(2), 0x14u64, 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(2), 0x15u64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 111; continue;
        }
        111 => {
            blk = 216; continue;
        }
        112 => {
            c.cmp(c.r32(2), 0x11u64, 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(2), 0x12u64, 32);
            if c.cc_e() { blk = 116; continue; }
            blk = 114; continue;
        }
        114 => {
            blk = 216; continue;
        }
        115 => {
            c.w32(2, 0x10u64);
            blk = 137; continue;
        }
        116 => {
            c.w32(2, 0x11u64);
            blk = 137; continue;
        }
        117 => {
            c.w32(2, 0x12u64);
            blk = 137; continue;
        }
        118 => {
            c.w32(2, 0x13u64);
            blk = 137; continue;
        }
        119 => {
            c.w32(2, 0x14u64);
            blk = 137; continue;
        }
        120 => {
            c.w32(2, 0x15u64);
            blk = 137; continue;
        }
        121 => {
            c.w32(2, 0x16u64);
            blk = 137; continue;
        }
        122 => {
            c.w32(2, 0x17u64);
            blk = 137; continue;
        }
        123 => {
            c.w32(2, 0x18u64);
            blk = 137; continue;
        }
        124 => {
            c.w32(2, 0x19u64);
            blk = 137; continue;
        }
        125 => {
            c.w32(2, 0x1au64);
            blk = 137; continue;
        }
        126 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        127 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        128 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        129 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        130 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        131 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        132 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        133 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        134 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        135 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        136 => {
            c.w64(2, c.r64(3));
            blk = 137; continue;
        }
        137 => {
            let Some(t87) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0d91); };
            c.w32(9, t87);
            let r88 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r88);
            let Some(t89) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e0d98); };
            c.w32(9, t89);
            let r90 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r90);
            let Some(t91) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e0d9f); };
            c.w32(0, t91);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            return RegfileExit::fault(0x70008e0dae);
        }
        139 => {
            let Some(t92) = c.ld32(c.r64(9).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e0daf); };
            c.w32(2, t92);
            let r93 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r93);
            c.cmp(c.r64(0), 0xd4u64, 64);
            if c.cc_a() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            return RegfileExit::fault(0x70008e0dbf);
        }
        141 => {
            let Some(t94) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e0dc0); };
            c.w32(11, t94);
            let r95 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r95);
            let Some(t96) = c.ld32(c.r64(9).wrapping_add(0x357)) else { return RegfileExit::fault(0x70008e0dc7); };
            c.w32(9, t96);
            let Some(t97) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0dce); };
            c.w32(11, t97);
            let r98 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r98);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008e0dd5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008e0dd9); }
            let Some(t99) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e0ddd); };
            c.cmp(t99, 0xbdu64, 16);
            if c.cc_ae() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008e0de6); };
            c.w64(11, t100);
            blk = 144; continue;
        }
        143 => {
            let Some(t101) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008e0def); };
            c.w32(11, t101);
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 218; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xfc8)) else { return RegfileExit::fault(0x70008e0dfc); };
            c.w64(9, t102);
            c.w64(11, c.r64(3));
            let Some(t103) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e0e06); };
            c.cmp(t103, c.r32(11), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(2));
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e0e16); };
            c.w64(2, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0e1d); };
            c.w64(1, t105);
            c.w64(7, c.r64(2));
            c.w64(9, c.r64(2));
            return c.call(0x70008dd9b4, 0x70008e0e2c, 0x70008e0e27);
        }
        147 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e0e2c); }
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e0e30); };
            c.w64(1, t106);
            let Some(t107) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e0e34); };
            c.cmp(t107, c.r64(1), 64);
            if c.cc_b() { blk = 256; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t108) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0e3e); };
            c.cmp(t108, 0x0u64, 32);
            if c.cc_ne() { blk = 241; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w32(8, c.r64(1).wrapping_add(c.r64(1)));
            blk = 150; continue;
        }
        150 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0e4c); };
            c.w64(6, t109);
            let Some(t110) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0e50); };
            c.w32(9, t110);
            let r111 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r111);
            let Some(t112) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e0e57); };
            c.w32(9, t112);
            let r113 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r113);
            let Some(t114) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e0e5e); };
            c.w32(11, t114);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e0e68); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            return RegfileExit::fault(0x70008e0e72);
        }
        152 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e0e73); };
            c.w64(7, t115);
            let Some(t116) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e0e77); };
            c.w32(11, t116);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t117) = c.ld32(c.r64(9).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008e0e7e); };
            c.w32(9, t117);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008e0e88); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_a() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ae() { blk = 157; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e0e97); };
            c.w64(2, t118);
            let Some(t119) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008e0e9e); };
            c.w32(2, t119);
            let r120 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r120);
            return c.call(0x7000824234, 0x70008e0eac, 0x70008e0ea7);
        }
        155 => {
            return RegfileExit::fault(0x70008e0eac);
        }
        156 => {
            return RegfileExit::fault(0x70008e0ead);
        }
        157 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 172; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 252; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w32(11, c.r64(1).wrapping_add(c.r64(1)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w32(12, c.r32(12));
            let r121 = c.imul2(c.r32(12), 0x4u64, 32);
            c.w32(12, r121);
            if c.cc_o() { blk = 171; continue; }
            blk = 160; continue;
        }
        160 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0x8u64);
            let r122 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r122);
            if c.cc_o() { blk = 170; continue; }
            blk = 161; continue;
        }
        161 => {
            c.w32(3, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0ef4); };
            c.w64(2, t123);
            let Some(t124) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008e0efb); };
            c.w64(6, t124);
            c.w64(8, c.r64(6).wrapping_add(c.r64(12)));
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0f02); };
            c.w64(9, t125);
            let Some(t126) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e0f09); };
            c.cmp(t126, c.r64(8), 64);
            if c.cc_be() { blk = 250; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 250; continue; }
            blk = 163; continue;
        }
        163 => {
            c.w64(12, c.r64(2));
            if !c.st64(c.r64(12), c.r64(8)) { return RegfileExit::fault(0x70008e0f22); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 164; continue;
        }
        164 => {
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008e0f2a); };
            c.w64(2, t127);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008e0f31); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008e0f35); }
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(15));
            blk = 166; continue;
        }
        165 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008e0f46); }
            let r128 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r128);
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(2, c.r64(1).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t129) = c.ld32(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e0f61); };
            c.w32(2, t129);
            let r130 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r130);
            blk = 165; continue;
        }
        169 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008e0f6b); };
            c.w64(2, t131);
            blk = 165; continue;
        }
        170 => {
            return RegfileExit::fault(0x70008e0f74);
        }
        171 => {
            return RegfileExit::fault(0x70008e0f75);
        }
        172 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e0f76); };
            c.w64(8, t132);
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e0f7d); };
            c.w64(12, t133);
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e0f84); };
            c.w64(9, t134);
            blk = 173; continue;
        }
        173 => {
            c.w32(11, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t135) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e0f8f); };
            c.w64(15, t135);
            c.w64(3, c.r64(15).wrapping_add(0x10));
            let Some(t136) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e0f97); };
            c.cmp(t136, c.r64(3), 64);
            if c.cc_be() { blk = 254; continue; }
            blk = 174; continue;
        }
        174 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008e0fa0); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            blk = 175; continue;
        }
        175 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e0fa8); };
            c.w64(12, t137);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e0fac); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008e0fb0); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e0fb4); };
            c.w64(11, t138);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008e0fbb); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008e0fbf); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e0fc3); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e0fc4); }
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e0fc6); };
            if !c.push64(t139) { return RegfileExit::fault(0x70008e0fc6); }
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2b20)) else { return RegfileExit::fault(0x70008e0fc9); };
            c.w64(3, t140);
            c.w32(0, 0x3u64);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e0fd5); };
            c.w64(6, t141);
            return c.call(0x700086ed74, 0x70008e0fde, 0x70008e0fd9);
        }
        176 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e0fde); };
            c.w64(8, t142);
            let r143 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e0fe6); };
            c.w64(0, t144);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e0fed); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e0fee); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e0ff4); }
            return c.ret(0x0, 0x70008e0ff6);
        }
        177 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e0ff7); };
            c.w64(2, t145);
            let Some(t146) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008e0ffe); };
            c.w32(2, t146);
            let r147 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r147);
            return c.call(0x7000824234, 0x70008e100c, 0x70008e1007);
        }
        178 => {
            return RegfileExit::fault(0x70008e100c);
        }
        179 => {
            return RegfileExit::fault(0x70008e100d);
        }
        180 => {
            return RegfileExit::fault(0x70008e100e);
        }
        181 => {
            return RegfileExit::fault(0x70008e100f);
        }
        182 => {
            return RegfileExit::fault(0x70008e1010);
        }
        183 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t148) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e1016); };
            c.w64(8, t148);
            blk = 186; continue;
        }
        185 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e101c); };
            c.w64(8, t149);
            blk = 186; continue;
        }
        186 => {
            c.w32(9, 0x32u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e1029); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e102b); }
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e102d); };
            c.w64(3, t150);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e103e, 0x70008e1039);
        }
        187 => {
            return RegfileExit::fault(0x70008e103e);
        }
        188 => {
            c.w32(8, 0xd6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1045); }
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e1047); };
            c.w64(8, t151);
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0x3c7)) else { return RegfileExit::fault(0x70008e104e); };
            c.w32(8, t152);
            let r153 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r153);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1058); }
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e105a); };
            c.w64(3, t154);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e106b, 0x70008e1066);
        }
        189 => {
            return RegfileExit::fault(0x70008e106b);
        }
        190 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e106c); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e1072); };
            c.w64(3, t155);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008e1081, 0x70008e107c);
        }
        191 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e1081); };
            c.w64(6, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1085); };
            c.w64(0, t157);
            blk = 1; continue;
        }
        192 => {
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e108e); };
            c.w64(8, t158);
            let Some(t159) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e1095); };
            c.w64(9, t159);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e109c); };
            c.w64(12, t160);
            let Some(t161) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e10a3); };
            c.cmp(t161, c.r64(11), 64);
            if c.cc_a() { blk = 195; continue; }
            blk = 193; continue;
        }
        193 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e10b3, 0x70008e10ae);
        }
        194 => {
            c.w64(9, c.r64(0));
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e10b6); };
            c.w64(8, t162);
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e10bd); };
            c.w64(12, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e10c4); };
            c.w64(0, t164);
            blk = 196; continue;
        }
        195 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e10ca); }
            let r165 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r165);
            blk = 196; continue;
        }
        196 => {
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e10d1); };
            c.w64(11, t166);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008e10d8); }
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e10dc); };
            c.w64(10, t167);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 200; continue; }
            blk = 197; continue;
        }
        197 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let r168 = c.r64(10) | 0x1u64;
            c.w64(10, r168);
            c.logic(r168, 64);
            blk = 199; continue;
        }
        199 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 200; continue;
        }
        200 => {
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e1104); }
            blk = 43; continue;
        }
        201 => {
            let Some(t169) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e110f); };
            c.logic(t169 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 202; continue;
        }
        202 => {
            blk = 52; continue;
        }
        203 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e1121); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 52; continue; }
            blk = 204; continue;
        }
        204 => {
            blk = 75; continue;
        }
        205 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008e1139); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008e113d); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008e1141); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008e114d, 0x70008e1148);
        }
        206 => {
            c.w64(12, c.r64(0));
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e1150); };
            c.w64(0, t170);
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e1157); };
            c.w64(6, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e115e); };
            c.w64(1, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e1162); };
            c.w64(7, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1166); };
            c.w64(8, t174);
            c.w32(15, 0x8u64);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e1170); };
            c.w64(11, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e1174); };
            c.w64(9, t176);
            blk = 63; continue;
        }
        207 => {
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e117d); };
            c.w64(9, t177);
            let Some(t178) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e1184); };
            c.w64(0, t178);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008e118b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008e118f); }
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e1193); };
            c.w64(6, t179);
            let Some(t180) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008e119a); };
            c.cmp(t180, c.r64(2), 64);
            if c.cc_a() { blk = 210; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e11a9, 0x70008e11a4);
        }
        209 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e11ab); };
            c.w64(15, t181);
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008e11af); };
            c.w64(12, t182);
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e11b6); };
            c.w64(9, t183);
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e11bd); };
            c.w64(6, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e11c4); };
            c.w64(1, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e11c8); };
            c.w64(7, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e11cc); };
            c.w64(8, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e11d0); };
            c.w64(11, t188);
            blk = 211; continue;
        }
        210 => {
            if !c.st64(c.r64(9), c.r64(2)) { return RegfileExit::fault(0x70008e11d6); }
            let r189 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r189);
            blk = 211; continue;
        }
        211 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 215; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 214; continue; }
            blk = 213; continue;
        }
        213 => {
            let r190 = c.r64(10) | 0x1u64;
            c.w64(10, r190);
            c.logic(r190, 64);
            blk = 214; continue;
        }
        214 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 215; continue;
        }
        215 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e1204); };
            c.w64(2, t191);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008e120b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e120e); }
            c.w64(9, c.r64(0));
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e1216); };
            c.w64(6, t192);
            blk = 78; continue;
        }
        216 => {
            return RegfileExit::fault(0x70008e121f);
        }
        217 => {
            blk = 219; continue;
        }
        218 => {
            c.w64(9, c.r64(3));
            c.w64(11, c.r64(3));
            blk = 219; continue;
        }
        219 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008e1228); }
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008e122c); };
            c.w64(3, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e1233); };
            c.w64(0, t194);
            return c.call(0x7000870914, 0x70008e123c, 0x70008e1237);
        }
        220 => {
            let Some(t195) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e123c); };
            c.cmp(t195, c.r32(0), 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 221; continue;
        }
        221 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t196) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e1251); };
            c.w32(8, t196);
            let r197 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r197);
            let Some(t198) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e1258); };
            c.cmp(t198, 0xbau64, 16);
            if c.cc_b() { blk = 231; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008e1265); };
            c.w64(3, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e126c); };
            c.w64(6, t200);
            return c.call(0x7000870914, 0x70008e1275, 0x70008e1270);
        }
        224 => {
            let Some(t201) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008e1275); };
            c.cmp(t201, c.r32(0), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t202) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1284); };
            c.cmp(t202, c.r32(0), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r203 = c.r32(8) | c.r32(9);
            c.w32(8, r203);
            c.logic(r203, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 225; continue;
        }
        225 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 228; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t204) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e129c); };
            c.w32(8, t204);
            let r205 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r205);
            let Some(t206) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e12a3); };
            c.logic(t206 & 0x40u64, 8);
            if c.cc_e() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(2, c.r64(0));
            blk = 234; continue;
        }
        228 => {
            c.w32(8, 0x26eu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e12b5); }
            c.w32(0, 0x1u64);
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e12bc); };
            c.w64(3, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e12c3); };
            c.w64(6, t208);
            return c.call(0x700086ed74, 0x70008e12cc, 0x70008e12c7);
        }
        229 => {
            return RegfileExit::fault(0x70008e12cc);
        }
        230 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e12cd); };
            c.w64(2, t209);
            blk = 234; continue;
        }
        231 => {
            c.w32(8, 0x54u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e12d9); }
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e12db); };
            c.w64(8, t210);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e12e2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e12e4); }
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e12e5); };
            c.w64(3, t211);
            c.w32(0, 0x3u64);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e12f1); };
            c.w64(6, t212);
            return c.call(0x700086ed74, 0x70008e12fa, 0x70008e12f5);
        }
        232 => {
            return RegfileExit::fault(0x70008e12fa);
        }
        233 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e12fb); };
            c.w64(2, t213);
            blk = 234; continue;
        }
        234 => {
            let Some(t214) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e12ff); };
            c.w32(8, t214);
            let r215 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r215);
            let Some(t216) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e1306); };
            c.logic(t216 & 0x40u64, 8);
            if c.cc_ne() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            return RegfileExit::fault(0x70008e130d);
        }
        236 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e130e); };
            if !c.push64(t217) { return RegfileExit::fault(0x70008e130e); }
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1311); };
            c.w64(8, t218);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1318); }
            c.w64(7, c.r64(2));
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e131d); };
            c.w64(6, t219);
            c.w32(0, 0x1u64);
            return c.call(0x70007f7214, 0x70008e132b, 0x70008e1326);
        }
        237 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 260; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t220) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e1333); };
            c.w32(8, t220);
            let r221 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r221);
            let Some(t222) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e133a); };
            c.cmp(t222, 0x82eu64, 16);
            if c.cc_ne() { blk = 260; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t223) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e1347); };
            c.w32(8, t223);
            let r224 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r224);
            let Some(t225) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e134e); };
            c.logic(t225 & 0x4u64, 8);
            if c.cc_ne() { blk = 258; continue; }
            blk = 240; continue;
        }
        240 => {
            blk = 147; continue;
        }
        241 => {
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e135e); };
            c.w64(8, t226);
            let Some(t227) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e1365); };
            c.w64(9, t227);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(1) as i64));
            c.logic(c.r64(1) & c.r64(1), 64);
            if c.cc_ns() { blk = 245; continue; }
            blk = 242; continue;
        }
        242 => {
            c.w64(10, c.r64(1));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            let r228 = c.r64(10) | 0x1u64;
            c.w64(10, r228);
            c.logic(r228, 64);
            blk = 244; continue;
        }
        244 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 245; continue;
        }
        245 => {
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008e1393); }
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e1398); };
            c.w64(12, t229);
            let Some(t230) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e139f); };
            c.cmp(t230, c.r64(11), 64);
            if c.cc_a() { blk = 248; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e13af, 0x70008e13aa);
        }
        247 => {
            c.w64(9, c.r64(0));
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e13b2); };
            c.w64(0, t231);
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e13b6); };
            c.w64(8, t232);
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e13bd); };
            c.w64(12, t233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e13c4); };
            c.w64(1, t234);
            blk = 249; continue;
        }
        248 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e13ca); }
            let r235 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r235);
            blk = 249; continue;
        }
        249 => {
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e13d1); };
            c.w64(11, t236);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008e13d8); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008e13dc); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e13e1); }
            c.w64(8, c.r64(9));
            blk = 150; continue;
        }
        250 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008e13ef); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x70008e13f3); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008e13ff, 0x70008e13fa);
        }
        251 => {
            c.w64(8, c.r64(0));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1402); };
            c.w64(0, t237);
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e1406); };
            c.w64(12, t238);
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e140d); };
            c.w64(9, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e1414); };
            c.w64(1, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e1418); };
            c.w64(7, t241);
            c.w32(15, 0x8u64);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e1422); };
            c.w64(11, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e1426); };
            c.w64(3, t243);
            blk = 164; continue;
        }
        252 => {
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e142f); };
            c.w64(3, t244);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e143f, 0x70008e143a);
        }
        253 => {
            return RegfileExit::fault(0x70008e143f);
        }
        254 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008e1440); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008e1444); }
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008e1452, 0x70008e144d);
        }
        255 => {
            c.w64(9, c.r64(0));
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1455); };
            c.w64(0, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e1459); };
            c.w64(8, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e145d); };
            c.w64(11, t247);
            blk = 175; continue;
        }
        256 => {
            c.w32(8, 0x296u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e146c); }
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e146e); };
            c.w64(3, t248);
            c.w32(0, 0x1u64);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e147a); };
            c.w64(6, t249);
            return c.call(0x700086ed74, 0x70008e1483, 0x70008e147e);
        }
        257 => {
            return RegfileExit::fault(0x70008e1483);
        }
        258 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e148a); }
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e148c); };
            c.w64(8, t250);
            let Some(t251) = c.ld32(c.r64(8).wrapping_add(0x3c7)) else { return RegfileExit::fault(0x70008e1493); };
            c.w32(8, t251);
            let r252 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r252);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e149d); }
            c.w32(0, 0x2u64);
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e14a4); };
            c.w64(3, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e14ab); };
            c.w64(6, t254);
            return c.call(0x700086ed74, 0x70008e14b4, 0x70008e14af);
        }
        259 => {
            return RegfileExit::fault(0x70008e14b4);
        }
        260 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e14bb); }
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e14bd); };
            c.w64(8, t255);
            let Some(t256) = c.ld32(c.r64(8).wrapping_add(0x3c7)) else { return RegfileExit::fault(0x70008e14c4); };
            c.w32(8, t256);
            let r257 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r257);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e14ce); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e14d0); }
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e14d1); };
            c.w64(3, t258);
            c.w32(0, 0x3u64);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e14dd); };
            c.w64(6, t259);
            return c.call(0x700086ed74, 0x70008e14e6, 0x70008e14e1);
        }
        261 => {
            return RegfileExit::fault(0x70008e14e6);
        }
        262 => {
            blk = 177; continue;
        }
        263 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008e14ec); };
            c.w64(3, t260);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008e14fc, 0x70008e14f7);
        }
        264 => {
            return RegfileExit::fault(0x70008e14fc);
        }
        265 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1503); }
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e1505); };
            c.w64(8, t261);
            let Some(t262) = c.ld32(c.r64(8).wrapping_add(0x3c7)) else { return RegfileExit::fault(0x70008e150c); };
            c.w32(8, t262);
            let r263 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r263);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1516); }
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e1518); };
            c.w64(3, t264);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e1529, 0x70008e1524);
        }
        266 => {
            return RegfileExit::fault(0x70008e1529);
        }
        267 => {
            c.w32(12, c.r32(12));
            let r265 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r265);
            c.w64(15, 0xfffffffffffc0000u64);
            let r266 = c.r64(15) & c.r64(12);
            c.w64(15, r266);
            c.logic(r266, 64);
            let Some(t267) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e153a); };
            c.logic(t267 & 0x2u64, 8);
            if c.cc_e() { blk = 74; continue; }
            blk = 268; continue;
        }
        268 => {
            c.w64(3, c.r64(7).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1549); }
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008e1553, 0x70008e154e);
        }
        269 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008e1553); }
            blk = 74; continue;
        }
        270 => {
            return RegfileExit::jump(0x70008e155c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e1574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e1574); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e1578); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e1579); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e157a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e157f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e1583); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e159b); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008e15a0); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e15ae); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e15b5); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 61; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e15c2); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e15c9); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e15ea); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e15f0); };
            c.w64(0, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008e15f7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e1603); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e1609); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e1619); };
            c.w64(1, t14);
            blk = 12; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e161f); };
            c.w64(1, t15);
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e1626); };
            c.w32(8, t16);
            c.w64(11, c.r64(8));
            let r17 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            let r18 = c.r32(8) & 0xfcu64;
            c.w32(8, r18);
            c.logic(r18, 32);
            let r19 = c.r32(11) & 0x3fu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008e163d); }
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(8, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(8, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(8, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(8, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(8, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e16bd);
        }
        30 => {
            c.w32(8, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(8, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(8, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(8, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(8, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(8, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            let Some(t20) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e16ec); };
            c.w64(11, t20);
            let r21 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008e16f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008e16f8); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008e16ff); }
            blk = 38; continue;
        }
        37 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1705); };
            c.w64(15, t22);
            let r23 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e170d); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1711); };
            c.w64(12, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e1715); };
            c.w64(9, t26);
            blk = 38; continue;
        }
        38 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e1719); };
            c.cmp(c.r64(15), t27, 64);
            if c.cc_ae() { blk = 53; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e1723); };
            c.w32(3, t28);
            let r29 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r29);
            let Some(t30) = c.ld8(c.r64(3).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e172a); };
            c.logic(t30 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008e1734); }
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(15));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e173e); };
            return c.call(t31, 0x70008e1746, 0x70008e173e);
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008e1746); }
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e174a); };
            c.cmp(t32, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1758); };
            c.w64(15, t33);
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            blk = 43; continue;
        }
        43 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e1760); };
            if !c.push64(t34) { return RegfileExit::fault(0x70008e1760); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1763); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e1764); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e1765); };
            if !c.push64(t35) { return RegfileExit::fault(0x70008e1765); }
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e1768); };
            c.w64(6, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e176c); };
            c.w64(7, t37);
            c.w32(0, 0x3u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008e177d, 0x70008e1778);
        }
        44 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e177d); };
            c.cmp(t38, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e1790); };
            c.cmp(t39, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e179d); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            let Some(t42) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e17a3); };
            c.logic(t42 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e17ad); };
            c.cmp(t43, c.r32(1), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t44) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e17ba); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 74; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 52; continue;
        }
        51 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e17d0); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e17d4); };
            c.w64(1, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e17db); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e17dc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e17e2); }
            return c.ret(0x0, 0x70008e17e4);
        }
        53 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e17e5); };
            c.w64(0, t47);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e17ef); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e17f0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e17f6); }
            return c.ret(0x0, 0x70008e17f8);
        }
        54 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t48) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e17ff); };
            c.w64(1, t48);
            blk = 57; continue;
        }
        56 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1805); };
            c.w64(1, t49);
            blk = 57; continue;
        }
        57 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1812); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1814); }
            c.w32(0, 0x2u64);
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e181a); };
            c.w64(3, t50);
            return c.call(0x700086ed74, 0x70008e1826, 0x70008e1821);
        }
        58 => {
            return RegfileExit::fault(0x70008e1826);
        }
        59 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e182c); }
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e182d); };
            c.w64(1, t51);
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0x3cb)) else { return RegfileExit::fault(0x70008e1834); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e183d); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e183e); };
            c.w64(3, t54);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e184f, 0x70008e184a);
        }
        60 => {
            return RegfileExit::fault(0x70008e184f);
        }
        61 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1855); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e1856); };
            c.w64(1, t55);
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0x3cb)) else { return RegfileExit::fault(0x70008e185d); };
            c.w32(1, t56);
            let r57 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r57);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1866); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e1867); };
            c.w64(3, t58);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e1878, 0x70008e1873);
        }
        62 => {
            return RegfileExit::fault(0x70008e1878);
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e1879); }
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e187d); };
            c.w64(3, t59);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e188e, 0x70008e1889);
        }
        64 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e188e); };
            c.w64(6, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1892); };
            c.w64(0, t61);
            blk = 1; continue;
        }
        65 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e189b); };
            c.w64(1, t62);
            let Some(t63) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e18a2); };
            c.w64(8, t63);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e18a9); };
            c.w64(11, t64);
            let Some(t65) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e18b0); };
            c.cmp(t65, c.r64(9), 64);
            if c.cc_a() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e18bf, 0x70008e18ba);
        }
        67 => {
            c.w64(1, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e18c2); };
            c.w64(0, t66);
            blk = 69; continue;
        }
        68 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e18c8); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 69; continue;
        }
        69 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e18cf); };
            c.w64(8, t67);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e18d6); }
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e18da); };
            c.w64(10, t68);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 73; continue; }
            blk = 70; continue;
        }
        70 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let r69 = c.r64(10) | 0x1u64;
            c.w64(10, r69);
            c.logic(r69, 64);
            blk = 72; continue;
        }
        72 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 73; continue;
        }
        73 => {
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e1902); }
            blk = 43; continue;
        }
        74 => {
            let Some(t70) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e190c); };
            c.logic(t70 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 52; continue;
        }
        76 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e191e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 52; continue; }
            blk = 77; continue;
        }
        77 => {
            blk = 37; continue;
        }
        78 => {
            return RegfileExit::jump(0x70008e1938);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e1954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e1954); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e1958); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e1959); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e195a); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e195f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e1963); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 63; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e197b); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008e1980); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e198e); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e1995); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 61; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e19a2); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e19a9); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e19ca); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e19d0); };
            c.w64(0, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008e19d7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e19e3); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e19e9); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e19f9); };
            c.w64(1, t14);
            blk = 12; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e19ff); };
            c.w64(1, t15);
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e1a06); };
            c.w32(8, t16);
            c.w64(11, c.r64(8));
            let r17 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            let r18 = c.r32(8) & 0xfcu64;
            c.w32(8, r18);
            c.logic(r18, 32);
            let r19 = c.r32(11) & 0x3fu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008e1a1d); }
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(8, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(8, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(8, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(8, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(8, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e1a9d);
        }
        30 => {
            c.w32(8, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(8, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(8, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(8, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(8, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(8, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            let Some(t20) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e1acc); };
            c.w64(11, t20);
            let r21 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008e1ad4); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(11)) { return RegfileExit::fault(0x70008e1ad8); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008e1adf); }
            blk = 38; continue;
        }
        37 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1ae5); };
            c.w64(15, t22);
            let r23 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e1aed); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1af1); };
            c.w64(12, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e1af5); };
            c.w64(9, t26);
            blk = 38; continue;
        }
        38 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e1af9); };
            c.cmp(c.r64(15), t27, 64);
            if c.cc_ae() { blk = 53; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e1b03); };
            c.w32(3, t28);
            let r29 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r29);
            let Some(t30) = c.ld8(c.r64(3).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e1b0a); };
            c.logic(t30 & 0x4u64, 8);
            if c.cc_ne() { blk = 53; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008e1b14); }
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(15));
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e1b1e); };
            return c.call(t31, 0x70008e1b26, 0x70008e1b1e);
        }
        41 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1b26); };
            c.cmp(t32, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1b34); };
            c.w64(15, t33);
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            blk = 43; continue;
        }
        43 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e1b3c); };
            if !c.push64(t34) { return RegfileExit::fault(0x70008e1b3c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1b3f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e1b40); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e1b41); };
            if !c.push64(t35) { return RegfileExit::fault(0x70008e1b41); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e1b44); }
            c.w32(0, 0x3u64);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e1b4d); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e1b51); };
            c.w64(6, t37);
            return c.call(0x70007f2274, 0x70008e1b5a, 0x70008e1b55);
        }
        44 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e1b5a); };
            c.cmp(t38, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(1, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t39) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e1b69); };
            c.cmp(t39, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t40) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e1b76); };
            c.w32(1, t40);
            let r41 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r41);
            let Some(t42) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e1b7c); };
            c.logic(t42 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e1b86); };
            c.cmp(t43, c.r32(1), 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t44) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e1b93); };
            c.cmp(t44, 0x41u64, 16);
            if c.cc_e() { blk = 74; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 52; continue;
        }
        51 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e1ba9); };
            c.w64(0, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1bad); };
            c.w64(1, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e1bb4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e1bb5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e1bbb); }
            return c.ret(0x0, 0x70008e1bbd);
        }
        53 => {
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e1bc8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e1bc9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(12).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e1bcf); }
            return c.ret(0x0, 0x70008e1bd1);
        }
        54 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t47) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e1bd8); };
            c.w64(1, t47);
            blk = 57; continue;
        }
        56 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1bde); };
            c.w64(1, t48);
            blk = 57; continue;
        }
        57 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1beb); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1bed); }
            c.w32(0, 0x2u64);
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e1bf3); };
            c.w64(3, t49);
            return c.call(0x700086ed74, 0x70008e1bff, 0x70008e1bfa);
        }
        58 => {
            return RegfileExit::fault(0x70008e1bff);
        }
        59 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1c05); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e1c06); };
            c.w64(1, t50);
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x3cf)) else { return RegfileExit::fault(0x70008e1c0d); };
            c.w32(1, t51);
            let r52 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r52);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1c16); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e1c17); };
            c.w64(3, t53);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e1c28, 0x70008e1c23);
        }
        60 => {
            return RegfileExit::fault(0x70008e1c28);
        }
        61 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1c2e); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e1c2f); };
            c.w64(1, t54);
            let Some(t55) = c.ld32(c.r64(1).wrapping_add(0x3cf)) else { return RegfileExit::fault(0x70008e1c36); };
            c.w32(1, t55);
            let r56 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r56);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e1c3f); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e1c40); };
            c.w64(3, t57);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e1c51, 0x70008e1c4c);
        }
        62 => {
            return RegfileExit::fault(0x70008e1c51);
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e1c52); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e1c56); };
            c.w64(3, t58);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e1c67, 0x70008e1c62);
        }
        64 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e1c67); };
            c.w64(6, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1c6b); };
            c.w64(0, t60);
            blk = 1; continue;
        }
        65 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e1c74); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e1c7b); };
            c.w64(8, t62);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008e1c82); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e1c86); };
            c.w64(11, t63);
            let Some(t64) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e1c8d); };
            c.cmp(t64, c.r64(9), 64);
            if c.cc_a() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e1c9c, 0x70008e1c97);
        }
        67 => {
            c.w64(1, c.r64(0));
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e1c9f); };
            c.w64(0, t65);
            blk = 69; continue;
        }
        68 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e1ca5); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 69; continue;
        }
        69 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e1cac); };
            c.w64(8, t66);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e1cb3); }
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1cb7); };
            c.w64(10, t67);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 73; continue; }
            blk = 70; continue;
        }
        70 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let r68 = c.r64(10) | 0x1u64;
            c.w64(10, r68);
            c.logic(r68, 64);
            blk = 72; continue;
        }
        72 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 73; continue;
        }
        73 => {
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e1cdf); }
            blk = 43; continue;
        }
        74 => {
            let Some(t69) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e1ce9); };
            c.logic(t69 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 52; continue;
        }
        76 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e1cfb); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 52; continue; }
            blk = 77; continue;
        }
        77 => {
            blk = 37; continue;
        }
        78 => {
            return RegfileExit::jump(0x70008e1d14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e1d34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e1d34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e1d38); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008e1d3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e1d42); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008e1d46); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008e1d4a); }
            blk = 2; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e1d50); };
            c.w64(6, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e1d54); };
            c.w64(3, t2);
            blk = 2; continue;
        }
        2 => {
            c.w32(9, c.r32(2));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1d66); };
            c.w64(8, t3);
            c.xorpd_rr(0, 0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008e1d71); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008e1d75); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e1d7f); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            blk = 18; continue;
        }
        5 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e1d98); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008e1d98); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008e1d9b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e1d9c); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e1d9e); };
            if !c.push64(t5) { return RegfileExit::fault(0x70008e1d9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008e1da1); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x70007f2274, 0x70008e1db5, 0x70008e1db0);
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e1db5); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e1dc4); };
            c.cmp(t7, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e1dcd); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e1dd3); };
            c.logic(t10 & 0x10u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e1dd9); };
            c.cmp(t11, c.r32(3), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e1de2); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 20; continue;
        }
        14 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t13) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e1df0); };
            c.cmp(t13, 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e1df6); };
            c.w64(3, t14);
            let r15 = c.sub(c.r32(3), 0x2u64, 32);
            c.w32(3, r15);
            if c.cc_o() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        18 => {
            c.w64(0, c.r64(8));
            blk = 19; continue;
        }
        19 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e1e0d); }
            return c.ret(0x0, 0x70008e1e0e);
        }
        20 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1e0f); };
            c.w64(0, t16);
            blk = 19; continue;
        }
        21 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e1e18); };
            c.logic(t17 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 20; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e1e23); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 20; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 15; continue;
        }
        25 => {
            let Some(t18) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e1e34); };
            c.cmp(t18, 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e1e3a); };
            c.w64(2, t19);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e1e3e); }
            blk = 28; continue;
        }
        27 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e1e45); };
            c.w64(2, t20);
            let r21 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r21);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 28; continue;
        }
        28 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008e1e71); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.movq_rx(1, 0);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(1, c.r64(3));
            let r22 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r22);
            if c.cc_o() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(2));
            blk = 39; continue;
        }
        34 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e1ea0); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e1ea7); };
            c.w64(1, t24);
            c.w64(8, c.r64(1).wrapping_add(0xc));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e1eae); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e1eb5); };
            c.cmp(t26, c.r64(8), 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e1ec4, 0x70008e1ebf);
        }
        36 => {
            blk = 38; continue;
        }
        37 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008e1ec6); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            blk = 38; continue;
        }
        38 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e1ed0); };
            c.w64(3, t27);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008e1ed7); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008e1eda); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e1edf); }
            blk = 39; continue;
        }
        39 => {
            c.w64(2, c.r64(0));
            blk = 1; continue;
        }
        40 => {
            return RegfileExit::jump(0x70008e1ef0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e1ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e1ef4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e1ef8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e1ef9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e1efa); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e1eff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e1f03); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e1f1b); };
            c.w64(11, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008e1f20); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e1f2e); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e1f35); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 66; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e1f43); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let Some(t8) = c.ld8(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e1f4a); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(15), 0x0u64, 64);
            c.w8(0, (c.cc_be() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e1f67); };
            c.w64(3, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1f6d); };
            c.w64(3, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008e1f74); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e1f81); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            let Some(t13) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e1f87); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e1f97); };
            c.w64(8, t14);
            blk = 12; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e1f9d); };
            c.w64(8, t15);
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e1fa4); };
            c.w32(9, t16);
            c.w64(12, c.r64(9));
            let r17 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r17);
            let r18 = c.r32(9) & 0xfcu64;
            c.w32(9, r18);
            c.logic(r18, 32);
            let r19 = c.r32(12) & 0x3fu64;
            c.w32(12, r19);
            c.logic(r19, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008e1fbc); }
            c.cmp(c.r32(12), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(9), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(9, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(9, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(9, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(9, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(9, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(9), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(9), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e203c);
        }
        30 => {
            c.w32(9, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(9, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(9, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(9, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(9, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(9, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            let Some(t20) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e206b); };
            c.w64(12, t20);
            let r21 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008e2073); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008e2077); }
            blk = 38; continue;
        }
        37 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e207d); };
            c.w64(12, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2081); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e2085); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e2089); };
            c.w64(9, t25);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2097); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            let r28 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r28);
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            c.w8(2, (c.cc_a() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008e20af); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008e20b3); }
            let Some(t29) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e20b7); };
            c.logic(t29 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(12));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e20c7); };
            return c.call(t30, 0x70008e20cf, 0x70008e20c7);
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008e20cf); }
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e20d3); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e20dd); };
            c.w64(12, t32);
            c.w32(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 43; continue;
        }
        43 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e20e5); };
            if !c.push64(t33) { return RegfileExit::fault(0x70008e20e5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e20e8); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e20ea); }
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e20eb); };
            if !c.push64(t34) { return RegfileExit::fault(0x70008e20eb); }
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e20ee); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e20f2); };
            c.w64(7, t36);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008e2103, 0x70008e20fe);
        }
        44 => {
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e2103); };
            c.cmp(t37, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e2117); };
            c.cmp(t38, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t39) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2124); };
            c.w32(8, t39);
            let r40 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r40);
            let Some(t41) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e212b); };
            c.logic(t41 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e2136); };
            c.cmp(t42, c.r32(8), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t43) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e2143); };
            c.cmp(t43, 0x41u64, 16);
            if c.cc_e() { blk = 79; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 54; continue;
        }
        51 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 54; continue;
        }
        53 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e215d); };
            c.w64(0, t44);
            c.w64(8, c.r64(15));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e216a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e216b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e2171); }
            return c.ret(0x0, 0x70008e2173);
        }
        54 => {
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2174); };
            c.w64(8, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e2178); };
            c.w64(0, t46);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e217f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e2180); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e2186); }
            return c.ret(0x0, 0x70008e2188);
        }
        55 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 57; continue;
        }
        57 => {
            c.w64(0, c.r64(11));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e2199); };
            c.w64(3, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e219d); };
            c.w64(1, t48);
            c.w64(2, c.r64(8));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e21a4); };
            c.w64(6, t49);
            return c.call(0x70008e1d34, 0x70008e21ad, 0x70008e21a8);
        }
        58 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e21ad); };
            c.w64(8, t50);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e21b4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e21b5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e21bb); }
            return c.ret(0x0, 0x70008e21bd);
        }
        59 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t51) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e21c3); };
            c.w64(8, t51);
            blk = 62; continue;
        }
        61 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e21c9); };
            c.w64(8, t52);
            blk = 62; continue;
        }
        62 => {
            c.w32(9, 0x32u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e21d6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e21d8); }
            c.w32(0, 0x2u64);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e21df); };
            c.w64(3, t53);
            return c.call(0x700086ed74, 0x70008e21eb, 0x70008e21e6);
        }
        63 => {
            return RegfileExit::fault(0x70008e21eb);
        }
        64 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e21f2); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e21f4); };
            c.w64(8, t54);
            let Some(t55) = c.ld32(c.r64(8).wrapping_add(0x3d3)) else { return RegfileExit::fault(0x70008e21fb); };
            c.w32(8, t55);
            let r56 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r56);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e2205); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2207); };
            c.w64(3, t57);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e2218, 0x70008e2213);
        }
        65 => {
            return RegfileExit::fault(0x70008e2218);
        }
        66 => {
            c.w32(8, 0xd6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e221f); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e2221); };
            c.w64(8, t58);
            let Some(t59) = c.ld32(c.r64(8).wrapping_add(0x3d3)) else { return RegfileExit::fault(0x70008e2228); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e2232); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2234); };
            c.w64(3, t61);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e2245, 0x70008e2240);
        }
        67 => {
            return RegfileExit::fault(0x70008e2245);
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e2246); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e224a); };
            c.w64(3, t62);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008e225b, 0x70008e2256);
        }
        69 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e225b); };
            c.w64(6, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e225f); };
            c.w64(0, t64);
            blk = 1; continue;
        }
        70 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e2268); };
            c.w64(8, t65);
            let Some(t66) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e226f); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e2272); };
            c.w64(10, t67);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 74; continue; }
            blk = 71; continue;
        }
        71 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let r68 = c.r64(10) | 0x1u64;
            c.w64(10, r68);
            c.logic(r68, 64);
            blk = 73; continue;
        }
        73 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 74; continue;
        }
        74 => {
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x70008e229e); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e22a3); };
            c.w64(12, t69);
            let Some(t70) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e22aa); };
            c.cmp(t70, c.r64(11), 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e22ba, 0x70008e22b5);
        }
        76 => {
            c.w64(8, c.r64(0));
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e22bd); };
            c.w64(0, t71);
            blk = 78; continue;
        }
        77 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e22c3); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 78; continue;
        }
        78 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e22ca); };
            c.w64(9, t72);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008e22d1); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70008e22d5); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e22da); }
            blk = 43; continue;
        }
        79 => {
            let Some(t73) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e22e5); };
            c.logic(t73 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 54; continue;
        }
        81 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e22f7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 54; continue; }
            blk = 82; continue;
        }
        82 => {
            blk = 37; continue;
        }
        83 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e230f); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e2316); };
            c.w64(9, t75);
            c.w64(0, c.r64(9).wrapping_add(0xc));
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e231d); };
            c.w64(3, t76);
            let Some(t77) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e2324); };
            c.cmp(t77, c.r64(0), 64);
            if c.cc_a() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e2333, 0x70008e232e);
        }
        85 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e2333); };
            c.w64(12, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2337); };
            c.w64(15, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e233b); };
            c.w64(11, t80);
            blk = 87; continue;
        }
        86 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008e2341); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 87; continue;
        }
        87 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 91; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            let r81 = c.r64(10) | 0x1u64;
            c.w64(10, r81);
            c.logic(r81, 64);
            blk = 90; continue;
        }
        90 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 91; continue;
        }
        91 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e2372); };
            c.w64(8, t82);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e2379); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e237d); }
            c.w64(8, c.r64(0));
            blk = 57; continue;
        }
        92 => {
            return RegfileExit::jump(0x70008e238c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e2394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e2394); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008e2398); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008e239e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008e23a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008e23a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008e23aa); }
            blk = 2; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e23b0); };
            c.w64(6, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e23b4); };
            c.w64(3, t2);
            blk = 2; continue;
        }
        2 => {
            c.w32(8, c.r32(2));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.xorpd_rr(0, 0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008e23ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008e23ce); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e23d8); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            blk = 18; continue;
        }
        5 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e23f1); };
            if !c.push64(t3) { return RegfileExit::fault(0x70008e23f1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008e23f4); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e23f5); };
            c.w64(11, t4);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e23fc); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e23fe); };
            if !c.push64(t5) { return RegfileExit::fault(0x70008e23fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008e2401); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x70007f2274, 0x70008e2415, 0x70008e2410);
        }
        7 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e2415); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(3, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e2424); };
            c.cmp(t7, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e242d); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e2433); };
            c.logic(t10 & 0x10u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t11) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e2439); };
            c.cmp(t11, c.r32(3), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e2442); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 20; continue;
        }
        14 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t13) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e2450); };
            c.cmp(t13, 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e2456); };
            c.w64(3, t14);
            let r15 = c.sub(c.r32(3), 0x2u64, 32);
            c.w32(3, r15);
            if c.cc_o() { blk = 25; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(2, (c.r32(3) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        18 => {
            c.w64(0, 0xfffffffffffffffeu64);
            blk = 19; continue;
        }
        19 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e2471); }
            return c.ret(0x0, 0x70008e2472);
        }
        20 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e2473); };
            c.w64(0, t16);
            blk = 19; continue;
        }
        21 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e2479); };
            c.logic(t17 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 20; continue;
        }
        23 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e2484); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 20; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 15; continue;
        }
        25 => {
            let Some(t18) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008e2495); };
            c.cmp(t18, 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e249b); };
            c.w64(0, t19);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e249f); }
            blk = 28; continue;
        }
        27 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008e24a6); };
            c.w64(0, t20);
            let r21 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r21);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 28; continue;
        }
        28 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(3, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008e24d2); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            if c.cc_ne() { blk = 34; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            c.movq_rx(1, 0);
            if let Some(sr) = c.shr(c.r64(1), 0x20u64, 64) { c.w64(1, sr); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_l() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(1, c.r64(3));
            let r22 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r22);
            if c.cc_o() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(2, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(2));
            blk = 39; continue;
        }
        34 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e2501); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e2508); };
            c.w64(1, t24);
            c.w64(8, c.r64(1).wrapping_add(0xc));
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e250f); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e2516); };
            c.cmp(t26, c.r64(8), 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e2525, 0x70008e2520);
        }
        36 => {
            blk = 38; continue;
        }
        37 => {
            if !c.st64(c.r64(3), c.r64(8)) { return RegfileExit::fault(0x70008e2527); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            c.w64(0, c.r64(3));
            blk = 38; continue;
        }
        38 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e2531); };
            c.w64(3, t27);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008e2538); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008e253b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e2540); }
            blk = 39; continue;
        }
        39 => {
            c.w64(2, c.r64(0));
            blk = 1; continue;
        }
        40 => {
            return RegfileExit::jump(0x70008e2550);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e2554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e2554); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e2558); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e2559); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e255a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e255f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e2563); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e257b); };
            c.w64(11, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008e2580); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e258e); };
            c.w32(12, t3);
            let r4 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r4);
            let Some(t5) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e2595); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 66; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e25a3); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let Some(t8) = c.ld8(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e25aa); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 64; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(15), 0x0u64, 64);
            c.w8(0, (c.cc_be() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e25c7); };
            c.w64(3, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e25cd); };
            c.w64(3, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008e25d4); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e25e1); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            let Some(t13) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e25e7); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e25f7); };
            c.w64(8, t14);
            blk = 12; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e25fd); };
            c.w64(8, t15);
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e2604); };
            c.w32(9, t16);
            c.w64(12, c.r64(9));
            let r17 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r17);
            let r18 = c.r32(9) & 0xfcu64;
            c.w32(9, r18);
            c.logic(r18, 32);
            let r19 = c.r32(12) & 0x3fu64;
            c.w32(12, r19);
            c.logic(r19, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008e261c); }
            c.cmp(c.r32(12), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(9), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(9, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(9, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(9, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(9, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(9, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(9), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(9), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e269c);
        }
        30 => {
            c.w32(9, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(9, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(9, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(9, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(9, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(9, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            let Some(t20) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e26cb); };
            c.w64(12, t20);
            let r21 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r21);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008e26d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008e26d7); }
            blk = 38; continue;
        }
        37 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e26dd); };
            c.w64(12, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e26e1); };
            c.w64(15, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e26e5); };
            c.w64(11, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e26e9); };
            c.w64(9, t25);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 53; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e26f7); };
            c.w32(0, t26);
            let r27 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r27);
            let r28 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r28);
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            c.w8(2, (c.cc_a() as u64));
            c.w32(2, c.r8(2));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008e270f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008e2713); }
            let Some(t29) = c.ld8(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e2717); };
            c.logic(t29 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(12));
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e2727); };
            return c.call(t30, 0x70008e272f, 0x70008e2727);
        }
        41 => {
            let Some(t31) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e272f); };
            c.cmp(t31, 0x0u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e2739); };
            c.w64(12, t32);
            c.w32(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 43; continue;
        }
        43 => {
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e2741); };
            if !c.push64(t33) { return RegfileExit::fault(0x70008e2741); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e2744); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e2746); }
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e2747); };
            if !c.push64(t34) { return RegfileExit::fault(0x70008e2747); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008e274a); }
            c.w32(0, 0x3u64);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e2753); };
            c.w64(7, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e2757); };
            c.w64(6, t36);
            return c.call(0x70007f2274, 0x70008e2760, 0x70008e275b);
        }
        44 => {
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008e2760); };
            c.cmp(t37, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008e2774); };
            c.cmp(t38, c.r32(0), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t39) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2781); };
            c.w32(8, t39);
            let r40 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r40);
            let Some(t41) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e2788); };
            c.logic(t41 & 0x10u64, 8);
            if c.cc_ne() { blk = 37; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e2793); };
            c.cmp(t42, c.r32(8), 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t43) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e27a0); };
            c.cmp(t43, 0x41u64, 16);
            if c.cc_e() { blk = 79; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 54; continue;
        }
        51 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 54; continue;
        }
        53 => {
            c.w64(0, 0xfffffffffffffffeu64);
            c.w64(8, c.r64(15));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e27c7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e27c8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e27ce); }
            return c.ret(0x0, 0x70008e27d0);
        }
        54 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e27d1); };
            c.w64(8, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e27d5); };
            c.w64(0, t45);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e27dc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e27dd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e27e3); }
            return c.ret(0x0, 0x70008e27e5);
        }
        55 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w32(8, c.r64(12).wrapping_add(c.r64(12)));
            blk = 57; continue;
        }
        57 => {
            c.w64(0, c.r64(11));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e27f6); };
            c.w64(3, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e27fa); };
            c.w64(1, t47);
            c.w64(2, c.r64(8));
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e2801); };
            c.w64(6, t48);
            return c.call(0x70008e2394, 0x70008e280a, 0x70008e2805);
        }
        58 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e280a); };
            c.w64(8, t49);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e2811); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e2812); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e2818); }
            return c.ret(0x0, 0x70008e281a);
        }
        59 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t50) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e2820); };
            c.w64(8, t50);
            blk = 62; continue;
        }
        61 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2826); };
            c.w64(8, t51);
            blk = 62; continue;
        }
        62 => {
            c.w32(9, 0x32u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008e2833); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e2835); }
            c.w32(0, 0x2u64);
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e283c); };
            c.w64(3, t52);
            return c.call(0x700086ed74, 0x70008e2848, 0x70008e2843);
        }
        63 => {
            return RegfileExit::fault(0x70008e2848);
        }
        64 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e284f); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e2851); };
            c.w64(8, t53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x3d7)) else { return RegfileExit::fault(0x70008e2858); };
            c.w32(8, t54);
            let r55 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r55);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e2862); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2864); };
            c.w64(3, t56);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e2875, 0x70008e2870);
        }
        65 => {
            return RegfileExit::fault(0x70008e2875);
        }
        66 => {
            c.w32(8, 0xd6u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e287c); }
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e287e); };
            c.w64(8, t57);
            let Some(t58) = c.ld32(c.r64(8).wrapping_add(0x3d7)) else { return RegfileExit::fault(0x70008e2885); };
            c.w32(8, t58);
            let r59 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r59);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e288f); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2891); };
            c.w64(3, t60);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e28a2, 0x70008e289d);
        }
        67 => {
            return RegfileExit::fault(0x70008e28a2);
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e28a3); }
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e28a7); };
            c.w64(3, t61);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008e28b8, 0x70008e28b3);
        }
        69 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e28b8); };
            c.w64(6, t62);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e28bc); };
            c.w64(0, t63);
            blk = 1; continue;
        }
        70 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e28c5); };
            c.w64(8, t64);
            let Some(t65) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e28cc); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e28cf); };
            c.w64(10, t66);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 74; continue; }
            blk = 71; continue;
        }
        71 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let r67 = c.r64(10) | 0x1u64;
            c.w64(10, r67);
            c.logic(r67, 64);
            blk = 73; continue;
        }
        73 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 74; continue;
        }
        74 => {
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008e28fb); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x50), 0) { return RegfileExit::fault(0x70008e28ff); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e2904); };
            c.w64(12, t68);
            let Some(t69) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e290b); };
            c.cmp(t69, c.r64(11), 64);
            if c.cc_a() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e291b, 0x70008e2916);
        }
        76 => {
            c.w64(8, c.r64(0));
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e291e); };
            c.w64(0, t70);
            blk = 78; continue;
        }
        77 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008e2924); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 78; continue;
        }
        78 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e292b); };
            c.w64(9, t71);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008e2932); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x50)) { return RegfileExit::fault(0x70008e2936); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e293b); }
            blk = 43; continue;
        }
        79 => {
            let Some(t72) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e2946); };
            c.logic(t72 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 54; continue;
        }
        81 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e2958); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 54; continue; }
            blk = 82; continue;
        }
        82 => {
            blk = 37; continue;
        }
        83 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e2970); };
            c.w64(8, t73);
            let Some(t74) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008e2977); };
            c.w64(9, t74);
            c.w64(0, c.r64(9).wrapping_add(0xc));
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e297e); };
            c.w64(3, t75);
            let Some(t76) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e2985); };
            c.cmp(t76, c.r64(0), 64);
            if c.cc_a() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e2994, 0x70008e298f);
        }
        85 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e2994); };
            c.w64(12, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2998); };
            c.w64(15, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e299c); };
            c.w64(11, t79);
            blk = 87; continue;
        }
        86 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008e29a2); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 87; continue;
        }
        87 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 91; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            let r80 = c.r64(10) | 0x1u64;
            c.w64(10, r80);
            c.logic(r80, 64);
            blk = 90; continue;
        }
        90 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 91; continue;
        }
        91 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e29d3); };
            c.w64(8, t81);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e29da); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e29de); }
            c.w64(8, c.r64(0));
            blk = 57; continue;
        }
        92 => {
            return RegfileExit::jump(0x70008e29ec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e29f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e29f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e29f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e29f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e29fa); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008e29ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e2a03); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 54; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(8, c.r64(1).wrapping_add(0x18));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e2a1b); };
            c.w64(9, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008e2a20); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2a2e); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e2a35); };
            c.cmp(t5, 0x82eu64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2a42); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e2a49); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 50; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(12), 0x0u64, 64);
            c.w8(15, (c.cc_be() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008e2a64); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e2a6e); };
            c.w64(0, t9);
            blk = 7; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2a74); };
            c.w64(0, t10);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008e2a7b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t11) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2a87); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            let Some(t13) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e2a8d); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e2a9d); };
            c.w64(1, t14);
            blk = 12; continue;
        }
        11 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2aa3); };
            c.w64(1, t15);
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e2aaa); };
            c.w32(8, t16);
            c.w64(11, c.r64(8));
            let r17 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r17);
            let r18 = c.r32(8) & 0xfcu64;
            c.w32(8, r18);
            c.logic(r18, 32);
            let r19 = c.r32(11) & 0x3fu64;
            c.w32(11, r19);
            c.logic(r19, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008e2ac1); }
            c.cmp(c.r32(11), 0x15u64, 32);
            if c.cc_g() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w32(8, 0x906u64);
            blk = 36; continue;
        }
        19 => {
            c.w32(8, 0x900u64);
            blk = 36; continue;
        }
        20 => {
            c.w32(8, 0x8fau64);
            blk = 36; continue;
        }
        21 => {
            c.w32(8, 0x8f4u64);
            blk = 36; continue;
        }
        22 => {
            c.w32(8, 0x8eeu64);
            blk = 36; continue;
        }
        23 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008e2b41);
        }
        30 => {
            c.w32(8, 0x8e8u64);
            blk = 36; continue;
        }
        31 => {
            c.w32(8, 0x8e2u64);
            blk = 36; continue;
        }
        32 => {
            c.w32(8, 0x8dcu64);
            blk = 36; continue;
        }
        33 => {
            c.w32(8, 0x8d6u64);
            blk = 36; continue;
        }
        34 => {
            c.w32(8, 0x8d0u64);
            blk = 36; continue;
        }
        35 => {
            c.w32(8, 0x8cau64);
            blk = 36; continue;
        }
        36 => {
            let Some(t20) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e2b70); };
            c.w64(11, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008e2b74); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008e2b7b); }
            blk = 39; continue;
        }
        37 => {
            let r21 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e2b85); };
            if !c.push64(t22) { return RegfileExit::fault(0x70008e2b85); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e2b88); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e2b89); }
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e2b8a); };
            if !c.push64(t23) { return RegfileExit::fault(0x70008e2b8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008e2b8d); }
            c.w32(0, 0x3u64);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e2b96); };
            c.w64(7, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e2b9a); };
            c.w64(6, t25);
            return c.call(0x70007f2274, 0x70008e2ba3, 0x70008e2b9e);
        }
        38 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e2ba3); };
            c.w64(15, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2ba7); };
            c.w64(12, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e2bab); };
            c.w64(9, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e2baf); };
            c.w64(8, t29);
            blk = 39; continue;
        }
        39 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e2bb3); };
            c.cmp(c.r64(15), t30, 64);
            if c.cc_ae() { blk = 44; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2bb9); };
            c.w32(3, t31);
            let r32 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r32);
            let Some(t33) = c.ld8(c.r64(3).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e2bc0); };
            c.logic(t33 & 0x4u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008e2bc6); }
            c.w64(0, c.r64(9));
            c.w64(3, c.r64(15));
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e2bd0); };
            return c.call(t34, 0x70008e2bd8, 0x70008e2bd0);
        }
        42 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e2bd8); };
            c.cmp(t35, 0x3fffffffu64, 64);
            if c.cc_a() { blk = 56; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e2be6); };
            c.w64(15, t36);
            c.w32(1, c.r64(15).wrapping_add(c.r64(15)));
            blk = 37; continue;
        }
        44 => {
            c.w64(1, c.r64(12));
            let r37 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2bf7); };
            c.w64(0, t38);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e2c01); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e2c02); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e2c08); }
            return c.ret(0x0, 0x70008e2c0a);
        }
        45 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t39) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e2c11); };
            c.w64(1, t39);
            blk = 48; continue;
        }
        47 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2c17); };
            c.w64(1, t40);
            blk = 48; continue;
        }
        48 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e2c24); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e2c26); }
            c.w32(0, 0x2u64);
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2c2c); };
            c.w64(3, t41);
            return c.call(0x700086ed74, 0x70008e2c38, 0x70008e2c33);
        }
        49 => {
            return RegfileExit::fault(0x70008e2c38);
        }
        50 => {
            c.w32(1, 0x66u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e2c3e); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e2c3f); };
            c.w64(1, t42);
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x3db)) else { return RegfileExit::fault(0x70008e2c46); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r44);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e2c4f); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2c50); };
            c.w64(3, t45);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e2c61, 0x70008e2c5c);
        }
        51 => {
            return RegfileExit::fault(0x70008e2c61);
        }
        52 => {
            c.w32(1, 0xd6u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e2c67); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e2c68); };
            c.w64(1, t46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x3db)) else { return RegfileExit::fault(0x70008e2c6f); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008e2c78); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2c79); };
            c.w64(3, t49);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e2c8a, 0x70008e2c85);
        }
        53 => {
            return RegfileExit::fault(0x70008e2c8a);
        }
        54 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008e2c8b); }
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e2c8f); };
            c.w64(3, t50);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008e2ca0, 0x70008e2c9b);
        }
        55 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e2ca0); };
            c.w64(6, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2ca4); };
            c.w64(0, t52);
            blk = 1; continue;
        }
        56 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e2cad); };
            c.w64(1, t53);
            let Some(t54) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008e2cb4); };
            c.w64(8, t54);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x70008e2cbb); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e2cbf); };
            c.w64(11, t55);
            let Some(t56) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008e2cc6); };
            c.cmp(t56, c.r64(9), 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e2cd5, 0x70008e2cd0);
        }
        58 => {
            c.w64(1, c.r64(0));
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e2cd8); };
            c.w64(0, t57);
            blk = 60; continue;
        }
        59 => {
            if !c.st64(c.r64(1), c.r64(9)) { return RegfileExit::fault(0x70008e2cde); }
            c.w64(1, c.r64(8).wrapping_add(0x1));
            blk = 60; continue;
        }
        60 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e2ce5); };
            c.w64(10, t58);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.logic(c.r64(10) & c.r64(10), 64);
            if c.cc_ns() { blk = 64; continue; }
            blk = 61; continue;
        }
        61 => {
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let r59 = c.r64(10) | 0x1u64;
            c.w64(10, r59);
            c.logic(r59, 64);
            blk = 63; continue;
        }
        63 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 64; continue;
        }
        64 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e2d0d); };
            c.w64(8, t60);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008e2d14); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e2d18); }
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e2d1d); };
            c.w64(15, t61);
            blk = 37; continue;
        }
        65 => {
            return RegfileExit::jump(0x70008e2d28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008e2d34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008e2d34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008e2d38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008e2d39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e2d3a); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008e2d3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008e2d43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 150; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e2d5b); };
            c.w64(11, t2);
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008e2d63); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008e2d67); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008e2d71); };
            c.w64(8, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2d77); };
            c.w64(8, t4);
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(8)) { return RegfileExit::fault(0x70008e2d7e); }
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008e2d88); };
            c.w64(15, t5);
            blk = 7; continue;
        }
        6 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2d8e); };
            c.w64(15, t6);
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008e2d95); }
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t7) = c.ld64(c.r64(9).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008e2d9f); };
            c.w64(9, t7);
            blk = 10; continue;
        }
        9 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2da5); };
            c.w64(9, t8);
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008e2dac); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2dba); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let Some(t11) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e2dc1); };
            c.logic(t11 & 0x40u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2dcb); };
            c.cmp(t12, c.r32(15), 32);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70008e2dd8); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t13) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2de7); };
            c.w32(3, t13);
            let r14 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r14);
            let Some(t15) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e2dee); };
            c.logic(t15 & 0x2u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w32(2, 0x1u64);
            blk = 17; continue;
        }
        16 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 17; continue;
        }
        17 => {
            c.w32(3, 0x1u64);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008e2e0b); };
            c.w64(3, t16);
            c.w64(0, c.r64(8));
            return c.call(0x7000870914, 0x70008e2e1a, 0x70008e2e15);
        }
        19 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e2e1a); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008e2e27); };
            c.cmp(t18, c.r32(0), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 21; continue;
        }
        21 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2e3c); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            let Some(t21) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e2e43); };
            c.logic(t21 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t22) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e2e4e); };
            c.cmp(t22, 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t23) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e2e58); };
            c.w32(8, t23);
            let r24 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r24);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 43; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70008e2e71); };
            c.w64(9, t25);
            let Some(t26) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2e78); };
            c.cmp(t26, c.r32(8), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t27) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e2e82); };
            c.logic(t27 & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e2e8c); };
            c.w64(9, t28);
            let Some(t29) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2e90); };
            c.w32(11, t29);
            let r30 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r30);
            let Some(t31) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e2e97); };
            c.cmp(t31, 0x834u64, 16);
            if c.cc_ne() { blk = 35; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2ea0); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            c.w32(12, 0x19au64);
            let Some(t34) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e2ead); };
            c.cmp(t34, c.r32(12), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t35) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2eb7); };
            c.w32(11, t35);
            let r36 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r36);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 90; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e2ecf); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(10, 0x8000000000000000u64);
            let r37 = c.r64(11) | c.r64(10);
            c.w64(11, r37);
            c.logic(r37, 64);
            blk = 34; continue;
        }
        34 => {
            blk = 90; continue;
        }
        35 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        36 => {
            c.w32(11, 0x1u64);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e2f1a); };
            c.w64(9, t38);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t39) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e2f24); };
            c.w32(11, t39);
            let r40 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r40);
            let Some(t41) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e2f2b); };
            c.cmp(t41, 0x82eu64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t42) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2f34); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            let Some(t44) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e2f3b); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2f42); };
            c.w32(11, t45);
            let r46 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r46);
            c.w32(12, 0x7f0u64);
            let Some(t47) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e2f4f); };
            c.cmp(t47, c.r32(12), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t48) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e2f55); };
            c.w64(11, t48);
            blk = 90; continue;
        }
        42 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e2f5e); };
            c.w64(9, t49);
            blk = 45; continue;
        }
        43 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e2f64); };
            c.w64(9, t50);
            blk = 45; continue;
        }
        44 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e2f6a); };
            c.w64(9, t51);
            blk = 45; continue;
        }
        45 => {
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(9));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2f74); };
            c.w64(6, t52);
            return c.call(0x7000844d94, 0x70008e2f7d, 0x70008e2f78);
        }
        46 => {
            let Some(t53) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e2f7d); };
            c.w32(8, t53);
            let r54 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r54);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 52; continue;
        }
        48 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e2f92); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t55) = c.ld64(0x70008e2ee1u64) else { return RegfileExit::fault(0x70008e2fa2); };
            c.w64(10, t55);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t56) = c.ld64(0x70008e2effu64) else { return RegfileExit::fault(0x70008e2fbd); };
            c.w64(10, t56);
            let r57 = c.r64(11) | c.r64(10);
            c.w64(11, r57);
            c.logic(r57, 64);
            blk = 51; continue;
        }
        51 => {
            c.w64(8, c.r64(11));
            blk = 52; continue;
        }
        52 => {
            c.w64(11, c.r64(8));
            c.w64(9, c.r64(0));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 90; continue;
        }
        53 => {
            c.w32(8, 0x90u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e2fde); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e2fe0); };
            c.w64(3, t58);
            c.w32(0, 0x1u64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e2fec); };
            c.w64(6, t59);
            return c.call(0x700086ed74, 0x70008e2ff5, 0x70008e2ff0);
        }
        54 => {
            return RegfileExit::fault(0x70008e2ff5);
        }
        55 => {
            let Some(t60) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e2ff6); };
            c.logic(t60 & 0x1u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e3000); };
            c.w64(0, t61);
            let Some(t62) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3004); };
            c.w32(8, t62);
            let r63 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r63);
            let Some(t64) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e300b); };
            c.cmp(t64, 0xbau64, 16);
            if c.cc_b() { blk = 153; continue; }
            blk = 57; continue;
        }
        57 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x70008e3022); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 61; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e302c); };
            c.w32(9, t65);
            let r66 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r66);
            let Some(t67) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e3033); };
            c.cmp(t67, 0x834u64, 16);
            if c.cc_ne() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t68) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e303c); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3043); };
            c.w64(6, t70);
            blk = 83; continue;
        }
        60 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 62; continue;
        }
        61 => {
            c.w32(9, 0x1u64);
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e305d); };
            c.w64(6, t71);
            let Some(t72) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3061); };
            c.w32(9, t72);
            let r73 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r73);
            let Some(t74) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e3068); };
            c.w32(9, t74);
            let r75 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r75);
            let Some(t76) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e306f); };
            c.w32(11, t76);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t77) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3076); };
            c.w32(12, t77);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xb0u64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x70008e3086);
        }
        65 => {
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008e3087); };
            c.cmp(t78, c.r32(12), 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 66; continue;
        }
        66 => {
            blk = 155; continue;
        }
        67 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3095); };
            c.w64(6, t79);
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t80) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e30a3); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008e30aa); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            let Some(t84) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008e30b1); };
            c.w32(9, t84);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t85) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e30b8); };
            c.w32(11, t85);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x44u64, 64);
            if c.cc_a() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            return RegfileExit::fault(0x70008e30c5);
        }
        71 => {
            let Some(t86) = c.ld32(c.r64(8).wrapping_add(0x117)) else { return RegfileExit::fault(0x70008e30c6); };
            c.cmp(t86, c.r32(11), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r64(9), 0xaeu64, 64);
            if c.cc_a() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            return RegfileExit::fault(0x70008e30d8);
        }
        74 => {
            let Some(t87) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008e30d9); };
            c.cmp(t87, c.r32(11), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r64(9), 0xafu64, 64);
            if c.cc_a() { blk = 156; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x70008e30ef);
        }
        77 => {
            let Some(t88) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e30f0); };
            c.w32(8, t88);
            let r89 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r89);
            blk = 78; continue;
        }
        78 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a17f4, 0x70008e3105, 0x70008e3100);
        }
        80 => {
            c.w64(8, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3108); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e310c); };
            c.w64(0, t91);
            blk = 83; continue;
        }
        81 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ge() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 83; continue;
        }
        83 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 89; continue;
        }
        85 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e3129); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t92) = c.ld64(0x70008e2ee1u64) else { return RegfileExit::fault(0x70008e3139); };
            c.w64(10, t92);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t93) = c.ld64(0x70008e2effu64) else { return RegfileExit::fault(0x70008e3154); };
            c.w64(10, t93);
            let r94 = c.r64(11) | c.r64(10);
            c.w64(11, r94);
            c.logic(r94, 64);
            blk = 88; continue;
        }
        88 => {
            c.w64(8, c.r64(11));
            blk = 89; continue;
        }
        89 => {
            c.w64(11, c.r64(8));
            c.w64(9, c.r64(0));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 90; continue;
        }
        90 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008e316a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008e316e); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 159; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w32(12, c.r64(11).wrapping_add(c.r64(11)));
            blk = 92; continue;
        }
        92 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008e3183); }
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008e3185); };
            c.w64(15, t95);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008e318c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008e318e); }
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3192); };
            c.w64(6, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e3196); };
            c.w64(2, t97);
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(2));
            return c.call(0x70007f7214, 0x70008e31a7, 0x70008e31a2);
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008e31a7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t98) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e31b3); };
            c.w32(8, t98);
            let r99 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r99);
            let Some(t100) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008e31ba); };
            c.cmp(t100, 0x82eu64, 16);
            if c.cc_ne() { blk = 181; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t101) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e31c7); };
            c.w32(9, t101);
            let r102 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r102);
            let Some(t103) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008e31ce); };
            c.logic(t103 & 0x4u64, 8);
            if c.cc_ne() { blk = 179; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t104) = c.ld32(c.r64(0).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008e31d9); };
            c.w32(9, t104);
            c.w64(11, c.r64(9));
            let r105 = c.r32(11) & 0xfffffffffffffffeu64;
            c.w32(11, r105);
            c.logic(r105, 32);
            let r106 = c.r32(9) & 0xfffffffffffffffcu64;
            c.w32(9, r106);
            c.logic(r106, 32);
            if !c.st32(c.r64(0).wrapping_add(0x33), c.r32(11)) { return RegfileExit::fault(0x70008e31e8); }
            if !c.st32(c.r64(0).wrapping_add(0x33), c.r32(9)) { return RegfileExit::fault(0x70008e31ec); }
            let Some(t107) = c.ld64(c.r64(0).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008e31f0); };
            c.w64(9, t107);
            let Some(t108) = c.ld8(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e31f4); };
            c.logic(t108 & 0x1u64, 8);
            if c.cc_ne() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t109) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e31fa); };
            c.w64(11, (t109 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e3201); };
            c.w64(12, t110);
            blk = 101; continue;
        }
        98 => {
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e3207); };
            c.w64(12, t111);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008e320b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 101; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t112) = c.ld64(0x70008e2ee1u64) else { return RegfileExit::fault(0x70008e321c); };
            c.w64(10, t112);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t113) = c.ld64(0x70008e2effu64) else { return RegfileExit::fault(0x70008e3237); };
            c.w64(10, t113);
            let r114 = c.r64(11) | c.r64(10);
            c.w64(11, r114);
            c.logic(r114, 64);
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 177; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t115) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e324a); };
            c.cmp(t115, 0x0u64, 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t116) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e3254); };
            c.logic(t116 & 0x1u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e325e); };
            c.w64(15, t117);
            let Some(t118) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008e3262); };
            c.w32(9, t118);
            let r119 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r119);
            let Some(t120) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008e3269); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t121) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008e3274); };
            c.w32(8, t121);
            c.w64(9, c.r64(8));
            let r122 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r122);
            let r123 = c.r32(8) & 0xfcu64;
            c.w32(8, r123);
            c.logic(r123, 32);
            let r124 = c.r32(9) & 0x3fu64;
            c.w32(9, r124);
            c.logic(r124, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_g() { blk = 116; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(8), 0x44u64, 32);
            if c.cc_e() { blk = 115; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(8), 0x48u64, 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(8), 0x4cu64, 32);
            if c.cc_e() { blk = 113; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(8), 0x50u64, 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(8), 0x54u64, 32);
            if c.cc_ne() { blk = 122; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w32(8, 0x90au64);
            blk = 129; continue;
        }
        112 => {
            c.w32(8, 0x904u64);
            blk = 129; continue;
        }
        113 => {
            c.w32(8, 0x8feu64);
            blk = 129; continue;
        }
        114 => {
            c.w32(8, 0x8f8u64);
            blk = 129; continue;
        }
        115 => {
            c.w32(8, 0x8f2u64);
            blk = 129; continue;
        }
        116 => {
            c.cmp(c.r32(8), 0x58u64, 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(8), 0x5cu64, 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(8), 0x60u64, 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(8), 0x64u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(8), 0x68u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(8), 0x6cu64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            return RegfileExit::fault(0x70008e3307);
        }
        123 => {
            c.w32(8, 0x8ecu64);
            blk = 129; continue;
        }
        124 => {
            c.w32(8, 0x8e6u64);
            blk = 129; continue;
        }
        125 => {
            c.w32(8, 0x8e0u64);
            blk = 129; continue;
        }
        126 => {
            c.w32(8, 0x8dau64);
            blk = 129; continue;
        }
        127 => {
            c.w32(8, 0x8d4u64);
            blk = 129; continue;
        }
        128 => {
            c.w32(8, 0x8ceu64);
            blk = 129; continue;
        }
        129 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008e3336); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 131; continue;
        }
        130 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e333e); };
            c.w64(8, t125);
            c.w64(3, c.r64(8).wrapping_add(0x1));
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e3346); };
            c.w64(8, t126);
            blk = 131; continue;
        }
        131 => {
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e334a); };
            c.cmp(c.r64(3), t127, 64);
            if c.cc_ae() { blk = 140; continue; }
            blk = 132; continue;
        }
        132 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008e3350); }
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 168; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w32(9, c.r64(3).wrapping_add(c.r64(3)));
            blk = 134; continue;
        }
        134 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008e3365); }
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e3369); };
            c.w64(0, t128);
            c.w64(3, c.r64(9));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3370); };
            c.w64(6, t129);
            return c.call(0x7000870914, 0x70008e3379, 0x70008e3374);
        }
        135 => {
            let Some(t130) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008e3379); };
            c.cmp(t130, 0x0u64, 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008e337f); };
            if !c.push64(t131) { return RegfileExit::fault(0x70008e337f); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e3382); }
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008e3383); };
            if !c.push64(t132) { return RegfileExit::fault(0x70008e3383); }
            c.w32(0, 0x2u64);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008e338b); };
            c.w64(7, t133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e338f); };
            c.w64(6, t134);
            return c.call(0x70007f2274, 0x70008e3398, 0x70008e3393);
        }
        137 => {
            c.w64(1, c.r64(0));
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e339b); };
            c.w64(0, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e339f); };
            c.w64(3, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e33a3); };
            c.w64(6, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e33a7); };
            c.w64(8, t138);
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008e33ab); };
            return c.call(t139, 0x70008e33b3, 0x70008e33ab);
        }
        138 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 130; continue;
        }
        140 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e33ba); };
            c.w64(8, t140);
            let r141 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e33c2); };
            c.w64(0, t142);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e33c9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e33ca); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e33d0); }
            return c.ret(0x0, 0x70008e33d2);
        }
        141 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e33d9); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e33db); };
            c.w64(8, t143);
            let Some(t144) = c.ld32(c.r64(8).wrapping_add(0x3df)) else { return RegfileExit::fault(0x70008e33e2); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e33ec); }
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e33ee); };
            c.w64(3, t146);
            c.w32(0, 0x2u64);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e33fa); };
            c.w64(6, t147);
            return c.call(0x700086ed74, 0x70008e3403, 0x70008e33fe);
        }
        142 => {
            return RegfileExit::fault(0x70008e3403);
        }
        143 => {
            return RegfileExit::fault(0x70008e3404);
        }
        144 => {
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e3405); };
            c.cmp(t148, 0x0u64, 64);
            if c.cc_e() { blk = 147; continue; }
            blk = 145; continue;
        }
        145 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e340c); }
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e340d); };
            if !c.push64(t149) { return RegfileExit::fault(0x70008e340d); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008e3410); }
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2b20)) else { return RegfileExit::fault(0x70008e3412); };
            c.w64(3, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3419); };
            c.w64(6, t151);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008e342a, 0x70008e3425);
        }
        146 => {
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e342a); };
            c.w64(0, t152);
            blk = 147; continue;
        }
        147 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008e342e); };
            c.w64(8, t153);
            let r154 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r154);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008e3439); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008e343a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008e3440); }
            return c.ret(0x0, 0x70008e3442);
        }
        148 => {
            c.w32(8, 0xc2u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3449); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008e344b); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e344d); };
            c.w64(3, t155);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e345e, 0x70008e3459);
        }
        149 => {
            return RegfileExit::fault(0x70008e345e);
        }
        150 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008e345f); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008e3465); };
            c.w64(3, t156);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008e3474, 0x70008e346f);
        }
        151 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3474); };
            c.w64(6, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008e3478); };
            c.w64(0, t158);
            blk = 1; continue;
        }
        152 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e3481); };
            c.w64(0, t159);
            blk = 153; continue;
        }
        153 => {
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3485); };
            c.w64(6, t160);
            return c.call(0x70008a2054, 0x70008e348e, 0x70008e3489);
        }
        154 => {
            blk = 57; continue;
        }
        155 => {
            let Some(t161) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008e3493); };
            c.w32(8, t161);
            let r162 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r162);
            blk = 78; continue;
        }
        156 => {
            let Some(t163) = c.ld32(c.r64(8).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x70008e349f); };
            c.cmp(t163, c.r32(11), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008e34ac); };
            c.w64(3, t164);
            return c.call(0x7000870914, 0x70008e34b8, 0x70008e34b3);
        }
        158 => {
            c.w64(8, c.r64(0));
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e34bb); };
            c.w64(6, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e34bf); };
            c.w64(0, t166);
            blk = 78; continue;
        }
        159 => {
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e34c8); };
            c.w64(12, t167);
            let Some(t168) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008e34cf); };
            c.w64(15, t168);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e34d7); };
            c.w64(3, t169);
            let Some(t170) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008e34de); };
            c.cmp(t170, c.r64(0), 64);
            if c.cc_a() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e34ed, 0x70008e34e8);
        }
        161 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008e34f0); };
            c.w64(9, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008e34f4); };
            c.w64(11, t172);
            blk = 163; continue;
        }
        162 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008e34fa); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 163; continue;
        }
        163 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 167; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            let r173 = c.r64(10) | 0x1u64;
            c.w64(10, r173);
            c.logic(r173, 64);
            blk = 166; continue;
        }
        166 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 167; continue;
        }
        167 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e352c); };
            c.w64(12, t174);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008e3533); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e3537); }
            c.w64(12, c.r64(0));
            blk = 92; continue;
        }
        168 => {
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008e3544); };
            c.w64(9, t175);
            let Some(t176) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008e354b); };
            c.w64(11, t176);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008e3552); };
            c.w64(2, t177);
            let Some(t178) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008e3559); };
            c.cmp(t178, c.r64(12), 64);
            if c.cc_a() { blk = 171; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008e3568, 0x70008e3563);
        }
        170 => {
            c.w64(9, c.r64(0));
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008e356b); };
            c.w64(3, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008e356f); };
            c.w64(8, t180);
            blk = 172; continue;
        }
        171 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008e3575); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 172; continue;
        }
        172 => {
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008e357c); };
            c.w64(11, t181);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008e3583); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 176; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            let r182 = c.r64(10) | 0x1u64;
            c.w64(10, r182);
            c.logic(r182, 64);
            blk = 175; continue;
        }
        175 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 176; continue;
        }
        176 => {
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008e35ae); }
            blk = 134; continue;
        }
        177 => {
            c.w32(8, 0x296u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e35bf); }
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e35c1); };
            c.w64(3, t183);
            c.w32(0, 0x1u64);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e35cd); };
            c.w64(6, t184);
            return c.call(0x700086ed74, 0x70008e35d6, 0x70008e35d1);
        }
        178 => {
            return RegfileExit::fault(0x70008e35d6);
        }
        179 => {
            c.w32(8, 0x66u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e35dd); }
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e35df); };
            c.w64(8, t185);
            let Some(t186) = c.ld32(c.r64(8).wrapping_add(0x3df)) else { return RegfileExit::fault(0x70008e35e6); };
            c.w32(8, t186);
            let r187 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r187);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e35f0); }
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e35f2); };
            c.w64(3, t188);
            c.w32(0, 0x2u64);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e35fe); };
            c.w64(6, t189);
            return c.call(0x700086ed74, 0x70008e3607, 0x70008e3602);
        }
        180 => {
            return RegfileExit::fault(0x70008e3607);
        }
        181 => {
            c.w32(8, 0x80u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e360e); }
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008e3610); };
            c.w64(8, t190);
            let Some(t191) = c.ld32(c.r64(8).wrapping_add(0x3df)) else { return RegfileExit::fault(0x70008e3617); };
            c.w32(8, t191);
            let r192 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r192);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3621); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008e3623); }
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3624); };
            c.w64(3, t193);
            c.w32(0, 0x3u64);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008e3630); };
            c.w64(6, t194);
            return c.call(0x700086ed74, 0x70008e3639, 0x70008e3634);
        }
        182 => {
            return RegfileExit::fault(0x70008e3639);
        }
        183 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008e3640); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008e3642); }
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008e3644); };
            c.w64(3, t195);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008e3655, 0x70008e3650);
        }
        184 => {
            return RegfileExit::fault(0x70008e3655);
        }
        185 => {
            return RegfileExit::jump(0x70008e3658);
        }
            _ => unreachable!(),
        }
    }
}
