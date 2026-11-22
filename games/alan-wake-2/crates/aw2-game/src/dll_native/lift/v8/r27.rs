#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008d2594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d2594); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d2598); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d2599); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d259a); }
            let r0 = c.sub(c.r64(4), 0x98u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008d25a2); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d25a6); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 620; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d25b0); };
            c.w64(0, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d25b4); };
            c.cmp(t3, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d25c3); };
            c.cmp(t4, c.r32(0), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r5 = c.r32(11) | c.r32(12);
            c.w32(11, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 618; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d25db); };
            c.w64(11, t6);
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008d25eb); }
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d25ef); };
            c.cmp(t7, c.r32(11), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d25fc); };
            c.cmp(t8, c.r32(11), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2613); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d261a); };
            c.w32(15, t11);
            let r12 = c.sub(c.r32(15), 0xbau64, 32);
            c.w32(15, r12);
            c.cmp(c.r32(15), 0x78eu64, 32);
            if c.cc_a() { blk = 55; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x70008d2633); };
            c.w64(3, t13);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x70008d2642, 0x70008d263d);
        }
        7 => {
            let Some(t14) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d2642); };
            c.cmp(t14, c.r32(0), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008d264f); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(11, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d2663); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2670); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            let Some(t19) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008d2677); };
            c.logic(t19 & 0x10u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d2682); };
            c.cmp(t20, c.r32(11), 32);
            if c.cc_e() { blk = 624; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d268f); };
            c.cmp(t21, 0x41u64, 16);
            if c.cc_e() { blk = 622; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 16; continue;
        }
        15 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d26a3); };
            c.w64(11, t22);
            let Some(t23) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d26a7); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d26ae); };
            c.cmp(t25, 0x83eu64, 16);
            if c.cc_ne() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(12, 0x1u64);
            blk = 19; continue;
        }
        18 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        19 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w32(12, 0x90u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008d26d8); }
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008d26da); };
            c.w64(3, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d26e1); };
            c.w64(6, t27);
            return c.call(0x700086ed74, 0x70008d26ea, 0x70008d26e5);
        }
        21 => {
            c.w32(0, 0x1u64);
            let Some(t28) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d26ef); };
            c.w64(11, t28);
            blk = 31; continue;
        }
        22 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d26f5); };
            c.w64(11, t29);
            let Some(t30) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d26f9); };
            c.w32(12, t30);
            let r31 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r31);
            let Some(t32) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2700); };
            c.cmp(t32, 0x83eu64, 16);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(12, 0x92u64);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008d2710); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008d2712); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d271e); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x70008d2727, 0x70008d2722);
        }
        24 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d2727); };
            c.w64(11, t35);
            blk = 55; continue;
        }
        25 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d2730); };
            c.w64(0, t36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2734); };
            c.w32(11, t37);
            let r38 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r38);
            let Some(t39) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d273b); };
            c.cmp(t39, 0x83eu64, 16);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w32(11, 0x1u64);
            blk = 28; continue;
        }
        27 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w64(11, c.r64(0));
            blk = 55; continue;
        }
        30 => {
            c.w64(11, c.r64(0));
            c.w32(0, 0x1u64);
            blk = 31; continue;
        }
        31 => {
            let Some(t40) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2765); };
            c.w32(12, t40);
            let r41 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r41);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xfd8)) else { return RegfileExit::fault(0x70008d276e); };
            c.w64(15, t42);
            let Some(t43) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d2775); };
            c.cmp(t43, c.r32(1), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d277f); };
            c.w64(6, t44);
            let Some(t45) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2783); };
            c.w32(15, t45);
            let r46 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r46);
            let Some(t47) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d278a); };
            c.w32(15, t47);
            let r48 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r48);
            let Some(t49) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d2791); };
            c.w32(3, t49);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x9au64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x70008d27a4);
        }
        34 => {
            let Some(t50) = c.ld32(c.r64(15).wrapping_add(0x26f)) else { return RegfileExit::fault(0x70008d27a5); };
            c.w32(2, t50);
            let r51 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r51);
            let Some(t52) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008d27af); };
            c.cmp(t52, c.r32(12), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008d27b9); };
            c.w32(2, t53);
            c.logic(c.r32(2) & 0x80000001u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(3), 0x9fu64, 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008d27ce);
        }
        38 => {
            let Some(t54) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d27cf); };
            c.w32(12, t54);
            let r55 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r55);
            let Some(t56) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d27d7); };
            c.w32(12, t56);
            let r57 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r57);
            let Some(t58) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008d27df); };
            c.w32(3, t58);
            let r59 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r59);
            let Some(t60) = c.ld32(c.r64(15).wrapping_add(0x283)) else { return RegfileExit::fault(0x70008d27e7); };
            c.cmp(t60, c.r32(12), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t61) = c.ld32(c.r64(3).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008d27f0); };
            c.w32(12, t61);
            c.w32(12, c.r32(12));
            let r62 = c.r32(12) & 0x8u64;
            c.w32(12, r62);
            c.logic(r62, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d2801); };
            c.w32(12, t63);
            c.w32(12, c.r32(12));
            let r64 = c.r32(12) & 0x2u64;
            c.w32(12, r64);
            c.logic(r64, 32);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r65 = c.r64(12) & 0xfffffffffffffffeu64;
            c.w64(12, r65);
            c.logic(r65, 64);
            let r66 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r66);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 56; continue;
        }
        42 => {
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d2825); };
            c.w64(6, t67);
            blk = 43; continue;
        }
        43 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x700)) else { return RegfileExit::fault(0x70008d2829); };
            c.w64(3, t68);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x70008d2838, 0x70008d2833);
        }
        44 => {
            let Some(t69) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d2838); };
            c.cmp(t69, c.r32(0), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d2847); };
            c.cmp(t70, c.r32(0), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r71 = c.r32(11) | c.r32(12);
            c.w32(11, r71);
            c.logic(r71, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 626; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t72) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2867); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            let Some(t74) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d286e); };
            c.cmp(t74, 0x40u64, 16);
            if c.cc_ae() { blk = 626; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d287a); };
            c.w64(3, t75);
            let Some(t76) = c.ld32(c.r64(3).wrapping_add(0xa3)) else { return RegfileExit::fault(0x70008d2881); };
            c.w32(3, t76);
            let r77 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r77);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d288c); };
            c.w64(6, t78);
            return c.call(0x70008e8b74, 0x70008d2895, 0x70008d2890);
        }
        48 => {
            c.cmp(c.r32(0), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d289a); };
            c.w64(6, t79);
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            let Some(t80) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d28a5); };
            c.w64(11, t80);
            blk = 56; continue;
        }
        50 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d28ab); };
            c.w64(6, t81);
            blk = 51; continue;
        }
        51 => {
            c.w32(11, 0x148u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d28b5); }
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d28b7); };
            c.w64(11, t82);
            let Some(t83) = c.ld32(c.r64(11).wrapping_add(0x37f)) else { return RegfileExit::fault(0x70008d28be); };
            c.w32(11, t83);
            let r84 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r84);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d28c8); }
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d28ca); };
            c.w64(3, t85);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d28db, 0x70008d28d6);
        }
        52 => {
            return RegfileExit::fault(0x70008d28db);
        }
        53 => {
            c.w32(11, 0x36u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d28e2); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d28e4); };
            c.w64(11, t86);
            let Some(t87) = c.ld32(c.r64(11).wrapping_add(0x37f)) else { return RegfileExit::fault(0x70008d28eb); };
            c.w32(11, t87);
            let r88 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r88);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d28f5); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d28f7); };
            c.w64(3, t89);
            c.w32(0, 0x2u64);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d2903); };
            c.w64(6, t90);
            return c.call(0x700086ed74, 0x70008d290c, 0x70008d2907);
        }
        54 => {
            return RegfileExit::fault(0x70008d290c);
        }
        55 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d290d); };
            c.w64(6, t91);
            blk = 56; continue;
        }
        56 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xcd8)) else { return RegfileExit::fault(0x70008d2911); };
            c.w64(3, t92);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x70008d2920, 0x70008d291b);
        }
        57 => {
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d2920); };
            c.cmp(t93, c.r32(0), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d292d); };
            c.cmp(t94, c.r32(0), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 59; continue;
        }
        59 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 65; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t95) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d293a); };
            c.w32(11, t95);
            let r96 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r96);
            let Some(t97) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008d2941); };
            c.logic(t97 & 0x2u64, 8);
            if c.cc_e() { blk = 65; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d2948); };
            if !c.push64(t98) { return RegfileExit::fault(0x70008d2948); }
            let Some(t99) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d294b); };
            if !c.push64(t99) { return RegfileExit::fault(0x70008d294b); }
            let Some(t100) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d294e); };
            if !c.push64(t100) { return RegfileExit::fault(0x70008d294e); }
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d2951); };
            c.w64(6, t101);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x70007f2274, 0x70008d2965, 0x70008d2960);
        }
        62 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008d2965); };
            c.w64(1, t102);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d296c); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            return c.ret(0x18, 0x70008d2973);
        }
        64 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d2976); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d297d); }
            return c.ret(0x0, 0x70008d297f);
        }
        65 => {
            c.w32(11, 0xf6u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d2986); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d2988); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xcd8)) else { return RegfileExit::fault(0x70008d2989); };
            c.w64(11, t103);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d2990); }
            let Some(t104) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d2992); };
            if !c.push64(t104) { return RegfileExit::fault(0x70008d2992); }
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d2995); };
            c.w64(3, t105);
            c.w32(0, 0x4u64);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d29a1); };
            c.w64(6, t106);
            return c.call(0x700086ed74, 0x70008d29aa, 0x70008d29a5);
        }
        66 => {
            return RegfileExit::fault(0x70008d29aa);
        }
        67 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d29ab); };
            c.w64(6, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d29af); };
            c.w64(11, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d29b3); };
            c.w64(12, t109);
            let Some(t110) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008d29b7); };
            c.w64(0, t110);
            blk = 68; continue;
        }
        68 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 628; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t111) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d29c3); };
            c.w32(15, t111);
            let r112 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r112);
            let Some(t113) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d29ca); };
            c.cmp(t113, 0x40u64, 16);
            if c.cc_ae() { blk = 628; continue; }
            blk = 70; continue;
        }
        70 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d29d6); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 630; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t114) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d29e4); };
            c.w32(12, t114);
            let r115 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r115);
            let Some(t116) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d29eb); };
            c.cmp(t116, 0x40u64, 16);
            if c.cc_ae() { blk = 630; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d29f8); };
            c.w64(12, t117);
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(11)) { return RegfileExit::fault(0x70008d2a08); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t118) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2a12); };
            c.w32(3, t118);
            let r119 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r119);
            let Some(t120) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008d2a1a); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(3, 0x1u64);
            blk = 76; continue;
        }
        75 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 76; continue;
        }
        76 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(3)) { return RegfileExit::fault(0x70008d2a29); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 632; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t121) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2a3f); };
            c.w32(15, t121);
            let r122 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r122);
            let Some(t123) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2a47); };
            c.cmp(t123, 0x40u64, 16);
            if c.cc_ae() { blk = 632; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t124) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2a53); };
            c.w32(15, t124);
            c.w32(2, c.r64(15).wrapping_add(c.r64(15)));
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(12)) { return RegfileExit::fault(0x70008d2a5b); }
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(2)) { return RegfileExit::fault(0x70008d2a62); }
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_g() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(1, 0x2u64);
            blk = 82; continue;
        }
        81 => {
            c.w64(1, c.r64(2));
            blk = 82; continue;
        }
        82 => {
            let Some(t125) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2a78); };
            c.w32(7, t125);
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(1)) { return RegfileExit::fault(0x70008d2a7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d2a82); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_ne() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ge() { blk = 221; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(7), c.r32(15), 32);
            if c.cc_b() { blk = 220; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d2aa2); }
            c.w32(9, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(0));
            blk = 87; continue;
        }
        86 => {
            c.w64(15, c.r64(11));
            blk = 87; continue;
        }
        87 => {
            let Some(t126) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2ab1); };
            c.w32(11, t126);
            let r127 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r127);
            let Some(t128) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2ab8); };
            c.w32(11, t128);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d2abd); }
            let r129 = c.r32(11) & 0xfu64;
            c.w32(11, r129);
            c.logic(r129, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008d2ac5); }
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d2add); };
            c.w64(11, t130);
            let r131 = c.r32(11) & 0x7u64;
            c.w32(11, r131);
            c.logic(r131, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t132) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d2aeb); };
            c.w32(15, t132);
            let r133 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r133);
            c.w64(11, c.r64(15));
            blk = 86; continue;
        }
        91 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_ne() { blk = 96; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t134) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d2afd); };
            c.w32(11, t134);
            let r135 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r135);
            let Some(t136) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2b04); };
            c.cmp(t136, 0x0u64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 93; continue;
        }
        93 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008d2b0b); }
            c.w64(0, c.r64(15));
            return c.call(0x70008e88b4, 0x70008d2b17, 0x70008d2b12);
        }
        94 => {
            c.w64(11, c.r64(0));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d2b1a); };
            c.w64(0, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008d2b1e); };
            c.w64(1, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d2b25); };
            c.w64(9, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008d2b29); };
            c.w64(2, t140);
            blk = 86; continue;
        }
        95 => {
            let Some(t141) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d2b35); };
            c.w32(15, t141);
            let r142 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r142);
            c.w64(11, c.r64(15));
            blk = 86; continue;
        }
        96 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t143) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d2b4a); };
            c.w32(11, t143);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r144 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r144);
            let Some(t145) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d2b57); };
            c.w32(15, t145);
            let r146 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r146);
            c.w64(11, c.r64(15));
            blk = 86; continue;
        }
        98 => {
            let Some(t147) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2b66); };
            c.w32(11, t147);
            let Some(t148) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d2b6a); };
            c.cmp(c.r32(11), t148, 32);
            if c.cc_b() { blk = 101; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2b73); };
            let r150 = c.sub(c.r64(11), t149, 64);
            c.w64(11, r150);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(11, c.r64(9).wrapping_add(0xc));
            blk = 114; continue;
        }
        101 => {
            return RegfileExit::fault(0x70008d2b85);
        }
        102 => {
            let Some(t151) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2b86); };
            c.w32(11, t151);
            let Some(t152) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d2b8a); };
            c.cmp(c.r32(11), t152, 32);
            if c.cc_b() { blk = 105; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2b93); };
            let r154 = c.sub(c.r64(11), t153, 64);
            c.w64(11, r154);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(11, c.r64(9).wrapping_mul(2).wrapping_add(0xc));
            blk = 173; continue;
        }
        105 => {
            return RegfileExit::fault(0x70008d2ba9);
        }
        106 => {
            let Some(t155) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d2baa); };
            c.cmp(t155, 0xau64, 32);
            if c.cc_ne() { blk = 165; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t156) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d2bb4); };
            c.logic(t156 & 0x10u64, 8);
            if c.cc_ne() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t157) = c.ld64(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d2bba); };
            c.w64(11, t157);
            blk = 111; continue;
        }
        109 => {
            c.w64(10, c.r64(4));
            let r158 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r158);
            let r159 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r159);
            c.logic(r159, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d2bcb); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008d2bd0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008d2bd4); }
            c.w64(1, c.r64(15));
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d2bdb); };
            c.w64(0, t160);
            c.w64(10, 0x70008d2be9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d2be9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d2bed); }
            return c.call(c.r64(0), 0x70008d2bf3, 0x70008d2bf1);
        }
        110 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d2bf3); }
            let Some(t161) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d2bfb); };
            c.w64(4, t161);
            c.w64(11, c.r64(0));
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d2c03); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008d2c07); };
            c.w64(1, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d2c0e); };
            c.w64(15, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d2c12); };
            c.w64(9, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008d2c16); };
            c.w64(2, t166);
            blk = 111; continue;
        }
        111 => {
            c.w64(12, c.r64(15));
            let Some(t167) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2c20); };
            c.w32(12, t167);
            let Some(t168) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d2c25); };
            c.cmp(c.r32(12), t168, 32);
            if c.cc_b() { blk = 164; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2c32); };
            let r170 = c.sub(c.r64(12), t169, 64);
            c.w64(12, r170);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 164; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x1));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 114; continue;
        }
        114 => {
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d2c47); };
            c.w64(12, t171);
            let Some(t172) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2c4b); };
            c.w32(3, t172);
            c.w64(6, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d2c53); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008d2c57); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008d2c5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008d2c5f); }
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(12));
            blk = 116; continue;
        }
        115 => {
            c.w64(7, c.r64(0));
            blk = 116; continue;
        }
        116 => {
            let Some(t173) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d2c6e); };
            c.w32(9, t173);
            let r174 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r174);
            let Some(t175) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2c75); };
            c.w32(9, t175);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008d2c7a); }
            let r176 = c.r32(9) & 0xfu64;
            c.w32(9, r176);
            c.logic(r176, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008d2c82); }
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d2c9a); };
            c.w64(9, t177);
            let r178 = c.r32(9) & 0x7u64;
            c.w32(9, r178);
            c.logic(r178, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t179) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d2ca8); };
            c.w32(7, t179);
            let r180 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r180);
            c.w64(0, c.r64(7));
            blk = 115; continue;
        }
        120 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t181) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d2cb9); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2cc0); };
            c.cmp(t183, 0x0u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 122; continue;
        }
        122 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008d2cc7); }
            c.w64(0, c.r64(7));
            return c.call(0x70008e88b4, 0x70008d2cd3, 0x70008d2cce);
        }
        123 => {
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d2cd3); };
            c.w64(8, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d2cd7); };
            c.w64(12, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d2cdb); };
            c.w64(6, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d2cdf); };
            c.w64(11, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d2ce3); };
            c.w64(15, t188);
            blk = 115; continue;
        }
        124 => {
            let Some(t189) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d2ce9); };
            c.w32(7, t189);
            let r190 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r190);
            c.w64(0, c.r64(7));
            blk = 115; continue;
        }
        125 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t191) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d2cfd); };
            c.w32(0, t191);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r192 = c.add(c.r64(8), c.r64(0), 64);
            c.w64(8, r192);
            let Some(t193) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d2d09); };
            c.w32(7, t193);
            let r194 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r194);
            c.w64(0, c.r64(7));
            blk = 115; continue;
        }
        127 => {
            let Some(t195) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2d17); };
            c.w32(0, t195);
            let Some(t196) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d2d1a); };
            c.cmp(c.r32(0), t196, 32);
            if c.cc_b() { blk = 130; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r197 = c.sub(c.r64(0), c.r64(6), 64);
            c.w64(0, r197);
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_b() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(0, c.r64(8).wrapping_add(0xc));
            blk = 143; continue;
        }
        130 => {
            return RegfileExit::fault(0x70008d2d33);
        }
        131 => {
            let Some(t198) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2d34); };
            c.w32(0, t198);
            let Some(t199) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d2d37); };
            c.cmp(c.r32(0), t199, 32);
            if c.cc_b() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r200 = c.sub(c.r64(0), c.r64(6), 64);
            c.w64(0, r200);
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_b() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(0, c.r64(8).wrapping_mul(2).wrapping_add(0xc));
            blk = 160; continue;
        }
        134 => {
            return RegfileExit::fault(0x70008d2d54);
        }
        135 => {
            let Some(t201) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d2d55); };
            c.cmp(t201, 0xau64, 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t202) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d2d5f); };
            c.logic(t202 & 0x10u64, 8);
            if c.cc_ne() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t203) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d2d65); };
            c.w64(0, t203);
            blk = 140; continue;
        }
        138 => {
            c.w64(10, c.r64(4));
            let r204 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r204);
            let r205 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r205);
            c.logic(r205, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d2d76); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d2d7b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008d2d7f); }
            c.w64(1, c.r64(7));
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d2d86); };
            c.w64(0, t206);
            c.w64(10, 0x70008d2d94u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d2d94); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d2d98); }
            return c.call(c.r64(0), 0x70008d2d9e, 0x70008d2d9c);
        }
        139 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d2d9e); }
            let Some(t207) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d2da6); };
            c.w64(4, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d2dab); };
            c.w64(8, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d2daf); };
            c.w64(12, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d2db3); };
            c.w64(6, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d2db7); };
            c.w64(11, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d2dbb); };
            c.w64(15, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d2dbf); };
            c.w64(7, t213);
            blk = 140; continue;
        }
        140 => {
            c.w64(1, c.r64(7));
            let Some(t214) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2dc6); };
            c.w32(3, t214);
            let Some(t215) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d2dc9); };
            c.cmp(c.r32(3), t215, 32);
            if c.cc_b() { blk = 151; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r216 = c.sub(c.r64(3), c.r64(6), 64);
            c.w64(3, r216);
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_b() { blk = 151; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 143; continue;
        }
        143 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(2, c.r64(15).wrapping_add(c.r64(11)).wrapping_sub(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008d2df2); }
            c.cmp(c.r64(6), 0x1u64, 64);
            if c.cc_e() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w64(10, c.r64(4));
            let r217 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r217);
            let r218 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r218);
            c.logic(r218, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d2e07); }
            c.w64(0, c.r64(6));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d2e0f); }
            if !c.st64(c.r64(4).wrapping_add(0x28), 0x0u64) { return RegfileExit::fault(0x70008d2e14); }
            c.w64(1, c.r64(13));
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2e20); };
            c.w64(8, t219);
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x1430)) else { return RegfileExit::fault(0x70008d2e24); };
            c.w64(0, t220);
            c.w64(10, 0x70008d2e32u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d2e32); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d2e36); }
            return c.call(c.r64(0), 0x70008d2e3c, 0x70008d2e3a);
        }
        145 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d2e3c); }
            let Some(t221) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d2e44); };
            c.w64(4, t221);
            blk = 150; continue;
        }
        146 => {
            let Some(t222) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x70008d2e4b); };
            c.w32(11, t222);
            c.w64(10, c.r64(4));
            let r223 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r223);
            let r224 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r224);
            c.logic(r224, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d2e5a); }
            c.w64(1, c.r64(2));
            c.w64(2, c.r64(11));
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2e65); };
            c.w64(8, t225);
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x13c0)) else { return RegfileExit::fault(0x70008d2e69); };
            c.w64(0, t226);
            c.w64(10, 0x70008d2e77u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d2e77); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d2e7b); }
            return c.call(c.r64(0), 0x70008d2e81, 0x70008d2e7f);
        }
        147 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d2e81); }
            let Some(t227) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d2e89); };
            c.w64(4, t227);
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 149; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d2e94); };
            let r229 = c.sub(c.r64(0), t228, 64);
            c.w64(0, r229);
            blk = 150; continue;
        }
        149 => {
            c.w64(0, 0xffffffffffffffffu64);
            blk = 150; continue;
        }
        150 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            blk = 215; continue;
        }
        151 => {
            return RegfileExit::fault(0x70008d2eaa);
        }
        152 => {
            let Some(t230) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d2eab); };
            c.cmp(t230, 0x2u64, 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t231) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d2eb5); };
            c.logic(t231 & 0x10u64, 8);
            if c.cc_ne() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t232) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d2ebb); };
            c.w64(0, t232);
            blk = 157; continue;
        }
        155 => {
            c.w64(10, c.r64(4));
            let r233 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r233);
            let r234 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r234);
            c.logic(r234, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d2ecc); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d2ed1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008d2ed5); }
            c.w64(1, c.r64(7));
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d2edc); };
            c.w64(0, t235);
            c.w64(10, 0x70008d2eeau64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d2eea); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d2eee); }
            return c.call(c.r64(0), 0x70008d2ef4, 0x70008d2ef2);
        }
        156 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d2ef4); }
            let Some(t236) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d2efc); };
            c.w64(4, t236);
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d2f01); };
            c.w64(8, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d2f05); };
            c.w64(12, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d2f09); };
            c.w64(6, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d2f0d); };
            c.w64(11, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d2f11); };
            c.w64(15, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d2f15); };
            c.w64(7, t242);
            blk = 157; continue;
        }
        157 => {
            let Some(t243) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d2f19); };
            c.w32(3, t243);
            let Some(t244) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d2f1c); };
            c.cmp(c.r32(3), t244, 32);
            if c.cc_b() { blk = 162; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r245 = c.sub(c.r64(3), c.r64(6), 64);
            c.w64(3, r245);
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_b() { blk = 162; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w64(0, c.r64(0).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 160; continue;
        }
        160 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(2, c.r64(15).wrapping_add(c.r64(11)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r246 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r246);
            let r247 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r247);
            c.logic(r247, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d2f48); }
            c.w64(11, c.r64(6));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d2f50); }
            if !c.st64(c.r64(4).wrapping_add(0x28), 0x0u64) { return RegfileExit::fault(0x70008d2f55); }
            c.w64(1, c.r64(13));
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d2f61); };
            c.w64(8, t248);
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x1438)) else { return RegfileExit::fault(0x70008d2f65); };
            c.w64(0, t249);
            c.w64(10, 0x70008d2f73u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d2f73); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d2f77); }
            return c.call(c.r64(0), 0x70008d2f7d, 0x70008d2f7b);
        }
        161 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d2f7d); }
            let Some(t250) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d2f85); };
            c.w64(4, t250);
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            blk = 215; continue;
        }
        162 => {
            return RegfileExit::fault(0x70008d2f93);
        }
        163 => {
            return RegfileExit::fault(0x70008d2f94);
        }
        164 => {
            return RegfileExit::fault(0x70008d2f95);
        }
        165 => {
            let Some(t251) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d2f96); };
            c.cmp(t251, 0x2u64, 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t252) = c.ld8(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d2fa0); };
            c.logic(t252 & 0x10u64, 8);
            if c.cc_ne() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t253) = c.ld64(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d2fa6); };
            c.w64(11, t253);
            blk = 170; continue;
        }
        168 => {
            c.w64(10, c.r64(4));
            let r254 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r254);
            let r255 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r255);
            c.logic(r255, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d2fb7); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008d2fbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008d2fc0); }
            c.w64(1, c.r64(15));
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d2fc7); };
            c.w64(0, t256);
            c.w64(10, 0x70008d2fd5u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d2fd5); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d2fd9); }
            return c.call(c.r64(0), 0x70008d2fdf, 0x70008d2fdd);
        }
        169 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d2fdf); }
            let Some(t257) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d2fe7); };
            c.w64(4, t257);
            c.w64(11, c.r64(0));
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d2fef); };
            c.w64(0, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008d2ff3); };
            c.w64(1, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d2ffa); };
            c.w64(15, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d2ffe); };
            c.w64(9, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008d3002); };
            c.w64(2, t262);
            blk = 170; continue;
        }
        170 => {
            let Some(t263) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3009); };
            c.w32(12, t263);
            let Some(t264) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d300d); };
            c.cmp(c.r32(12), t264, 32);
            if c.cc_b() { blk = 218; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d301a); };
            let r266 = c.sub(c.r64(12), t265, 64);
            c.w64(12, r266);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 218; continue; }
            blk = 172; continue;
        }
        172 => {
            c.w64(11, c.r64(11).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 173; continue;
        }
        173 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d302f); };
            c.w64(12, t267);
            let Some(t268) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3033); };
            c.w32(3, t268);
            c.w64(6, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d303b); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(15)) { return RegfileExit::fault(0x70008d303f); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(3)) { return RegfileExit::fault(0x70008d3043); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008d3047); }
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(12));
            blk = 175; continue;
        }
        174 => {
            c.w64(7, c.r64(0));
            blk = 175; continue;
        }
        175 => {
            let Some(t269) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3056); };
            c.w32(9, t269);
            let r270 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r270);
            let Some(t271) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d305d); };
            c.w32(9, t271);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008d3062); }
            let r272 = c.r32(9) & 0xfu64;
            c.w32(9, r272);
            c.logic(r272, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008d306a); }
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3082); };
            c.w64(9, t273);
            let r274 = c.r32(9) & 0x7u64;
            c.w32(9, r274);
            c.logic(r274, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t275) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3090); };
            c.w32(7, t275);
            let r276 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r276);
            c.w64(0, c.r64(7));
            blk = 174; continue;
        }
        179 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 184; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t277) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d30a1); };
            c.w32(9, t277);
            let r278 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r278);
            let Some(t279) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d30a8); };
            c.cmp(t279, 0x0u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 181; continue;
        }
        181 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008d30af); }
            c.w64(0, c.r64(7));
            return c.call(0x70008e88b4, 0x70008d30bb, 0x70008d30b6);
        }
        182 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d30bb); };
            c.w64(8, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d30bf); };
            c.w64(12, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d30c3); };
            c.w64(6, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d30c7); };
            c.w64(11, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d30cb); };
            c.w64(15, t284);
            blk = 174; continue;
        }
        183 => {
            let Some(t285) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d30d1); };
            c.w32(7, t285);
            let r286 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r286);
            c.w64(0, c.r64(7));
            blk = 174; continue;
        }
        184 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t287) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d30e5); };
            c.w32(0, t287);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r288 = c.add(c.r64(8), c.r64(0), 64);
            c.w64(8, r288);
            let Some(t289) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d30f1); };
            c.w32(7, t289);
            let r290 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r290);
            c.w64(0, c.r64(7));
            blk = 174; continue;
        }
        186 => {
            let Some(t291) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d30ff); };
            c.w32(0, t291);
            let Some(t292) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d3102); };
            c.cmp(c.r32(0), t292, 32);
            if c.cc_b() { blk = 189; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r293 = c.sub(c.r64(0), c.r64(6), 64);
            c.w64(0, r293);
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_b() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w64(0, c.r64(8).wrapping_add(0xc));
            blk = 202; continue;
        }
        189 => {
            return RegfileExit::fault(0x70008d311b);
        }
        190 => {
            let Some(t294) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d311c); };
            c.w32(0, t294);
            let Some(t295) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d311f); };
            c.cmp(c.r32(0), t295, 32);
            if c.cc_b() { blk = 193; continue; }
            blk = 191; continue;
        }
        191 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r296 = c.sub(c.r64(0), c.r64(6), 64);
            c.w64(0, r296);
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_b() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w64(0, c.r64(8).wrapping_mul(2).wrapping_add(0xc));
            blk = 213; continue;
        }
        193 => {
            return RegfileExit::fault(0x70008d313c);
        }
        194 => {
            let Some(t297) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d313d); };
            c.cmp(t297, 0xau64, 32);
            if c.cc_ne() { blk = 205; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t298) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3147); };
            c.logic(t298 & 0x10u64, 8);
            if c.cc_ne() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t299) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d314d); };
            c.w64(0, t299);
            blk = 199; continue;
        }
        197 => {
            c.w64(10, c.r64(4));
            let r300 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r300);
            let r301 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r301);
            c.logic(r301, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d315e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d3163); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008d3167); }
            c.w64(1, c.r64(7));
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d316e); };
            c.w64(0, t302);
            c.w64(10, 0x70008d317cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d317c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d3180); }
            return c.call(c.r64(0), 0x70008d3186, 0x70008d3184);
        }
        198 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3186); }
            let Some(t303) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d318e); };
            c.w64(4, t303);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d3193); };
            c.w64(8, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d3197); };
            c.w64(12, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d319b); };
            c.w64(6, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d319f); };
            c.w64(11, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d31a3); };
            c.w64(15, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d31a7); };
            c.w64(7, t309);
            blk = 199; continue;
        }
        199 => {
            c.w64(1, c.r64(7));
            let Some(t310) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d31ae); };
            c.w32(3, t310);
            let Some(t311) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d31b1); };
            c.cmp(c.r32(3), t311, 32);
            if c.cc_b() { blk = 204; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r312 = c.sub(c.r64(3), c.r64(6), 64);
            c.w64(3, r312);
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_b() { blk = 204; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 202; continue;
        }
        202 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(2, c.r64(15).wrapping_add(c.r64(11)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r313 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r313);
            let r314 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r314);
            c.logic(r314, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d31dd); }
            c.w64(0, c.r64(6));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d31e5); }
            if !c.st64(c.r64(4).wrapping_add(0x28), 0x0u64) { return RegfileExit::fault(0x70008d31ea); }
            c.w64(1, c.r64(13));
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d31f6); };
            c.w64(8, t315);
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0x1440)) else { return RegfileExit::fault(0x70008d31fa); };
            c.w64(0, t316);
            c.w64(10, 0x70008d3208u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d3208); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d320c); }
            return c.call(c.r64(0), 0x70008d3212, 0x70008d3210);
        }
        203 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3212); }
            let Some(t317) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d321a); };
            c.w64(4, t317);
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            blk = 215; continue;
        }
        204 => {
            return RegfileExit::fault(0x70008d3228);
        }
        205 => {
            let Some(t318) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d3229); };
            c.cmp(t318, 0x2u64, 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t319) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3233); };
            c.logic(t319 & 0x10u64, 8);
            if c.cc_ne() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t320) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d3239); };
            c.w64(0, t320);
            blk = 210; continue;
        }
        208 => {
            c.w64(10, c.r64(4));
            let r321 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r321);
            let r322 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r322);
            c.logic(r322, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d324a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d324f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008d3253); }
            c.w64(1, c.r64(7));
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d325a); };
            c.w64(0, t323);
            c.w64(10, 0x70008d3268u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d3268); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d326c); }
            return c.call(c.r64(0), 0x70008d3272, 0x70008d3270);
        }
        209 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3272); }
            let Some(t324) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d327a); };
            c.w64(4, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d327f); };
            c.w64(8, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d3283); };
            c.w64(12, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3287); };
            c.w64(6, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d328b); };
            c.w64(11, t328);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d328f); };
            c.w64(15, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d3293); };
            c.w64(7, t330);
            blk = 210; continue;
        }
        210 => {
            let Some(t331) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3297); };
            c.w32(3, t331);
            let Some(t332) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d329a); };
            c.cmp(c.r32(3), t332, 32);
            if c.cc_b() { blk = 216; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r333 = c.sub(c.r64(3), c.r64(6), 64);
            c.w64(3, r333);
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_b() { blk = 216; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(0, c.r64(0).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 213; continue;
        }
        213 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(2, c.r64(15).wrapping_add(c.r64(11)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r334 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r334);
            let r335 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r335);
            c.logic(r335, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d32ce); }
            c.w64(11, c.r64(6));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d32d6); }
            if !c.st64(c.r64(4).wrapping_add(0x28), 0x0u64) { return RegfileExit::fault(0x70008d32db); }
            c.w64(1, c.r64(13));
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d32e7); };
            c.w64(8, t336);
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x1448)) else { return RegfileExit::fault(0x70008d32eb); };
            c.w64(0, t337);
            c.w64(10, 0x70008d32f9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d32f9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d32fd); }
            return c.call(c.r64(0), 0x70008d3303, 0x70008d3301);
        }
        214 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3303); }
            let Some(t338) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d330b); };
            c.w64(4, t338);
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            blk = 215; continue;
        }
        215 => {
            c.w64(15, c.r64(11));
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d3317); };
            c.w64(6, t339);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d331b); };
            c.w64(0, t340);
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d331f); };
            c.w64(11, t341);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008d3323); };
            c.w64(1, t342);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008d332a); };
            c.w64(12, t343);
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008d3331); };
            c.w64(3, t344);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008d3338); };
            c.w64(2, t345);
            blk = 222; continue;
        }
        216 => {
            return RegfileExit::fault(0x70008d3341);
        }
        217 => {
            return RegfileExit::fault(0x70008d3342);
        }
        218 => {
            return RegfileExit::fault(0x70008d3343);
        }
        219 => {
            return RegfileExit::fault(0x70008d3344);
        }
        220 => {
            c.w64(15, 0xfffffffffffffffeu64);
            blk = 222; continue;
        }
        221 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 222; continue;
        }
        222 => {
            let Some(t346) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d3351); };
            c.w64(7, t346);
            if !c.st64(c.r64(5).wrapping_sub(0x60), 0x0u64) { return RegfileExit::fault(0x70008d3358); }
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(2));
            c.w64(2, c.r64(10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008d3369); }
            blk = 224; continue;
        }
        223 => {
            c.w64(12, c.r64(2));
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008d3372); };
            c.w64(3, t347);
            let r348 = c.add(c.r32(12), c.r32(3), 32);
            c.w32(12, r348);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x70008d337f); }
            c.w64(2, c.r64(15));
            c.w64(15, c.r64(6));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d3389); }
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d338d); };
            c.w64(6, t349);
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008d3391); };
            c.w64(12, t350);
            c.w64(0, c.r64(3));
            blk = 224; continue;
        }
        224 => {
            let Some(t351) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d339b); };
            c.w64(8, t351);
            c.cmp(c.r32(15), 0xfffffffffffffffeu64, 32);
            if c.cc_e() { blk = 475; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(15)) { return RegfileExit::fault(0x70008d33b2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d33b9); }
            let Some(t352) = c.ld32(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008d33bd); };
            c.cmp(t352, 0x0u64, 32);
            if c.cc_ne() { blk = 232; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(0, c.r64(12));
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d33cd); };
            c.w64(3, t353);
            let Some(t354) = c.ld32(c.r64(3).wrapping_add(0xb3)) else { return RegfileExit::fault(0x70008d33d4); };
            c.w32(3, t354);
            let r355 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r355);
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70008e8b74, 0x70008d33e4, 0x70008d33df);
        }
        227 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008d33e4); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 231; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008d33ed); };
            c.w64(11, t356);
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x70008d33f4); };
            c.w64(12, t357);
            c.w32(15, c.r64(11).wrapping_add(c.r64(12)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d3405); };
            c.w64(0, t358);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d3409); };
            c.w64(3, t359);
            c.w64(1, c.r64(15));
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d3410); };
            c.w64(6, t360);
            return c.call(0x70007fbc74, 0x70008d3419, 0x70008d3414);
        }
        229 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d3419); }
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d341a); };
            if !c.push64(t361) { return RegfileExit::fault(0x70008d341a); }
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008d341d); };
            if !c.push64(t362) { return RegfileExit::fault(0x70008d341d); }
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008d3423); };
            if !c.push64(t363) { return RegfileExit::fault(0x70008d3423); }
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3429); };
            if !c.push64(t364) { return RegfileExit::fault(0x70008d3429); }
            let Some(t365) = c.ld64(c.r64(13).wrapping_add(0x2750)) else { return RegfileExit::fault(0x70008d342c); };
            c.w64(3, t365);
            c.w32(0, 0x5u64);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d3438); };
            c.w64(6, t366);
            return c.call(0x700086ed74, 0x70008d3441, 0x70008d343c);
        }
        230 => {
            blk = 235; continue;
        }
        231 => {
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008d3443); };
            c.w64(0, t367);
            blk = 235; continue;
        }
        232 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d344c); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008d344d); }
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d344f); };
            if !c.push64(t368) { return RegfileExit::fault(0x70008d344f); }
            c.w64(3, c.r64(8));
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008d3455); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(12));
            return c.call(0x70007f2134, 0x70008d3463, 0x70008d345e);
        }
        233 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 634; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t369) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d346b); };
            c.w32(11, t369);
            let r370 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r370);
            let Some(t371) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3472); };
            c.cmp(t371, 0x40u64, 16);
            if c.cc_ae() { blk = 634; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t372) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d347e); };
            c.w64(11, (t372 as u32 as i32 as i64 as u64));
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d3482); };
            c.w64(12, t373);
            let Some(t374) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3486); };
            c.w32(15, t374);
            let r375 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r375);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t376) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3491); };
            c.w32(3, t376);
            let Some(t377) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3496); };
            c.w32(15, t377);
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d349b); };
            c.w64(2, t378);
            let r379 = c.sub(c.r64(2), c.r64(11), 64);
            c.w64(2, r379);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008d34a2); }
            c.cmp(c.r64(3), c.r64(2), 64);
            if c.cc_be() { blk = 345; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d34af); };
            c.w64(3, t380);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_e() { blk = 344; continue; }
            blk = 237; continue;
        }
        237 => {
            c.logic(c.r8(15) & 0x7u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_e() { blk = 319; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 254; continue; }
            blk = 239; continue;
        }
        239 => {
            c.w64(6, c.r64(15));
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(12));
            blk = 240; continue;
        }
        240 => {
            c.w64(7, c.r64(6));
            let r381 = c.r32(7) & 0x7u64;
            c.w32(7, r381);
            c.logic(r381, 32);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_l() { blk = 245; continue; }
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 242; continue;
        }
        242 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 243; continue;
        }
        243 => {
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 244; continue;
        }
        244 => {
            blk = 348; continue;
        }
        245 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 246; continue;
        }
        246 => {
            c.cmp(c.r32(7), 0x1u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 247; continue;
        }
        247 => {
            blk = 348; continue;
        }
        248 => {
            let Some(t382) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3514); };
            c.w32(6, t382);
            let Some(t383) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d3517); };
            c.cmp(t383, c.r32(6), 32);
            if c.cc_ne() { blk = 348; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t384) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3524); };
            c.w32(1, t384);
            let r385 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r385);
            let Some(t386) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d352a); };
            c.w32(6, t386);
            let r387 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r387);
            let Some(t388) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3530); };
            c.w32(6, t388);
            blk = 240; continue;
        }
        250 => {
            let Some(t389) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3536); };
            c.w32(6, t389);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            let r390 = c.add(c.r64(15), c.r64(6), 64);
            c.w64(15, r390);
            let Some(t391) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3542); };
            c.w32(1, t391);
            let r392 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r392);
            let Some(t393) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3548); };
            c.w32(6, t393);
            let r394 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r394);
            let Some(t395) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d354e); };
            c.w32(6, t395);
            blk = 240; continue;
        }
        251 => {
            let Some(t396) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3554); };
            c.w32(1, t396);
            let r397 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r397);
            let Some(t398) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d355a); };
            c.w32(6, t398);
            let r399 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r399);
            let Some(t400) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3560); };
            c.w32(6, t400);
            blk = 240; continue;
        }
        252 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 255; continue;
        }
        253 => {
            c.w32(7, 0x1u64);
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 255; continue;
        }
        254 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(15));
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(12));
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r64(2), 0xdu64, 64);
            if c.cc_l() { blk = 263; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t401) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d359b); };
            c.w64(11, (t401 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r402 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r402);
            let r403 = c.r32(1) & 0x8u64;
            c.w32(1, r403);
            c.logic(r403, 32);
            let r404 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r404);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d35b0); };
            c.w64(8, t405);
            let Some(t406) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d35b7); };
            c.w64(15, t406);
            c.w64(3, c.r64(15).wrapping_add(0x14));
            let Some(t407) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d35be); };
            c.w64(9, t407);
            let Some(t408) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d35c5); };
            c.cmp(t408, c.r64(3), 64);
            if c.cc_be() { blk = 636; continue; }
            blk = 258; continue;
        }
        258 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008d35ce); }
            let r409 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r409);
            blk = 259; continue;
        }
        259 => {
            let Some(t410) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d35d5); };
            c.w64(3, t410);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008d35dc); }
            if !c.st32(c.r64(15).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d35e0); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d35e8); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x70008d35ec); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008d35f0); }
            blk = 350; continue;
        }
        260 => {
            let Some(t411) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d35f9); };
            c.w64(8, t411);
            let Some(t412) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d3600); };
            c.w64(15, t412);
            c.w64(3, c.r64(15).wrapping_add(0x14));
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d3607); };
            c.w64(9, t413);
            let Some(t414) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d360e); };
            c.cmp(t414, c.r64(3), 64);
            if c.cc_be() { blk = 638; continue; }
            blk = 261; continue;
        }
        261 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008d3617); }
            let r415 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r415);
            blk = 262; continue;
        }
        262 => {
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d361e); };
            c.w64(3, t416);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008d3625); }
            if !c.st32(c.r64(15).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d3629); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d3631); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x70008d3635); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008d3639); }
            blk = 350; continue;
        }
        263 => {
            let Some(t417) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d3642); };
            c.w64(9, t417);
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d3649); };
            c.w64(8, t418);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 291; continue; }
            blk = 264; continue;
        }
        264 => {
            c.w32(11, c.r32(2));
            let r419 = c.r32(1) & 0x8u64;
            c.w32(1, r419);
            c.logic(r419, 32);
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 265; continue;
        }
        265 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 272; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w32(11, c.r32(2));
            c.w64(3, c.r64(11));
            let r420 = c.add(c.r32(3), c.r32(11), 32);
            c.w32(3, r420);
            if c.cc_o() { blk = 598; continue; }
            blk = 267; continue;
        }
        267 => {
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(3, 0xcu64);
            let r421 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r421);
            if c.cc_o() { blk = 597; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t422) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d369a); };
            c.w64(7, t422);
            let Some(t423) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d36a1); };
            c.w64(3, t423);
            let r424 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r424);
            let r425 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r425);
            c.logic(r425, 64);
            c.w64(1, c.r64(3).wrapping_add(c.r64(11)));
            let Some(t426) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d36b0); };
            c.w64(9, t426);
            let Some(t427) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d36b7); };
            c.cmp(t427, c.r64(1), 64);
            if c.cc_be() { blk = 640; continue; }
            blk = 269; continue;
        }
        269 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 640; continue; }
            blk = 270; continue;
        }
        270 => {
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x70008d36cd); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 271; continue;
        }
        271 => {
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d36d4); };
            c.w64(3, t428);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008d36db); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d36df); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d36e7); }
            blk = 273; continue;
        }
        272 => {
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d36ed); };
            c.w64(11, t429);
            let Some(t430) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d36f4); };
            c.w64(7, t430);
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d36fb); };
            c.w64(9, t431);
            let Some(t432) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d3702); };
            c.w64(3, t432);
            blk = 273; continue;
        }
        273 => {
            let Some(t433) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3709); };
            c.w64(1, (t433 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r434 = c.add(c.r64(15), c.r64(1), 64);
            c.w64(15, r434);
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0xb));
            c.w64(2, c.r64(15).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_e() { blk = 290; continue; }
            blk = 274; continue;
        }
        274 => {
            c.w32(1, 0xbu64);
            blk = 276; continue;
        }
        275 => {
            let r435 = c.add(c.r64(1), 0x2u64, 64);
            c.w64(1, r435);
            blk = 276; continue;
        }
        276 => {
            let Some(t436) = c.ld16(c.r64(6).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008d3733); };
            c.w32(12, t436);
            if !c.st16(c.r64(11).wrapping_add(c.r64(1)), c.r16(12)) { return RegfileExit::fault(0x70008d3738); }
            let r437 = c.add(c.r64(15), 0x2u64, 64);
            c.w64(15, r437);
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_ne() { blk = 275; continue; }
            blk = 277; continue;
        }
        277 => {
            blk = 290; continue;
        }
        278 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 284; continue; }
            blk = 279; continue;
        }
        279 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(3, 0xcu64);
            let r438 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r438);
            if c.cc_o() { blk = 599; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d3762); };
            c.w64(7, t439);
            let Some(t440) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d3769); };
            c.w64(3, t440);
            let r441 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r441);
            let r442 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r442);
            c.logic(r442, 64);
            c.w64(1, c.r64(3).wrapping_add(c.r64(11)));
            let Some(t443) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d3778); };
            c.w64(9, t443);
            let Some(t444) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d377f); };
            c.cmp(t444, c.r64(1), 64);
            if c.cc_be() { blk = 642; continue; }
            blk = 281; continue;
        }
        281 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 642; continue; }
            blk = 282; continue;
        }
        282 => {
            if !c.st64(c.r64(7), c.r64(1)) { return RegfileExit::fault(0x70008d3795); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 283; continue;
        }
        283 => {
            let Some(t445) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d379c); };
            c.w64(3, t445);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008d37a3); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d37a7); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d37af); }
            blk = 285; continue;
        }
        284 => {
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d37b5); };
            c.w64(11, t446);
            let Some(t447) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d37bc); };
            c.w64(3, t447);
            let Some(t448) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d37c3); };
            c.w64(7, t448);
            let Some(t449) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d37ca); };
            c.w64(9, t449);
            blk = 285; continue;
        }
        285 => {
            let Some(t450) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d37d1); };
            c.w64(1, (t450 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(15, c.r64(15).wrapping_add(c.r64(1)).wrapping_add(0xb));
            let r451 = c.add(c.r64(2), c.r64(15), 64);
            c.w64(2, r451);
            c.cmp(c.r64(15), c.r64(2), 64);
            if c.cc_e() { blk = 289; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w32(1, 0xbu64);
            blk = 288; continue;
        }
        287 => {
            let r452 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r452);
            blk = 288; continue;
        }
        288 => {
            let Some(t453) = c.ld8(c.r64(6).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x70008d37f0); };
            c.w32(12, t453);
            if !c.st8(c.r64(11).wrapping_add(c.r64(1)), c.r8(12)) { return RegfileExit::fault(0x70008d37f5); }
            let r454 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r454);
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_ne() { blk = 287; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t455) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d3802); };
            c.w64(3, t455);
            blk = 290; continue;
        }
        290 => {
            c.w64(15, c.r64(11));
            blk = 350; continue;
        }
        291 => {
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d3811); };
            c.w64(12, t456);
            let Some(t457) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d3818); };
            c.w64(7, t457);
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 318; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w32(11, c.r32(2));
            let Some(t458) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d382b); };
            c.w64(3, t458);
            let r459 = c.r32(1) & 0x8u64;
            c.w32(1, r459);
            c.logic(r459, 32);
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 293; continue;
        }
        293 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 300; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w32(11, c.r32(2));
            c.w64(1, c.r64(11));
            let r460 = c.add(c.r32(1), c.r32(11), 32);
            c.w32(1, r460);
            if c.cc_o() { blk = 601; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(1, 0xcu64);
            let r461 = c.add(c.r64(11), c.r64(1), 64);
            c.w64(11, r461);
            if c.cc_o() { blk = 600; continue; }
            blk = 296; continue;
        }
        296 => {
            let Some(t462) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d386d); };
            c.w64(6, t462);
            let Some(t463) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008d3874); };
            c.w64(1, t463);
            let r464 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r464);
            let r465 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r465);
            c.logic(r465, 64);
            c.w64(0, c.r64(1).wrapping_add(c.r64(11)));
            let Some(t466) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d3883); };
            c.w64(9, t466);
            let Some(t467) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d388a); };
            c.cmp(t467, c.r64(0), 64);
            if c.cc_be() { blk = 644; continue; }
            blk = 297; continue;
        }
        297 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 644; continue; }
            blk = 298; continue;
        }
        298 => {
            if !c.st64(c.r64(6), c.r64(0)) { return RegfileExit::fault(0x70008d38a0); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 299; continue;
        }
        299 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008d38a7); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d38ab); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d38b3); }
            blk = 301; continue;
        }
        300 => {
            let Some(t468) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d38b9); };
            c.w64(11, t468);
            let Some(t469) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d38c0); };
            c.w64(6, t469);
            let Some(t470) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d38c7); };
            c.w64(9, t470);
            blk = 301; continue;
        }
        301 => {
            let Some(t471) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d38ce); };
            c.w64(0, (t471 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r472 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r472);
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0xb));
            c.w64(0, c.r64(15).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_e() { blk = 317; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w32(2, 0xbu64);
            blk = 304; continue;
        }
        303 => {
            let r473 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r473);
            blk = 304; continue;
        }
        304 => {
            let Some(t474) = c.ld16(c.r64(15).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d38f8); };
            c.w32(1, t474);
            if !c.st16(c.r64(11).wrapping_add(c.r64(2)), c.r16(1)) { return RegfileExit::fault(0x70008d38fe); }
            let r475 = c.add(c.r64(15), 0x2u64, 64);
            c.w64(15, r475);
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_ne() { blk = 303; continue; }
            blk = 305; continue;
        }
        305 => {
            blk = 317; continue;
        }
        306 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 312; continue; }
            blk = 307; continue;
        }
        307 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, 0xcu64);
            let r476 = c.add(c.r64(11), c.r64(1), 64);
            c.w64(11, r476);
            if c.cc_o() { blk = 602; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t477) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d3928); };
            c.w64(6, t477);
            let Some(t478) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008d392f); };
            c.w64(1, t478);
            let r479 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r479);
            let r480 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r480);
            c.logic(r480, 64);
            c.w64(0, c.r64(1).wrapping_add(c.r64(11)));
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d393e); };
            c.w64(9, t481);
            let Some(t482) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d3945); };
            c.cmp(t482, c.r64(0), 64);
            if c.cc_be() { blk = 646; continue; }
            blk = 309; continue;
        }
        309 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 646; continue; }
            blk = 310; continue;
        }
        310 => {
            if !c.st64(c.r64(6), c.r64(0)) { return RegfileExit::fault(0x70008d395b); }
            c.w64(11, c.r64(1).wrapping_add(0x1));
            blk = 311; continue;
        }
        311 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008d3962); }
            if !c.st32(c.r64(11).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d3966); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d396e); }
            blk = 313; continue;
        }
        312 => {
            let Some(t483) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d3974); };
            c.w64(11, t483);
            let Some(t484) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d397b); };
            c.w64(6, t484);
            let Some(t485) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d3982); };
            c.w64(9, t485);
            blk = 313; continue;
        }
        313 => {
            let Some(t486) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3989); };
            c.w64(0, (t486 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(15, c.r64(15).wrapping_add(c.r64(0)).wrapping_add(0xb));
            c.w64(0, c.r64(15).wrapping_add(c.r64(2)));
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_e() { blk = 317; continue; }
            blk = 314; continue;
        }
        314 => {
            c.w32(2, 0xbu64);
            blk = 316; continue;
        }
        315 => {
            let r487 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r487);
            blk = 316; continue;
        }
        316 => {
            let Some(t488) = c.ld8(c.r64(15).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d39a9); };
            c.w32(1, t488);
            if !c.st8(c.r64(11).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008d39af); }
            let r489 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r489);
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_ne() { blk = 315; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(15, c.r64(11));
            blk = 350; continue;
        }
        318 => {
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d39c4); };
            c.w64(12, t490);
            blk = 348; continue;
        }
        319 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 335; continue; }
            blk = 320; continue;
        }
        320 => {
            c.w64(3, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(12));
            blk = 321; continue;
        }
        321 => {
            c.w64(2, c.r64(3));
            let r491 = c.r32(2) & 0x7u64;
            c.w32(2, r491);
            c.logic(r491, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 326; continue; }
            blk = 322; continue;
        }
        322 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 334; continue; }
            blk = 323; continue;
        }
        323 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 331; continue; }
            blk = 324; continue;
        }
        324 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 332; continue; }
            blk = 325; continue;
        }
        325 => {
            blk = 650; continue;
        }
        326 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 333; continue; }
            blk = 327; continue;
        }
        327 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 329; continue; }
            blk = 328; continue;
        }
        328 => {
            blk = 650; continue;
        }
        329 => {
            let Some(t492) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3a15); };
            c.w32(3, t492);
            let Some(t493) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d3a19); };
            c.cmp(t493, c.r32(3), 32);
            if c.cc_ne() { blk = 650; continue; }
            blk = 330; continue;
        }
        330 => {
            let Some(t494) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3a26); };
            c.w32(15, t494);
            let r495 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r495);
            let Some(t496) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3a2d); };
            c.w32(3, t496);
            let r497 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r497);
            let Some(t498) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3a34); };
            c.w32(3, t498);
            blk = 321; continue;
        }
        331 => {
            let Some(t499) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3a3a); };
            c.w32(3, t499);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r500 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r500);
            let Some(t501) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3a47); };
            c.w32(15, t501);
            let r502 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r502);
            let Some(t503) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3a4e); };
            c.w32(3, t503);
            let r504 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r504);
            let Some(t505) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3a55); };
            c.w32(3, t505);
            blk = 321; continue;
        }
        332 => {
            let Some(t506) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3a5b); };
            c.w32(15, t506);
            let r507 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r507);
            let Some(t508) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3a62); };
            c.w32(3, t508);
            let r509 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r509);
            let Some(t510) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3a69); };
            c.w32(3, t510);
            blk = 321; continue;
        }
        333 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 336; continue;
        }
        334 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(3));
            c.w64(3, c.r64(10));
            c.w32(2, 0x1u64);
            blk = 336; continue;
        }
        335 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(12));
            blk = 336; continue;
        }
        336 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 648; continue; }
            blk = 337; continue;
        }
        337 => {
            let r511 = c.add(c.r64(3), 0xbu64, 64);
            c.w64(3, r511);
            blk = 338; continue;
        }
        338 => {
            let Some(t512) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3ab6); };
            c.w64(2, (t512 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r513 = c.r32(11) & 0x8u64;
            c.w32(11, r513);
            c.logic(r513, 32);
            let r514 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r514);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 340; continue; }
            blk = 339; continue;
        }
        339 => {
            let Some(t515) = c.ld16(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d3aca); };
            c.w32(11, t515);
            blk = 341; continue;
        }
        340 => {
            let Some(t516) = c.ld8(c.r64(15).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x70008d3ad1); };
            c.w32(11, t516);
            blk = 341; continue;
        }
        341 => {
            c.cmp(c.r32(11), 0xffu64, 32);
            if c.cc_g() { blk = 663; continue; }
            blk = 342; continue;
        }
        342 => {
            c.w64(15, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t517) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d3aeb); };
            c.w64(2, t517);
            let Some(t518) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3af2); };
            c.w32(3, t518);
            let r519 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r519);
            let Some(t520) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d3afa); };
            c.cmp(t520, c.r32(3), 32);
            if c.cc_e() { blk = 668; continue; }
            blk = 343; continue;
        }
        343 => {
            c.w64(15, c.r64(3));
            blk = 350; continue;
        }
        344 => {
            let Some(t521) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d3b0c); };
            c.w64(11, t521);
            c.w64(15, c.r64(11));
            blk = 350; continue;
        }
        345 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 347; continue; }
            blk = 346; continue;
        }
        346 => {
            c.w64(15, c.r64(12));
            blk = 350; continue;
        }
        347 => {
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d3b23); };
            c.w64(3, t522);
            blk = 348; continue;
        }
        348 => {
            let r523 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r523);
            c.w32(15, c.r64(3).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008d3b2e); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d3b30); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008d3b32); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t524) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008d3b36); };
            c.w64(3, t524);
            c.w32(0, 0x3u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008d3b4a, 0x70008d3b45);
        }
        349 => {
            c.w64(15, c.r64(0));
            blk = 350; continue;
        }
        350 => {
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d3b4d); };
            c.w64(0, t525);
            c.w64(3, c.r64(15));
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d3b54); };
            c.w64(6, t526);
            return c.call(0x700086f9d4, 0x70008d3b5d, 0x70008d3b58);
        }
        351 => {
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d3b5d); };
            c.w64(6, t527);
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3b61); };
            c.w64(3, t528);
            return c.call(0x700086f9d4, 0x70008d3b6a, 0x70008d3b65);
        }
        352 => {
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d3b6a); };
            c.w64(11, t529);
            let Some(t530) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3b6e); };
            c.w32(12, t530);
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d3b72); };
            c.w64(15, t531);
            let Some(t532) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3b76); };
            c.w32(3, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008d3b7a); };
            c.w64(2, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008d3b81); };
            c.w64(1, t534);
            c.w32(6, c.r64(2).wrapping_add(c.r64(1)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(6));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008d3b94); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(3)) { return RegfileExit::fault(0x70008d3b98); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d3b9c); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 354; continue; }
            blk = 353; continue;
        }
        353 => {
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_le() { blk = 223; continue; }
            blk = 354; continue;
        }
        354 => {
            let r535 = c.add(c.r64(12), c.r64(7), 64);
            c.w64(12, r535);
            c.cmp(c.r64(3), c.r64(12), 64);
            if c.cc_l() { blk = 474; continue; }
            blk = 355; continue;
        }
        355 => {
            c.w64(8, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d3bbe); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(15));
            blk = 357; continue;
        }
        356 => {
            c.w64(12, c.r64(0));
            blk = 357; continue;
        }
        357 => {
            let Some(t536) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3bcc); };
            c.w32(9, t536);
            let r537 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r537);
            let Some(t538) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3bd4); };
            c.w32(9, t538);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008d3bd9); }
            let r539 = c.r32(9) & 0xfu64;
            c.w32(9, r539);
            c.logic(r539, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008d3be1); }
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 368; continue; }
            blk = 358; continue;
        }
        358 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 371; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d3bf9); };
            c.w64(9, t540);
            let r541 = c.r32(9) & 0x7u64;
            c.w32(9, r541);
            c.logic(r541, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_ne() { blk = 361; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t542) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3c07); };
            c.w32(12, t542);
            let r543 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r543);
            c.w64(0, c.r64(12));
            blk = 356; continue;
        }
        361 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 366; continue; }
            blk = 362; continue;
        }
        362 => {
            let Some(t544) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3c1a); };
            c.w32(9, t544);
            let r545 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r545);
            let Some(t546) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3c22); };
            c.cmp(t546, 0x0u64, 32);
            if c.cc_e() { blk = 365; continue; }
            blk = 363; continue;
        }
        363 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008d3c29); }
            c.w64(0, c.r64(12));
            return c.call(0x70008e88b4, 0x70008d3c35, 0x70008d3c30);
        }
        364 => {
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008d3c35); };
            c.w64(2, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d3c3c); };
            c.w64(15, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d3c40); };
            c.w64(7, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d3c44); };
            c.w64(11, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d3c48); };
            c.w64(6, t551);
            blk = 356; continue;
        }
        365 => {
            let Some(t552) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3c51); };
            c.w32(12, t552);
            let r553 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r553);
            c.w64(0, c.r64(12));
            blk = 356; continue;
        }
        366 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_ne() { blk = 374; continue; }
            blk = 367; continue;
        }
        367 => {
            let Some(t554) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3c67); };
            c.w32(0, t554);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r555 = c.add(c.r64(6), c.r64(0), 64);
            c.w64(6, r555);
            let Some(t556) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3c75); };
            c.w32(12, t556);
            let r557 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r557);
            c.w64(0, c.r64(12));
            blk = 356; continue;
        }
        368 => {
            let Some(t558) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3c85); };
            c.w32(0, t558);
            let Some(t559) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d3c8a); };
            c.cmp(c.r32(0), t559, 32);
            if c.cc_b() { blk = 603; continue; }
            blk = 369; continue;
        }
        369 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3c96); };
            let r561 = c.sub(c.r64(0), t560, 64);
            c.w64(0, r561);
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_b() { blk = 603; continue; }
            blk = 370; continue;
        }
        370 => {
            c.w64(0, c.r64(6).wrapping_add(0xc));
            blk = 382; continue;
        }
        371 => {
            let Some(t562) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3cac); };
            c.w32(0, t562);
            let Some(t563) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d3cb1); };
            c.cmp(c.r32(0), t563, 32);
            if c.cc_b() { blk = 604; continue; }
            blk = 372; continue;
        }
        372 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3cbd); };
            let r565 = c.sub(c.r64(0), t564, 64);
            c.w64(0, r565);
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_b() { blk = 604; continue; }
            blk = 373; continue;
        }
        373 => {
            c.w64(0, c.r64(6).wrapping_mul(2).wrapping_add(0xc));
            blk = 435; continue;
        }
        374 => {
            let Some(t566) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3cd7); };
            c.cmp(t566, 0xau64, 32);
            if c.cc_ne() { blk = 427; continue; }
            blk = 375; continue;
        }
        375 => {
            let Some(t567) = c.ld8(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d3ce1); };
            c.logic(t567 & 0x10u64, 8);
            if c.cc_ne() { blk = 377; continue; }
            blk = 376; continue;
        }
        376 => {
            let Some(t568) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d3ce7); };
            c.w64(0, t568);
            blk = 379; continue;
        }
        377 => {
            c.w64(10, c.r64(4));
            let r569 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r569);
            let r570 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r570);
            c.logic(r570, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d3cf9); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x70008d3cfe); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008d3d05); }
            c.w64(1, c.r64(12));
            let Some(t571) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d3d0c); };
            c.w64(0, t571);
            c.w64(10, 0x70008d3d1au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d3d1a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d3d1e); }
            return c.call(c.r64(0), 0x70008d3d24, 0x70008d3d22);
        }
        378 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3d24); }
            let Some(t572) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d3d2c); };
            c.w64(4, t572);
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008d3d31); };
            c.w64(2, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d3d38); };
            c.w64(15, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d3d3c); };
            c.w64(7, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d3d40); };
            c.w64(11, t576);
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d3d44); };
            c.w64(12, t577);
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d3d4b); };
            c.w64(6, t578);
            blk = 379; continue;
        }
        379 => {
            c.w64(1, c.r64(12));
            let Some(t579) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3d52); };
            c.w32(12, t579);
            let Some(t580) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d3d56); };
            c.cmp(c.r32(12), t580, 32);
            if c.cc_b() { blk = 610; continue; }
            blk = 380; continue;
        }
        380 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3d63); };
            let r582 = c.sub(c.r64(12), t581, 64);
            c.w64(12, r582);
            c.cmp(c.r64(12), c.r64(6), 64);
            if c.cc_b() { blk = 610; continue; }
            blk = 381; continue;
        }
        381 => {
            c.w64(0, c.r64(6).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 382; continue;
        }
        382 => {
            let Some(t583) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3d78); };
            c.w32(3, t583);
            c.w64(1, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d3d7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x70008d3d83); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x70008d3d8a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008d3d8e); }
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(11));
            blk = 384; continue;
        }
        383 => {
            c.w64(6, c.r64(15));
            blk = 384; continue;
        }
        384 => {
            let Some(t584) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d3d9d); };
            c.w32(9, t584);
            let r585 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r585);
            let Some(t586) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3da4); };
            c.w32(9, t586);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008d3da9); }
            let r587 = c.r32(9) & 0xfu64;
            c.w32(9, r587);
            c.logic(r587, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x70008d3db1); }
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 395; continue; }
            blk = 385; continue;
        }
        385 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 398; continue; }
            blk = 386; continue;
        }
        386 => {
            let Some(t588) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d3dc9); };
            c.w64(9, t588);
            let r589 = c.r32(9) & 0x7u64;
            c.w32(9, r589);
            c.logic(r589, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_ne() { blk = 388; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t590) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3dd7); };
            c.w32(6, t590);
            let r591 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r591);
            c.w64(15, c.r64(6));
            blk = 383; continue;
        }
        388 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 393; continue; }
            blk = 389; continue;
        }
        389 => {
            let Some(t592) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3de8); };
            c.w32(9, t592);
            let r593 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r593);
            let Some(t594) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3def); };
            c.cmp(t594, 0x0u64, 32);
            if c.cc_e() { blk = 392; continue; }
            blk = 390; continue;
        }
        390 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008d3df6); }
            c.w64(0, c.r64(6));
            return c.call(0x70008e88b4, 0x70008d3e02, 0x70008d3dfd);
        }
        391 => {
            c.w64(15, c.r64(0));
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d3e05); };
            c.w64(11, t595);
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d3e09); };
            c.w64(1, t596);
            let Some(t597) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d3e0d); };
            c.w64(0, t597);
            let Some(t598) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d3e11); };
            c.w64(12, t598);
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3e18); };
            c.w64(8, t599);
            blk = 383; continue;
        }
        392 => {
            let Some(t600) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3e21); };
            c.w32(6, t600);
            let r601 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r601);
            c.w64(15, c.r64(6));
            blk = 383; continue;
        }
        393 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_ne() { blk = 401; continue; }
            blk = 394; continue;
        }
        394 => {
            let Some(t602) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d3e35); };
            c.w32(15, t602);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r603 = c.add(c.r64(8), c.r64(15), 64);
            c.w64(8, r603);
            let Some(t604) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d3e42); };
            c.w32(6, t604);
            let r605 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r605);
            c.w64(15, c.r64(6));
            blk = 383; continue;
        }
        395 => {
            let Some(t606) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3e50); };
            c.w32(15, t606);
            let Some(t607) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d3e54); };
            c.cmp(c.r32(15), t607, 32);
            if c.cc_b() { blk = 605; continue; }
            blk = 396; continue;
        }
        396 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r608 = c.sub(c.r64(15), c.r64(1), 64);
            c.w64(15, r608);
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_b() { blk = 605; continue; }
            blk = 397; continue;
        }
        397 => {
            c.w64(15, c.r64(8).wrapping_add(0xc));
            blk = 409; continue;
        }
        398 => {
            let Some(t609) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3e76); };
            c.w32(15, t609);
            let Some(t610) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d3e7a); };
            c.cmp(c.r32(15), t610, 32);
            if c.cc_b() { blk = 606; continue; }
            blk = 399; continue;
        }
        399 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r611 = c.sub(c.r64(15), c.r64(1), 64);
            c.w64(15, r611);
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_b() { blk = 606; continue; }
            blk = 400; continue;
        }
        400 => {
            c.w64(15, c.r64(8).wrapping_mul(2).wrapping_add(0xc));
            blk = 425; continue;
        }
        401 => {
            let Some(t612) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008d3ea0); };
            c.cmp(t612, 0xau64, 32);
            if c.cc_ne() { blk = 417; continue; }
            blk = 402; continue;
        }
        402 => {
            let Some(t613) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d3eaa); };
            c.logic(t613 & 0x10u64, 8);
            if c.cc_ne() { blk = 404; continue; }
            blk = 403; continue;
        }
        403 => {
            let Some(t614) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d3eb0); };
            c.w64(15, t614);
            blk = 406; continue;
        }
        404 => {
            c.w64(10, c.r64(4));
            let r615 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r615);
            let r616 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r616);
            c.logic(r616, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d3ec1); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d3ec6); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008d3eca); }
            c.w64(1, c.r64(6));
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d3ed1); };
            c.w64(0, t617);
            c.w64(10, 0x70008d3edfu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d3edf); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d3ee3); }
            return c.call(c.r64(0), 0x70008d3ee9, 0x70008d3ee7);
        }
        405 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3ee9); }
            let Some(t618) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d3ef1); };
            c.w64(4, t618);
            c.w64(15, c.r64(0));
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d3ef9); };
            c.w64(11, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d3efd); };
            c.w64(1, t620);
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d3f01); };
            c.w64(0, t621);
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d3f05); };
            c.w64(12, t622);
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d3f0c); };
            c.w64(6, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d3f10); };
            c.w64(8, t624);
            blk = 406; continue;
        }
        406 => {
            c.w64(3, c.r64(6));
            let Some(t625) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d3f17); };
            c.w32(3, t625);
            let Some(t626) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d3f1a); };
            c.cmp(c.r32(3), t626, 32);
            if c.cc_b() { blk = 607; continue; }
            blk = 407; continue;
        }
        407 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r627 = c.sub(c.r64(3), c.r64(1), 64);
            c.w64(3, r627);
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_b() { blk = 607; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(15)).wrapping_add(0x1));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 409; continue;
        }
        409 => {
            c.w64(9, c.r64(6).wrapping_add(c.r64(15)).wrapping_sub(0x1));
            c.w64(2, c.r64(12).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_e() { blk = 412; continue; }
            blk = 410; continue;
        }
        410 => {
            c.w64(10, c.r64(4));
            let r628 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r628);
            let r629 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r629);
            c.logic(r629, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d3f54); }
            c.w64(15, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(15)) { return RegfileExit::fault(0x70008d3f5c); }
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d3f61); };
            c.w64(12, t630);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(12)) { return RegfileExit::fault(0x70008d3f65); }
            c.w64(1, c.r64(13));
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3f6d); };
            c.w64(8, t631);
            let Some(t632) = c.ld64(c.r64(13).wrapping_add(0x1430)) else { return RegfileExit::fault(0x70008d3f71); };
            c.w64(0, t632);
            c.w64(10, 0x70008d3f7fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d3f7f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d3f83); }
            return c.call(c.r64(0), 0x70008d3f89, 0x70008d3f87);
        }
        411 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3f89); }
            let Some(t633) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d3f91); };
            c.w64(4, t633);
            blk = 416; continue;
        }
        412 => {
            let Some(t634) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x70008d3f98); };
            c.w32(15, t634);
            let Some(t635) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d3f9c); };
            c.w64(12, t635);
            c.w64(1, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d3fa4); };
            c.w64(8, t636);
            let r637 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r637);
            c.w64(10, c.r64(4));
            let r638 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r638);
            let r639 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r639);
            c.logic(r639, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d3fb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008d3fbb); }
            c.w64(2, c.r64(15));
            let Some(t640) = c.ld64(c.r64(13).wrapping_add(0x13c0)) else { return RegfileExit::fault(0x70008d3fc2); };
            c.w64(0, t640);
            c.w64(10, 0x70008d3fd0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d3fd0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d3fd4); }
            return c.call(c.r64(0), 0x70008d3fda, 0x70008d3fd8);
        }
        413 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d3fda); }
            let Some(t641) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d3fe2); };
            c.w64(4, t641);
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 415; continue; }
            blk = 414; continue;
        }
        414 => {
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d3fed); };
            let r643 = c.sub(c.r64(0), t642, 64);
            c.w64(0, r643);
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d3ff1); };
            c.w64(11, t644);
            let r645 = c.add(c.r64(0), c.r64(11), 64);
            c.w64(0, r645);
            blk = 416; continue;
        }
        415 => {
            c.w64(0, 0xffffffffffffffffu64);
            blk = 416; continue;
        }
        416 => {
            c.w32(6, c.r64(0).wrapping_add(c.r64(0)));
            blk = 473; continue;
        }
        417 => {
            let Some(t646) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008d4009); };
            c.cmp(t646, 0x2u64, 32);
            if c.cc_ne() { blk = 609; continue; }
            blk = 418; continue;
        }
        418 => {
            let Some(t647) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d4013); };
            c.logic(t647 & 0x10u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t648) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d4019); };
            c.w64(15, t648);
            blk = 422; continue;
        }
        420 => {
            c.w64(10, c.r64(4));
            let r649 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r649);
            let r650 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r650);
            c.logic(r650, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d402a); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d402f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008d4033); }
            c.w64(1, c.r64(6));
            let Some(t651) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d403a); };
            c.w64(0, t651);
            c.w64(10, 0x70008d4048u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d4048); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d404c); }
            return c.call(c.r64(0), 0x70008d4052, 0x70008d4050);
        }
        421 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d4052); }
            let Some(t652) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d405a); };
            c.w64(4, t652);
            c.w64(15, c.r64(0));
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d4062); };
            c.w64(11, t653);
            let Some(t654) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4066); };
            c.w64(1, t654);
            let Some(t655) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d406a); };
            c.w64(0, t655);
            let Some(t656) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d406e); };
            c.w64(12, t656);
            let Some(t657) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4075); };
            c.w64(6, t657);
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d4079); };
            c.w64(8, t658);
            blk = 422; continue;
        }
        422 => {
            let Some(t659) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d407d); };
            c.w32(3, t659);
            let Some(t660) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d4080); };
            c.cmp(c.r32(3), t660, 32);
            if c.cc_b() { blk = 608; continue; }
            blk = 423; continue;
        }
        423 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r661 = c.sub(c.r64(3), c.r64(1), 64);
            c.w64(3, r661);
            c.cmp(c.r64(3), c.r64(8), 64);
            if c.cc_b() { blk = 608; continue; }
            blk = 424; continue;
        }
        424 => {
            c.w64(15, c.r64(15).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 425; continue;
        }
        425 => {
            c.w64(9, c.r64(6).wrapping_add(c.r64(15)).wrapping_sub(0x1));
            c.w64(2, c.r64(12).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r662 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r662);
            let r663 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r663);
            c.logic(r663, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d40b4); }
            c.w64(12, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(12)) { return RegfileExit::fault(0x70008d40bc); }
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d40c1); };
            c.w64(12, t664);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(12)) { return RegfileExit::fault(0x70008d40c5); }
            c.w64(1, c.r64(13));
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d40cd); };
            c.w64(8, t665);
            let Some(t666) = c.ld64(c.r64(13).wrapping_add(0x1438)) else { return RegfileExit::fault(0x70008d40d1); };
            c.w64(0, t666);
            c.w64(10, 0x70008d40dfu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d40df); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d40e3); }
            return c.call(c.r64(0), 0x70008d40e9, 0x70008d40e7);
        }
        426 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d40e9); }
            let Some(t667) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d40f1); };
            c.w64(4, t667);
            c.w32(6, c.r64(0).wrapping_add(c.r64(0)));
            blk = 473; continue;
        }
        427 => {
            let Some(t668) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d40fe); };
            c.cmp(t668, 0x2u64, 32);
            if c.cc_ne() { blk = 617; continue; }
            blk = 428; continue;
        }
        428 => {
            let Some(t669) = c.ld8(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4108); };
            c.logic(t669 & 0x10u64, 8);
            if c.cc_ne() { blk = 430; continue; }
            blk = 429; continue;
        }
        429 => {
            let Some(t670) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d410e); };
            c.w64(0, t670);
            blk = 432; continue;
        }
        430 => {
            c.w64(10, c.r64(4));
            let r671 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r671);
            let r672 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r672);
            c.logic(r672, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d4120); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x70008d4125); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008d412c); }
            c.w64(1, c.r64(12));
            let Some(t673) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d4133); };
            c.w64(0, t673);
            c.w64(10, 0x70008d4141u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d4141); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d4145); }
            return c.call(c.r64(0), 0x70008d414b, 0x70008d4149);
        }
        431 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d414b); }
            let Some(t674) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d4153); };
            c.w64(4, t674);
            let Some(t675) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008d4158); };
            c.w64(2, t675);
            let Some(t676) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d415f); };
            c.w64(15, t676);
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4163); };
            c.w64(7, t677);
            let Some(t678) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d4167); };
            c.w64(11, t678);
            let Some(t679) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d416b); };
            c.w64(12, t679);
            let Some(t680) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4172); };
            c.w64(6, t680);
            blk = 432; continue;
        }
        432 => {
            let Some(t681) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d4176); };
            c.w32(12, t681);
            let Some(t682) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d417b); };
            c.cmp(c.r32(12), t682, 32);
            if c.cc_b() { blk = 616; continue; }
            blk = 433; continue;
        }
        433 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t683) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4188); };
            let r684 = c.sub(c.r64(12), t683, 64);
            c.w64(12, r684);
            c.cmp(c.r64(12), c.r64(6), 64);
            if c.cc_b() { blk = 616; continue; }
            blk = 434; continue;
        }
        434 => {
            c.w64(0, c.r64(0).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 435; continue;
        }
        435 => {
            let Some(t685) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d419d); };
            c.w32(3, t685);
            c.w64(1, (c.r32(3) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d41a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x70008d41a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x70008d41af); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008d41b3); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 436; continue;
        }
        436 => {
            c.w64(6, c.r64(11));
            let Some(t686) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d41bd); };
            c.w32(9, t686);
            let r687 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r687);
            let Some(t688) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d41c4); };
            c.w32(9, t688);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008d41c9); }
            let r689 = c.r32(9) & 0xfu64;
            c.w32(9, r689);
            c.logic(r689, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x70008d41d1); }
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 447; continue; }
            blk = 437; continue;
        }
        437 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 450; continue; }
            blk = 438; continue;
        }
        438 => {
            let Some(t690) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d41e9); };
            c.w64(9, t690);
            let r691 = c.r32(9) & 0x7u64;
            c.w32(9, r691);
            c.logic(r691, 32);
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_ne() { blk = 440; continue; }
            blk = 439; continue;
        }
        439 => {
            let Some(t692) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d41f7); };
            c.w32(6, t692);
            let r693 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r693);
            c.w64(11, c.r64(6));
            blk = 436; continue;
        }
        440 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 445; continue; }
            blk = 441; continue;
        }
        441 => {
            let Some(t694) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d4208); };
            c.w32(9, t694);
            let r695 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r695);
            let Some(t696) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d420f); };
            c.cmp(t696, 0x0u64, 32);
            if c.cc_e() { blk = 444; continue; }
            blk = 442; continue;
        }
        442 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008d4216); }
            c.w64(0, c.r64(6));
            return c.call(0x70008e88b4, 0x70008d4222, 0x70008d421d);
        }
        443 => {
            c.w64(11, c.r64(0));
            let Some(t697) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4225); };
            c.w64(1, t697);
            let Some(t698) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d4229); };
            c.w64(0, t698);
            let Some(t699) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d422d); };
            c.w64(12, t699);
            let Some(t700) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d4234); };
            c.w64(8, t700);
            blk = 436; continue;
        }
        444 => {
            let Some(t701) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d423a); };
            c.w32(6, t701);
            let r702 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r702);
            c.w64(11, c.r64(6));
            blk = 436; continue;
        }
        445 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_ne() { blk = 453; continue; }
            blk = 446; continue;
        }
        446 => {
            let Some(t703) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d424e); };
            c.w32(11, t703);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r704 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r704);
            let Some(t705) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d425b); };
            c.w32(6, t705);
            let r706 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r706);
            c.w64(11, c.r64(6));
            blk = 436; continue;
        }
        447 => {
            let Some(t707) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d4269); };
            c.w32(11, t707);
            let Some(t708) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d426d); };
            c.cmp(c.r32(11), t708, 32);
            if c.cc_b() { blk = 611; continue; }
            blk = 448; continue;
        }
        448 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r709 = c.sub(c.r64(11), c.r64(1), 64);
            c.w64(11, r709);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 611; continue; }
            blk = 449; continue;
        }
        449 => {
            c.w64(11, c.r64(8).wrapping_add(0xc));
            blk = 461; continue;
        }
        450 => {
            let Some(t710) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d428f); };
            c.w32(11, t710);
            let Some(t711) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d4293); };
            c.cmp(c.r32(11), t711, 32);
            if c.cc_b() { blk = 612; continue; }
            blk = 451; continue;
        }
        451 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r712 = c.sub(c.r64(11), c.r64(1), 64);
            c.w64(11, r712);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 612; continue; }
            blk = 452; continue;
        }
        452 => {
            c.w64(11, c.r64(8).wrapping_mul(2).wrapping_add(0xc));
            blk = 471; continue;
        }
        453 => {
            let Some(t713) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008d42b9); };
            c.cmp(t713, 0xau64, 32);
            if c.cc_ne() { blk = 463; continue; }
            blk = 454; continue;
        }
        454 => {
            let Some(t714) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d42c3); };
            c.logic(t714 & 0x10u64, 8);
            if c.cc_ne() { blk = 456; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t715) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d42c9); };
            c.w64(11, t715);
            blk = 458; continue;
        }
        456 => {
            c.w64(10, c.r64(4));
            let r716 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r716);
            let r717 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r717);
            c.logic(r717, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d42da); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d42df); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008d42e3); }
            c.w64(1, c.r64(6));
            let Some(t718) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d42ea); };
            c.w64(0, t718);
            c.w64(10, 0x70008d42f8u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d42f8); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d42fc); }
            return c.call(c.r64(0), 0x70008d4302, 0x70008d4300);
        }
        457 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d4302); }
            let Some(t719) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d430a); };
            c.w64(4, t719);
            c.w64(11, c.r64(0));
            let Some(t720) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4312); };
            c.w64(1, t720);
            let Some(t721) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d4316); };
            c.w64(0, t721);
            let Some(t722) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d431a); };
            c.w64(12, t722);
            let Some(t723) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4321); };
            c.w64(6, t723);
            let Some(t724) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d4325); };
            c.w64(8, t724);
            blk = 458; continue;
        }
        458 => {
            c.w64(15, c.r64(6));
            let Some(t725) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d432c); };
            c.w32(15, t725);
            let Some(t726) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d4330); };
            c.cmp(c.r32(15), t726, 32);
            if c.cc_b() { blk = 613; continue; }
            blk = 459; continue;
        }
        459 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r727 = c.sub(c.r64(15), c.r64(1), 64);
            c.w64(15, r727);
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_b() { blk = 613; continue; }
            blk = 460; continue;
        }
        460 => {
            c.w64(11, c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x1));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 461; continue;
        }
        461 => {
            c.w64(9, c.r64(6).wrapping_add(c.r64(11)).wrapping_sub(0x1));
            c.w64(2, c.r64(12).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r728 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r728);
            let r729 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r729);
            c.logic(r729, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d4365); }
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d436d); }
            let Some(t730) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4372); };
            c.w64(3, t730);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(3)) { return RegfileExit::fault(0x70008d4376); }
            c.w64(1, c.r64(13));
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d437e); };
            c.w64(8, t731);
            let Some(t732) = c.ld64(c.r64(13).wrapping_add(0x1440)) else { return RegfileExit::fault(0x70008d4382); };
            c.w64(0, t732);
            c.w64(10, 0x70008d4390u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d4390); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d4394); }
            return c.call(c.r64(0), 0x70008d439a, 0x70008d4398);
        }
        462 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d439a); }
            let Some(t733) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d43a2); };
            c.w64(4, t733);
            c.w32(6, c.r64(0).wrapping_add(c.r64(0)));
            blk = 473; continue;
        }
        463 => {
            let Some(t734) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008d43af); };
            c.cmp(t734, 0x2u64, 32);
            if c.cc_ne() { blk = 615; continue; }
            blk = 464; continue;
        }
        464 => {
            let Some(t735) = c.ld8(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d43b9); };
            c.logic(t735 & 0x10u64, 8);
            if c.cc_ne() { blk = 466; continue; }
            blk = 465; continue;
        }
        465 => {
            let Some(t736) = c.ld64(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d43bf); };
            c.w64(11, t736);
            blk = 468; continue;
        }
        466 => {
            c.w64(10, c.r64(4));
            let r737 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r737);
            let r738 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r738);
            c.logic(r738, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d43d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d43d5); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008d43d9); }
            c.w64(1, c.r64(6));
            let Some(t739) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d43e0); };
            c.w64(0, t739);
            c.w64(10, 0x70008d43eeu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d43ee); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d43f2); }
            return c.call(c.r64(0), 0x70008d43f8, 0x70008d43f6);
        }
        467 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d43f8); }
            let Some(t740) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d4400); };
            c.w64(4, t740);
            c.w64(11, c.r64(0));
            let Some(t741) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4408); };
            c.w64(1, t741);
            let Some(t742) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d440c); };
            c.w64(0, t742);
            let Some(t743) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008d4410); };
            c.w64(12, t743);
            let Some(t744) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4417); };
            c.w64(6, t744);
            let Some(t745) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d441b); };
            c.w64(8, t745);
            blk = 468; continue;
        }
        468 => {
            let Some(t746) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d441f); };
            c.w32(15, t746);
            let Some(t747) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d4423); };
            c.cmp(c.r32(15), t747, 32);
            if c.cc_b() { blk = 614; continue; }
            blk = 469; continue;
        }
        469 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r748 = c.sub(c.r64(15), c.r64(1), 64);
            c.w64(15, r748);
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_b() { blk = 614; continue; }
            blk = 470; continue;
        }
        470 => {
            c.w64(11, c.r64(11).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 471; continue;
        }
        471 => {
            c.w64(9, c.r64(6).wrapping_add(c.r64(11)).wrapping_sub(0x1));
            c.w64(2, c.r64(12).wrapping_add(c.r64(0)).wrapping_sub(0x1));
            c.w64(10, c.r64(4));
            let r749 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r749);
            let r750 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r750);
            c.logic(r750, 64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008d4458); }
            c.w64(11, c.r64(1));
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d4460); }
            let Some(t751) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4465); };
            c.w64(11, t751);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(11)) { return RegfileExit::fault(0x70008d4469); }
            c.w64(1, c.r64(13));
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4471); };
            c.w64(8, t752);
            let Some(t753) = c.ld64(c.r64(13).wrapping_add(0x1448)) else { return RegfileExit::fault(0x70008d4475); };
            c.w64(0, t753);
            c.w64(10, 0x70008d4483u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d4483); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d4487); }
            return c.call(c.r64(0), 0x70008d448d, 0x70008d448b);
        }
        472 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d448d); }
            let Some(t754) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008d4495); };
            c.w64(4, t754);
            c.w32(6, c.r64(0).wrapping_add(c.r64(0)));
            blk = 473; continue;
        }
        473 => {
            let Some(t755) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008d449d); };
            c.w64(2, t755);
            let Some(t756) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d44a4); };
            c.w64(15, t756);
            let Some(t757) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d44a8); };
            c.w64(11, t757);
            let Some(t758) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008d44ac); };
            c.w64(1, t758);
            let Some(t759) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d44b3); };
            c.w64(0, t759);
            blk = 223; continue;
        }
        474 => {
            c.w64(6, 0xfffffffffffffffeu64);
            blk = 223; continue;
        }
        475 => {
            c.w64(0, c.r64(2));
            let Some(t760) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d44cb); };
            c.w32(11, t760);
            c.w32(12, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t761) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d44d3); };
            c.w64(15, t761);
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_l() { blk = 477; continue; }
            blk = 476; continue;
        }
        476 => {
            let Some(t762) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d44dc); };
            c.w64(0, t762);
            blk = 62; continue;
        }
        477 => {
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t763) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d44e8); };
            c.w32(2, t763);
            let r764 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r764);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let Some(t765) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d44f1); };
            c.w32(2, t765);
            c.w64(1, c.r64(11));
            let r766 = c.sub(c.r64(1), c.r64(3), 64);
            c.w64(1, r766);
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_be() { blk = 591; continue; }
            blk = 478; continue;
        }
        478 => {
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_e() { blk = 590; continue; }
            blk = 479; continue;
        }
        479 => {
            c.logic(c.r8(2) & 0x7u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_e() { blk = 565; continue; }
            blk = 480; continue;
        }
        480 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 496; continue; }
            blk = 481; continue;
        }
        481 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(0));
            blk = 482; continue;
        }
        482 => {
            c.w64(8, c.r64(7));
            let r767 = c.r32(8) & 0x7u64;
            c.w32(8, r767);
            c.logic(r767, 32);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 487; continue; }
            blk = 483; continue;
        }
        483 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 495; continue; }
            blk = 484; continue;
        }
        484 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 492; continue; }
            blk = 485; continue;
        }
        485 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 493; continue; }
            blk = 486; continue;
        }
        486 => {
            blk = 593; continue;
        }
        487 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 494; continue; }
            blk = 488; continue;
        }
        488 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 490; continue; }
            blk = 489; continue;
        }
        489 => {
            blk = 593; continue;
        }
        490 => {
            let Some(t768) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d456e); };
            c.w32(7, t768);
            let Some(t769) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d4571); };
            c.cmp(t769, c.r32(7), 32);
            if c.cc_ne() { blk = 593; continue; }
            blk = 491; continue;
        }
        491 => {
            let Some(t770) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d457e); };
            c.w32(2, t770);
            let r771 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r771);
            let Some(t772) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d4584); };
            c.w32(7, t772);
            let r773 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r773);
            let Some(t774) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d458a); };
            c.w32(7, t774);
            blk = 482; continue;
        }
        492 => {
            let Some(t775) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d4590); };
            c.w32(7, t775);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r776 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r776);
            let Some(t777) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d459c); };
            c.w32(2, t777);
            let r778 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r778);
            let Some(t779) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d45a2); };
            c.w32(7, t779);
            let r780 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r780);
            let Some(t781) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d45a8); };
            c.w32(7, t781);
            blk = 482; continue;
        }
        493 => {
            let Some(t782) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d45ae); };
            c.w32(2, t782);
            let r783 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r783);
            let Some(t784) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d45b4); };
            c.w32(7, t784);
            let r785 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r785);
            let Some(t786) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d45ba); };
            c.w32(7, t786);
            blk = 482; continue;
        }
        494 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(10));
            blk = 497; continue;
        }
        495 => {
            c.w32(8, 0x1u64);
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(2));
            c.w64(2, c.r64(10));
            blk = 497; continue;
        }
        496 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(0));
            blk = 497; continue;
        }
        497 => {
            c.cmp(c.r64(1), 0xdu64, 64);
            if c.cc_l() { blk = 505; continue; }
            blk = 498; continue;
        }
        498 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r787 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r787);
            let r788 = c.r32(2) & 0x8u64;
            c.w32(2, r788);
            c.logic(r788, 32);
            let r789 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r789);
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 499; continue;
        }
        499 => {
            let Some(t790) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4609); };
            c.w64(9, t790);
            let Some(t791) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d4610); };
            c.w64(12, t791);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t792) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4618); };
            c.w64(0, t792);
            let Some(t793) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d461f); };
            c.cmp(t793, c.r64(15), 64);
            if c.cc_be() { blk = 676; continue; }
            blk = 500; continue;
        }
        500 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008d4628); }
            let r794 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r794);
            blk = 501; continue;
        }
        501 => {
            let Some(t795) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d462f); };
            c.w64(15, t795);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008d4636); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d463b); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d4644); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008d4649); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008d464e); }
            blk = 595; continue;
        }
        502 => {
            let Some(t796) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4658); };
            c.w64(9, t796);
            let Some(t797) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d465f); };
            c.w64(12, t797);
            c.w64(0, c.r64(12).wrapping_add(0x14));
            let Some(t798) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4667); };
            c.w64(15, t798);
            let Some(t799) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d466e); };
            c.cmp(t799, c.r64(0), 64);
            if c.cc_be() { blk = 678; continue; }
            blk = 503; continue;
        }
        503 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x70008d4677); }
            let r800 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r800);
            blk = 504; continue;
        }
        504 => {
            let Some(t801) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d467e); };
            c.w64(15, t801);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008d4685); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d468a); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d4693); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008d4698); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008d469d); }
            blk = 595; continue;
        }
        505 => {
            let Some(t802) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d46a7); };
            c.w64(9, t802);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 535; continue; }
            blk = 506; continue;
        }
        506 => {
            c.w32(12, c.r32(1));
            let r803 = c.r32(2) & 0x8u64;
            c.w32(2, r803);
            c.logic(r803, 32);
            c.cmp(c.r32(12), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 522; continue; }
            blk = 507; continue;
        }
        507 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 516; continue; }
            blk = 508; continue;
        }
        508 => {
            c.w32(12, c.r32(1));
            c.w64(0, c.r64(12));
            let r804 = c.add(c.r32(0), c.r32(12), 32);
            c.w32(0, r804);
            if c.cc_o() { blk = 515; continue; }
            blk = 509; continue;
        }
        509 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(0, 0xcu64);
            let r805 = c.add(c.r64(12), c.r64(0), 64);
            c.w64(12, r805);
            if c.cc_o() { blk = 514; continue; }
            blk = 510; continue;
        }
        510 => {
            let Some(t806) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d46f1); };
            c.w64(8, t806);
            let Some(t807) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d46f8); };
            c.w64(0, t807);
            let r808 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r808);
            let r809 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r809);
            c.logic(r809, 64);
            c.w64(3, c.r64(0).wrapping_add(c.r64(12)));
            let Some(t810) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d4707); };
            c.cmp(t810, c.r64(3), 64);
            if c.cc_be() { blk = 680; continue; }
            blk = 511; continue;
        }
        511 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 680; continue; }
            blk = 512; continue;
        }
        512 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008d471d); }
            c.w64(12, c.r64(0).wrapping_add(0x1));
            blk = 513; continue;
        }
        513 => {
            let Some(t811) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d4724); };
            c.w64(0, t811);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008d472b); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d4730); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d4739); }
            c.w64(0, c.r64(12));
            blk = 517; continue;
        }
        514 => {
            return RegfileExit::fault(0x70008d4743);
        }
        515 => {
            return RegfileExit::fault(0x70008d4744);
        }
        516 => {
            let Some(t812) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d4745); };
            c.w64(0, t812);
            blk = 517; continue;
        }
        517 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r813 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r813);
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0xb));
            c.w64(12, c.r64(11).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_e() { blk = 534; continue; }
            blk = 518; continue;
        }
        518 => {
            c.w32(15, 0xbu64);
            blk = 520; continue;
        }
        519 => {
            let r814 = c.add(c.r64(15), 0x2u64, 64);
            c.w64(15, r814);
            blk = 520; continue;
        }
        520 => {
            let Some(t815) = c.ld16(c.r64(7).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008d4776); };
            c.w32(3, t815);
            if !c.st16(c.r64(0).wrapping_add(c.r64(15)), c.r16(3)) { return RegfileExit::fault(0x70008d477b); }
            let r816 = c.add(c.r64(11), 0x2u64, 64);
            c.w64(11, r816);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ne() { blk = 519; continue; }
            blk = 521; continue;
        }
        521 => {
            blk = 534; continue;
        }
        522 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 529; continue; }
            blk = 523; continue;
        }
        523 => {
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(0, 0xcu64);
            let r817 = c.add(c.r64(12), c.r64(0), 64);
            c.w64(12, r817);
            if c.cc_o() { blk = 528; continue; }
            blk = 524; continue;
        }
        524 => {
            let Some(t818) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d47a1); };
            c.w64(8, t818);
            let Some(t819) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d47a8); };
            c.w64(0, t819);
            let r820 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r820);
            let r821 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r821);
            c.logic(r821, 64);
            c.w64(3, c.r64(0).wrapping_add(c.r64(12)));
            let Some(t822) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d47b7); };
            c.cmp(t822, c.r64(3), 64);
            if c.cc_be() { blk = 682; continue; }
            blk = 525; continue;
        }
        525 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 682; continue; }
            blk = 526; continue;
        }
        526 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008d47cd); }
            c.w64(12, c.r64(0).wrapping_add(0x1));
            blk = 527; continue;
        }
        527 => {
            let Some(t823) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d47d4); };
            c.w64(0, t823);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008d47db); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d47e0); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d47e9); }
            c.w64(0, c.r64(12));
            blk = 530; continue;
        }
        528 => {
            return RegfileExit::fault(0x70008d47f3);
        }
        529 => {
            let Some(t824) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d47f4); };
            c.w64(0, t824);
            blk = 530; continue;
        }
        530 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(11, c.r64(11).wrapping_add(c.r64(12)).wrapping_add(0xb));
            c.w64(12, c.r64(11).wrapping_add(c.r64(1)));
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_e() { blk = 534; continue; }
            blk = 531; continue;
        }
        531 => {
            c.w32(15, 0xbu64);
            blk = 533; continue;
        }
        532 => {
            let r825 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r825);
            blk = 533; continue;
        }
        533 => {
            let Some(t826) = c.ld8(c.r64(7).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008d481b); };
            c.w32(3, t826);
            if !c.st8(c.r64(0).wrapping_add(c.r64(15)), c.r8(3)) { return RegfileExit::fault(0x70008d4820); }
            let r827 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r827);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ne() { blk = 532; continue; }
            blk = 534; continue;
        }
        534 => {
            c.w64(12, c.r64(0));
            blk = 595; continue;
        }
        535 => {
            let Some(t828) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d4835); };
            c.w64(6, t828);
            let Some(t829) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d483c); };
            c.w64(8, t829);
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 593; continue; }
            blk = 536; continue;
        }
        536 => {
            c.w32(12, c.r32(1));
            let Some(t830) = c.ld64(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d484f); };
            c.w64(0, t830);
            let r831 = c.r32(2) & 0x8u64;
            c.w32(2, r831);
            c.logic(r831, 32);
            c.cmp(c.r32(12), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 552; continue; }
            blk = 537; continue;
        }
        537 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 546; continue; }
            blk = 538; continue;
        }
        538 => {
            c.w32(12, c.r32(1));
            c.w64(3, c.r64(12));
            let r832 = c.add(c.r32(3), c.r32(12), 32);
            c.w32(3, r832);
            if c.cc_o() { blk = 545; continue; }
            blk = 539; continue;
        }
        539 => {
            c.w64(12, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(3, 0xcu64);
            let r833 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r833);
            if c.cc_o() { blk = 544; continue; }
            blk = 540; continue;
        }
        540 => {
            let Some(t834) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4889); };
            c.w64(3, t834);
            let Some(t835) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d4890); };
            c.w64(2, t835);
            let r836 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r836);
            let r837 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r837);
            c.logic(r837, 64);
            c.w64(6, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t838) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d489f); };
            c.cmp(t838, c.r64(6), 64);
            if c.cc_be() { blk = 684; continue; }
            blk = 541; continue;
        }
        541 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 684; continue; }
            blk = 542; continue;
        }
        542 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008d48b5); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 543; continue;
        }
        543 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008d48bc); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d48c1); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d48ca); }
            blk = 547; continue;
        }
        544 => {
            return RegfileExit::fault(0x70008d48d1);
        }
        545 => {
            return RegfileExit::fault(0x70008d48d2);
        }
        546 => {
            let Some(t839) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d48d3); };
            c.w64(12, t839);
            blk = 547; continue;
        }
        547 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r840 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r840);
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0xb));
            c.w64(15, c.r64(11).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 564; continue; }
            blk = 548; continue;
        }
        548 => {
            c.w32(3, 0xbu64);
            blk = 550; continue;
        }
        549 => {
            let r841 = c.add(c.r64(3), 0x2u64, 64);
            c.w64(3, r841);
            blk = 550; continue;
        }
        550 => {
            let Some(t842) = c.ld16(c.r64(11).wrapping_add(c.r64(0)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d4903); };
            c.w32(2, t842);
            if !c.st16(c.r64(12).wrapping_add(c.r64(3)), c.r16(2)) { return RegfileExit::fault(0x70008d4909); }
            let r843 = c.add(c.r64(11), 0x2u64, 64);
            c.w64(11, r843);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ne() { blk = 549; continue; }
            blk = 551; continue;
        }
        551 => {
            blk = 564; continue;
        }
        552 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 559; continue; }
            blk = 553; continue;
        }
        553 => {
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w32(3, 0xcu64);
            let r844 = c.add(c.r64(12), c.r64(3), 64);
            c.w64(12, r844);
            if c.cc_o() { blk = 558; continue; }
            blk = 554; continue;
        }
        554 => {
            let Some(t845) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d492f); };
            c.w64(3, t845);
            let Some(t846) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d4936); };
            c.w64(2, t846);
            let r847 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r847);
            let r848 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r848);
            c.logic(r848, 64);
            c.w64(7, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t849) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d4945); };
            c.cmp(t849, c.r64(7), 64);
            if c.cc_be() { blk = 686; continue; }
            blk = 555; continue;
        }
        555 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 686; continue; }
            blk = 556; continue;
        }
        556 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008d495b); }
            c.w64(12, c.r64(2).wrapping_add(0x1));
            blk = 557; continue;
        }
        557 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008d4962); }
            if !c.st32(c.r64(12).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d4967); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008d4970); }
            blk = 560; continue;
        }
        558 => {
            return RegfileExit::fault(0x70008d4977);
        }
        559 => {
            let Some(t850) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d4978); };
            c.w64(12, t850);
            blk = 560; continue;
        }
        560 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(11, c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xb));
            c.w64(15, c.r64(11).wrapping_add(c.r64(1)));
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 564; continue; }
            blk = 561; continue;
        }
        561 => {
            c.w32(3, 0xbu64);
            blk = 563; continue;
        }
        562 => {
            let r851 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r851);
            blk = 563; continue;
        }
        563 => {
            let Some(t852) = c.ld8(c.r64(11).wrapping_add(c.r64(0)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d499e); };
            c.w32(2, t852);
            if !c.st8(c.r64(12).wrapping_add(c.r64(3)), c.r8(2)) { return RegfileExit::fault(0x70008d49a4); }
            let r853 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r853);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ne() { blk = 562; continue; }
            blk = 564; continue;
        }
        564 => {
            let Some(t854) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d49b1); };
            c.w64(6, t854);
            blk = 595; continue;
        }
        565 => {
            c.w32(12, c.r16(2));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 581; continue; }
            blk = 566; continue;
        }
        566 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(0));
            blk = 567; continue;
        }
        567 => {
            c.w64(2, c.r64(3));
            let r855 = c.r32(2) & 0x7u64;
            c.w32(2, r855);
            c.logic(r855, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 572; continue; }
            blk = 568; continue;
        }
        568 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 580; continue; }
            blk = 569; continue;
        }
        569 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 577; continue; }
            blk = 570; continue;
        }
        570 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 578; continue; }
            blk = 571; continue;
        }
        571 => {
            blk = 690; continue;
        }
        572 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 579; continue; }
            blk = 573; continue;
        }
        573 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 575; continue; }
            blk = 574; continue;
        }
        574 => {
            blk = 690; continue;
        }
        575 => {
            let Some(t856) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d4a03); };
            c.w32(3, t856);
            let Some(t857) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d4a08); };
            c.cmp(t857, c.r32(3), 32);
            if c.cc_ne() { blk = 690; continue; }
            blk = 576; continue;
        }
        576 => {
            let Some(t858) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d4a15); };
            c.w32(12, t858);
            let r859 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r859);
            let Some(t860) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d4a1d); };
            c.w32(3, t860);
            let r861 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r861);
            let Some(t862) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d4a25); };
            c.w32(3, t862);
            blk = 567; continue;
        }
        577 => {
            let Some(t863) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d4a2b); };
            c.w32(3, t863);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r864 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r864);
            let Some(t865) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d4a39); };
            c.w32(12, t865);
            let r866 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r866);
            let Some(t867) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d4a41); };
            c.w32(3, t867);
            let r868 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r868);
            let Some(t869) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d4a49); };
            c.w32(3, t869);
            blk = 567; continue;
        }
        578 => {
            let Some(t870) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d4a4f); };
            c.w32(12, t870);
            let r871 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r871);
            let Some(t872) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d4a57); };
            c.w32(3, t872);
            let r873 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r873);
            let Some(t874) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d4a5f); };
            c.w32(3, t874);
            blk = 567; continue;
        }
        579 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 582; continue;
        }
        580 => {
            c.w32(2, 0x1u64);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 582; continue;
        }
        581 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(0));
            blk = 582; continue;
        }
        582 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 688; continue; }
            blk = 583; continue;
        }
        583 => {
            c.w64(0, c.r64(3).wrapping_add(0xb));
            blk = 584; continue;
        }
        584 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r875 = c.r32(12) & 0x8u64;
            c.w32(12, r875);
            c.logic(r875, 32);
            let r876 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r876);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 586; continue; }
            blk = 585; continue;
        }
        585 => {
            let Some(t877) = c.ld16(c.r64(0).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d4aad); };
            c.w32(11, t877);
            blk = 587; continue;
        }
        586 => {
            let Some(t878) = c.ld8(c.r64(11).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x70008d4ab4); };
            c.w32(11, t878);
            blk = 587; continue;
        }
        587 => {
            c.cmp(c.r32(11), 0xffu64, 32);
            if c.cc_g() { blk = 703; continue; }
            blk = 588; continue;
        }
        588 => {
            c.w64(12, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t879) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d4ace); };
            c.w64(15, t879);
            let Some(t880) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d4ad5); };
            c.w32(0, t880);
            let r881 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r881);
            let Some(t882) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d4add); };
            c.cmp(t882, c.r32(0), 32);
            if c.cc_e() { blk = 708; continue; }
            blk = 589; continue;
        }
        589 => {
            c.w64(12, c.r64(0));
            blk = 595; continue;
        }
        590 => {
            let Some(t883) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d4aef); };
            c.w64(0, t883);
            c.w64(12, c.r64(0));
            blk = 595; continue;
        }
        591 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_a() { blk = 593; continue; }
            blk = 592; continue;
        }
        592 => {
            c.w64(12, c.r64(0));
            blk = 595; continue;
        }
        593 => {
            c.w32(11, c.r64(3).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d4b0a); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d4b0b); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008d4b0d); }
            c.w32(0, 0x3u64);
            let Some(t884) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008d4b14); };
            c.w64(3, t884);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d4b22, 0x70008d4b1d);
        }
        594 => {
            c.w64(12, c.r64(0));
            let Some(t885) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4b25); };
            c.w64(6, t885);
            blk = 595; continue;
        }
        595 => {
            let Some(t886) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d4b29); };
            c.w64(0, t886);
            c.w64(3, c.r64(12));
            return c.call(0x700086f9d4, 0x70008d4b35, 0x70008d4b30);
        }
        596 => {
            blk = 62; continue;
        }
        597 => {
            return RegfileExit::fault(0x70008d4b3a);
        }
        598 => {
            return RegfileExit::fault(0x70008d4b3b);
        }
        599 => {
            return RegfileExit::fault(0x70008d4b3c);
        }
        600 => {
            return RegfileExit::fault(0x70008d4b3d);
        }
        601 => {
            return RegfileExit::fault(0x70008d4b3e);
        }
        602 => {
            return RegfileExit::fault(0x70008d4b3f);
        }
        603 => {
            return RegfileExit::fault(0x70008d4b40);
        }
        604 => {
            return RegfileExit::fault(0x70008d4b41);
        }
        605 => {
            return RegfileExit::fault(0x70008d4b42);
        }
        606 => {
            return RegfileExit::fault(0x70008d4b43);
        }
        607 => {
            return RegfileExit::fault(0x70008d4b44);
        }
        608 => {
            return RegfileExit::fault(0x70008d4b45);
        }
        609 => {
            return RegfileExit::fault(0x70008d4b46);
        }
        610 => {
            return RegfileExit::fault(0x70008d4b47);
        }
        611 => {
            return RegfileExit::fault(0x70008d4b48);
        }
        612 => {
            return RegfileExit::fault(0x70008d4b49);
        }
        613 => {
            return RegfileExit::fault(0x70008d4b4a);
        }
        614 => {
            return RegfileExit::fault(0x70008d4b4b);
        }
        615 => {
            return RegfileExit::fault(0x70008d4b4c);
        }
        616 => {
            return RegfileExit::fault(0x70008d4b4d);
        }
        617 => {
            return RegfileExit::fault(0x70008d4b4e);
        }
        618 => {
            c.w32(11, 0x36u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d4b55); }
            let Some(t887) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d4b57); };
            c.w64(11, t887);
            let Some(t888) = c.ld32(c.r64(11).wrapping_add(0x37f)) else { return RegfileExit::fault(0x70008d4b5e); };
            c.w32(11, t888);
            let r889 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r889);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d4b68); }
            let Some(t890) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d4b6a); };
            c.w64(3, t890);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d4b7b, 0x70008d4b76);
        }
        619 => {
            return RegfileExit::fault(0x70008d4b7b);
        }
        620 => {
            let Some(t891) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d4b7c); };
            c.w64(3, t891);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d4b8a, 0x70008d4b85);
        }
        621 => {
            let Some(t892) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4b8a); };
            c.w64(6, t892);
            blk = 1; continue;
        }
        622 => {
            let Some(t893) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d4b93); };
            c.logic(t893 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 623; continue;
        }
        623 => {
            blk = 16; continue;
        }
        624 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d4ba5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 16; continue; }
            blk = 625; continue;
        }
        625 => {
            blk = 22; continue;
        }
        626 => {
            let Some(t894) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4bbd); };
            c.w64(6, t894);
            return c.call(0x700089c794, 0x70008d4bc6, 0x70008d4bc1);
        }
        627 => {
            blk = 47; continue;
        }
        628 => {
            return c.call(0x700089c794, 0x70008d4bd0, 0x70008d4bcb);
        }
        629 => {
            let Some(t895) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d4bd0); };
            c.w64(12, t895);
            let Some(t896) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4bd4); };
            c.w64(6, t896);
            let Some(t897) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d4bd8); };
            c.w64(11, t897);
            blk = 70; continue;
        }
        630 => {
            c.w64(0, c.r64(11));
            return c.call(0x700089c794, 0x70008d4be9, 0x70008d4be4);
        }
        631 => {
            c.w64(11, c.r64(0));
            let Some(t898) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4bec); };
            c.w64(6, t898);
            let Some(t899) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4bf0); };
            c.w64(0, t899);
            blk = 72; continue;
        }
        632 => {
            c.w64(0, c.r64(12));
            return c.call(0x700089c794, 0x70008d4c01, 0x70008d4bfc);
        }
        633 => {
            c.w64(12, c.r64(0));
            let Some(t900) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4c04); };
            c.w64(6, t900);
            let Some(t901) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4c08); };
            c.w64(0, t901);
            let Some(t902) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008d4c0c); };
            c.w64(11, t902);
            let Some(t903) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008d4c10); };
            c.w64(3, t903);
            blk = 79; continue;
        }
        634 => {
            let Some(t904) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4c1c); };
            c.w64(6, t904);
            return c.call(0x700089c794, 0x70008d4c25, 0x70008d4c20);
        }
        635 => {
            blk = 235; continue;
        }
        636 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d4c2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d4c2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d4c32); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d4c40, 0x70008d4c3b);
        }
        637 => {
            c.w64(15, c.r64(0));
            let Some(t905) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4c43); };
            c.w64(12, t905);
            let Some(t906) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4c47); };
            c.w64(8, t906);
            let Some(t907) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4c4e); };
            c.w64(9, t907);
            let Some(t908) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4c55); };
            c.w64(0, t908);
            let Some(t909) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4c59); };
            c.w64(2, t909);
            let Some(t910) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4c5d); };
            c.w64(6, t910);
            let Some(t911) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4c61); };
            c.w64(11, t911);
            blk = 259; continue;
        }
        638 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d4c6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d4c6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d4c72); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d4c80, 0x70008d4c7b);
        }
        639 => {
            c.w64(15, c.r64(0));
            let Some(t912) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4c83); };
            c.w64(12, t912);
            let Some(t913) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4c87); };
            c.w64(8, t913);
            let Some(t914) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4c8e); };
            c.w64(9, t914);
            let Some(t915) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4c95); };
            c.w64(0, t915);
            let Some(t916) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4c99); };
            c.w64(2, t916);
            let Some(t917) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4c9d); };
            c.w64(6, t917);
            let Some(t918) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4ca1); };
            c.w64(11, t918);
            blk = 262; continue;
        }
        640 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d4caa); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008d4cae); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d4cb2); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008d4cbe, 0x70008d4cb9);
        }
        641 => {
            c.w64(11, c.r64(0));
            let Some(t919) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4cc1); };
            c.w64(12, t919);
            let Some(t920) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4cc5); };
            c.w64(7, t920);
            let Some(t921) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4ccc); };
            c.w64(9, t921);
            let Some(t922) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d4cd3); };
            c.w64(8, t922);
            let Some(t923) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4cda); };
            c.w64(0, t923);
            let Some(t924) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4cde); };
            c.w64(2, t924);
            let Some(t925) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d4ce2); };
            c.w64(15, t925);
            let Some(t926) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4ce6); };
            c.w64(6, t926);
            blk = 271; continue;
        }
        642 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d4cef); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008d4cf3); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008d4cf7); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008d4d03, 0x70008d4cfe);
        }
        643 => {
            c.w64(11, c.r64(0));
            let Some(t927) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4d06); };
            c.w64(12, t927);
            let Some(t928) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4d0a); };
            c.w64(7, t928);
            let Some(t929) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4d11); };
            c.w64(9, t929);
            let Some(t930) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d4d18); };
            c.w64(8, t930);
            let Some(t931) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4d1f); };
            c.w64(0, t931);
            let Some(t932) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4d23); };
            c.w64(2, t932);
            let Some(t933) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d4d27); };
            c.w64(15, t933);
            let Some(t934) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d4d2b); };
            c.w64(6, t934);
            blk = 283; continue;
        }
        644 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008d4d34); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d4d38); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008d4d3c); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008d4d48, 0x70008d4d43);
        }
        645 => {
            c.w64(11, c.r64(0));
            let Some(t935) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d4d4b); };
            c.w64(12, t935);
            let Some(t936) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4d52); };
            c.w64(6, t936);
            let Some(t937) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4d59); };
            c.w64(9, t937);
            let Some(t938) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d4d60); };
            c.w64(7, t938);
            let Some(t939) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d4d67); };
            c.w64(8, t939);
            let Some(t940) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4d6e); };
            c.w64(3, t940);
            let Some(t941) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4d72); };
            c.w64(2, t941);
            let Some(t942) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d4d76); };
            c.w64(15, t942);
            blk = 299; continue;
        }
        646 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008d4d7f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d4d83); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008d4d87); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008d4d93, 0x70008d4d8e);
        }
        647 => {
            c.w64(11, c.r64(0));
            let Some(t943) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d4d96); };
            c.w64(12, t943);
            let Some(t944) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4d9d); };
            c.w64(6, t944);
            let Some(t945) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4da4); };
            c.w64(9, t945);
            let Some(t946) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d4dab); };
            c.w64(7, t946);
            let Some(t947) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d4db2); };
            c.w64(8, t947);
            let Some(t948) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4db9); };
            c.w64(3, t948);
            let Some(t949) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4dbd); };
            c.w64(2, t949);
            let Some(t950) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d4dc1); };
            c.w64(15, t950);
            blk = 311; continue;
        }
        648 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 650; continue; }
            blk = 649; continue;
        }
        649 => {
            let Some(t951) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d4dd0); };
            c.w64(3, t951);
            blk = 338; continue;
        }
        650 => {
            let Some(t952) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d4dd9); };
            c.w64(11, (t952 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 652; continue; }
            blk = 651; continue;
        }
        651 => {
            let r953 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r953);
            blk = 661; continue;
        }
        652 => {
            let Some(t954) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4df1); };
            c.w64(15, t954);
            let Some(t955) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d4df8); };
            c.w64(3, t955);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 656; continue; }
            blk = 653; continue;
        }
        653 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 655; continue; }
            blk = 654; continue;
        }
        654 => {
            let r956 = c.r64(10) | 0x1u64;
            c.w64(10, r956);
            c.logic(r956, 64);
            blk = 655; continue;
        }
        655 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 656; continue;
        }
        656 => {
            c.w64(11, c.r64(3).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008d4e26); }
            let Some(t957) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4e2b); };
            c.w64(9, t957);
            let Some(t958) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d4e32); };
            c.cmp(t958, c.r64(11), 64);
            if c.cc_a() { blk = 659; continue; }
            blk = 657; continue;
        }
        657 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008d4e41, 0x70008d4e3c);
        }
        658 => {
            c.w64(11, c.r64(0));
            let Some(t959) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4e44); };
            c.w64(12, t959);
            let Some(t960) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4e48); };
            c.w64(15, t960);
            let Some(t961) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4e4f); };
            c.w64(9, t961);
            let Some(t962) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4e56); };
            c.w64(0, t962);
            blk = 660; continue;
        }
        659 => {
            if !c.st64(c.r64(15), c.r64(11)) { return RegfileExit::fault(0x70008d4e5c); }
            c.w64(11, c.r64(3).wrapping_add(0x1));
            blk = 660; continue;
        }
        660 => {
            let Some(t963) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d4e63); };
            c.w64(3, t963);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008d4e6a); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008d4e6e); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d4e73); }
            blk = 661; continue;
        }
        661 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008d4e79); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d4e7b); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t964) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d4e7f); };
            c.w64(3, t964);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008d4e93, 0x70008d4e8e);
        }
        662 => {
            let r965 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r965);
            c.w64(11, c.r64(0));
            let Some(t966) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4e98); };
            c.w64(12, t966);
            let Some(t967) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4e9c); };
            c.w64(0, t967);
            blk = 341; continue;
        }
        663 => {
            let Some(t968) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4ea5); };
            c.w64(15, t968);
            let Some(t969) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d4eac); };
            c.w64(3, t969);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d4eb3); }
            let Some(t970) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4eb7); };
            c.w64(1, t970);
            let Some(t971) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d4ebe); };
            c.cmp(t971, c.r64(2), 64);
            if c.cc_a() { blk = 666; continue; }
            blk = 664; continue;
        }
        664 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d4ecd, 0x70008d4ec8);
        }
        665 => {
            c.w64(3, c.r64(0));
            let Some(t972) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4ed0); };
            c.w64(12, t972);
            let Some(t973) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4ed4); };
            c.w64(15, t973);
            let Some(t974) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4edb); };
            c.w64(1, t974);
            let Some(t975) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4ee2); };
            c.w64(0, t975);
            let Some(t976) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4ee6); };
            c.w64(11, t976);
            blk = 667; continue;
        }
        666 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008d4eec); }
            let r977 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r977);
            blk = 667; continue;
        }
        667 => {
            let Some(t978) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d4ef3); };
            c.w64(7, t978);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008d4efa); }
            if !c.st32(c.r64(3).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d4efd); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d4f04); }
            if !c.st16(c.r64(3).wrapping_add(0xb), c.r16(11)) { return RegfileExit::fault(0x70008d4f0b); }
            let Some(t979) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d4f10); };
            c.w64(2, t979);
            blk = 343; continue;
        }
        668 => {
            let Some(t980) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4f1c); };
            c.w64(3, t980);
            let Some(t981) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d4f23); };
            c.w64(1, t981);
            c.w64(6, c.r64(1).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008d4f2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008d4f2e); }
            let Some(t982) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4f32); };
            c.w64(7, t982);
            let Some(t983) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d4f39); };
            c.cmp(t983, c.r64(6), 64);
            if c.cc_a() { blk = 671; continue; }
            blk = 669; continue;
        }
        669 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d4f48, 0x70008d4f43);
        }
        670 => {
            c.w64(3, c.r64(0));
            let Some(t984) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4f4b); };
            c.w64(12, t984);
            let Some(t985) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d4f4f); };
            c.w64(2, t985);
            let Some(t986) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4f56); };
            c.w64(11, t986);
            let Some(t987) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d4f5a); };
            c.w64(8, t987);
            let Some(t988) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d4f61); };
            c.w64(7, t988);
            let Some(t989) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4f68); };
            c.w64(0, t989);
            let Some(t990) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d4f6c); };
            c.w64(15, t990);
            blk = 672; continue;
        }
        671 => {
            c.w64(8, c.r64(3));
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x70008d4f75); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 672; continue;
        }
        672 => {
            let Some(t991) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d4f7c); };
            c.w64(1, t991);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d4f83); }
            if !c.st32(c.r64(3).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d4f86); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d4f8d); }
            if !c.st8(c.r64(3).wrapping_add(0xb), c.r8(11)) { return RegfileExit::fault(0x70008d4f94); }
            let Some(t992) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d4f98); };
            c.w32(6, t992);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(11), c.r64(6), 64);
            if c.cc_ae() { blk = 717; continue; }
            blk = 673; continue;
        }
        673 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(15)), c.r32(3)) { return RegfileExit::fault(0x70008d4faa); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 675; continue; }
            blk = 674; continue;
        }
        674 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r993 = c.r64(11) & c.r64(2);
            c.w64(11, r993);
            c.logic(r993, 64);
            let Some(t994) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d4fbd); };
            c.logic(t994 & 0x4u64, 8);
            if c.cc_ne() { blk = 721; continue; }
            blk = 675; continue;
        }
        675 => {
            blk = 343; continue;
        }
        676 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008d4fcd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d4fd1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008d4fd5); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d4fe3, 0x70008d4fde);
        }
        677 => {
            c.w64(12, c.r64(0));
            let Some(t995) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d4fe6); };
            c.w64(6, t995);
            let Some(t996) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d4fea); };
            c.w64(1, t996);
            let Some(t997) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d4fee); };
            c.w64(7, t997);
            let Some(t998) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d4ff2); };
            c.w64(11, t998);
            blk = 501; continue;
        }
        678 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008d4ffb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d4fff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008d5003); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d5011, 0x70008d500c);
        }
        679 => {
            c.w64(12, c.r64(0));
            let Some(t999) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5014); };
            c.w64(6, t999);
            let Some(t1000) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5018); };
            c.w64(1, t1000);
            let Some(t1001) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d501c); };
            c.w64(7, t1001);
            let Some(t1002) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5020); };
            c.w64(11, t1002);
            blk = 504; continue;
        }
        680 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008d5029); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d502d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d5031); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008d503d, 0x70008d5038);
        }
        681 => {
            c.w64(12, c.r64(0));
            let Some(t1003) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5040); };
            c.w64(6, t1003);
            let Some(t1004) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5044); };
            c.w64(1, t1004);
            let Some(t1005) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5048); };
            c.w64(11, t1005);
            let Some(t1006) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d504c); };
            c.w64(7, t1006);
            let Some(t1007) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d5050); };
            c.w64(15, t1007);
            blk = 513; continue;
        }
        682 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008d5059); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d505d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d5061); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008d506d, 0x70008d5068);
        }
        683 => {
            c.w64(12, c.r64(0));
            let Some(t1008) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5070); };
            c.w64(6, t1008);
            let Some(t1009) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5074); };
            c.w64(1, t1009);
            let Some(t1010) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5078); };
            c.w64(11, t1010);
            let Some(t1011) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d507c); };
            c.w64(7, t1011);
            let Some(t1012) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d5080); };
            c.w64(15, t1012);
            blk = 527; continue;
        }
        684 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d5089); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008d508d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d5091); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008d509d, 0x70008d5098);
        }
        685 => {
            c.w64(12, c.r64(0));
            let Some(t1013) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d50a0); };
            c.w64(8, t1013);
            let Some(t1014) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d50a7); };
            c.w64(0, t1014);
            let Some(t1015) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d50ab); };
            c.w64(1, t1015);
            let Some(t1016) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d50af); };
            c.w64(11, t1016);
            let Some(t1017) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d50b3); };
            c.w64(15, t1017);
            blk = 543; continue;
        }
        686 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d50bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008d50c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d50c4); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008d50d0, 0x70008d50cb);
        }
        687 => {
            c.w64(12, c.r64(0));
            let Some(t1018) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d50d3); };
            c.w64(6, t1018);
            let Some(t1019) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d50da); };
            c.w64(0, t1019);
            let Some(t1020) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d50de); };
            c.w64(1, t1020);
            let Some(t1021) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d50e2); };
            c.w64(11, t1021);
            let Some(t1022) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d50e6); };
            c.w64(15, t1022);
            blk = 557; continue;
        }
        688 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 690; continue; }
            blk = 689; continue;
        }
        689 => {
            let Some(t1023) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d50f5); };
            c.w64(0, t1023);
            blk = 584; continue;
        }
        690 => {
            c.w64(11, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d5104); }
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 692; continue; }
            blk = 691; continue;
        }
        691 => {
            let r1024 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r1024);
            blk = 701; continue;
        }
        692 => {
            let Some(t1025) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5119); };
            c.w64(12, t1025);
            let Some(t1026) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d5120); };
            c.w64(15, t1026);
            c.w64(2, c.r64(15).wrapping_add(0xc));
            let Some(t1027) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d5128); };
            c.w64(3, t1027);
            let Some(t1028) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d512f); };
            c.cmp(t1028, c.r64(2), 64);
            if c.cc_a() { blk = 695; continue; }
            blk = 693; continue;
        }
        693 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008d513e, 0x70008d5139);
        }
        694 => {
            c.w64(15, c.r64(0));
            let Some(t1029) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5141); };
            c.w64(6, t1029);
            let Some(t1030) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5145); };
            c.w64(0, t1030);
            let Some(t1031) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5149); };
            c.w64(12, t1031);
            let Some(t1032) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d5150); };
            c.w64(3, t1032);
            let Some(t1033) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5157); };
            c.w64(11, t1033);
            blk = 696; continue;
        }
        695 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x70008d515d); }
            let r1034 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r1034);
            blk = 696; continue;
        }
        696 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 700; continue; }
            blk = 697; continue;
        }
        697 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 699; continue; }
            blk = 698; continue;
        }
        698 => {
            let r1035 = c.r64(10) | 0x1u64;
            c.w64(10, r1035);
            c.logic(r1035, 64);
            blk = 699; continue;
        }
        699 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 700; continue;
        }
        700 => {
            let Some(t1036) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d518c); };
            c.w64(11, t1036);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d5193); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d5197); }
            c.w64(11, c.r64(15));
            blk = 701; continue;
        }
        701 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d51a0); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d51a1); }
            c.w32(0, 0x2u64);
            let Some(t1037) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d51a8); };
            c.w64(3, t1037);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d51b6, 0x70008d51b1);
        }
        702 => {
            let r1038 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r1038);
            c.w64(11, c.r64(0));
            let Some(t1039) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d51bb); };
            c.w64(6, t1039);
            blk = 587; continue;
        }
        703 => {
            let Some(t1040) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d51c4); };
            c.w64(12, t1040);
            let Some(t1041) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d51cb); };
            c.w64(15, t1041);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d51d3); }
            let Some(t1042) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d51d7); };
            c.w64(3, t1042);
            let Some(t1043) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d51de); };
            c.cmp(t1043, c.r64(0), 64);
            if c.cc_a() { blk = 706; continue; }
            blk = 704; continue;
        }
        704 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d51ed, 0x70008d51e8);
        }
        705 => {
            blk = 707; continue;
        }
        706 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008d51ef); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 707; continue;
        }
        707 => {
            let Some(t1044) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d51fa); };
            c.w64(11, t1044);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d5201); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d5205); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d520c); }
            let Some(t1045) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5213); };
            c.w64(11, t1045);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(11)) { return RegfileExit::fault(0x70008d5217); }
            let Some(t1046) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d521c); };
            c.w64(6, t1046);
            blk = 589; continue;
        }
        708 => {
            let Some(t1047) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5225); };
            c.w64(0, t1047);
            let Some(t1048) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d522c); };
            c.w64(3, t1048);
            c.w64(1, c.r64(3).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008d5233); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008d5237); }
            let Some(t1049) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d523b); };
            c.w64(2, t1049);
            let Some(t1050) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008d5242); };
            c.cmp(t1050, c.r64(1), 64);
            if c.cc_a() { blk = 711; continue; }
            blk = 709; continue;
        }
        709 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d5251, 0x70008d524c);
        }
        710 => {
            blk = 712; continue;
        }
        711 => {
            if !c.st64(c.r64(0), c.r64(1)) { return RegfileExit::fault(0x70008d5253); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 712; continue;
        }
        712 => {
            let Some(t1051) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d525a); };
            c.w64(11, t1051);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d5261); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d5265); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d526c); }
            let Some(t1052) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5273); };
            c.w64(11, t1052);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(11)) { return RegfileExit::fault(0x70008d5277); }
            let Some(t1053) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d527b); };
            c.w64(15, t1053);
            let Some(t1054) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d5282); };
            c.w32(12, t1054);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 714; continue; }
            blk = 713; continue;
        }
        713 => {
            return RegfileExit::fault(0x70008d5291);
        }
        714 => {
            let Some(t1055) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5292); };
            c.w64(12, t1055);
            if !c.st32(c.r64(15).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x70008d5296); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 716; continue; }
            blk = 715; continue;
        }
        715 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r1056 = c.r64(11) & c.r64(15);
            c.w64(11, r1056);
            c.logic(r1056, 64);
            let Some(t1057) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d52a8); };
            c.logic(t1057 & 0x4u64, 8);
            if c.cc_ne() { blk = 718; continue; }
            blk = 716; continue;
        }
        716 => {
            let Some(t1058) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d52af); };
            c.w64(6, t1058);
            blk = 589; continue;
        }
        717 => {
            return RegfileExit::fault(0x70008d52b8);
        }
        718 => {
            c.w32(0, c.r32(0));
            let r1059 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1059);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1060 = c.r64(11) & c.r64(0);
            c.w64(11, r1060);
            c.logic(r1060, 64);
            let Some(t1061) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d52c8); };
            c.logic(t1061 & 0x2u64, 8);
            if c.cc_e() { blk = 716; continue; }
            blk = 719; continue;
        }
        719 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d52d3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d52d4); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008d52e1, 0x70008d52dc);
        }
        720 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008d52e1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008d52e3); }
            blk = 716; continue;
        }
        721 => {
            c.w32(3, c.r32(3));
            let r1062 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1062);
            c.w64(11, 0xfffffffffffc0000u64);
            let r1063 = c.r64(11) & c.r64(3);
            c.w64(11, r1063);
            c.logic(r1063, 64);
            let Some(t1064) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d52f5); };
            c.logic(t1064 & 0x2u64, 8);
            if c.cc_e() { blk = 675; continue; }
            blk = 722; continue;
        }
        722 => {
            c.w64(6, c.r64(2).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d5304); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d5305); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008d5312, 0x70008d530d);
        }
        723 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008d5312); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008d5314); }
            blk = 675; continue;
        }
        724 => {
            return RegfileExit::jump(0x70008d531c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d5334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d5334); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d5338); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d5339); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d533a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008d533f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d5343); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 185; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d535b); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d5360); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 192; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d536e); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5375); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 187; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5381); };
            c.w32(9, t6);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d5388); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008d538c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d5390); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d539a); };
            c.w64(1, t7);
            c.w64(12, c.r64(1));
            blk = 6; continue;
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d53a3); };
            c.w64(12, t8);
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d53aa); };
            c.w64(1, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d53b1); };
            c.cmp(t10, c.r32(12), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        8 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(12));
            blk = 12; continue;
        }
        10 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x70008d53d5, 0x70008d53d0);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d53d5); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d53d9); };
            c.w64(11, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d53dd); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d53e1); };
            c.w64(6, t14);
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d53e5); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d53ec); };
            c.w64(7, t16);
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_l() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 34; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(12, c.r64(9));
            blk = 34; continue;
        }
        16 => {
            let r17 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r17);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 34; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        18 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let r18 = c.r64(10) | 0x1u64;
            c.w64(10, r18);
            c.logic(r18, 64);
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 22; continue;
        }
        22 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d544c); }
            c.xorpd_rr(2, 2);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.mov128_rr(1, 0);
            blk = 25; continue;
        }
        25 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(1));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(10, 0x8000000000000000u64);
            let r19 = c.r64(12) | c.r64(10);
            c.w64(12, r19);
            c.logic(r19, 64);
            blk = 28; continue;
        }
        28 => {
            blk = 34; continue;
        }
        29 => {
            c.arith_sd(0, 1, c.xmm_u64(0));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(2));
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        31 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t20) = c.ld64(0x70008d5470u64) else { return RegfileExit::fault(0x70008d54b4); };
            c.w64(10, t20);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(1));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t21) = c.ld64(0x70008d548eu64) else { return RegfileExit::fault(0x70008d54cf); };
            c.w64(10, t21);
            let r22 = c.r64(12) | c.r64(10);
            c.w64(12, r22);
            c.logic(r22, 64);
            blk = 34; continue;
        }
        34 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008d54dc); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(1, c.r64(7));
            let Some(t23) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d54e9); };
            c.w64(1, t23);
            blk = 36; continue;
        }
        36 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d54ed); };
            c.cmp(t24, c.r32(1), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(1, c.r64(9));
            blk = 65; continue;
        }
        38 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(1));
            blk = 42; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008d5510, 0x70008d550b);
        }
        41 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d5510); };
            c.w64(9, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5514); };
            c.w64(11, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d5518); };
            c.w64(8, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d551c); };
            c.w64(12, t28);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_l() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_b() { blk = 65; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(1, c.r64(9));
            blk = 65; continue;
        }
        46 => {
            let r29 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r29);
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_b() { blk = 65; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(1, c.r64(15));
            blk = 65; continue;
        }
        48 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d5558); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let r30 = c.r64(10) | 0x1u64;
            c.w64(10, r30);
            c.logic(r30, 64);
            blk = 51; continue;
        }
        51 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 52; continue;
        }
        52 => {
            c.xorpd_rr(2, 2);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 59; continue; }
            blk = 53; continue;
        }
        53 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.mov128_rr(0, 1);
            blk = 55; continue;
        }
        55 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 58; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t31) = c.ld64(0x70008d5470u64) else { return RegfileExit::fault(0x70008d55a1); };
            c.w64(10, t31);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_s() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t32) = c.ld64(0x70008d548eu64) else { return RegfileExit::fault(0x70008d55bc); };
            c.w64(10, t32);
            let r33 = c.r64(9) | c.r64(10);
            c.w64(9, r33);
            c.logic(r33, 64);
            blk = 58; continue;
        }
        58 => {
            c.w64(1, c.r64(9));
            blk = 65; continue;
        }
        59 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(1, c.r64(15));
            blk = 65; continue;
        }
        61 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t34) = c.ld64(0x70008d5470u64) else { return RegfileExit::fault(0x70008d55e4); };
            c.w64(10, t34);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_s() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t35) = c.ld64(0x70008d548eu64) else { return RegfileExit::fault(0x70008d55ff); };
            c.w64(10, t35);
            let r36 = c.r64(9) | c.r64(10);
            c.w64(9, r36);
            c.logic(r36, 64);
            blk = 64; continue;
        }
        64 => {
            c.w64(1, c.r64(9));
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_be() { blk = 184; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(0, c.r64(8));
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d5618); };
            c.w32(7, t37);
            let r38 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r38);
            c.w64(8, c.r64(1));
            let r39 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r39);
            let Some(t40) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5624); };
            c.w32(9, t40);
            let Some(t41) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5628); };
            c.w32(7, t41);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_be() { blk = 180; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_e() { blk = 179; continue; }
            blk = 68; continue;
        }
        68 => {
            c.logic(c.r8(7) & 0x7u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_e() { blk = 154; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(3, c.r64(7));
            c.w64(7, c.r64(15));
            c.w64(9, c.r64(0));
            blk = 71; continue;
        }
        71 => {
            c.w64(2, c.r64(3));
            let r42 = c.r32(2) & 0x7u64;
            c.w32(2, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 76; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 74; continue;
        }
        74 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 75; continue;
        }
        75 => {
            blk = 182; continue;
        }
        76 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            blk = 182; continue;
        }
        79 => {
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d5699); };
            c.w32(3, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d569d); };
            c.cmp(t44, c.r32(3), 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d56aa); };
            c.w32(9, t45);
            let r46 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r46);
            let Some(t47) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d56b1); };
            c.w32(3, t47);
            let r48 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r48);
            let Some(t49) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d56b8); };
            c.w32(3, t49);
            blk = 71; continue;
        }
        81 => {
            let Some(t50) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d56be); };
            c.w32(3, t50);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r51 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r51);
            let Some(t52) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d56cb); };
            c.w32(9, t52);
            let r53 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r53);
            let Some(t54) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d56d2); };
            c.w32(3, t54);
            let r55 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r55);
            let Some(t56) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d56d9); };
            c.w32(3, t56);
            blk = 71; continue;
        }
        82 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d56df); };
            c.w32(9, t57);
            let r58 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r58);
            let Some(t59) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d56e6); };
            c.w32(3, t59);
            let r60 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r60);
            let Some(t61) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d56ed); };
            c.w32(3, t61);
            blk = 71; continue;
        }
        83 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 86; continue;
        }
        84 => {
            c.w32(2, 0x1u64);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 86; continue;
        }
        85 => {
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(15));
            c.w64(3, c.r64(0));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r64(8), 0xdu64, 64);
            if c.cc_l() { blk = 94; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            let r62 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r62);
            let r63 = c.r32(9) & 0x8u64;
            c.w32(9, r63);
            c.logic(r63, 32);
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d573c); };
            c.w64(9, t64);
            let Some(t65) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d5743); };
            c.w64(11, t65);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d574a); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d5751); };
            c.cmp(t67, c.r64(12), 64);
            if c.cc_be() { blk = 194; continue; }
            blk = 89; continue;
        }
        89 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008d575a); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 90; continue;
        }
        90 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d5761); };
            c.w64(11, t68);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d5768); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d576c); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d5774); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d5778); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d577c); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5786); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5787); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d578d); }
            return c.ret(0x0, 0x70008d578f);
        }
        91 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5790); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d5797); };
            c.w64(11, t70);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d579e); };
            c.w64(12, t71);
            let Some(t72) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d57a5); };
            c.cmp(t72, c.r64(15), 64);
            if c.cc_be() { blk = 196; continue; }
            blk = 92; continue;
        }
        92 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008d57af); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 93; continue;
        }
        93 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d57b6); };
            c.w64(11, t73);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d57bd); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d57c1); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d57c9); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d57cd); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d57d1); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d57db); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d57dc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d57e2); }
            return c.ret(0x0, 0x70008d57e4);
        }
        94 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 124; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(1, c.r32(8));
            let r74 = c.r32(9) & 0x8u64;
            c.w32(9, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w32(1, c.r32(8));
            c.w64(9, c.r64(1));
            let r75 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r75);
            if c.cc_o() { blk = 104; continue; }
            blk = 98; continue;
        }
        98 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r76 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r76);
            if c.cc_o() { blk = 103; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d582a); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d5831); };
            c.w64(9, t78);
            let r79 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r79);
            let r80 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r80);
            c.logic(r80, 64);
            c.w64(15, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d5840); };
            c.w64(0, t81);
            let Some(t82) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d5847); };
            c.cmp(t82, c.r64(15), 64);
            if c.cc_be() { blk = 198; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 198; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d585d); };
            c.w64(1, t83);
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008d5864); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 102; continue;
        }
        102 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d586b); };
            c.w64(9, t84);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d5872); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d5876); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d587d); }
            c.w64(0, c.r64(1));
            blk = 106; continue;
        }
        103 => {
            return RegfileExit::fault(0x70008d5886);
        }
        104 => {
            return RegfileExit::fault(0x70008d5887);
        }
        105 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d5888); };
            c.w64(0, t85);
            blk = 106; continue;
        }
        106 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 110; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w32(8, 0xbu64);
            blk = 109; continue;
        }
        108 => {
            let r86 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r86);
            c.w64(1, c.r64(9));
            blk = 109; continue;
        }
        109 => {
            c.w64(9, c.r64(1).wrapping_add(0x2));
            let Some(t87) = c.ld16(c.r64(3).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008d58b7); };
            c.w32(1, t87);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70008d58bb); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 108; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d58cc); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d58cd); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d58d3); }
            return c.ret(0x0, 0x70008d58d5);
        }
        111 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r88 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r88);
            if c.cc_o() { blk = 117; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d58e9); };
            c.w64(9, t89);
            let Some(t90) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d58f0); };
            c.w64(9, t90);
            let r91 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r91);
            let r92 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r92);
            c.logic(r92, 64);
            c.w64(15, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d58ff); };
            c.w64(0, t93);
            let Some(t94) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d5906); };
            c.cmp(t94, c.r64(15), 64);
            if c.cc_be() { blk = 200; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 200; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d591c); };
            c.w64(1, t95);
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008d5923); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 116; continue;
        }
        116 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d592a); };
            c.w64(9, t96);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d5931); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d5935); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d593c); }
            c.w64(0, c.r64(1));
            blk = 119; continue;
        }
        117 => {
            return RegfileExit::fault(0x70008d5945);
        }
        118 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d5946); };
            c.w64(0, t97);
            blk = 119; continue;
        }
        119 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 123; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w32(8, 0xbu64);
            blk = 122; continue;
        }
        121 => {
            let r98 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r98);
            c.w64(1, c.r64(9));
            blk = 122; continue;
        }
        122 => {
            c.w64(9, c.r64(1).wrapping_add(0x1));
            let Some(t99) = c.ld8(c.r64(3).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008d596e); };
            c.w32(1, t99);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70008d5972); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 121; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5982); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5983); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5989); }
            return c.ret(0x0, 0x70008d598b);
        }
        124 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 182; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w32(1, c.r32(8));
            let Some(t100) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d5999); };
            c.w64(15, t100);
            let r101 = c.r32(9) & 0x8u64;
            c.w32(9, r101);
            c.logic(r101, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 135; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w32(1, c.r32(8));
            c.w64(9, c.r64(1));
            let r102 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r102);
            if c.cc_o() { blk = 134; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r103 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r103);
            if c.cc_o() { blk = 133; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d59d6); };
            c.w64(9, t104);
            let Some(t105) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d59dd); };
            c.w64(9, t105);
            let r106 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r106);
            let r107 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r107);
            c.logic(r107, 64);
            c.w64(0, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d59ec); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d59f3); };
            c.cmp(t109, c.r64(0), 64);
            if c.cc_be() { blk = 202; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 202; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5a09); };
            c.w64(1, t110);
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008d5a10); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 132; continue;
        }
        132 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d5a17); };
            c.w64(9, t111);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d5a1e); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d5a22); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d5a29); }
            c.w64(0, c.r64(1));
            blk = 136; continue;
        }
        133 => {
            return RegfileExit::fault(0x70008d5a32);
        }
        134 => {
            return RegfileExit::fault(0x70008d5a33);
        }
        135 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d5a34); };
            c.w64(0, t112);
            blk = 136; continue;
        }
        136 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 140; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w32(8, 0xbu64);
            blk = 139; continue;
        }
        138 => {
            let r113 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r113);
            c.w64(1, c.r64(9));
            blk = 139; continue;
        }
        139 => {
            c.w64(9, c.r64(1).wrapping_add(0x2));
            let Some(t114) = c.ld16(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d5a63); };
            c.w32(1, t114);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70008d5a69); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 138; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5a7a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5a7b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5a81); }
            return c.ret(0x0, 0x70008d5a83);
        }
        141 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 148; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r115 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r115);
            if c.cc_o() { blk = 147; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5a97); };
            c.w64(9, t116);
            let Some(t117) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d5a9e); };
            c.w64(9, t117);
            let r118 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r118);
            let r119 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r119);
            c.logic(r119, 64);
            c.w64(0, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d5aad); };
            c.w64(3, t120);
            let Some(t121) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d5ab4); };
            c.cmp(t121, c.r64(0), 64);
            if c.cc_be() { blk = 204; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 204; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5aca); };
            c.w64(1, t122);
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008d5ad1); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 146; continue;
        }
        146 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d5ad8); };
            c.w64(9, t123);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d5adf); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d5ae3); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d5aea); }
            c.w64(0, c.r64(1));
            blk = 149; continue;
        }
        147 => {
            return RegfileExit::fault(0x70008d5af3);
        }
        148 => {
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d5af4); };
            c.w64(0, t124);
            blk = 149; continue;
        }
        149 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 153; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w32(8, 0xbu64);
            blk = 152; continue;
        }
        151 => {
            let r125 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r125);
            c.w64(1, c.r64(9));
            blk = 152; continue;
        }
        152 => {
            c.w64(9, c.r64(1).wrapping_add(0x1));
            let Some(t126) = c.ld8(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d5b1c); };
            c.w32(1, t126);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70008d5b22); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 151; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5b32); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5b33); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5b39); }
            return c.ret(0x0, 0x70008d5b3b);
        }
        154 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(15));
            c.w64(7, c.r64(0));
            blk = 156; continue;
        }
        156 => {
            c.w64(9, c.r64(8));
            let r127 = c.r32(9) & 0x7u64;
            c.w32(9, r127);
            c.logic(r127, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 161; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 166; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 160; continue;
        }
        160 => {
            blk = 208; continue;
        }
        161 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            blk = 208; continue;
        }
        164 => {
            let Some(t128) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d5b8b); };
            c.w32(8, t128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d5b8f); };
            c.cmp(t129, c.r32(8), 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d5b9c); };
            c.w32(7, t130);
            let r131 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r131);
            let Some(t132) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d5ba2); };
            c.w32(8, t132);
            let r133 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r133);
            let Some(t134) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5ba9); };
            c.w32(8, t134);
            blk = 156; continue;
        }
        166 => {
            let Some(t135) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d5bb0); };
            c.w32(8, t135);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r136 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r136);
            let Some(t137) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d5bbd); };
            c.w32(7, t137);
            let r138 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r138);
            let Some(t139) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d5bc3); };
            c.w32(8, t139);
            let r140 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r140);
            let Some(t141) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5bca); };
            c.w32(8, t141);
            blk = 156; continue;
        }
        167 => {
            let Some(t142) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d5bd1); };
            c.w32(7, t142);
            let r143 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r143);
            let Some(t144) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d5bd7); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            let Some(t146) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5bde); };
            c.w32(8, t146);
            blk = 156; continue;
        }
        168 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 171; continue;
        }
        169 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0x1u64);
            blk = 171; continue;
        }
        170 => {
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 172; continue;
        }
        172 => {
            let r147 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r147);
            blk = 173; continue;
        }
        173 => {
            let r148 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r148);
            let r149 = c.r32(1) & 0x8u64;
            c.w32(1, r149);
            c.logic(r149, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t150) = c.ld16(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d5c3b); };
            c.w32(1, t150);
            blk = 176; continue;
        }
        175 => {
            let Some(t151) = c.ld8(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008d5c42); };
            c.w32(1, t151);
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 221; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d5c5b); };
            c.w64(8, t152);
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d5c62); };
            c.w32(0, t153);
            let r154 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r154);
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d5c6a); };
            c.cmp(t155, c.r32(0), 32);
            if c.cc_e() { blk = 226; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5c7e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5c7f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5c85); }
            return c.ret(0x0, 0x70008d5c87);
        }
        179 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d5c8c); };
            c.w64(0, t156);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5c96); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5c97); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5c9d); }
            return c.ret(0x0, 0x70008d5c9f);
        }
        180 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5cad); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5cae); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5cb4); }
            return c.ret(0x0, 0x70008d5cb6);
        }
        182 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let r157 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r157);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d5cbd); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d5cbe); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d5cbf); }
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008d5cc0); };
            c.w64(3, t158);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x70008d5cd4, 0x70008d5ccf);
        }
        183 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5cd4); };
            c.w64(1, t159);
            let r160 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r160);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5cdf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5ce0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5ce6); }
            return c.ret(0x0, 0x70008d5ce8);
        }
        184 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d5ced); };
            c.w64(0, t161);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5cf7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5cf8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d5cfe); }
            return c.ret(0x0, 0x70008d5d00);
        }
        185 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d5d01); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d5d07); };
            c.w64(3, t162);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008d5d16, 0x70008d5d11);
        }
        186 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5d16); };
            c.w64(6, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5d1a); };
            c.w64(0, t164);
            blk = 1; continue;
        }
        187 => {
            let Some(t165) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d5d23); };
            c.cmp(t165, c.r32(8), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d5d32); };
            c.cmp(t166, c.r32(8), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d5d41); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d5d45); }
            let r167 = c.r32(9) | c.r32(11);
            c.w32(9, r167);
            c.logic(r167, 32);
            if c.cc_ne() { blk = 190; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d5d56, 0x70008d5d51);
        }
        189 => {
            c.w64(8, c.r64(0));
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5d59); };
            c.w64(6, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5d5d); };
            c.w64(7, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5d61); };
            c.w64(1, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5d65); };
            c.w64(0, t171);
            blk = 3; continue;
        }
        190 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d5d73); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d5d74); };
            c.w64(1, t172);
            let Some(t173) = c.ld32(c.r64(1).wrapping_add(0x383)) else { return RegfileExit::fault(0x70008d5d7b); };
            c.w32(1, t173);
            let r174 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r174);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d5d84); }
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d5d85); };
            c.w64(3, t175);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d5d96, 0x70008d5d91);
        }
        191 => {
            return RegfileExit::fault(0x70008d5d96);
        }
        192 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d5d97); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d5d9b); }
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d5da7, 0x70008d5da2);
        }
        193 => {
            c.w64(8, c.r64(0));
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5daa); };
            c.w64(6, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5dae); };
            c.w64(7, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5db2); };
            c.w64(1, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5db6); };
            c.w64(0, t179);
            blk = 3; continue;
        }
        194 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d5dc4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d5dc8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008d5dcc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008d5dd0); }
            return c.call(0x70007febf4, 0x70008d5dd9, 0x70008d5dd4);
        }
        195 => {
            c.w64(9, c.r64(0));
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5ddc); };
            c.w64(8, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d5de0); };
            c.w64(3, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5de4); };
            c.w64(7, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d5de8); };
            c.w64(1, t183);
            blk = 90; continue;
        }
        196 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d5df1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d5df5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008d5df9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008d5dfd); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d5e0b, 0x70008d5e06);
        }
        197 => {
            c.w64(9, c.r64(0));
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5e0e); };
            c.w64(8, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d5e12); };
            c.w64(3, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5e16); };
            c.w64(7, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d5e1a); };
            c.w64(1, t187);
            blk = 93; continue;
        }
        198 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d5e23); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d5e27); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d5e2b); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d5e37, 0x70008d5e32);
        }
        199 => {
            c.w64(1, c.r64(0));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5e3a); };
            c.w64(11, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5e3e); };
            c.w64(12, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5e42); };
            c.w64(8, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5e46); };
            c.w64(7, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d5e4a); };
            c.w64(3, t192);
            blk = 102; continue;
        }
        200 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d5e53); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d5e57); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d5e5b); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d5e67, 0x70008d5e62);
        }
        201 => {
            c.w64(1, c.r64(0));
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5e6a); };
            c.w64(11, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5e6e); };
            c.w64(12, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5e72); };
            c.w64(8, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5e76); };
            c.w64(7, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d5e7a); };
            c.w64(3, t197);
            blk = 116; continue;
        }
        202 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008d5e83); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d5e87); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d5e8b); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d5e97, 0x70008d5e92);
        }
        203 => {
            c.w64(1, c.r64(0));
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5e9a); };
            c.w64(11, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5e9e); };
            c.w64(12, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d5ea2); };
            c.w64(15, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5ea6); };
            c.w64(8, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5eaa); };
            c.w64(7, t202);
            blk = 132; continue;
        }
        204 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008d5eb3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d5eb7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d5ebb); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d5ec7, 0x70008d5ec2);
        }
        205 => {
            c.w64(1, c.r64(0));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5eca); };
            c.w64(11, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5ece); };
            c.w64(12, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d5ed2); };
            c.w64(15, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d5ed6); };
            c.w64(8, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d5eda); };
            c.w64(7, t207);
            blk = 146; continue;
        }
        206 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t208) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d5ee8); };
            c.w64(8, t208);
            blk = 173; continue;
        }
        208 => {
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w32(1, c.r64(12).wrapping_add(c.r64(12)));
            blk = 219; continue;
        }
        210 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5f00); };
            c.w64(1, t209);
            let Some(t210) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d5f07); };
            c.w64(7, t210);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d5f0e); };
            c.w64(9, t211);
            let Some(t212) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d5f15); };
            c.cmp(t212, c.r64(8), 64);
            if c.cc_a() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008d5f24, 0x70008d5f1f);
        }
        212 => {
            c.w64(1, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5f27); };
            c.w64(11, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d5f2b); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5f2f); };
            c.w64(12, t215);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 214; continue;
        }
        213 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008d5f38); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 214; continue;
        }
        214 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 218; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            let r216 = c.r64(10) | 0x1u64;
            c.w64(10, r216);
            c.logic(r216, 64);
            blk = 217; continue;
        }
        217 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 218; continue;
        }
        218 => {
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d5f66); };
            c.w64(7, t217);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008d5f6d); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d5f70); }
            blk = 219; continue;
        }
        219 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d5f75); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d5f76); }
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d5f77); };
            c.w64(3, t218);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x70008d5f8b, 0x70008d5f86);
        }
        220 => {
            let r219 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r219);
            c.w64(1, c.r64(0));
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5f90); };
            c.w64(11, t220);
            blk = 176; continue;
        }
        221 => {
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d5f99); };
            c.w64(7, t221);
            let Some(t222) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d5fa0); };
            c.w64(8, t222);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let r223 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r223);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d5fab); }
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d5faf); };
            c.w64(12, t224);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d5fb6); }
            let Some(t225) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d5fba); };
            c.cmp(t225, c.r64(9), 64);
            if c.cc_a() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d5fca, 0x70008d5fc5);
        }
        223 => {
            blk = 225; continue;
        }
        224 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008d5fcc); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 225; continue;
        }
        225 => {
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d5fd6); };
            c.w64(1, t226);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d5fdd); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d5fe0); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d5fe7); }
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d5fee); };
            c.w64(1, t227);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70008d5ff2); }
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d5ff6); };
            c.w64(1, t228);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d5ffd); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d5ffe); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d6004); }
            return c.ret(0x0, 0x70008d6006);
        }
        226 => {
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d6007); };
            c.w64(9, t229);
            let Some(t230) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d600e); };
            c.w64(12, t230);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008d6016); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d601a); }
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d601e); };
            c.w64(0, t231);
            let Some(t232) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d6025); };
            c.cmp(t232, c.r64(15), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d6034, 0x70008d602f);
        }
        228 => {
            blk = 230; continue;
        }
        229 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008d6036); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 230; continue;
        }
        230 => {
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d6041); };
            c.w64(1, t233);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d6048); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d604b); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d6052); }
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6059); };
            c.w64(1, t234);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008d605d); }
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d6060); };
            c.w64(8, t235);
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d6067); };
            c.w32(7, t236);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            return RegfileExit::fault(0x70008d6076);
        }
        232 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6077); };
            c.w64(7, t237);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008d607b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r238 = c.r64(1) & c.r64(8);
            c.w64(1, r238);
            c.logic(r238, 64);
            let Some(t239) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d608d); };
            c.logic(t239 & 0x4u64, 8);
            if c.cc_ne() { blk = 235; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6093); };
            c.w64(7, t240);
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d609e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d609f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d60a5); }
            return c.ret(0x0, 0x70008d60a7);
        }
        235 => {
            c.w32(0, c.r32(0));
            let r241 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r241);
            c.w64(1, 0xfffffffffffc0000u64);
            let r242 = c.r64(1) & c.r64(0);
            c.w64(1, r242);
            c.logic(r242, 64);
            let Some(t243) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d60b7); };
            c.logic(t243 & 0x2u64, 8);
            if c.cc_e() { blk = 234; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d60c1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d60c2); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008d60cf, 0x70008d60ca);
        }
        237 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008d60cf); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008d60d1); }
            blk = 234; continue;
        }
        238 => {
            return RegfileExit::jump(0x70008d60d8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d60f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d60f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d60f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d60f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d60fa); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x70008d60ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d6103); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 235; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            c.w64(7, c.r64(2).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d611b); };
            c.w64(8, t2);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008d6123); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d612d); };
            c.w64(2, t3);
            c.w64(11, c.r64(2));
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d6136); };
            c.w64(11, t4);
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d613d); };
            c.w64(2, t5);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x70008d6144); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d614e); };
            c.w64(2, t6);
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008d6152); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 242; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6160); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6167); };
            c.cmp(t9, 0x40u64, 16);
            if c.cc_ae() { blk = 237; continue; }
            blk = 8; continue;
        }
        8 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008d617e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d6182); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d618f); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6196); };
            c.w32(12, t12);
            let r13 = c.sub(c.r32(12), 0xbau64, 32);
            c.w32(12, r13);
            c.cmp(c.r32(12), 0x78eu64, 32);
            if c.cc_a() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xcd0)) else { return RegfileExit::fault(0x70008d61b0); };
            c.w64(3, t14);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x70008d61bf, 0x70008d61ba);
        }
        11 => {
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d61bf); };
            c.cmp(t15, c.r32(0), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008d61cc); };
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
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d61df); };
            c.cmp(t17, c.r32(0), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d61ec); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            let Some(t20) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008d61f2); };
            c.logic(t20 & 0x10u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d61f8); };
            c.cmp(t21, c.r32(2), 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t22) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6205); };
            c.cmp(t22, 0x41u64, 16);
            if c.cc_e() { blk = 244; continue; }
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
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6217); };
            c.w64(2, t23);
            let Some(t24) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d621b); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6221); };
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
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d6245); }
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008d6246); };
            c.w64(3, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d624d); };
            c.w64(6, t28);
            return c.call(0x700086ed74, 0x70008d6256, 0x70008d6251);
        }
        25 => {
            blk = 33; continue;
        }
        26 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6258); };
            c.w64(2, t29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d625c); };
            c.w32(7, t30);
            let r31 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r31);
            let Some(t32) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6262); };
            c.cmp(t32, 0x83eu64, 16);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w32(7, 0x92u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d626f); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008d6270); };
            c.w64(3, t33);
            c.w32(0, 0x1u64);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d627c); };
            c.w64(6, t34);
            return c.call(0x700086ed74, 0x70008d6285, 0x70008d6280);
        }
        28 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6285); };
            c.w64(0, t35);
            blk = 36; continue;
        }
        29 => {
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d628b); };
            c.w64(0, t36);
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d628f); };
            c.w32(2, t37);
            let r38 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r38);
            let Some(t39) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6295); };
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
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d62b0); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d62b1); };
            c.w64(2, t40);
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x387)) else { return RegfileExit::fault(0x70008d62b8); };
            c.w32(2, t41);
            let r42 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r42);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d62c1); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d62c2); };
            c.w64(3, t43);
            c.w32(0, 0x2u64);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d62ce); };
            c.w64(6, t44);
            return c.call(0x700086ed74, 0x70008d62d7, 0x70008d62d2);
        }
        34 => {
            return RegfileExit::fault(0x70008d62d7);
        }
        35 => {
            c.w64(0, c.r64(11));
            blk = 36; continue;
        }
        36 => {
            let Some(t45) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d62db); };
            c.cmp(t45, 0x0u64, 32);
            if c.cc_ne() { blk = 248; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d62e5); };
            c.w32(2, t46);
            let r47 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r47);
            let Some(t48) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d62eb); };
            c.cmp(t48, 0x40u64, 16);
            if c.cc_ae() { blk = 248; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d62f6); };
            c.w64(8, t49);
            let Some(t50) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d62fa); };
            c.w32(2, t50);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x70008d62fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008d6302); }
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6306); };
            c.w64(7, t51);
            let Some(t52) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d630a); };
            c.cmp(t52, c.r32(7), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 60; continue;
        }
        40 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(0, c.r64(7));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6323); };
            c.w64(6, t53);
            return c.call(0x7000899a14, 0x70008d632c, 0x70008d6327);
        }
        42 => {
            c.w64(7, c.r64(0));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d632f); };
            c.w64(0, t54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d6333); };
            c.w64(8, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d6337); };
            c.w64(2, t56);
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_a() { blk = 60; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w64(7, c.r64(2));
            blk = 60; continue;
        }
        47 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 60; continue;
        }
        48 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d6364); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 59; continue; }
            blk = 49; continue;
        }
        49 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(2) as i64));
            c.logic(c.r64(2) & c.r64(2), 64);
            if c.cc_ns() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(10, c.r64(2));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let r57 = c.r64(10) | 0x1u64;
            c.w64(10, r57);
            c.logic(r57, 64);
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
            if c.cc_ae() { blk = 58; continue; }
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
            let r58 = c.r64(7) | c.r64(10);
            c.w64(7, r58);
            c.logic(r58, 64);
            blk = 57; continue;
        }
        57 => {
            blk = 60; continue;
        }
        58 => {
            c.w64(7, c.r64(2));
            blk = 60; continue;
        }
        59 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 60; continue;
        }
        60 => {
            c.w64(11, c.r64(0));
            let Some(t59) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d63e1); };
            c.w32(12, t59);
            let r60 = c.sub(c.r64(2), c.r64(7), 64);
            c.w64(2, r60);
            c.w32(9, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d63eb); }
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_b() { blk = 234; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t61) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d63f8); };
            c.w32(2, t61);
            c.w64(12, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d63ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008d6403); }
            c.w64(15, c.r64(9));
            blk = 63; continue;
        }
        62 => {
            c.w64(8, c.r64(0));
            blk = 63; continue;
        }
        63 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d640f); };
            c.w32(0, t62);
            let r63 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r63);
            let Some(t64) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6416); };
            c.w32(0, t64);
            c.w64(3, c.r64(0));
            let r65 = c.r32(3) & 0xfu64;
            c.w32(3, r65);
            c.logic(r65, 32);
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(1, c.r64(0));
            let r66 = c.r32(1) & 0x7u64;
            c.w32(1, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d643d); };
            c.w32(8, t67);
            let r68 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r68);
            c.w64(0, c.r64(8));
            blk = 62; continue;
        }
        67 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d644e); };
            c.w32(0, t69);
            let r70 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r70);
            let Some(t71) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6455); };
            c.cmp(t71, 0x0u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008d645b); }
            c.w64(0, c.r64(8));
            return c.call(0x70008e88b4, 0x70008d6467, 0x70008d6462);
        }
        70 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6467); };
            c.w64(15, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d646b); };
            c.w64(11, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d646f); };
            c.w64(7, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d6473); };
            c.w64(2, t75);
            blk = 62; continue;
        }
        71 => {
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d6479); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            c.w64(0, c.r64(8));
            blk = 62; continue;
        }
        72 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 82; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d648a); };
            c.w32(9, t78);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r79 = c.add(c.r64(15), c.r64(9), 64);
            c.w64(15, r79);
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d6497); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            c.w64(0, c.r64(8));
            blk = 62; continue;
        }
        74 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d64a6); };
            c.w32(9, t82);
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_b() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d64b2); };
            let r84 = c.sub(c.r64(9), t83, 64);
            c.w64(9, r84);
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(9, c.r64(15).wrapping_add(0xc));
            blk = 90; continue;
        }
        77 => {
            return RegfileExit::fault(0x70008d64c4);
        }
        78 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d64c5); };
            c.w32(9, t85);
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_b() { blk = 81; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d64d1); };
            let r87 = c.sub(c.r64(9), t86, 64);
            c.w64(9, r87);
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(9, c.r64(15).wrapping_mul(2).wrapping_add(0xc));
            blk = 165; continue;
        }
        81 => {
            return RegfileExit::fault(0x70008d64e7);
        }
        82 => {
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 83; continue;
        }
        83 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t88) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d64f5); };
            c.w64(9, t88);
            c.w64(0, c.r64(9));
            blk = 87; continue;
        }
        85 => {
            c.w64(10, c.r64(4));
            let r89 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r89);
            let r90 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r90);
            c.logic(r90, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d6509); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008d650e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008d6512); }
            c.w64(1, c.r64(8));
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d6519); };
            c.w64(0, t91);
            c.w64(10, 0x70008d6527u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d6527); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d652b); }
            return c.call(c.r64(0), 0x70008d6531, 0x70008d652f);
        }
        86 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d6531); }
            let Some(t92) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d6539); };
            c.w64(4, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d653e); };
            c.w64(15, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6542); };
            c.w64(11, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d6546); };
            c.w64(7, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d654a); };
            c.w64(2, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d654e); };
            c.w64(8, t97);
            blk = 87; continue;
        }
        87 => {
            c.w64(1, c.r64(8));
            let Some(t98) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6555); };
            c.w32(8, t98);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_b() { blk = 156; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6565); };
            let r100 = c.sub(c.r64(8), t99, 64);
            c.w64(8, r100);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 156; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(9, c.r64(15).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 90; continue;
        }
        90 => {
            let Some(t101) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d657a); };
            c.w32(12, t101);
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            let r102 = c.add(c.r64(9), c.r64(7), 64);
            c.w64(9, r102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6584); };
            c.w64(0, t103);
            let r104 = c.sub(c.r64(0), c.r64(15), 64);
            c.w64(0, r104);
            c.cmp(c.r32(2), c.r32(12), 32);
            c.w8(2, (c.cc_b() as u64));
            c.w32(2, c.r8(2));
            c.w64(3, c.r64(9).wrapping_add(c.r64(15)));
            c.cmp(c.r64(0), c.r64(7), 64);
            c.w8(7, (c.cc_b() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008d65a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008d65a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d65ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008d65af); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d65b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008d65b7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d65bb); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 92; continue;
        }
        91 => {
            c.w64(11, c.r64(2));
            blk = 92; continue;
        }
        92 => {
            let Some(t105) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d65c6); };
            c.w32(1, t105);
            let r106 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r106);
            let Some(t107) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d65cd); };
            c.w32(1, t107);
            c.w64(6, c.r64(1));
            let r108 = c.r32(6) & 0xfu64;
            c.w32(6, r108);
            c.logic(r108, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008d65d7); }
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
            let Some(t110) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d65f5); };
            c.w32(11, t110);
            let r111 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r111);
            c.w64(2, c.r64(11));
            blk = 91; continue;
        }
        96 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t112) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d6606); };
            c.w32(1, t112);
            let r113 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r113);
            let Some(t114) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d660d); };
            c.cmp(t114, 0x0u64, 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 98; continue;
        }
        98 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d6613); }
            c.w64(0, c.r64(11));
            return c.call(0x70008e88b4, 0x70008d661f, 0x70008d661a);
        }
        99 => {
            c.w64(2, c.r64(0));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d6622); };
            c.w64(8, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6626); };
            c.w64(3, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d662a); };
            c.w64(15, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d662e); };
            c.w64(0, t118);
            blk = 91; continue;
        }
        100 => {
            let Some(t119) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d6634); };
            c.w32(2, t119);
            let r120 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r120);
            blk = 91; continue;
        }
        101 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d6642); };
            c.w32(2, t121);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r122 = c.add(c.r64(0), c.r64(2), 64);
            c.w64(0, r122);
            let Some(t123) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d664f); };
            c.w32(2, t123);
            let r124 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r124);
            blk = 91; continue;
        }
        103 => {
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d665b); };
            c.w32(2, t125);
            let Some(t126) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d665f); };
            c.cmp(c.r32(2), t126, 32);
            if c.cc_b() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r127 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r127);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w64(2, c.r64(0).wrapping_add(0xc));
            blk = 119; continue;
        }
        106 => {
            return RegfileExit::fault(0x70008d6678);
        }
        107 => {
            let Some(t128) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6679); };
            c.w32(2, t128);
            let Some(t129) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d667d); };
            c.cmp(c.r32(2), t129, 32);
            if c.cc_b() { blk = 110; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r130 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r130);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(2, c.r64(0).wrapping_mul(2).wrapping_add(0xc));
            blk = 141; continue;
        }
        110 => {
            return RegfileExit::fault(0x70008d669a);
        }
        111 => {
            c.cmp(c.r32(6), 0xau64, 32);
            if c.cc_ne() { blk = 133; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t131) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d66a4); };
            c.logic(t131 & 0x10u64, 8);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t132) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d66aa); };
            c.w64(2, t132);
            blk = 116; continue;
        }
        114 => {
            c.w64(10, c.r64(4));
            let r133 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r133);
            let r134 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r134);
            c.logic(r134, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d66bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x70008d66c0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d66c4); }
            c.w64(1, c.r64(11));
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d66cb); };
            c.w64(0, t135);
            c.w64(10, 0x70008d66d9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d66d9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d66dd); }
            return c.call(c.r64(0), 0x70008d66e3, 0x70008d66e1);
        }
        115 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d66e3); }
            let Some(t136) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d66eb); };
            c.w64(4, t136);
            c.w64(2, c.r64(0));
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d66f3); };
            c.w64(8, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d66f7); };
            c.w64(3, t138);
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d66fb); };
            c.w64(15, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d66ff); };
            c.w64(11, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d6703); };
            c.w64(0, t141);
            blk = 116; continue;
        }
        116 => {
            c.w64(1, c.r64(11));
            let Some(t142) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d670a); };
            c.w32(7, t142);
            let Some(t143) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d670d); };
            c.cmp(c.r32(7), t143, 32);
            if c.cc_b() { blk = 132; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r144 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r144);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 132; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x1));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 119; continue;
        }
        119 => {
            let Some(t145) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d6729); };
            c.cmp(t145, 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t146) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d672f); };
            c.cmp(t146, 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(7, c.r64(15));
            let r147 = c.add(c.r64(7), c.r64(2), 64);
            c.w64(7, r147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d673b); };
            c.w64(9, t148);
            blk = 123; continue;
        }
        122 => {
            let r149 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r149);
            let r150 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r150);
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 128; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t151) = c.ld8(c.r64(11).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d674e); };
            c.w32(12, t151);
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_e() { blk = 127; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t152) = c.ld8(c.r64(8).wrapping_add(c.r64(9)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6759); };
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
            return RegfileExit::fault(0x70008d6764);
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
            if c.cc_ne() { blk = 229; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 228; continue;
        }
        132 => {
            return RegfileExit::fault(0x70008d677c);
        }
        133 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_ne() { blk = 155; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t153) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d6786); };
            c.logic(t153 & 0x10u64, 8);
            if c.cc_ne() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t154) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d678c); };
            c.w64(2, t154);
            blk = 138; continue;
        }
        136 => {
            c.w64(10, c.r64(4));
            let r155 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r155);
            let r156 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r156);
            c.logic(r156, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d679d); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x70008d67a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d67a6); }
            c.w64(1, c.r64(11));
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d67ad); };
            c.w64(0, t157);
            c.w64(10, 0x70008d67bbu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d67bb); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d67bf); }
            return c.call(c.r64(0), 0x70008d67c5, 0x70008d67c3);
        }
        137 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d67c5); }
            let Some(t158) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d67cd); };
            c.w64(4, t158);
            c.w64(2, c.r64(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d67d5); };
            c.w64(8, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d67d9); };
            c.w64(3, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d67dd); };
            c.w64(15, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d67e1); };
            c.w64(11, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d67e5); };
            c.w64(0, t163);
            blk = 138; continue;
        }
        138 => {
            c.w64(7, c.r64(11));
            let Some(t164) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d67ec); };
            c.w32(7, t164);
            let Some(t165) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d67ef); };
            c.cmp(c.r32(7), t165, 32);
            if c.cc_b() { blk = 154; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r166 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r166);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 154; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(2, c.r64(2).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 141; continue;
        }
        141 => {
            let Some(t167) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d680b); };
            c.cmp(t167, 0x0u64, 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t168) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d6811); };
            c.cmp(t168, 0x0u64, 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(7, c.r64(15));
            c.w64(7, c.r64(2).wrapping_add(c.r64(7).wrapping_mul(2)));
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d681e); };
            c.w64(9, t169);
            c.w64(12, c.r64(8));
            c.w64(8, c.r64(3));
            blk = 145; continue;
        }
        144 => {
            let r170 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r170);
            let r171 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r171);
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 150; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t172) = c.ld16(c.r64(11).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6837); };
            c.w32(15, t172);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_e() { blk = 149; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t173) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6842); };
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
            return RegfileExit::fault(0x70008d6851);
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
            if c.cc_ne() { blk = 229; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 228; continue;
        }
        154 => {
            return RegfileExit::fault(0x70008d6869);
        }
        155 => {
            return RegfileExit::fault(0x70008d686a);
        }
        156 => {
            return RegfileExit::fault(0x70008d686b);
        }
        157 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_ne() { blk = 233; continue; }
            blk = 158; continue;
        }
        158 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t174) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d6879); };
            c.w64(9, t174);
            c.w64(0, c.r64(9));
            blk = 162; continue;
        }
        160 => {
            c.w64(10, c.r64(4));
            let r175 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r175);
            let r176 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r176);
            c.logic(r176, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d688d); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008d6892); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008d6896); }
            c.w64(1, c.r64(8));
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d689d); };
            c.w64(0, t177);
            c.w64(10, 0x70008d68abu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d68ab); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d68af); }
            return c.call(c.r64(0), 0x70008d68b5, 0x70008d68b3);
        }
        161 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d68b5); }
            let Some(t178) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d68bd); };
            c.w64(4, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d68c2); };
            c.w64(15, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d68c6); };
            c.w64(11, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d68ca); };
            c.w64(7, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d68ce); };
            c.w64(2, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d68d2); };
            c.w64(8, t183);
            blk = 162; continue;
        }
        162 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d68d6); };
            c.w32(8, t184);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_b() { blk = 232; continue; }
            blk = 163; continue;
        }
        163 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d68e6); };
            let r186 = c.sub(c.r64(8), t185, 64);
            c.w64(8, r186);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 232; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(9, c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 165; continue;
        }
        165 => {
            let Some(t187) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d68fb); };
            c.w32(12, t187);
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_add(c.r64(7).wrapping_mul(2)));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6906); };
            c.w64(0, t188);
            let r189 = c.sub(c.r64(0), c.r64(15), 64);
            c.w64(0, r189);
            c.cmp(c.r32(2), c.r32(12), 32);
            c.w8(2, (c.cc_b() as u64));
            c.w32(2, c.r8(2));
            c.w64(3, c.r64(9).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.cmp(c.r64(0), c.r64(7), 64);
            c.w8(7, (c.cc_b() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008d6925); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008d6929); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d692d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008d6931); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008d6935); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008d6939); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d693d); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 167; continue;
        }
        166 => {
            c.w64(11, c.r64(2));
            blk = 167; continue;
        }
        167 => {
            let Some(t190) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6948); };
            c.w32(1, t190);
            let r191 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r191);
            let Some(t192) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d694f); };
            c.w32(1, t192);
            c.w64(6, c.r64(1));
            let r193 = c.r32(6) & 0xfu64;
            c.w32(6, r193);
            c.logic(r193, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x70008d6959); }
            c.cmp(c.r32(6), 0x8u64, 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 168; continue;
        }
        168 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 169; continue;
        }
        169 => {
            let r194 = c.r32(1) & 0x7u64;
            c.w32(1, r194);
            c.logic(r194, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_ne() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t195) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d6977); };
            c.w32(11, t195);
            let r196 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r196);
            c.w64(2, c.r64(11));
            blk = 166; continue;
        }
        171 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t197) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d6988); };
            c.w32(1, t197);
            let r198 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r198);
            let Some(t199) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d698f); };
            c.cmp(t199, 0x0u64, 32);
            if c.cc_e() { blk = 175; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d6995); }
            c.w64(0, c.r64(11));
            return c.call(0x70008e88b4, 0x70008d69a1, 0x70008d699c);
        }
        174 => {
            c.w64(2, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d69a4); };
            c.w64(8, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d69a8); };
            c.w64(3, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d69ac); };
            c.w64(15, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d69b0); };
            c.w64(0, t203);
            blk = 166; continue;
        }
        175 => {
            let Some(t204) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d69b6); };
            c.w32(2, t204);
            let r205 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r205);
            blk = 166; continue;
        }
        176 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t206) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d69c4); };
            c.w32(2, t206);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            let r207 = c.add(c.r64(0), c.r64(2), 64);
            c.w64(0, r207);
            let Some(t208) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d69d1); };
            c.w32(2, t208);
            let r209 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r209);
            blk = 166; continue;
        }
        178 => {
            let Some(t210) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d69dd); };
            c.w32(2, t210);
            let Some(t211) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d69e1); };
            c.cmp(c.r32(2), t211, 32);
            if c.cc_b() { blk = 181; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r212 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r212);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(2, c.r64(0).wrapping_add(0xc));
            blk = 194; continue;
        }
        181 => {
            return RegfileExit::fault(0x70008d69fa);
        }
        182 => {
            let Some(t213) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d69fb); };
            c.w32(2, t213);
            let Some(t214) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d69ff); };
            c.cmp(c.r32(2), t214, 32);
            if c.cc_b() { blk = 185; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let r215 = c.sub(c.r64(2), c.r64(15), 64);
            c.w64(2, r215);
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_b() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(2, c.r64(0).wrapping_mul(2).wrapping_add(0xc));
            blk = 216; continue;
        }
        185 => {
            return RegfileExit::fault(0x70008d6a1c);
        }
        186 => {
            c.cmp(c.r32(6), 0xau64, 32);
            if c.cc_ne() { blk = 208; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t216) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d6a26); };
            c.logic(t216 & 0x10u64, 8);
            if c.cc_ne() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t217) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d6a2c); };
            c.w64(2, t217);
            blk = 191; continue;
        }
        189 => {
            c.w64(10, c.r64(4));
            let r218 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r218);
            let r219 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r219);
            c.logic(r219, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d6a3d); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x70008d6a42); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d6a46); }
            c.w64(1, c.r64(11));
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d6a4d); };
            c.w64(0, t220);
            c.w64(10, 0x70008d6a5bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d6a5b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d6a5f); }
            return c.call(c.r64(0), 0x70008d6a65, 0x70008d6a63);
        }
        190 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d6a65); }
            let Some(t221) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d6a6d); };
            c.w64(4, t221);
            c.w64(2, c.r64(0));
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d6a75); };
            c.w64(8, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6a79); };
            c.w64(3, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d6a7d); };
            c.w64(15, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6a81); };
            c.w64(11, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d6a85); };
            c.w64(0, t226);
            blk = 191; continue;
        }
        191 => {
            c.w64(1, c.r64(11));
            let Some(t227) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6a8c); };
            c.w32(7, t227);
            let Some(t228) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d6a8f); };
            c.cmp(c.r32(7), t228, 32);
            if c.cc_b() { blk = 207; continue; }
            blk = 192; continue;
        }
        192 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r229 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r229);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 207; continue; }
            blk = 193; continue;
        }
        193 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x1));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 194; continue;
        }
        194 => {
            let Some(t230) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d6aab); };
            c.cmp(t230, 0x0u64, 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t231) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d6ab1); };
            c.cmp(t231, 0x0u64, 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(7, c.r64(15));
            let r232 = c.add(c.r64(7), c.r64(2), 64);
            c.w64(7, r232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6abd); };
            c.w64(9, t233);
            blk = 198; continue;
        }
        197 => {
            let r234 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r234);
            let r235 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r235);
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 203; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t236) = c.ld8(c.r64(11).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6ad0); };
            c.w32(12, t236);
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_e() { blk = 202; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t237) = c.ld16(c.r64(8).wrapping_add(c.r64(9)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6adb); };
            c.w32(15, t237);
            c.cmp(c.r32(15), c.r32(12), 32);
            if c.cc_e() { blk = 197; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 205; continue;
        }
        202 => {
            return RegfileExit::fault(0x70008d6aea);
        }
        203 => {
            c.w32(2, 0x1u64);
            blk = 205; continue;
        }
        204 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 229; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 228; continue;
        }
        207 => {
            return RegfileExit::fault(0x70008d6b02);
        }
        208 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_ne() { blk = 231; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t238) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008d6b0c); };
            c.logic(t238 & 0x10u64, 8);
            if c.cc_ne() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t239) = c.ld64(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d6b12); };
            c.w64(2, t239);
            blk = 213; continue;
        }
        211 => {
            c.w64(10, c.r64(4));
            let r240 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r240);
            let r241 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r241);
            c.logic(r241, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d6b23); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x70008d6b28); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008d6b2c); }
            c.w64(1, c.r64(11));
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d6b33); };
            c.w64(0, t242);
            c.w64(10, 0x70008d6b41u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d6b41); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d6b45); }
            return c.call(c.r64(0), 0x70008d6b4b, 0x70008d6b49);
        }
        212 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d6b4b); }
            let Some(t243) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d6b53); };
            c.w64(4, t243);
            c.w64(2, c.r64(0));
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008d6b5b); };
            c.w64(8, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6b5f); };
            c.w64(3, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d6b63); };
            c.w64(15, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6b67); };
            c.w64(11, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008d6b6b); };
            c.w64(0, t248);
            blk = 213; continue;
        }
        213 => {
            c.w64(7, c.r64(11));
            let Some(t249) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6b72); };
            c.w32(7, t249);
            let Some(t250) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008d6b75); };
            c.cmp(c.r32(7), t250, 32);
            if c.cc_b() { blk = 230; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let r251 = c.sub(c.r64(7), c.r64(15), 64);
            c.w64(7, r251);
            c.cmp(c.r64(7), c.r64(0), 64);
            if c.cc_b() { blk = 230; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w64(2, c.r64(2).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x1));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 216; continue;
        }
        216 => {
            let Some(t252) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d6b95); };
            c.cmp(t252, 0x0u64, 32);
            if c.cc_ne() { blk = 226; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t253) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d6b9b); };
            c.cmp(t253, 0x0u64, 32);
            if c.cc_ne() { blk = 226; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(7, c.r64(15));
            c.w64(7, c.r64(2).wrapping_add(c.r64(7).wrapping_mul(2)));
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6ba8); };
            c.w64(9, t254);
            c.w64(12, c.r64(8));
            c.w64(8, c.r64(3));
            blk = 220; continue;
        }
        219 => {
            let r255 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r255);
            let r256 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r256);
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_e() { blk = 225; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t257) = c.ld16(c.r64(11).wrapping_add(c.r64(2)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6bc1); };
            c.w32(15, t257);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_e() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t258) = c.ld16(c.r64(12).wrapping_add(c.r64(9)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6bcc); };
            c.cmp(t258, c.r16(15), 16);
            if c.cc_e() { blk = 219; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 227; continue;
        }
        224 => {
            return RegfileExit::fault(0x70008d6bd8);
        }
        225 => {
            c.w32(2, 0x1u64);
            blk = 227; continue;
        }
        226 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6be7); };
            c.w64(2, t259);
            let r260 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r260);
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008d6bef); };
            c.w64(0, t261);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d6bf9); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d6bfa); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d6c00); }
            return c.ret(0x0, 0x70008d6c02);
        }
        229 => {
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6c03); };
            c.w64(2, t262);
            let r263 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r263);
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008d6c0b); };
            c.w64(0, t264);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d6c15); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d6c16); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d6c1c); }
            return c.ret(0x0, 0x70008d6c1e);
        }
        230 => {
            return RegfileExit::fault(0x70008d6c1f);
        }
        231 => {
            return RegfileExit::fault(0x70008d6c20);
        }
        232 => {
            return RegfileExit::fault(0x70008d6c21);
        }
        233 => {
            return RegfileExit::fault(0x70008d6c22);
        }
        234 => {
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6c23); };
            c.w64(2, t265);
            let r266 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r266);
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008d6c2b); };
            c.w64(0, t267);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d6c35); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d6c36); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(2).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d6c3c); }
            return c.ret(0x0, 0x70008d6c3e);
        }
        235 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d6c3f); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d6c45); };
            c.w64(3, t268);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008d6c54, 0x70008d6c4f);
        }
        236 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6c54); };
            c.w64(6, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6c58); };
            c.w64(0, t270);
            blk = 1; continue;
        }
        237 => {
            let Some(t271) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d6c61); };
            c.cmp(t271, c.r32(8), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let Some(t272) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d6c70); };
            c.cmp(t272, c.r32(8), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let r273 = c.r32(7) | c.r32(12);
            c.w32(7, r273);
            c.logic(r273, 32);
            if c.cc_ne() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d6c8c, 0x70008d6c87);
        }
        239 => {
            c.w64(8, c.r64(0));
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6c8f); };
            c.w64(9, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6c93); };
            c.w64(2, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6c97); };
            c.w64(6, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6c9b); };
            c.w64(11, t277);
            blk = 8; continue;
        }
        240 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d6ca9); }
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d6caa); };
            c.w64(2, t278);
            let Some(t279) = c.ld32(c.r64(2).wrapping_add(0x387)) else { return RegfileExit::fault(0x70008d6cb1); };
            c.w32(2, t279);
            let r280 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r280);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008d6cba); }
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d6cbb); };
            c.w64(3, t281);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d6ccc, 0x70008d6cc7);
        }
        241 => {
            return RegfileExit::fault(0x70008d6ccc);
        }
        242 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d6cd5, 0x70008d6cd0);
        }
        243 => {
            c.w64(8, c.r64(0));
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6cd8); };
            c.w64(9, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6cdc); };
            c.w64(2, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6ce0); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008d6ce4); };
            c.w64(11, t285);
            blk = 8; continue;
        }
        244 => {
            let Some(t286) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d6ced); };
            c.logic(t286 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 245; continue;
        }
        245 => {
            blk = 20; continue;
        }
        246 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d6cff); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 20; continue; }
            blk = 247; continue;
        }
        247 => {
            blk = 26; continue;
        }
        248 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6d17); };
            c.w64(6, t287);
            return c.call(0x700089c794, 0x70008d6d20, 0x70008d6d1b);
        }
        249 => {
            blk = 38; continue;
        }
        250 => {
            return RegfileExit::jump(0x70008d6d28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d6d34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d6d34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d6d38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d6d39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d6d3a); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x70008d6d3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d6d43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 179; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d6d5b); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008d6d60); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 186; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6d6e); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6d75); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 181; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6d81); };
            c.w32(9, t6);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d6d88); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008d6d8c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d6d90); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d6d9a); };
            c.w64(1, t7);
            c.w64(12, c.r64(1));
            blk = 6; continue;
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d6da3); };
            c.w64(12, t8);
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d6daa); };
            c.w64(1, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d6db1); };
            c.cmp(t10, c.r32(12), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        8 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(12));
            blk = 12; continue;
        }
        10 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x70008d6dd5, 0x70008d6dd0);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d6dd5); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6dd9); };
            c.w64(11, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6ddd); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d6de1); };
            c.w64(9, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d6de5); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d6de9); };
            c.w64(1, t16);
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_l() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 34; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(12, c.r64(9));
            blk = 34; continue;
        }
        16 => {
            let r17 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r17);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_b() { blk = 34; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        18 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d6e25); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let r18 = c.r64(10) | 0x1u64;
            c.w64(10, r18);
            c.logic(r18, 64);
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 22; continue;
        }
        22 => {
            c.xorpd_rr(2, 2);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 29; continue; }
            blk = 23; continue;
        }
        23 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            c.mov128_rr(0, 1);
            blk = 25; continue;
        }
        25 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(10, 0x8000000000000000u64);
            let r19 = c.r64(12) | c.r64(10);
            c.w64(12, r19);
            c.logic(r19, 64);
            blk = 28; continue;
        }
        28 => {
            blk = 34; continue;
        }
        29 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 34; continue;
        }
        31 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t20) = c.ld64(0x70008d6e70u64) else { return RegfileExit::fault(0x70008d6eb4); };
            c.w64(10, t20);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t21) = c.ld64(0x70008d6e8eu64) else { return RegfileExit::fault(0x70008d6ecf); };
            c.w64(10, t21);
            let r22 = c.r64(12) | c.r64(10);
            c.w64(12, r22);
            c.logic(r22, 64);
            blk = 34; continue;
        }
        34 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008d6edc); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(1, c.r64(7));
            let Some(t23) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d6ee9); };
            c.w64(1, t23);
            blk = 36; continue;
        }
        36 => {
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d6eed); };
            c.cmp(t24, c.r32(1), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let r25 = c.sub(c.r64(9), c.r64(12), 64);
            c.w64(9, r25);
            c.w64(1, c.r64(9));
            blk = 59; continue;
        }
        38 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(0, c.r64(1));
            blk = 42; continue;
        }
        40 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008d6f13, 0x70008d6f0e);
        }
        41 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d6f13); };
            c.w64(9, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d6f17); };
            c.w64(11, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d6f1b); };
            c.w64(8, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d6f1f); };
            c.w64(12, t29);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 47; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 46; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let r30 = c.sub(c.r64(9), c.r64(12), 64);
            c.w64(9, r30);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w64(1, c.r64(9));
            blk = 59; continue;
        }
        46 => {
            c.w64(1, c.r64(15));
            blk = 59; continue;
        }
        47 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d6f51); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 58; continue; }
            blk = 48; continue;
        }
        48 => {
            let r31 = c.sub(c.r64(9), c.r64(12), 64);
            c.w64(9, r31);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 52; continue; }
            blk = 49; continue;
        }
        49 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let r32 = c.r64(10) | 0x1u64;
            c.w64(10, r32);
            c.logic(r32, 64);
            blk = 51; continue;
        }
        51 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 52; continue;
        }
        52 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 57; continue; }
            blk = 53; continue;
        }
        53 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t33) = c.ld64(0x70008d6e70u64) else { return RegfileExit::fault(0x70008d6f9a); };
            c.w64(10, t33);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_s() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t34) = c.ld64(0x70008d6e8eu64) else { return RegfileExit::fault(0x70008d6fb5); };
            c.w64(10, t34);
            let r35 = c.r64(9) | c.r64(10);
            c.w64(9, r35);
            c.logic(r35, 64);
            blk = 56; continue;
        }
        56 => {
            c.w64(1, c.r64(9));
            blk = 59; continue;
        }
        57 => {
            c.w64(1, c.r64(9));
            blk = 59; continue;
        }
        58 => {
            c.w64(1, c.r64(15));
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 178; continue; }
            blk = 60; continue;
        }
        60 => {
            let r36 = c.add(c.r64(1), c.r64(12), 64);
            c.w64(1, r36);
            c.w64(0, c.r64(8));
            let Some(t37) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d6fdc); };
            c.w32(7, t37);
            let r38 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r38);
            c.w64(8, c.r64(1));
            let r39 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r39);
            let Some(t40) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6fe8); };
            c.w32(9, t40);
            let Some(t41) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d6fec); };
            c.w32(7, t41);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_be() { blk = 174; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 62; continue;
        }
        62 => {
            c.logic(c.r8(7) & 0x7u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_e() { blk = 148; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(3, c.r64(7));
            c.w64(7, c.r64(15));
            c.w64(9, c.r64(0));
            blk = 65; continue;
        }
        65 => {
            c.w64(2, c.r64(3));
            let r42 = c.r32(2) & 0x7u64;
            c.w32(2, r42);
            c.logic(r42, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 76; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 176; continue;
        }
        70 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            blk = 176; continue;
        }
        73 => {
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d705d); };
            c.w32(3, t43);
            let Some(t44) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d7061); };
            c.cmp(t44, c.r32(3), 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d706e); };
            c.w32(9, t45);
            let r46 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r46);
            let Some(t47) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7075); };
            c.w32(3, t47);
            let r48 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r48);
            let Some(t49) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d707c); };
            c.w32(3, t49);
            blk = 65; continue;
        }
        75 => {
            let Some(t50) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d7082); };
            c.w32(3, t50);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r51 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r51);
            let Some(t52) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d708f); };
            c.w32(9, t52);
            let r53 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r53);
            let Some(t54) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7096); };
            c.w32(3, t54);
            let r55 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r55);
            let Some(t56) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d709d); };
            c.w32(3, t56);
            blk = 65; continue;
        }
        76 => {
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d70a3); };
            c.w32(9, t57);
            let r58 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r58);
            let Some(t59) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d70aa); };
            c.w32(3, t59);
            let r60 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r60);
            let Some(t61) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d70b1); };
            c.w32(3, t61);
            blk = 65; continue;
        }
        77 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 80; continue;
        }
        78 => {
            c.w32(2, 0x1u64);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 80; continue;
        }
        79 => {
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(15));
            c.w64(3, c.r64(0));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(8), 0xdu64, 64);
            if c.cc_l() { blk = 88; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            let r62 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r62);
            let r63 = c.r32(9) & 0x8u64;
            c.w32(9, r63);
            c.logic(r63, 32);
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7100); };
            c.w64(9, t64);
            let Some(t65) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d7107); };
            c.w64(11, t65);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d710e); };
            c.w64(15, t66);
            let Some(t67) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d7115); };
            c.cmp(t67, c.r64(12), 64);
            if c.cc_be() { blk = 188; continue; }
            blk = 83; continue;
        }
        83 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008d711e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 84; continue;
        }
        84 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d7125); };
            c.w64(11, t68);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d712c); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d7130); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d7138); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d713c); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d7140); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d714a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d714b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7151); }
            return c.ret(0x0, 0x70008d7153);
        }
        85 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7154); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d715b); };
            c.w64(11, t70);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7162); };
            c.w64(12, t71);
            let Some(t72) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d7169); };
            c.cmp(t72, c.r64(15), 64);
            if c.cc_be() { blk = 190; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008d7173); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 87; continue;
        }
        87 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d717a); };
            c.w64(11, t73);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d7181); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d7185); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d718d); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d7191); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d7195); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d719f); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d71a0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d71a6); }
            return c.ret(0x0, 0x70008d71a8);
        }
        88 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w32(1, c.r32(8));
            let r74 = c.r32(9) & 0x8u64;
            c.w32(9, r74);
            c.logic(r74, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w32(1, c.r32(8));
            c.w64(9, c.r64(1));
            let r75 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r75);
            if c.cc_o() { blk = 98; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r76 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r76);
            if c.cc_o() { blk = 97; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d71ee); };
            c.w64(9, t77);
            let Some(t78) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d71f5); };
            c.w64(9, t78);
            let r79 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r79);
            let r80 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r80);
            c.logic(r80, 64);
            c.w64(15, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7204); };
            c.w64(0, t81);
            let Some(t82) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d720b); };
            c.cmp(t82, c.r64(15), 64);
            if c.cc_be() { blk = 192; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 192; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7221); };
            c.w64(1, t83);
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008d7228); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 96; continue;
        }
        96 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d722f); };
            c.w64(9, t84);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d7236); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d723a); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d7241); }
            c.w64(0, c.r64(1));
            blk = 100; continue;
        }
        97 => {
            return RegfileExit::fault(0x70008d724a);
        }
        98 => {
            return RegfileExit::fault(0x70008d724b);
        }
        99 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d724c); };
            c.w64(0, t85);
            blk = 100; continue;
        }
        100 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 104; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w32(8, 0xbu64);
            blk = 103; continue;
        }
        102 => {
            let r86 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r86);
            c.w64(1, c.r64(9));
            blk = 103; continue;
        }
        103 => {
            c.w64(9, c.r64(1).wrapping_add(0x2));
            let Some(t87) = c.ld16(c.r64(3).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008d727b); };
            c.w32(1, t87);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70008d727f); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 102; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7290); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7291); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7297); }
            return c.ret(0x0, 0x70008d7299);
        }
        105 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r88 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r88);
            if c.cc_o() { blk = 111; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d72ad); };
            c.w64(9, t89);
            let Some(t90) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d72b4); };
            c.w64(9, t90);
            let r91 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r91);
            let r92 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r92);
            c.logic(r92, 64);
            c.w64(15, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d72c3); };
            c.w64(0, t93);
            let Some(t94) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d72ca); };
            c.cmp(t94, c.r64(15), 64);
            if c.cc_be() { blk = 194; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 194; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d72e0); };
            c.w64(1, t95);
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008d72e7); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 110; continue;
        }
        110 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d72ee); };
            c.w64(9, t96);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d72f5); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d72f9); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d7300); }
            c.w64(0, c.r64(1));
            blk = 113; continue;
        }
        111 => {
            return RegfileExit::fault(0x70008d7309);
        }
        112 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d730a); };
            c.w64(0, t97);
            blk = 113; continue;
        }
        113 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 117; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w32(8, 0xbu64);
            blk = 116; continue;
        }
        115 => {
            let r98 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r98);
            c.w64(1, c.r64(9));
            blk = 116; continue;
        }
        116 => {
            c.w64(9, c.r64(1).wrapping_add(0x1));
            let Some(t99) = c.ld8(c.r64(3).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008d7332); };
            c.w32(1, t99);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70008d7336); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 115; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7346); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7347); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d734d); }
            return c.ret(0x0, 0x70008d734f);
        }
        118 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 176; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w32(1, c.r32(8));
            let Some(t100) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d735d); };
            c.w64(15, t100);
            let r101 = c.r32(9) & 0x8u64;
            c.w32(9, r101);
            c.logic(r101, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w32(1, c.r32(8));
            c.w64(9, c.r64(1));
            let r102 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r102);
            if c.cc_o() { blk = 128; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r103 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r103);
            if c.cc_o() { blk = 127; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d739a); };
            c.w64(9, t104);
            let Some(t105) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d73a1); };
            c.w64(9, t105);
            let r106 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r106);
            let r107 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r107);
            c.logic(r107, 64);
            c.w64(0, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d73b0); };
            c.w64(3, t108);
            let Some(t109) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d73b7); };
            c.cmp(t109, c.r64(0), 64);
            if c.cc_be() { blk = 196; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 196; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d73cd); };
            c.w64(1, t110);
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008d73d4); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 126; continue;
        }
        126 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d73db); };
            c.w64(9, t111);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d73e2); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d73e6); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d73ed); }
            c.w64(0, c.r64(1));
            blk = 130; continue;
        }
        127 => {
            return RegfileExit::fault(0x70008d73f6);
        }
        128 => {
            return RegfileExit::fault(0x70008d73f7);
        }
        129 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d73f8); };
            c.w64(0, t112);
            blk = 130; continue;
        }
        130 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w32(8, 0xbu64);
            blk = 133; continue;
        }
        132 => {
            let r113 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r113);
            c.w64(1, c.r64(9));
            blk = 133; continue;
        }
        133 => {
            c.w64(9, c.r64(1).wrapping_add(0x2));
            let Some(t114) = c.ld16(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d7427); };
            c.w32(1, t114);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70008d742d); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 132; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d743e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d743f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7445); }
            return c.ret(0x0, 0x70008d7447);
        }
        135 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r115 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r115);
            if c.cc_o() { blk = 141; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d745b); };
            c.w64(9, t116);
            let Some(t117) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d7462); };
            c.w64(9, t117);
            let r118 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r118);
            let r119 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r119);
            c.logic(r119, 64);
            c.w64(0, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7471); };
            c.w64(3, t120);
            let Some(t121) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d7478); };
            c.cmp(t121, c.r64(0), 64);
            if c.cc_be() { blk = 198; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 198; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d748e); };
            c.w64(1, t122);
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008d7495); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 140; continue;
        }
        140 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d749c); };
            c.w64(9, t123);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d74a3); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d74a7); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d74ae); }
            c.w64(0, c.r64(1));
            blk = 143; continue;
        }
        141 => {
            return RegfileExit::fault(0x70008d74b7);
        }
        142 => {
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d74b8); };
            c.w64(0, t124);
            blk = 143; continue;
        }
        143 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 147; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w32(8, 0xbu64);
            blk = 146; continue;
        }
        145 => {
            let r125 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r125);
            c.w64(1, c.r64(9));
            blk = 146; continue;
        }
        146 => {
            c.w64(9, c.r64(1).wrapping_add(0x1));
            let Some(t126) = c.ld8(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d74e0); };
            c.w32(1, t126);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70008d74e6); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 145; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d74f6); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d74f7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d74fd); }
            return c.ret(0x0, 0x70008d74ff);
        }
        148 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 164; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(15));
            c.w64(7, c.r64(0));
            blk = 150; continue;
        }
        150 => {
            c.w64(9, c.r64(8));
            let r127 = c.r32(9) & 0x7u64;
            c.w32(9, r127);
            c.logic(r127, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 155; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 160; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 154; continue;
        }
        154 => {
            blk = 202; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            blk = 202; continue;
        }
        158 => {
            let Some(t128) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d754f); };
            c.w32(8, t128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d7553); };
            c.cmp(t129, c.r32(8), 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d7560); };
            c.w32(7, t130);
            let r131 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r131);
            let Some(t132) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7566); };
            c.w32(8, t132);
            let r133 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r133);
            let Some(t134) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d756d); };
            c.w32(8, t134);
            blk = 150; continue;
        }
        160 => {
            let Some(t135) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d7574); };
            c.w32(8, t135);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r136 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r136);
            let Some(t137) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d7581); };
            c.w32(7, t137);
            let r138 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r138);
            let Some(t139) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7587); };
            c.w32(8, t139);
            let r140 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r140);
            let Some(t141) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d758e); };
            c.w32(8, t141);
            blk = 150; continue;
        }
        161 => {
            let Some(t142) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d7595); };
            c.w32(7, t142);
            let r143 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r143);
            let Some(t144) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d759b); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            let Some(t146) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d75a2); };
            c.w32(8, t146);
            blk = 150; continue;
        }
        162 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 165; continue;
        }
        163 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0x1u64);
            blk = 165; continue;
        }
        164 => {
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 200; continue; }
            blk = 166; continue;
        }
        166 => {
            let r147 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r147);
            blk = 167; continue;
        }
        167 => {
            let r148 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r148);
            let r149 = c.r32(1) & 0x8u64;
            c.w32(1, r149);
            c.logic(r149, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t150) = c.ld16(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d75ff); };
            c.w32(1, t150);
            blk = 170; continue;
        }
        169 => {
            let Some(t151) = c.ld8(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008d7606); };
            c.w32(1, t151);
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 215; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d761f); };
            c.w64(8, t152);
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7626); };
            c.w32(0, t153);
            let r154 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r154);
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d762e); };
            c.cmp(t155, c.r32(0), 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 172; continue;
        }
        172 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7642); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7643); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7649); }
            return c.ret(0x0, 0x70008d764b);
        }
        173 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d7650); };
            c.w64(0, t156);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d765a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d765b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7661); }
            return c.ret(0x0, 0x70008d7663);
        }
        174 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7671); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7672); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7678); }
            return c.ret(0x0, 0x70008d767a);
        }
        176 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let r157 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r157);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d7681); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d7682); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d7683); }
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008d7684); };
            c.w64(3, t158);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x70008d7698, 0x70008d7693);
        }
        177 => {
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d7698); };
            c.w64(1, t159);
            let r160 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r160);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d76a3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d76a4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d76aa); }
            return c.ret(0x0, 0x70008d76ac);
        }
        178 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d76b1); };
            c.w64(0, t161);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d76bb); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d76bc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d76c2); }
            return c.ret(0x0, 0x70008d76c4);
        }
        179 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d76c5); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d76cb); };
            c.w64(3, t162);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008d76da, 0x70008d76d5);
        }
        180 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d76da); };
            c.w64(6, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d76de); };
            c.w64(0, t164);
            blk = 1; continue;
        }
        181 => {
            let Some(t165) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d76e7); };
            c.cmp(t165, c.r32(8), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t166) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d76f6); };
            c.cmp(t166, c.r32(8), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d7705); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d7709); }
            let r167 = c.r32(9) | c.r32(11);
            c.w32(9, r167);
            c.logic(r167, 32);
            if c.cc_ne() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d771a, 0x70008d7715);
        }
        183 => {
            c.w64(8, c.r64(0));
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d771d); };
            c.w64(6, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d7721); };
            c.w64(7, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7725); };
            c.w64(1, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d7729); };
            c.w64(0, t171);
            blk = 3; continue;
        }
        184 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d7737); }
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d7738); };
            c.w64(1, t172);
            let Some(t173) = c.ld32(c.r64(1).wrapping_add(0x38b)) else { return RegfileExit::fault(0x70008d773f); };
            c.w32(1, t173);
            let r174 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r174);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d7748); }
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d7749); };
            c.w64(3, t175);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d775a, 0x70008d7755);
        }
        185 => {
            return RegfileExit::fault(0x70008d775a);
        }
        186 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d775b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d775f); }
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d776b, 0x70008d7766);
        }
        187 => {
            c.w64(8, c.r64(0));
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d776e); };
            c.w64(6, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d7772); };
            c.w64(7, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7776); };
            c.w64(1, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d777a); };
            c.w64(0, t179);
            blk = 3; continue;
        }
        188 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d7788); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d778c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008d7790); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008d7794); }
            return c.call(0x70007febf4, 0x70008d779d, 0x70008d7798);
        }
        189 => {
            c.w64(9, c.r64(0));
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d77a0); };
            c.w64(8, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d77a4); };
            c.w64(3, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d77a8); };
            c.w64(7, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d77ac); };
            c.w64(1, t183);
            blk = 84; continue;
        }
        190 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d77b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d77b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008d77bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008d77c1); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d77cf, 0x70008d77ca);
        }
        191 => {
            c.w64(9, c.r64(0));
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d77d2); };
            c.w64(8, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d77d6); };
            c.w64(3, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d77da); };
            c.w64(7, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d77de); };
            c.w64(1, t187);
            blk = 87; continue;
        }
        192 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d77e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d77eb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d77ef); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d77fb, 0x70008d77f6);
        }
        193 => {
            c.w64(1, c.r64(0));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d77fe); };
            c.w64(11, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7802); };
            c.w64(12, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d7806); };
            c.w64(8, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d780a); };
            c.w64(7, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d780e); };
            c.w64(3, t192);
            blk = 96; continue;
        }
        194 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d7817); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d781b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d781f); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d782b, 0x70008d7826);
        }
        195 => {
            c.w64(1, c.r64(0));
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d782e); };
            c.w64(11, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7832); };
            c.w64(12, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d7836); };
            c.w64(8, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d783a); };
            c.w64(7, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d783e); };
            c.w64(3, t197);
            blk = 110; continue;
        }
        196 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008d7847); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d784b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d784f); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d785b, 0x70008d7856);
        }
        197 => {
            c.w64(1, c.r64(0));
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d785e); };
            c.w64(11, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7862); };
            c.w64(12, t199);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d7866); };
            c.w64(15, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d786a); };
            c.w64(8, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d786e); };
            c.w64(7, t202);
            blk = 126; continue;
        }
        198 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008d7877); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008d787b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d787f); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d788b, 0x70008d7886);
        }
        199 => {
            c.w64(1, c.r64(0));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d788e); };
            c.w64(11, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7892); };
            c.w64(12, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d7896); };
            c.w64(15, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d789a); };
            c.w64(8, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d789e); };
            c.w64(7, t207);
            blk = 140; continue;
        }
        200 => {
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t208) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d78ac); };
            c.w64(8, t208);
            blk = 167; continue;
        }
        202 => {
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w32(1, c.r64(12).wrapping_add(c.r64(12)));
            blk = 213; continue;
        }
        204 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d78c4); };
            c.w64(1, t209);
            let Some(t210) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d78cb); };
            c.w64(7, t210);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d78d2); };
            c.w64(9, t211);
            let Some(t212) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d78d9); };
            c.cmp(t212, c.r64(8), 64);
            if c.cc_a() { blk = 207; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008d78e8, 0x70008d78e3);
        }
        206 => {
            c.w64(1, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d78eb); };
            c.w64(11, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d78ef); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d78f3); };
            c.w64(12, t215);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 208; continue;
        }
        207 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008d78fc); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 208; continue;
        }
        208 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 212; continue; }
            blk = 209; continue;
        }
        209 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            let r216 = c.r64(10) | 0x1u64;
            c.w64(10, r216);
            c.logic(r216, 64);
            blk = 211; continue;
        }
        211 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 212; continue;
        }
        212 => {
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d792a); };
            c.w64(7, t217);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008d7931); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d7934); }
            blk = 213; continue;
        }
        213 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d7939); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d793a); }
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d793b); };
            c.w64(3, t218);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x70008d794f, 0x70008d794a);
        }
        214 => {
            let r219 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r219);
            c.w64(1, c.r64(0));
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d7954); };
            c.w64(11, t220);
            blk = 170; continue;
        }
        215 => {
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d795d); };
            c.w64(7, t221);
            let Some(t222) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d7964); };
            c.w64(8, t222);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let r223 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r223);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d796f); }
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7973); };
            c.w64(12, t224);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d797a); }
            let Some(t225) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d797e); };
            c.cmp(t225, c.r64(9), 64);
            if c.cc_a() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d798e, 0x70008d7989);
        }
        217 => {
            blk = 219; continue;
        }
        218 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008d7990); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 219; continue;
        }
        219 => {
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d799a); };
            c.w64(1, t226);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d79a1); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d79a4); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d79ab); }
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d79b2); };
            c.w64(1, t227);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70008d79b6); }
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d79ba); };
            c.w64(1, t228);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d79c1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d79c2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d79c8); }
            return c.ret(0x0, 0x70008d79ca);
        }
        220 => {
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d79cb); };
            c.w64(9, t229);
            let Some(t230) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d79d2); };
            c.w64(12, t230);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008d79da); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d79de); }
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d79e2); };
            c.w64(0, t231);
            let Some(t232) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d79e9); };
            c.cmp(t232, c.r64(15), 64);
            if c.cc_a() { blk = 223; continue; }
            blk = 221; continue;
        }
        221 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d79f8, 0x70008d79f3);
        }
        222 => {
            blk = 224; continue;
        }
        223 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008d79fa); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 224; continue;
        }
        224 => {
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d7a05); };
            c.w64(1, t233);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d7a0c); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d7a0f); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d7a16); }
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7a1d); };
            c.w64(1, t234);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008d7a21); }
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d7a24); };
            c.w64(8, t235);
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d7a2b); };
            c.w32(7, t236);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            return RegfileExit::fault(0x70008d7a3a);
        }
        226 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d7a3b); };
            c.w64(7, t237);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008d7a3f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r238 = c.r64(1) & c.r64(8);
            c.w64(1, r238);
            c.logic(r238, 64);
            let Some(t239) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d7a51); };
            c.logic(t239 & 0x4u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d7a57); };
            c.w64(7, t240);
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7a62); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7a63); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7a69); }
            return c.ret(0x0, 0x70008d7a6b);
        }
        229 => {
            c.w32(0, c.r32(0));
            let r241 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r241);
            c.w64(1, 0xfffffffffffc0000u64);
            let r242 = c.r64(1) & c.r64(0);
            c.w64(1, r242);
            c.logic(r242, 64);
            let Some(t243) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d7a7b); };
            c.logic(t243 & 0x2u64, 8);
            if c.cc_e() { blk = 228; continue; }
            blk = 230; continue;
        }
        230 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d7a85); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d7a86); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008d7a93, 0x70008d7a8e);
        }
        231 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008d7a93); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008d7a95); }
            blk = 228; continue;
        }
        232 => {
            return RegfileExit::jump(0x70008d7a9c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d7ab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d7ab4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d7ab8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d7ab9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d7aba); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008d7abf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d7ac3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 174; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(5));
            c.w64(7, c.r64(1).wrapping_add(0x18));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(1)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d7adb); };
            c.w64(8, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008d7ae0); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7aee); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7af5); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 176; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7b01); };
            c.w32(9, t6);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008d7b08); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008d7b0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d7b10); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(1).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d7b1a); };
            c.w64(1, t7);
            c.w64(12, c.r64(1));
            blk = 6; continue;
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d7b23); };
            c.w64(12, t8);
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d7b2a); };
            c.w64(1, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d7b31); };
            c.cmp(t10, c.r32(12), 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        8 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(0, c.r64(12));
            blk = 12; continue;
        }
        10 => {
            c.w64(0, c.r64(12));
            return c.call(0x7000899a14, 0x70008d7b55, 0x70008d7b50);
        }
        11 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d7b55); };
            c.w64(7, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d7b59); };
            c.w64(11, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d7b5d); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7b61); };
            c.w64(9, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d7b65); };
            c.w64(6, t15);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d7b69); };
            c.w64(1, t16);
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_a() { blk = 29; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(12, c.r64(9));
            blk = 29; continue;
        }
        16 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        17 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d7b98); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 28; continue; }
            blk = 18; continue;
        }
        18 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let r17 = c.r64(10) | 0x1u64;
            c.w64(10, r17);
            c.logic(r17, 64);
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 22; continue;
        }
        22 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 27; continue; }
            blk = 23; continue;
        }
        23 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 26; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(12, cv);
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_s() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(10, 0x8000000000000000u64);
            let r18 = c.r64(12) | c.r64(10);
            c.w64(12, r18);
            c.logic(r18, 64);
            blk = 26; continue;
        }
        26 => {
            blk = 29; continue;
        }
        27 => {
            c.w64(12, c.r64(9));
            blk = 29; continue;
        }
        28 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 29; continue;
        }
        29 => {
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d7c16); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(1, c.r64(7));
            let Some(t19) = c.ld64(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d7c23); };
            c.w64(1, t19);
            blk = 31; continue;
        }
        31 => {
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d7c27); };
            c.cmp(t20, c.r32(1), 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(1, c.r64(9));
            blk = 54; continue;
        }
        33 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(0, c.r64(1));
            blk = 37; continue;
        }
        35 => {
            c.w64(0, c.r64(1));
            return c.call(0x7000899a14, 0x70008d7c4a, 0x70008d7c45);
        }
        36 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d7c4a); };
            c.w64(9, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d7c4e); };
            c.w64(11, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d7c52); };
            c.w64(8, t23);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d7c59); };
            c.w64(12, t24);
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_a() { blk = 54; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(1, c.r64(9));
            blk = 54; continue;
        }
        41 => {
            c.w64(1, c.r64(15));
            blk = 54; continue;
        }
        42 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008d7c85); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 53; continue; }
            blk = 43; continue;
        }
        43 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 47; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let r25 = c.r64(10) | 0x1u64;
            c.w64(10, r25);
            c.logic(r25, 64);
            blk = 46; continue;
        }
        46 => {
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(10) as i64));
            c.arith_sd(0, 1, c.xmm_u64(1));
            blk = 47; continue;
        }
        47 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t26) = c.ld64(0x70008d7be0u64) else { return RegfileExit::fault(0x70008d7ccb); };
            c.w64(10, t26);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(9, cv);
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_s() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t27) = c.ld64(0x70008d7bfeu64) else { return RegfileExit::fault(0x70008d7ce6); };
            c.w64(10, t27);
            let r28 = c.r64(9) | c.r64(10);
            c.w64(9, r28);
            c.logic(r28, 64);
            blk = 51; continue;
        }
        51 => {
            c.w64(1, c.r64(9));
            blk = 54; continue;
        }
        52 => {
            c.w64(1, c.r64(9));
            blk = 54; continue;
        }
        53 => {
            c.w64(1, c.r64(15));
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_ae() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(1));
            c.w64(1, c.r64(10));
            blk = 56; continue;
        }
        56 => {
            c.w64(0, c.r64(8));
            let Some(t29) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7d0e); };
            c.w32(7, t29);
            let r30 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r30);
            c.w64(8, c.r64(1));
            let r31 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7d1a); };
            c.w32(9, t32);
            let Some(t33) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7d1e); };
            c.w32(7, t33);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_be() { blk = 170; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_e() { blk = 169; continue; }
            blk = 58; continue;
        }
        58 => {
            c.logic(c.r8(7) & 0x7u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_e() { blk = 144; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(3, c.r64(7));
            c.w64(7, c.r64(15));
            c.w64(9, c.r64(0));
            blk = 61; continue;
        }
        61 => {
            c.w64(2, c.r64(3));
            let r34 = c.r32(2) & 0x7u64;
            c.w32(2, r34);
            c.logic(r34, 32);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_l() { blk = 66; continue; }
            blk = 62; continue;
        }
        62 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 65; continue;
        }
        65 => {
            blk = 172; continue;
        }
        66 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 67; continue;
        }
        67 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            blk = 172; continue;
        }
        69 => {
            let Some(t35) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d7d8f); };
            c.w32(3, t35);
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d7d93); };
            c.cmp(t36, c.r32(3), 32);
            if c.cc_ne() { blk = 172; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d7da0); };
            c.w32(9, t37);
            let r38 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r38);
            let Some(t39) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7da7); };
            c.w32(3, t39);
            let r40 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r40);
            let Some(t41) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7dae); };
            c.w32(3, t41);
            blk = 61; continue;
        }
        71 => {
            let Some(t42) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d7db4); };
            c.w32(3, t42);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r43 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r43);
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d7dc1); };
            c.w32(9, t44);
            let r45 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r45);
            let Some(t46) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7dc8); };
            c.w32(3, t46);
            let r47 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r47);
            let Some(t48) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7dcf); };
            c.w32(3, t48);
            blk = 61; continue;
        }
        72 => {
            let Some(t49) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d7dd5); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d7ddc); };
            c.w32(3, t51);
            let r52 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r52);
            let Some(t53) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d7de3); };
            c.w32(3, t53);
            blk = 61; continue;
        }
        73 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 76; continue;
        }
        74 => {
            c.w32(2, 0x1u64);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(9));
            c.w64(9, c.r64(10));
            blk = 76; continue;
        }
        75 => {
            c.w64(9, c.r64(7));
            c.w64(7, c.r64(15));
            c.w64(3, c.r64(0));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r64(8), 0xdu64, 64);
            if c.cc_l() { blk = 84; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            let r54 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r54);
            let r55 = c.r32(9) & 0x8u64;
            c.w32(9, r55);
            c.logic(r55, 32);
            c.w64(7, c.r64(11).wrapping_add(0x1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7e32); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d7e39); };
            c.w64(11, t57);
            c.w64(12, c.r64(11).wrapping_add(0x14));
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7e40); };
            c.w64(15, t58);
            let Some(t59) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008d7e47); };
            c.cmp(t59, c.r64(12), 64);
            if c.cc_be() { blk = 183; continue; }
            blk = 79; continue;
        }
        79 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008d7e50); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 80; continue;
        }
        80 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d7e57); };
            c.w64(11, t60);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d7e5e); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d7e62); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d7e6a); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d7e6e); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d7e72); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7e7c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7e7d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7e83); }
            return c.ret(0x0, 0x70008d7e85);
        }
        81 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7e86); };
            c.w64(9, t61);
            let Some(t62) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d7e8d); };
            c.w64(11, t62);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7e94); };
            c.w64(12, t63);
            let Some(t64) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d7e9b); };
            c.cmp(t64, c.r64(15), 64);
            if c.cc_be() { blk = 185; continue; }
            blk = 82; continue;
        }
        82 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008d7ea5); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 83; continue;
        }
        83 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d7eac); };
            c.w64(11, t65);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d7eb3); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d7eb7); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d7ebf); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d7ec3); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008d7ec7); }
            c.w64(0, c.r64(9));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7ed1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7ed2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7ed8); }
            return c.ret(0x0, 0x70008d7eda);
        }
        84 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 114; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w32(1, c.r32(8));
            let r66 = c.r32(9) & 0x8u64;
            c.w32(9, r66);
            c.logic(r66, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w32(1, c.r32(8));
            c.w64(9, c.r64(1));
            let r67 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r67);
            if c.cc_o() { blk = 94; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r68 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r68);
            if c.cc_o() { blk = 93; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7f20); };
            c.w64(9, t69);
            let Some(t70) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d7f27); };
            c.w64(9, t70);
            let r71 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r71);
            let r72 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r72);
            c.logic(r72, 64);
            c.w64(15, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7f36); };
            c.w64(0, t73);
            let Some(t74) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d7f3d); };
            c.cmp(t74, c.r64(15), 64);
            if c.cc_be() { blk = 187; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 187; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7f53); };
            c.w64(1, t75);
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008d7f5a); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 92; continue;
        }
        92 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d7f61); };
            c.w64(9, t76);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d7f68); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d7f6c); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d7f73); }
            c.w64(0, c.r64(1));
            blk = 96; continue;
        }
        93 => {
            return RegfileExit::fault(0x70008d7f7c);
        }
        94 => {
            return RegfileExit::fault(0x70008d7f7d);
        }
        95 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d7f7e); };
            c.w64(0, t77);
            blk = 96; continue;
        }
        96 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 100; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w32(8, 0xbu64);
            blk = 99; continue;
        }
        98 => {
            let r78 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r78);
            c.w64(1, c.r64(9));
            blk = 99; continue;
        }
        99 => {
            c.w64(9, c.r64(1).wrapping_add(0x2));
            let Some(t79) = c.ld16(c.r64(3).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008d7fad); };
            c.w32(1, t79);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70008d7fb1); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 98; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d7fc2); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d7fc3); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d7fc9); }
            return c.ret(0x0, 0x70008d7fcb);
        }
        101 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r80 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r80);
            if c.cc_o() { blk = 107; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d7fdf); };
            c.w64(9, t81);
            let Some(t82) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d7fe6); };
            c.w64(9, t82);
            let r83 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r83);
            let r84 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r84);
            c.logic(r84, 64);
            c.w64(15, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d7ff5); };
            c.w64(0, t85);
            let Some(t86) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d7ffc); };
            c.cmp(t86, c.r64(15), 64);
            if c.cc_be() { blk = 189; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 189; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8012); };
            c.w64(1, t87);
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008d8019); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 106; continue;
        }
        106 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d8020); };
            c.w64(9, t88);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d8027); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d802b); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d8032); }
            c.w64(0, c.r64(1));
            blk = 109; continue;
        }
        107 => {
            return RegfileExit::fault(0x70008d803b);
        }
        108 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d803c); };
            c.w64(0, t89);
            blk = 109; continue;
        }
        109 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 113; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w32(8, 0xbu64);
            blk = 112; continue;
        }
        111 => {
            let r90 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r90);
            c.w64(1, c.r64(9));
            blk = 112; continue;
        }
        112 => {
            c.w64(9, c.r64(1).wrapping_add(0x1));
            let Some(t91) = c.ld8(c.r64(3).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x70008d8064); };
            c.w32(1, t91);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70008d8068); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 111; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8078); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8079); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d807f); }
            return c.ret(0x0, 0x70008d8081);
        }
        114 => {
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 172; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w32(1, c.r32(8));
            let Some(t92) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d808f); };
            c.w64(15, t92);
            let r93 = c.r32(9) & 0x8u64;
            c.w32(9, r93);
            c.logic(r93, 32);
            c.cmp(c.r32(1), 0x0u64, 32);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w32(1, c.r32(8));
            c.w64(9, c.r64(1));
            let r94 = c.add(c.r32(9), c.r32(1), 32);
            c.w32(9, r94);
            if c.cc_o() { blk = 124; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(1, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r95 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r95);
            if c.cc_o() { blk = 123; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d80cc); };
            c.w64(9, t96);
            let Some(t97) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d80d3); };
            c.w64(9, t97);
            let r98 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r98);
            let r99 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r99);
            c.logic(r99, 64);
            c.w64(0, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d80e2); };
            c.w64(3, t100);
            let Some(t101) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d80e9); };
            c.cmp(t101, c.r64(0), 64);
            if c.cc_be() { blk = 191; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 191; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d80ff); };
            c.w64(1, t102);
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008d8106); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 122; continue;
        }
        122 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d810d); };
            c.w64(9, t103);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d8114); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d8118); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d811f); }
            c.w64(0, c.r64(1));
            blk = 126; continue;
        }
        123 => {
            return RegfileExit::fault(0x70008d8128);
        }
        124 => {
            return RegfileExit::fault(0x70008d8129);
        }
        125 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d812a); };
            c.w64(0, t104);
            blk = 126; continue;
        }
        126 => {
            c.w64(1, c.r64(12).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8).wrapping_mul(2)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 130; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w32(8, 0xbu64);
            blk = 129; continue;
        }
        128 => {
            let r105 = c.add(c.r64(8), 0x2u64, 64);
            c.w64(8, r105);
            c.w64(1, c.r64(9));
            blk = 129; continue;
        }
        129 => {
            c.w64(9, c.r64(1).wrapping_add(0x2));
            let Some(t106) = c.ld16(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d8159); };
            c.w32(1, t106);
            if !c.st16(c.r64(0).wrapping_add(c.r64(8)), c.r16(1)) { return RegfileExit::fault(0x70008d815f); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 128; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8170); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8171); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d8177); }
            return c.ret(0x0, 0x70008d8179);
        }
        131 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 138; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(1, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(9, 0xcu64);
            let r107 = c.add(c.r64(1), c.r64(9), 64);
            c.w64(1, r107);
            if c.cc_o() { blk = 137; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d818d); };
            c.w64(9, t108);
            let Some(t109) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d8194); };
            c.w64(9, t109);
            let r110 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r110);
            let r111 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r111);
            c.logic(r111, 64);
            c.w64(0, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d81a3); };
            c.w64(3, t112);
            let Some(t113) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008d81aa); };
            c.cmp(t113, c.r64(0), 64);
            if c.cc_be() { blk = 193; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r64(1), 0x20000u64, 64);
            if c.cc_ae() { blk = 193; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d81c0); };
            c.w64(1, t114);
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x70008d81c7); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 136; continue;
        }
        136 => {
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d81ce); };
            c.w64(9, t115);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d81d5); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d81d9); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008d81e0); }
            c.w64(0, c.r64(1));
            blk = 139; continue;
        }
        137 => {
            return RegfileExit::fault(0x70008d81e9);
        }
        138 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d81ea); };
            c.w64(0, t116);
            blk = 139; continue;
        }
        139 => {
            c.w64(1, c.r64(7).wrapping_add(c.r64(12)).wrapping_add(0xb));
            c.w64(7, c.r64(1).wrapping_add(c.r64(8)));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 143; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w32(8, 0xbu64);
            blk = 142; continue;
        }
        141 => {
            let r117 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r117);
            c.w64(1, c.r64(9));
            blk = 142; continue;
        }
        142 => {
            c.w64(9, c.r64(1).wrapping_add(0x1));
            let Some(t118) = c.ld8(c.r64(1).wrapping_add(c.r64(15)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d8212); };
            c.w32(1, t118);
            if !c.st8(c.r64(0).wrapping_add(c.r64(8)), c.r8(1)) { return RegfileExit::fault(0x70008d8218); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 141; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8228); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8229); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d822f); }
            return c.ret(0x0, 0x70008d8231);
        }
        144 => {
            c.w32(1, c.r16(7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 160; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(15));
            c.w64(7, c.r64(0));
            blk = 146; continue;
        }
        146 => {
            c.w64(9, c.r64(8));
            let r119 = c.r32(9) & 0x7u64;
            c.w32(9, r119);
            c.logic(r119, 32);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 151; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 195; continue;
        }
        151 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            blk = 195; continue;
        }
        154 => {
            let Some(t120) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d8281); };
            c.w32(8, t120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d8285); };
            c.cmp(t121, c.r32(8), 32);
            if c.cc_ne() { blk = 196; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t122) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d8292); };
            c.w32(7, t122);
            let r123 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r123);
            let Some(t124) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8298); };
            c.w32(8, t124);
            let r125 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r125);
            let Some(t126) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d829f); };
            c.w32(8, t126);
            blk = 146; continue;
        }
        156 => {
            let Some(t127) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d82a6); };
            c.w32(8, t127);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r128 = c.add(c.r64(1), c.r64(8), 64);
            c.w64(1, r128);
            let Some(t129) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d82b3); };
            c.w32(7, t129);
            let r130 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r130);
            let Some(t131) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d82b9); };
            c.w32(8, t131);
            let r132 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r132);
            let Some(t133) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d82c0); };
            c.w32(8, t133);
            blk = 146; continue;
        }
        157 => {
            let Some(t134) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d82c7); };
            c.w32(7, t134);
            let r135 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r135);
            let Some(t136) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d82cd); };
            c.w32(8, t136);
            let r137 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r137);
            let Some(t138) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d82d4); };
            c.w32(8, t138);
            blk = 146; continue;
        }
        158 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 161; continue;
        }
        159 => {
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(7));
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(10));
            c.w32(9, 0x1u64);
            blk = 161; continue;
        }
        160 => {
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(0));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 162; continue;
        }
        162 => {
            let r139 = c.add(c.r64(8), 0xbu64, 64);
            c.w64(8, r139);
            blk = 163; continue;
        }
        163 => {
            let r140 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r140);
            let r141 = c.r32(1) & 0x8u64;
            c.w32(1, r141);
            c.logic(r141, 32);
            c.cmp(c.r32(1), 0x8u64, 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t142) = c.ld16(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d8331); };
            c.w32(1, t142);
            blk = 166; continue;
        }
        165 => {
            let Some(t143) = c.ld8(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008d8338); };
            c.w32(1, t143);
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(1), 0xffu64, 32);
            if c.cc_g() { blk = 212; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(7, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d8351); };
            c.w64(8, t144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8358); };
            c.w32(0, t145);
            let r146 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d8360); };
            c.cmp(t147, c.r32(0), 32);
            if c.cc_e() { blk = 217; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8374); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8375); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d837b); }
            return c.ret(0x0, 0x70008d837d);
        }
        169 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d8382); };
            c.w64(0, t148);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d838c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d838d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d8393); }
            return c.ret(0x0, 0x70008d8395);
        }
        170 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_a() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d83a3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d83a4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d83aa); }
            return c.ret(0x0, 0x70008d83ac);
        }
        172 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let r149 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r149);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d83b3); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d83b4); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d83b5); }
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008d83b6); };
            c.w64(3, t150);
            c.w32(0, 0x3u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x70008d83ca, 0x70008d83c5);
        }
        173 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d83ca); };
            c.w64(1, t151);
            let r152 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r152);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d83d5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d83d6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d83dc); }
            return c.ret(0x0, 0x70008d83de);
        }
        174 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d83df); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d83e5); };
            c.w64(3, t153);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008d83f4, 0x70008d83ef);
        }
        175 => {
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d83f4); };
            c.w64(6, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d83f8); };
            c.w64(0, t155);
            blk = 1; continue;
        }
        176 => {
            let Some(t156) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d8401); };
            c.cmp(t156, c.r32(8), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let Some(t157) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d8410); };
            c.cmp(t157, c.r32(8), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008d841f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d8423); }
            let r158 = c.r32(9) | c.r32(11);
            c.w32(9, r158);
            c.logic(r158, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(0, c.r64(8));
            return c.call(0x700089c794, 0x70008d8434, 0x70008d842f);
        }
        178 => {
            c.w64(8, c.r64(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d8437); };
            c.w64(6, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d843b); };
            c.w64(7, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d843f); };
            c.w64(1, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d8443); };
            c.w64(0, t162);
            blk = 3; continue;
        }
        179 => {
            c.w32(1, 0x36u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d8451); }
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d8452); };
            c.w64(1, t163);
            let Some(t164) = c.ld32(c.r64(1).wrapping_add(0x38f)) else { return RegfileExit::fault(0x70008d8459); };
            c.w32(1, t164);
            let r165 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r165);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d8462); }
            let Some(t166) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d8463); };
            c.w64(3, t166);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d8474, 0x70008d846f);
        }
        180 => {
            return RegfileExit::fault(0x70008d8474);
        }
        181 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008d8475); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d8479); }
            c.w64(0, c.r64(8));
            return c.call(0x70008a1214, 0x70008d8485, 0x70008d8480);
        }
        182 => {
            c.w64(8, c.r64(0));
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d8488); };
            c.w64(6, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d848c); };
            c.w64(7, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d8490); };
            c.w64(1, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d8494); };
            c.w64(0, t170);
            blk = 3; continue;
        }
        183 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008d84a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008d84a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008d84aa); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008d84ae); }
            return c.call(0x70007febf4, 0x70008d84b7, 0x70008d84b2);
        }
        184 => {
            c.w64(9, c.r64(0));
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d84ba); };
            c.w64(8, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d84be); };
            c.w64(3, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d84c2); };
            c.w64(7, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d84c6); };
            c.w64(1, t174);
            blk = 80; continue;
        }
        185 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008d84cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008d84d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008d84d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008d84db); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d84e9, 0x70008d84e4);
        }
        186 => {
            c.w64(9, c.r64(0));
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d84ec); };
            c.w64(8, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d84f0); };
            c.w64(3, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d84f4); };
            c.w64(7, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d84f8); };
            c.w64(1, t178);
            blk = 83; continue;
        }
        187 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d8501); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008d8505); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d8509); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008d850d); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d8519, 0x70008d8514);
        }
        188 => {
            c.w64(1, c.r64(0));
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d851c); };
            c.w64(11, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d8520); };
            c.w64(12, t180);
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d8524); };
            c.w64(8, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d8528); };
            c.w64(7, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d852c); };
            c.w64(3, t183);
            blk = 92; continue;
        }
        189 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d8535); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008d8539); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d853d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008d8541); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d854d, 0x70008d8548);
        }
        190 => {
            c.w64(1, c.r64(0));
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d8550); };
            c.w64(11, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d8554); };
            c.w64(12, t185);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d8558); };
            c.w64(8, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d855c); };
            c.w64(7, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d8560); };
            c.w64(3, t188);
            blk = 106; continue;
        }
        191 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d8569); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008d856d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008d8571); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d8575); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d8581, 0x70008d857c);
        }
        192 => {
            c.w64(1, c.r64(0));
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d8584); };
            c.w64(11, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d8588); };
            c.w64(12, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d858c); };
            c.w64(15, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d8590); };
            c.w64(8, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d8594); };
            c.w64(7, t193);
            blk = 122; continue;
        }
        193 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d859d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008d85a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008d85a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008d85a9); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febb4, 0x70008d85b5, 0x70008d85b0);
        }
        194 => {
            c.w64(1, c.r64(0));
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d85b8); };
            c.w64(11, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d85bc); };
            c.w64(12, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d85c0); };
            c.w64(15, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d85c4); };
            c.w64(8, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d85c8); };
            c.w64(7, t198);
            blk = 136; continue;
        }
        195 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d85d1); }
            blk = 199; continue;
        }
        196 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d85d7); }
            blk = 199; continue;
        }
        197 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x70008d85dd); }
            c.logic(c.r8(1) & 0x10u64, 8);
            if c.cc_ne() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t199) = c.ld64(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d85e6); };
            c.w64(8, t199);
            blk = 163; continue;
        }
        199 => {
            c.cmp(c.r64(12), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w32(1, c.r64(12).wrapping_add(c.r64(12)));
            blk = 210; continue;
        }
        201 => {
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d85fe); };
            c.w64(1, t200);
            let Some(t201) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d8605); };
            c.w64(7, t201);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d860c); };
            c.w64(9, t202);
            let Some(t203) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d8613); };
            c.cmp(t203, c.r64(8), 64);
            if c.cc_a() { blk = 204; continue; }
            blk = 202; continue;
        }
        202 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008d8622, 0x70008d861d);
        }
        203 => {
            c.w64(1, c.r64(0));
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d8625); };
            c.w64(11, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d8629); };
            c.w64(0, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d862d); };
            c.w64(12, t206);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 205; continue;
        }
        204 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008d8636); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 205; continue;
        }
        205 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(12) as i64));
            c.logic(c.r64(12) & c.r64(12), 64);
            if c.cc_ns() { blk = 209; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(10, c.r64(12));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let r207 = c.r64(10) | 0x1u64;
            c.w64(10, r207);
            c.logic(r207, 64);
            blk = 208; continue;
        }
        208 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 209; continue;
        }
        209 => {
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d8664); };
            c.w64(7, t208);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008d866b); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d866e); }
            blk = 210; continue;
        }
        210 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d8673); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008d8674); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d8675); };
            c.w64(3, t209);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x70008d8689, 0x70008d8684);
        }
        211 => {
            let r210 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r210);
            c.w64(1, c.r64(0));
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d868e); };
            c.w64(11, t211);
            blk = 166; continue;
        }
        212 => {
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8697); };
            c.w64(7, t212);
            let Some(t213) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008d869e); };
            c.w64(8, t213);
            c.w64(9, c.r64(8).wrapping_add(0x10));
            let r214 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r214);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d86a9); }
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d86ad); };
            c.w64(12, t215);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008d86b4); }
            let Some(t216) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d86b8); };
            c.cmp(t216, c.r64(9), 64);
            if c.cc_a() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d86c8, 0x70008d86c3);
        }
        214 => {
            blk = 216; continue;
        }
        215 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008d86ca); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 216; continue;
        }
        216 => {
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d86d4); };
            c.w64(1, t217);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d86db); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d86de); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d86e5); }
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d86ec); };
            c.w64(1, t218);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(1)) { return RegfileExit::fault(0x70008d86f0); }
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d86f4); };
            c.w64(1, t219);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d86fb); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d86fc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d8702); }
            return c.ret(0x0, 0x70008d8704);
        }
        217 => {
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8705); };
            c.w64(9, t220);
            let Some(t221) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008d870c); };
            c.w64(12, t221);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008d8714); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008d8718); }
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d871c); };
            c.w64(0, t222);
            let Some(t223) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d8723); };
            c.cmp(t223, c.r64(15), 64);
            if c.cc_a() { blk = 220; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d8732, 0x70008d872d);
        }
        219 => {
            blk = 221; continue;
        }
        220 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008d8734); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 221; continue;
        }
        221 => {
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d873f); };
            c.w64(1, t224);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008d8746); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d8749); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d8750); }
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d8757); };
            c.w64(1, t225);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(1)) { return RegfileExit::fault(0x70008d875b); }
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d875e); };
            c.w64(8, t226);
            let Some(t227) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d8765); };
            c.w32(7, t227);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_b() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            return RegfileExit::fault(0x70008d8774);
        }
        223 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d8775); };
            c.w64(7, t228);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008d8779); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r229 = c.r64(1) & c.r64(8);
            c.w64(1, r229);
            c.logic(r229, 64);
            let Some(t230) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d878b); };
            c.logic(t230 & 0x4u64, 8);
            if c.cc_ne() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d8791); };
            c.w64(7, t231);
            c.w64(1, c.r64(7).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d879c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d879d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d87a3); }
            return c.ret(0x0, 0x70008d87a5);
        }
        226 => {
            c.w32(0, c.r32(0));
            let r232 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r232);
            c.w64(1, 0xfffffffffffc0000u64);
            let r233 = c.r64(1) & c.r64(0);
            c.w64(1, r233);
            c.logic(r233, 64);
            let Some(t234) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d87b5); };
            c.logic(t234 & 0x2u64, 8);
            if c.cc_e() { blk = 225; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d87bf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d87c0); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008d87cd, 0x70008d87c8);
        }
        228 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008d87cd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008d87cf); }
            blk = 225; continue;
        }
        229 => {
            return RegfileExit::jump(0x70008d87d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008d87f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008d87f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008d87f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d87f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d87fa); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008d87ff); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 435; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008d8813); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 442; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8822); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8829); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_ae() { blk = 437; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8835); };
            c.w32(8, t6);
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let r7 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r7);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008d8843); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008d8847); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008d884b); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008d8851); }
            c.w64(12, c.r64(7));
            c.w64(15, c.r64(6));
            blk = 5; continue;
        }
        4 => {
            c.w64(12, c.r64(0));
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8860); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            let Some(t10) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8868); };
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
            let Some(t13) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d888f); };
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
            let Some(t15) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d88a1); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d88a9); };
            c.cmp(t17, 0x0u64, 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008d88af); }
            c.w64(0, c.r64(12));
            return c.call(0x70008e88b4, 0x70008d88bb, 0x70008d88b6);
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d88bb); };
            c.w64(15, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d88bf); };
            c.w64(9, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d88c3); };
            c.w64(7, t20);
            blk = 4; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d88c9); };
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
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d88df); };
            c.w32(8, t23);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r24 = c.add(c.r64(15), c.r64(8), 64);
            c.w64(15, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d88ed); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            c.w64(0, c.r64(12));
            blk = 4; continue;
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d88fd); };
            c.w32(8, t27);
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d8902); };
            c.cmp(c.r32(8), t28, 32);
            if c.cc_b() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d890b); };
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
            return RegfileExit::fault(0x70008d891d);
        }
        20 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d891e); };
            c.w32(8, t31);
            let Some(t32) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d8923); };
            c.cmp(c.r32(8), t32, 32);
            if c.cc_b() { blk = 23; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d892c); };
            let r34 = c.sub(c.r64(8), t33, 64);
            c.w64(8, r34);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(15).wrapping_mul(2).wrapping_add(0xc));
            c.w64(15, c.r64(8));
            c.w32(2, 0xbu64);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d8945); };
            c.w64(11, t35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d894c); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d8953); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d895a); };
            c.w64(1, t38);
            blk = 238; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008d8966);
        }
        24 => {
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t39) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d8974); };
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
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d8989); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008d898e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008d8992); }
            c.w64(1, c.r64(12));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1460)) else { return RegfileExit::fault(0x70008d8999); };
            c.w64(0, t42);
            c.w64(10, 0x70008d89a7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d89a7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d89ab); }
            return c.call(c.r64(0), 0x70008d89b1, 0x70008d89af);
        }
        28 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d89b1); }
            let Some(t43) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d89b9); };
            c.w64(4, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d89be); };
            c.w64(15, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d89c2); };
            c.w64(9, t45);
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d89c6); };
            c.w64(7, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d89ca); };
            c.w64(12, t47);
            blk = 29; continue;
        }
        29 => {
            c.w64(1, c.r64(12));
            let Some(t48) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d89d1); };
            c.w32(8, t48);
            let Some(t49) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d89d5); };
            c.cmp(c.r32(8), t49, 32);
            if c.cc_b() { blk = 229; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d89e2); };
            let r51 = c.sub(c.r64(8), t50, 64);
            c.w64(8, r51);
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 229; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(8, c.r64(15).wrapping_add(c.r64(0)).wrapping_add(0x1));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 32; continue;
        }
        32 => {
            let Some(t52) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d89f7); };
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
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d8a12); };
            c.cmp(c.r64(15), t54, 64);
            if c.cc_ae() { blk = 67; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(15)));
            let Some(t55) = c.ld8(c.r64(12).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8a20); };
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
            return RegfileExit::fault(0x70008d8aef);
        }
        68 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 70; continue;
        }
        69 => {
            c.w64(0, c.r64(2));
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            blk = 73; continue;
        }
        72 => {
            let r56 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r56);
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r64(0), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 108; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d8b14); };
            c.cmp(c.r64(0), t57, 64);
            if c.cc_ae() { blk = 106; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(3, c.r64(8).wrapping_add(c.r64(0)));
            let Some(t58) = c.ld8(c.r64(12).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8b22); };
            c.w32(3, t58);
            c.cmp(c.r8(3), 0x20u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w32(1, c.r64(3).wrapping_sub(0xe));
            c.cmp(c.r32(1), 0x92u64, 32);
            if c.cc_b() { blk = 102; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r8(3), 0x9u64, 8);
            if c.cc_b() { blk = 101; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r8(3), 0xdu64, 8);
            if c.cc_be() { blk = 100; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r8(3), 0xffffffffffffffa0u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(3), 0x1680u64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(3), 0x2000u64, 32);
            if c.cc_b() { blk = 97; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(3), 0x200au64, 32);
            if c.cc_be() { blk = 96; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(3), 0x2028u64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(3), 0x2029u64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(3), 0x202fu64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(3), 0x205fu64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(3), 0xfeffu64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(3), 0x3000u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 104; continue;
        }
        90 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        91 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        92 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        93 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        94 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        95 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        96 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        97 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 104; continue;
        }
        98 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        99 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        100 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        101 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 104; continue;
        }
        102 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 104; continue;
        }
        103 => {
            c.w64(3, c.r64(2));
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 72; continue;
        }
        106 => {
            return RegfileExit::fault(0x70008d8bfa);
        }
        107 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 109; continue;
        }
        108 => {
            c.w64(8, c.r64(2));
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 110; continue;
        }
        110 => {
            c.w64(8, c.r64(0).wrapping_add(0x1));
            let Some(t59) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8c11); };
            c.w32(12, t59);
            let r60 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r60);
            c.w64(0, c.r64(8));
            let r61 = c.sub(c.r64(0), c.r64(15), 64);
            c.w64(0, r61);
            let Some(t62) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8c1e); };
            c.w32(12, t62);
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_be() { blk = 223; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_e() { blk = 222; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(12) & 0x7u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_e() { blk = 197; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(7));
            blk = 115; continue;
        }
        115 => {
            c.w64(1, c.r64(3));
            let r63 = c.r32(1) & 0x7u64;
            c.w32(1, r63);
            c.logic(r63, 32);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_l() { blk = 120; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 225; continue;
        }
        120 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 127; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            blk = 225; continue;
        }
        123 => {
            let Some(t64) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d8c91); };
            c.w32(3, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d8c96); };
            c.cmp(t65, c.r32(3), 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t66) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d8ca3); };
            c.w32(12, t66);
            let r67 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r67);
            let Some(t68) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8cab); };
            c.w32(3, t68);
            let r69 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r69);
            let Some(t70) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8cb3); };
            c.w32(3, t70);
            blk = 115; continue;
        }
        125 => {
            let Some(t71) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d8cb9); };
            c.w32(3, t71);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r72 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r72);
            let Some(t73) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d8cc7); };
            c.w32(12, t73);
            let r74 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r74);
            let Some(t75) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8ccf); };
            c.w32(3, t75);
            let r76 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r76);
            let Some(t77) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8cd7); };
            c.w32(3, t77);
            blk = 115; continue;
        }
        126 => {
            let Some(t78) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d8cdd); };
            c.w32(12, t78);
            let r79 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r79);
            let Some(t80) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d8ce5); };
            c.w32(3, t80);
            let r81 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r81);
            let Some(t82) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d8ced); };
            c.w32(3, t82);
            blk = 115; continue;
        }
        127 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 130; continue;
        }
        128 => {
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 130; continue;
        }
        129 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(7));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r64(0), 0xdu64, 64);
            if c.cc_l() { blk = 138; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(11)));
            let r83 = c.r32(12) & 0x8u64;
            c.w32(12, r83);
            c.logic(r83, 32);
            let r84 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r84);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8d30); };
            c.w64(6, t85);
            let Some(t86) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008d8d37); };
            c.w64(8, t86);
            c.w64(11, c.r64(8).wrapping_add(0x14));
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d8d3e); };
            c.w64(1, t87);
            let Some(t88) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d8d45); };
            c.cmp(t88, c.r64(11), 64);
            if c.cc_be() { blk = 444; continue; }
            blk = 133; continue;
        }
        133 => {
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x70008d8d4e); }
            let r89 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r89);
            blk = 134; continue;
        }
        134 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d8d55); };
            c.w64(11, t90);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d8d5c); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d8d60); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008d8d68); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d8d6c); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008d8d70); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8d7a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8d7b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d8d81); }
            return c.ret(0x0, 0x70008d8d83);
        }
        135 => {
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8d84); };
            c.w64(6, t91);
            let Some(t92) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008d8d8b); };
            c.w64(8, t92);
            c.w64(11, c.r64(8).wrapping_add(0x14));
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d8d92); };
            c.w64(1, t93);
            let Some(t94) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d8d99); };
            c.cmp(t94, c.r64(11), 64);
            if c.cc_be() { blk = 446; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.st64(c.r64(6), c.r64(11)) { return RegfileExit::fault(0x70008d8da2); }
            let r95 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r95);
            blk = 137; continue;
        }
        137 => {
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d8da9); };
            c.w64(12, t96);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008d8db0); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d8db4); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008d8dbc); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008d8dc0); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008d8dc4); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8dce); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8dcf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d8dd5); }
            return c.ret(0x0, 0x70008d8dd7);
        }
        138 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 168; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w32(7, c.r32(0));
            let r97 = c.r32(12) & 0x8u64;
            c.w32(12, r97);
            c.logic(r97, 32);
            c.cmp(c.r32(7), 0x0u64, 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 149; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w32(7, c.r32(0));
            c.w64(8, c.r64(7));
            let r98 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r98);
            if c.cc_o() { blk = 148; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r99 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r99);
            if c.cc_o() { blk = 147; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8e18); };
            c.w64(1, t100);
            let Some(t101) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d8e1f); };
            c.w64(8, t101);
            let r102 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r102);
            let r103 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r103);
            c.logic(r103, 64);
            c.w64(12, c.r64(8).wrapping_add(c.r64(7)));
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d8e2e); };
            c.w64(2, t104);
            let Some(t105) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008d8e35); };
            c.cmp(t105, c.r64(12), 64);
            if c.cc_be() { blk = 448; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 448; continue; }
            blk = 145; continue;
        }
        145 => {
            if !c.st64(c.r64(1), c.r64(12)) { return RegfileExit::fault(0x70008d8e4b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 146; continue;
        }
        146 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d8e52); };
            c.w64(8, t106);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008d8e59); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d8e5d); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008d8e64); }
            c.w64(8, c.r64(7));
            blk = 150; continue;
        }
        147 => {
            return RegfileExit::fault(0x70008d8e6c);
        }
        148 => {
            return RegfileExit::fault(0x70008d8e6d);
        }
        149 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d8e6e); };
            c.w64(8, t107);
            blk = 150; continue;
        }
        150 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(11)));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(11, c.r64(7).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 154; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w32(12, 0xbu64);
            blk = 153; continue;
        }
        152 => {
            let r108 = c.add(c.r64(12), 0x2u64, 64);
            c.w64(12, r108);
            c.w64(7, c.r64(15));
            blk = 153; continue;
        }
        153 => {
            c.w64(15, c.r64(7).wrapping_add(0x2));
            let Some(t109) = c.ld16(c.r64(3).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008d8e9d); };
            c.w32(7, t109);
            if !c.st16(c.r64(8).wrapping_add(c.r64(12)), c.r16(7)) { return RegfileExit::fault(0x70008d8ea1); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 152; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8eb1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8eb2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d8eb8); }
            return c.ret(0x0, 0x70008d8eba);
        }
        155 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 162; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r110 = c.add(c.r64(7), c.r64(6), 64);
            c.w64(7, r110);
            if c.cc_o() { blk = 161; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8ecd); };
            c.w64(1, t111);
            let Some(t112) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008d8ed4); };
            c.w64(8, t112);
            let r113 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r113);
            let r114 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r114);
            c.logic(r114, 64);
            c.w64(12, c.r64(8).wrapping_add(c.r64(7)));
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d8ee3); };
            c.w64(2, t115);
            let Some(t116) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008d8eea); };
            c.cmp(t116, c.r64(12), 64);
            if c.cc_be() { blk = 450; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 450; continue; }
            blk = 159; continue;
        }
        159 => {
            if !c.st64(c.r64(1), c.r64(12)) { return RegfileExit::fault(0x70008d8f00); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 160; continue;
        }
        160 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d8f07); };
            c.w64(8, t117);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008d8f0e); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d8f12); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008d8f19); }
            c.w64(8, c.r64(7));
            blk = 163; continue;
        }
        161 => {
            return RegfileExit::fault(0x70008d8f21);
        }
        162 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d8f22); };
            c.w64(8, t118);
            blk = 163; continue;
        }
        163 => {
            c.w64(7, c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xb));
            c.w64(11, c.r64(7).wrapping_add(c.r64(0)));
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 167; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w32(12, 0xbu64);
            blk = 166; continue;
        }
        165 => {
            let r119 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r119);
            c.w64(7, c.r64(15));
            blk = 166; continue;
        }
        166 => {
            c.w64(15, c.r64(7).wrapping_add(0x1));
            let Some(t120) = c.ld8(c.r64(3).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008d8f4a); };
            c.w32(7, t120);
            if !c.st8(c.r64(8).wrapping_add(c.r64(12)), c.r8(7)) { return RegfileExit::fault(0x70008d8f4e); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 165; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d8f5d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d8f5e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d8f64); }
            return c.ret(0x0, 0x70008d8f66);
        }
        168 => {
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d8f67); };
            c.w64(2, t121);
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d8f6e); };
            c.w64(7, t122);
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d8f75); };
            c.w64(6, t123);
            c.w32(1, 0xbu64);
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 196; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w32(8, c.r32(0));
            let Some(t124) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d8f8e); };
            c.w64(3, t124);
            let r125 = c.r32(12) & 0x8u64;
            c.w32(12, r125);
            c.logic(r125, 32);
            c.cmp(c.r32(8), 0x0u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w32(7, c.r32(0));
            c.w64(8, c.r64(7));
            let r126 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r126);
            if c.cc_o() { blk = 178; continue; }
            blk = 172; continue;
        }
        172 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(12, 0xcu64);
            let r127 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r127);
            if c.cc_o() { blk = 177; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d8fca); };
            c.w64(8, t128);
            let Some(t129) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d8fd1); };
            c.w64(12, t129);
            let r130 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r130);
            let r131 = c.r64(7) & 0xfffffffffffffffcu64;
            c.w64(7, r131);
            c.logic(r131, 64);
            c.w64(9, c.r64(12).wrapping_add(c.r64(7)));
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d8fe0); };
            c.w64(2, t132);
            let Some(t133) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008d8fe7); };
            c.cmp(t133, c.r64(9), 64);
            if c.cc_be() { blk = 452; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r64(7), 0x20000u64, 64);
            if c.cc_ae() { blk = 452; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.st64(c.r64(8), c.r64(9)) { return RegfileExit::fault(0x70008d8ffd); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            blk = 176; continue;
        }
        176 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008d9005); }
            if !c.st32(c.r64(7).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d9008); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008d900f); }
            c.w64(2, c.r64(7));
            blk = 179; continue;
        }
        177 => {
            return RegfileExit::fault(0x70008d9017);
        }
        178 => {
            return RegfileExit::fault(0x70008d9018);
        }
        179 => {
            c.w64(7, c.r64(15).wrapping_add(c.r64(11)));
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 183; continue; }
            blk = 180; continue;
        }
        180 => {
            blk = 182; continue;
        }
        181 => {
            let r134 = c.add(c.r64(1), 0x2u64, 64);
            c.w64(1, r134);
            c.w64(7, c.r64(9));
            blk = 182; continue;
        }
        182 => {
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t135) = c.ld16(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d903b); };
            c.w32(7, t135);
            if !c.st16(c.r64(2).wrapping_add(c.r64(1)), c.r16(7)) { return RegfileExit::fault(0x70008d9040); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 181; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(0, c.r64(2));
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d904c); };
            c.w64(9, t136);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9053); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9054); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d905a); }
            return c.ret(0x0, 0x70008d905c);
        }
        184 => {
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d905d); };
            c.w64(12, t137);
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d9064); };
            c.w64(6, t138);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 191; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(8, 0xcu64);
            let r139 = c.add(c.r64(2), c.r64(8), 64);
            c.w64(2, r139);
            if c.cc_o() { blk = 190; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t140) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008d907f); };
            c.w64(8, t140);
            let r141 = c.add(c.r64(2), 0x3u64, 64);
            c.w64(2, r141);
            let r142 = c.r64(2) & 0xfffffffffffffffcu64;
            c.w64(2, r142);
            c.logic(r142, 64);
            c.w64(9, c.r64(8).wrapping_add(c.r64(2)));
            let Some(t143) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008d908f); };
            c.cmp(t143, c.r64(9), 64);
            if c.cc_be() { blk = 454; continue; }
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 454; continue; }
            blk = 188; continue;
        }
        188 => {
            if !c.st64(c.r64(12), c.r64(9)) { return RegfileExit::fault(0x70008d90a5); }
            let r144 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r144);
            blk = 189; continue;
        }
        189 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008d90ad); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d90b1); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008d90b9); }
            c.w64(2, c.r64(8));
            blk = 191; continue;
        }
        190 => {
            return RegfileExit::fault(0x70008d90c2);
        }
        191 => {
            c.w64(7, c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(0)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 195; continue; }
            blk = 192; continue;
        }
        192 => {
            blk = 194; continue;
        }
        193 => {
            let r145 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r145);
            c.w64(7, c.r64(9));
            blk = 194; continue;
        }
        194 => {
            c.w64(9, c.r64(7).wrapping_add(0x1));
            let Some(t146) = c.ld8(c.r64(7).wrapping_add(c.r64(3)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d90de); };
            c.w32(7, t146);
            if !c.st8(c.r64(2).wrapping_add(c.r64(1)), c.r8(7)) { return RegfileExit::fault(0x70008d90e3); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 193; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(0, c.r64(2));
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d90ef); };
            c.w64(9, t147);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d90f6); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d90f7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d90fd); }
            return c.ret(0x0, 0x70008d90ff);
        }
        196 => {
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9100); };
            c.w64(7, t148);
            blk = 225; continue;
        }
        197 => {
            c.w32(12, c.r16(12));
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d910d); };
            c.w64(8, t149);
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d9114); };
            c.w64(0, t150);
            c.w32(6, 0xcu64);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 213; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(7));
            blk = 199; continue;
        }
        199 => {
            c.w64(1, c.r64(3));
            let r151 = c.r32(1) & 0x7u64;
            c.w32(1, r151);
            c.logic(r151, 32);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_l() { blk = 204; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 203; continue;
        }
        203 => {
            blk = 456; continue;
        }
        204 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 456; continue;
        }
        207 => {
            let Some(t152) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d9165); };
            c.w32(3, t152);
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d916a); };
            c.cmp(t153, c.r32(3), 32);
            if c.cc_ne() { blk = 457; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t154) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d9177); };
            c.w32(12, t154);
            let r155 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r155);
            let Some(t156) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d917f); };
            c.w32(3, t156);
            let r157 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r157);
            let Some(t158) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9187); };
            c.w32(3, t158);
            blk = 199; continue;
        }
        209 => {
            let Some(t159) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d918d); };
            c.w32(3, t159);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r160 = c.add(c.r64(11), c.r64(3), 64);
            c.w64(11, r160);
            let Some(t161) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d919b); };
            c.w32(12, t161);
            let r162 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r162);
            let Some(t163) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d91a3); };
            c.w32(3, t163);
            let r164 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r164);
            let Some(t165) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d91ab); };
            c.w32(3, t165);
            blk = 199; continue;
        }
        210 => {
            let Some(t166) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d91b1); };
            c.w32(12, t166);
            let r167 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r167);
            let Some(t168) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d91b9); };
            c.w32(3, t168);
            let r169 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r169);
            let Some(t170) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d91c1); };
            c.w32(3, t170);
            blk = 199; continue;
        }
        211 => {
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 214; continue;
        }
        212 => {
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 214; continue;
        }
        213 => {
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(7));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 458; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w64(7, c.r64(3).wrapping_add(0xb));
            blk = 216; continue;
        }
        216 => {
            let r171 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r171);
            let r172 = c.r32(12) & 0x8u64;
            c.w32(12, r172);
            c.logic(r172, 32);
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t173) = c.ld16(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d9204); };
            c.w32(7, t173);
            blk = 219; continue;
        }
        218 => {
            let Some(t174) = c.ld8(c.r64(11).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008d920b); };
            c.w32(7, t174);
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 473; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d9224); };
            c.w64(12, t175);
            let Some(t176) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d922b); };
            c.w32(15, t176);
            let r177 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r177);
            let Some(t178) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d9233); };
            c.cmp(t178, c.r32(15), 32);
            if c.cc_e() { blk = 478; continue; }
            blk = 221; continue;
        }
        221 => {
            c.w64(0, c.r64(15));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9246); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9247); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d924d); }
            return c.ret(0x0, 0x70008d924f);
        }
        222 => {
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d9250); };
            c.w64(0, t179);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d925a); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d925b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9261); }
            return c.ret(0x0, 0x70008d9263);
        }
        223 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9270); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9271); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9277); }
            return c.ret(0x0, 0x70008d9279);
        }
        225 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15)));
            let r180 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r180);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d9281); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d9282); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d9284); }
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008d9286); };
            c.w64(3, t181);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d9299, 0x70008d9294);
        }
        226 => {
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9299); };
            c.w64(7, t182);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d92a0); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d92a1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d92a7); }
            return c.ret(0x0, 0x70008d92a9);
        }
        227 => {
            let Some(t183) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d92aa); };
            c.w64(0, t183);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d92b4); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d92b5); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d92bb); }
            return c.ret(0x0, 0x70008d92bd);
        }
        228 => {
            let Some(t184) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d92be); };
            c.w64(0, t184);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d92c8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d92c9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d92cf); }
            return c.ret(0x0, 0x70008d92d1);
        }
        229 => {
            return RegfileExit::fault(0x70008d92d2);
        }
        230 => {
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d92d3); };
            c.w64(11, t185);
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d92da); };
            c.w64(8, t186);
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d92e1); };
            c.w64(6, t187);
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d92e8); };
            c.w64(1, t188);
            c.w32(2, 0xbu64);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_ne() { blk = 434; continue; }
            blk = 231; continue;
        }
        231 => {
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t189) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d9301); };
            c.w64(0, t189);
            blk = 235; continue;
        }
        233 => {
            c.w64(10, c.r64(4));
            let r190 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r190);
            let r191 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r191);
            c.logic(r191, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008d9313); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x70008d9318); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008d931c); }
            c.w64(1, c.r64(12));
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x1468)) else { return RegfileExit::fault(0x70008d9323); };
            c.w64(0, t192);
            c.w64(10, 0x70008d9331u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008d9331); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008d9335); }
            return c.call(c.r64(0), 0x70008d933b, 0x70008d9339);
        }
        234 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008d933b); }
            let Some(t193) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008d9343); };
            c.w64(4, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d9348); };
            c.w64(15, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d934c); };
            c.w64(9, t195);
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d9350); };
            c.w64(11, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9357); };
            c.w64(7, t197);
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d935b); };
            c.w64(8, t198);
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d9362); };
            c.w64(6, t199);
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d9369); };
            c.w64(1, t200);
            c.w32(2, 0xbu64);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d9375); };
            c.w64(12, t201);
            blk = 235; continue;
        }
        235 => {
            let Some(t202) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9379); };
            c.w32(12, t202);
            let Some(t203) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d937e); };
            c.cmp(c.r32(12), t203, 32);
            if c.cc_b() { blk = 433; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d938b); };
            let r205 = c.sub(c.r64(12), t204, 64);
            c.w64(12, r205);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_b() { blk = 433; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w64(12, c.r64(0).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x1));
            c.w64(15, c.r64(12));
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 238; continue;
        }
        238 => {
            c.w64(0, c.r64(7));
            let Some(t206) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d93a6); };
            c.w32(7, t206);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 240; continue;
        }
        239 => {
            let r207 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r207);
            blk = 240; continue;
        }
        240 => {
            c.cmp(c.r64(3), c.r64(7), 64);
            if c.cc_e() { blk = 275; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d93ba); };
            c.cmp(c.r64(3), t208, 64);
            if c.cc_ae() { blk = 273; continue; }
            blk = 242; continue;
        }
        242 => {
            c.w64(6, c.r64(15).wrapping_add(c.r64(3).wrapping_mul(2)));
            let Some(t209) = c.ld16(c.r64(12).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d93c8); };
            c.w32(6, t209);
            c.cmp(c.r16(6), 0x20u64, 16);
            if c.cc_e() { blk = 270; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w32(1, c.r64(6).wrapping_sub(0xe));
            c.cmp(c.r32(1), 0x92u64, 32);
            if c.cc_b() { blk = 269; continue; }
            blk = 244; continue;
        }
        244 => {
            c.cmp(c.r16(6), 0x9u64, 16);
            if c.cc_b() { blk = 268; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r16(6), 0xdu64, 16);
            if c.cc_be() { blk = 267; continue; }
            blk = 246; continue;
        }
        246 => {
            c.cmp(c.r16(6), 0xa0u64, 16);
            if c.cc_e() { blk = 266; continue; }
            blk = 247; continue;
        }
        247 => {
            c.cmp(c.r16(6), 0x1680u64, 16);
            if c.cc_e() { blk = 265; continue; }
            blk = 248; continue;
        }
        248 => {
            c.cmp(c.r16(6), 0x2000u64, 16);
            if c.cc_b() { blk = 264; continue; }
            blk = 249; continue;
        }
        249 => {
            c.cmp(c.r16(6), 0x200au64, 16);
            if c.cc_be() { blk = 263; continue; }
            blk = 250; continue;
        }
        250 => {
            c.cmp(c.r16(6), 0x2028u64, 16);
            if c.cc_e() { blk = 262; continue; }
            blk = 251; continue;
        }
        251 => {
            c.cmp(c.r16(6), 0x2029u64, 16);
            if c.cc_e() { blk = 261; continue; }
            blk = 252; continue;
        }
        252 => {
            c.cmp(c.r16(6), 0x202fu64, 16);
            if c.cc_e() { blk = 260; continue; }
            blk = 253; continue;
        }
        253 => {
            c.cmp(c.r16(6), 0x205fu64, 16);
            if c.cc_e() { blk = 259; continue; }
            blk = 254; continue;
        }
        254 => {
            c.cmp(c.r16(6), 0xfeffu64, 16);
            if c.cc_e() { blk = 258; continue; }
            blk = 255; continue;
        }
        255 => {
            c.cmp(c.r16(6), 0x3000u64, 16);
            if c.cc_e() { blk = 257; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 271; continue;
        }
        257 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        258 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        259 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        260 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        261 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        262 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        263 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        264 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 271; continue;
        }
        265 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        266 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        267 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        268 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 271; continue;
        }
        269 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 271; continue;
        }
        270 => {
            c.w32(1, 0x1u64);
            blk = 271; continue;
        }
        271 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 274; continue; }
            blk = 272; continue;
        }
        272 => {
            blk = 239; continue;
        }
        273 => {
            return RegfileExit::fault(0x70008d94b6);
        }
        274 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 276; continue;
        }
        275 => {
            c.w32(1, 0x1u64);
            blk = 276; continue;
        }
        276 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 432; continue; }
            blk = 277; continue;
        }
        277 => {
            c.w64(1, c.r64(7).wrapping_sub(0x1));
            blk = 279; continue;
        }
        278 => {
            let r210 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r210);
            blk = 279; continue;
        }
        279 => {
            c.cmp(c.r64(1), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 314; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d94dd); };
            c.cmp(c.r64(1), t211, 64);
            if c.cc_ae() { blk = 312; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w64(6, c.r64(15).wrapping_add(c.r64(1).wrapping_mul(2)));
            let Some(t212) = c.ld16(c.r64(12).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d94eb); };
            c.w32(6, t212);
            c.cmp(c.r16(6), 0x20u64, 16);
            if c.cc_e() { blk = 309; continue; }
            blk = 282; continue;
        }
        282 => {
            c.w32(2, c.r64(6).wrapping_sub(0xe));
            c.cmp(c.r32(2), 0x92u64, 32);
            if c.cc_b() { blk = 308; continue; }
            blk = 283; continue;
        }
        283 => {
            c.cmp(c.r16(6), 0x9u64, 16);
            if c.cc_b() { blk = 307; continue; }
            blk = 284; continue;
        }
        284 => {
            c.cmp(c.r16(6), 0xdu64, 16);
            if c.cc_be() { blk = 306; continue; }
            blk = 285; continue;
        }
        285 => {
            c.cmp(c.r16(6), 0xa0u64, 16);
            if c.cc_e() { blk = 305; continue; }
            blk = 286; continue;
        }
        286 => {
            c.cmp(c.r16(6), 0x1680u64, 16);
            if c.cc_e() { blk = 304; continue; }
            blk = 287; continue;
        }
        287 => {
            c.cmp(c.r16(6), 0x2000u64, 16);
            if c.cc_b() { blk = 303; continue; }
            blk = 288; continue;
        }
        288 => {
            c.cmp(c.r16(6), 0x200au64, 16);
            if c.cc_be() { blk = 302; continue; }
            blk = 289; continue;
        }
        289 => {
            c.cmp(c.r16(6), 0x2028u64, 16);
            if c.cc_e() { blk = 301; continue; }
            blk = 290; continue;
        }
        290 => {
            c.cmp(c.r16(6), 0x2029u64, 16);
            if c.cc_e() { blk = 300; continue; }
            blk = 291; continue;
        }
        291 => {
            c.cmp(c.r16(6), 0x202fu64, 16);
            if c.cc_e() { blk = 299; continue; }
            blk = 292; continue;
        }
        292 => {
            c.cmp(c.r16(6), 0x205fu64, 16);
            if c.cc_e() { blk = 298; continue; }
            blk = 293; continue;
        }
        293 => {
            c.cmp(c.r16(6), 0xfeffu64, 16);
            if c.cc_e() { blk = 297; continue; }
            blk = 294; continue;
        }
        294 => {
            c.cmp(c.r16(6), 0x3000u64, 16);
            if c.cc_e() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 310; continue;
        }
        296 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        297 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        298 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        299 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        300 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        301 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        302 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        303 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 310; continue;
        }
        304 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        305 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        306 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        307 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 310; continue;
        }
        308 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 310; continue;
        }
        309 => {
            c.w32(2, 0x1u64);
            blk = 310; continue;
        }
        310 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 311; continue;
        }
        311 => {
            blk = 278; continue;
        }
        312 => {
            return RegfileExit::fault(0x70008d95d9);
        }
        313 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 315; continue;
        }
        314 => {
            c.w32(12, 0x1u64);
            blk = 315; continue;
        }
        315 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 431; continue; }
            blk = 316; continue;
        }
        316 => {
            c.w64(12, c.r64(1).wrapping_add(0x1));
            let Some(t213) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d95f3); };
            c.w32(15, t213);
            let r214 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r214);
            c.w64(2, c.r64(12));
            let r215 = c.sub(c.r64(2), c.r64(3), 64);
            c.w64(2, r215);
            let Some(t216) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9600); };
            c.w32(15, t216);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008d9605); }
            c.cmp(c.r64(7), c.r64(2), 64);
            if c.cc_be() { blk = 427; continue; }
            blk = 317; continue;
        }
        317 => {
            c.cmp(c.r64(12), c.r64(3), 64);
            if c.cc_e() { blk = 426; continue; }
            blk = 318; continue;
        }
        318 => {
            c.logic(c.r8(15) & 0x7u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_e() { blk = 401; continue; }
            blk = 319; continue;
        }
        319 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 335; continue; }
            blk = 320; continue;
        }
        320 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(15));
            c.w64(15, c.r64(0));
            blk = 321; continue;
        }
        321 => {
            c.w64(6, c.r64(1));
            let r217 = c.r32(6) & 0x7u64;
            c.w32(6, r217);
            c.logic(r217, 32);
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_l() { blk = 326; continue; }
            blk = 322; continue;
        }
        322 => {
            c.cmp(c.r32(6), 0x2u64, 32);
            if c.cc_e() { blk = 334; continue; }
            blk = 323; continue;
        }
        323 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 331; continue; }
            blk = 324; continue;
        }
        324 => {
            c.cmp(c.r32(6), 0x5u64, 32);
            if c.cc_e() { blk = 332; continue; }
            blk = 325; continue;
        }
        325 => {
            blk = 429; continue;
        }
        326 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 333; continue; }
            blk = 327; continue;
        }
        327 => {
            c.cmp(c.r32(6), 0x1u64, 32);
            if c.cc_e() { blk = 329; continue; }
            blk = 328; continue;
        }
        328 => {
            blk = 429; continue;
        }
        329 => {
            let Some(t218) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d9674); };
            c.w32(1, t218);
            let Some(t219) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d9678); };
            c.cmp(t219, c.r32(1), 32);
            if c.cc_ne() { blk = 429; continue; }
            blk = 330; continue;
        }
        330 => {
            let Some(t220) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d9685); };
            c.w32(15, t220);
            let r221 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r221);
            let Some(t222) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d968c); };
            c.w32(1, t222);
            let r223 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r223);
            let Some(t224) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9693); };
            c.w32(1, t224);
            blk = 321; continue;
        }
        331 => {
            let Some(t225) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d9699); };
            c.w32(1, t225);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r226 = c.add(c.r64(7), c.r64(1), 64);
            c.w64(7, r226);
            let Some(t227) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d96a6); };
            c.w32(15, t227);
            let r228 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r228);
            let Some(t229) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d96ad); };
            c.w32(1, t229);
            let r230 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r230);
            let Some(t231) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d96b4); };
            c.w32(1, t231);
            blk = 321; continue;
        }
        332 => {
            let Some(t232) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d96ba); };
            c.w32(15, t232);
            let r233 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r233);
            let Some(t234) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d96c1); };
            c.w32(1, t234);
            let r235 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r235);
            let Some(t236) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d96c8); };
            c.w32(1, t236);
            blk = 321; continue;
        }
        333 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 336; continue;
        }
        334 => {
            c.w32(6, 0x1u64);
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(15));
            c.w64(15, c.r64(10));
            blk = 336; continue;
        }
        335 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 336; continue;
        }
        336 => {
            c.cmp(c.r64(2), 0xdu64, 64);
            if c.cc_l() { blk = 344; continue; }
            blk = 337; continue;
        }
        337 => {
            let r237 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r237);
            let r238 = c.r32(15) & 0x8u64;
            c.w32(15, r238);
            c.logic(r238, 32);
            let r239 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r239);
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 341; continue; }
            blk = 338; continue;
        }
        338 => {
            let Some(t240) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d970e); };
            c.w64(11, t240);
            let Some(t241) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008d9715); };
            c.w64(12, t241);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t242) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d971d); };
            c.cmp(t242, c.r64(15), 64);
            if c.cc_be() { blk = 487; continue; }
            blk = 339; continue;
        }
        339 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008d9726); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 340; continue;
        }
        340 => {
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x310)) else { return RegfileExit::fault(0x70008d972e); };
            c.w64(11, t243);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d9735); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d9739); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d9741); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008d9745); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008d9749); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9753); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9754); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d975a); }
            return c.ret(0x0, 0x70008d975c);
        }
        341 => {
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d975d); };
            c.w64(11, t244);
            let Some(t245) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008d9764); };
            c.w64(12, t245);
            c.w64(15, c.r64(12).wrapping_add(0x14));
            let Some(t246) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d976c); };
            c.cmp(t246, c.r64(15), 64);
            if c.cc_be() { blk = 489; continue; }
            blk = 342; continue;
        }
        342 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008d9775); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 343; continue;
        }
        343 => {
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x318)) else { return RegfileExit::fault(0x70008d977d); };
            c.w64(11, t247);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d9784); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d9788); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d9790); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008d9794); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(7)) { return RegfileExit::fault(0x70008d9798); }
            c.w64(0, c.r64(8));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d97a2); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d97a3); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d97a9); }
            return c.ret(0x0, 0x70008d97ab);
        }
        344 => {
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d97ac); };
            c.w64(0, t248);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 372; continue; }
            blk = 345; continue;
        }
        345 => {
            c.w32(12, c.r32(2));
            let r249 = c.r32(15) & 0x8u64;
            c.w32(15, r249);
            c.logic(r249, 32);
            c.cmp(c.r32(12), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 360; continue; }
            blk = 346; continue;
        }
        346 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 355; continue; }
            blk = 347; continue;
        }
        347 => {
            c.w32(11, c.r32(2));
            c.w64(12, c.r64(11));
            let r250 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r250);
            if c.cc_o() { blk = 354; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r251 = c.add(c.r64(11), c.r64(6), 64);
            c.w64(11, r251);
            if c.cc_o() { blk = 353; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t252) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d97f7); };
            c.w64(12, t252);
            let r253 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r253);
            let r254 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r254);
            c.logic(r254, 64);
            c.w64(15, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t255) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d9806); };
            c.cmp(t255, c.r64(15), 64);
            if c.cc_be() { blk = 491; continue; }
            blk = 350; continue;
        }
        350 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 491; continue; }
            blk = 351; continue;
        }
        351 => {
            if !c.st64(c.r64(0), c.r64(15)) { return RegfileExit::fault(0x70008d981c); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 352; continue;
        }
        352 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d9824); };
            c.w64(11, t256);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d982b); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d982f); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d9837); }
            c.w64(11, c.r64(8));
            blk = 355; continue;
        }
        353 => {
            return RegfileExit::fault(0x70008d9840);
        }
        354 => {
            return RegfileExit::fault(0x70008d9841);
        }
        355 => {
            let r257 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r257);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 359; continue; }
            blk = 356; continue;
        }
        356 => {
            c.w32(2, 0xbu64);
            blk = 358; continue;
        }
        357 => {
            let r258 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r258);
            c.w64(7, c.r64(12));
            blk = 358; continue;
        }
        358 => {
            c.w64(12, c.r64(7).wrapping_add(0x2));
            let Some(t259) = c.ld16(c.r64(1).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008d9868); };
            c.w32(7, t259);
            if !c.st16(c.r64(11).wrapping_add(c.r64(2)), c.r16(7)) { return RegfileExit::fault(0x70008d986c); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 357; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d987c); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d987d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9883); }
            return c.ret(0x0, 0x70008d9885);
        }
        360 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 367; continue; }
            blk = 361; continue;
        }
        361 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(6, 0xcu64);
            let r260 = c.add(c.r64(11), c.r64(6), 64);
            c.w64(11, r260);
            if c.cc_o() { blk = 366; continue; }
            blk = 362; continue;
        }
        362 => {
            let Some(t261) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d9899); };
            c.w64(12, t261);
            let r262 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r262);
            let r263 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r263);
            c.logic(r263, 64);
            c.w64(15, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t264) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d98a8); };
            c.cmp(t264, c.r64(15), 64);
            if c.cc_be() { blk = 493; continue; }
            blk = 363; continue;
        }
        363 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 493; continue; }
            blk = 364; continue;
        }
        364 => {
            if !c.st64(c.r64(0), c.r64(15)) { return RegfileExit::fault(0x70008d98be); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 365; continue;
        }
        365 => {
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d98c6); };
            c.w64(11, t265);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d98cd); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d98d1); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d98d9); }
            c.w64(11, c.r64(8));
            blk = 367; continue;
        }
        366 => {
            return RegfileExit::fault(0x70008d98e2);
        }
        367 => {
            c.w64(7, c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 371; continue; }
            blk = 368; continue;
        }
        368 => {
            c.w32(2, 0xbu64);
            blk = 370; continue;
        }
        369 => {
            let r266 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r266);
            c.w64(7, c.r64(12));
            blk = 370; continue;
        }
        370 => {
            c.w64(12, c.r64(7).wrapping_add(0x1));
            let Some(t267) = c.ld8(c.r64(1).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008d9903); };
            c.w32(7, t267);
            if !c.st8(c.r64(11).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008d9907); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 369; continue; }
            blk = 371; continue;
        }
        371 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9916); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9917); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d991d); }
            return c.ret(0x0, 0x70008d991f);
        }
        372 => {
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d9920); };
            c.w64(12, t268);
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d9927); };
            c.w64(6, t269);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 400; continue; }
            blk = 373; continue;
        }
        373 => {
            c.w32(12, c.r32(2));
            let Some(t270) = c.ld64(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d993b); };
            c.w64(1, t270);
            let r271 = c.r32(15) & 0x8u64;
            c.w32(15, r271);
            c.logic(r271, 32);
            c.cmp(c.r32(12), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 388; continue; }
            blk = 374; continue;
        }
        374 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 383; continue; }
            blk = 375; continue;
        }
        375 => {
            c.w32(11, c.r32(2));
            c.w64(12, c.r64(11));
            let r272 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r272);
            if c.cc_o() { blk = 382; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w32(15, 0xcu64);
            let r273 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r273);
            if c.cc_o() { blk = 381; continue; }
            blk = 377; continue;
        }
        377 => {
            let Some(t274) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d9978); };
            c.w64(12, t274);
            let r275 = c.add(c.r64(11), 0x3u64, 64);
            c.w64(11, r275);
            let r276 = c.r64(11) & 0xfffffffffffffffcu64;
            c.w64(11, r276);
            c.logic(r276, 64);
            c.w64(15, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t277) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d9987); };
            c.cmp(t277, c.r64(15), 64);
            if c.cc_be() { blk = 495; continue; }
            blk = 378; continue;
        }
        378 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 495; continue; }
            blk = 379; continue;
        }
        379 => {
            if !c.st64(c.r64(0), c.r64(15)) { return RegfileExit::fault(0x70008d999d); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 380; continue;
        }
        380 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008d99a5); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d99a9); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d99b1); }
            c.w64(11, c.r64(8));
            blk = 383; continue;
        }
        381 => {
            return RegfileExit::fault(0x70008d99ba);
        }
        382 => {
            return RegfileExit::fault(0x70008d99bb);
        }
        383 => {
            let r278 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r278);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 387; continue; }
            blk = 384; continue;
        }
        384 => {
            c.w32(2, 0xbu64);
            blk = 386; continue;
        }
        385 => {
            let r279 = c.add(c.r64(2), 0x2u64, 64);
            c.w64(2, r279);
            c.w64(7, c.r64(12));
            blk = 386; continue;
        }
        386 => {
            c.w64(12, c.r64(7).wrapping_add(0x2));
            let Some(t280) = c.ld16(c.r64(7).wrapping_add(c.r64(1)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d99e2); };
            c.w32(7, t280);
            if !c.st16(c.r64(11).wrapping_add(c.r64(2)), c.r16(7)) { return RegfileExit::fault(0x70008d99e7); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 385; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d99f7); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d99f8); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d99fe); }
            return c.ret(0x0, 0x70008d9a00);
        }
        388 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 395; continue; }
            blk = 389; continue;
        }
        389 => {
            c.w64(12, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(11, 0xcu64);
            let r281 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r281);
            if c.cc_o() { blk = 394; continue; }
            blk = 390; continue;
        }
        390 => {
            let Some(t282) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d9a15); };
            c.w64(11, t282);
            let r283 = c.add(c.r64(12), 0x3u64, 64);
            c.w64(12, r283);
            let r284 = c.r64(12) & 0xfffffffffffffffcu64;
            c.w64(12, r284);
            c.logic(r284, 64);
            c.w64(15, c.r64(11).wrapping_add(c.r64(12)));
            let Some(t285) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d9a24); };
            c.cmp(t285, c.r64(15), 64);
            if c.cc_be() { blk = 497; continue; }
            blk = 391; continue;
        }
        391 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 497; continue; }
            blk = 392; continue;
        }
        392 => {
            if !c.st64(c.r64(0), c.r64(15)) { return RegfileExit::fault(0x70008d9a3a); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 393; continue;
        }
        393 => {
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d9a41); };
            c.w64(11, t286);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008d9a48); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d9a4c); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008d9a54); }
            c.w64(11, c.r64(8));
            blk = 395; continue;
        }
        394 => {
            return RegfileExit::fault(0x70008d9a5d);
        }
        395 => {
            c.w64(7, c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0xb));
            c.w64(8, c.r64(7).wrapping_add(c.r64(2)));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 399; continue; }
            blk = 396; continue;
        }
        396 => {
            c.w32(2, 0xbu64);
            blk = 398; continue;
        }
        397 => {
            let r287 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r287);
            c.w64(7, c.r64(12));
            blk = 398; continue;
        }
        398 => {
            c.w64(12, c.r64(7).wrapping_add(0x1));
            let Some(t288) = c.ld8(c.r64(7).wrapping_add(c.r64(1)).wrapping_sub(0xb)) else { return RegfileExit::fault(0x70008d9a7e); };
            c.w32(7, t288);
            if !c.st8(c.r64(11).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008d9a83); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ne() { blk = 397; continue; }
            blk = 399; continue;
        }
        399 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9a92); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9a93); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9a99); }
            return c.ret(0x0, 0x70008d9a9b);
        }
        400 => {
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9a9c); };
            c.w64(0, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9aa0); };
            c.w64(12, t290);
            blk = 429; continue;
        }
        401 => {
            c.w32(11, c.r16(15));
            c.w32(2, 0xcu64);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 417; continue; }
            blk = 402; continue;
        }
        402 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(11));
            c.w64(11, c.r64(0));
            blk = 403; continue;
        }
        403 => {
            c.w64(15, c.r64(12));
            let r291 = c.r32(15) & 0x7u64;
            c.w32(15, r291);
            c.logic(r291, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 408; continue; }
            blk = 404; continue;
        }
        404 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 416; continue; }
            blk = 405; continue;
        }
        405 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 413; continue; }
            blk = 406; continue;
        }
        406 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 414; continue; }
            blk = 407; continue;
        }
        407 => {
            blk = 499; continue;
        }
        408 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 415; continue; }
            blk = 409; continue;
        }
        409 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 411; continue; }
            blk = 410; continue;
        }
        410 => {
            blk = 499; continue;
        }
        411 => {
            let Some(t292) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d9afc); };
            c.w32(12, t292);
            let Some(t293) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008d9b00); };
            c.cmp(t293, c.r32(12), 32);
            if c.cc_ne() { blk = 500; continue; }
            blk = 412; continue;
        }
        412 => {
            let Some(t294) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d9b0d); };
            c.w32(11, t294);
            let r295 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r295);
            let Some(t296) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d9b14); };
            c.w32(12, t296);
            let r297 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r297);
            let Some(t298) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9b1b); };
            c.w32(12, t298);
            blk = 403; continue;
        }
        413 => {
            let Some(t299) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008d9b23); };
            c.w32(12, t299);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r300 = c.add(c.r64(7), c.r64(12), 64);
            c.w64(7, r300);
            let Some(t301) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d9b30); };
            c.w32(11, t301);
            let r302 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r302);
            let Some(t303) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d9b37); };
            c.w32(12, t303);
            let r304 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r304);
            let Some(t305) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9b3e); };
            c.w32(12, t305);
            blk = 403; continue;
        }
        414 => {
            let Some(t306) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008d9b49); };
            c.w32(11, t306);
            let r307 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r307);
            let Some(t308) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008d9b50); };
            c.w32(12, t308);
            let r309 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r309);
            let Some(t310) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9b57); };
            c.w32(12, t310);
            blk = 403; continue;
        }
        415 => {
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 418; continue;
        }
        416 => {
            c.w32(15, 0x1u64);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 418; continue;
        }
        417 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(0));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 418; continue;
        }
        418 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 501; continue; }
            blk = 419; continue;
        }
        419 => {
            let r311 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r311);
            blk = 420; continue;
        }
        420 => {
            let r312 = c.add(c.r64(7), c.r64(3), 64);
            c.w64(7, r312);
            let r313 = c.r32(11) & 0x8u64;
            c.w32(11, r313);
            c.logic(r313, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 422; continue; }
            blk = 421; continue;
        }
        421 => {
            let Some(t314) = c.ld16(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(2))) else { return RegfileExit::fault(0x70008d9ba4); };
            c.w32(7, t314);
            blk = 423; continue;
        }
        422 => {
            let Some(t315) = c.ld8(c.r64(7).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008d9bab); };
            c.w32(7, t315);
            blk = 423; continue;
        }
        423 => {
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_g() { blk = 516; continue; }
            blk = 424; continue;
        }
        424 => {
            c.w64(11, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d9bc4); };
            c.w64(12, t316);
            let Some(t317) = c.ld32(c.r64(12).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008d9bcb); };
            c.w32(0, t317);
            let r318 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r318);
            let Some(t319) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d9bd3); };
            c.cmp(t319, c.r32(0), 32);
            if c.cc_e() { blk = 521; continue; }
            blk = 425; continue;
        }
        425 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9be3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9be4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9bea); }
            return c.ret(0x0, 0x70008d9bec);
        }
        426 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9bf3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9bf4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9bfa); }
            return c.ret(0x0, 0x70008d9bfc);
        }
        427 => {
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_a() { blk = 429; continue; }
            blk = 428; continue;
        }
        428 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9c06); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9c07); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9c0d); }
            return c.ret(0x0, 0x70008d9c0f);
        }
        429 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(8, c.r64(12));
            let r320 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r320);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008d9c19); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d9c1a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008d9c1b); }
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x27c0)) else { return RegfileExit::fault(0x70008d9c1d); };
            c.w64(3, t321);
            c.w32(0, 0x3u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008d9c30, 0x70008d9c2b);
        }
        430 => {
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9c30); };
            c.w64(7, t322);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9c37); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9c38); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9c3e); }
            return c.ret(0x0, 0x70008d9c40);
        }
        431 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9c47); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9c48); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9c4e); }
            return c.ret(0x0, 0x70008d9c50);
        }
        432 => {
            c.w64(0, c.r64(11));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9c57); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9c58); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9c5e); }
            return c.ret(0x0, 0x70008d9c60);
        }
        433 => {
            return RegfileExit::fault(0x70008d9c61);
        }
        434 => {
            return RegfileExit::fault(0x70008d9c62);
        }
        435 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d9c63); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008d9c67); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008d9c6d); };
            c.w64(3, t323);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008d9c7c, 0x70008d9c77);
        }
        436 => {
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9c7c); };
            c.w64(0, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9c80); };
            c.w64(6, t325);
            blk = 1; continue;
        }
        437 => {
            let Some(t326) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008d9c89); };
            c.cmp(t326, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t327) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008d9c98); };
            c.cmp(t327, c.r32(7), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d9ca7); }
            let r328 = c.r32(8) | c.r32(9);
            c.w32(8, r328);
            c.logic(r328, 32);
            if c.cc_ne() { blk = 440; continue; }
            blk = 438; continue;
        }
        438 => {
            c.w64(0, c.r64(7));
            return c.call(0x700089c794, 0x70008d9cb8, 0x70008d9cb3);
        }
        439 => {
            c.w64(7, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9cbb); };
            c.w64(0, t329);
            blk = 3; continue;
        }
        440 => {
            c.w32(7, 0x36u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d9cc9); }
            let Some(t330) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008d9cca); };
            c.w64(7, t330);
            let Some(t331) = c.ld32(c.r64(7).wrapping_add(0x393)) else { return RegfileExit::fault(0x70008d9cd1); };
            c.w32(7, t331);
            let r332 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r332);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d9cda); }
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008d9cdb); };
            c.w64(3, t333);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008d9cec, 0x70008d9ce7);
        }
        441 => {
            return RegfileExit::fault(0x70008d9cec);
        }
        442 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008d9ced); }
            c.w64(0, c.r64(7));
            return c.call(0x70008a1214, 0x70008d9cf9, 0x70008d9cf4);
        }
        443 => {
            c.w64(7, c.r64(0));
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9cfc); };
            c.w64(0, t334);
            blk = 3; continue;
        }
        444 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d9d0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d9d0e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008d9d12); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008d9d1e, 0x70008d9d19);
        }
        445 => {
            c.w64(8, c.r64(0));
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9d21); };
            c.w64(9, t335);
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9d25); };
            c.w64(7, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9d29); };
            c.w64(0, t337);
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d9d2d); };
            c.w64(3, t338);
            blk = 134; continue;
        }
        446 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d9d36); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d9d3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008d9d3e); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008d9d4c, 0x70008d9d47);
        }
        447 => {
            c.w64(8, c.r64(0));
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9d4f); };
            c.w64(9, t339);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9d53); };
            c.w64(7, t340);
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9d57); };
            c.w64(0, t341);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d9d5b); };
            c.w64(3, t342);
            blk = 137; continue;
        }
        448 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d9d64); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d9d68); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d9d6c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d9d70); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008d9d7c, 0x70008d9d77);
        }
        449 => {
            c.w64(7, c.r64(0));
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9d7f); };
            c.w64(9, t343);
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d9d83); };
            c.w64(3, t344);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d9d87); };
            c.w64(15, t345);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9d8b); };
            c.w64(0, t346);
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d9d8f); };
            c.w64(11, t347);
            blk = 146; continue;
        }
        450 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d9d98); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d9d9c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d9da0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008d9da4); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008d9db0, 0x70008d9dab);
        }
        451 => {
            c.w64(7, c.r64(0));
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9db3); };
            c.w64(9, t348);
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008d9db7); };
            c.w64(3, t349);
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d9dbb); };
            c.w64(15, t350);
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9dbf); };
            c.w64(0, t351);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d9dc3); };
            c.w64(11, t352);
            blk = 160; continue;
        }
        452 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d9dcc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008d9dd0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d9dd4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d9dd8); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febb4, 0x70008d9de4, 0x70008d9ddf);
        }
        453 => {
            c.w64(7, c.r64(0));
            let Some(t353) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d9de7); };
            c.w64(6, t353);
            c.w32(1, 0xbu64);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d9df3); };
            c.w64(15, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9df7); };
            c.w64(3, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9dfb); };
            c.w64(0, t356);
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d9dff); };
            c.w64(11, t357);
            blk = 176; continue;
        }
        454 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d9e08); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008d9e0c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008d9e10); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008d9e14); }
            return c.call(0x70007febb4, 0x70008d9e1d, 0x70008d9e18);
        }
        455 => {
            c.w64(8, c.r64(0));
            let Some(t358) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d9e20); };
            c.w64(7, t358);
            c.w32(1, 0xbu64);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d9e2c); };
            c.w64(15, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9e30); };
            c.w64(3, t360);
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9e34); };
            c.w64(0, t361);
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008d9e38); };
            c.w64(11, t362);
            blk = 189; continue;
        }
        456 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d9e41); }
            blk = 460; continue;
        }
        457 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d9e47); }
            blk = 460; continue;
        }
        458 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008d9e4d); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 460; continue; }
            blk = 459; continue;
        }
        459 => {
            let Some(t363) = c.ld64(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008d9e57); };
            c.w64(7, t363);
            blk = 216; continue;
        }
        460 => {
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 462; continue; }
            blk = 461; continue;
        }
        461 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15)));
            blk = 471; continue;
        }
        462 => {
            let Some(t364) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d9e6f); };
            c.w64(11, t364);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t365) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d9e76); };
            c.cmp(t365, c.r64(12), 64);
            if c.cc_a() { blk = 465; continue; }
            blk = 463; continue;
        }
        463 => {
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x70008d9e83, 0x70008d9e7e);
        }
        464 => {
            c.w64(11, c.r64(0));
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9e86); };
            c.w64(9, t366);
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9e8a); };
            c.w64(7, t367);
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d9e8e); };
            c.w64(8, t368);
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d9e95); };
            c.w64(0, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008d9e9c); };
            c.w64(15, t370);
            blk = 466; continue;
        }
        465 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008d9ea2); }
            let r371 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r371);
            blk = 466; continue;
        }
        466 => {
            let Some(t372) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008d9ea9); };
            c.w64(12, t372);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008d9eb0); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(15) as i64));
            c.logic(c.r64(15) & c.r64(15), 64);
            if c.cc_ns() { blk = 470; continue; }
            blk = 467; continue;
        }
        467 => {
            c.w64(10, c.r64(15));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 469; continue; }
            blk = 468; continue;
        }
        468 => {
            let r373 = c.r64(10) | 0x1u64;
            c.w64(10, r373);
            c.logic(r373, 64);
            blk = 469; continue;
        }
        469 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 470; continue;
        }
        470 => {
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008d9edb); }
            blk = 471; continue;
        }
        471 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008d9ee1); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008d9ee2); }
            let Some(t374) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008d9ee4); };
            c.w64(3, t374);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008d9efa, 0x70008d9ef5);
        }
        472 => {
            let r375 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r375);
            c.w64(7, c.r64(0));
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9eff); };
            c.w64(9, t376);
            let Some(t377) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008d9f03); };
            c.w64(8, t377);
            let Some(t378) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008d9f0a); };
            c.w64(0, t378);
            blk = 219; continue;
        }
        473 => {
            let Some(t379) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d9f16); };
            c.w64(11, t379);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d9f1d); }
            let Some(t380) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d9f21); };
            c.cmp(t380, c.r64(12), 64);
            if c.cc_a() { blk = 476; continue; }
            blk = 474; continue;
        }
        474 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d9f30, 0x70008d9f2b);
        }
        475 => {
            blk = 477; continue;
        }
        476 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008d9f32); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 477; continue;
        }
        477 => {
            let Some(t381) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008d9f3c); };
            c.w64(8, t381);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008d9f43); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d9f47); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d9f4e); }
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9f55); };
            c.w64(7, t382);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008d9f59); }
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9f5d); };
            c.w64(9, t383);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9f64); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9f65); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9f6b); }
            return c.ret(0x0, 0x70008d9f6d);
        }
        478 => {
            let Some(t384) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008d9f6e); };
            c.w64(15, t384);
            c.w64(3, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008d9f75); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008d9f79); }
            let Some(t385) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008d9f7d); };
            c.cmp(t385, c.r64(3), 64);
            if c.cc_a() { blk = 481; continue; }
            blk = 479; continue;
        }
        479 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008d9f8c, 0x70008d9f87);
        }
        480 => {
            blk = 482; continue;
        }
        481 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008d9f8e); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 482; continue;
        }
        482 => {
            let Some(t386) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008d9f98); };
            c.w64(9, t386);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008d9f9f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008d9fa3); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008d9faa); }
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008d9fb1); };
            c.w64(7, t387);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x70008d9fb5); }
            let Some(t388) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008d9fb9); };
            c.w64(12, t388);
            let Some(t389) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008d9fc0); };
            c.w32(8, t389);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 484; continue; }
            blk = 483; continue;
        }
        483 => {
            return RegfileExit::fault(0x70008d9fd0);
        }
        484 => {
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008d9fd1); };
            c.w64(11, t390);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008d9fd5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 486; continue; }
            blk = 485; continue;
        }
        485 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r391 = c.r64(7) & c.r64(12);
            c.w64(7, r391);
            c.logic(r391, 64);
            let Some(t392) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008d9fe7); };
            c.logic(t392 & 0x4u64, 8);
            if c.cc_ne() { blk = 533; continue; }
            blk = 486; continue;
        }
        486 => {
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008d9ff1); };
            c.w64(9, t393);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008d9ff8); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008d9ff9); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008d9fff); }
            return c.ret(0x0, 0x70008da001);
        }
        487 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008da002); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008da006); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008da00a); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008da018, 0x70008da013);
        }
        488 => {
            c.w64(8, c.r64(0));
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da01b); };
            c.w64(9, t394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da01f); };
            c.w64(2, t395);
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008da023); };
            c.w64(1, t396);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da027); };
            c.w64(7, t397);
            blk = 340; continue;
        }
        489 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008da030); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008da034); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x70008da038); }
            c.w32(2, 0x14u64);
            return c.call(0x70007febf4, 0x70008da046, 0x70008da041);
        }
        490 => {
            c.w64(8, c.r64(0));
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da049); };
            c.w64(9, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da04d); };
            c.w64(2, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008da051); };
            c.w64(1, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da055); };
            c.w64(7, t401);
            blk = 343; continue;
        }
        491 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008da05e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008da062); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008da066); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008da06a); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008da076, 0x70008da071);
        }
        492 => {
            c.w64(8, c.r64(0));
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da079); };
            c.w64(9, t402);
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da07d); };
            c.w64(3, t403);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da081); };
            c.w64(2, t404);
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008da085); };
            c.w64(7, t405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008da089); };
            c.w64(1, t406);
            blk = 352; continue;
        }
        493 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008da092); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008da096); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008da09a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008da09e); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008da0aa, 0x70008da0a5);
        }
        494 => {
            c.w64(8, c.r64(0));
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da0ad); };
            c.w64(9, t407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da0b1); };
            c.w64(3, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da0b5); };
            c.w64(2, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008da0b9); };
            c.w64(7, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008da0bd); };
            c.w64(1, t411);
            blk = 365; continue;
        }
        495 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008da0c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008da0ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008da0ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008da0d2); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008da0de, 0x70008da0d9);
        }
        496 => {
            c.w64(8, c.r64(0));
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da0e1); };
            c.w64(9, t412);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da0e5); };
            c.w64(3, t413);
            let Some(t414) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008da0e9); };
            c.w64(6, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da0f0); };
            c.w64(1, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da0f4); };
            c.w64(2, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008da0f8); };
            c.w64(7, t417);
            blk = 380; continue;
        }
        497 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008da101); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008da105); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008da109); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008da10d); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008da119, 0x70008da114);
        }
        498 => {
            c.w64(8, c.r64(0));
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da11c); };
            c.w64(9, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da120); };
            c.w64(3, t419);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da124); };
            c.w64(1, t420);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008da128); };
            c.w64(2, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008da12c); };
            c.w64(7, t422);
            blk = 393; continue;
        }
        499 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008da135); }
            blk = 503; continue;
        }
        500 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008da13b); }
            blk = 503; continue;
        }
        501 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008da141); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 503; continue; }
            blk = 502; continue;
        }
        502 => {
            let Some(t423) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008da14b); };
            c.w64(12, t423);
            blk = 420; continue;
        }
        503 => {
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 505; continue; }
            blk = 504; continue;
        }
        504 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            c.w64(11, c.r64(7));
            blk = 514; continue;
        }
        505 => {
            let Some(t424) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da166); };
            c.w64(7, t424);
            let Some(t425) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008da16d); };
            c.w64(11, t425);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t426) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008da174); };
            c.cmp(t426, c.r64(12), 64);
            if c.cc_a() { blk = 508; continue; }
            blk = 506; continue;
        }
        506 => {
            return c.call(0x70007febf4, 0x70008da17e, 0x70008da179);
        }
        507 => {
            c.w64(11, c.r64(0));
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da181); };
            c.w64(9, t427);
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da185); };
            c.w64(0, t428);
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008da189); };
            c.w64(3, t429);
            let Some(t430) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da18d); };
            c.w64(7, t430);
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008da194); };
            c.w64(8, t431);
            blk = 509; continue;
        }
        508 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008da19d); }
            let r432 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r432);
            blk = 509; continue;
        }
        509 => {
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008da1a4); };
            c.w64(12, t433);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008da1ab); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 513; continue; }
            blk = 510; continue;
        }
        510 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 512; continue; }
            blk = 511; continue;
        }
        511 => {
            let r434 = c.r64(10) | 0x1u64;
            c.w64(10, r434);
            c.logic(r434, 64);
            blk = 512; continue;
        }
        512 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 513; continue;
        }
        513 => {
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008da1d6); }
            blk = 514; continue;
        }
        514 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008da1dc); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008da1dd); }
            c.w32(0, 0x2u64);
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008da1e4); };
            c.w64(3, t435);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008da1f2, 0x70008da1ed);
        }
        515 => {
            let r436 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r436);
            c.w64(7, c.r64(0));
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da1f7); };
            c.w64(9, t437);
            let Some(t438) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008da1fb); };
            c.w64(8, t438);
            blk = 423; continue;
        }
        516 => {
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da207); };
            c.w64(11, t439);
            let Some(t440) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008da20e); };
            c.w64(12, t440);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008da216); }
            let Some(t441) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008da21a); };
            c.cmp(t441, c.r64(15), 64);
            if c.cc_a() { blk = 519; continue; }
            blk = 517; continue;
        }
        517 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008da229, 0x70008da224);
        }
        518 => {
            blk = 520; continue;
        }
        519 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008da22b); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 520; continue;
        }
        520 => {
            let Some(t442) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x70008da236); };
            c.w64(7, t442);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008da23d); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008da240); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008da247); }
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008da24e); };
            c.w64(7, t443);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(7)) { return RegfileExit::fault(0x70008da252); }
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da256); };
            c.w64(9, t444);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008da25d); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008da25e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008da264); }
            return c.ret(0x0, 0x70008da266);
        }
        521 => {
            let Some(t445) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008da267); };
            c.w64(15, t445);
            let Some(t446) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008da26e); };
            c.w64(0, t446);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008da275); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x70008da279); }
            let Some(t447) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008da27d); };
            c.cmp(t447, c.r64(3), 64);
            if c.cc_a() { blk = 524; continue; }
            blk = 522; continue;
        }
        522 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008da28c, 0x70008da287);
        }
        523 => {
            blk = 525; continue;
        }
        524 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008da28e); }
            c.w64(8, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 525; continue;
        }
        525 => {
            let Some(t448) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008da298); };
            c.w64(7, t448);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008da29f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008da2a2); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008da2a9); }
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008da2b0); };
            c.w64(7, t449);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(7)) { return RegfileExit::fault(0x70008da2b4); }
            let Some(t450) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x70008da2b8); };
            c.w64(12, t450);
            let Some(t451) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008da2bf); };
            c.w32(8, t451);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_b() { blk = 527; continue; }
            blk = 526; continue;
        }
        526 => {
            return RegfileExit::fault(0x70008da2cf);
        }
        527 => {
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008da2d0); };
            c.w64(11, t452);
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008da2d4); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 529; continue; }
            blk = 528; continue;
        }
        528 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r453 = c.r64(7) & c.r64(12);
            c.w64(7, r453);
            c.logic(r453, 64);
            let Some(t454) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008da2e6); };
            c.logic(t454 & 0x4u64, 8);
            if c.cc_ne() { blk = 530; continue; }
            blk = 529; continue;
        }
        529 => {
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008da2ec); };
            c.w64(7, t455);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008da2f3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008da2f4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008da2fa); }
            return c.ret(0x0, 0x70008da2fc);
        }
        530 => {
            c.w32(0, c.r32(0));
            let r456 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r456);
            c.w64(7, 0xfffffffffffc0000u64);
            let r457 = c.r64(7) & c.r64(0);
            c.w64(7, r457);
            c.logic(r457, 64);
            let Some(t458) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008da30c); };
            c.logic(t458 & 0x2u64, 8);
            if c.cc_e() { blk = 529; continue; }
            blk = 531; continue;
        }
        531 => {
            c.w64(8, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008da316); }
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x70008da31f, 0x70008da31a);
        }
        532 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008da31f); }
            blk = 529; continue;
        }
        533 => {
            c.w32(0, c.r32(0));
            let r459 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r459);
            c.w64(7, 0xfffffffffffc0000u64);
            let r460 = c.r64(7) & c.r64(0);
            c.w64(7, r460);
            c.logic(r460, 64);
            let Some(t461) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008da331); };
            c.logic(t461 & 0x2u64, 8);
            if c.cc_e() { blk = 486; continue; }
            blk = 534; continue;
        }
        534 => {
            c.w64(8, c.r64(12).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008da33f); }
            c.w64(7, c.r64(12));
            return c.call(0x70007f0e54, 0x70008da348, 0x70008da343);
        }
        535 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008da348); }
            blk = 486; continue;
        }
        536 => {
            return RegfileExit::jump(0x70008da350);
        }
            _ => unreachable!(),
        }
    }
}
