#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008b01d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0201); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0207); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b020d); };
            c.w32(8, t4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0212); };
            c.w32(9, t5);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 59; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 22; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x41u64, 16);
            if c.cc_e() { blk = 51; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 48; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t6) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b0252); };
            c.logic(t6 & 0x10u64, 8);
            if c.cc_ne() { blk = 38; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 49; continue;
        }
        11 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70008b0261); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t8) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b0276); };
            c.w32(0, t8);
            let r9 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r9);
            blk = 0; continue;
        }
        14 => {
            c.cmp(c.r16(8), c.r16(9), 16);
            if c.cc_e() { blk = 43; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 58; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 23; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 48; continue;
        }
        20 => {
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70008b02b1); };
            c.cmp(t10, c.r32(7), 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b02be); };
            c.w32(3, t11);
            let r12 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r12);
            blk = 0; continue;
        }
        22 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_b() { blk = 48; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 0; continue;
        }
        24 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t13) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b02ec); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b02f2); };
            c.cmp(t15, c.r32(1), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t16) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b02ff); };
            c.w32(7, t16);
            c.cmp(c.r16(7), 0x40u64, 16);
            if c.cc_b() { blk = 63; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r16(7), 0x43u64, 16);
            if c.cc_e() { blk = 31; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r16(7), 0x41u64, 16);
            if c.cc_e() { blk = 74; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r16(7), 0xbau64, 16);
            if c.cc_ae() { blk = 61; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 48; continue;
        }
        31 => {
            let Some(t17) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70008b032d); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t18) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b033a); };
            c.w32(3, t18);
            let r19 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r19);
            blk = 0; continue;
        }
        33 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b034e); };
            c.w32(1, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b0351); };
            c.cmp(t21, c.r32(1), 32);
            if c.cc_ne() { blk = 47; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b035a); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 48; continue; }
            blk = 36; continue;
        }
        36 => {
            if c.cc_e() { blk = 47; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 48; continue;
        }
        38 => {
            let Some(t22) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b0369); };
            c.logic(t22 & 0x10u64, 8);
            if c.cc_ne() { blk = 47; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 48; continue;
        }
        40 => {
            let Some(t23) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b0371); };
            c.logic(t23 & 0x10u64, 8);
            if c.cc_ne() { blk = 47; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_e() { blk = 48; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            blk = 48; continue;
        }
        43 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b0385); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b038a); }
            blk = 45; continue;
        }
        44 => {
            let r24 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r24);
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b0393); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 45; continue;
        }
        45 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b03a8); };
            c.w64(0, t25);
            return c.ret(0x0, 0x70008b03af);
        }
        48 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b03b0); };
            c.w64(0, t26);
            return c.ret(0x0, 0x70008b03b7);
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b03b8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b03bc); }
            let r27 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r27);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008b03c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b03c6); }
            return c.call(0x70008a2374, 0x70008b03cf, 0x70008b03ca);
        }
        50 => {
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b03cf); };
            c.w64(6, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b03d3); };
            c.w64(3, t29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b03da); }
            blk = 0; continue;
        }
        51 => {
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b03e0); };
            c.cmp(t30, c.r32(7), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 68; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 66; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0x380)) else { return RegfileExit::fault(0x70008b0403); };
            c.cmp(t31, c.r32(7), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 23; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 48; continue;
        }
        57 => {
            let Some(t32) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b041a); };
            c.w32(3, t32);
            let r33 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r33);
            blk = 0; continue;
        }
        58 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b0425); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b0429); }
            let r34 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r34);
            blk = 64; continue;
        }
        59 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 72; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 23; continue;
        }
        61 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b0441); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b0445); }
            let r35 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r35);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008b044b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b044f); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a2374, 0x70008b045b, 0x70008b0456);
        }
        62 => {
            c.w64(3, c.r64(0));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b045e); };
            c.w64(0, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0462); };
            c.w64(6, t37);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b0469); }
            blk = 0; continue;
        }
        63 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b046f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b0473); }
            let r38 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r38);
            blk = 64; continue;
        }
        64 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008b0479); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b047d); }
            c.w64(0, c.r64(3));
            return c.call(0x70008a0fd4, 0x70008b0489, 0x70008b0484);
        }
        65 => {
            c.w64(3, c.r64(0));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b048c); };
            c.w64(0, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0490); };
            c.w64(6, t40);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b0497); }
            blk = 0; continue;
        }
        66 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b049d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b04a1); }
            let r41 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r41);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b04a7); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b04a8); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x1d50)) else { return RegfileExit::fault(0x70008b04a9); };
            c.w64(3, t42);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b04bc, 0x70008b04b7);
        }
        67 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b04bf); }
            return c.ret(0x0, 0x70008b04c0);
        }
        68 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b04c1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b04c5); }
            let r43 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r43);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b04cb); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b04cc); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008b04cd); };
            c.w64(3, t44);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b04e0, 0x70008b04db);
        }
        69 => {
            blk = 67; continue;
        }
        70 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b04e2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b04e6); }
            let r45 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r45);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b04ec); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b04ed); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x70008b04ee); };
            c.w64(3, t46);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b0501, 0x70008b04fc);
        }
        71 => {
            blk = 67; continue;
        }
        72 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b0503); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b0507); }
            let r47 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r47);
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb534, 0x70008b0518, 0x70008b0513);
        }
        73 => {
            blk = 67; continue;
        }
        74 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b051a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b051e); }
            let r48 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r48);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b0524); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b0525); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x1d48)) else { return RegfileExit::fault(0x70008b0526); };
            c.w64(3, t49);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b0539, 0x70008b0534);
        }
        75 => {
            blk = 67; continue;
        }
        76 => {
            return RegfileExit::jump(0x70008b053c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b0554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(1, (c.cc_e() as u64));
            c.w32(1, c.r8(1));
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 1; continue;
        }
        1 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0578); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b057e); };
            c.cmp(t2, c.r32(1), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t3) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0590); };
            c.w32(1, t3);
            c.cmp(c.r16(1), 0x40u64, 16);
            if c.cc_b() { blk = 30; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r16(1), 0x41u64, 16);
            if c.cc_ne() { blk = 26; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b05a8); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b05ae); };
            c.cmp(t6, 0x41u64, 16);
            if c.cc_e() { blk = 28; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 26; continue;
        }
        8 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t7) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b05c3); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b05c6); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b05d3); }
            if !c.movsd_rm(1, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b05d8); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 11; continue;
        }
        11 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 12; continue;
        }
        12 => {
            blk = 26; continue;
        }
        13 => {
            let r9 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r9);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b05f4); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 26; continue;
        }
        16 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0614); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b061a); };
            c.cmp(t12, c.r32(1), 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t13) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0623); };
            c.cmp(t13, 0x43u64, 16);
            blk = 26; continue;
        }
        19 => {
            let r14 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r14);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b062c); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 26; continue;
        }
        22 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t15) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0648); };
            c.w32(1, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b064b); };
            c.cmp(t16, c.r32(1), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b0654); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            if c.cc_e() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b0661); };
            c.w64(0, t17);
            return c.ret(0x0, 0x70008b0668);
        }
        27 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b0669); };
            c.w64(0, t18);
            return c.ret(0x0, 0x70008b0670);
        }
        28 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b0671); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b0675); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b0677); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b0678); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x1d40)) else { return RegfileExit::fault(0x70008b0679); };
            c.w64(3, t19);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b068c, 0x70008b0687);
        }
        29 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b068f); }
            return c.ret(0x0, 0x70008b0690);
        }
        30 => {
            let Some(t20) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0691); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            let Some(t22) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0697); };
            c.cmp(t22, 0x40u64, 16);
            if c.cc_ae() { blk = 26; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b069e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b06a2); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            c.w64(0, c.r64(3));
            return c.call(0x70007fb534, 0x70008b06b1, 0x70008b06ac);
        }
        32 => {
            blk = 29; continue;
        }
        33 => {
            return RegfileExit::jump(0x70008b06b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b06d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b06d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b06d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b06d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b06da); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b06df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b06e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 207; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(5));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008b06f3); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b06fd); };
            c.w64(9, t2);
            c.w64(11, c.r64(9));
            blk = 4; continue;
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0706); };
            c.w64(11, t3);
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b070d); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b071c); };
            c.cmp(t5, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b072b); };
            c.w64(9, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008b0732); }
            let r7 = c.r32(12) | c.r32(15);
            c.w32(12, r7);
            c.logic(r7, 32);
            if c.cc_ne() { blk = 205; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xfe8)) else { return RegfileExit::fault(0x70008b074b); };
            c.w64(12, t8);
            let Some(t9) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0752); };
            c.cmp(t9, c.r32(3), 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b075d); };
            c.w32(12, t10);
            let r11 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r11);
            let Some(t12) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0764); };
            c.cmp(t12, 0x834u64, 16);
            if c.cc_ne() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w32(15, 0x1u64);
            blk = 10; continue;
        }
        9 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        10 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t13) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008b0788); };
            c.w32(15, t13);
            let r14 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r14);
            let r15 = c.r32(15) & 0x3fu64;
            c.w32(15, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_a() { blk = 99; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b07a0); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b07a7); };
            c.w32(15, t18);
            let r19 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r19);
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b07ae); };
            c.w32(2, t20);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x70008b07be);
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(15).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008b07bf); };
            c.w32(1, t21);
            let Some(t22) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b07c6); };
            c.cmp(t22, c.r32(1), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008b07d1); };
            c.w64(12, t23);
            let Some(t24) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b07d8); };
            c.cmp(t24, c.r32(3), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t25) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b07e3); };
            c.w32(12, t25);
            let r26 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r26);
            let Some(t27) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b07ea); };
            c.w32(1, t27);
            let r28 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r28);
            let Some(t29) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b07f2); };
            c.w32(1, t29);
            let r30 = c.sub(c.r32(1), 0x77u64, 32);
            c.w32(1, r30);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008b07f9); }
            c.cmp(c.r32(1), 0xcu64, 32);
            if c.cc_a() { blk = 99; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0806); };
            c.w32(1, t31);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(1)) { return RegfileExit::fault(0x70008b080a); }
            c.cmp(c.r64(2), 0x8cu64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x70008b0817);
        }
        19 => {
            let Some(t32) = c.ld32(c.r64(15).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008b0818); };
            c.w32(15, t32);
            let r33 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r33);
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b0822); };
            c.w32(15, t34);
            let r35 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r35);
            let Some(t36) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0829); };
            c.w32(2, t36);
            let Some(t37) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008b082d); };
            c.cmp(t37, c.r32(2), 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t38) = c.ld8(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b083a); };
            c.w32(2, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b083f); };
            c.w64(7, t39);
            let Some(t40) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b0846); };
            c.w64(8, t40);
            if let Some(sr) = c.shl(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b0851); };
            c.w64(6, t41);
            let Some(t42) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008b0858); };
            c.cmp(t42, c.r64(11), 64);
            if c.cc_be() { blk = 209; continue; }
            blk = 21; continue;
        }
        21 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008b0861); }
            let r43 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r43);
            blk = 22; continue;
        }
        22 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008b086c); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b0870); };
            c.w64(15, t44);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008b0877); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008b087b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008b087f); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let r45 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r45);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x70008b088d); }
            c.cmp(c.r64(11), 0xbu64, 64);
            if c.cc_ne() { blk = 23; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(11, c.r64(3));
            blk = 26; continue;
        }
        25 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b089c); };
            c.w64(11, t46);
            c.w32(8, c.r64(11).wrapping_add(0x2));
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b08a7); };
            c.w64(12, t47);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        26 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b08ad); };
            c.w64(6, t48);
            let Some(t49) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b08b1); };
            c.cmp(c.r32(11), t49, 32);
            if c.cc_ge() { blk = 80; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t50) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b08bb); };
            c.w32(15, t50);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_ae() { blk = 81; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t51) = c.ld32(c.r64(12).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b08cc); };
            c.w32(15, t51);
            let r52 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r52);
            let Some(t53) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b08d4); };
            c.cmp(t53, c.r32(15), 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b08dd); };
            c.w64(15, t54);
            blk = 30; continue;
        }
        30 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t55) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b08ee); };
            c.w32(2, t55);
            let r56 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r56);
            let Some(t57) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b08f5); };
            c.cmp(t57, 0x834u64, 16);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x1u64) { return RegfileExit::fault(0x70008b08fd); }
            blk = 34; continue;
        }
        33 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x0u64) { return RegfileExit::fault(0x70008b0907); }
            blk = 34; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t58) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b0911); };
            c.cmp(t58, 0x0u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t59) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008b091b); };
            c.w32(7, t59);
            let r60 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r60);
            let r61 = c.r32(7) & 0x3fu64;
            c.w32(7, r61);
            c.logic(r61, 32);
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 89; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t62) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b092e); };
            c.w32(7, t62);
            let r63 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r63);
            let Some(t64) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b0934); };
            c.w32(7, t64);
            let r65 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r65);
            let Some(t66) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b093a); };
            c.w32(8, t66);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_be() { blk = 82; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008b094e); };
            c.w32(8, t67);
            let Some(t68) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b0955); };
            c.cmp(t68, c.r32(8), 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008b095f); };
            c.w64(8, t69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0966); };
            c.cmp(t70, c.r32(3), 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t71) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0970); };
            c.w32(8, t71);
            let r72 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r72);
            let Some(t73) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0977); };
            c.w32(2, t73);
            let r74 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r74);
            let Some(t75) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b097e); };
            c.w32(15, t75);
            let Some(t76) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0982); };
            c.w32(7, t76);
            let r77 = c.sub(c.r32(7), 0x77u64, 32);
            c.w32(7, r77);
            c.cmp(c.r32(7), 0xcu64, 32);
            if c.cc_a() { blk = 50; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_g() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0998); };
            c.w64(2, t78);
            blk = 45; continue;
        }
        42 => {
            let Some(t79) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b09a1); };
            c.w32(2, t79);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 83; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t80) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b09b5); };
            c.w32(2, t80);
            let r81 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r81);
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b09bc); };
            c.cmp(t82, c.r32(2), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b09c5); };
            c.w64(2, t83);
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_g() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008b09d2); }
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b09d6); };
            c.w64(8, t84);
            blk = 69; continue;
        }
        47 => {
            let Some(t85) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b09e2); };
            c.w32(15, t85);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 84; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t86) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b09f6); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008b09fd); }
            let Some(t88) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b0a01); };
            c.cmp(t88, c.r32(8), 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0a0e); };
            c.w64(8, t89);
            blk = 69; continue;
        }
        50 => {
            let Some(t90) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008b0a1a); };
            c.cmp(t90, c.r32(2), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_g() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008b0a2d); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0a31); };
            c.w64(2, t91);
            blk = 60; continue;
        }
        53 => {
            let Some(t92) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0a3d); };
            c.w32(2, t92);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_be() { blk = 85; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x7)) { return RegfileExit::fault(0x70008b0a51); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008b0a57); }
            c.w64(2, 0xfff7fffffff7ffffu64);
            let Some(t93) = c.ld64(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0a65); };
            c.cmp(t93, c.r64(2), 64);
            if c.cc_e() { blk = 58; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b0a6b); };
            c.w64(2, t94);
            let Some(t95) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b0a72); };
            c.w64(7, t95);
            c.w64(11, c.r64(7).wrapping_add(0xc));
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b0a79); };
            c.w64(12, t96);
            let Some(t97) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b0a80); };
            c.cmp(t97, c.r64(11), 64);
            if c.cc_be() { blk = 211; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.st64(c.r64(2), c.r64(11)) { return RegfileExit::fault(0x70008b0a8a); }
            c.w64(11, c.r64(7).wrapping_add(0x1));
            blk = 57; continue;
        }
        57 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b0a91); };
            c.w64(12, t98);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008b0a98); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008b0a9c); }
            blk = 59; continue;
        }
        58 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0aa4); };
            c.w64(11, t99);
            blk = 59; continue;
        }
        59 => {
            c.w64(2, c.r64(11));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b0aae); };
            c.w64(12, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b0ab2); };
            c.w64(11, t101);
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_g() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0abc); };
            c.w64(8, t102);
            blk = 69; continue;
        }
        62 => {
            let Some(t103) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0ac5); };
            c.w32(15, t103);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0xf)) { return RegfileExit::fault(0x70008b0ad9); }
            let Some(t104) = c.ld64(0x70008b0a5du64) else { return RegfileExit::fault(0x70008b0adf); };
            c.w64(15, t104);
            let Some(t105) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b0ae6); };
            c.cmp(t105, c.r64(15), 64);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b0aec); };
            c.w64(8, t106);
            let Some(t107) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008b0af3); };
            c.w64(15, t107);
            c.w64(7, c.r64(15).wrapping_add(0xc));
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b0afa); };
            c.w64(11, t108);
            let Some(t109) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008b0b01); };
            c.cmp(t109, c.r64(7), 64);
            if c.cc_be() { blk = 213; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st64(c.r64(8), c.r64(7)) { return RegfileExit::fault(0x70008b0b0a); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 66; continue;
        }
        66 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b0b11); };
            c.w64(11, t110);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008b0b18); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008b0b1c); }
            blk = 68; continue;
        }
        67 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0b24); };
            c.w64(8, t111);
            blk = 68; continue;
        }
        68 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b0b2b); };
            c.w64(11, t112);
            blk = 69; continue;
        }
        69 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t113) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0b40); };
            c.w32(7, t113);
            let r114 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r114);
            let Some(t115) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0b46); };
            c.cmp(t115, 0x40u64, 16);
            if c.cc_a() { blk = 73; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b0b4d); };
            c.w64(0, t116);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(8));
            return c.call(0x7000875c54, 0x70008b0b5c, 0x70008b0b57);
        }
        72 => {
            blk = 25; continue;
        }
        73 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t117) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0b67); };
            c.w32(15, t117);
            let r118 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b0b6e); };
            c.cmp(t119, c.r32(15), 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b0b77); };
            c.w64(0, t120);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(8));
            return c.call(0x7000875c54, 0x70008b0b86, 0x70008b0b81);
        }
        76 => {
            blk = 25; continue;
        }
        77 => {
            let Some(t121) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0b8b); };
            c.cmp(t121, 0x43u64, 16);
            if c.cc_ne() { blk = 99; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t122) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0b97); };
            c.w32(9, t122);
            let r123 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b0b9e); };
            c.w64(0, t124);
            c.w64(3, c.r64(9));
            c.w64(1, c.r64(8));
            return c.call(0x7000875c54, 0x70008b0bad, 0x70008b0ba8);
        }
        79 => {
            blk = 25; continue;
        }
        80 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b0bb2); };
            c.w64(8, t125);
            let r126 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b0bba); };
            c.w64(0, t127);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b0bc1); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b0bc2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b0bc8); }
            return c.ret(0x0, 0x70008b0bca);
        }
        81 => {
            return RegfileExit::fault(0x70008b0bcb);
        }
        82 => {
            return RegfileExit::fault(0x70008b0bcc);
        }
        83 => {
            return RegfileExit::fault(0x70008b0bcd);
        }
        84 => {
            return RegfileExit::fault(0x70008b0bce);
        }
        85 => {
            return RegfileExit::fault(0x70008b0bcf);
        }
        86 => {
            return RegfileExit::fault(0x70008b0bd0);
        }
        87 => {
            blk = 99; continue;
        }
        88 => {
            return RegfileExit::fault(0x70008b0bd3);
        }
        89 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 91; continue;
        }
        90 => {
            c.w32(7, 0x1u64);
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t128) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0be2); };
            c.w32(8, t128);
            let r129 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r129);
            let Some(t130) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0be9); };
            c.w32(8, t130);
            let r131 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r131);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_be() { blk = 99; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w32(8, 0x92u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b0c04); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008b0c06); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b0c08); };
            c.w64(3, t132);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b0c19, 0x70008b0c14);
        }
        94 => {
            return RegfileExit::fault(0x70008b0c19);
        }
        95 => {
            return RegfileExit::fault(0x70008b0c1a);
        }
        96 => {
            blk = 99; continue;
        }
        97 => {
            c.w32(0, 0x1u64);
            blk = 99; continue;
        }
        98 => {
            c.w32(0, 0x1u64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 99; continue;
        }
        99 => {
            let Some(t133) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0c2b); };
            c.w32(8, t133);
            let r134 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r134);
            let Some(t135) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b0c32); };
            c.w32(8, t135);
            let r136 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r136);
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0c39); };
            c.w32(9, t137);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x8cu64, 64);
            if c.cc_a() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            return RegfileExit::fault(0x70008b0c4c);
        }
        101 => {
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008b0c4d); };
            c.w32(8, t138);
            let r139 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r139);
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b0c57); };
            c.w32(8, t140);
            let r141 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r141);
            let Some(t142) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0c5e); };
            c.w32(9, t142);
            let Some(t143) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008b0c62); };
            c.cmp(t143, c.r32(9), 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t144) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0c6f); };
            c.w32(9, t144);
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b0c74); };
            c.w64(11, t145);
            let Some(t146) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008b0c7b); };
            c.w64(12, t146);
            c.w64(2, c.r64(9).wrapping_mul(4));
            c.w64(9, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b0c8a); };
            c.w64(15, t147);
            let Some(t148) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b0c91); };
            c.cmp(t148, c.r64(9), 64);
            if c.cc_be() { blk = 215; continue; }
            blk = 103; continue;
        }
        103 => {
            if !c.st64(c.r64(11), c.r64(9)) { return RegfileExit::fault(0x70008b0c9a); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 104; continue;
        }
        104 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b0ca2); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b0ca6); };
            c.w64(8, t149);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b0cad); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b0cb1); }
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008b0cb9); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 108; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0cc3); };
            c.w64(11, t150);
            blk = 106; continue;
        }
        106 => {
            let r151 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r151);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x70008b0cce); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 106; continue; }
            blk = 107; continue;
        }
        107 => {
            blk = 109; continue;
        }
        108 => {
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0cda); };
            c.w64(11, t152);
            blk = 109; continue;
        }
        109 => {
            let Some(t153) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0ce1); };
            c.w32(8, t153);
            let r154 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r154);
            let Some(t155) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b0ce8); };
            c.w32(8, t155);
            let r156 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r156);
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0cef); };
            c.w32(12, t157);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x68u64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return RegfileExit::fault(0x70008b0cff);
        }
        111 => {
            let Some(t158) = c.ld32(c.r64(8).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008b0d00); };
            c.w32(8, t158);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008b0d07); }
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008b0d0b); };
            c.w64(3, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b0d12); };
            c.w64(0, t160);
            c.w64(12, c.r64(3));
            return c.call(0x7000870914, 0x70008b0d1e, 0x70008b0d19);
        }
        112 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 240; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t161) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0d26); };
            c.w32(8, t161);
            let r162 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r162);
            let Some(t163) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b0d2d); };
            c.logic(t163 & 0x2u64, 8);
            if c.cc_e() { blk = 240; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b0d38); };
            if !c.push64(t164) { return RegfileExit::fault(0x70008b0d38); }
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0d3b); };
            c.w64(6, t165);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008b0d4c, 0x70008b0d47);
        }
        115 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b0d4c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 238; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t166) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0d58); };
            c.w32(8, t166);
            let r167 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r167);
            let Some(t168) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0d5f); };
            c.cmp(t168, 0xbau64, 16);
            if c.cc_b() { blk = 238; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008b0d6c); };
            c.w64(3, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0d73); };
            c.w64(6, t170);
            return c.call(0x7000870914, 0x70008b0d7c, 0x70008b0d77);
        }
        118 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70008b0d7c); }
            blk = 120; continue;
        }
        119 => {
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b0d82); };
            c.w64(0, t171);
            blk = 120; continue;
        }
        120 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b0d86); };
            if !c.push64(t172) { return RegfileExit::fault(0x70008b0d86); }
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0d89); };
            c.w64(6, t173);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f21d4, 0x70008b0d97, 0x70008b0d92);
        }
        121 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b0d97); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 229; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t174) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0da3); };
            c.w32(8, t174);
            let r175 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r175);
            let Some(t176) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b0daa); };
            c.cmp(c.r32(8), t176, 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t177) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0db4); };
            c.cmp(t177, 0xbau64, 16);
            if c.cc_b() { blk = 229; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x70008b0dc1); };
            c.w64(3, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0dc8); };
            c.w64(6, t179);
            return c.call(0x7000870914, 0x70008b0dd1, 0x70008b0dcc);
        }
        125 => {
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b0dd1); };
            c.cmp(t180, c.r32(0), 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 134; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b0de1); };
            c.cmp(t181, c.r32(0), 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t182) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0dea); };
            c.w32(8, t182);
            let r183 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r183);
            let Some(t184) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b0df1); };
            c.logic(t184 & 0x10u64, 8);
            if c.cc_ne() { blk = 132; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t185) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b0df8); };
            c.cmp(t185, c.r32(8), 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t186) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0e05); };
            c.cmp(t186, 0x41u64, 16);
            if c.cc_e() { blk = 217; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 195; continue;
        }
        132 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b0e16); };
            c.w64(0, t187);
            blk = 145; continue;
        }
        133 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b0e1f); };
            c.w64(0, t188);
            blk = 145; continue;
        }
        134 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b0e32); };
            c.w64(0, t189);
            blk = 145; continue;
        }
        136 => {
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b0e38); };
            c.w64(0, t190);
            blk = 145; continue;
        }
        137 => {
            let Some(t191) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b0e3e); };
            c.w32(8, t191);
            let r192 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r192);
            let Some(t193) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b0e45); };
            c.cmp(t193, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t194) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b0e57); };
            c.cmp(t194, c.r32(8), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0e60); };
            c.w32(9, t195);
            let r196 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r196);
            let Some(t197) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b0e67); };
            c.logic(t197 & 0x10u64, 8);
            if c.cc_ne() { blk = 145; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b0e6e); };
            c.cmp(t198, c.r32(9), 32);
            if c.cc_e() { blk = 223; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t199) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0e7b); };
            c.cmp(t199, 0x41u64, 16);
            if c.cc_e() { blk = 221; continue; }
            blk = 143; continue;
        }
        143 => {
            blk = 195; continue;
        }
        144 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b0e96); };
            c.w64(8, t200);
            let Some(t201) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0e9a); };
            c.cmp(t201, c.r32(8), 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008b0ea0); };
            c.w64(3, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0ea7); };
            c.w64(6, t203);
            return c.call(0x7000870914, 0x70008b0eb0, 0x70008b0eab);
        }
        147 => {
            blk = 149; continue;
        }
        148 => {
            let Some(t204) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0eb2); };
            c.w32(0, t204);
            let r205 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r205);
            blk = 149; continue;
        }
        149 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b0eb8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 187; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t206) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0ec4); };
            c.w32(8, t206);
            let r207 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r207);
            let Some(t208) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0ecb); };
            c.cmp(t208, 0x834u64, 16);
            if c.cc_ne() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w32(11, 0x1u64);
            blk = 153; continue;
        }
        152 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 153; continue;
        }
        153 => {
            c.w32(9, 0);
            c.logic(0, 32);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t209) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008b0eec); };
            c.w32(11, t209);
            let r210 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r210);
            let r211 = c.r32(11) & 0x3fu64;
            c.w32(11, r211);
            c.logic(r211, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_a() { blk = 188; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b0f03); };
            c.w64(6, t212);
            let Some(t213) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0f07); };
            c.w32(11, t213);
            let r214 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r214);
            let Some(t215) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b0f0e); };
            c.w32(11, t215);
            let r216 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r216);
            let Some(t217) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0f15); };
            c.w32(12, t217);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x4cu64, 64);
            if c.cc_be() { blk = 196; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t218) = c.ld32(c.r64(11).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008b0f29); };
            c.w32(11, t218);
            let Some(t219) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b0f30); };
            c.cmp(t219, c.r32(11), 32);
            if c.cc_ne() { blk = 188; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008b0f3a); };
            c.w64(8, t220);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t221) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0f43); };
            c.cmp(t221, c.r32(3), 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t222) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0f4d); };
            c.w32(8, t222);
            let r223 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r223);
            let Some(t224) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b0f54); };
            c.w32(9, t224);
            let r225 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r225);
            let Some(t226) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0f5b); };
            c.w32(11, t226);
            let Some(t227) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0f5f); };
            c.w32(12, t227);
            let r228 = c.sub(c.r32(12), 0x77u64, 32);
            c.w32(12, r228);
            c.cmp(c.r32(12), 0xcu64, 32);
            if c.cc_a() { blk = 169; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_g() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0f78); };
            c.w64(9, t229);
            blk = 164; continue;
        }
        161 => {
            let Some(t230) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0f81); };
            c.w32(9, t230);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 197; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t231) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b0f95); };
            c.w32(9, t231);
            let r232 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r232);
            let Some(t233) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b0f9c); };
            c.cmp(t233, c.r32(9), 32);
            if c.cc_ne() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0fa5); };
            c.w64(9, t234);
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_g() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0fb2); };
            c.w64(8, t235);
            blk = 186; continue;
        }
        166 => {
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b0fbe); };
            c.w32(11, t236);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 198; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t237) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b0fd2); };
            c.w32(8, t237);
            let r238 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b0fd9); };
            c.cmp(t239, c.r32(8), 32);
            if c.cc_ne() { blk = 186; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t240) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b0fe6); };
            c.w64(8, t240);
            blk = 186; continue;
        }
        169 => {
            let Some(t241) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008b0ff2); };
            c.cmp(t241, c.r32(9), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_g() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b1005); };
            c.w64(9, t242);
            blk = 178; continue;
        }
        172 => {
            let Some(t243) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b100e); };
            c.w32(9, t243);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 199; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x7)) { return RegfileExit::fault(0x70008b1022); }
            let Some(t244) = c.ld64(0x70008b0a5du64) else { return RegfileExit::fault(0x70008b1028); };
            c.w64(9, t244);
            let Some(t245) = c.ld64(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b102f); };
            c.cmp(t245, c.r64(9), 64);
            if c.cc_e() { blk = 177; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b1035); };
            c.w64(9, t246);
            let Some(t247) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b103c); };
            c.w64(12, t247);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b1044); };
            c.w64(0, t248);
            let Some(t249) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008b104b); };
            c.cmp(t249, c.r64(15), 64);
            if c.cc_be() { blk = 225; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008b1054); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 176; continue;
        }
        176 => {
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b105c); };
            c.w64(12, t250);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008b1063); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008b1067); }
            blk = 178; continue;
        }
        177 => {
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b106f); };
            c.w64(9, t251);
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_g() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b107c); };
            c.w64(8, t252);
            blk = 186; continue;
        }
        180 => {
            let Some(t253) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1085); };
            c.w32(11, t253);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_be() { blk = 200; continue; }
            blk = 181; continue;
        }
        181 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0xf)) { return RegfileExit::fault(0x70008b1099); }
            let Some(t254) = c.ld64(0x70008b0a5du64) else { return RegfileExit::fault(0x70008b109f); };
            c.w64(11, t254);
            let Some(t255) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b10a6); };
            c.cmp(t255, c.r64(11), 64);
            if c.cc_e() { blk = 185; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b10ac); };
            c.w64(8, t256);
            let Some(t257) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008b10b3); };
            c.w64(11, t257);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b10ba); };
            c.w64(15, t258);
            let Some(t259) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b10c1); };
            c.cmp(t259, c.r64(12), 64);
            if c.cc_be() { blk = 227; continue; }
            blk = 183; continue;
        }
        183 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008b10ca); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 184; continue;
        }
        184 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b10d1); };
            c.w64(11, t260);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008b10d8); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008b10dc); }
            blk = 186; continue;
        }
        185 => {
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b10e4); };
            c.w64(8, t261);
            blk = 186; continue;
        }
        186 => {
            c.w64(0, c.r64(8));
            blk = 193; continue;
        }
        187 => {
            c.w32(9, 0x1u64);
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t262) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1100); };
            c.w32(8, t262);
            let r263 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r263);
            let Some(t264) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1107); };
            c.w32(8, t264);
            let r265 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r265);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 202; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b111c); };
            c.w64(6, t266);
            c.w32(3, 0);
            c.logic(0, 32);
            return c.call(0x7000870914, 0x70008b1127, 0x70008b1122);
        }
        191 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008b1127); }
            c.w32(3, 0x2u64);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b1130); };
            c.w64(0, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1134); };
            c.w64(6, t268);
            return c.call(0x7000870914, 0x70008b113d, 0x70008b1138);
        }
        192 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b113d); };
            c.w64(6, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b1141); };
            c.w64(9, t270);
            blk = 193; continue;
        }
        193 => {
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b1145); };
            if !c.push64(t271) { return RegfileExit::fault(0x70008b1145); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b1148); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b114a); }
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0x2340)) else { return RegfileExit::fault(0x70008b114b); };
            c.w64(3, t272);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008b115f, 0x70008b115a);
        }
        194 => {
            blk = 119; continue;
        }
        195 => {
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b1164); };
            c.w64(8, t273);
            let r274 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b116c); };
            c.w64(0, t275);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1173); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b1174); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b117a); }
            return c.ret(0x0, 0x70008b117c);
        }
        196 => {
            return RegfileExit::fault(0x70008b117d);
        }
        197 => {
            return RegfileExit::fault(0x70008b117e);
        }
        198 => {
            return RegfileExit::fault(0x70008b117f);
        }
        199 => {
            return RegfileExit::fault(0x70008b1180);
        }
        200 => {
            return RegfileExit::fault(0x70008b1181);
        }
        201 => {
            return RegfileExit::fault(0x70008b1182);
        }
        202 => {
            c.w32(8, 0x92u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b1189); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b118b); }
            let Some(t276) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b118c); };
            c.w64(3, t276);
            c.w32(0, 0x2u64);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1198); };
            c.w64(6, t277);
            return c.call(0x700086ed74, 0x70008b11a1, 0x70008b119c);
        }
        203 => {
            return RegfileExit::fault(0x70008b11a1);
        }
        204 => {
            return RegfileExit::fault(0x70008b11a2);
        }
        205 => {
            c.w32(8, 0xceu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b11a9); }
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b11ab); };
            c.w64(3, t278);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008b11bc, 0x70008b11b7);
        }
        206 => {
            return RegfileExit::fault(0x70008b11bc);
        }
        207 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b11bd); }
            let Some(t279) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b11c1); };
            c.w64(3, t279);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008b11d2, 0x70008b11cd);
        }
        208 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b11d2); };
            c.w64(6, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b11d6); };
            c.w64(0, t281);
            blk = 1; continue;
        }
        209 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008b11df); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008b11e3); }
            return c.call(0x70007febf4, 0x70008b11ec, 0x70008b11e7);
        }
        210 => {
            c.w64(8, c.r64(0));
            c.w32(0, 0x1u64);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b11f6); };
            c.w64(9, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b11fd); };
            c.w64(12, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b1201); };
            c.w64(1, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b1205); };
            c.w64(15, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b1209); };
            c.w64(2, t286);
            blk = 22; continue;
        }
        211 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008b1217); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x70008b121b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008b1220); }
            return c.call(0x70007febf4, 0x70008b1229, 0x70008b1224);
        }
        212 => {
            c.w64(11, c.r64(0));
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b122c); };
            c.w64(6, t287);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b1232); };
            c.w64(8, t288);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x70008b1236); }
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b123b); };
            c.w64(15, t289);
            blk = 57; continue;
        }
        213 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008b1244); }
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008b124d); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008b125a, 0x70008b1255);
        }
        214 => {
            c.w64(8, c.r64(0));
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b125d); };
            c.w64(6, t290);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b1263); };
            c.w64(2, t291);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008b1267); }
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b126c); };
            c.w64(12, t292);
            blk = 66; continue;
        }
        215 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008b1275); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008b1279); }
            return c.call(0x70007febf4, 0x70008b1282, 0x70008b127d);
        }
        216 => {
            c.w64(9, c.r64(0));
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1285); };
            c.w64(6, t293);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b128b); };
            c.w64(2, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b128f); };
            c.w64(8, t295);
            blk = 104; continue;
        }
        217 => {
            let Some(t296) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1298); };
            c.logic(t296 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 195; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b12a5); };
            c.w64(0, t297);
            blk = 145; continue;
        }
        219 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b12ae); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 195; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b12c1); };
            c.w64(0, t298);
            blk = 145; continue;
        }
        221 => {
            let Some(t299) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b12ca); };
            c.logic(t299 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 222; continue;
        }
        222 => {
            blk = 195; continue;
        }
        223 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b12dd); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 195; continue; }
            blk = 224; continue;
        }
        224 => {
            blk = 145; continue;
        }
        225 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008b12fb); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008b12ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008b1304); }
            return c.call(0x70007febf4, 0x70008b130d, 0x70008b1308);
        }
        226 => {
            c.w64(9, c.r64(0));
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1310); };
            c.w64(6, t300);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b1316); };
            c.w64(8, t301);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008b131a); }
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b131f); };
            c.w64(11, t302);
            blk = 176; continue;
        }
        227 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008b132d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x70008b1331); }
            return c.call(0x70007febf4, 0x70008b133b, 0x70008b1336);
        }
        228 => {
            c.w64(8, c.r64(0));
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b133e); };
            c.w64(6, t303);
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b1344); };
            c.w64(9, t304);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008b1348); }
            blk = 184; continue;
        }
        229 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b1352); }
            let Some(t305) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008b1353); };
            c.w64(3, t305);
            c.w32(0, 0x1u64);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b135f); };
            c.w64(6, t306);
            return c.call(0x700086ed74, 0x70008b1368, 0x70008b1363);
        }
        230 => {
            blk = 237; continue;
        }
        231 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b136a); }
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x70008b136e); };
            c.w64(3, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b1375); };
            c.w64(0, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1379); };
            c.w64(6, t309);
            return c.call(0x7000870914, 0x70008b1382, 0x70008b137d);
        }
        232 => {
            let Some(t310) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b1382); };
            c.cmp(t310, c.r32(0), 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t311) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b138b); };
            c.cmp(t311, c.r32(0), 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b1394); };
            if !c.push64(t312) { return RegfileExit::fault(0x70008b1394); }
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1397); };
            c.w64(6, t313);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008b13a8, 0x70008b13a3);
        }
        235 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b13a8); };
            if !c.push64(t314) { return RegfileExit::fault(0x70008b13a8); }
            let Some(t315) = c.ld64(c.r64(13).wrapping_add(0x2190)) else { return RegfileExit::fault(0x70008b13ab); };
            c.w64(3, t315);
            c.w32(0, 0x1u64);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b13b7); };
            c.w64(6, t316);
            return c.call(0x700086ed74, 0x70008b13c0, 0x70008b13bb);
        }
        236 => {
            return RegfileExit::fault(0x70008b13c0);
        }
        237 => {
            return RegfileExit::fault(0x70008b13c1);
        }
        238 => {
            let Some(t317) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x70008b13c2); };
            c.w64(3, t317);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b13cb); };
            c.w64(6, t318);
            return c.call(0x700086ed74, 0x70008b13d4, 0x70008b13cf);
        }
        239 => {
            return RegfileExit::fault(0x70008b13d4);
        }
        240 => {
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b13d5); };
            if !c.push64(t319) { return RegfileExit::fault(0x70008b13d5); }
            let Some(t320) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x70008b13d8); };
            c.w64(3, t320);
            c.w32(0, 0x1u64);
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b13e4); };
            c.w64(6, t321);
            return c.call(0x700086ed74, 0x70008b13ed, 0x70008b13e8);
        }
        241 => {
            return RegfileExit::fault(0x70008b13ed);
        }
        242 => {
            return RegfileExit::jump(0x70008b13f0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b13f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b13fc); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1409); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b140f); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1415); };
            c.w32(7, t5);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xb1u64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return RegfileExit::fault(0x70008b1427);
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x2cb)) else { return RegfileExit::fault(0x70008b1428); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b1431); };
            c.w64(7, t8);
            let Some(t9) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b1438); };
            c.w64(8, t9);
            c.w64(9, c.r64(8).wrapping_add(0x4c));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b143f); };
            c.w64(11, t10);
            let Some(t11) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008b1446); };
            c.cmp(t11, c.r64(9), 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008b144f); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008b1456); };
            c.w64(8, t12);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b145d); }
            c.w32(8, 0x22u64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b1467); }
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b146b); };
            c.w32(8, t13);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(8, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b1482); }
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1486); };
            c.w32(9, t14);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 38; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b149a); }
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b149e); };
            c.w32(9, t15);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(9, 0x8u64);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b14b8); }
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b14bc); };
            c.w32(9, t16);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_be() { blk = 40; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(9, 0x2u64);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b14d6); }
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b14da); };
            c.w32(9, t17);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 41; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008b14ee); }
            c.w64(8, c.r64(7).wrapping_add(0x4b));
            c.w64(9, c.r64(7).wrapping_add(0x1b));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_e() { blk = 29; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b1503); };
            c.w64(11, t18);
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(9), c.r32(11)) { return RegfileExit::fault(0x70008b150a); }
            let r19 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r19);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ne() { blk = 13; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 29; continue;
        }
        15 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 17; continue;
        }
        16 => {
            c.w32(1, 0x1u64);
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 50; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b152d); };
            c.w32(1, t20);
            let r21 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r21);
            let Some(t22) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1533); };
            c.w32(7, t22);
            let r23 = c.sub(c.r32(7), 0xbau64, 32);
            c.w32(7, r23);
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 49; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t24) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1549); };
            c.w32(7, t24);
            let r25 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b154f); };
            c.w32(7, t26);
            let r27 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r27);
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1555); };
            c.w32(8, t28);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x8cu64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x70008b1568);
        }
        21 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008b1569); };
            c.w32(7, t29);
            let r30 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r30);
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b1572); };
            c.w32(7, t31);
            let r32 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r32);
            let Some(t33) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b1578); };
            c.cmp(t33, c.r32(0), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008b157d); };
            c.w32(1, t34);
            let r35 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r35);
            c.w32(7, c.r32(1));
            let r36 = c.r32(7) & 0x3u64;
            c.w32(7, r36);
            c.logic(r36, 32);
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 23; continue;
        }
        23 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t37) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b159a); };
            c.w32(7, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xe08)) else { return RegfileExit::fault(0x70008b159d); };
            c.cmp(t38, c.r32(7), 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b15aa); };
            c.w32(1, t39);
            let r40 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r40);
            let Some(t41) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b15b0); };
            c.cmp(t41, c.r32(1), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 27; continue;
        }
        27 => {
            let r42 = c.r64(1) & 0xfffffffffffffffdu64;
            c.w64(1, r42);
            c.logic(r42, 64);
            c.w64(7, c.r64(1));
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b15cd); };
            c.w64(0, t43);
            c.w64(1, c.r64(7));
            c.w64(7, c.r64(0));
            blk = 29; continue;
        }
        29 => {
            let Some(t44) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b15da); };
            c.w32(8, t44);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b15df); };
            c.w64(9, t45);
            let Some(t46) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b15e6); };
            c.w64(11, t46);
            c.w64(2, c.r64(8).wrapping_mul(4));
            c.w64(8, c.r64(2).wrapping_add(c.r64(11)));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b15f5); };
            c.w64(12, t47);
            let Some(t48) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b15fc); };
            c.cmp(t48, c.r64(8), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(9), c.r64(8)) { return RegfileExit::fault(0x70008b1606); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 31; continue;
        }
        31 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b1611); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008b1615); }
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b1619); };
            c.w64(1, t49);
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008b1620); }
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b162a); };
            c.w64(1, t50);
            blk = 33; continue;
        }
        33 => {
            let r51 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r51);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008b1635); }
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_ne() { blk = 33; continue; }
            blk = 34; continue;
        }
        34 => {
            c.w64(0, c.r64(8));
            return c.ret(0x0, 0x70008b1642);
        }
        35 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1643); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b1647); }
            let r52 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r52);
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008b1652); }
            return c.call(0x70007febf4, 0x70008b165b, 0x70008b1656);
        }
        36 => {
            c.w64(7, c.r64(0));
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b165e); };
            c.w64(1, t53);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1665); }
            blk = 6; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008b166b);
        }
        38 => {
            return RegfileExit::fault(0x70008b166c);
        }
        39 => {
            return RegfileExit::fault(0x70008b166d);
        }
        40 => {
            return RegfileExit::fault(0x70008b166e);
        }
        41 => {
            return RegfileExit::fault(0x70008b166f);
        }
        42 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1670); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b1674); }
            let r54 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r54);
            blk = 51; continue;
        }
        43 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b167f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b1683); }
            let r55 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r55);
            blk = 51; continue;
        }
        44 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b168b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b168f); }
            let r56 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r56);
            blk = 51; continue;
        }
        45 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1697); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b169b); }
            let r57 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r57);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008b16a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008b16a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008b16a9); }
            return c.call(0x70007febf4, 0x70008b16b2, 0x70008b16ad);
        }
        46 => {
            c.w64(8, c.r64(0));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b16b5); };
            c.w64(7, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b16b9); };
            c.w64(1, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b16bd); };
            c.w64(2, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b16c4); }
            blk = 31; continue;
        }
        47 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b16ca); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b16ce); }
            let r61 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r61);
            blk = 51; continue;
        }
        48 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b16d6); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b16da); }
            let r62 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r62);
            blk = 51; continue;
        }
        49 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b16e2); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b16e6); }
            let r63 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r63);
            blk = 51; continue;
        }
        50 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b16ee); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b16f2); }
            let r64 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r64);
            blk = 51; continue;
        }
        51 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b16f8); }
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b16f9); };
            c.w64(1, t65);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b1700); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2410)) else { return RegfileExit::fault(0x70008b1701); };
            c.w64(3, t66);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b1712, 0x70008b170d);
        }
        52 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1715); }
            return c.ret(0x0, 0x70008b1716);
        }
        53 => {
            return RegfileExit::jump(0x70008b1718);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1734); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b1738); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1739); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b173a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b173f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b1745); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b174d); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1753); };
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
            return c.call(0x70008bd5f4, 0x70008b1775, 0x70008b1770);
        }
        5 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1775); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b177c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x10, 0x70008b1783);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b1786); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b178d); }
            return c.ret(0x0, 0x70008b178f);
        }
        8 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b1790); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2440)) else { return RegfileExit::fault(0x70008b1791); };
            c.w64(3, t7);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008b17a2, 0x70008b179d);
        }
        9 => {
            blk = 5; continue;
        }
        10 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b17a4); };
            c.w64(0, t8);
            blk = 5; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b17ad); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b17b1); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b17bf, 0x70008b17ba);
        }
        12 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b17bf); };
            c.w64(6, t10);
            blk = 1; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008b17c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b17d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b17d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b17d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b17d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b17da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b17df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b17e5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b17ed); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b17f3); };
            c.w32(2, t5);
            c.w32(1, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 11; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b1810); };
            c.w64(3, t6);
            return c.call(0x70008bd914, 0x70008b181c, 0x70008b1817);
        }
        5 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b181c); };
            c.w64(0, t7);
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1820); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1827); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return c.ret(0x10, 0x70008b182e);
        }
        8 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b1831); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b1838); }
            return c.ret(0x0, 0x70008b183a);
        }
        9 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b183b); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x23d8)) else { return RegfileExit::fault(0x70008b183c); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008b184d, 0x70008b1848);
        }
        10 => {
            blk = 6; continue;
        }
        11 => {
            blk = 6; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b1851); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b1855); };
            c.w64(3, t10);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b1863, 0x70008b185e);
        }
        13 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1863); };
            c.w64(6, t11);
            blk = 1; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008b1870);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1874); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b1878); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1879); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b187a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b187f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b1885); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b188d); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1893); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b189b); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b18a1); };
            c.cmp(t8, 0xbcu64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.call(0x70008bca74, 0x70008b18ae, 0x70008b18a9);
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b18ae); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b18b5); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x10, 0x70008b18bc);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b18bf); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b18c6); }
            return c.ret(0x0, 0x70008b18c8);
        }
        8 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b18c9); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x23e0)) else { return RegfileExit::fault(0x70008b18ca); };
            c.w64(3, t10);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008b18db, 0x70008b18d6);
        }
        9 => {
            blk = 5; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b18dd); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b18e1); };
            c.w64(3, t11);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b18ef, 0x70008b18ea);
        }
        11 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b18ef); };
            c.w64(6, t12);
            blk = 1; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b18f5); }
            return c.call(0x70008a2054, 0x70008b18fe, 0x70008b18f9);
        }
        13 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b18fe); };
            c.w64(6, t13);
            blk = 3; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008b1908);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1914); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b1918); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1919); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b191a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b191f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b1929); };
            c.w64(0, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b192d); };
            c.cmp(t3, c.r32(0), 32);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b193a); };
            c.cmp(t4, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            let r5 = c.r32(2) | c.r32(7);
            c.w32(2, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008b1951); };
            c.w64(3, t6);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b195e); };
            c.cmp(t7, c.r32(3), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1967); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b196d); };
            c.w32(2, t10);
            let r11 = c.sub(c.r32(2), 0xbau64, 32);
            c.w32(2, r11);
            c.cmp(c.r32(2), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1983); };
            c.w32(2, t12);
            let r13 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r13);
            let Some(t14) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1989); };
            c.w32(2, t14);
            c.w32(7, c.r64(2).wrapping_sub(0xba));
            c.cmp(c.r32(7), 0x78eu64, 32);
            if c.cc_a() { blk = 15; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0xbcu64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b19a6); };
            c.w64(1, t15);
            return c.call(0x70008bead4, 0x70008b19b2, 0x70008b19ad);
        }
        9 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b19b2); };
            c.w64(0, t16);
            blk = 10; continue;
        }
        10 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b19b6); };
            c.w64(1, t17);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b19bd); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x18, 0x70008b19c4);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b19c7); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b19ce); }
            return c.ret(0x0, 0x70008b19d0);
        }
        13 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b19d1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b19d2); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x23f0)) else { return RegfileExit::fault(0x70008b19d3); };
            c.w64(3, t18);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b19e4, 0x70008b19df);
        }
        14 => {
            blk = 10; continue;
        }
        15 => {
            blk = 10; continue;
        }
        16 => {
            c.w32(2, 0xf8u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b19ed); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b19ee); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b19ef); };
            c.w64(3, t19);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b1a00, 0x70008b19fb);
        }
        17 => {
            return RegfileExit::fault(0x70008b1a00);
        }
        18 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b1a06); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b1a07); };
            c.w64(2, t20);
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x223)) else { return RegfileExit::fault(0x70008b1a0e); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b1a17); }
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b1a18); };
            c.w64(3, t23);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b1a29, 0x70008b1a24);
        }
        19 => {
            return RegfileExit::fault(0x70008b1a29);
        }
        20 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b1a2a); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b1a2e); };
            c.w64(3, t24);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b1a3c, 0x70008b1a37);
        }
        21 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1a3c); };
            c.w64(6, t25);
            blk = 1; continue;
        }
        22 => {
            return RegfileExit::jump(0x70008b1a48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1a54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1a54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b1a58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1a59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b1a5a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b1a5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b1a65); };
            c.w64(0, t2);
            return c.call(0x7000856b94, 0x70008b1a6e, 0x70008b1a69);
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1a6e); };
            c.w64(1, t3);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1a75); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            return c.ret(0x8, 0x70008b1a7c);
        }
        4 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b1a7f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b1a86); }
            return c.ret(0x0, 0x70008b1a88);
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b1a89); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b1a8d); };
            c.w64(3, t4);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b1a9b, 0x70008b1a96);
        }
        6 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1a9b); };
            c.w64(6, t5);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008b1aa4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1ab4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1ab4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b1ab8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1ab9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b1aba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b1abf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b1ac5); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1acd); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1ad3); };
            c.cmp(t5, 0xbau64, 16);
            if c.cc_b() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1adb); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1ae2); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x8, 0x70008b1ae9);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b1aec); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b1af3); }
            return c.ret(0x0, 0x70008b1af5);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b1af6); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b1afa); };
            c.w64(3, t7);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b1b08, 0x70008b1b03);
        }
        7 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1b08); };
            c.w64(6, t8);
            blk = 1; continue;
        }
        8 => {
            return c.call(0x70008a2054, 0x70008b1b13, 0x70008b1b0e);
        }
        9 => {
            blk = 3; continue;
        }
        10 => {
            return RegfileExit::jump(0x70008b1b18);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1b34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1b34); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b1b38); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1b39); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b1b3a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b1b3f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b1b43); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b1b4d); };
            c.w64(0, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b1b51); };
            c.cmp(t3, c.r32(0), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b1b5a); };
            c.cmp(t4, c.r32(0), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xaf0)) else { return RegfileExit::fault(0x70008b1b63); };
            c.w64(3, t5);
            return c.call(0x7000870914, 0x70008b1b6f, 0x70008b1b6a);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b1b6f); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008b1b6f); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1b72); };
            c.w64(6, t7);
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2274, 0x70008b1b83, 0x70008b1b7e);
        }
        5 => {
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1b83); };
            c.w64(1, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1b8a); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return c.ret(0x8, 0x70008b1b91);
        }
        7 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b1b94); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b1b9b); }
            return c.ret(0x0, 0x70008b1b9d);
        }
        8 => {
            c.w32(2, 0x36u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b1ba3); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b1ba4); };
            c.w64(2, t9);
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x227)) else { return RegfileExit::fault(0x70008b1bab); };
            c.w32(2, t10);
            let r11 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r11);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b1bb4); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b1bb5); };
            c.w64(3, t12);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b1bc6, 0x70008b1bc1);
        }
        9 => {
            return RegfileExit::fault(0x70008b1bc6);
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b1bc7); };
            c.w64(3, t13);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b1bd5, 0x70008b1bd0);
        }
        11 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1bd5); };
            c.w64(6, t14);
            blk = 1; continue;
        }
        12 => {
            return RegfileExit::jump(0x70008b1be0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1bf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1bf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b1bf8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008b1bfe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008b1c02); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008b1c06); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b1c0a); };
            c.w64(2, t1);
            let Some(t2) = c.ld8(c.r64(2)) else { return RegfileExit::fault(0x70008b1c11); };
            c.logic(t2 & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1c1a); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b1c20); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1c26); };
            c.w32(1, t7);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x91u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b1c38);
        }
        3 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x24b)) else { return RegfileExit::fault(0x70008b1c39); };
            c.w32(7, t8);
            let r9 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r9);
            let Some(t10) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1c42); };
            c.w32(2, t10);
            let r11 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r11);
            let Some(t12) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b1c48); };
            c.logic(t12 & 0x2u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1c4e); };
            c.w32(2, t13);
            let r14 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r14);
            let Some(t15) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1c54); };
            c.w32(2, t15);
            c.cmp(c.r16(2), 0x83du64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(2, c.r64(0));
            blk = 8; continue;
        }
        6 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0x6bu64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t16) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1c6c); };
            c.w32(2, t16);
            let r17 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r17);
            let Some(t18) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1c72); };
            c.w32(1, t18);
            let r19 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r19);
            let Some(t20) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1c78); };
            c.cmp(t20, 0x83du64, 16);
            if c.cc_ne() { blk = 10; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b1c80); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b1c81); };
            c.w64(2, t21);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b1c88); }
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2134, 0x70008b1c96, 0x70008b1c91);
        }
        9 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b1c96); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1c9a); };
            c.w64(6, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1c9e); };
            c.w64(0, t24);
            blk = 10; continue;
        }
        10 => {
            c.w64(2, c.r64(0));
            let Some(t25) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b1ca5); };
            c.w32(1, t25);
            let r26 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r26);
            c.w64(8, c.r64(3));
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b1cae); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r27 = c.r64(9) & c.r64(2);
            c.w64(9, r27);
            c.logic(r27, 64);
            let Some(t28) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1cc2); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t29) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b1ccd); };
            c.w32(9, t29);
            let r30 = c.r64(9) & 0xfffffffffffffff9u64;
            c.w64(9, r30);
            c.logic(r30, 64);
            let r31 = c.r64(9) | 0x2u64;
            c.w64(9, r31);
            c.logic(r31, 64);
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b1cd9); }
            c.w64(9, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(1));
            blk = 16; continue;
        }
        13 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1ce7); };
            c.w32(11, t32);
            let r33 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r33);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008b1cee); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r34 = c.r64(12) & c.r64(9);
            c.w64(12, r34);
            c.logic(r34, 64);
            let Some(t35) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1d01); };
            c.logic(t35 & 0x4u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(1, c.r64(9));
            c.w64(9, c.r64(11));
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 13; continue;
        }
        18 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xd20)) else { return RegfileExit::fault(0x70008b1d1a); };
            c.w64(15, t36);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008b1d21); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b1d25); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r37 = c.r64(15) & c.r64(1);
            c.w64(15, r37);
            c.logic(r37, 64);
            let Some(t38) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1d39); };
            c.logic(t38 & 0x4u64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 20; continue;
        }
        20 => {
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008b1d44); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r39 = c.r64(15) & c.r64(1);
            c.w64(15, r39);
            c.logic(r39, 64);
            let Some(t40) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1d58); };
            c.logic(t40 & 0x4u64, 8);
            if c.cc_ne() { blk = 52; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(0, c.r64(1));
            c.w64(6, c.r64(12));
            return c.call(0x700081f374, 0x70008b1d6e, 0x70008b1d69);
        }
        23 => {
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b1d6e); };
            c.w64(1, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b1d72); };
            c.w64(8, t42);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t43) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1d80); };
            c.w32(11, t43);
            let r44 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r44);
            let Some(t45) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b1d87); };
            c.w32(12, t45);
            let r46 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1d8e); };
            c.w32(15, t47);
            let r48 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r48);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008b1d95); }
            blk = 26; continue;
        }
        26 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 30; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t49) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1da8); };
            c.w32(3, t49);
            let r50 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r50);
            let Some(t51) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1db0); };
            c.cmp(t51, 0x80bu64, 16);
            if c.cc_ne() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t52) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b1db8); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            blk = 26; continue;
        }
        29 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 31; continue;
        }
        30 => {
            c.w32(3, 0x1u64);
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t54) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1dd0); };
            c.w32(3, t54);
            let r55 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r55);
            let Some(t56) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1dd8); };
            c.cmp(t56, 0xbcu64, 16);
            if c.cc_ne() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t57) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1de0); };
            c.w32(12, t57);
            let r58 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r58);
            blk = 26; continue;
        }
        34 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t59) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1def); };
            c.w32(0, t59);
            let r60 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r60);
            let Some(t61) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1df7); };
            c.w32(0, t61);
            let r62 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r62);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t63) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b1e05); };
            c.w32(12, t63);
            let r64 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r64);
            blk = 18; continue;
        }
        37 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t65) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1e24); };
            c.w32(0, t65);
            let r66 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r66);
            let Some(t67) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1e2b); };
            c.cmp(t67, 0x80bu64, 16);
            if c.cc_ne() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t68) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b1e33); };
            c.w32(15, t68);
            let r69 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r69);
            blk = 37; continue;
        }
        40 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        41 => {
            c.w32(0, 0x1u64);
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t70) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1e4a); };
            c.w32(0, t70);
            let r71 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r71);
            let Some(t72) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1e51); };
            c.cmp(t72, 0xbcu64, 16);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t73) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1e59); };
            c.w32(15, t73);
            let r74 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r74);
            blk = 37; continue;
        }
        45 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1e68); };
            c.w32(12, t75);
            let r76 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r76);
            let Some(t77) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b1e6f); };
            c.w32(12, t77);
            let r78 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r78);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t79) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b1e82); };
            c.w32(12, t79);
            let r80 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r80);
            blk = 18; continue;
        }
        48 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1e8e); };
            c.w64(12, t81);
            blk = 18; continue;
        }
        49 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b1e97); };
            c.w64(0, t82);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b1ea1); }
            return c.ret(0x0, 0x70008b1ea2);
        }
        50 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b1ea3); }
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008b1ea4); };
            c.w64(3, t83);
            c.w32(0, 0x1u64);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b1eb0); };
            c.w64(6, t84);
            return c.call(0x700086ed74, 0x70008b1eb9, 0x70008b1eb4);
        }
        51 => {
            blk = 9; continue;
        }
        52 => {
            c.w32(12, c.r32(12));
            let r85 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r85);
            c.w64(15, 0xfffffffffffc0000u64);
            let r86 = c.r64(15) & c.r64(12);
            c.w64(15, r86);
            c.logic(r86, 64);
            let Some(t87) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1ece); };
            c.logic(t87 & 0x2u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(0, c.r64(1).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1edd); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b1ede); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008b1eeb, 0x70008b1ee6);
        }
        54 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b1eeb); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b1eed); }
            blk = 22; continue;
        }
        55 => {
            c.w32(8, c.r32(8));
            let r88 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r88);
            c.w64(15, 0xfffffffffffc0000u64);
            let r89 = c.r64(15) & c.r64(8);
            c.w64(15, r89);
            c.logic(r89, 64);
            let Some(t90) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1f03); };
            c.logic(t90 & 0x2u64, 8);
            if c.cc_e() { blk = 20; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(0, c.r64(1).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1f12); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b1f13); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008b1f20, 0x70008b1f1b);
        }
        57 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b1f20); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b1f22); }
            blk = 20; continue;
        }
        58 => {
            c.w32(1, c.r32(1));
            let r91 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r91);
            c.w64(12, 0xfffffffffffc0000u64);
            let r92 = c.r64(12) & c.r64(1);
            c.w64(12, r92);
            c.logic(r92, 64);
            let Some(t93) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1f37); };
            c.logic(t93 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(15, c.r64(9).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1f47); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b1f48); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b1f55, 0x70008b1f50);
        }
        60 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b1f55); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b1f57); }
            blk = 15; continue;
        }
        61 => {
            c.w32(8, c.r32(8));
            let r94 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r94);
            c.w64(9, 0xfffffffffffc0000u64);
            let r95 = c.r64(9) & c.r64(8);
            c.w64(9, r95);
            c.logic(r95, 64);
            let Some(t96) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b1f6d); };
            c.logic(t96 & 0x2u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(11, c.r64(2).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b1f7c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b1f7d); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008b1f8a, 0x70008b1f85);
        }
        63 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b1f8a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b1f8c); }
            blk = 12; continue;
        }
        64 => {
            return RegfileExit::jump(0x70008b1f94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b1fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b1fb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b1fb8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b1fbe); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(2)) else { return RegfileExit::fault(0x70008b1fc5); };
            c.w32(2, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008b1fc7); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b1fcb); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008b1fcf); }
            c.logic(c.r8(2) & 0xeu64, 8);
            if c.cc_ne() { blk = 55; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b1fdc); };
            c.w32(8, t3);
            c.logic(c.r8(8) & 0x8u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b1ff3); };
            c.w32(2, t4);
            let r5 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r5);
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b1ff9); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b1fff); };
            c.w32(1, t8);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x91u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008b2011);
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(2).wrapping_add(0x24b)) else { return RegfileExit::fault(0x70008b2012); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b201b); };
            c.w32(2, t11);
            let r12 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r12);
            let Some(t13) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b2021); };
            c.logic(t13 & 0x2u64, 8);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2027); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r15);
            let Some(t16) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b202d); };
            c.w32(2, t16);
            c.cmp(c.r16(2), 0x83du64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(2, c.r64(0));
            blk = 10; continue;
        }
        8 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0x6bu64, 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2045); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r18);
            let Some(t19) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b204b); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2051); };
            c.cmp(t21, 0x83du64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b2059); }
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b205a); };
            c.w64(2, t22);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b2061); }
            c.w32(0, 0x1u64);
            c.w64(2, c.r64(0));
            return c.call(0x70007f2134, 0x70008b206f, 0x70008b206a);
        }
        11 => {
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b206f); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2073); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2077); };
            c.w64(6, t25);
            blk = 12; continue;
        }
        12 => {
            c.w64(2, c.r64(0));
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b207e); };
            c.w32(1, t26);
            let r27 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r27);
            c.w64(8, c.r64(3));
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b2087); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r28 = c.r64(9) & c.r64(2);
            c.w64(9, r28);
            c.logic(r28, 64);
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b209b); };
            c.logic(t29 & 0x4u64, 8);
            if c.cc_ne() { blk = 71; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b20a6); };
            c.w32(9, t30);
            let r31 = c.r64(9) & 0xfffffffffffffff9u64;
            c.w64(9, r31);
            c.logic(r31, 64);
            let r32 = c.r64(9) | 0x4u64;
            c.w64(9, r32);
            c.logic(r32, 64);
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b20b2); }
            c.w64(9, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(1));
            blk = 18; continue;
        }
        15 => {
            let Some(t33) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b20c0); };
            c.w32(11, t33);
            let r34 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r34);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008b20c7); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r35 = c.r64(12) & c.r64(9);
            c.w64(12, r35);
            c.logic(r35, 64);
            let Some(t36) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b20da); };
            c.logic(t36 & 0x4u64, 8);
            if c.cc_ne() { blk = 68; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(1, c.r64(9));
            c.w64(9, c.r64(11));
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 15; continue;
        }
        20 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xd28)) else { return RegfileExit::fault(0x70008b20f7); };
            c.w64(0, t37);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008b20fe); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b2101); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r38 = c.r64(0) & c.r64(1);
            c.w64(0, r38);
            c.logic(r38, 64);
            let Some(t39) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b2115); };
            c.logic(t39 & 0x4u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 22; continue;
        }
        22 => {
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008b211f); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r40 = c.r64(0) & c.r64(1);
            c.w64(0, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b2133); };
            c.logic(t41 & 0x4u64, 8);
            if c.cc_ne() { blk = 62; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008b213d); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r42 = c.r64(0) & c.r64(1);
            c.w64(0, r42);
            c.logic(r42, 64);
            let Some(t43) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b2151); };
            c.logic(t43 & 0x4u64, 8);
            if c.cc_ne() { blk = 59; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(0, c.r64(1));
            c.w64(6, c.r64(15));
            return c.call(0x700081f374, 0x70008b2166, 0x70008b2161);
        }
        27 => {
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2166); };
            c.w64(1, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b216a); };
            c.w64(8, t45);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 53; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2178); };
            c.w32(11, t46);
            let r47 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r47);
            let Some(t48) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b217f); };
            c.w32(12, t48);
            let r49 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b2186); };
            c.w32(15, t50);
            let r51 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(11)) { return RegfileExit::fault(0x70008b218d); }
            c.w64(0, c.r64(12));
            blk = 30; continue;
        }
        30 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t52) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b21a1); };
            c.w32(7, t52);
            let r53 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r53);
            let Some(t54) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b21a7); };
            c.cmp(t54, 0x80bu64, 16);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b21af); };
            c.w32(0, t55);
            let r56 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r56);
            blk = 30; continue;
        }
        33 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        34 => {
            c.w32(7, 0x1u64);
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b21c5); };
            c.w32(7, t57);
            let r58 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r58);
            let Some(t59) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b21cb); };
            c.cmp(t59, 0xbcu64, 16);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t60) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b21d3); };
            c.w32(0, t60);
            let r61 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r61);
            blk = 30; continue;
        }
        38 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t62) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b21e0); };
            c.w32(3, t62);
            let r63 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r63);
            let Some(t64) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b21e6); };
            c.w32(3, t64);
            let r65 = c.sub(c.r32(3), 0x80cu64, 32);
            c.w32(3, r65);
            c.cmp(c.r32(3), 0xeu64, 32);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t66) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b21f5); };
            c.w32(15, t66);
            let r67 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r67);
            blk = 20; continue;
        }
        41 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t68) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2210); };
            c.w32(3, t68);
            let r69 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r69);
            let Some(t70) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2217); };
            c.cmp(t70, 0x80bu64, 16);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t71) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b221f); };
            c.w32(15, t71);
            let r72 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r72);
            blk = 41; continue;
        }
        44 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        45 => {
            c.w32(3, 0x1u64);
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t73) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2236); };
            c.w32(3, t73);
            let r74 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r74);
            let Some(t75) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b223d); };
            c.cmp(t75, 0xbcu64, 16);
            if c.cc_ne() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t76) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2245); };
            c.w32(15, t76);
            let r77 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r77);
            blk = 41; continue;
        }
        49 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t78) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2253); };
            c.w32(0, t78);
            let r79 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r79);
            let Some(t80) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b225a); };
            c.w32(0, t80);
            let r81 = c.sub(c.r32(0), 0x80cu64, 32);
            c.w32(0, r81);
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t82) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b2268); };
            c.w32(15, t82);
            let r83 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r83);
            blk = 20; continue;
        }
        52 => {
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2274); };
            c.w64(15, t84);
            blk = 20; continue;
        }
        53 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b227d); };
            c.w64(0, t85);
            blk = 54; continue;
        }
        54 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b2287); }
            return c.ret(0x0, 0x70008b2288);
        }
        55 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b2289); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b228a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b228b); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x25c8)) else { return RegfileExit::fault(0x70008b228c); };
            c.w64(3, t86);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008b229d, 0x70008b2298);
        }
        56 => {
            blk = 54; continue;
        }
        57 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b229f); }
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008b22a0); };
            c.w64(3, t87);
            c.w32(0, 0x1u64);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b22ac); };
            c.w64(6, t88);
            return c.call(0x700086ed74, 0x70008b22b5, 0x70008b22b0);
        }
        58 => {
            blk = 11; continue;
        }
        59 => {
            c.w32(12, c.r32(12));
            let r89 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r89);
            c.w64(0, 0xfffffffffffc0000u64);
            let r90 = c.r64(0) & c.r64(12);
            c.w64(0, r90);
            c.logic(r90, 64);
            let Some(t91) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b22ca); };
            c.logic(t91 & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(3, c.r64(1).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b22d8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b22d9); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008b22e6, 0x70008b22e1);
        }
        61 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b22e6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b22e8); }
            blk = 26; continue;
        }
        62 => {
            c.w32(15, c.r32(15));
            let r92 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r92);
            c.w64(0, 0xfffffffffffc0000u64);
            let r93 = c.r64(0) & c.r64(15);
            c.w64(0, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b22fe); };
            c.logic(t94 & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(3, c.r64(1).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b230c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b230d); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008b231a, 0x70008b2315);
        }
        64 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b231a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b231c); }
            blk = 24; continue;
        }
        65 => {
            c.w32(8, c.r32(8));
            let r95 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r95);
            c.w64(0, 0xfffffffffffc0000u64);
            let r96 = c.r64(0) & c.r64(8);
            c.w64(0, r96);
            c.logic(r96, 64);
            let Some(t97) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b2332); };
            c.logic(t97 & 0x2u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(3, c.r64(1).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b2340); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b2341); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008b234e, 0x70008b2349);
        }
        67 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b234e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b2350); }
            blk = 22; continue;
        }
        68 => {
            c.w32(1, c.r32(1));
            let r98 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r98);
            c.w64(12, 0xfffffffffffc0000u64);
            let r99 = c.r64(12) & c.r64(1);
            c.w64(12, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b2365); };
            c.logic(t100 & 0x2u64, 8);
            if c.cc_e() { blk = 17; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(15, c.r64(9).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b2375); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b2376); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b2383, 0x70008b237e);
        }
        70 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b2383); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b2385); }
            blk = 17; continue;
        }
        71 => {
            c.w32(8, c.r32(8));
            let r101 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r101);
            c.w64(9, 0xfffffffffffc0000u64);
            let r102 = c.r64(9) & c.r64(8);
            c.w64(9, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b239b); };
            c.logic(t103 & 0x2u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(11, c.r64(2).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b23aa); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b23ab); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008b23b8, 0x70008b23b3);
        }
        73 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b23b8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b23ba); }
            blk = 14; continue;
        }
        74 => {
            return RegfileExit::jump(0x70008b23c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b23d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b23d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b23d8); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008b23de); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008b23e2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(1, 0xc2u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b23ef); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b23f0); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b23f1); };
            c.w64(3, t1);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b2402, 0x70008b23fd);
        }
        2 => {
            return RegfileExit::fault(0x70008b2402);
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2403); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b2409); };
            c.logic(t4 & 0x40u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2413); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b2419); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b241f); };
            c.w32(8, t9);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008b2429); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008b2436);
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b2437); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b244a); };
            c.w64(9, t12);
            let Some(t13) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b2451); };
            c.w64(8, t13);
            c.w64(11, c.r64(8).wrapping_add(0x10));
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2458); };
            c.w64(12, t14);
            let Some(t15) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b245f); };
            c.cmp(t15, c.r64(11), 64);
            if c.cc_be() { blk = 154; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.st64(c.r64(9), c.r64(11)) { return RegfileExit::fault(0x70008b2469); }
            let r16 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r16);
            blk = 9; continue;
        }
        9 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xdf0)) else { return RegfileExit::fault(0x70008b2470); };
            c.w64(11, t17);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008b2477); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b247b); };
            c.w64(11, t18);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b2482); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008b2486); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b248a); }
            let Some(t19) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b248e); };
            c.w32(11, t19);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008b2498); }
            c.cmp(c.r64(11), 0x79u64, 64);
            if c.cc_a() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return RegfileExit::fault(0x70008b24a2);
        }
        11 => {
            let Some(t20) = c.ld32(c.r64(1).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b24a3); };
            c.w32(11, t20);
            c.w32(3, 0x8u64);
            c.w64(15, c.r64(3));
            let r21 = c.add(c.r64(3), 0xcu64, 64);
            c.w64(3, r21);
            if c.cc_o() { blk = 65; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t22) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b24bc); };
            c.w64(2, t22);
            c.w64(7, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t23) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b24c3); };
            c.cmp(t23, c.r64(7), 64);
            if c.cc_be() { blk = 156; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 156; continue; }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x70008b24da); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 15; continue;
        }
        15 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008b24e1); }
            c.w32(11, 0x6u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b24eb); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 17; continue;
        }
        16 => {
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008b24f4); }
            let r24 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r24);
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(15), 0x14u64, 64);
            if c.cc_ne() { blk = 16; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2503); };
            c.w32(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008b2513);
        }
        20 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b2514); };
            c.w64(15, t26);
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008b251b); }
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x70008b2525);
        }
        22 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b2526); };
            c.w64(15, t27);
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b252d); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008b2537);
        }
        24 => {
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b2538); }
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b253c); };
            c.w32(11, t28);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xbeu64, 64);
            if c.cc_a() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x70008b254f);
        }
        26 => {
            let Some(t29) = c.ld32(c.r64(1).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b2550); };
            c.w32(1, t29);
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x10c8)) else { return RegfileExit::fault(0x70008b2556); };
            c.w64(11, t30);
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b255d); };
            c.w32(15, t31);
            let r32 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r32);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(13).wrapping_add(0x63f0));
            let Some(t33) = c.ld64(c.r64(2).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b2574); };
            c.w64(15, t33);
            blk = 47; continue;
        }
        28 => {
            let Some(t34) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b257d); };
            c.w32(2, t34);
            let r35 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r35);
            let Some(t36) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2584); };
            c.w32(2, t36);
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_l() { blk = 35; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_l() { blk = 33; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(2), 0x85u64, 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(2), 0x9du64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(2), 0x9eu64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(2), 0x5cu64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(2), 0x5eu64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(2), 0x4au64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(2), 0x53u64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(2), 0x48u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(2), 0x49u64, 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t37) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b25d0); };
            c.w32(15, t37);
            blk = 47; continue;
        }
        41 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b25d6); };
            c.w64(15, t38);
            blk = 47; continue;
        }
        42 => {
            let Some(t39) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b25df); };
            c.w32(15, t39);
            blk = 47; continue;
        }
        43 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b25e5); };
            c.w64(15, t40);
            blk = 47; continue;
        }
        44 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b25ee); };
            c.w64(15, t41);
            blk = 47; continue;
        }
        45 => {
            let Some(t42) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b25f7); };
            c.w32(15, t42);
            blk = 47; continue;
        }
        46 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b25fd); };
            c.w64(15, t43);
            blk = 47; continue;
        }
        47 => {
            let Some(t44) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b2604); };
            c.w64(2, t44);
            c.w64(7, c.r64(2).wrapping_add(0x1c));
            let Some(t45) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b260b); };
            c.cmp(t45, c.r64(7), 64);
            if c.cc_be() { blk = 158; continue; }
            blk = 48; continue;
        }
        48 => {
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x70008b2615); }
            c.w64(9, c.r64(2).wrapping_add(0x1));
            blk = 49; continue;
        }
        49 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b261c); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b2620); };
            c.w64(1, t46);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008b2627); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008b262b); }
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b262f); };
            c.w64(1, t47);
            if !c.st32(c.r64(9).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008b2636); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b263a); }
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(3)) { return RegfileExit::fault(0x70008b263e); }
            if !c.st32(c.r64(9).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008b2642); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b2646); }
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b2648); };
            c.w64(1, t48);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b264f); }
            c.w64(7, c.r64(0));
            c.w64(2, c.r64(0));
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f7214, 0x70008b2663, 0x70008b265e);
        }
        50 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2663); };
            c.w64(1, t49);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008b2667); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r50 = c.r64(8) & c.r64(1);
            c.w64(8, r50);
            c.logic(r50, 64);
            let Some(t51) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b2678); };
            c.logic(t51 & 0x4u64, 8);
            if c.cc_ne() { blk = 172; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2683); };
            c.w32(8, t52);
            let r53 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r53);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t54) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2690); };
            c.w32(8, t54);
            let r55 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r55);
            let Some(t56) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b2697); };
            c.logic(t56 & 0x2u64, 8);
            if c.cc_e() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w32(9, 0x1u64);
            blk = 56; continue;
        }
        55 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        56 => {
            c.w32(8, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t57) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b26b2); };
            c.w32(9, t57);
            let r58 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r58);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t59) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b26bf); };
            c.w32(9, t59);
            let r60 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r60);
            let Some(t61) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b26c6); };
            c.logic(t61 & 0x2u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w32(8, 0x1u64);
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(0, c.r64(1));
            blk = 62; continue;
        }
        62 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b26df); }
            return c.ret(0x0, 0x70008b26e0);
        }
        63 => {
            c.w32(1, 0xf2u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b26e6); }
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b26e7); };
            c.w64(3, t62);
            c.w32(0, 0x1u64);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b26f3); };
            c.w64(6, t63);
            return c.call(0x700086ed74, 0x70008b26fc, 0x70008b26f7);
        }
        64 => {
            return RegfileExit::fault(0x70008b26fc);
        }
        65 => {
            return RegfileExit::fault(0x70008b26fd);
        }
        66 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b26fe); };
            c.w32(8, t64);
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2702); };
            c.w64(9, t65);
            let Some(t66) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b2709); };
            c.w64(11, t66);
            c.w64(15, c.r64(11).wrapping_add(0x14));
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2710); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b2717); };
            c.cmp(t68, c.r64(15), 64);
            if c.cc_be() { blk = 160; continue; }
            blk = 67; continue;
        }
        67 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x70008b2721); }
            let r69 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r69);
            blk = 68; continue;
        }
        68 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b2728); }
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b272c); };
            c.w64(8, t70);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b2733); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b2737); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b273e); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008b2742); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b2746); }
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008b274a); }
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b274e); };
            c.w64(8, t71);
            let Some(t72) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x70008b2755); };
            c.w32(8, t72);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008b2758); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008b275c); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 81; continue; }
            blk = 69; continue;
        }
        69 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t73) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2774); };
            c.w32(0, t73);
            let r74 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r74);
            let Some(t75) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b277a); };
            c.w32(0, t75);
            let r76 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r76);
            let Some(t77) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2780); };
            c.w32(2, t77);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x8eu64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::fault(0x70008b2792);
        }
        72 => {
            let Some(t78) = c.ld32(c.r64(0).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008b2793); };
            c.w32(7, t78);
            let r79 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r79);
            let Some(t80) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b279c); };
            c.w32(0, t80);
            let r81 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r81);
            let Some(t82) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b27a2); };
            c.logic(t82 & 0x2u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b27a8); };
            c.w64(0, t83);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b27b3); };
            c.w32(2, t84);
            let r85 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r85);
            let Some(t86) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b27b9); };
            c.w32(2, t86);
            c.cmp(c.r16(2), 0x83du64, 16);
            if c.cc_ne() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, c.r16(2));
            c.cmp(c.r32(2), 0x83du64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            return RegfileExit::fault(0x70008b27cf);
        }
        77 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b27d0); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b27d1); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b27d3); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x70008b27de, 0x70008b27d9);
        }
        78 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b27de); };
            c.w64(6, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b27e2); };
            c.w64(11, t88);
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b27e6); };
            c.w64(9, t89);
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b27ed); };
            c.w64(12, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b27f4); };
            c.w64(1, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b27f8); };
            c.w64(3, t92);
            c.w32(15, 0);
            c.logic(0, 32);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b27ff); };
            c.w64(8, t93);
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(8) & 0x6u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b2809); }
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b280b); };
            c.w64(8, t94);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b2812); }
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x70008b2814); };
            c.w64(3, t95);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008b2828, 0x70008b2823);
        }
        81 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b2828); };
            c.w64(1, t96);
            let Some(t97) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b282c); };
            c.w32(8, t97);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x79u64, 64);
            if c.cc_a() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            return RegfileExit::fault(0x70008b283c);
        }
        83 => {
            let Some(t98) = c.ld32(c.r64(1).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b283d); };
            c.w32(8, t98);
            c.w32(9, 0x8u64);
            c.w64(11, c.r64(9));
            let r99 = c.add(c.r64(11), 0x14u64, 64);
            c.w64(11, r99);
            if c.cc_o() { blk = 151; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2857); };
            c.w64(12, t100);
            let Some(t101) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b285e); };
            c.w64(15, t101);
            c.w64(0, c.r64(15).wrapping_add(c.r64(11)));
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2866); };
            c.w64(3, t102);
            let Some(t103) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b286d); };
            c.cmp(t103, c.r64(0), 64);
            if c.cc_be() { blk = 164; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r64(11), 0x20000u64, 64);
            if c.cc_ae() { blk = 164; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008b2883); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 87; continue;
        }
        87 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b288b); }
            c.w32(8, 0xau64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b2895); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 89; continue;
        }
        88 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b289e); }
            let r104 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r104);
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(9), 0x1cu64, 64);
            if c.cc_ne() { blk = 88; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t105) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b28ad); };
            c.w32(8, t105);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            return RegfileExit::fault(0x70008b28bd);
        }
        92 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b28be); };
            c.w64(9, t106);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b28c5); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            return RegfileExit::fault(0x70008b28cf);
        }
        94 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b28d0); };
            c.w64(9, t107);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008b28d7); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_a() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            return RegfileExit::fault(0x70008b28e1);
        }
        96 => {
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b28e2); };
            c.w64(0, t108);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008b28e6); }
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_a() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            return RegfileExit::fault(0x70008b28f0);
        }
        98 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b28f1); };
            c.w64(9, t109);
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b28f8); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_a() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            return RegfileExit::fault(0x70008b2902);
        }
        100 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b2903); };
            c.w64(8, t110);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008b2907); }
            let Some(t111) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b290b); };
            c.w32(8, t111);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xbeu64, 64);
            if c.cc_a() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            return RegfileExit::fault(0x70008b291e);
        }
        102 => {
            let Some(t112) = c.ld32(c.r64(1).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b291f); };
            c.w32(1, t112);
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x10b8)) else { return RegfileExit::fault(0x70008b2925); };
            c.w64(8, t113);
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b292c); };
            c.w32(9, t114);
            let r115 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r115);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(13).wrapping_add(0x63f0));
            let Some(t116) = c.ld64(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b2943); };
            c.w64(9, t116);
            blk = 123; continue;
        }
        104 => {
            let Some(t117) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b294c); };
            c.w32(15, t117);
            let r118 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r118);
            let Some(t119) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2953); };
            c.w32(15, t119);
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_l() { blk = 111; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_l() { blk = 109; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(15), 0x9du64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(15), 0x9eu64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(15), 0x5eu64, 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_l() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(15), 0x53u64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(15), 0x48u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(15), 0x49u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t120) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b29ac); };
            c.w32(9, t120);
            blk = 123; continue;
        }
        117 => {
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b29b2); };
            c.w64(9, t121);
            blk = 123; continue;
        }
        118 => {
            let Some(t122) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b29bb); };
            c.w32(9, t122);
            blk = 123; continue;
        }
        119 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b29c1); };
            c.w64(9, t123);
            blk = 123; continue;
        }
        120 => {
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b29ca); };
            c.w64(9, t124);
            blk = 123; continue;
        }
        121 => {
            let Some(t125) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b29d3); };
            c.w32(9, t125);
            blk = 123; continue;
        }
        122 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b29d9); };
            c.w64(9, t126);
            blk = 123; continue;
        }
        123 => {
            let Some(t127) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b29e0); };
            c.w64(15, t127);
            c.w64(2, c.r64(15).wrapping_add(0x1c));
            let Some(t128) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b29e8); };
            c.cmp(t128, c.r64(2), 64);
            if c.cc_be() { blk = 166; continue; }
            blk = 124; continue;
        }
        124 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x70008b29f1); }
            let r129 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r129);
            blk = 125; continue;
        }
        125 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b29f9); }
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b29fd); };
            c.w64(2, t130);
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008b2a04); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008b2a08); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b2a0c); };
            c.w64(6, t131);
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(6)) { return RegfileExit::fault(0x70008b2a13); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b2a17); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b2a1b); }
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008b2a1f); }
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x10b0)) else { return RegfileExit::fault(0x70008b2a23); };
            c.w64(8, t132);
            let Some(t133) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2a2a); };
            c.w32(9, t133);
            let r134 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r134);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008b2a31); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(13).wrapping_add(0x63f0));
            let Some(t135) = c.ld64(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b2a45); };
            c.w64(9, t135);
            blk = 146; continue;
        }
        127 => {
            let Some(t136) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2a4e); };
            c.w32(7, t136);
            let r137 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r137);
            let Some(t138) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2a55); };
            c.w32(7, t138);
            c.cmp(c.r32(7), 0x5cu64, 32);
            if c.cc_l() { blk = 134; continue; }
            blk = 128; continue;
        }
        128 => {
            c.cmp(c.r32(7), 0x85u64, 32);
            if c.cc_l() { blk = 132; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r32(7), 0x85u64, 32);
            if c.cc_e() { blk = 140; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(7), 0x9du64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(7), 0x9eu64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(7), 0x5cu64, 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(7), 0x5eu64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 134; continue;
        }
        134 => {
            c.cmp(c.r32(7), 0x4au64, 32);
            if c.cc_l() { blk = 137; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(7), 0x4au64, 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(7), 0x53u64, 32);
            if c.cc_e() { blk = 143; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(7), 0x48u64, 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(7), 0x49u64, 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t139) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2aa1); };
            c.w32(9, t139);
            blk = 146; continue;
        }
        140 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b2aa7); };
            c.w64(9, t140);
            blk = 146; continue;
        }
        141 => {
            let Some(t141) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2ab0); };
            c.w32(9, t141);
            blk = 146; continue;
        }
        142 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b2ab6); };
            c.w64(9, t142);
            blk = 146; continue;
        }
        143 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b2abf); };
            c.w64(9, t143);
            blk = 146; continue;
        }
        144 => {
            let Some(t144) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b2ac8); };
            c.w32(9, t144);
            blk = 146; continue;
        }
        145 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b2ace); };
            c.w64(9, t145);
            blk = 146; continue;
        }
        146 => {
            let Some(t146) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b2ad5); };
            c.w64(7, t146);
            c.w64(15, c.r64(7).wrapping_add(0x1c));
            let Some(t147) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b2add); };
            c.cmp(t147, c.r64(15), 64);
            if c.cc_be() { blk = 168; continue; }
            blk = 147; continue;
        }
        147 => {
            if !c.st64(c.r64(12), c.r64(15)) { return RegfileExit::fault(0x70008b2ae6); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            blk = 148; continue;
        }
        148 => {
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b2aee); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008b2af2); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008b2af6); }
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(6)) { return RegfileExit::fault(0x70008b2afa); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b2afe); }
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b2b02); }
            if !c.st32(c.r64(15).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008b2b06); }
            let Some(t148) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b2b0a); };
            c.w64(1, t148);
            c.w64(8, c.r64(1).wrapping_add(0x10));
            let Some(t149) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b2b12); };
            c.cmp(t149, c.r64(8), 64);
            if c.cc_be() { blk = 170; continue; }
            blk = 149; continue;
        }
        149 => {
            if !c.st64(c.r64(12), c.r64(8)) { return RegfileExit::fault(0x70008b2b1b); }
            let r150 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r150);
            blk = 150; continue;
        }
        150 => {
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0xdf0)) else { return RegfileExit::fault(0x70008b2b23); };
            c.w64(8, t151);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b2b2a); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008b2b2e); }
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2b31); };
            c.w64(0, t152);
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008b2b35); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b2b38); }
            c.w64(0, c.r64(1));
            blk = 62; continue;
        }
        151 => {
            return RegfileExit::fault(0x70008b2b44);
        }
        152 => {
            c.w32(1, 0xc2u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b2b4a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b2b4b); }
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b2b4c); };
            c.w64(3, t153);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b2b5d, 0x70008b2b58);
        }
        153 => {
            return RegfileExit::fault(0x70008b2b5d);
        }
        154 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008b2b63); }
            return c.call(0x70007febb4, 0x70008b2b6c, 0x70008b2b67);
        }
        155 => {
            c.w64(8, c.r64(0));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2b6f); };
            c.w64(0, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2b73); };
            c.w64(6, t155);
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2b77); };
            c.w64(9, t156);
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2b7e); };
            c.w64(12, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b2b85); };
            c.w64(1, t158);
            blk = 9; continue;
        }
        156 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008b2b8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008b2b92); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008b2b9e, 0x70008b2b99);
        }
        157 => {
            c.w64(3, c.r64(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2ba1); };
            c.w64(0, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2ba5); };
            c.w64(6, t160);
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2ba9); };
            c.w64(9, t161);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2bb0); };
            c.w64(12, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b2bb7); };
            c.w64(1, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2bbb); };
            c.w64(8, t164);
            c.w32(15, 0x8u64);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b2bc5); };
            c.w64(11, t165);
            blk = 15; continue;
        }
        158 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008b2bd3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008b2bd7); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008b2bdb); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008b2bdf); }
            return c.call(0x70007febf4, 0x70008b2be8, 0x70008b2be3);
        }
        159 => {
            c.w64(9, c.r64(0));
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2beb); };
            c.w64(0, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2bef); };
            c.w64(6, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b2bf3); };
            c.w64(15, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b2bf7); };
            c.w64(3, t169);
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x10c8)) else { return RegfileExit::fault(0x70008b2bfb); };
            c.w64(11, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b2c02); };
            c.w64(1, t171);
            blk = 49; continue;
        }
        160 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008b2c10); }
            return c.call(0x70007febf4, 0x70008b2c19, 0x70008b2c14);
        }
        161 => {
            c.w64(11, c.r64(0));
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2c1c); };
            c.w64(6, t172);
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2c20); };
            c.w64(9, t173);
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2c27); };
            c.w64(12, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b2c2e); };
            c.w64(1, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b2c32); };
            c.w64(3, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2c36); };
            c.w64(8, t177);
            blk = 68; continue;
        }
        162 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b2c3f); }
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008b2c40); };
            c.w64(3, t178);
            c.w32(0, 0x1u64);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2c4c); };
            c.w64(6, t179);
            return c.call(0x700086ed74, 0x70008b2c55, 0x70008b2c50);
        }
        163 => {
            blk = 78; continue;
        }
        164 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008b2c5a); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008b2c66, 0x70008b2c61);
        }
        165 => {
            c.w64(11, c.r64(0));
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2c69); };
            c.w64(12, t180);
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2c70); };
            c.w64(3, t181);
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b2c77); };
            c.w64(1, t182);
            c.w32(9, 0x8u64);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2c81); };
            c.w64(8, t183);
            blk = 87; continue;
        }
        166 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008b2c8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008b2c93); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008b2c97); }
            return c.call(0x70007febf4, 0x70008b2ca0, 0x70008b2c9b);
        }
        167 => {
            c.w64(15, c.r64(0));
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2ca3); };
            c.w64(0, t184);
            let Some(t185) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2ca7); };
            c.w64(12, t185);
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2cae); };
            c.w64(3, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b2cb5); };
            c.w64(11, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b2cb9); };
            c.w64(1, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2cbd); };
            c.w64(9, t189);
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x10b8)) else { return RegfileExit::fault(0x70008b2cc1); };
            c.w64(8, t190);
            blk = 125; continue;
        }
        168 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008b2ccd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008b2cd1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008b2cd5); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008b2ce3, 0x70008b2cde);
        }
        169 => {
            c.w64(15, c.r64(0));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2ce6); };
            c.w64(0, t191);
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b2cea); };
            c.w64(12, t192);
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b2cf1); };
            c.w64(3, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b2cf8); };
            c.w64(9, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b2cfc); };
            c.w64(11, t195);
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0x10b0)) else { return RegfileExit::fault(0x70008b2d00); };
            c.w64(8, t196);
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b2d07); };
            c.w64(6, t197);
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b2d0e); };
            c.w64(2, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b2d15); };
            c.w64(1, t199);
            blk = 148; continue;
        }
        170 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008b2d23); }
            return c.call(0x70007febb4, 0x70008b2d2c, 0x70008b2d27);
        }
        171 => {
            c.w64(1, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b2d2f); };
            c.w64(15, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2d33); };
            c.w64(0, t201);
            blk = 150; continue;
        }
        172 => {
            c.w32(0, c.r32(0));
            let r202 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r202);
            c.w64(8, 0xfffffffffffc0000u64);
            let r203 = c.r64(8) & c.r64(0);
            c.w64(8, r203);
            c.logic(r203, 64);
            let Some(t204) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b2d4b); };
            c.logic(t204 & 0x2u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(9, c.r64(1).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b2d5a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b2d5b); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008b2d68, 0x70008b2d63);
        }
        174 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b2d68); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b2d6a); }
            blk = 52; continue;
        }
        175 => {
            return RegfileExit::jump(0x70008b2d74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b2d94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b2d94); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b2d98); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b2d99); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b2d9a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b2d9f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2da9); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b2db8);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b2db9); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008b2dc5);
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b2dc6); };
            c.w32(7, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b2dc9); };
            c.cmp(t6, c.r32(7), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b2dd2); };
            c.w64(7, t7);
            if !c.st32(c.r64(6).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008b2dd9); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_a() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x70008b2de2);
        }
        8 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b2de3); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b2de9); };
            c.w64(3, t10);
            return c.call(0x70008b1fb4, 0x70008b2df2, 0x70008b2ded);
        }
        9 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2df2); };
            c.w64(1, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b2df9); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x10, 0x70008b2e00);
        }
        11 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b2e03); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b2e0a); }
            return c.ret(0x0, 0x70008b2e0c);
        }
        12 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b2e0d); }
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b2e0e); };
            if !c.push64(t12) { return RegfileExit::fault(0x70008b2e0e); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x25d8)) else { return RegfileExit::fault(0x70008b2e11); };
            c.w64(3, t13);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b2e22, 0x70008b2e1d);
        }
        13 => {
            blk = 9; continue;
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b2e24); }
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b2e28); };
            c.w64(3, t14);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b2e36, 0x70008b2e31);
        }
        15 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2e36); };
            c.w64(6, t15);
            blk = 1; continue;
        }
        16 => {
            return RegfileExit::jump(0x70008b2e40);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b2e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b2e54); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b2e58); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b2e59); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b2e5a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b2e5f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b2e65); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b2e74);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b2e75); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008b2e81);
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b2e82); };
            c.w32(2, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b2e85); };
            c.cmp(t6, c.r32(2), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b2e8e); };
            c.w64(2, t7);
            if !c.st32(c.r64(6).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008b2e95); }
            let Some(t8) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b2e98); };
            c.w64(3, t8);
            return c.call(0x70008ba734, 0x70008b2ea1, 0x70008b2e9c);
        }
        7 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b2ea1); };
            c.w64(1, t9);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b2ea8); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return c.ret(0x10, 0x70008b2eaf);
        }
        9 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b2eb2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b2eb9); }
            return c.ret(0x0, 0x70008b2ebb);
        }
        10 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b2ebc); }
            let Some(t10) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b2ebd); };
            if !c.push64(t10) { return RegfileExit::fault(0x70008b2ebd); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x25e0)) else { return RegfileExit::fault(0x70008b2ec0); };
            c.w64(3, t11);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b2ed1, 0x70008b2ecc);
        }
        11 => {
            blk = 7; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b2ed3); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b2ed7); };
            c.w64(3, t12);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b2ee5, 0x70008b2ee0);
        }
        13 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b2ee5); };
            c.w64(6, t13);
            blk = 1; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008b2ef0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b2ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b2ef4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008b2ef8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b2efe); };
            c.w32(7, t1);
            c.w32(7, c.r32(7));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008b2f03); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008b2f07); }
            let r2 = c.r32(7) & 0x6u64;
            c.w32(7, r2);
            c.logic(r2, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b2f14); };
            c.w32(8, t3);
            c.cmp(c.r32(7), 0x2u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(7, c.r64(3));
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2f36); };
            c.w32(11, t4);
            let r5 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r5);
            let Some(t6) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2f3d); };
            c.cmp(t6, 0x80bu64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b2f46); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            blk = 3; continue;
        }
        6 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        7 => {
            c.w32(11, 0x1u64);
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2f5f); };
            c.w32(11, t9);
            let r10 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r10);
            let Some(t11) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2f66); };
            c.cmp(t11, 0xbcu64, 16);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2f6f); };
            c.w32(7, t12);
            let r13 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r13);
            blk = 3; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t14) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2f7d); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            let Some(t16) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2f84); };
            c.w32(9, t16);
            let r17 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r17);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b2f96); };
            c.w32(7, t18);
            let r19 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r19);
            c.w64(6, c.r64(7));
            blk = 25; continue;
        }
        14 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t20) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2fb4); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            let Some(t22) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2fbb); };
            c.cmp(t22, 0x80bu64, 16);
            if c.cc_ne() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b2fc4); };
            c.w32(1, t23);
            let r24 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r24);
            blk = 14; continue;
        }
        17 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 19; continue;
        }
        18 => {
            c.w32(11, 0x1u64);
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t25) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2fdd); };
            c.w32(9, t25);
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            let Some(t27) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2fe4); };
            c.cmp(t27, 0xbcu64, 16);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b2fed); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            blk = 14; continue;
        }
        22 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t30) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b2ffa); };
            c.w32(7, t30);
            let r31 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r31);
            let Some(t32) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3000); };
            c.w32(7, t32);
            let r33 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r33);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t34) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b300f); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            c.w64(6, c.r64(7));
            blk = 25; continue;
        }
        25 => {
            let Some(t36) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3018); };
            c.w32(1, t36);
            let r37 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r37);
            let Some(t38) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b301e); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            let Some(t40) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3024); };
            c.w32(7, t40);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x5u64, 64);
            if c.cc_a() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            return RegfileExit::fault(0x70008b3033);
        }
        27 => {
            let Some(t41) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b3034); };
            c.w32(1, t41);
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b3037); };
            c.w64(7, t42);
            let Some(t43) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b303e); };
            c.w64(9, t43);
            c.w64(11, c.r64(9).wrapping_add(0x18));
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b3045); };
            c.w64(12, t44);
            let Some(t45) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b304c); };
            c.cmp(t45, c.r64(11), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008b3056); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xd20)) else { return RegfileExit::fault(0x70008b305d); };
            c.w64(9, t46);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b3064); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b3068); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70008b306c); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008b306f); }
            c.w64(8, c.r64(2));
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b3075); }
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008b3079); }
            blk = 30; continue;
        }
        30 => {
            c.w64(0, c.r64(7));
            return c.call(0x700081f374, 0x70008b3084, 0x70008b307f);
        }
        31 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b3084); };
            c.w64(0, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b3088); };
            c.w64(9, t48);
            blk = 62; continue;
        }
        32 => {
            c.w64(7, c.r64(3));
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t49) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b30a6); };
            c.w32(9, t49);
            let r50 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r50);
            let Some(t51) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b30ad); };
            c.cmp(t51, 0x80bu64, 16);
            if c.cc_ne() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t52) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b30b6); };
            c.w32(7, t52);
            let r53 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r53);
            blk = 33; continue;
        }
        36 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        37 => {
            c.w32(11, 0x1u64);
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b30cf); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            let Some(t56) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b30d6); };
            c.cmp(t56, 0xbcu64, 16);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b30df); };
            c.w32(7, t57);
            let r58 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r58);
            blk = 33; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b30ed); };
            c.w32(8, t59);
            let r60 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r60);
            let Some(t61) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b30f4); };
            c.w32(8, t61);
            let r62 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r62);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3106); };
            c.w32(6, t63);
            let r64 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r64);
            blk = 56; continue;
        }
        44 => {
            c.w64(7, c.r64(1));
            blk = 45; continue;
        }
        45 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 49; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t65) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3126); };
            c.w32(9, t65);
            let r66 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r66);
            let Some(t67) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b312d); };
            c.cmp(t67, 0x80bu64, 16);
            if c.cc_ne() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3136); };
            c.w32(7, t68);
            let r69 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r69);
            blk = 45; continue;
        }
        48 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        49 => {
            c.w32(11, 0x1u64);
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t70) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b314f); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r71);
            let Some(t72) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3156); };
            c.cmp(t72, 0xbcu64, 16);
            if c.cc_ne() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t73) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b315f); };
            c.w32(7, t73);
            let r74 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r74);
            blk = 45; continue;
        }
        53 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t75) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b316d); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            let Some(t77) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3174); };
            c.w32(8, t77);
            let r78 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r78);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t79) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3186); };
            c.w32(6, t79);
            let r80 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r80);
            blk = 56; continue;
        }
        56 => {
            let Some(t81) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b318c); };
            c.w32(7, t81);
            let r82 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r82);
            let Some(t83) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b3192); };
            c.w32(7, t83);
            let r84 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r84);
            let Some(t85) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3198); };
            c.w32(8, t85);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(0));
            let Some(t86) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b31a2); };
            c.w32(11, t86);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x5u64, 64);
            if c.cc_a() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            return RegfileExit::fault(0x70008b31af);
        }
        58 => {
            let Some(t87) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b31b0); };
            c.w32(7, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b31b3); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008b31ba); };
            c.w64(12, t89);
            c.w64(15, c.r64(12).wrapping_add(0x18));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b31c2); };
            c.w64(0, t90);
            let Some(t91) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008b31c9); };
            c.cmp(t91, c.r64(15), 64);
            if c.cc_be() { blk = 99; continue; }
            blk = 59; continue;
        }
        59 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x70008b31d2); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 60; continue;
        }
        60 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0xdf8)) else { return RegfileExit::fault(0x70008b31da); };
            c.w64(12, t92);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008b31e1); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b31e5); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008b31e9); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008b31ed); }
            c.w64(0, c.r64(2));
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008b31f4); }
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(7)) { return RegfileExit::fault(0x70008b31f8); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b31fc); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r93 = c.r64(1) & c.r64(9);
            c.w64(1, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b3210); };
            c.logic(t94 & 0x4u64, 8);
            if c.cc_ne() { blk = 101; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b321a); };
            c.w32(1, t95);
            let r96 = c.r64(1) | 0x8u64;
            c.w64(1, r96);
            c.logic(r96, 64);
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008b3222); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b3229); }
            return c.ret(0x0, 0x70008b322a);
        }
        63 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b322b); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b322f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008b3233); }
            c.w64(7, c.r64(1));
            c.w32(11, 0x1u64);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 64; continue;
        }
        64 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t97) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3255); };
            c.w32(15, t97);
            let r98 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r98);
            let Some(t99) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b325c); };
            c.cmp(t99, 0x80bu64, 16);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t100) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3265); };
            c.w32(7, t100);
            let r101 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r101);
            blk = 64; continue;
        }
        67 => {
            c.w64(15, c.r64(9));
            blk = 69; continue;
        }
        68 => {
            c.w64(15, c.r64(11));
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t102) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b327b); };
            c.w32(15, t102);
            let r103 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r103);
            let Some(t104) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3282); };
            c.cmp(t104, 0xbcu64, 16);
            if c.cc_ne() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t105) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b328b); };
            c.w32(7, t105);
            let r106 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r106);
            blk = 64; continue;
        }
        72 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 75; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t107) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3299); };
            c.w32(12, t107);
            let r108 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r108);
            let Some(t109) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b32a0); };
            c.w32(12, t109);
            let r110 = c.sub(c.r32(12), 0x80cu64, 32);
            c.w32(12, r110);
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_a() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t111) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b32b3); };
            c.w32(7, t111);
            let r112 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r112);
            blk = 87; continue;
        }
        75 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t113) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b32ce); };
            c.w32(12, t113);
            let r114 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r114);
            let Some(t115) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b32d5); };
            c.cmp(t115, 0x80bu64, 16);
            if c.cc_ne() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t116) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b32df); };
            c.w32(3, t116);
            let r117 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r117);
            blk = 75; continue;
        }
        78 => {
            c.w64(12, c.r64(9));
            blk = 80; continue;
        }
        79 => {
            c.w64(12, c.r64(11));
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t118) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b32f5); };
            c.w32(12, t118);
            let r119 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r119);
            let Some(t120) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b32fc); };
            c.cmp(t120, 0xbcu64, 16);
            if c.cc_ne() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t121) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3306); };
            c.w32(3, t121);
            let r122 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r122);
            blk = 75; continue;
        }
        83 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t123) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3313); };
            c.w32(7, t123);
            let r124 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r124);
            let Some(t125) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3319); };
            c.w32(7, t125);
            let r126 = c.sub(c.r32(7), 0x80cu64, 32);
            c.w32(7, r126);
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_a() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t127) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3328); };
            c.w32(7, t127);
            let r128 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r128);
            blk = 87; continue;
        }
        86 => {
            c.w64(7, c.r64(6));
            blk = 87; continue;
        }
        87 => {
            let Some(t129) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3333); };
            c.w32(9, t129);
            let r130 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r130);
            let Some(t131) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b333a); };
            c.w32(9, t131);
            let r132 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r132);
            let Some(t133) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3341); };
            c.w32(12, t133);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008b334b); }
            c.cmp(c.r64(12), 0x5u64, 64);
            if c.cc_a() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            return RegfileExit::fault(0x70008b3355);
        }
        89 => {
            let Some(t134) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b3356); };
            c.w32(9, t134);
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b335a); };
            c.w64(12, t135);
            let Some(t136) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b3361); };
            c.w64(15, t136);
            c.w64(3, c.r64(15).wrapping_add(0x18));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008b3369); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008b336d); }
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b3371); };
            c.w64(15, t137);
            let Some(t138) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b3378); };
            c.cmp(t138, c.r64(3), 64);
            if c.cc_a() { blk = 92; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w32(2, 0x18u64);
            c.w64(12, c.r64(2));
            return c.call(0x70007febb4, 0x70008b338a, 0x70008b3385);
        }
        91 => {
            c.w64(12, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b338d); };
            c.w64(2, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b3391); };
            c.w64(0, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b3395); };
            c.w64(1, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3399); };
            c.w64(6, t142);
            c.w32(11, 0x1u64);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b33a3); };
            c.w64(7, t143);
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b33a7); };
            c.w64(8, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b33ab); };
            c.w64(9, t145);
            blk = 93; continue;
        }
        92 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008b33b1); }
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b33b5); };
            c.w64(12, t146);
            let r147 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r147);
            blk = 93; continue;
        }
        93 => {
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0xd28)) else { return RegfileExit::fault(0x70008b33bd); };
            c.w64(15, t148);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008b33c4); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b33c9); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008b33ce); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008b33d3); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x70008b33d8); }
            c.w64(1, c.r64(9));
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x70008b33e0); }
            let Some(t149) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b33e5); };
            c.w32(1, t149);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008b33e8); }
            c.logic(c.r8(1) & 0x8u64, 8);
            if c.cc_ne() { blk = 96; continue; }
            blk = 94; continue;
        }
        94 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b33f1); }
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x25b8)) else { return RegfileExit::fault(0x70008b33f2); };
            c.w64(3, t150);
            c.w64(0, c.r64(11));
            return c.call(0x700086ed74, 0x70008b3401, 0x70008b33fc);
        }
        95 => {
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b3401); };
            c.w64(2, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b3405); };
            c.w64(0, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b3409); };
            c.w64(7, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b340d); };
            c.w64(12, t154);
            blk = 96; continue;
        }
        96 => {
            c.w64(6, c.r64(7));
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(2));
            blk = 30; continue;
        }
        97 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008b3424); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008b3428); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008b342c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008b3430); }
            c.w64(7, c.r64(2));
            return c.call(0x70007febb4, 0x70008b343c, 0x70008b3437);
        }
        98 => {
            c.w64(7, c.r64(0));
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b343f); };
            c.w64(2, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b3443); };
            c.w64(0, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b3447); };
            c.w64(3, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b344b); };
            c.w64(6, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b344f); };
            c.w64(1, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b3453); };
            c.w64(8, t160);
            blk = 29; continue;
        }
        99 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b3461); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008b3465); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b3469); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008b346d); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febb4, 0x70008b3479, 0x70008b3474);
        }
        100 => {
            c.w64(8, c.r64(0));
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008b347c); };
            c.w64(2, t161);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b3480); };
            c.w64(9, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3484); };
            c.w64(7, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b3488); };
            c.w64(3, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b348c); };
            c.w64(1, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b3490); };
            c.w64(11, t166);
            blk = 60; continue;
        }
        101 => {
            c.w32(8, c.r32(8));
            let r167 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r167);
            c.w64(1, 0xfffffffffffc0000u64);
            let r168 = c.r64(1) & c.r64(8);
            c.w64(1, r168);
            c.logic(r168, 64);
            let Some(t169) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b34a9); };
            c.logic(t169 & 0x2u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(7, c.r64(9).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b34b7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b34b8); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x70008b34c5, 0x70008b34c0);
        }
        103 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b34c5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b34c7); }
            blk = 62; continue;
        }
        104 => {
            return RegfileExit::jump(0x70008b34d0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b34d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b34d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b34d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b34d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b34da); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b34df); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b34e3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b34ed); };
            c.w64(0, t2);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 32; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b34f9); };
            c.w32(1, t3);
            let r4 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r4);
            let Some(t5) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b34ff); };
            c.w32(1, t5);
            let r6 = c.sub(c.r32(1), 0xbau64, 32);
            c.w32(1, r6);
            c.cmp(c.r32(1), 0x78eu64, 32);
            if c.cc_a() { blk = 32; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3515); };
            c.w32(1, t7);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xeau64, 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008b3527);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b3528); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b3530); };
            c.w64(3, t9);
            return c.call(0x70008b23d4, 0x70008b353c, 0x70008b3537);
        }
        7 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b353c); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b3542); };
            if !c.push64(t12) { return RegfileExit::fault(0x70008b3542); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b3545); };
            c.w64(1, t13);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b354c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b354d); }
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3551); };
            c.w64(6, t14);
            c.w32(0, 0x1u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2134, 0x70008b3562, 0x70008b355d);
        }
        8 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b3562); };
            c.w64(1, t15);
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3566); };
            c.w32(0, t16);
            let r17 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r17);
            blk = 9; continue;
        }
        9 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b356c); };
            c.w64(1, t18);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b3573); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x10, 0x70008b357a);
        }
        11 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b357d); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b3584); }
            return c.ret(0x0, 0x70008b3586);
        }
        12 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3587); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b358d); };
            c.w32(1, t21);
            let r22 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3593); };
            c.w32(8, t23);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            return RegfileExit::fault(0x70008b35a6);
        }
        14 => {
            let Some(t24) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b35a7); };
            c.w32(1, t24);
            let r25 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r25);
            let Some(t26) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b35b0); };
            c.w32(1, t26);
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b35b3); };
            c.w64(8, t27);
            let Some(t28) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008b35ba); };
            c.w64(9, t28);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b35c1); };
            c.w64(12, t29);
            let Some(t30) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b35c8); };
            c.cmp(t30, c.r64(11), 64);
            if c.cc_be() { blk = 36; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008b35d2); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 16; continue;
        }
        16 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b35d9); }
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b35dd); };
            c.w64(1, t31);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008b35e4); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008b35e8); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008b35ee); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008b35f2); }
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b35f6); };
            c.w64(1, t32);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008b35fa); }
            c.w32(9, 0x4u64);
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b3604); }
            let Some(t33) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b3608); };
            c.w64(9, t33);
            let Some(t34) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x70008b360f); };
            c.w32(9, t34);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008b3612); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b3616); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 17; continue;
        }
        17 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t35) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b362e); };
            c.w32(11, t35);
            let r36 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r36);
            let Some(t37) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b3635); };
            c.w32(11, t37);
            let r38 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r38);
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b363c); };
            c.w32(12, t39);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x8eu64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008b364f);
        }
        20 => {
            let Some(t40) = c.ld32(c.r64(11).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008b3650); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b365a); };
            c.w32(11, t42);
            let r43 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r43);
            let Some(t44) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b3661); };
            c.logic(t44 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b3668); };
            c.w64(11, t45);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 25; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t46) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3675); };
            c.w32(12, t46);
            let r47 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r47);
            let Some(t48) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b367c); };
            c.w32(12, t48);
            c.cmp(c.r16(12), 0x83du64, 16);
            if c.cc_ne() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(12, c.r16(12));
            c.cmp(c.r32(12), 0x83du64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::fault(0x70008b3697);
        }
        25 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b3698); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b369a); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b369c); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2134, 0x70008b36a8, 0x70008b36a3);
        }
        26 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b36a8); };
            c.w64(6, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b36ac); };
            c.w64(8, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b36b0); };
            c.w64(1, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b36b4); };
            c.w64(9, t52);
            blk = 27; continue;
        }
        27 => {
            c.logic(c.r8(9) & 0x6u64, 8);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b36be); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b36c0); };
            c.w64(9, t53);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b36c7); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x70008b36c9); };
            c.w64(3, t54);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b36da, 0x70008b36d5);
        }
        29 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b36da); };
            c.w64(6, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b36de); };
            c.w64(8, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b36e2); };
            c.w64(1, t57);
            blk = 30; continue;
        }
        30 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b36e6); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b36e8); }
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x25b0)) else { return RegfileExit::fault(0x70008b36e9); };
            c.w64(3, t58);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b36fa, 0x70008b36f5);
        }
        31 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b36fa); };
            c.w64(0, t59);
            blk = 9; continue;
        }
        32 => {
            c.w32(1, 0x34u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b3708); }
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b3709); };
            c.w64(1, t60);
            let Some(t61) = c.ld32(c.r64(1).wrapping_add(0x22b)) else { return RegfileExit::fault(0x70008b3710); };
            c.w32(1, t61);
            let r62 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r62);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b3719); }
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b371a); };
            c.w64(3, t63);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b372b, 0x70008b3726);
        }
        33 => {
            return RegfileExit::fault(0x70008b372b);
        }
        34 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b372c); };
            c.w64(3, t64);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b373a, 0x70008b3735);
        }
        35 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b373a); };
            c.w64(6, t65);
            blk = 1; continue;
        }
        36 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b3748); }
            return c.call(0x70007febf4, 0x70008b3751, 0x70008b374c);
        }
        37 => {
            c.w64(8, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3754); };
            c.w64(6, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b3758); };
            c.w64(1, t67);
            blk = 16; continue;
        }
        38 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b3761); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008b3762); };
            c.w64(3, t68);
            c.w32(0, 0x1u64);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b376e); };
            c.w64(6, t69);
            return c.call(0x700086ed74, 0x70008b3777, 0x70008b3772);
        }
        39 => {
            blk = 26; continue;
        }
        40 => {
            return RegfileExit::jump(0x70008b3780);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b3794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b3794); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b3798); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b3799); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b379a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b379f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b37a9); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b37b8);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b37b9); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b37bf); };
            c.w32(1, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b37c2); };
            c.cmp(t6, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b37cb); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b37ce); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b37d7); };
            c.w64(1, t9);
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008b37db); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r10 = c.r64(7) & c.r64(2);
            c.w64(7, r10);
            c.logic(r10, 64);
            let Some(t11) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b37ed); };
            c.logic(t11 & 0x4u64, 8);
            if c.cc_ne() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008b37f7); };
            c.w64(1, t12);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008b37fb); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r13 = c.r64(7) & c.r64(2);
            c.w64(7, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b380d); };
            c.logic(t14 & 0x4u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b3813); };
            c.w64(0, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b381a); };
            c.w64(1, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b3821); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_g() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            return c.ret(0x18, 0x70008b3828);
        }
        11 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b382b); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b3832); }
            return c.ret(0x0, 0x70008b3834);
        }
        12 => {
            c.w32(2, 0xf0u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b383a); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b383b); };
            c.w64(3, t17);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008b384c, 0x70008b3847);
        }
        13 => {
            return RegfileExit::fault(0x70008b384c);
        }
        14 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b384d); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b3851); };
            c.w64(3, t18);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b385f, 0x70008b385a);
        }
        15 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b385f); };
            c.w64(6, t19);
            blk = 1; continue;
        }
        16 => {
            c.w32(1, c.r32(1));
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            c.w64(7, 0xfffffffffffc0000u64);
            let r21 = c.r64(7) & c.r64(1);
            c.w64(7, r21);
            c.logic(r21, 64);
            let Some(t22) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b3877); };
            c.logic(t22 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 17; continue;
        }
        17 => {
            c.w64(8, c.r64(2).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b3881); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x70008b388a, 0x70008b3885);
        }
        18 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b388a); }
            blk = 9; continue;
        }
        19 => {
            c.w32(1, c.r32(1));
            let r23 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r23);
            c.w64(7, 0xfffffffffffc0000u64);
            let r24 = c.r64(7) & c.r64(1);
            c.w64(7, r24);
            c.logic(r24, 64);
            let Some(t25) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b389c); };
            c.logic(t25 & 0x2u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b38aa); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x70008b38b3, 0x70008b38ae);
        }
        21 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b38b3); }
            blk = 7; continue;
        }
        22 => {
            return RegfileExit::jump(0x70008b38bc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b38d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b38d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b38d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b38d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b38da); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b38df); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008b38e5); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b38e9); };
            c.cmp(t3, c.r32(2), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b38f2); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b38f3); };
            c.w64(2, t4);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b38fa); }
            c.w32(0, 0x1u64);
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008b3900); };
            c.w64(7, t5);
            return c.call(0x70007f2134, 0x70008b3909, 0x70008b3904);
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b3909); };
            c.w64(0, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b390d); };
            c.w64(1, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b3914); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x28, 0x70008b391b);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b391e); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b3925); }
            return c.ret(0x0, 0x70008b3927);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b3928); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b392c); };
            c.w64(3, t8);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b393a, 0x70008b3935);
        }
        7 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b393a); };
            c.w64(6, t9);
            blk = 1; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008b3944);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b3954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b3954); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b3958); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b3959); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b395a); }
            let r0 = c.sub(c.r64(4), 0x88u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b3962); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b3966); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 236; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3970); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b3977); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b397e); };
            c.w64(0, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008b3982); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 234; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b398e); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3995); };
            c.w32(9, t9);
            let r10 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r10);
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 234; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b39ae); };
            c.w64(3, t11);
            return c.call(0x70008b23d4, 0x70008b39ba, 0x70008b39b5);
        }
        4 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b39ba); };
            c.w64(6, t12);
            let Some(t13) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b39be); };
            c.w32(8, t13);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b39c8); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008b39d5);
        }
        6 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b39d6); };
            c.w64(8, t14);
            let Some(t15) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b39da); };
            c.cmp(t15, c.r32(8), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x1000)) else { return RegfileExit::fault(0x70008b39eb); };
            c.w64(11, t16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b39f2); };
            c.cmp(t17, c.r32(9), 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b3a04); };
            c.w64(9, t18);
            blk = 15; continue;
        }
        11 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b3a0d); };
            c.w64(3, t19);
            c.w64(0, c.r64(8));
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3a17); };
            c.w64(6, t20);
            return c.call(0x7000870914, 0x70008b3a20, 0x70008b3a1b);
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 232; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3a28); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b3a2f); };
            c.logic(t23 & 0x2u64, 8);
            if c.cc_e() { blk = 232; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(9, c.r64(0));
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x70008b3a3d); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008b3a41); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b3a48); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3a4c); };
            c.w64(6, t26);
            return c.call(0x7000870914, 0x70008b3a55, 0x70008b3a50);
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 327; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3a5d); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b3a64); };
            c.logic(t29 & 0x2u64, 8);
            if c.cc_e() { blk = 327; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b3a6f); };
            if !c.push64(t30) { return RegfileExit::fault(0x70008b3a6f); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3a77); };
            c.w64(6, t31);
            return c.call(0x70007f2274, 0x70008b3a80, 0x70008b3a7b);
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008b3a80); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 325; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3a8c); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3a93); };
            c.cmp(t34, 0xbau64, 16);
            if c.cc_b() { blk = 325; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008b3aa0); };
            c.w64(3, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3aa7); };
            c.w64(6, t36);
            return c.call(0x7000870914, 0x70008b3ab0, 0x70008b3aab);
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(0)) { return RegfileExit::fault(0x70008b3ab0); }
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b3ab7); };
            c.w64(8, t37);
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3abb); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3ac2); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3ac9); };
            c.w32(9, t42);
            let r43 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008b3ad0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008b3ad4); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b3ad8); };
            c.w64(11, t44);
            let Some(t45) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008b3adf); };
            c.cmp(t45, 0x0u64, 8);
            if c.cc_ne() { blk = 238; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b3ae9); };
            c.w64(6, t46);
            let Some(t47) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3aed); };
            c.w32(9, t47);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x79u64, 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::fault(0x70008b3afd);
        }
        25 => {
            let Some(t48) = c.ld32(c.r64(6).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b3afe); };
            c.w32(9, t48);
            c.w32(12, 0x8u64);
            c.w64(15, c.r64(12));
            let r49 = c.add(c.r64(15), 0x14u64, 64);
            c.w64(15, r49);
            if c.cc_o() { blk = 231; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b3b18); };
            c.w64(3, t50);
            let Some(t51) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b3b1f); };
            c.w64(2, t51);
            c.w64(11, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b3b26); };
            c.w64(0, t52);
            let Some(t53) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008b3b2d); };
            c.cmp(t53, c.r64(11), 64);
            if c.cc_be() { blk = 240; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 240; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(3), c.r64(11)) { return RegfileExit::fault(0x70008b3b43); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b3b4a); }
            c.w32(9, 0xau64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b3b54); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008b3b58); }
            c.w64(9, c.r64(12));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 31; continue;
        }
        30 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008b3b64); }
            let r54 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r54);
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(9), 0x1cu64, 64);
            if c.cc_ne() { blk = 30; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3b73); };
            c.w32(9, t55);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x70008b3b83);
        }
        34 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b3b84); };
            c.w64(2, t56);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008b3b8b); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_a() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            return RegfileExit::fault(0x70008b3b95);
        }
        36 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b3b96); };
            c.w64(2, t57);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008b3b9d); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008b3ba7);
        }
        38 => {
            c.w32(2, 0x2u64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x70008b3bad); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008b3bb7);
        }
        40 => {
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x70008b3bb8); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x70008b3bc2);
        }
        42 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b3bc3); };
            c.w64(9, t58);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008b3bca); }
            let Some(t59) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3bce); };
            c.w32(8, t59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x68u64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x70008b3bde);
        }
        44 => {
            let Some(t60) = c.ld32(c.r64(6).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008b3bdf); };
            c.w32(8, t60);
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(8)) { return RegfileExit::fault(0x70008b3be6); }
            blk = 46; continue;
        }
        45 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b3bef); };
            c.w64(2, t61);
            c.w32(8, c.r64(2).wrapping_add(0x2));
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(1));
            c.w64(3, c.r64(9));
            blk = 46; continue;
        }
        46 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b3c00); };
            if !c.push64(t62) { return RegfileExit::fault(0x70008b3c00); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008b3c03); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008b3c07); };
            c.w64(7, t63);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3c10); };
            c.w64(6, t64);
            return c.call(0x70007f21d4, 0x70008b3c19, 0x70008b3c14);
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b3c19); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 323; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3c25); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008b3c2c); };
            c.cmp(c.r32(8), t67, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t68) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3c39); };
            c.cmp(t68, 0xbau64, 16);
            if c.cc_b() { blk = 323; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x70008b3c46); };
            c.w64(3, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3c4d); };
            c.w64(6, t70);
            return c.call(0x7000870914, 0x70008b3c56, 0x70008b3c51);
        }
        51 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b3c56); };
            c.cmp(t71, c.r32(0), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b3c66); };
            c.cmp(t72, c.r32(0), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t73) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3c6f); };
            c.w32(8, t73);
            let r74 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r74);
            let Some(t75) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b3c76); };
            c.logic(t75 & 0x10u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b3c7d); };
            c.cmp(t76, c.r32(8), 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t77) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3c8a); };
            c.cmp(t77, 0x41u64, 16);
            if c.cc_e() { blk = 242; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 194; continue;
        }
        58 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b3c9b); };
            c.w64(0, t78);
            blk = 71; continue;
        }
        59 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b3ca4); };
            c.w64(0, t79);
            blk = 71; continue;
        }
        60 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b3cb7); };
            c.w64(0, t80);
            blk = 71; continue;
        }
        62 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b3cbd); };
            c.w64(0, t81);
            blk = 71; continue;
        }
        63 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3cc3); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            let Some(t84) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b3cca); };
            c.cmp(t84, c.r32(8), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b3cdc); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t86) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3ce5); };
            c.w32(9, t86);
            let r87 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r87);
            let Some(t88) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b3cec); };
            c.logic(t88 & 0x10u64, 8);
            if c.cc_ne() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b3cf3); };
            c.cmp(t89, c.r32(9), 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t90) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3d00); };
            c.cmp(t90, 0x41u64, 16);
            if c.cc_e() { blk = 246; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 194; continue;
        }
        70 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008b3d1b); };
            c.w64(8, t91);
            let Some(t92) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3d22); };
            c.cmp(t92, c.r32(8), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008b3d28); };
            c.w64(3, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b3d2f); };
            c.w64(6, t94);
            return c.call(0x7000870914, 0x70008b3d38, 0x70008b3d33);
        }
        73 => {
            blk = 75; continue;
        }
        74 => {
            let Some(t95) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3d3a); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r96);
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b3d40); }
            let Some(t97) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b3d44); };
            c.cmp(t97, 0x1ffffeu64, 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b3d51); };
            c.w64(8, t98);
            let Some(t99) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3d55); };
            c.w32(9, t99);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 224; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3d69); };
            c.w32(9, t100);
            let r101 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r101);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b3d74); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b3d78); };
            c.w64(6, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3d7c); };
            c.w32(9, t103);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_be() { blk = 225; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t104) = c.ld32(c.r64(6).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b3d93); };
            c.w32(9, t104);
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x1090)) else { return RegfileExit::fault(0x70008b3d9a); };
            c.w64(11, t105);
            let Some(t106) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3da1); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(13).wrapping_add(0x63f0));
            let Some(t108) = c.ld64(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b3db8); };
            c.w64(12, t108);
            blk = 99; continue;
        }
        80 => {
            let Some(t109) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3dc1); };
            c.w32(15, t109);
            let r110 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r110);
            let Some(t111) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3dc9); };
            c.w32(15, t111);
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_l() { blk = 87; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_l() { blk = 85; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(15), 0x9du64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(15), 0x9eu64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(15), 0x5eu64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_l() { blk = 90; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(15), 0x53u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(15), 0x48u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(15), 0x49u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t112) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3e22); };
            c.w32(12, t112);
            blk = 99; continue;
        }
        93 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b3e29); };
            c.w64(12, t113);
            blk = 99; continue;
        }
        94 => {
            let Some(t114) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3e32); };
            c.w32(12, t114);
            blk = 99; continue;
        }
        95 => {
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b3e39); };
            c.w64(12, t115);
            blk = 99; continue;
        }
        96 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b3e42); };
            c.w64(12, t116);
            blk = 99; continue;
        }
        97 => {
            let Some(t117) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3e4b); };
            c.w32(12, t117);
            blk = 99; continue;
        }
        98 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b3e52); };
            c.w64(12, t118);
            blk = 99; continue;
        }
        99 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b3e59); };
            c.w64(15, t119);
            let Some(t120) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b3e60); };
            c.w64(3, t120);
            c.w64(2, c.r64(3).wrapping_add(0x1c));
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b3e67); };
            c.w64(1, t121);
            let Some(t122) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b3e6e); };
            c.cmp(t122, c.r64(2), 64);
            if c.cc_be() { blk = 250; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008b3e77); }
            let r123 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r123);
            blk = 101; continue;
        }
        101 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b3e7e); }
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b3e82); };
            c.w64(9, t124);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b3e89); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b3e8d); }
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b3e91); };
            c.w64(2, t125);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008b3e98); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b3e9b); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b3e9f); }
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008b3ea3); }
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b3ea7); };
            c.w64(2, t126);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008b3eab); }
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b3eae); };
            c.w64(11, t127);
            let Some(t128) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3eb2); };
            c.w32(12, t128);
            let r129 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b3eb9); };
            c.w64(7, t130);
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b3ebd); };
            c.cmp(t131, c.r32(7), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x70008b3ecc); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x70008b3ed0); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008b3ed4); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b3ee2); };
            c.w64(8, t132);
            let Some(t133) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x70008b3ee9); };
            c.cmp(t133, 0x0u64, 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x70008b3ef3); };
            c.w64(8, t134);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t135) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3efd); };
            c.cmp(t135, c.r32(9), 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w32(8, 0x1u64);
            blk = 107; continue;
        }
        106 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 180; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t136) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3f20); };
            c.w32(8, t136);
            let r137 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r137);
            let Some(t138) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3f27); };
            c.cmp(t138, 0x83du64, 16);
            if c.cc_ne() { blk = 113; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t139) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b3f30); };
            c.w32(11, t139);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3f37); };
            c.w32(8, t140);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x97u64, 64);
            if c.cc_be() { blk = 226; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t141) = c.ld32(c.r64(6).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b3f4b); };
            c.cmp(t141, c.r32(8), 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008b3f54); };
            c.w64(8, t142);
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3f5b); };
            c.cmp(t143, c.r32(9), 32);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            blk = 114; continue;
        }
        112 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 114; continue;
        }
        113 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t144) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b3f7b); };
            c.w32(8, t144);
            c.w32(8, c.r32(8));
            let r145 = c.r32(8) & 0x6u64;
            c.w32(8, r145);
            c.logic(r145, 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t146) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3f8c); };
            c.w32(11, t146);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(8, c.r64(3));
            blk = 118; continue;
        }
        118 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 122; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t147) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3faf); };
            c.w32(9, t147);
            let r148 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r148);
            let Some(t149) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3fb6); };
            c.cmp(t149, 0x80bu64, 16);
            if c.cc_ne() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t150) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b3fbf); };
            c.w32(8, t150);
            let r151 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r151);
            blk = 118; continue;
        }
        121 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 123; continue;
        }
        122 => {
            c.w32(9, 0x1u64);
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 126; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3fd9); };
            c.w32(9, t152);
            let r153 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r153);
            let Some(t154) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3fe0); };
            c.cmp(t154, 0xbcu64, 16);
            if c.cc_ne() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t155) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3fe9); };
            c.w32(8, t155);
            let r156 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r156);
            blk = 118; continue;
        }
        126 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 129; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t157) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b3ff8); };
            c.w32(9, t157);
            let r158 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r158);
            let Some(t159) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b3fff); };
            c.w32(9, t159);
            let r160 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r160);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b4011); };
            c.w32(8, t161);
            let r162 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r162);
            blk = 142; continue;
        }
        129 => {
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b401d); };
            c.w64(12, t163);
            blk = 130; continue;
        }
        130 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t164) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4033); };
            c.w32(9, t164);
            let r165 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r165);
            let Some(t166) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b403b); };
            c.cmp(t166, 0x80bu64, 16);
            if c.cc_ne() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t167) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4044); };
            c.w32(12, t167);
            let r168 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r168);
            blk = 130; continue;
        }
        133 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 135; continue;
        }
        134 => {
            c.w32(9, 0x1u64);
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 138; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t169) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b405f); };
            c.w32(9, t169);
            let r170 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r170);
            let Some(t171) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4067); };
            c.cmp(t171, 0xbcu64, 16);
            if c.cc_ne() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t172) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4070); };
            c.w32(12, t172);
            let r173 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r173);
            blk = 130; continue;
        }
        138 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 141; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t174) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4080); };
            c.w32(8, t174);
            let r175 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r175);
            let Some(t176) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4088); };
            c.w32(8, t176);
            let r177 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r177);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t178) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b409a); };
            c.w32(8, t178);
            let r179 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r179);
            blk = 142; continue;
        }
        141 => {
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b40a4); };
            c.w64(8, t180);
            blk = 142; continue;
        }
        142 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b40a8); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b40af); };
            c.w32(9, t183);
            let r184 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r184);
            let Some(t185) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b40b6); };
            c.w32(12, t185);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x5u64, 64);
            if c.cc_be() { blk = 227; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t186) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b40ca); };
            c.w32(9, t186);
            c.w64(12, c.r64(15));
            let Some(t187) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b40d1); };
            c.w64(15, t187);
            c.w64(2, c.r64(15).wrapping_add(0x18));
            let Some(t188) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b40d9); };
            c.cmp(t188, c.r64(2), 64);
            if c.cc_be() { blk = 288; continue; }
            blk = 144; continue;
        }
        144 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x70008b40e2); }
            let r189 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r189);
            blk = 145; continue;
        }
        145 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0xd20)) else { return RegfileExit::fault(0x70008b40ea); };
            c.w64(2, t190);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b40f1); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b40f5); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b40f9); }
            c.w64(11, c.r64(3));
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b4100); }
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b4104); };
            c.w64(11, t191);
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b410b); }
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b410f); }
            blk = 146; continue;
        }
        146 => {
            c.w64(0, c.r64(15));
            c.w64(6, c.r64(8));
            return c.call(0x700081f374, 0x70008b411e, 0x70008b4119);
        }
        147 => {
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b411e); };
            c.w64(9, t192);
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b4125); };
            c.w64(1, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b412c); };
            c.w64(6, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4130); };
            c.w64(11, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b4134); };
            c.w64(7, t196);
            blk = 178; continue;
        }
        148 => {
            c.w64(8, c.r64(3));
            blk = 149; continue;
        }
        149 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t197) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4152); };
            c.w32(9, t197);
            let r198 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r198);
            let Some(t199) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4159); };
            c.cmp(t199, 0x80bu64, 16);
            if c.cc_ne() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t200) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4162); };
            c.w32(8, t200);
            let r201 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r201);
            blk = 149; continue;
        }
        152 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 154; continue;
        }
        153 => {
            c.w32(9, 0x1u64);
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 157; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t202) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b417c); };
            c.w32(9, t202);
            let r203 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r203);
            let Some(t204) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4183); };
            c.cmp(t204, 0xbcu64, 16);
            if c.cc_ne() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t205) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b418c); };
            c.w32(8, t205);
            let r206 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r206);
            blk = 149; continue;
        }
        157 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 160; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t207) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b419b); };
            c.w32(9, t207);
            let r208 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r208);
            let Some(t209) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b41a2); };
            c.w32(9, t209);
            let r210 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r210);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t211) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b41b4); };
            c.w32(8, t211);
            let r212 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r212);
            blk = 173; continue;
        }
        160 => {
            c.w64(8, c.r64(12));
            blk = 161; continue;
        }
        161 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 165; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t213) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b41d5); };
            c.w32(11, t213);
            let r214 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r214);
            let Some(t215) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b41dc); };
            c.cmp(t215, 0x80bu64, 16);
            if c.cc_ne() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t216) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b41e5); };
            c.w32(8, t216);
            let r217 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r217);
            blk = 161; continue;
        }
        164 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 166; continue;
        }
        165 => {
            c.w32(11, 0x1u64);
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t218) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b41ff); };
            c.w32(11, t218);
            let r219 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r219);
            let Some(t220) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4206); };
            c.cmp(t220, 0xbcu64, 16);
            if c.cc_ne() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t221) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b420f); };
            c.w32(8, t221);
            let r222 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r222);
            blk = 161; continue;
        }
        169 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t223) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b421e); };
            c.w32(9, t223);
            let r224 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r224);
            let Some(t225) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4225); };
            c.w32(9, t225);
            let r226 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r226);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t227) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b4237); };
            c.w32(8, t227);
            let r228 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r228);
            blk = 173; continue;
        }
        172 => {
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4240); };
            c.w64(8, t229);
            blk = 173; continue;
        }
        173 => {
            let Some(t230) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4244); };
            c.w32(8, t230);
            let r231 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r231);
            let Some(t232) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b424b); };
            c.w32(8, t232);
            let r233 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r233);
            let Some(t234) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4252); };
            c.w32(9, t234);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(0));
            let Some(t235) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b425c); };
            c.w32(0, t235);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x5u64, 64);
            if c.cc_be() { blk = 228; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t236) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b426d); };
            c.w32(8, t236);
            c.w64(9, c.r64(15));
            let Some(t237) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b4274); };
            c.w64(15, t237);
            c.w64(2, c.r64(15).wrapping_add(0x18));
            let Some(t238) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b427b); };
            c.cmp(t238, c.r64(2), 64);
            if c.cc_be() { blk = 290; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.st64(c.r64(9), c.r64(2)) { return RegfileExit::fault(0x70008b4284); }
            let r239 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r239);
            blk = 176; continue;
        }
        176 => {
            let Some(t240) = c.ld64(c.r64(13).wrapping_add(0xdf8)) else { return RegfileExit::fault(0x70008b428b); };
            c.w64(2, t240);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b4292); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008b4296); }
            if !c.st32(c.r64(15).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x70008b429a); }
            if !c.st32(c.r64(15).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008b429e); }
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b42a2); };
            c.w64(12, t241);
            if !c.st32(c.r64(15).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008b42a9); }
            if !c.st32(c.r64(15).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x70008b42ad); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b42b1); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r242 = c.r64(8) & c.r64(11);
            c.w64(8, r242);
            c.logic(r242, 64);
            let Some(t243) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b42c5); };
            c.logic(t243 & 0x4u64, 8);
            if c.cc_ne() { blk = 334; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t244) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b42d0); };
            c.w32(8, t244);
            let r245 = c.r64(8) | 0x8u64;
            c.w64(8, r245);
            c.logic(r245, 64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b42d8); }
            blk = 45; continue;
        }
        179 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b42e1); };
            c.w64(11, t246);
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b42e5); };
            c.w64(9, t247);
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b42ec); };
            c.w64(8, t248);
            blk = 183; continue;
        }
        180 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b42f2); };
            c.w64(9, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b42f9); };
            c.w64(8, t250);
            blk = 183; continue;
        }
        181 => {
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b42ff); };
            c.w64(9, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b4306); };
            c.w64(8, t252);
            blk = 183; continue;
        }
        182 => {
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b430c); };
            c.w64(8, t253);
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(3, c.r64(0));
            let Some(t254) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b431d); };
            c.w64(0, t254);
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4321); };
            c.w64(6, t255);
            return c.call(0x70008ba474, 0x70008b432a, 0x70008b4325);
        }
        185 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b432a); }
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b432e); };
            c.w64(3, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4335); };
            c.w64(6, t257);
            return c.call(0x7000870914, 0x70008b433e, 0x70008b4339);
        }
        186 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b433e); };
            if !c.push64(t258) { return RegfileExit::fault(0x70008b433e); }
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b4341); };
            if !c.push64(t259) { return RegfileExit::fault(0x70008b4341); }
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4344); };
            if !c.push64(t260) { return RegfileExit::fault(0x70008b4344); }
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b4347); };
            c.w64(6, t261);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008b435b, 0x70008b4356);
        }
        187 => {
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b435b); };
            c.w64(8, t262);
            let Some(t263) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x70008b4362); };
            c.cmp(t263, 0x0u64, 8);
            if c.cc_e() { blk = 193; continue; }
            blk = 188; continue;
        }
        188 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 191; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t264) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b436c); };
            c.w32(9, t264);
            let r265 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r265);
            let Some(t266) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4373); };
            c.cmp(t266, 0x83du64, 16);
            if c.cc_ne() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            c.w32(9, 0x1u64);
            blk = 192; continue;
        }
        191 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 294; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b4391); };
            c.w64(9, t267);
            let Some(t268) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b4398); };
            c.w64(1, t268);
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b439f); };
            c.w64(6, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b43a3); };
            c.w64(7, t270);
            blk = 45; continue;
        }
        194 => {
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b43ac); };
            c.w64(0, t271);
            let Some(t272) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b43b0); };
            c.w32(8, t272);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_a() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            return RegfileExit::fault(0x70008b43c0);
        }
        196 => {
            let Some(t273) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b43c1); };
            c.w32(9, t273);
            let r274 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r274);
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b43cc); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 296; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r64(8), 0x4u64, 64);
            c.w8(8, (c.cc_a() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_le() { blk = 299; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            return RegfileExit::fault(0x70008b43f6);
        }
        200 => {
            let Some(t275) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b43f7); };
            c.w64(8, (t275 as u32 as i32 as i64 as u64));
            let Some(t276) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b43fb); };
            c.w32(9, t276);
            let r277 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r277);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r278 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r278);
            let Some(t279) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4409); };
            c.w32(11, t279);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), c.r64(8), 64);
            if c.cc_ge() { blk = 220; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_b() { blk = 309; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 217; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 314; continue; }
            blk = 204; continue;
        }
        204 => {
            let r280 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r280);
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r281 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r281);
            if c.cc_o() { blk = 216; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(3, 0x8u64);
            let r282 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r282);
            if c.cc_o() { blk = 215; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b446a); };
            c.w64(2, t283);
            let Some(t284) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b4471); };
            c.w64(1, t284);
            c.w64(7, c.r64(1).wrapping_add(c.r64(15)));
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b4478); };
            c.w64(6, t285);
            let Some(t286) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008b447f); };
            c.cmp(t286, c.r64(7), 64);
            if c.cc_be() { blk = 312; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 312; continue; }
            blk = 208; continue;
        }
        208 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x70008b4495); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 209; continue;
        }
        209 => {
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008b449c); };
            c.w64(2, t287);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b44a3); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b44a7); }
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(12, c.r64(3));
            blk = 211; continue;
        }
        210 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b44b8); }
            let r288 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r288);
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_e() { blk = 218; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(2, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_e() { blk = 214; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t289) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b44d3); };
            c.w32(2, t289);
            let r290 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r290);
            blk = 210; continue;
        }
        214 => {
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b44de); };
            c.w64(2, t291);
            blk = 210; continue;
        }
        215 => {
            return RegfileExit::fault(0x70008b44e7);
        }
        216 => {
            return RegfileExit::fault(0x70008b44e8);
        }
        217 => {
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b44e9); };
            c.w64(15, t292);
            blk = 218; continue;
        }
        218 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008b44f0); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r293 = c.r64(8) & c.r64(0);
            c.w64(8, r293);
            c.logic(r293, 64);
            let Some(t294) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b4504); };
            c.logic(t294 & 0x4u64, 8);
            if c.cc_ne() { blk = 331; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b450f); };
            c.w64(0, t295);
            blk = 221; continue;
        }
        221 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b4513); };
            c.w64(1, t296);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b451a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            return c.ret(0x10, 0x70008b4521);
        }
        223 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b4524); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b452b); }
            return c.ret(0x0, 0x70008b452d);
        }
        224 => {
            return RegfileExit::fault(0x70008b452e);
        }
        225 => {
            return RegfileExit::fault(0x70008b452f);
        }
        226 => {
            return RegfileExit::fault(0x70008b4530);
        }
        227 => {
            return RegfileExit::fault(0x70008b4531);
        }
        228 => {
            return RegfileExit::fault(0x70008b4532);
        }
        229 => {
            c.w32(8, 0x294u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b4539); }
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b453b); };
            c.w64(8, t297);
            let Some(t298) = c.ld32(c.r64(8).wrapping_add(0x22f)) else { return RegfileExit::fault(0x70008b4542); };
            c.w32(8, t298);
            let r299 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r299);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b454c); }
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008b454e); };
            c.w64(3, t300);
            c.w32(0, 0x2u64);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b455a); };
            c.w64(6, t301);
            return c.call(0x700086ed74, 0x70008b4563, 0x70008b455e);
        }
        230 => {
            return RegfileExit::fault(0x70008b4563);
        }
        231 => {
            return RegfileExit::fault(0x70008b4564);
        }
        232 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b456b); }
            let Some(t302) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b456d); };
            c.w64(8, t302);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b4574); }
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b4576); };
            c.w64(3, t303);
            c.w32(0, 0x2u64);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4582); };
            c.w64(6, t304);
            return c.call(0x700086ed74, 0x70008b458b, 0x70008b4586);
        }
        233 => {
            return RegfileExit::fault(0x70008b458b);
        }
        234 => {
            c.w32(8, 0x34u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b4592); }
            let Some(t305) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b4594); };
            c.w64(8, t305);
            let Some(t306) = c.ld32(c.r64(8).wrapping_add(0x233)) else { return RegfileExit::fault(0x70008b459b); };
            c.w32(8, t306);
            let r307 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r307);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b45a5); }
            let Some(t308) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b45a7); };
            c.w64(3, t308);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b45b8, 0x70008b45b3);
        }
        235 => {
            return RegfileExit::fault(0x70008b45b8);
        }
        236 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b45bb); };
            c.w64(3, t309);
            return c.call(0x700086ed74, 0x70008b45c7, 0x70008b45c2);
        }
        237 => {
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b45c7); };
            c.w64(6, t310);
            blk = 1; continue;
        }
        238 => {
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0xc38)) else { return RegfileExit::fault(0x70008b45d0); };
            c.w64(3, t311);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b45d7); };
            c.w64(6, t312);
            c.w64(0, c.r64(9));
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b45de); };
            c.w64(1, t313);
            c.w64(12, c.r64(1));
            return c.call(0x7000871cb4, 0x70008b45ed, 0x70008b45e8);
        }
        239 => {
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b45ed); };
            c.w64(8, t314);
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b45f1); };
            c.w64(1, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b45f5); };
            c.w64(7, t316);
            let Some(t317) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b45f9); };
            c.w64(11, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008b4600); };
            c.w64(0, t318);
            blk = 23; continue;
        }
        240 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b460c); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008b4618, 0x70008b4613);
        }
        241 => {
            c.w64(11, c.r64(0));
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b461b); };
            c.w64(8, t319);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b461f); };
            c.w64(1, t320);
            c.w32(12, 0x8u64);
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b4629); };
            c.w64(3, t321);
            let Some(t322) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b4630); };
            c.w64(0, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b4637); };
            c.w64(7, t323);
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b463b); };
            c.w64(6, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b463f); };
            c.w64(9, t325);
            blk = 29; continue;
        }
        242 => {
            let Some(t326) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4648); };
            c.logic(t326 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 194; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4655); };
            c.w64(0, t327);
            blk = 71; continue;
        }
        244 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b465e); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 194; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4671); };
            c.w64(0, t328);
            blk = 71; continue;
        }
        246 => {
            let Some(t329) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b467a); };
            c.logic(t329 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 247; continue;
        }
        247 => {
            blk = 194; continue;
        }
        248 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b468d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 194; continue; }
            blk = 249; continue;
        }
        249 => {
            blk = 71; continue;
        }
        250 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x70008b46a6); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x70008b46aa); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008b46b8, 0x70008b46b3);
        }
        251 => {
            c.w64(3, c.r64(0));
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b46bb); };
            c.w64(8, t330);
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b46bf); };
            c.w64(15, t331);
            let Some(t332) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b46c6); };
            c.w64(1, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b46cd); };
            c.w64(6, t333);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b46d1); };
            c.w64(0, t334);
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b46d5); };
            c.w64(12, t335);
            let Some(t336) = c.ld64(c.r64(13).wrapping_add(0x1090)) else { return RegfileExit::fault(0x70008b46d9); };
            c.w64(11, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b46e0); };
            c.w64(9, t337);
            blk = 101; continue;
        }
        252 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(11)) { return RegfileExit::fault(0x70008b46e9); }
            c.w64(8, c.r64(12));
            blk = 253; continue;
        }
        253 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 257; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t338) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4702); };
            c.w32(9, t338);
            let r339 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r339);
            let Some(t340) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4709); };
            c.cmp(t340, 0x80bu64, 16);
            if c.cc_ne() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t341) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4712); };
            c.w32(8, t341);
            let r342 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r342);
            blk = 261; continue;
        }
        256 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 258; continue;
        }
        257 => {
            c.w32(9, 0x1u64);
            blk = 258; continue;
        }
        258 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 262; continue; }
            blk = 259; continue;
        }
        259 => {
            let Some(t343) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b472c); };
            c.w32(9, t343);
            let r344 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r344);
            let Some(t345) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4733); };
            c.cmp(t345, 0xbcu64, 16);
            if c.cc_ne() { blk = 262; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t346) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b473c); };
            c.w32(8, t346);
            let r347 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r347);
            blk = 261; continue;
        }
        261 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 253; continue;
        }
        262 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 265; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t348) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b474d); };
            c.w32(9, t348);
            let r349 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r349);
            let Some(t350) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4754); };
            c.w32(9, t350);
            let r351 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r351);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t352) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b4766); };
            c.w32(8, t352);
            let r353 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r353);
            blk = 279; continue;
        }
        265 => {
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b4772); };
            c.w64(3, t354);
            blk = 266; continue;
        }
        266 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 270; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t355) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4787); };
            c.w32(9, t355);
            let r356 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r356);
            let Some(t357) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b478e); };
            c.cmp(t357, 0x80bu64, 16);
            if c.cc_ne() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t358) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4797); };
            c.w32(8, t358);
            let r359 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r359);
            blk = 274; continue;
        }
        269 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 271; continue;
        }
        270 => {
            c.w32(9, 0x1u64);
            blk = 271; continue;
        }
        271 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 275; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t360) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b47b1); };
            c.w32(9, t360);
            let r361 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r361);
            let Some(t362) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b47b8); };
            c.cmp(t362, 0xbcu64, 16);
            if c.cc_ne() { blk = 275; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t363) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b47c1); };
            c.w32(8, t363);
            let r364 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r364);
            blk = 274; continue;
        }
        274 => {
            c.w64(3, c.r64(8));
            blk = 266; continue;
        }
        275 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t365) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b47d3); };
            c.w32(8, t365);
            let r366 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r366);
            let Some(t367) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b47da); };
            c.w32(8, t367);
            let r368 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r368);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 278; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t369) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b47ec); };
            c.w32(8, t369);
            let r370 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r370);
            blk = 279; continue;
        }
        278 => {
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b47f5); };
            c.w64(8, t371);
            blk = 279; continue;
        }
        279 => {
            let Some(t372) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b47f9); };
            c.w32(9, t372);
            let r373 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r373);
            let Some(t374) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b4800); };
            c.w32(9, t374);
            let r375 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r375);
            let Some(t376) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4807); };
            c.w32(3, t376);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008b4811); }
            c.cmp(c.r64(3), 0x5u64, 64);
            if c.cc_be() { blk = 317; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t377) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b481f); };
            c.w32(9, t377);
            let Some(t378) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b4823); };
            c.w64(3, t378);
            c.w64(8, c.r64(3).wrapping_add(0x18));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x70008b482a); }
            let Some(t379) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b4831); };
            c.cmp(t379, c.r64(8), 64);
            if c.cc_a() { blk = 283; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w32(2, 0x18u64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febb4, 0x70008b4843, 0x70008b483e);
        }
        282 => {
            blk = 284; continue;
        }
        283 => {
            if !c.st64(c.r64(15), c.r64(8)) { return RegfileExit::fault(0x70008b4845); }
            c.w64(8, c.r64(3).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 284; continue;
        }
        284 => {
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0xd28)) else { return RegfileExit::fault(0x70008b484f); };
            c.w64(8, t380);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b4856); }
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008b485a); };
            c.w64(11, t381);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b4861); }
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b4865); };
            c.w64(8, t382);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b4869); }
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b486d); };
            c.w64(12, t383);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008b4871); }
            let Some(t384) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b4875); };
            c.w64(9, t384);
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b487c); }
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008b4880); };
            c.w64(9, t385);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b4887); }
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b488b); };
            c.w64(9, t386);
            let Some(t387) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b488f); };
            c.w32(11, t387);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70008b4893); }
            c.logic(c.r8(11) & 0x8u64, 8);
            if c.cc_ne() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b489d); }
            let Some(t388) = c.ld64(c.r64(13).wrapping_add(0x25b8)) else { return RegfileExit::fault(0x70008b489f); };
            c.w64(3, t388);
            c.w32(0, 0x1u64);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b48ab); };
            c.w64(6, t389);
            return c.call(0x700086ed74, 0x70008b48b4, 0x70008b48af);
        }
        286 => {
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b48b4); };
            c.w64(9, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b48b8); };
            c.w64(8, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b48bc); };
            c.w64(0, t392);
            blk = 287; continue;
        }
        287 => {
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(9));
            blk = 146; continue;
        }
        288 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008b48d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008b48d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(11)) { return RegfileExit::fault(0x70008b48d8); }
            return c.call(0x70007febb4, 0x70008b48e4, 0x70008b48df);
        }
        289 => {
            c.w64(15, c.r64(0));
            let Some(t393) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b48e7); };
            c.w64(12, t393);
            let Some(t394) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b48ee); };
            c.w64(1, t394);
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b48f5); };
            c.w64(6, t395);
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b48f9); };
            c.w64(3, t396);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b48fd); };
            c.w64(0, t397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b4901); };
            c.w64(7, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b4905); };
            c.w64(8, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b4909); };
            c.w64(9, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008b490d); };
            c.w64(11, t401);
            blk = 145; continue;
        }
        290 => {
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008b491e); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(0)) { return RegfileExit::fault(0x70008b4922); }
            return c.call(0x70007febb4, 0x70008b492e, 0x70008b4929);
        }
        291 => {
            c.w64(15, c.r64(0));
            let Some(t402) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b4931); };
            c.w64(9, t402);
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b4938); };
            c.w64(1, t403);
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b493f); };
            c.w64(6, t404);
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b4943); };
            c.w64(3, t405);
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b4947); };
            c.w64(12, t406);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b494b); };
            c.w64(11, t407);
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b494f); };
            c.w64(7, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b4953); };
            c.w64(8, t409);
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008b4957); };
            c.w64(0, t410);
            blk = 176; continue;
        }
        292 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b4963); }
            let Some(t411) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b4964); };
            if !c.push64(t411) { return RegfileExit::fault(0x70008b4964); }
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4967); };
            c.w64(6, t412);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x70008b4975, 0x70008b4970);
        }
        293 => {
            blk = 185; continue;
        }
        294 => {
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0xc40)) else { return RegfileExit::fault(0x70008b497a); };
            c.w64(3, t413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b4981); };
            c.w64(1, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4985); };
            c.w64(6, t415);
            return c.call(0x7000871cb4, 0x70008b498e, 0x70008b4989);
        }
        295 => {
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b498e); };
            c.w64(8, t416);
            blk = 193; continue;
        }
        296 => {
            let Some(t417) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b499a); };
            c.w64(2, t417);
            let Some(t418) = c.ld32(c.r64(2).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008b49a1); };
            c.w32(2, t418);
            let r419 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r419);
            return c.call(0x7000824234, 0x70008b49af, 0x70008b49aa);
        }
        297 => {
            return RegfileExit::fault(0x70008b49af);
        }
        298 => {
            return RegfileExit::fault(0x70008b49b0);
        }
        299 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            return RegfileExit::fault(0x70008b49b7);
        }
        301 => {
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b49b8); };
            c.w64(6, t420);
            let Some(t421) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b49bc); };
            c.w32(8, t421);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t422) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b49c3); };
            c.w32(9, t422);
            let r423 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r423);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008b49cd); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            return RegfileExit::fault(0x70008b49d7);
        }
        303 => {
            let Some(t424) = c.ld32(c.r64(6).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008b49d8); };
            c.w32(8, t424);
            let Some(t425) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b49df); };
            c.w32(11, t425);
            let Some(t426) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b49e3); };
            c.w64(12, t426);
            let Some(t427) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b49ea); };
            c.w64(15, t427);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008b49f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008b49f6); }
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b49fa); };
            c.w64(3, t428);
            let Some(t429) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b4a01); };
            c.cmp(t429, c.r64(0), 64);
            if c.cc_a() { blk = 306; continue; }
            blk = 304; continue;
        }
        304 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x70008b4a10, 0x70008b4a0b);
        }
        305 => {
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b4a10); };
            c.w64(8, t430);
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b4a14); };
            c.w64(6, t431);
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4a18); };
            c.w64(11, t432);
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b4a1c); };
            c.w64(9, t433);
            blk = 307; continue;
        }
        306 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008b4a22); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 307; continue;
        }
        307 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b4a2d); }
            let Some(t434) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b4a31); };
            c.w64(15, t434);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008b4a38); }
            c.w64(8, c.r64(9));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b4a3f); }
            c.w64(8, c.r64(11));
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b4a46); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b4a4a); }
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b4a4b); };
            c.w64(8, t435);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b4a52); }
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b4a54); };
            c.w64(7, t436);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b4a62, 0x70008b4a5d);
        }
        308 => {
            blk = 220; continue;
        }
        309 => {
            let Some(t437) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b4a67); };
            c.w64(2, t437);
            let Some(t438) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008b4a6e); };
            c.w32(2, t438);
            let r439 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r439);
            return c.call(0x7000824234, 0x70008b4a7c, 0x70008b4a77);
        }
        310 => {
            return RegfileExit::fault(0x70008b4a7c);
        }
        311 => {
            return RegfileExit::fault(0x70008b4a7d);
        }
        312 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b4a7e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008b4a82); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008b4a86); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008b4a8a); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008b4a96, 0x70008b4a91);
        }
        313 => {
            c.w64(15, c.r64(0));
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b4a99); };
            c.w64(0, t440);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4a9d); };
            c.w64(9, t441);
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b4aa1); };
            c.w64(11, t442);
            c.w32(3, 0x8u64);
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4aaa); };
            c.w64(12, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b4aae); };
            c.w64(8, t444);
            blk = 209; continue;
        }
        314 => {
            let Some(t445) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008b4ab7); };
            c.w64(3, t445);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b4ac7, 0x70008b4ac2);
        }
        315 => {
            return RegfileExit::fault(0x70008b4ac7);
        }
        316 => {
            blk = 329; continue;
        }
        317 => {
            return RegfileExit::fault(0x70008b4acd);
        }
        318 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b4ace); }
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x70008b4ad2); };
            c.w64(3, t446);
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b4ad9); };
            c.w64(0, t447);
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4add); };
            c.w64(6, t448);
            return c.call(0x7000870914, 0x70008b4ae6, 0x70008b4ae1);
        }
        319 => {
            let Some(t449) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b4ae6); };
            c.cmp(t449, c.r32(0), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t450) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b4aef); };
            c.cmp(t450, c.r32(0), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b4af8); };
            if !c.push64(t451) { return RegfileExit::fault(0x70008b4af8); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4b00); };
            c.w64(6, t452);
            return c.call(0x70007f2274, 0x70008b4b09, 0x70008b4b04);
        }
        322 => {
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4b09); };
            c.w64(0, t453);
            blk = 329; continue;
        }
        323 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b4b0f); }
            let Some(t454) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008b4b10); };
            c.w64(3, t454);
            c.w32(0, 0x1u64);
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4b1c); };
            c.w64(6, t455);
            return c.call(0x700086ed74, 0x70008b4b25, 0x70008b4b20);
        }
        324 => {
            return RegfileExit::fault(0x70008b4b25);
        }
        325 => {
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x70008b4b26); };
            c.w64(3, t456);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4b2f); };
            c.w64(6, t457);
            return c.call(0x700086ed74, 0x70008b4b38, 0x70008b4b33);
        }
        326 => {
            return RegfileExit::fault(0x70008b4b38);
        }
        327 => {
            let Some(t458) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b4b39); };
            if !c.push64(t458) { return RegfileExit::fault(0x70008b4b39); }
            let Some(t459) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x70008b4b3c); };
            c.w64(3, t459);
            c.w32(0, 0x1u64);
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4b48); };
            c.w64(6, t460);
            return c.call(0x700086ed74, 0x70008b4b51, 0x70008b4b4c);
        }
        328 => {
            return RegfileExit::fault(0x70008b4b51);
        }
        329 => {
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b4b52); };
            c.w64(8, t461);
            let Some(t462) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4b56); };
            c.w32(7, t462);
            let r463 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r463);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b4b5d); }
            let Some(t464) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b4b5e); };
            c.w64(9, t464);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b4b65); }
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4b67); };
            c.w64(6, t465);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b4b75, 0x70008b4b70);
        }
        330 => {
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b4b75); };
            c.w64(8, t466);
            let Some(t467) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4b79); };
            c.w32(0, t467);
            let r468 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r468);
            blk = 221; continue;
        }
        331 => {
            c.w32(15, c.r32(15));
            let r469 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r469);
            c.w64(8, 0xfffffffffffc0000u64);
            let r470 = c.r64(8) & c.r64(15);
            c.w64(8, r470);
            c.logic(r470, 64);
            let Some(t471) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b4b95); };
            c.logic(t471 & 0x2u64, 8);
            if c.cc_e() { blk = 220; continue; }
            blk = 332; continue;
        }
        332 => {
            c.w64(9, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b4ba4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b4ba5); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008b4bb2, 0x70008b4bad);
        }
        333 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b4bb2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b4bb4); }
            blk = 220; continue;
        }
        334 => {
            c.w32(15, c.r32(15));
            let r472 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r472);
            c.w64(8, 0xfffffffffffc0000u64);
            let r473 = c.r64(8) & c.r64(15);
            c.w64(8, r473);
            c.logic(r473, 64);
            let Some(t474) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b4bca); };
            c.logic(t474 & 0x2u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 335; continue;
        }
        335 => {
            c.w64(12, c.r64(11).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b4bd9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b4bda); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008b4be7, 0x70008b4be2);
        }
        336 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b4be7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b4be9); }
            blk = 178; continue;
        }
        337 => {
            return RegfileExit::jump(0x70008b4bf0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b4bf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b4bf4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b4bf8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b4bf9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b4bfa); }
            let r0 = c.sub(c.r64(4), 0x88u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b4c02); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b4c06); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 259; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4c10); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b4c17); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b4c1e); };
            c.w64(0, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008b4c22); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 257; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4c2e); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4c35); };
            c.w32(9, t9);
            let r10 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r10);
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 257; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b4c4e); };
            c.w64(3, t11);
            return c.call(0x70008b23d4, 0x70008b4c5a, 0x70008b4c55);
        }
        4 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b4c5a); };
            c.w64(6, t12);
            let Some(t13) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4c5e); };
            c.w32(8, t13);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b4c68); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008b4c75);
        }
        6 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b4c76); };
            c.w64(8, t14);
            let Some(t15) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b4c7a); };
            c.cmp(t15, c.r32(8), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x1000)) else { return RegfileExit::fault(0x70008b4c8b); };
            c.w64(11, t16);
            let Some(t17) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4c92); };
            c.cmp(t17, c.r32(9), 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b4ca4); };
            c.w64(9, t18);
            blk = 15; continue;
        }
        11 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b4cad); };
            c.w64(3, t19);
            c.w64(0, c.r64(8));
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4cb7); };
            c.w64(6, t20);
            return c.call(0x7000870914, 0x70008b4cc0, 0x70008b4cbb);
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4cc8); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b4ccf); };
            c.logic(t23 & 0x2u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(9, c.r64(0));
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(9)) { return RegfileExit::fault(0x70008b4cdd); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008b4ce1); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b4ce8); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4cec); };
            c.w64(6, t26);
            return c.call(0x7000870914, 0x70008b4cf5, 0x70008b4cf0);
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 351; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4cfd); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b4d04); };
            c.logic(t29 & 0x2u64, 8);
            if c.cc_e() { blk = 351; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b4d0f); };
            if !c.push64(t30) { return RegfileExit::fault(0x70008b4d0f); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4d17); };
            c.w64(6, t31);
            return c.call(0x70007f2274, 0x70008b4d20, 0x70008b4d1b);
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008b4d20); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4d2c); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4d33); };
            c.cmp(t34, 0xbau64, 16);
            if c.cc_b() { blk = 349; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008b4d40); };
            c.w64(3, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4d47); };
            c.w64(6, t36);
            return c.call(0x7000870914, 0x70008b4d50, 0x70008b4d4b);
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(0)) { return RegfileExit::fault(0x70008b4d50); }
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b4d57); };
            c.w64(8, t37);
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4d5b); };
            c.w32(1, t38);
            let r39 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r39);
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4d62); };
            c.w32(7, t40);
            let r41 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r41);
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4d69); };
            c.w32(9, t42);
            let r43 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008b4d70); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008b4d74); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b4d78); };
            c.w64(11, t44);
            let Some(t45) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008b4d7f); };
            c.cmp(t45, 0x0u64, 8);
            if c.cc_ne() { blk = 261; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b4d89); };
            c.w64(6, t46);
            let Some(t47) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4d8d); };
            c.w32(9, t47);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x79u64, 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::fault(0x70008b4d9d);
        }
        25 => {
            let Some(t48) = c.ld32(c.r64(6).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b4d9e); };
            c.w32(9, t48);
            c.w32(12, 0x8u64);
            c.w64(15, c.r64(12));
            let r49 = c.add(c.r64(15), 0x14u64, 64);
            c.w64(15, r49);
            if c.cc_o() { blk = 254; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b4db8); };
            c.w64(3, t50);
            let Some(t51) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b4dbf); };
            c.w64(2, t51);
            c.w64(11, c.r64(2).wrapping_add(c.r64(15)));
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b4dc6); };
            c.w64(0, t52);
            let Some(t53) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008b4dcd); };
            c.cmp(t53, c.r64(11), 64);
            if c.cc_be() { blk = 263; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 263; continue; }
            blk = 28; continue;
        }
        28 => {
            if !c.st64(c.r64(3), c.r64(11)) { return RegfileExit::fault(0x70008b4de3); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            blk = 29; continue;
        }
        29 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b4dea); }
            c.w32(9, 0xau64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b4df4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008b4df8); }
            c.w64(9, c.r64(12));
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 31; continue;
        }
        30 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008b4e04); }
            let r54 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r54);
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r64(9), 0x1cu64, 64);
            if c.cc_ne() { blk = 30; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4e13); };
            c.w32(9, t55);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x70008b4e23);
        }
        34 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b4e24); };
            c.w64(2, t56);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008b4e2b); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_a() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            return RegfileExit::fault(0x70008b4e35);
        }
        36 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b4e36); };
            c.w64(2, t57);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(2)) { return RegfileExit::fault(0x70008b4e3d); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008b4e47);
        }
        38 => {
            c.w32(2, 0x2u64);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x70008b4e4d); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008b4e57);
        }
        40 => {
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x70008b4e58); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x70008b4e62);
        }
        42 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b4e63); };
            c.w64(9, t58);
            if !c.st32(c.r64(11).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008b4e6a); }
            let Some(t59) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4e6e); };
            c.w32(8, t59);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x68u64, 64);
            if c.cc_a() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            return RegfileExit::fault(0x70008b4e7e);
        }
        44 => {
            let Some(t60) = c.ld32(c.r64(6).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008b4e7f); };
            c.w32(8, t60);
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(8)) { return RegfileExit::fault(0x70008b4e86); }
            blk = 46; continue;
        }
        45 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b4e8f); };
            c.w64(2, t61);
            c.w32(8, c.r64(2).wrapping_add(0x2));
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(6));
            c.w64(6, c.r64(12));
            c.w64(3, c.r64(9));
            c.w64(0, c.r64(1));
            blk = 46; continue;
        }
        46 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b4ea6); };
            if !c.push64(t62) { return RegfileExit::fault(0x70008b4ea6); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008b4ea9); }
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008b4ead); };
            c.w64(7, t63);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4eb6); };
            c.w64(6, t64);
            return c.call(0x70007f21d4, 0x70008b4ebf, 0x70008b4eba);
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b4ebf); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 347; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4ecb); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            let Some(t67) = c.ld32(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008b4ed2); };
            c.cmp(c.r32(8), t67, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t68) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4edf); };
            c.cmp(t68, 0xbau64, 16);
            if c.cc_b() { blk = 347; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x70008b4eec); };
            c.w64(3, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4ef3); };
            c.w64(6, t70);
            return c.call(0x7000870914, 0x70008b4efc, 0x70008b4ef7);
        }
        51 => {
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b4efc); };
            c.cmp(t71, c.r32(0), 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b4f0c); };
            c.cmp(t72, c.r32(0), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t73) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4f15); };
            c.w32(8, t73);
            let r74 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r74);
            let Some(t75) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b4f1c); };
            c.logic(t75 & 0x10u64, 8);
            if c.cc_ne() { blk = 58; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b4f23); };
            c.cmp(t76, c.r32(8), 32);
            if c.cc_e() { blk = 267; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t77) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4f30); };
            c.cmp(t77, 0x41u64, 16);
            if c.cc_e() { blk = 265; continue; }
            blk = 57; continue;
        }
        57 => {
            blk = 216; continue;
        }
        58 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4f41); };
            c.w64(0, t78);
            blk = 71; continue;
        }
        59 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4f4a); };
            c.w64(0, t79);
            blk = 71; continue;
        }
        60 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 216; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4f5d); };
            c.w64(0, t80);
            blk = 71; continue;
        }
        62 => {
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b4f63); };
            c.w64(0, t81);
            blk = 71; continue;
        }
        63 => {
            let Some(t82) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b4f69); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            let Some(t84) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b4f70); };
            c.cmp(t84, c.r32(8), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b4f82); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t86) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4f8b); };
            c.w32(9, t86);
            let r87 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r87);
            let Some(t88) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b4f92); };
            c.logic(t88 & 0x10u64, 8);
            if c.cc_ne() { blk = 71; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b4f99); };
            c.cmp(t89, c.r32(9), 32);
            if c.cc_e() { blk = 271; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t90) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b4fa6); };
            c.cmp(t90, 0x41u64, 16);
            if c.cc_e() { blk = 269; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 216; continue;
        }
        70 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 216; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008b4fc1); };
            c.w64(8, t91);
            let Some(t92) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b4fc8); };
            c.cmp(t92, c.r32(8), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008b4fce); };
            c.w64(3, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b4fd5); };
            c.w64(6, t94);
            return c.call(0x7000870914, 0x70008b4fde, 0x70008b4fd9);
        }
        73 => {
            blk = 75; continue;
        }
        74 => {
            let Some(t95) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b4fe0); };
            c.w32(0, t95);
            let r96 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r96);
            blk = 75; continue;
        }
        75 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b4fe6); }
            let Some(t97) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b4fea); };
            c.cmp(t97, 0x1ffffeu64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b4ff7); };
            c.w64(8, t98);
            let Some(t99) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b4ffb); };
            c.w32(9, t99);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_be() { blk = 246; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b500f); };
            c.w32(9, t100);
            let r101 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r101);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b501a); }
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b501e); };
            c.w64(6, t102);
            let Some(t103) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5022); };
            c.w32(9, t103);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_be() { blk = 247; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t104) = c.ld32(c.r64(6).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b5039); };
            c.w32(9, t104);
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x1098)) else { return RegfileExit::fault(0x70008b5040); };
            c.w64(11, t105);
            let Some(t106) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5047); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(13).wrapping_add(0x63f0));
            let Some(t108) = c.ld64(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b505e); };
            c.w64(12, t108);
            blk = 99; continue;
        }
        80 => {
            let Some(t109) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5067); };
            c.w32(15, t109);
            let r110 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r110);
            let Some(t111) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b506f); };
            c.w32(15, t111);
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_l() { blk = 87; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_l() { blk = 85; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(15), 0x9du64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(15), 0x9eu64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(15), 0x5eu64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_l() { blk = 90; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(15), 0x53u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(15), 0x48u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(15), 0x49u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t112) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b50c8); };
            c.w32(12, t112);
            blk = 99; continue;
        }
        93 => {
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b50cf); };
            c.w64(12, t113);
            blk = 99; continue;
        }
        94 => {
            let Some(t114) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b50d8); };
            c.w32(12, t114);
            blk = 99; continue;
        }
        95 => {
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b50df); };
            c.w64(12, t115);
            blk = 99; continue;
        }
        96 => {
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b50e8); };
            c.w64(12, t116);
            blk = 99; continue;
        }
        97 => {
            let Some(t117) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b50f1); };
            c.w32(12, t117);
            blk = 99; continue;
        }
        98 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b50f8); };
            c.w64(12, t118);
            blk = 99; continue;
        }
        99 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b50ff); };
            c.w64(15, t119);
            let Some(t120) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b5106); };
            c.w64(3, t120);
            c.w64(2, c.r64(3).wrapping_add(0x1c));
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b510d); };
            c.w64(1, t121);
            let Some(t122) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b5114); };
            c.cmp(t122, c.r64(2), 64);
            if c.cc_be() { blk = 273; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008b511d); }
            let r123 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r123);
            blk = 101; continue;
        }
        101 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b5124); }
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b5128); };
            c.w64(9, t124);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b512f); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b5133); }
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b5137); };
            c.w64(2, t125);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008b513e); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b5141); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b5145); }
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008b5149); }
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b514d); };
            c.w64(11, t126);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b5151); }
            let Some(t127) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5155); };
            c.w32(12, t127);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x70008b515f); }
            c.cmp(c.r64(12), 0xbeu64, 64);
            if c.cc_be() { blk = 248; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t128) = c.ld32(c.r64(6).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b5170); };
            c.w32(12, t128);
            let Some(t129) = c.ld64(c.r64(13).wrapping_add(0x10a0)) else { return RegfileExit::fault(0x70008b5177); };
            c.w64(7, t129);
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b517e); };
            c.w32(3, t130);
            let r131 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r131);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(13).wrapping_add(0x63f0));
            let Some(t132) = c.ld64(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b5193); };
            c.w64(3, t132);
            blk = 123; continue;
        }
        104 => {
            let Some(t133) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b519c); };
            c.w32(6, t133);
            let r134 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r134);
            let Some(t135) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b51a2); };
            c.w32(6, t135);
            c.cmp(c.r32(6), 0x5cu64, 32);
            if c.cc_l() { blk = 111; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(6), 0x85u64, 32);
            if c.cc_l() { blk = 109; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(6), 0x85u64, 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 107; continue;
        }
        107 => {
            c.cmp(c.r32(6), 0x9du64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(6), 0x9eu64, 32);
            if c.cc_e() { blk = 119; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(6), 0x5cu64, 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r32(6), 0x5eu64, 32);
            if c.cc_e() { blk = 118; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(6), 0x4au64, 32);
            if c.cc_l() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(6), 0x4au64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(6), 0x53u64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(6), 0x48u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(6), 0x49u64, 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t136) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b51ee); };
            c.w32(3, t136);
            blk = 123; continue;
        }
        117 => {
            let Some(t137) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b51f3); };
            c.w64(3, t137);
            blk = 123; continue;
        }
        118 => {
            let Some(t138) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b51fc); };
            c.w32(3, t138);
            blk = 123; continue;
        }
        119 => {
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b5201); };
            c.w64(3, t139);
            blk = 123; continue;
        }
        120 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b520a); };
            c.w64(3, t140);
            blk = 123; continue;
        }
        121 => {
            let Some(t141) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5213); };
            c.w32(3, t141);
            blk = 123; continue;
        }
        122 => {
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b5218); };
            c.w64(3, t142);
            blk = 123; continue;
        }
        123 => {
            let Some(t143) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b521f); };
            c.w64(6, t143);
            c.w64(0, c.r64(6).wrapping_add(0x1c));
            let Some(t144) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b5226); };
            c.cmp(t144, c.r64(0), 64);
            if c.cc_be() { blk = 275; continue; }
            blk = 124; continue;
        }
        124 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x70008b522f); }
            c.w64(0, c.r64(6).wrapping_add(0x1));
            blk = 125; continue;
        }
        125 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008b5236); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b523a); }
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b523e); }
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008b5242); }
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(7)) { return RegfileExit::fault(0x70008b5245); }
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b5248); }
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70008b524c); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b524f); }
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b5253); };
            c.w64(7, t145);
            let Some(t146) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b5257); };
            c.cmp(t146, c.r32(7), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x70008b5266); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(12)) { return RegfileExit::fault(0x70008b526a); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b5278); };
            c.w64(3, t147);
            let Some(t148) = c.ld32(c.r64(3)) else { return RegfileExit::fault(0x70008b527f); };
            c.cmp(t148, 0x0u64, 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0xfd0)) else { return RegfileExit::fault(0x70008b5288); };
            c.w64(3, t149);
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t150) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b5291); };
            c.cmp(t150, c.r32(6), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t151) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b529a); };
            c.logic(t151 & 0x1u64, 8);
            if c.cc_ne() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w32(3, 0x1u64);
            blk = 131; continue;
        }
        130 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 205; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b52b2); };
            c.w64(3, t152);
            let Some(t153) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b52b6); };
            c.w32(2, t153);
            let r154 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r154);
            let Some(t155) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b52bc); };
            c.cmp(t155, 0x83du64, 16);
            if c.cc_ne() { blk = 137; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b52c4); };
            c.w64(12, t156);
            let Some(t157) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b52c8); };
            c.w32(8, t157);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t158) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b52d0); };
            c.w32(2, t158);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x97u64, 64);
            if c.cc_be() { blk = 249; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t159) = c.ld32(c.r64(12).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b52e3); };
            c.cmp(t159, c.r32(2), 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008b52ed); };
            c.w64(8, t160);
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b52f4); };
            c.cmp(t161, c.r32(6), 32);
            c.w8(8, (c.cc_ne() as u64));
            c.w32(8, c.r8(8));
            blk = 138; continue;
        }
        136 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 138; continue;
        }
        137 => {
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b530a); };
            c.w64(12, t162);
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t163) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5318); };
            c.w32(8, t163);
            c.w32(8, c.r32(8));
            let r164 = c.r32(8) & 0x6u64;
            c.w32(8, r164);
            c.logic(r164, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t165) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b5329); };
            c.w32(2, t165);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_ne() { blk = 277; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b5336); };
            c.w64(8, t166);
            blk = 142; continue;
        }
        142 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t167) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5349); };
            c.w32(9, t167);
            let r168 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r168);
            let Some(t169) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5350); };
            c.cmp(t169, 0x80bu64, 16);
            if c.cc_ne() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t170) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b5359); };
            c.w32(8, t170);
            let r171 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r171);
            blk = 142; continue;
        }
        145 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 147; continue;
        }
        146 => {
            c.w32(9, 0x1u64);
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t172) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5373); };
            c.w32(9, t172);
            let r173 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r173);
            let Some(t174) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b537a); };
            c.cmp(t174, 0xbcu64, 16);
            if c.cc_ne() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t175) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5383); };
            c.w32(8, t175);
            let r176 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r176);
            blk = 142; continue;
        }
        150 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t177) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5391); };
            c.w32(9, t177);
            let r178 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r178);
            let Some(t179) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5398); };
            c.w32(9, t179);
            let r180 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r180);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 153; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b53aa); };
            c.w32(8, t181);
            let r182 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r182);
            blk = 166; continue;
        }
        153 => {
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b53b6); };
            c.w64(0, t183);
            blk = 154; continue;
        }
        154 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 158; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t184) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b53ca); };
            c.w32(9, t184);
            let r185 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r185);
            let Some(t186) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b53d1); };
            c.cmp(t186, 0x80bu64, 16);
            if c.cc_ne() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t187) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b53da); };
            c.w32(0, t187);
            let r188 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r188);
            blk = 154; continue;
        }
        157 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 159; continue;
        }
        158 => {
            c.w32(9, 0x1u64);
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t189) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b53f3); };
            c.w32(9, t189);
            let r190 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r190);
            let Some(t191) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b53fa); };
            c.cmp(t191, 0xbcu64, 16);
            if c.cc_ne() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t192) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5403); };
            c.w32(0, t192);
            let r193 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r193);
            blk = 154; continue;
        }
        162 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 165; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t194) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5411); };
            c.w32(8, t194);
            let r195 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r195);
            let Some(t196) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5418); };
            c.w32(8, t196);
            let r197 = c.sub(c.r32(8), 0x80cu64, 32);
            c.w32(8, r197);
            c.cmp(c.r32(8), 0xeu64, 32);
            if c.cc_a() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t198) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b542a); };
            c.w32(8, t198);
            let r199 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r199);
            blk = 166; continue;
        }
        165 => {
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5433); };
            c.w64(8, t200);
            blk = 166; continue;
        }
        166 => {
            let Some(t201) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5437); };
            c.w32(9, t201);
            let r202 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r202);
            let Some(t203) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b543e); };
            c.w32(9, t203);
            let r204 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r204);
            let Some(t205) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5445); };
            c.w32(0, t205);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x5u64, 64);
            if c.cc_be() { blk = 250; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t206) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b5459); };
            c.w32(9, t206);
            let Some(t207) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b545d); };
            c.w64(0, t207);
            c.w64(11, c.r64(0).wrapping_add(0x18));
            let Some(t208) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b5464); };
            c.cmp(t208, c.r64(11), 64);
            if c.cc_be() { blk = 312; continue; }
            blk = 168; continue;
        }
        168 => {
            if !c.st64(c.r64(15), c.r64(11)) { return RegfileExit::fault(0x70008b546d); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 169; continue;
        }
        169 => {
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0xd20)) else { return RegfileExit::fault(0x70008b5474); };
            c.w64(0, t209);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008b547b); }
            c.w64(0, c.r64(2));
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008b5482); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b5486); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b548a); };
            c.w64(0, t210);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008b548e); }
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b5492); };
            c.w64(0, t211);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(0)) { return RegfileExit::fault(0x70008b5499); }
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b549d); }
            blk = 170; continue;
        }
        170 => {
            c.w64(0, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x700081f374, 0x70008b54ac, 0x70008b54a7);
        }
        171 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b54ae); };
            c.w64(9, t212);
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b54b5); };
            c.w64(1, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b54bc); };
            c.w64(12, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b54c0); };
            c.w64(3, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b54c4); };
            c.w64(7, t216);
            blk = 202; continue;
        }
        172 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b54cd); };
            c.w64(8, t217);
            blk = 173; continue;
        }
        173 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(2, (c.cc_e() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 177; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t218) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b54e0); };
            c.w32(9, t218);
            let r219 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r219);
            let Some(t220) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b54e7); };
            c.cmp(t220, 0x80bu64, 16);
            if c.cc_ne() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t221) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b54f0); };
            c.w32(8, t221);
            let r222 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r222);
            blk = 173; continue;
        }
        176 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 178; continue;
        }
        177 => {
            c.w32(9, 0x1u64);
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 181; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t223) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b550a); };
            c.w32(9, t223);
            let r224 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r224);
            let Some(t225) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5511); };
            c.cmp(t225, 0xbcu64, 16);
            if c.cc_ne() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t226) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b551a); };
            c.w32(8, t226);
            let r227 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r227);
            blk = 173; continue;
        }
        181 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 184; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t228) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5528); };
            c.w32(9, t228);
            let r229 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r229);
            let Some(t230) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b552f); };
            c.w32(9, t230);
            let r231 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r231);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t232) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5541); };
            c.w32(8, t232);
            let r233 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r233);
            blk = 197; continue;
        }
        184 => {
            c.w64(8, c.r64(0));
            blk = 185; continue;
        }
        185 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 189; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t234) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5562); };
            c.w32(2, t234);
            let r235 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r235);
            let Some(t236) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5569); };
            c.cmp(t236, 0x80bu64, 16);
            if c.cc_ne() { blk = 188; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t237) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b5571); };
            c.w32(8, t237);
            let r238 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r238);
            blk = 185; continue;
        }
        188 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 190; continue;
        }
        189 => {
            c.w32(2, 0x1u64);
            blk = 190; continue;
        }
        190 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 193; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t239) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5588); };
            c.w32(2, t239);
            let r240 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r240);
            let Some(t241) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b558f); };
            c.cmp(t241, 0xbcu64, 16);
            if c.cc_ne() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t242) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5597); };
            c.w32(8, t242);
            let r243 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r243);
            blk = 185; continue;
        }
        193 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 196; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t244) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b55a6); };
            c.w32(9, t244);
            let r245 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r245);
            let Some(t246) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b55ad); };
            c.w32(9, t246);
            let r247 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r247);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t248) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b55bf); };
            c.w32(8, t248);
            let r249 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r249);
            blk = 197; continue;
        }
        196 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b55c8); };
            c.w64(8, t250);
            blk = 197; continue;
        }
        197 => {
            let Some(t251) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b55cc); };
            c.w32(8, t251);
            let r252 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r252);
            let Some(t253) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b55d3); };
            c.w32(8, t253);
            let r254 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r254);
            let Some(t255) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b55da); };
            c.w32(9, t255);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t256) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b55e1); };
            c.w32(2, t256);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x5u64, 64);
            if c.cc_be() { blk = 251; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t257) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b55f1); };
            c.w32(8, t257);
            c.w64(9, c.r64(15));
            let Some(t258) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b55f8); };
            c.w64(15, t258);
            c.w64(11, c.r64(15).wrapping_add(0x18));
            let Some(t259) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b55ff); };
            c.cmp(t259, c.r64(11), 64);
            if c.cc_be() { blk = 314; continue; }
            blk = 199; continue;
        }
        199 => {
            if !c.st64(c.r64(9), c.r64(11)) { return RegfileExit::fault(0x70008b5608); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 200; continue;
        }
        200 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0xdf8)) else { return RegfileExit::fault(0x70008b560f); };
            c.w64(15, t260);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008b5616); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008b561a); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008b561e); }
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b5622); };
            c.w64(15, t261);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b5626); }
            let Some(t262) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b562a); };
            c.w64(15, t262);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008b5631); }
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(8)) { return RegfileExit::fault(0x70008b5635); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b5639); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r263 = c.r64(8) & c.r64(3);
            c.w64(8, r263);
            c.logic(r263, 64);
            let Some(t264) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b564d); };
            c.logic(t264 & 0x4u64, 8);
            if c.cc_ne() { blk = 358; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t265) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5658); };
            c.w32(8, t265);
            let r266 = c.r64(8) | 0x8u64;
            c.w64(8, r266);
            c.logic(r266, 64);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b5660); }
            blk = 45; continue;
        }
        203 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b5669); };
            c.w64(8, t267);
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b566d); };
            c.w64(12, t268);
            blk = 205; continue;
        }
        204 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 316; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b567f); };
            c.w64(0, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5683); };
            c.w64(3, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5687); };
            c.w64(6, t271);
            return c.call(0x70008ba474, 0x70008b5690, 0x70008b568b);
        }
        207 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b5690); }
            let Some(t272) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b5694); };
            c.w64(3, t272);
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b569b); };
            c.w64(6, t273);
            return c.call(0x7000870914, 0x70008b56a4, 0x70008b569f);
        }
        208 => {
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b56a4); };
            if !c.push64(t274) { return RegfileExit::fault(0x70008b56a4); }
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b56a7); };
            if !c.push64(t275) { return RegfileExit::fault(0x70008b56a7); }
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b56aa); };
            if !c.push64(t276) { return RegfileExit::fault(0x70008b56aa); }
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b56ad); };
            c.w64(6, t277);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x70008b56c1, 0x70008b56bc);
        }
        209 => {
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b56c1); };
            c.w64(8, t278);
            let Some(t279) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x70008b56c8); };
            c.cmp(t279, 0x0u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 210; continue;
        }
        210 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t280) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b56d2); };
            c.w32(9, t280);
            let r281 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r281);
            let Some(t282) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b56d9); };
            c.cmp(t282, 0x83du64, 16);
            if c.cc_ne() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w32(9, 0x1u64);
            blk = 214; continue;
        }
        213 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 318; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t283) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b56f9); };
            c.w64(9, t283);
            let Some(t284) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b5700); };
            c.w64(1, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b5707); };
            c.w64(12, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b570b); };
            c.w64(7, t286);
            blk = 45; continue;
        }
        216 => {
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b5714); };
            c.w64(0, t287);
            let Some(t288) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5718); };
            c.w32(8, t288);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_a() { blk = 218; continue; }
            blk = 217; continue;
        }
        217 => {
            return RegfileExit::fault(0x70008b5728);
        }
        218 => {
            let Some(t289) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5729); };
            c.w32(9, t289);
            let r290 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r290);
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b5734); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 320; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r64(8), 0x4u64, 64);
            c.w8(8, (c.cc_a() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_le() { blk = 323; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 222; continue; }
            blk = 221; continue;
        }
        221 => {
            return RegfileExit::fault(0x70008b575e);
        }
        222 => {
            let Some(t291) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b575f); };
            c.w64(8, (t291 as u32 as i32 as i64 as u64));
            let Some(t292) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b5763); };
            c.w32(9, t292);
            let r293 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r293);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r294 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r294);
            let Some(t295) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5771); };
            c.w32(11, t295);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), c.r64(8), 64);
            if c.cc_ge() { blk = 242; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_b() { blk = 333; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 239; continue; }
            blk = 225; continue;
        }
        225 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 338; continue; }
            blk = 226; continue;
        }
        226 => {
            let r296 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r296);
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r297 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r297);
            if c.cc_o() { blk = 238; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(3, 0x8u64);
            let r298 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r298);
            if c.cc_o() { blk = 237; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b57d2); };
            c.w64(2, t299);
            let Some(t300) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b57d9); };
            c.w64(1, t300);
            c.w64(7, c.r64(1).wrapping_add(c.r64(15)));
            let Some(t301) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b57e0); };
            c.w64(6, t301);
            let Some(t302) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008b57e7); };
            c.cmp(t302, c.r64(7), 64);
            if c.cc_be() { blk = 336; continue; }
            blk = 229; continue;
        }
        229 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 336; continue; }
            blk = 230; continue;
        }
        230 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x70008b57fd); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            blk = 231; continue;
        }
        231 => {
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008b5804); };
            c.w64(2, t303);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b580b); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b580f); }
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(12, c.r64(3));
            blk = 233; continue;
        }
        232 => {
            if !c.st32(c.r64(15).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b5820); }
            let r304 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r304);
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r64(8), c.r64(3), 64);
            if c.cc_e() { blk = 240; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w64(2, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(12), 64);
            if c.cc_e() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t305) = c.ld32(c.r64(9).wrapping_add(c.r64(12)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b583b); };
            c.w32(2, t305);
            let r306 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r306);
            blk = 232; continue;
        }
        236 => {
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b5846); };
            c.w64(2, t307);
            blk = 232; continue;
        }
        237 => {
            return RegfileExit::fault(0x70008b584f);
        }
        238 => {
            return RegfileExit::fault(0x70008b5850);
        }
        239 => {
            let Some(t308) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b5851); };
            c.w64(15, t308);
            blk = 240; continue;
        }
        240 => {
            if !c.st32(c.r64(0).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008b5858); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r309 = c.r64(8) & c.r64(0);
            c.w64(8, r309);
            c.logic(r309, 64);
            let Some(t310) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b586c); };
            c.logic(t310 & 0x4u64, 8);
            if c.cc_ne() { blk = 355; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b5877); };
            c.w64(0, t311);
            blk = 243; continue;
        }
        243 => {
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b587b); };
            c.w64(1, t312);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b5882); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            return c.ret(0x10, 0x70008b5889);
        }
        245 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b588c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b5893); }
            return c.ret(0x0, 0x70008b5895);
        }
        246 => {
            return RegfileExit::fault(0x70008b5896);
        }
        247 => {
            return RegfileExit::fault(0x70008b5897);
        }
        248 => {
            return RegfileExit::fault(0x70008b5898);
        }
        249 => {
            return RegfileExit::fault(0x70008b5899);
        }
        250 => {
            return RegfileExit::fault(0x70008b589a);
        }
        251 => {
            return RegfileExit::fault(0x70008b589b);
        }
        252 => {
            c.w32(8, 0x294u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b58a2); }
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b58a4); };
            c.w64(8, t313);
            let Some(t314) = c.ld32(c.r64(8).wrapping_add(0x22f)) else { return RegfileExit::fault(0x70008b58ab); };
            c.w32(8, t314);
            let r315 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r315);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b58b5); }
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008b58b7); };
            c.w64(3, t316);
            c.w32(0, 0x2u64);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b58c3); };
            c.w64(6, t317);
            return c.call(0x700086ed74, 0x70008b58cc, 0x70008b58c7);
        }
        253 => {
            return RegfileExit::fault(0x70008b58cc);
        }
        254 => {
            return RegfileExit::fault(0x70008b58cd);
        }
        255 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b58d4); }
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b58d6); };
            c.w64(8, t318);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b58dd); }
            let Some(t319) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b58df); };
            c.w64(3, t319);
            c.w32(0, 0x2u64);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b58eb); };
            c.w64(6, t320);
            return c.call(0x700086ed74, 0x70008b58f4, 0x70008b58ef);
        }
        256 => {
            return RegfileExit::fault(0x70008b58f4);
        }
        257 => {
            c.w32(8, 0x34u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b58fb); }
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b58fd); };
            c.w64(8, t321);
            let Some(t322) = c.ld32(c.r64(8).wrapping_add(0x233)) else { return RegfileExit::fault(0x70008b5904); };
            c.w32(8, t322);
            let r323 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r323);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b590e); }
            let Some(t324) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b5910); };
            c.w64(3, t324);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b5921, 0x70008b591c);
        }
        258 => {
            return RegfileExit::fault(0x70008b5921);
        }
        259 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t325) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b5924); };
            c.w64(3, t325);
            return c.call(0x700086ed74, 0x70008b5930, 0x70008b592b);
        }
        260 => {
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5930); };
            c.w64(6, t326);
            blk = 1; continue;
        }
        261 => {
            let Some(t327) = c.ld64(c.r64(13).wrapping_add(0xc38)) else { return RegfileExit::fault(0x70008b5939); };
            c.w64(3, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5940); };
            c.w64(6, t328);
            c.w64(0, c.r64(9));
            let Some(t329) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b5947); };
            c.w64(1, t329);
            c.w64(12, c.r64(1));
            return c.call(0x7000871cb4, 0x70008b5956, 0x70008b5951);
        }
        262 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b5956); };
            c.w64(8, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b595a); };
            c.w64(1, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b595e); };
            c.w64(7, t332);
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b5962); };
            c.w64(11, t333);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x70008b5969); };
            c.w64(0, t334);
            blk = 23; continue;
        }
        263 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b5975); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008b5981, 0x70008b597c);
        }
        264 => {
            c.w64(11, c.r64(0));
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b5984); };
            c.w64(8, t335);
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b5988); };
            c.w64(1, t336);
            c.w32(12, 0x8u64);
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b5992); };
            c.w64(3, t337);
            let Some(t338) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b5999); };
            c.w64(0, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b59a0); };
            c.w64(7, t339);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b59a4); };
            c.w64(6, t340);
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b59a8); };
            c.w64(9, t341);
            blk = 29; continue;
        }
        265 => {
            let Some(t342) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b59b1); };
            c.logic(t342 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 216; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b59be); };
            c.w64(0, t343);
            blk = 71; continue;
        }
        267 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b59c7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 216; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b59da); };
            c.w64(0, t344);
            blk = 71; continue;
        }
        269 => {
            let Some(t345) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b59e3); };
            c.logic(t345 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 270; continue;
        }
        270 => {
            blk = 216; continue;
        }
        271 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b59f6); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 216; continue; }
            blk = 272; continue;
        }
        272 => {
            blk = 71; continue;
        }
        273 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(12)) { return RegfileExit::fault(0x70008b5a0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(9)) { return RegfileExit::fault(0x70008b5a13); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008b5a21, 0x70008b5a1c);
        }
        274 => {
            c.w64(3, c.r64(0));
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b5a24); };
            c.w64(8, t346);
            let Some(t347) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b5a28); };
            c.w64(15, t347);
            let Some(t348) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b5a2f); };
            c.w64(1, t348);
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b5a36); };
            c.w64(6, t349);
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5a3a); };
            c.w64(0, t350);
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b5a3e); };
            c.w64(12, t351);
            let Some(t352) = c.ld64(c.r64(13).wrapping_add(0x1098)) else { return RegfileExit::fault(0x70008b5a42); };
            c.w64(11, t352);
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b5a49); };
            c.w64(9, t353);
            blk = 101; continue;
        }
        275 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(3)) { return RegfileExit::fault(0x70008b5a52); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(12)) { return RegfileExit::fault(0x70008b5a56); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008b5a67, 0x70008b5a62);
        }
        276 => {
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008b5a67); };
            c.w64(12, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b5a6e); };
            c.w64(8, t355);
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b5a72); };
            c.w64(9, t356);
            let Some(t357) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b5a79); };
            c.w64(15, t357);
            let Some(t358) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b5a80); };
            c.w64(1, t358);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b5a87); };
            c.w64(11, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b5a8b); };
            c.w64(3, t360);
            let Some(t361) = c.ld64(c.r64(13).wrapping_add(0x10a0)) else { return RegfileExit::fault(0x70008b5a8f); };
            c.w64(7, t361);
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b5a96); };
            c.w64(2, t362);
            blk = 125; continue;
        }
        277 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(2)) { return RegfileExit::fault(0x70008b5aa2); }
            c.w64(8, c.r64(0));
            blk = 278; continue;
        }
        278 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 282; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t363) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5abe); };
            c.w32(11, t363);
            let r364 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r364);
            let Some(t365) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5ac5); };
            c.cmp(t365, 0x80bu64, 16);
            if c.cc_ne() { blk = 281; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t366) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b5ace); };
            c.w32(8, t366);
            let r367 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r367);
            blk = 286; continue;
        }
        281 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 283; continue;
        }
        282 => {
            c.w32(11, 0x1u64);
            blk = 283; continue;
        }
        283 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 287; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t368) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5ae8); };
            c.w32(11, t368);
            let r369 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r369);
            let Some(t370) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5aef); };
            c.cmp(t370, 0xbcu64, 16);
            if c.cc_ne() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t371) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5af8); };
            c.w32(8, t371);
            let r372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r372);
            blk = 286; continue;
        }
        286 => {
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b5aff); };
            c.w64(11, t373);
            blk = 278; continue;
        }
        287 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 290; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t374) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5b0b); };
            c.w32(9, t374);
            let r375 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r375);
            let Some(t376) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5b12); };
            c.w32(9, t376);
            let r377 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r377);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t378) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5b24); };
            c.w32(8, t378);
            let r379 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r379);
            blk = 303; continue;
        }
        290 => {
            let Some(t380) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b5b30); };
            c.w64(8, t380);
            blk = 291; continue;
        }
        291 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 295; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t381) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5b46); };
            c.w32(11, t381);
            let r382 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r382);
            let Some(t383) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5b4d); };
            c.cmp(t383, 0x80bu64, 16);
            if c.cc_ne() { blk = 294; continue; }
            blk = 293; continue;
        }
        293 => {
            let Some(t384) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b5b56); };
            c.w32(8, t384);
            let r385 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r385);
            blk = 291; continue;
        }
        294 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 296; continue;
        }
        295 => {
            c.w32(11, 0x1u64);
            blk = 296; continue;
        }
        296 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 299; continue; }
            blk = 297; continue;
        }
        297 => {
            let Some(t386) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5b70); };
            c.w32(11, t386);
            let r387 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r387);
            let Some(t388) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5b77); };
            c.cmp(t388, 0xbcu64, 16);
            if c.cc_ne() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            let Some(t389) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5b80); };
            c.w32(8, t389);
            let r390 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r390);
            blk = 291; continue;
        }
        299 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 302; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t391) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5b8f); };
            c.w32(9, t391);
            let r392 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r392);
            let Some(t393) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5b96); };
            c.w32(9, t393);
            let r394 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r394);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_a() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            let Some(t395) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5ba8); };
            c.w32(8, t395);
            let r396 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r396);
            blk = 303; continue;
        }
        302 => {
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5bb1); };
            c.w64(8, t397);
            blk = 303; continue;
        }
        303 => {
            let Some(t398) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5bb5); };
            c.w32(9, t398);
            let r399 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r399);
            let Some(t400) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b5bbc); };
            c.w32(9, t400);
            let r401 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r401);
            let Some(t402) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5bc3); };
            c.w32(11, t402);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008b5bcd); }
            c.cmp(c.r64(11), 0x5u64, 64);
            if c.cc_be() { blk = 341; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t403) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b5bdb); };
            c.w32(9, t403);
            let Some(t404) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b5bdf); };
            c.w64(11, t404);
            c.w64(8, c.r64(11).wrapping_add(0x18));
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x70008b5be6); }
            let Some(t405) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b5bed); };
            c.cmp(t405, c.r64(8), 64);
            if c.cc_a() { blk = 307; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w32(2, 0x18u64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febb4, 0x70008b5bff, 0x70008b5bfa);
        }
        306 => {
            blk = 308; continue;
        }
        307 => {
            if !c.st64(c.r64(15), c.r64(8)) { return RegfileExit::fault(0x70008b5c01); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 308; continue;
        }
        308 => {
            let Some(t406) = c.ld64(c.r64(13).wrapping_add(0xd28)) else { return RegfileExit::fault(0x70008b5c0b); };
            c.w64(8, t406);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b5c12); }
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008b5c16); };
            c.w64(2, t407);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008b5c1d); }
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b5c20); };
            c.w64(8, t408);
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b5c24); }
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b5c28); };
            c.w64(9, t409);
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008b5c2c); }
            let Some(t410) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b5c30); };
            c.w64(9, t410);
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b5c37); }
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008b5c3b); };
            c.w64(9, t411);
            if !c.st32(c.r64(0).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b5c42); }
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5c46); };
            c.w64(3, t412);
            let Some(t413) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b5c4a); };
            c.w32(9, t413);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x70008b5c4e); }
            c.logic(c.r8(9) & 0x8u64, 8);
            if c.cc_ne() { blk = 311; continue; }
            blk = 309; continue;
        }
        309 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008b5c58); }
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5c59); };
            c.w64(6, t414);
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0x25b8)) else { return RegfileExit::fault(0x70008b5c5d); };
            c.w64(3, t415);
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x70008b5c71, 0x70008b5c6c);
        }
        310 => {
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5c71); };
            c.w64(3, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b5c75); };
            c.w64(8, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b5c79); };
            c.w64(0, t418);
            blk = 311; continue;
        }
        311 => {
            c.w64(11, c.r64(0));
            blk = 170; continue;
        }
        312 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(2)) { return RegfileExit::fault(0x70008b5c85); }
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008b5c91); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x70008b5c95); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febb4, 0x70008b5ca1, 0x70008b5c9c);
        }
        313 => {
            c.w64(11, c.r64(0));
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t419) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b5ca6); };
            c.w64(15, t419);
            let Some(t420) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b5cad); };
            c.w64(1, t420);
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b5cb4); };
            c.w64(12, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5cb8); };
            c.w64(3, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b5cbc); };
            c.w64(7, t423);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b5cc0); };
            c.w64(8, t424);
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b5cc4); };
            c.w64(9, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008b5cc8); };
            c.w64(2, t426);
            blk = 169; continue;
        }
        314 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(2)) { return RegfileExit::fault(0x70008b5cd4); }
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(8)) { return RegfileExit::fault(0x70008b5ce0); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febb4, 0x70008b5cec, 0x70008b5ce7);
        }
        315 => {
            c.w64(11, c.r64(0));
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t427) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b5cf1); };
            c.w64(9, t427);
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b5cf8); };
            c.w64(1, t428);
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b5cff); };
            c.w64(12, t429);
            let Some(t430) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b5d03); };
            c.w64(0, t430);
            let Some(t431) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5d07); };
            c.w64(3, t431);
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b5d0b); };
            c.w64(7, t432);
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b5d0f); };
            c.w64(8, t433);
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008b5d13); };
            c.w64(2, t434);
            blk = 200; continue;
        }
        316 => {
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5d1f); };
            if !c.push64(t435) { return RegfileExit::fault(0x70008b5d1f); }
            let Some(t436) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b5d22); };
            if !c.push64(t436) { return RegfileExit::fault(0x70008b5d22); }
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5d25); };
            c.w64(6, t437);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x70008b5d33, 0x70008b5d2e);
        }
        317 => {
            blk = 207; continue;
        }
        318 => {
            let Some(t438) = c.ld64(c.r64(13).wrapping_add(0xc40)) else { return RegfileExit::fault(0x70008b5d38); };
            c.w64(3, t438);
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b5d3f); };
            c.w64(1, t439);
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5d43); };
            c.w64(6, t440);
            return c.call(0x7000871cb4, 0x70008b5d4c, 0x70008b5d47);
        }
        319 => {
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b5d4c); };
            c.w64(8, t441);
            blk = 215; continue;
        }
        320 => {
            let Some(t442) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b5d58); };
            c.w64(2, t442);
            let Some(t443) = c.ld32(c.r64(2).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008b5d5f); };
            c.w32(2, t443);
            let r444 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r444);
            return c.call(0x7000824234, 0x70008b5d6d, 0x70008b5d68);
        }
        321 => {
            return RegfileExit::fault(0x70008b5d6d);
        }
        322 => {
            return RegfileExit::fault(0x70008b5d6e);
        }
        323 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 325; continue; }
            blk = 324; continue;
        }
        324 => {
            return RegfileExit::fault(0x70008b5d75);
        }
        325 => {
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b5d76); };
            c.w64(6, t445);
            let Some(t446) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5d7a); };
            c.w32(8, t446);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t447) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b5d81); };
            c.w32(9, t447);
            let r448 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r448);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008b5d8b); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 327; continue; }
            blk = 326; continue;
        }
        326 => {
            return RegfileExit::fault(0x70008b5d95);
        }
        327 => {
            let Some(t449) = c.ld32(c.r64(6).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008b5d96); };
            c.w32(8, t449);
            let Some(t450) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5d9d); };
            c.w32(11, t450);
            let Some(t451) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b5da1); };
            c.w64(12, t451);
            let Some(t452) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b5da8); };
            c.w64(15, t452);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008b5db0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008b5db4); }
            let Some(t453) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b5db8); };
            c.w64(3, t453);
            let Some(t454) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b5dbf); };
            c.cmp(t454, c.r64(0), 64);
            if c.cc_a() { blk = 330; continue; }
            blk = 328; continue;
        }
        328 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x70008b5dce, 0x70008b5dc9);
        }
        329 => {
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b5dce); };
            c.w64(8, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b5dd2); };
            c.w64(6, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5dd6); };
            c.w64(11, t457);
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b5dda); };
            c.w64(9, t458);
            blk = 331; continue;
        }
        330 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008b5de0); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 331; continue;
        }
        331 => {
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b5deb); }
            let Some(t459) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b5def); };
            c.w64(15, t459);
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008b5df6); }
            c.w64(8, c.r64(9));
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b5dfd); }
            c.w64(8, c.r64(11));
            if !c.st32(c.r64(0).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b5e04); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b5e08); }
            let Some(t460) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b5e09); };
            c.w64(8, t460);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b5e10); }
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b5e12); };
            c.w64(7, t461);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b5e20, 0x70008b5e1b);
        }
        332 => {
            blk = 242; continue;
        }
        333 => {
            let Some(t462) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b5e25); };
            c.w64(2, t462);
            let Some(t463) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008b5e2c); };
            c.w32(2, t463);
            let r464 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r464);
            return c.call(0x7000824234, 0x70008b5e3a, 0x70008b5e35);
        }
        334 => {
            return RegfileExit::fault(0x70008b5e3a);
        }
        335 => {
            return RegfileExit::fault(0x70008b5e3b);
        }
        336 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b5e3c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008b5e40); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x70008b5e44); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008b5e48); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008b5e54, 0x70008b5e4f);
        }
        337 => {
            c.w64(15, c.r64(0));
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b5e57); };
            c.w64(0, t465);
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5e5b); };
            c.w64(9, t466);
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b5e5f); };
            c.w64(11, t467);
            c.w32(3, 0x8u64);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5e68); };
            c.w64(12, t468);
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b5e6c); };
            c.w64(8, t469);
            blk = 231; continue;
        }
        338 => {
            let Some(t470) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008b5e75); };
            c.w64(3, t470);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b5e85, 0x70008b5e80);
        }
        339 => {
            return RegfileExit::fault(0x70008b5e85);
        }
        340 => {
            blk = 353; continue;
        }
        341 => {
            return RegfileExit::fault(0x70008b5e8b);
        }
        342 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b5e8c); }
            let Some(t471) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x70008b5e90); };
            c.w64(3, t471);
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b5e97); };
            c.w64(0, t472);
            let Some(t473) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5e9b); };
            c.w64(6, t473);
            return c.call(0x7000870914, 0x70008b5ea4, 0x70008b5e9f);
        }
        343 => {
            let Some(t474) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b5ea4); };
            c.cmp(t474, c.r32(0), 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 344; continue;
        }
        344 => {
            let Some(t475) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b5ead); };
            c.cmp(t475, c.r32(0), 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t476) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b5eb6); };
            if !c.push64(t476) { return RegfileExit::fault(0x70008b5eb6); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t477) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5ebe); };
            c.w64(6, t477);
            return c.call(0x70007f2274, 0x70008b5ec7, 0x70008b5ec2);
        }
        346 => {
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b5ec7); };
            c.w64(0, t478);
            blk = 353; continue;
        }
        347 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b5ecd); }
            let Some(t479) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008b5ece); };
            c.w64(3, t479);
            c.w32(0, 0x1u64);
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5eda); };
            c.w64(6, t480);
            return c.call(0x700086ed74, 0x70008b5ee3, 0x70008b5ede);
        }
        348 => {
            return RegfileExit::fault(0x70008b5ee3);
        }
        349 => {
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x70008b5ee4); };
            c.w64(3, t481);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5eed); };
            c.w64(6, t482);
            return c.call(0x700086ed74, 0x70008b5ef6, 0x70008b5ef1);
        }
        350 => {
            return RegfileExit::fault(0x70008b5ef6);
        }
        351 => {
            let Some(t483) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b5ef7); };
            if !c.push64(t483) { return RegfileExit::fault(0x70008b5ef7); }
            let Some(t484) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x70008b5efa); };
            c.w64(3, t484);
            c.w32(0, 0x1u64);
            let Some(t485) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5f06); };
            c.w64(6, t485);
            return c.call(0x700086ed74, 0x70008b5f0f, 0x70008b5f0a);
        }
        352 => {
            return RegfileExit::fault(0x70008b5f0f);
        }
        353 => {
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b5f10); };
            c.w64(8, t486);
            let Some(t487) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b5f14); };
            c.w32(7, t487);
            let r488 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r488);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b5f1b); }
            let Some(t489) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b5f1c); };
            c.w64(9, t489);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b5f23); }
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b5f25); };
            c.w64(6, t490);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b5f33, 0x70008b5f2e);
        }
        354 => {
            let Some(t491) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b5f33); };
            c.w64(8, t491);
            let Some(t492) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5f37); };
            c.w32(0, t492);
            let r493 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r493);
            blk = 243; continue;
        }
        355 => {
            c.w32(15, c.r32(15));
            let r494 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r494);
            c.w64(8, 0xfffffffffffc0000u64);
            let r495 = c.r64(8) & c.r64(15);
            c.w64(8, r495);
            c.logic(r495, 64);
            let Some(t496) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b5f53); };
            c.logic(t496 & 0x2u64, 8);
            if c.cc_e() { blk = 242; continue; }
            blk = 356; continue;
        }
        356 => {
            c.w64(9, c.r64(0).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b5f62); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b5f63); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008b5f70, 0x70008b5f6b);
        }
        357 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b5f70); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b5f72); }
            blk = 242; continue;
        }
        358 => {
            c.w32(11, c.r32(11));
            let r497 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r497);
            c.w64(8, 0xfffffffffffc0000u64);
            let r498 = c.r64(8) & c.r64(11);
            c.w64(8, r498);
            c.logic(r498, 64);
            let Some(t499) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b5f88); };
            c.logic(t499 & 0x2u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(15, c.r64(3).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b5f97); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b5f98); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b5fa5, 0x70008b5fa0);
        }
        360 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b5fa5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b5fa7); }
            blk = 202; continue;
        }
        361 => {
            return RegfileExit::jump(0x70008b5fb0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b5fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b5fb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b5fb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b5fb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b5fba); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b5fbf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b5fc9); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b5fcf); };
            c.cmp(t4, 0x90u64, 16);
            if c.cc_e() { blk = 33; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b5fdb); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008b5fe1); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r7 = c.r64(8) & c.r64(7);
            c.w64(8, r7);
            c.logic(r7, 64);
            let Some(t8) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b5ff3); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b5ffe); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b600b); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b6012); };
            c.w32(9, t13);
            c.cmp(c.r16(9), 0xb0u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(9), 0x7bu64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b602b); };
            c.w32(9, t14);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b603f); };
            c.w32(8, t15);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            blk = 12; continue;
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b604e); };
            c.w32(8, t16);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if let Some(sr) = c.shr(c.r64(8), 0xau64, 64) { c.w64(8, sr); }
            let r17 = c.r64(8) & 0xfffffu64;
            c.w64(8, r17);
            c.logic(r17, 64);
            c.w64(9, c.r64(8));
            blk = 12; continue;
        }
        11 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 36; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b607b); };
            c.w32(8, t18);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008b608b);
        }
        15 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b608c); };
            c.w32(11, t19);
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008b6096);
        }
        17 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b6097); };
            c.w32(8, t20);
            let r21 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r21);
            let r22 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r22);
            c.w64(12, c.r64(9).wrapping_add(0x1));
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b60a6); };
            c.w32(15, t23);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), c.r64(12), 64);
            if c.cc_l() { blk = 40; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t24) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b60bc); };
            c.w32(12, t24);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_b() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008b60cb);
        }
        20 => {
            let r25 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r25);
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t26) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b60d8); };
            c.w64(12, t26);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x70008b60dc); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r27 = c.r64(15) & c.r64(8);
            c.w64(15, r27);
            c.logic(r27, 64);
            let Some(t28) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b60f0); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 77; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 69; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t29) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6105); };
            c.w32(9, t29);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 25; continue; }
            blk = 24; continue;
        }
        24 => {
            return RegfileExit::fault(0x70008b6115);
        }
        25 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(6).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008b6119); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 33; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x70008b612d);
        }
        28 => {
            let Some(t30) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b612e); };
            c.w32(9, t30);
            let Some(t31) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b6132); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b613c); };
            c.w32(7, t33);
            let r34 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r34);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008b614c);
        }
        30 => {
            let Some(t35) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008b614d); };
            c.w32(1, t35);
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6153); };
            c.w32(9, t36);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b6157); };
            c.w64(11, t37);
            let Some(t38) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008b615e); };
            c.w64(12, t38);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b6166); };
            c.w64(0, t39);
            let Some(t40) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008b616d); };
            c.cmp(t40, c.r64(15), 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 31; continue;
        }
        31 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008b6176); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 32; continue;
        }
        32 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b617e); }
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b6182); };
            c.w64(1, t41);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008b6189); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b618d); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008b6191); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b6195); }
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b6197); };
            c.w64(1, t42);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b619e); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b61a9, 0x70008b61a4);
        }
        33 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b61a9); };
            c.w64(0, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b61b0); };
            c.w64(1, t44);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b61b7); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return c.ret(0x10, 0x70008b61be);
        }
        35 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b61c1); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b61c8); }
            return c.ret(0x0, 0x70008b61ca);
        }
        36 => {
            return RegfileExit::fault(0x70008b61cb);
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008b61cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b61d0); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b61d6); };
            c.w64(3, t45);
            return c.call(0x700086ed74, 0x70008b61e2, 0x70008b61dd);
        }
        38 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b61e2); };
            c.w64(6, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b61e6); };
            c.w64(7, t47);
            blk = 1; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008b61ef);
        }
        40 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008b61f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b61f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008b61f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008b61fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008b6200); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008b6204); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ae() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b620e); };
            c.w64(2, t48);
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008b6215); };
            c.w32(2, t49);
            let r50 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r50);
            return c.call(0x7000824234, 0x70008b6223, 0x70008b621e);
        }
        42 => {
            return RegfileExit::fault(0x70008b6223);
        }
        43 => {
            return RegfileExit::fault(0x70008b6224);
        }
        44 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 63; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 61; continue; }
            blk = 46; continue;
        }
        46 => {
            let r51 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r51);
            c.w64(0, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(3, c.r32(3));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x70008b624a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008b624e); }
            let r52 = c.imul2(c.r32(3), 0x4u64, 32);
            c.w32(3, r52);
            if c.cc_o() { blk = 60; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r53 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r53);
            if c.cc_o() { blk = 59; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b626c); };
            c.w64(7, t54);
            let Some(t55) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b6273); };
            c.w64(7, t55);
            c.w64(1, c.r64(7).wrapping_add(c.r64(3)));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008b627a); }
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b627e); };
            c.w64(7, t56);
            let Some(t57) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b6285); };
            c.cmp(t57, c.r64(1), 64);
            if c.cc_be() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_b() { blk = 52; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008b629b, 0x70008b6296);
        }
        51 => {
            c.w64(1, c.r64(0));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b629e); };
            c.w64(6, t58);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b62a2); };
            c.w64(9, t59);
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b62a6); };
            c.w64(11, t60);
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b62aa); };
            c.w64(8, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b62ae); };
            c.w64(15, t62);
            c.w32(2, 0x8u64);
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b62b7); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b62bb); };
            c.w64(12, t64);
            blk = 53; continue;
        }
        52 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b62c1); };
            c.w64(3, t65);
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008b62c8); }
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b62cb); };
            c.w64(1, t66);
            let r67 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r67);
            blk = 53; continue;
        }
        53 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008b62d3); };
            c.w64(3, t68);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008b62da); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008b62dd); }
            c.w64(12, c.r64(0));
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(2));
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_e() { blk = 64; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(3, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(3), c.r64(0), 64);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b6301); };
            c.w32(3, t69);
            let r70 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r70);
            blk = 58; continue;
        }
        57 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b630c); };
            c.w64(3, t71);
            blk = 58; continue;
        }
        58 => {
            if !c.st32(c.r64(1).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008b6313); }
            let r72 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r72);
            blk = 54; continue;
        }
        59 => {
            return RegfileExit::fault(0x70008b631d);
        }
        60 => {
            return RegfileExit::fault(0x70008b631e);
        }
        61 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008b631f); };
            c.w64(3, t73);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b632f, 0x70008b632a);
        }
        62 => {
            return RegfileExit::fault(0x70008b632f);
        }
        63 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b6330); };
            c.w64(1, t74);
            c.w64(8, c.r64(1));
            blk = 64; continue;
        }
        64 => {
            let Some(t75) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b633a); };
            c.w32(8, t75);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_a() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            return RegfileExit::fault(0x70008b634a);
        }
        66 => {
            if !c.st32(c.r64(6).wrapping_add(0x17), c.r32(1)) { return RegfileExit::fault(0x70008b634b); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r76 = c.r64(8) & c.r64(6);
            c.w64(8, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b635d); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 74; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(8, c.r64(1));
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b6367); };
            c.w64(1, t78);
            blk = 18; continue;
        }
        69 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6370); };
            c.w64(2, t79);
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(0x23b)) else { return RegfileExit::fault(0x70008b6377); };
            c.w32(2, t80);
            let r81 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r81);
            return c.call(0x7000824234, 0x70008b6385, 0x70008b6380);
        }
        70 => {
            return RegfileExit::fault(0x70008b6385);
        }
        71 => {
            return RegfileExit::fault(0x70008b6386);
        }
        72 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b638c); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008b6390); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b6394); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008b6398); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008b639c); }
            return c.call(0x70007febb4, 0x70008b63a5, 0x70008b63a0);
        }
        73 => {
            c.w64(11, c.r64(0));
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b63a8); };
            c.w64(7, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b63ac); };
            c.w64(6, t83);
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b63b0); };
            c.w64(9, t84);
            let Some(t85) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b63b4); };
            c.w64(8, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b63b8); };
            c.w64(1, t86);
            blk = 32; continue;
        }
        74 => {
            c.w32(1, c.r32(1));
            let r87 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r87);
            c.w64(8, 0xfffffffffffc0000u64);
            let r88 = c.r64(8) & c.r64(1);
            c.w64(8, r88);
            c.logic(r88, 64);
            let Some(t89) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b63d0); };
            c.logic(t89 & 0x2u64, 8);
            if c.cc_e() { blk = 68; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(12, c.r64(6).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b63db); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b63dc); }
            c.w64(7, c.r64(6));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008b63e9, 0x70008b63e4);
        }
        76 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b63e9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b63eb); }
            blk = 68; continue;
        }
        77 => {
            c.w32(12, c.r32(12));
            let r90 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r90);
            c.w64(15, 0xfffffffffffc0000u64);
            let r91 = c.r64(15) & c.r64(12);
            c.w64(15, r91);
            c.logic(r91, 64);
            let Some(t92) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6401); };
            c.logic(t92 & 0x2u64, 8);
            if c.cc_e() { blk = 22; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(0, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b6410); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b6411); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x70008b641e, 0x70008b6419);
        }
        79 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b641e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b6420); }
            blk = 22; continue;
        }
        80 => {
            c.w32(1, c.r32(1));
            let r93 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r93);
            c.w64(8, 0xfffffffffffc0000u64);
            let r94 = c.r64(8) & c.r64(1);
            c.w64(8, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6435); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(9, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b6444); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008b644e, 0x70008b6449);
        }
        82 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b644e); }
            blk = 4; continue;
        }
        83 => {
            return RegfileExit::jump(0x70008b6458);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b6474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b6474); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b6478); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b6479); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b647a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008b647f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b6483); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b648d); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b6494); };
            c.cmp(t4, 0x90u64, 16);
            if c.cc_e() { blk = 45; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b64a1); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b64a8); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r7 = c.r64(9) & c.r64(7);
            c.w64(9, r7);
            c.logic(r7, 64);
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b64bc); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 94; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b64c7); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008b64ce); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b64d8); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            let Some(t13) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b64df); };
            c.w32(11, t13);
            c.cmp(c.r16(11), 0xb0u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(11), 0x7bu64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b64f8); };
            c.w32(11, t14);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 51; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b650c); };
            c.w32(9, t15);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 12; continue;
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b651b); };
            c.w32(9, t16);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r17 = c.r64(9) & 0xfffffu64;
            c.w64(9, r17);
            c.logic(r17, 64);
            c.w64(11, c.r64(9));
            blk = 12; continue;
        }
        11 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6548); };
            c.w32(9, t18);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008b6558);
        }
        15 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b6559); };
            c.w32(12, t19);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008b655d); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008b6567);
        }
        17 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b6568); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            let r22 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r22);
            c.w64(15, c.r64(11).wrapping_add(0x1));
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6577); };
            c.w32(0, t23);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008b6584); }
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_l() { blk = 52; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6591); };
            c.w32(15, t24);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008b659b); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008b65a4);
        }
        20 => {
            c.w64(15, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b65ad); };
            c.w32(0, t25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008b65b2); }
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b65b6); };
            c.cmp(t26, c.r32(0), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b65c3); };
            c.w32(0, t27);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x8cu64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70008b65d5);
        }
        23 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008b65d6); };
            c.w32(0, t28);
            let r29 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r29);
            let Some(t30) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b65e0); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b65e6); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b65ed); };
            c.w64(2, t33);
            let Some(t34) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b65f0); };
            c.w32(1, t34);
            if let Some(sr) = c.shl(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            c.w64(7, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b65fc); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008b6603); };
            c.cmp(t36, c.r64(7), 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008b660c); }
            c.w64(8, c.r64(2).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            c.w64(3, c.r64(1).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008b6617); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b661b); };
            c.w64(0, t37);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008b6622); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008b6626); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008b662a); }
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b6634); };
            c.w64(0, t38);
            blk = 27; continue;
        }
        27 => {
            let r39 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008b663f); }
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_ne() { blk = 27; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6649); };
            c.w64(3, t40);
            let Some(t41) = c.ld32(c.r64(3).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008b6650); };
            c.w32(3, t41);
            let r42 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r42);
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6659); };
            c.w64(1, t43);
            let Some(t44) = c.ld32(c.r64(1).wrapping_add(0x243)) else { return RegfileExit::fault(0x70008b6660); };
            c.w32(1, t44);
            let r45 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r45);
            c.w64(0, c.r64(8));
            return c.call(0x7000899bb4, 0x70008b6671, 0x70008b666c);
        }
        29 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008b6671); };
            c.w64(3, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b6678); };
            c.w64(0, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b667c); };
            c.w64(1, t48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6680); };
            c.w64(6, t49);
            return c.call(0x7000899bb4, 0x70008b6689, 0x70008b6684);
        }
        30 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b6689); };
            c.w64(9, t50);
            let Some(t51) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b668d); };
            c.w32(8, t51);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b6697); };
            c.w64(11, t52);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008b66a0);
        }
        32 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b66a1); };
            c.w64(8, t53);
            let r54 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r54);
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b66a9); };
            c.w64(11, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b66ad); };
            c.w64(0, t56);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008b66b1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r57 = c.r64(12) & c.r64(9);
            c.w64(12, r57);
            c.logic(r57, 64);
            let Some(t58) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b66c3); };
            c.logic(t58 & 0x4u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 83; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b66d9); };
            c.w64(6, t59);
            let Some(t60) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b66dd); };
            c.w32(11, t60);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008b66ed);
        }
        37 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(6).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008b66f1); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008b6705);
        }
        40 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b6706); };
            c.w64(8, t61);
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b670a); };
            c.w32(11, t62);
            let Some(t63) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b670e); };
            c.w32(12, t63);
            let r64 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r64);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t65) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b6718); };
            c.w32(7, t65);
            let r66 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r66);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x70008b6729);
        }
        42 => {
            let Some(t67) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008b672a); };
            c.w32(8, t67);
            let Some(t68) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6731); };
            c.w32(11, t68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b6735); };
            c.w64(12, t69);
            let Some(t70) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b673c); };
            c.w64(15, t70);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b6744); };
            c.w64(3, t71);
            let Some(t72) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b674b); };
            c.cmp(t72, c.r64(0), 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008b6754); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 44; continue;
        }
        44 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b675c); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b6761); };
            c.w64(8, t73);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b6768); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b676d); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b6772); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b6777); }
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b6779); };
            c.w64(8, t74);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b6780); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b678c, 0x70008b6787);
        }
        45 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b678c); };
            c.w64(0, t75);
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b6793); };
            c.w64(1, t76);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b679a); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            return c.ret(0x10, 0x70008b67a1);
        }
        47 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b67a4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b67ab); }
            return c.ret(0x0, 0x70008b67ad);
        }
        48 => {
            return RegfileExit::fault(0x70008b67ae);
        }
        49 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b67af); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b67b5); };
            c.w64(3, t77);
            return c.call(0x700086ed74, 0x70008b67c1, 0x70008b67bc);
        }
        50 => {
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b67c1); };
            c.w64(6, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b67c5); };
            c.w64(7, t79);
            blk = 1; continue;
        }
        51 => {
            return RegfileExit::fault(0x70008b67ce);
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008b67cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008b67d3); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_ae() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b67dd); };
            c.w64(2, t80);
            let Some(t81) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008b67e4); };
            c.w32(2, t81);
            let r82 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r82);
            return c.call(0x7000824234, 0x70008b67f2, 0x70008b67ed);
        }
        54 => {
            return RegfileExit::fault(0x70008b67f2);
        }
        55 => {
            return RegfileExit::fault(0x70008b67f3);
        }
        56 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 75; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 73; continue; }
            blk = 58; continue;
        }
        58 => {
            let r83 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r83);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008b6819); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008b681d); }
            let r84 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r84);
            if c.cc_o() { blk = 72; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, 0x8u64);
            let r85 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r85);
            if c.cc_o() { blk = 71; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b683b); };
            c.w64(7, t86);
            let Some(t87) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b6842); };
            c.w64(7, t87);
            c.w64(8, c.r64(7).wrapping_add(c.r64(2)));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008b6849); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b684d); };
            c.w64(7, t88);
            let Some(t89) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b6854); };
            c.cmp(t89, c.r64(8), 64);
            if c.cc_be() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_b() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            return c.call(0x70007febb4, 0x70008b6867, 0x70008b6862);
        }
        63 => {
            c.w64(8, c.r64(0));
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b686a); };
            c.w64(6, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b686e); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b6872); };
            c.w64(11, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b6876); };
            c.w64(9, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b687a); };
            c.w64(0, t94);
            c.w32(1, 0x8u64);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b6883); };
            c.w64(3, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b6887); };
            c.w64(15, t96);
            blk = 65; continue;
        }
        64 => {
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b688d); };
            c.w64(2, t97);
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008b6894); }
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b6897); };
            c.w64(8, t98);
            let r99 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r99);
            blk = 65; continue;
        }
        65 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008b689f); };
            c.w64(2, t100);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b68a6); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008b68aa); }
            c.w64(15, c.r64(3));
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(1));
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_e() { blk = 76; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(2, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b68ce); };
            c.w32(2, t101);
            let r102 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r102);
            blk = 70; continue;
        }
        69 => {
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b68d9); };
            c.w64(2, t103);
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b68e0); }
            let r104 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r104);
            blk = 66; continue;
        }
        71 => {
            return RegfileExit::fault(0x70008b68eb);
        }
        72 => {
            return RegfileExit::fault(0x70008b68ec);
        }
        73 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008b68ed); };
            c.w64(3, t105);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b68fd, 0x70008b68f8);
        }
        74 => {
            return RegfileExit::fault(0x70008b68fd);
        }
        75 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b68fe); };
            c.w64(8, t106);
            c.w64(9, c.r64(8));
            blk = 76; continue;
        }
        76 => {
            let Some(t107) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6908); };
            c.w32(9, t107);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::fault(0x70008b6918);
        }
        78 => {
            if !c.st32(c.r64(6).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008b6919); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r108 = c.r64(9) & c.r64(6);
            c.w64(9, r108);
            c.logic(r108, 64);
            let Some(t109) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b692d); };
            c.logic(t109 & 0x4u64, 8);
            if c.cc_ne() { blk = 88; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(9, c.r64(8));
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b693b); };
            c.w64(8, t110);
            blk = 18; continue;
        }
        81 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008b6944); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008b6948); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008b6954, 0x70008b694f);
        }
        82 => {
            c.w64(8, c.r64(0));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6957); };
            c.w64(6, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b695b); };
            c.w64(9, t112);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b695f); };
            c.w64(15, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b6963); };
            c.w64(12, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b6967); };
            c.w64(11, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b696b); };
            c.w64(0, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b696f); };
            c.w64(1, t117);
            blk = 25; continue;
        }
        83 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6978); };
            c.w64(2, t118);
            let Some(t119) = c.ld32(c.r64(2).wrapping_add(0x23b)) else { return RegfileExit::fault(0x70008b697f); };
            c.w32(2, t119);
            let r120 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r120);
            return c.call(0x7000824234, 0x70008b698d, 0x70008b6988);
        }
        84 => {
            return RegfileExit::fault(0x70008b698d);
        }
        85 => {
            return RegfileExit::fault(0x70008b698e);
        }
        86 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b6994); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008b6998); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008b699c); }
            return c.call(0x70007febb4, 0x70008b69a5, 0x70008b69a0);
        }
        87 => {
            c.w64(12, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b69a8); };
            c.w64(7, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b69ac); };
            c.w64(6, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b69b0); };
            c.w64(11, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b69b4); };
            c.w64(9, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b69b8); };
            c.w64(8, t125);
            blk = 44; continue;
        }
        88 => {
            c.w32(8, c.r32(8));
            let r126 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r126);
            c.w64(9, 0xfffffffffffc0000u64);
            let r127 = c.r64(9) & c.r64(8);
            c.w64(9, r127);
            c.logic(r127, 64);
            let Some(t128) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b69d1); };
            c.logic(t128 & 0x2u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(15, c.r64(6).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b69e0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b69e1); }
            c.w64(7, c.r64(6));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b69ee, 0x70008b69e9);
        }
        90 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b69ee); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b69f0); }
            blk = 80; continue;
        }
        91 => {
            c.w32(0, c.r32(0));
            let r129 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r129);
            c.w64(12, 0xfffffffffffc0000u64);
            let r130 = c.r64(12) & c.r64(0);
            c.w64(12, r130);
            c.logic(r130, 64);
            let Some(t131) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6a05); };
            c.logic(t131 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b6a15); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b6a16); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b6a23, 0x70008b6a1e);
        }
        93 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b6a23); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b6a25); }
            blk = 34; continue;
        }
        94 => {
            c.w32(8, c.r32(8));
            let r132 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r132);
            c.w64(9, 0xfffffffffffc0000u64);
            let r133 = c.r64(9) & c.r64(8);
            c.w64(9, r133);
            c.logic(r133, 64);
            let Some(t134) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6a3b); };
            c.logic(t134 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(11, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b6a4a); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008b6a54, 0x70008b6a4f);
        }
        96 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b6a54); }
            blk = 4; continue;
        }
        97 => {
            return RegfileExit::jump(0x70008b6a5c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b6a74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b6a74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b6a78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b6a79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b6a7a); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008b6a7f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b6a83); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b6a8d); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b6a94); };
            c.cmp(t4, 0x90u64, 16);
            if c.cc_e() { blk = 45; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b6aa1); };
            c.w32(8, t5);
            let r6 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r6);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b6aa8); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r7 = c.r64(9) & c.r64(7);
            c.w64(9, r7);
            c.logic(r7, 64);
            let Some(t8) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6abc); };
            c.logic(t8 & 0x4u64, 8);
            if c.cc_ne() { blk = 94; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6ac7); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008b6ace); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b6ad8); };
            c.w32(11, t11);
            let r12 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r12);
            let Some(t13) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b6adf); };
            c.w32(11, t13);
            c.cmp(c.r16(11), 0xb0u64, 16);
            if c.cc_e() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r16(11), 0x7bu64, 16);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        8 => {
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6af8); };
            c.w32(11, t14);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x4u64, 64);
            if c.cc_be() { blk = 51; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b6b0c); };
            c.w32(9, t15);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 12; continue;
        }
        10 => {
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6b1b); };
            c.w32(9, t16);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r17 = c.r64(9) & 0xfffffu64;
            c.w64(9, r17);
            c.logic(r17, 64);
            c.w64(11, c.r64(9));
            blk = 12; continue;
        }
        11 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6b48); };
            c.w32(9, t18);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008b6b58);
        }
        15 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b6b59); };
            c.w32(12, t19);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008b6b5d); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008b6b67);
        }
        17 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b6b68); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            let r22 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r22);
            c.w64(15, c.r64(11).wrapping_add(0x1));
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6b77); };
            c.w32(0, t23);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008b6b84); }
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_l() { blk = 52; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6b91); };
            c.w32(15, t24);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008b6b9b); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008b6ba4);
        }
        20 => {
            c.w64(15, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t25) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b6bad); };
            c.w32(0, t25);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x70008b6bb2); }
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b6bb6); };
            c.cmp(t26, c.r32(0), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6bc3); };
            c.w32(0, t27);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x8cu64, 64);
            if c.cc_a() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            return RegfileExit::fault(0x70008b6bd5);
        }
        23 => {
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008b6bd6); };
            c.w32(0, t28);
            let r29 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r29);
            let Some(t30) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b6be0); };
            c.w32(0, t30);
            let r31 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r31);
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b6be6); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b6bed); };
            c.w64(2, t33);
            let Some(t34) = c.ld8(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6bf0); };
            c.w32(1, t34);
            if let Some(sr) = c.shl(c.r64(1), 0x2u64, 64) { c.w64(1, sr); }
            c.w64(7, c.r64(1).wrapping_add(c.r64(2)));
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b6bfc); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008b6c03); };
            c.cmp(t36, c.r64(7), 64);
            if c.cc_be() { blk = 81; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008b6c0c); }
            c.w64(8, c.r64(2).wrapping_add(0x1));
            blk = 25; continue;
        }
        25 => {
            c.w64(3, c.r64(1).wrapping_sub(0x1));
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008b6c17); }
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b6c1b); };
            c.w64(0, t37);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008b6c22); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008b6c26); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008b6c2a); }
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b6c34); };
            c.w64(0, t38);
            blk = 27; continue;
        }
        27 => {
            let r39 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r39);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008b6c3f); }
            c.cmp(c.r64(3), 0xbu64, 64);
            if c.cc_ne() { blk = 27; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6c49); };
            c.w64(3, t40);
            let Some(t41) = c.ld32(c.r64(3).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008b6c50); };
            c.w32(3, t41);
            let r42 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r42);
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6c59); };
            c.w64(1, t43);
            let Some(t44) = c.ld32(c.r64(1).wrapping_add(0x247)) else { return RegfileExit::fault(0x70008b6c60); };
            c.w32(1, t44);
            let r45 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r45);
            c.w64(0, c.r64(8));
            return c.call(0x7000899bb4, 0x70008b6c71, 0x70008b6c6c);
        }
        29 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6c71); };
            c.w64(3, t46);
            let Some(t47) = c.ld32(c.r64(3).wrapping_add(0x24b)) else { return RegfileExit::fault(0x70008b6c78); };
            c.w32(3, t47);
            let r48 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r48);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b6c81); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b6c85); };
            c.w64(1, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6c89); };
            c.w64(6, t51);
            return c.call(0x7000899bb4, 0x70008b6c92, 0x70008b6c8d);
        }
        30 => {
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b6c92); };
            c.w64(9, t52);
            let Some(t53) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6c96); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b6ca0); };
            c.w64(11, t54);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x70008b6ca9);
        }
        32 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b6caa); };
            c.w64(8, t55);
            let r56 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b6cb2); };
            c.w64(11, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b6cb6); };
            c.w64(0, t58);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11)), c.r32(0)) { return RegfileExit::fault(0x70008b6cba); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r59 = c.r64(12) & c.r64(9);
            c.w64(12, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6ccc); };
            c.logic(t60 & 0x4u64, 8);
            if c.cc_ne() { blk = 91; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 83; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6ce2); };
            c.w64(6, t61);
            let Some(t62) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6ce6); };
            c.w32(11, t62);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x2u64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008b6cf6);
        }
        37 => {
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(6).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x70008b6cfa); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_a() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            return RegfileExit::fault(0x70008b6d0e);
        }
        40 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b6d0f); };
            c.w64(8, t63);
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6d13); };
            c.w32(11, t64);
            let Some(t65) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b6d17); };
            c.w32(12, t65);
            let r66 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r66);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t67) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b6d21); };
            c.w32(7, t67);
            let r68 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r68);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x6bu64, 64);
            if c.cc_a() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            return RegfileExit::fault(0x70008b6d32);
        }
        42 => {
            let Some(t69) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008b6d33); };
            c.w32(8, t69);
            let Some(t70) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6d3a); };
            c.w32(11, t70);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b6d3e); };
            c.w64(12, t71);
            let Some(t72) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b6d45); };
            c.w64(15, t72);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b6d4d); };
            c.w64(3, t73);
            let Some(t74) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b6d54); };
            c.cmp(t74, c.r64(0), 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008b6d5d); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 44; continue;
        }
        44 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b6d65); }
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b6d6a); };
            c.w64(8, t75);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b6d71); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b6d76); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b6d7b); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b6d80); }
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b6d82); };
            c.w64(8, t76);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b6d89); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b6d95, 0x70008b6d90);
        }
        45 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b6d95); };
            c.w64(0, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b6d9c); };
            c.w64(1, t78);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b6da3); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            return c.ret(0x10, 0x70008b6daa);
        }
        47 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b6dad); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b6db4); }
            return c.ret(0x0, 0x70008b6db6);
        }
        48 => {
            return RegfileExit::fault(0x70008b6db7);
        }
        49 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b6db8); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b6dbe); };
            c.w64(3, t79);
            return c.call(0x700086ed74, 0x70008b6dca, 0x70008b6dc5);
        }
        50 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6dca); };
            c.w64(6, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b6dce); };
            c.w64(7, t81);
            blk = 1; continue;
        }
        51 => {
            return RegfileExit::fault(0x70008b6dd7);
        }
        52 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008b6dd8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008b6ddc); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_ae() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6de6); };
            c.w64(2, t82);
            let Some(t83) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008b6ded); };
            c.w32(2, t83);
            let r84 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r84);
            return c.call(0x7000824234, 0x70008b6dfb, 0x70008b6df6);
        }
        54 => {
            return RegfileExit::fault(0x70008b6dfb);
        }
        55 => {
            return RegfileExit::fault(0x70008b6dfc);
        }
        56 => {
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 75; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 73; continue; }
            blk = 58; continue;
        }
        58 => {
            let r85 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r85);
            c.w64(3, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(3));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(2, c.r32(2));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008b6e22); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x70008b6e26); }
            let r86 = c.imul2(c.r32(2), 0x4u64, 32);
            c.w32(2, r86);
            if c.cc_o() { blk = 72; continue; }
            blk = 59; continue;
        }
        59 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w32(1, 0x8u64);
            let r87 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r87);
            if c.cc_o() { blk = 71; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b6e44); };
            c.w64(7, t88);
            let Some(t89) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b6e4b); };
            c.w64(7, t89);
            c.w64(8, c.r64(7).wrapping_add(c.r64(2)));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008b6e52); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b6e56); };
            c.w64(7, t90);
            let Some(t91) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008b6e5d); };
            c.cmp(t91, c.r64(8), 64);
            if c.cc_be() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_b() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            return c.call(0x70007febb4, 0x70008b6e70, 0x70008b6e6b);
        }
        63 => {
            c.w64(8, c.r64(0));
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6e73); };
            c.w64(6, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b6e77); };
            c.w64(12, t93);
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b6e7b); };
            c.w64(11, t94);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b6e7f); };
            c.w64(9, t95);
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b6e83); };
            c.w64(0, t96);
            c.w32(1, 0x8u64);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b6e8c); };
            c.w64(3, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b6e90); };
            c.w64(15, t98);
            blk = 65; continue;
        }
        64 => {
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b6e96); };
            c.w64(2, t99);
            if !c.st64(c.r64(2), c.r64(8)) { return RegfileExit::fault(0x70008b6e9d); }
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b6ea0); };
            c.w64(8, t100);
            let r101 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r101);
            blk = 65; continue;
        }
        65 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008b6ea8); };
            c.w64(2, t102);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b6eaf); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x70008b6eb3); }
            c.w64(15, c.r64(3));
            c.w64(15, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(1));
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(15), c.r64(1), 64);
            if c.cc_e() { blk = 76; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(2, c.r64(0).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(3), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t103) = c.ld32(c.r64(9).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b6ed7); };
            c.w32(2, t103);
            let r104 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r104);
            blk = 70; continue;
        }
        69 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b6ee2); };
            c.w64(2, t105);
            blk = 70; continue;
        }
        70 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b6ee9); }
            let r106 = c.add(c.r64(1), 0x4u64, 64);
            c.w64(1, r106);
            blk = 66; continue;
        }
        71 => {
            return RegfileExit::fault(0x70008b6ef4);
        }
        72 => {
            return RegfileExit::fault(0x70008b6ef5);
        }
        73 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008b6ef6); };
            c.w64(3, t107);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b6f06, 0x70008b6f01);
        }
        74 => {
            return RegfileExit::fault(0x70008b6f06);
        }
        75 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b6f07); };
            c.w64(8, t108);
            c.w64(9, c.r64(8));
            blk = 76; continue;
        }
        76 => {
            let Some(t109) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b6f11); };
            c.w32(9, t109);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::fault(0x70008b6f21);
        }
        78 => {
            if !c.st32(c.r64(6).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008b6f22); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r110 = c.r64(9) & c.r64(6);
            c.w64(9, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6f36); };
            c.logic(t111 & 0x4u64, 8);
            if c.cc_ne() { blk = 88; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(9, c.r64(8));
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b6f44); };
            c.w64(8, t112);
            blk = 18; continue;
        }
        81 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008b6f4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008b6f51); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008b6f5d, 0x70008b6f58);
        }
        82 => {
            c.w64(8, c.r64(0));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6f60); };
            c.w64(6, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b6f64); };
            c.w64(9, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b6f68); };
            c.w64(15, t115);
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b6f6c); };
            c.w64(12, t116);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b6f70); };
            c.w64(11, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b6f74); };
            c.w64(0, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b6f78); };
            c.w64(1, t119);
            blk = 25; continue;
        }
        83 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b6f81); };
            c.w64(2, t120);
            let Some(t121) = c.ld32(c.r64(2).wrapping_add(0x23b)) else { return RegfileExit::fault(0x70008b6f88); };
            c.w32(2, t121);
            let r122 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r122);
            return c.call(0x7000824234, 0x70008b6f96, 0x70008b6f91);
        }
        84 => {
            return RegfileExit::fault(0x70008b6f96);
        }
        85 => {
            return RegfileExit::fault(0x70008b6f97);
        }
        86 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b6f9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008b6fa1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008b6fa5); }
            return c.call(0x70007febb4, 0x70008b6fae, 0x70008b6fa9);
        }
        87 => {
            c.w64(12, c.r64(0));
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b6fb1); };
            c.w64(7, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b6fb5); };
            c.w64(6, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b6fb9); };
            c.w64(11, t125);
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b6fbd); };
            c.w64(9, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b6fc1); };
            c.w64(8, t127);
            blk = 44; continue;
        }
        88 => {
            c.w32(8, c.r32(8));
            let r128 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r128);
            c.w64(9, 0xfffffffffffc0000u64);
            let r129 = c.r64(9) & c.r64(8);
            c.w64(9, r129);
            c.logic(r129, 64);
            let Some(t130) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b6fda); };
            c.logic(t130 & 0x2u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(15, c.r64(6).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b6fe9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b6fea); }
            c.w64(7, c.r64(6));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b6ff7, 0x70008b6ff2);
        }
        90 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b6ff7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b6ff9); }
            blk = 80; continue;
        }
        91 => {
            c.w32(0, c.r32(0));
            let r131 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r131);
            c.w64(12, 0xfffffffffffc0000u64);
            let r132 = c.r64(12) & c.r64(0);
            c.w64(12, r132);
            c.logic(r132, 64);
            let Some(t133) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b700e); };
            c.logic(t133 & 0x2u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(11)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b701e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b701f); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b702c, 0x70008b7027);
        }
        93 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b702c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b702e); }
            blk = 34; continue;
        }
        94 => {
            c.w32(8, c.r32(8));
            let r134 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r134);
            c.w64(9, 0xfffffffffffc0000u64);
            let r135 = c.r64(9) & c.r64(8);
            c.w64(9, r135);
            c.logic(r135, 64);
            let Some(t136) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b7044); };
            c.logic(t136 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(11, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7053); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008b705d, 0x70008b7058);
        }
        96 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b705d); }
            blk = 4; continue;
        }
        97 => {
            return RegfileExit::jump(0x70008b7068);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b7074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b7074); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b7078); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b7079); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b707a); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008b707f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b7083); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 42; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b708d); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7093); };
            c.cmp(t4, 0x90u64, 16);
            if c.cc_ne() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b709b); };
            c.w64(0, t5);
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b70a2); };
            c.w64(1, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b70a9); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x10, 0x70008b70b0);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b70b3); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b70ba); }
            return c.ret(0x0, 0x70008b70bc);
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b70bd); };
            c.w32(1, t7);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008b70c0); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r8 = c.r64(8) & c.r64(7);
            c.w64(8, r8);
            c.logic(r8, 64);
            let Some(t9) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b70d2); };
            c.logic(t9 & 0x4u64, 8);
            if c.cc_ne() { blk = 82; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b70dd); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b70e8); };
            c.w32(8, t12);
            let r13 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r13);
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b70ef); };
            c.w32(8, t14);
            c.cmp(c.r16(8), 0xb0u64, 16);
            if c.cc_e() { blk = 14; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r16(8), 0x7bu64, 16);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 16; continue;
        }
        12 => {
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7108); };
            c.w32(8, t15);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_be() { blk = 44; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b711c); };
            c.w32(1, t16);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(8, c.r64(1));
            blk = 16; continue;
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b712a); };
            c.w32(1, t17);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            if let Some(sr) = c.shr(c.r64(1), 0xau64, 64) { c.w64(1, sr); }
            let r18 = c.r64(1) & 0xfffffu64;
            c.w64(1, r18);
            c.logic(r18, 64);
            c.w64(8, c.r64(1));
            blk = 16; continue;
        }
        15 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(8, c.r64(1));
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 41; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t19) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7156); };
            c.w32(1, t19);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x70008b7165);
        }
        19 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b7166); };
            c.w32(9, t20);
            let r21 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r21);
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x70008b7173);
        }
        21 => {
            c.w64(1, c.r64(8).wrapping_sub(0x1));
            let Some(t22) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b7178); };
            c.w32(8, t22);
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(1).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_le() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(11, c.r64(12));
            blk = 23; continue;
        }
        23 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b718e); };
            c.w32(12, t23);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_l() { blk = 45; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b71a4); };
            c.w32(11, t24);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008b71ae); }
            c.cmp(c.r64(1), c.r64(11), 64);
            if c.cc_b() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            return RegfileExit::fault(0x70008b71b7);
        }
        26 => {
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b71c0); };
            c.w64(11, t25);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x70008b71c4); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r26 = c.r64(12) & c.r64(9);
            c.w64(12, r26);
            c.logic(r26, 64);
            let Some(t27) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b71d8); };
            c.logic(t27 & 0x4u64, 8);
            if c.cc_ne() { blk = 79; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t28) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b71e4); };
            c.w32(1, t28);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 30; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008b71f3);
        }
        30 => {
            c.w32(1, c.r64(8).wrapping_sub(0x2));
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(6).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b71fb); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(1, 0x326u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b720d); }
            c.w32(0, 0x1u64);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x25f0)) else { return RegfileExit::fault(0x70008b7213); };
            c.w64(3, t29);
            return c.call(0x700086ed74, 0x70008b721f, 0x70008b721a);
        }
        32 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b721f); };
            c.w64(6, t30);
            let Some(t31) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7223); };
            c.w32(1, t31);
            let r32 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r32);
            let Some(t33) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b7229); };
            c.w32(1, t33);
            let r34 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r34);
            let Some(t35) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b722f); };
            c.w32(8, t35);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008b7239); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            return RegfileExit::fault(0x70008b7243);
        }
        34 => {
            let Some(t36) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008b7244); };
            c.w32(1, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b724a); };
            c.w64(9, t37);
            let Some(t38) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b724e); };
            c.w32(8, t38);
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b7252); };
            c.w64(11, t39);
            let Some(t40) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008b7259); };
            c.w64(12, t40);
            c.w64(15, c.r64(12).wrapping_add(0x10));
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b7261); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b7268); };
            c.cmp(t42, c.r64(15), 64);
            if c.cc_be() { blk = 74; continue; }
            blk = 35; continue;
        }
        35 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008b7271); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 36; continue;
        }
        36 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b7279); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b727d); }
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b7281); };
            c.w64(1, t43);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008b7288); }
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b728c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b7290); }
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x6c8)) else { return RegfileExit::fault(0x70008b7291); };
            c.w64(1, t44);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b7298); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b7299); }
            c.w32(1, 0x4u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b72a0); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2480)) else { return RegfileExit::fault(0x70008b72a1); };
            c.w64(3, t45);
            c.w32(0, 0x4u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008b72b5, 0x70008b72b0);
        }
        37 => {
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b72b5); };
            c.w64(6, t46);
            let Some(t47) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b72b9); };
            c.w32(1, t47);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x70008b72c8);
        }
        39 => {
            let Some(t48) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b72c9); };
            c.w32(1, t48);
            let r49 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r49);
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b72cf); };
            c.w32(7, t50);
            let r51 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b72d5); };
            if !c.push64(t52) { return RegfileExit::fault(0x70008b72d5); }
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b72d8); };
            c.w64(1, t53);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b72df); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b72ea, 0x70008b72e5);
        }
        40 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b72ea); };
            c.w64(0, t54);
            blk = 3; continue;
        }
        41 => {
            return RegfileExit::fault(0x70008b72f6);
        }
        42 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008b72f7); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b72fd); };
            c.w64(3, t55);
            return c.call(0x700086ed74, 0x70008b7309, 0x70008b7304);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b7309); };
            c.w64(6, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b730d); };
            c.w64(7, t57);
            blk = 1; continue;
        }
        44 => {
            return RegfileExit::fault(0x70008b7316);
        }
        45 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008b7317); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008b731b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x70008b731f); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008b7323); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_ae() { blk = 49; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b732d); };
            c.w64(2, t58);
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008b7334); };
            c.w32(2, t59);
            let r60 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r60);
            return c.call(0x7000824234, 0x70008b7342, 0x70008b733d);
        }
        47 => {
            return RegfileExit::fault(0x70008b7342);
        }
        48 => {
            return RegfileExit::fault(0x70008b7343);
        }
        49 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 66; continue; }
            blk = 51; continue;
        }
        51 => {
            let r61 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r61);
            c.w64(15, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w32(0, c.r32(0));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008b7369); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008b736d); }
            let r62 = c.imul2(c.r32(0), 0x4u64, 32);
            c.w32(0, r62);
            if c.cc_o() { blk = 65; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w32(3, 0x8u64);
            let r63 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r63);
            if c.cc_o() { blk = 64; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b738b); };
            c.w64(2, t64);
            let Some(t65) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b7392); };
            c.w64(2, t65);
            c.w64(7, c.r64(2).wrapping_add(c.r64(0)));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008b7399); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b739d); };
            c.w64(2, t66);
            let Some(t67) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b73a4); };
            c.cmp(t67, c.r64(7), 64);
            if c.cc_be() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(0), 0x20000u64, 64);
            if c.cc_b() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(2, c.r64(0));
            return c.call(0x70007febb4, 0x70008b73b9, 0x70008b73b4);
        }
        56 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b73b9); };
            c.w64(8, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b73bd); };
            c.w64(6, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b73c1); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b73c5); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b73c9); };
            c.w64(12, t72);
            c.w32(3, 0x8u64);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b73d2); };
            c.w64(15, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b73d6); };
            c.w64(11, t74);
            blk = 58; continue;
        }
        57 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b73dc); };
            c.w64(0, t75);
            if !c.st64(c.r64(0), c.r64(7)) { return RegfileExit::fault(0x70008b73e3); }
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b73e6); };
            c.w64(0, t76);
            let r77 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r77);
            blk = 58; continue;
        }
        58 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008b73ee); };
            c.w64(2, t78);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b73f5); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b73f8); }
            c.w64(11, c.r64(15));
            c.w64(11, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.w64(15, c.r64(3));
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(2, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_e() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t79) = c.ld32(c.r64(9).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b741c); };
            c.w32(2, t79);
            let r80 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r80);
            blk = 63; continue;
        }
        62 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008b7427); };
            c.w64(2, t81);
            blk = 63; continue;
        }
        63 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(3)).wrapping_sub(0x1), c.r32(2)) { return RegfileExit::fault(0x70008b742e); }
            let r82 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r82);
            blk = 59; continue;
        }
        64 => {
            return RegfileExit::fault(0x70008b7438);
        }
        65 => {
            return RegfileExit::fault(0x70008b7439);
        }
        66 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008b743a); };
            c.w64(3, t83);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b744a, 0x70008b7445);
        }
        67 => {
            return RegfileExit::fault(0x70008b744a);
        }
        68 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b744b); };
            c.w64(0, t84);
            c.w64(9, c.r64(0));
            blk = 69; continue;
        }
        69 => {
            let Some(t85) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7455); };
            c.w32(9, t85);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            return RegfileExit::fault(0x70008b7465);
        }
        71 => {
            if !c.st32(c.r64(6).wrapping_add(0x17), c.r32(0)) { return RegfileExit::fault(0x70008b7466); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r86 = c.r64(9) & c.r64(6);
            c.w64(9, r86);
            c.logic(r86, 64);
            let Some(t87) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b7477); };
            c.logic(t87 & 0x4u64, 8);
            if c.cc_ne() { blk = 76; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(9, c.r64(0));
            blk = 24; continue;
        }
        74 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008b748b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x70008b748f); }
            return c.call(0x70007febf4, 0x70008b7498, 0x70008b7493);
        }
        75 => {
            c.w64(11, c.r64(0));
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b749b); };
            c.w64(0, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b749f); };
            c.w64(6, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b74a3); };
            c.w64(9, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b74a7); };
            c.w64(8, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b74ab); };
            c.w64(1, t92);
            blk = 36; continue;
        }
        76 => {
            c.w32(0, c.r32(0));
            let r93 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r93);
            c.w64(9, 0xfffffffffffc0000u64);
            let r94 = c.r64(9) & c.r64(0);
            c.w64(9, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b74c3); };
            c.logic(t95 & 0x2u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(11, c.r64(6).wrapping_add(0x17));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b74ce); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b74cf); }
            c.w64(7, c.r64(6));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008b74dc, 0x70008b74d7);
        }
        78 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b74dc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b74de); }
            blk = 73; continue;
        }
        79 => {
            c.w32(11, c.r32(11));
            let r96 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r96);
            c.w64(12, 0xfffffffffffc0000u64);
            let r97 = c.r64(12) & c.r64(11);
            c.w64(12, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b74f1); };
            c.logic(t98 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w64(15, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b7501); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7502); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x70008b750f, 0x70008b750a);
        }
        81 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b750f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008b7511); }
            blk = 28; continue;
        }
        82 => {
            c.w32(1, c.r32(1));
            let r99 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r99);
            c.w64(8, 0xfffffffffffc0000u64);
            let r100 = c.r64(8) & c.r64(1);
            c.w64(8, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008b7526); };
            c.logic(t101 & 0x2u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w64(9, c.r64(7).wrapping_add(0xf));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7535); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008b753f, 0x70008b753a);
        }
        84 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008b753f); }
            blk = 8; continue;
        }
        85 => {
            return RegfileExit::jump(0x70008b7548);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b7554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b7554); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b7558); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b7559); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b755a); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b755f); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b7563); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b756d); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b7574); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b757b); };
            c.w64(0, t6);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x70008b757f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b758b); };
            c.w32(9, t7);
            let r8 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r8);
            let Some(t9) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7592); };
            c.w32(9, t9);
            let r10 = c.sub(c.r32(9), 0xbau64, 32);
            c.w32(9, r10);
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 125; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b75ab); };
            c.w64(3, t11);
            return c.call(0x70008b23d4, 0x70008b75b7, 0x70008b75b2);
        }
        4 => {
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b75b7); };
            c.w64(8, t12);
            let Some(t13) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b75bb); };
            c.w32(9, t13);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008b75c5); }
            c.cmp(c.r64(9), 0xeau64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008b75d2);
        }
        6 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b75d3); };
            c.w64(9, t14);
            let Some(t15) = c.ld32(c.r64(8).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b75d7); };
            c.cmp(t15, c.r32(9), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        8 => {
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x1000)) else { return RegfileExit::fault(0x70008b75e8); };
            c.w64(12, t16);
            let Some(t17) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b75ef); };
            c.cmp(t17, c.r32(11), 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b7602); };
            c.w64(11, t18);
            blk = 15; continue;
        }
        11 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b760b); };
            c.w64(3, t19);
            c.w64(0, c.r64(9));
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7615); };
            c.w64(6, t20);
            return c.call(0x7000870914, 0x70008b761e, 0x70008b7619);
        }
        12 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t21) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7626); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b762d); };
            c.logic(t23 & 0x2u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(11, c.r64(0));
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(11)) { return RegfileExit::fault(0x70008b763b); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008b763f); };
            c.w64(3, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b7646); };
            c.w64(0, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b764a); };
            c.w64(6, t26);
            return c.call(0x7000870914, 0x70008b7653, 0x70008b764e);
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 169; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t27) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b765b); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            let Some(t29) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b7662); };
            c.logic(t29 & 0x2u64, 8);
            if c.cc_e() { blk = 169; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b766d); };
            if !c.push64(t30) { return RegfileExit::fault(0x70008b766d); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7675); };
            c.w64(6, t31);
            return c.call(0x70007f2274, 0x70008b767e, 0x70008b7679);
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008b767e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 167; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b768a); };
            c.w32(8, t32);
            let r33 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r33);
            let Some(t34) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7691); };
            c.cmp(t34, 0xbau64, 16);
            if c.cc_b() { blk = 167; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008b769e); };
            c.w64(3, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b76a5); };
            c.w64(6, t36);
            return c.call(0x7000870914, 0x70008b76ae, 0x70008b76a9);
        }
        22 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(0)) { return RegfileExit::fault(0x70008b76ae); }
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b76b2); };
            c.w64(8, t37);
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b76b6); };
            c.w32(9, t38);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x79u64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008b76c6);
        }
        24 => {
            let Some(t39) = c.ld32(c.r64(8).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b76c7); };
            c.w32(9, t39);
            c.w32(11, 0x8u64);
            c.w64(12, c.r64(11));
            let r40 = c.add(c.r64(12), 0x14u64, 64);
            c.w64(12, r40);
            if c.cc_o() { blk = 122; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b76e1); };
            c.w64(15, t41);
            let Some(t42) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b76e8); };
            c.w64(3, t42);
            c.w64(2, c.r64(3).wrapping_add(c.r64(12)));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b76ef); };
            c.w64(1, t43);
            let Some(t44) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b76f6); };
            c.cmp(t44, c.r64(2), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r64(12), 0x20000u64, 64);
            if c.cc_ae() { blk = 129; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008b770c); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            blk = 28; continue;
        }
        28 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b7713); }
            c.w32(9, 0xau64);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b771e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008b7723); }
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 30; continue;
        }
        29 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(11)).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b772c); }
            let r45 = c.add(c.r64(11), 0x4u64, 64);
            c.w64(11, r45);
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r64(11), 0x1cu64, 64);
            if c.cc_ne() { blk = 29; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t46) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b773b); };
            c.w32(9, t46);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x70008b774c);
        }
        33 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b774d); };
            c.w64(11, t47);
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008b7754); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_a() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::fault(0x70008b775f);
        }
        35 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b7760); };
            c.w64(11, t48);
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b7767); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            return RegfileExit::fault(0x70008b7772);
        }
        37 => {
            c.w32(11, 0x2u64);
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b7779); }
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_a() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            return RegfileExit::fault(0x70008b7784);
        }
        39 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b7785); };
            c.w64(3, t49);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(3)) { return RegfileExit::fault(0x70008b7789); }
            c.cmp(c.r64(9), 0x4u64, 64);
            if c.cc_a() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            return RegfileExit::fault(0x70008b7794);
        }
        41 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b7795); };
            c.w64(9, t50);
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(9)) { return RegfileExit::fault(0x70008b779c); }
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b77a1); };
            c.w32(2, t51);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x68u64, 64);
            if c.cc_a() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            return RegfileExit::fault(0x70008b77b1);
        }
        43 => {
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(0x1a7)) else { return RegfileExit::fault(0x70008b77b2); };
            c.w32(2, t52);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x70008b77b9); }
            blk = 45; continue;
        }
        44 => {
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b77bf); };
            c.w64(11, t53);
            c.w32(8, c.r64(11).wrapping_add(0x2));
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b77ca); };
            c.w64(0, t54);
            blk = 45; continue;
        }
        45 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b77ce); };
            if !c.push64(t55) { return RegfileExit::fault(0x70008b77ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008b77d1); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b77da); };
            c.w64(6, t56);
            return c.call(0x70007f21d4, 0x70008b77e3, 0x70008b77de);
        }
        46 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b77e3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 162; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t57) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b77ef); };
            c.w32(8, t57);
            let r58 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r58);
            let Some(t59) = c.ld32(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b77f6); };
            c.cmp(c.r32(8), t59, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t60) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7800); };
            c.cmp(t60, 0xbau64, 16);
            if c.cc_b() { blk = 162; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x648)) else { return RegfileExit::fault(0x70008b780d); };
            c.w64(3, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7814); };
            c.w64(6, t62);
            return c.call(0x7000870914, 0x70008b781d, 0x70008b7818);
        }
        50 => {
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b781d); };
            c.cmp(t63, c.r32(0), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b782d); };
            c.cmp(t64, c.r32(0), 32);
            if c.cc_e() { blk = 58; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t65) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7836); };
            c.w32(8, t65);
            let r66 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r66);
            let Some(t67) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b783d); };
            c.logic(t67 & 0x10u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t68) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b7844); };
            c.cmp(t68, c.r32(8), 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t69) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7851); };
            c.cmp(t69, 0x41u64, 16);
            if c.cc_e() { blk = 131; continue; }
            blk = 56; continue;
        }
        56 => {
            blk = 111; continue;
        }
        57 => {
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7862); };
            c.w64(0, t70);
            blk = 70; continue;
        }
        58 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b786b); };
            c.w64(0, t71);
            blk = 70; continue;
        }
        59 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b787e); };
            c.w64(0, t72);
            blk = 70; continue;
        }
        61 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7884); };
            c.w64(0, t73);
            blk = 70; continue;
        }
        62 => {
            let Some(t74) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b788a); };
            c.w32(8, t74);
            let r75 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r75);
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b7891); };
            c.cmp(t76, c.r32(8), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008b78a3); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t78) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b78ac); };
            c.w32(9, t78);
            let r79 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r79);
            let Some(t80) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b78b3); };
            c.logic(t80 & 0x10u64, 8);
            if c.cc_ne() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008b78ba); };
            c.cmp(t81, c.r32(9), 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t82) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b78c7); };
            c.cmp(t82, 0x41u64, 16);
            if c.cc_e() { blk = 135; continue; }
            blk = 68; continue;
        }
        68 => {
            blk = 111; continue;
        }
        69 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008b78e2); };
            c.w64(8, t83);
            let Some(t84) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b78e6); };
            c.cmp(t84, c.r32(8), 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xb50)) else { return RegfileExit::fault(0x70008b78ec); };
            c.w64(3, t85);
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b78f3); };
            c.w64(6, t86);
            return c.call(0x7000870914, 0x70008b78fc, 0x70008b78f7);
        }
        72 => {
            blk = 74; continue;
        }
        73 => {
            let Some(t87) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b78fe); };
            c.w32(0, t87);
            let r88 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r88);
            blk = 74; continue;
        }
        74 => {
            let Some(t89) = c.ld32(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b7904); };
            c.cmp(t89, 0x1ffffeu64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008b7911); };
            c.w64(7, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b7915); };
            c.cmp(t91, c.r32(7), 32);
            if c.cc_ne() { blk = 139; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(3, c.r64(0));
            let Some(t92) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b7925); };
            c.w64(0, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7929); };
            c.w64(6, t93);
            return c.call(0x70008ba474, 0x70008b7932, 0x70008b792d);
        }
        77 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b7932); };
            c.w64(8, t94);
            let Some(t95) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7936); };
            c.w32(9, t95);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x70008b7940); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_be() { blk = 118; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t96) = c.ld32(c.r64(8).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b7951); };
            c.w32(9, t96);
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0x10a8)) else { return RegfileExit::fault(0x70008b7958); };
            c.w64(11, t97);
            let Some(t98) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b795f); };
            c.w32(12, t98);
            let r99 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r99);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(13).wrapping_add(0x63f0));
            let Some(t100) = c.ld64(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b7976); };
            c.w64(12, t100);
            blk = 99; continue;
        }
        80 => {
            let Some(t101) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b797f); };
            c.w32(15, t101);
            let r102 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r102);
            let Some(t103) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7987); };
            c.w32(15, t103);
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_l() { blk = 87; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_l() { blk = 85; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x85u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r32(15), 0x9du64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(15), 0x9eu64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(15), 0x5cu64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r32(15), 0x5eu64, 32);
            if c.cc_e() { blk = 94; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_l() { blk = 90; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(15), 0x4au64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(15), 0x53u64, 32);
            if c.cc_e() { blk = 96; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(15), 0x48u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(15), 0x49u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b79e0); };
            c.w32(12, t104);
            blk = 99; continue;
        }
        93 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b79e7); };
            c.w64(12, t105);
            blk = 99; continue;
        }
        94 => {
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b79f0); };
            c.w32(12, t106);
            blk = 99; continue;
        }
        95 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b79f7); };
            c.w64(12, t107);
            blk = 99; continue;
        }
        96 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b7a00); };
            c.w64(12, t108);
            blk = 99; continue;
        }
        97 => {
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b7a09); };
            c.w32(12, t109);
            blk = 99; continue;
        }
        98 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b7a10); };
            c.w64(12, t110);
            blk = 99; continue;
        }
        99 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b7a17); };
            c.w64(15, t111);
            let Some(t112) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008b7a1e); };
            c.w64(3, t112);
            c.w64(2, c.r64(3).wrapping_add(0x1c));
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b7a25); };
            c.w64(1, t113);
            let Some(t114) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b7a2c); };
            c.cmp(t114, c.r64(2), 64);
            if c.cc_be() { blk = 141; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.st64(c.r64(15), c.r64(2)) { return RegfileExit::fault(0x70008b7a35); }
            let r115 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r115);
            blk = 101; continue;
        }
        101 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b7a3c); }
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b7a40); };
            c.w64(9, t116);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b7a47); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b7a4b); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b7a4f); };
            c.w64(2, t117);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(2)) { return RegfileExit::fault(0x70008b7a56); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b7a59); }
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b7a5d); };
            c.w64(11, t118);
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b7a61); }
            if !c.st32(c.r64(3).wrapping_add(0x17), c.r32(12)) { return RegfileExit::fault(0x70008b7a65); }
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b7a69); };
            c.w64(12, t119);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008b7a6d); }
            let Some(t120) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7a71); };
            c.w32(2, t120);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008b7a7b); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_be() { blk = 119; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t121) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b7a89); };
            c.w32(2, t121);
            let r122 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r122);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(2)) { return RegfileExit::fault(0x70008b7a93); }
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b7a97); };
            c.w64(2, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7a9e); };
            c.w64(6, t124);
            c.w64(3, c.r64(2));
            return c.call(0x7000870914, 0x70008b7aaa, 0x70008b7aa5);
        }
        103 => {
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b7aaa); };
            c.w64(8, t125);
            let Some(t126) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7aae); };
            c.w32(9, t126);
            let r127 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7ab5); };
            if !c.push64(t128) { return RegfileExit::fault(0x70008b7ab5); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b7ab8); }
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b7aba); };
            if !c.push64(t129) { return RegfileExit::fault(0x70008b7aba); }
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7abd); };
            c.w64(6, t130);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x70008b7ad1, 0x70008b7acc);
        }
        104 => {
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b7ad1); };
            c.w64(8, t131);
            let Some(t132) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x70008b7ad8); };
            c.cmp(t132, 0x0u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 105; continue;
        }
        105 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t133) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7ae6); };
            c.w32(8, t133);
            let r134 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r134);
            let Some(t135) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7aed); };
            c.cmp(t135, 0x83du64, 16);
            if c.cc_ne() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w32(8, 0x1u64);
            blk = 109; continue;
        }
        108 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 110; continue;
        }
        110 => {
            blk = 44; continue;
        }
        111 => {
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b7b10); };
            c.w64(0, t136);
            let Some(t137) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7b14); };
            c.w32(8, t137);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_a() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            return RegfileExit::fault(0x70008b7b24);
        }
        113 => {
            let Some(t138) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b7b25); };
            c.w32(9, t138);
            let r139 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r139);
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(0).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b7b30); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 146; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b7b3e); };
            c.w64(9, t140);
            let Some(t141) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7b42); };
            c.w32(0, t141);
            let r142 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r142);
            blk = 115; continue;
        }
        115 => {
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b7b49); };
            c.w64(1, t143);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b7b50); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            return c.ret(0x10, 0x70008b7b57);
        }
        117 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b7b5a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b7b61); }
            return c.ret(0x0, 0x70008b7b63);
        }
        118 => {
            return RegfileExit::fault(0x70008b7b64);
        }
        119 => {
            return RegfileExit::fault(0x70008b7b65);
        }
        120 => {
            c.w32(8, 0x294u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7b6c); }
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b7b6e); };
            c.w64(8, t144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(0x24f)) else { return RegfileExit::fault(0x70008b7b75); };
            c.w32(8, t145);
            let r146 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r146);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7b7f); }
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2218)) else { return RegfileExit::fault(0x70008b7b81); };
            c.w64(3, t147);
            c.w32(0, 0x2u64);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7b8d); };
            c.w64(6, t148);
            return c.call(0x700086ed74, 0x70008b7b96, 0x70008b7b91);
        }
        121 => {
            return RegfileExit::fault(0x70008b7b96);
        }
        122 => {
            return RegfileExit::fault(0x70008b7b97);
        }
        123 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7b9e); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008b7ba0); };
            c.w64(8, t149);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7ba7); }
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b7ba9); };
            c.w64(3, t150);
            c.w32(0, 0x2u64);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7bb5); };
            c.w64(6, t151);
            return c.call(0x700086ed74, 0x70008b7bbe, 0x70008b7bb9);
        }
        124 => {
            return RegfileExit::fault(0x70008b7bbe);
        }
        125 => {
            c.w32(8, 0x34u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7bc5); }
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b7bc7); };
            c.w64(8, t152);
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(0x253)) else { return RegfileExit::fault(0x70008b7bce); };
            c.w32(8, t153);
            let r154 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r154);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7bd8); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b7bda); };
            c.w64(3, t155);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b7beb, 0x70008b7be6);
        }
        126 => {
            return RegfileExit::fault(0x70008b7beb);
        }
        127 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b7bee); };
            c.w64(3, t156);
            return c.call(0x700086ed74, 0x70008b7bfa, 0x70008b7bf5);
        }
        128 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7bfa); };
            c.w64(6, t157);
            blk = 1; continue;
        }
        129 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008b7c03); }
            c.w64(2, c.r64(12));
            return c.call(0x70007febb4, 0x70008b7c0f, 0x70008b7c0a);
        }
        130 => {
            c.w64(12, c.r64(0));
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b7c12); };
            c.w64(15, t158);
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b7c19); };
            c.w64(1, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b7c20); };
            c.w64(8, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008b7c24); };
            c.w64(0, t161);
            c.w32(11, 0x8u64);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7c2e); };
            c.w64(9, t162);
            blk = 28; continue;
        }
        131 => {
            let Some(t163) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7c37); };
            c.logic(t163 & 0x7ffffffeu64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7c44); };
            c.w64(0, t164);
            blk = 70; continue;
        }
        133 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b7c4d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 111; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7c60); };
            c.w64(0, t165);
            blk = 70; continue;
        }
        135 => {
            let Some(t166) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7c69); };
            c.logic(t166 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 136; continue;
        }
        136 => {
            blk = 111; continue;
        }
        137 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008b7c7c); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 111; continue; }
            blk = 138; continue;
        }
        138 => {
            blk = 70; continue;
        }
        139 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b7c95); }
            let Some(t167) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b7c96); };
            if !c.push64(t167) { return RegfileExit::fault(0x70008b7c96); }
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7c99); };
            c.w64(6, t168);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x70008b7ca7, 0x70008b7ca2);
        }
        140 => {
            blk = 77; continue;
        }
        141 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008b7cac); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008b7cb0); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008b7cbe, 0x70008b7cb9);
        }
        142 => {
            c.w64(3, c.r64(0));
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b7cc1); };
            c.w64(15, t169);
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b7cc8); };
            c.w64(1, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b7ccf); };
            c.w64(0, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b7cd3); };
            c.w64(12, t172);
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x10a8)) else { return RegfileExit::fault(0x70008b7cd7); };
            c.w64(11, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008b7cde); };
            c.w64(9, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008b7ce2); };
            c.w64(8, t175);
            blk = 101; continue;
        }
        143 => {
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b7ceb); };
            c.w64(8, t176);
            let Some(t177) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7cef); };
            c.w32(1, t177);
            let r178 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r178);
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0xc40)) else { return RegfileExit::fault(0x70008b7cf6); };
            c.w64(3, t179);
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7cfd); };
            c.w64(6, t180);
            return c.call(0x7000871cb4, 0x70008b7d06, 0x70008b7d01);
        }
        144 => {
            let Some(t181) = c.ld64(c.r64(13).wrapping_add(0xc38)) else { return RegfileExit::fault(0x70008b7d06); };
            c.w64(3, t181);
            let Some(t182) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b7d0d); };
            c.w64(1, t182);
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7d14); };
            c.w64(0, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7d18); };
            c.w64(6, t184);
            return c.call(0x7000871cb4, 0x70008b7d21, 0x70008b7d1c);
        }
        145 => {
            blk = 44; continue;
        }
        146 => {
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_a() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            return RegfileExit::fault(0x70008b7d2c);
        }
        148 => {
            let Some(t185) = c.ld32(c.r64(0).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008b7d2d); };
            c.w32(8, t185);
            let r186 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r186);
            c.w32(9, 0x326u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b7d3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008b7d3c); }
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x25f0)) else { return RegfileExit::fault(0x70008b7d40); };
            c.w64(3, t187);
            c.w32(0, 0x1u64);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7d4c); };
            c.w64(6, t188);
            return c.call(0x700086ed74, 0x70008b7d55, 0x70008b7d50);
        }
        149 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b7d55); }
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7d59); };
            c.w64(6, t189);
            let Some(t190) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7d5d); };
            c.w32(8, t190);
            let r191 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r191);
            let Some(t192) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b7d64); };
            c.w32(8, t192);
            let r193 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r193);
            let Some(t194) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7d6b); };
            c.w32(9, t194);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x6bu64, 64);
            if c.cc_a() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            return RegfileExit::fault(0x70008b7d7b);
        }
        151 => {
            let Some(t195) = c.ld32(c.r64(8).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008b7d7c); };
            c.w32(8, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b7d83); };
            c.w64(9, t196);
            let Some(t197) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7d87); };
            c.w32(11, t197);
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b7d8b); };
            c.w64(12, t198);
            let Some(t199) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b7d92); };
            c.w64(15, t199);
            c.w64(3, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008b7d9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008b7d9e); }
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b7da2); };
            c.w64(2, t200);
            let Some(t201) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b7da9); };
            c.cmp(t201, c.r64(3), 64);
            if c.cc_a() { blk = 154; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008b7db8, 0x70008b7db3);
        }
        153 => {
            c.w64(12, c.r64(0));
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7dbb); };
            c.w64(6, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7dbf); };
            c.w64(0, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b7dc3); };
            c.w64(9, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b7dc7); };
            c.w64(11, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b7dcb); };
            c.w64(8, t206);
            blk = 155; continue;
        }
        154 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008b7dd1); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 155; continue;
        }
        155 => {
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b7dd9); }
            c.w64(8, c.r64(11));
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b7de1); }
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b7de6); };
            c.w64(8, t207);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b7ded); }
            c.w64(8, c.r64(9));
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008b7df5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b7dfa); }
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x6c8)) else { return RegfileExit::fault(0x70008b7dfb); };
            c.w64(8, t208);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7e02); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b7e04); }
            c.w32(8, 0x4u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b7e0c); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x2480)) else { return RegfileExit::fault(0x70008b7e0e); };
            c.w64(3, t209);
            c.w32(0, 0x4u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008b7e22, 0x70008b7e1d);
        }
        156 => {
            blk = 165; continue;
        }
        157 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b7e24); }
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x9e8)) else { return RegfileExit::fault(0x70008b7e28); };
            c.w64(3, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b7e2f); };
            c.w64(0, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7e33); };
            c.w64(6, t212);
            return c.call(0x7000870914, 0x70008b7e3c, 0x70008b7e37);
        }
        158 => {
            let Some(t213) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b7e3c); };
            c.cmp(t213, c.r32(0), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t214) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008b7e45); };
            c.cmp(t214, c.r32(0), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b7e4e); };
            if !c.push64(t215) { return RegfileExit::fault(0x70008b7e4e); }
            c.w64(7, c.r64(0));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7e56); };
            c.w64(6, t216);
            return c.call(0x70007f2274, 0x70008b7e5f, 0x70008b7e5a);
        }
        161 => {
            blk = 165; continue;
        }
        162 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b7e61); }
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x2200)) else { return RegfileExit::fault(0x70008b7e62); };
            c.w64(3, t217);
            c.w32(0, 0x1u64);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7e6e); };
            c.w64(6, t218);
            return c.call(0x700086ed74, 0x70008b7e77, 0x70008b7e72);
        }
        163 => {
            blk = 166; continue;
        }
        164 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008b7e79); }
            blk = 165; continue;
        }
        165 => {
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b7e7d); };
            c.w64(0, t219);
            blk = 171; continue;
        }
        166 => {
            return RegfileExit::fault(0x70008b7e83);
        }
        167 => {
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x2240)) else { return RegfileExit::fault(0x70008b7e84); };
            c.w64(3, t220);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7e8d); };
            c.w64(6, t221);
            return c.call(0x700086ed74, 0x70008b7e96, 0x70008b7e91);
        }
        168 => {
            return RegfileExit::fault(0x70008b7e96);
        }
        169 => {
            let Some(t222) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b7e97); };
            if !c.push64(t222) { return RegfileExit::fault(0x70008b7e97); }
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x21f0)) else { return RegfileExit::fault(0x70008b7e9a); };
            c.w64(3, t223);
            c.w32(0, 0x1u64);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7ea6); };
            c.w64(6, t224);
            return c.call(0x700086ed74, 0x70008b7eaf, 0x70008b7eaa);
        }
        170 => {
            return RegfileExit::fault(0x70008b7eaf);
        }
        171 => {
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b7eb0); };
            c.w64(8, t225);
            let Some(t226) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b7eb4); };
            c.w32(7, t226);
            let r227 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r227);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b7ebb); }
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b7ebc); };
            c.w64(9, t228);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b7ec3); }
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b7ec5); };
            c.w64(6, t229);
            c.w32(0, 0x1u64);
            return c.call(0x70007f2134, 0x70008b7ed3, 0x70008b7ece);
        }
        172 => {
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b7ed3); };
            c.w64(8, t230);
            let Some(t231) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7ed7); };
            c.w32(0, t231);
            let r232 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r232);
            blk = 115; continue;
        }
        173 => {
            return RegfileExit::jump(0x70008b7ee4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b7ef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b7ef4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b7ef8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b7ef9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b7efa); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008b7eff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b7f03); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 171; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b7f0d); };
            c.cmp(t2, c.r32(2), 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b7f1a); };
            c.w64(7, t3);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7f24); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld8(c.r64(1).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b7f2a); };
            c.logic(t6 & 0x2u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(1, 0x1u64);
            blk = 6; continue;
        }
        5 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 6; continue;
        }
        6 => {
            c.w32(0, 0x1u64);
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b7f47); };
            c.w32(1, t7);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xeau64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008b7f59);
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b7f5a); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            c.w64(8, c.r64(7));
            blk = 11; continue;
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b7f68); };
            c.w32(8, t10);
            let r11 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r11);
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7f75); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7f7c); };
            c.w32(9, t14);
            let r15 = c.sub(c.r32(9), 0x80cu64, 32);
            c.w32(9, r15);
            c.cmp(c.r32(9), 0xeu64, 32);
            if c.cc_be() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        14 => {
            c.w64(9, c.r64(0));
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 173; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7fa0); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b7fa7); };
            c.cmp(t18, 0x80bu64, 16);
            if c.cc_ne() { blk = 173; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 10; continue;
        }
        18 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b7fb6); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7fbd); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b7fc4); };
            c.cmp(t23, c.r32(6), 32);
            if c.cc_ne() { blk = 173; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t24) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7fd6); };
            c.w32(8, t24);
            let r25 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r25);
            let Some(t26) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b7fdd); };
            c.logic(t26 & 0xffffffffffffff80u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t27) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b7fe8); };
            c.w32(8, t27);
            let r28 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r28);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 52; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b7ff9); };
            c.w32(9, t29);
            let r30 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r30);
            let Some(t31) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8000); };
            c.cmp(t31, 0xacu64, 16);
            if c.cc_ne() { blk = 52; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b800d); };
            c.cmp(t32, c.r32(1), 32);
            if c.cc_ne() { blk = 51; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t33) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b8017); };
            c.logic(t33 & 0x200000u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b8021); };
            c.w64(12, t34);
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b8028); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b802f); };
            c.w64(2, t36);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b8039); };
            c.w64(1, t37);
            blk = 38; continue;
        }
        26 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b8045); };
            c.w64(1, t38);
            let Some(t39) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b804c); };
            c.w64(9, t39);
            c.w64(11, c.r64(9).wrapping_add(0x4c));
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b8053); };
            c.w64(12, t40);
            let Some(t41) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b805a); };
            c.cmp(t41, c.r64(11), 64);
            if c.cc_be() { blk = 176; continue; }
            blk = 27; continue;
        }
        27 => {
            if !c.st64(c.r64(1), c.r64(11)) { return RegfileExit::fault(0x70008b8064); }
            let r42 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r42);
            blk = 28; continue;
        }
        28 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x210)) else { return RegfileExit::fault(0x70008b806b); };
            c.w64(11, t43);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008b8072); }
            c.w32(11, 0x22u64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b807c); }
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8080); };
            c.w32(11, t44);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 178; continue; }
            blk = 29; continue;
        }
        29 => {
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008b8097); }
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b809b); };
            c.w32(15, t45);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 179; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b80af); }
            let Some(t46) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b80b3); };
            c.w32(15, t46);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x2u64, 64);
            if c.cc_be() { blk = 180; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w32(15, 0x8u64);
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(15)) { return RegfileExit::fault(0x70008b80cd); }
            let Some(t47) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b80d1); };
            c.w32(15, t47);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x3u64, 64);
            if c.cc_be() { blk = 181; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(15, 0x2u64);
            if !c.st32(c.r64(9).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008b80eb); }
            let Some(t48) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b80ef); };
            c.w32(15, t48);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x4u64, 64);
            if c.cc_be() { blk = 182; continue; }
            blk = 33; continue;
        }
        33 => {
            if !c.st32(c.r64(9).wrapping_add(0x17), c.r32(11)) { return RegfileExit::fault(0x70008b8103); }
            c.w64(15, c.r64(9).wrapping_add(0x4b));
            c.w64(3, c.r64(9).wrapping_add(0x1b));
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8114); };
            c.w64(2, t49);
            blk = 35; continue;
        }
        35 => {
            if !c.st32(c.r64(3), c.r32(2)) { return RegfileExit::fault(0x70008b811b); }
            let r50 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r50);
            c.cmp(c.r64(15), c.r64(3), 64);
            if c.cc_ne() { blk = 35; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 38; continue;
        }
        37 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8128); };
            c.w64(2, t51);
            blk = 38; continue;
        }
        38 => {
            let Some(t52) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b812f); };
            c.w32(15, t52);
            if let Some(sr) = c.shl(c.r64(15), 0x2u64, 64) { c.w64(15, sr); }
            let Some(t53) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b8138); };
            c.w64(3, t53);
            c.w64(6, c.r64(15).wrapping_add(c.r64(3)));
            let Some(t54) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b813f); };
            c.cmp(t54, c.r64(6), 64);
            if c.cc_be() { blk = 183; continue; }
            blk = 39; continue;
        }
        39 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x70008b8149); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 40; continue;
        }
        40 => {
            c.w64(12, c.r64(15).wrapping_sub(0x1));
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b8154); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b8158); }
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b815c); };
            c.w64(9, t55);
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b8163); }
            let Some(t56) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b8167); };
            c.w32(9, t56);
            c.w64(15, c.r64(9));
            let r57 = c.r32(15) & 0xe0000000u64;
            c.w32(15, r57);
            c.logic(r57, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 54; continue; }
            blk = 42; continue;
        }
        42 => {
            let r58 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r58);
            if !c.st32(c.r64(12).wrapping_add(c.r64(1)), c.r32(2)) { return RegfileExit::fault(0x70008b8185); }
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_ne() { blk = 42; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 54; continue;
        }
        44 => {
            let r59 = c.sub(c.r32(9), 0x20000000u64, 32);
            c.w32(9, r59);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008b819b); }
            let Some(t60) = c.ld8(c.r64(8).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008b819f); };
            c.w32(9, t60);
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x88)) else { return RegfileExit::fault(0x70008b81b1); };
            c.w64(3, t61);
            blk = 46; continue;
        }
        46 => {
            let r62 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r62);
            if !c.st32(c.r64(12).wrapping_add(c.r64(1)), c.r32(3)) { return RegfileExit::fault(0x70008b81bc); }
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ne() { blk = 46; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let r63 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r63);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(2)) { return RegfileExit::fault(0x70008b81cf); }
            c.cmp(c.r64(9), 0xbu64, 64);
            if c.cc_ne() { blk = 48; continue; }
            blk = 49; continue;
        }
        49 => {
            let r64 = c.add(c.r32(15), 0xe0000000u64, 32);
            c.w32(15, r64);
            if c.cc_e() { blk = 185; continue; }
            blk = 50; continue;
        }
        50 => {
            blk = 54; continue;
        }
        51 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b81e8); };
            c.w64(8, t65);
            blk = 52; continue;
        }
        52 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b81ef); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b81f0); }
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2408)) else { return RegfileExit::fault(0x70008b81f1); };
            c.w64(3, t66);
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008b8205, 0x70008b8200);
        }
        53 => {
            c.w64(1, c.r64(0));
            let Some(t67) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b8208); };
            c.w64(7, t67);
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b820c); };
            c.w64(2, t68);
            c.w32(0, 0x1u64);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 54; continue;
        }
        54 => {
            c.w64(8, c.r64(11));
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008b821e); }
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x70008b8222); }
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b8226); };
            c.w64(9, t69);
            let Some(t70) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x70008b822d); };
            c.w32(9, t70);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008b8230); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008b8234); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8248); };
            c.w64(6, t71);
            blk = 66; continue;
        }
        57 => {
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8251); };
            c.w64(6, t72);
            let Some(t73) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8255); };
            c.w32(11, t73);
            let r74 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r74);
            let Some(t75) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b825c); };
            c.w32(11, t75);
            let r76 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r76);
            let Some(t77) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8263); };
            c.w32(12, t77);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x8eu64, 64);
            if c.cc_a() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            return RegfileExit::fault(0x70008b8276);
        }
        59 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008b8277); };
            c.w32(11, t78);
            let r79 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r79);
            let Some(t80) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8281); };
            c.w32(12, t80);
            let r81 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r81);
            let Some(t82) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b8288); };
            c.logic(t82 & 0x2u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(2));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t83) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8299); };
            c.w32(15, t83);
            let r84 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r84);
            let Some(t85) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b82a1); };
            c.w32(15, t85);
            c.cmp(c.r16(15), 0x83du64, 16);
            if c.cc_ne() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w32(15, c.r16(15));
            c.cmp(c.r32(15), 0x83du64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            return RegfileExit::fault(0x70008b82bb);
        }
        64 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b82bc); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b82be); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b82bf); }
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(11));
            c.w64(15, c.r64(0));
            return c.call(0x70007f2134, 0x70008b82d1, 0x70008b82cc);
        }
        65 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b82d1); };
            c.w64(6, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b82d5); };
            c.w64(7, t87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b82d9); };
            c.w64(2, t88);
            c.w32(0, 0x1u64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b82e8); };
            c.w64(1, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b82ec); };
            c.w64(9, t90);
            blk = 66; continue;
        }
        66 => {
            c.logic(c.r8(9) & 0x6u64, 8);
            if c.cc_e() { blk = 89; continue; }
            blk = 67; continue;
        }
        67 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b82fa); }
            c.w64(9, c.r64(2));
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b82fe); }
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x70008b8300); };
            c.w64(3, t91);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008b8314, 0x70008b830f);
        }
        68 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8314); };
            c.w64(6, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b8318); };
            c.w64(7, t93);
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b831c); };
            c.w64(2, t94);
            c.w32(0, 0x1u64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b832b); };
            c.w64(1, t95);
            blk = 89; continue;
        }
        69 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8334); };
            c.w64(6, t96);
            blk = 89; continue;
        }
        70 => {
            let Some(t97) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b833d); };
            c.w32(1, t97);
            let r98 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r98);
            let Some(t99) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b8343); };
            c.w32(1, t99);
            let r100 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r100);
            let Some(t101) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8349); };
            c.w32(8, t101);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xeau64, 64);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            return RegfileExit::fault(0x70008b835c);
        }
        72 => {
            let Some(t102) = c.ld32(c.r64(1).wrapping_add(0x3af)) else { return RegfileExit::fault(0x70008b835d); };
            c.w32(1, t102);
            let r103 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r103);
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b8366); };
            c.w64(8, t104);
            let Some(t105) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008b836d); };
            c.w64(9, t105);
            let Some(t106) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008b8370); };
            c.w32(1, t106);
            c.w64(11, c.r64(9).wrapping_add(0x14));
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b8377); };
            c.w64(12, t107);
            let Some(t108) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b837e); };
            c.cmp(t108, c.r64(11), 64);
            if c.cc_be() { blk = 189; continue; }
            blk = 73; continue;
        }
        73 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008b8388); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 74; continue;
        }
        74 => {
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008b838f); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b8393); };
            c.w64(1, t109);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008b839a); }
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008b839e); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008b83a4); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008b83a8); }
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008b83ac); }
            if !c.st32(c.r64(8).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008b83b0); }
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x70008b83b4); };
            c.w64(9, t110);
            let Some(t111) = c.ld32(c.r64(9)) else { return RegfileExit::fault(0x70008b83bb); };
            c.w32(9, t111);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008b83be); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008b83c2); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 75; continue;
        }
        75 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t112) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b83da); };
            c.w32(11, t112);
            let r113 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r113);
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b83e1); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b83e8); };
            c.w32(12, t116);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x8eu64, 64);
            if c.cc_a() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            return RegfileExit::fault(0x70008b83fb);
        }
        78 => {
            let Some(t117) = c.ld32(c.r64(11).wrapping_add(0x23f)) else { return RegfileExit::fault(0x70008b83fc); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8406); };
            c.w32(12, t119);
            let r120 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r120);
            let Some(t121) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008b840d); };
            c.logic(t121 & 0x2u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8415); };
            c.w64(12, t122);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t123) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8422); };
            c.w32(15, t123);
            let r124 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r124);
            let Some(t125) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b842a); };
            c.w32(15, t125);
            c.cmp(c.r16(15), 0x83du64, 16);
            if c.cc_ne() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w32(15, c.r16(15));
            c.cmp(c.r32(15), 0x83du64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            return RegfileExit::fault(0x70008b8444);
        }
        83 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b8445); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b8447); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b8449); }
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(11));
            c.w64(15, c.r64(0));
            return c.call(0x70007f2134, 0x70008b845b, 0x70008b8456);
        }
        84 => {
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b845b); };
            c.w64(6, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b845f); };
            c.w64(7, t127);
            c.w32(0, 0x1u64);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b846a); };
            c.w64(8, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b846e); };
            c.w64(9, t129);
            blk = 85; continue;
        }
        85 => {
            c.logic(c.r8(9) & 0x6u64, 8);
            if c.cc_e() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b8478); }
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b847a); };
            c.w64(9, t130);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008b8481); }
            let Some(t131) = c.ld64(c.r64(13).wrapping_add(0x2598)) else { return RegfileExit::fault(0x70008b8483); };
            c.w64(3, t131);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008b8497, 0x70008b8492);
        }
        87 => {
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8497); };
            c.w64(6, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b849b); };
            c.w64(7, t133);
            c.w32(0, 0x1u64);
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b84a6); };
            c.w64(8, t134);
            blk = 88; continue;
        }
        88 => {
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(8));
            c.w64(8, c.r64(10));
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b84b3); };
            c.w64(2, t135);
            blk = 89; continue;
        }
        89 => {
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x2e20)) else { return RegfileExit::fault(0x70008b84ba); };
            c.w64(9, t136);
            let Some(t137) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x70008b84c1); };
            c.cmp(t137, 0x0u64, 8);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008b84cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008b84d1); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 90; continue;
        }
        90 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b84db); }
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x1ed8)) else { return RegfileExit::fault(0x70008b84dc); };
            c.w64(3, t138);
            return c.call(0x700086ed74, 0x70008b84e8, 0x70008b84e3);
        }
        91 => {
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b84e8); };
            c.w64(6, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b84ec); };
            c.w64(7, t140);
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b84f0); };
            c.w64(2, t141);
            c.w32(0, 0x1u64);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b84fc); };
            c.w64(1, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8500); };
            c.w64(9, t143);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 92; continue;
        }
        92 => {
            let Some(t144) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8507); };
            c.w32(11, t144);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x79u64, 64);
            if c.cc_a() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            return RegfileExit::fault(0x70008b8517);
        }
        94 => {
            let Some(t145) = c.ld32(c.r64(6).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b8518); };
            c.w32(11, t145);
            c.w32(12, 0x8u64);
            c.w64(15, c.r64(12));
            let r146 = c.add(c.r64(15), 0x14u64, 64);
            c.w64(15, r146);
            if c.cc_o() { blk = 166; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b8532); };
            c.w64(3, t147);
            let Some(t148) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b8539); };
            c.w64(3, t148);
            c.w64(7, c.r64(3).wrapping_add(c.r64(15)));
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b8540); };
            c.w64(9, t149);
            let Some(t150) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008b8547); };
            c.cmp(t150, c.r64(7), 64);
            if c.cc_be() { blk = 193; continue; }
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 193; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b855d); };
            c.w64(9, t151);
            if !c.st64(c.r64(9), c.r64(7)) { return RegfileExit::fault(0x70008b8564); }
            c.w64(9, c.r64(3).wrapping_add(0x1));
            blk = 98; continue;
        }
        98 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008b856b); }
            c.w32(11, 0xau64);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008b8575); }
            blk = 100; continue;
        }
        99 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(12)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b857b); }
            let r152 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r152);
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r64(12), 0x1cu64, 64);
            if c.cc_ne() { blk = 99; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t153) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b858a); };
            c.w32(8, t153);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_a() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            return RegfileExit::fault(0x70008b859a);
        }
        103 => {
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b859b); };
            c.w64(11, t154);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x70008b85a2); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_a() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            return RegfileExit::fault(0x70008b85ac);
        }
        105 => {
            c.w64(11, c.r64(2));
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008b85b0); }
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_a() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            return RegfileExit::fault(0x70008b85ba);
        }
        107 => {
            if !c.st32(c.r64(9).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008b85bb); }
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_a() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            return RegfileExit::fault(0x70008b85c5);
        }
        109 => {
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008b85c6); };
            c.w64(12, t155);
            if !c.st32(c.r64(9).wrapping_add(0x13), c.r32(12)) { return RegfileExit::fault(0x70008b85cd); }
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_a() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            return RegfileExit::fault(0x70008b85d7);
        }
        111 => {
            let Some(t156) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b85d8); };
            c.w64(8, t156);
            if !c.st32(c.r64(9).wrapping_add(0x17), c.r32(8)) { return RegfileExit::fault(0x70008b85df); }
            let Some(t157) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b85e3); };
            c.w32(8, t157);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xbeu64, 64);
            if c.cc_a() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            return RegfileExit::fault(0x70008b85f6);
        }
        113 => {
            let Some(t158) = c.ld32(c.r64(6).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b85f7); };
            c.w32(8, t158);
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x10b8)) else { return RegfileExit::fault(0x70008b85fe); };
            c.w64(12, t159);
            let Some(t160) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8605); };
            c.w32(15, t160);
            let r161 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r161);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 115; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(13).wrapping_add(0x63f0));
            let Some(t162) = c.ld64(c.r64(3).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b861d); };
            c.w64(15, t162);
            blk = 134; continue;
        }
        115 => {
            let Some(t163) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8626); };
            c.w32(3, t163);
            let r164 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r164);
            let Some(t165) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b862d); };
            c.w32(3, t165);
            c.cmp(c.r32(3), 0x5cu64, 32);
            if c.cc_l() { blk = 122; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(3), 0x85u64, 32);
            if c.cc_l() { blk = 120; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(3), 0x85u64, 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(3), 0x9du64, 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(3), 0x9eu64, 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(3), 0x5cu64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(3), 0x5eu64, 32);
            if c.cc_e() { blk = 129; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(3), 0x4au64, 32);
            if c.cc_l() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(3), 0x4au64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(3), 0x53u64, 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(3), 0x48u64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(3), 0x49u64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t166) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8679); };
            c.w32(15, t166);
            blk = 134; continue;
        }
        128 => {
            let Some(t167) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b867f); };
            c.w64(15, t167);
            blk = 134; continue;
        }
        129 => {
            let Some(t168) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8688); };
            c.w32(15, t168);
            blk = 134; continue;
        }
        130 => {
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b868e); };
            c.w64(15, t169);
            blk = 134; continue;
        }
        131 => {
            let Some(t170) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b8697); };
            c.w64(15, t170);
            blk = 134; continue;
        }
        132 => {
            let Some(t171) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b86a0); };
            c.w32(15, t171);
            blk = 134; continue;
        }
        133 => {
            let Some(t172) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b86a6); };
            c.w64(15, t172);
            blk = 134; continue;
        }
        134 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b86ad); };
            c.w64(3, t173);
            let Some(t174) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b86b4); };
            c.w64(2, t174);
            c.w64(7, c.r64(2).wrapping_add(0x1c));
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b86bb); };
            c.w64(1, t175);
            let Some(t176) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b86c2); };
            c.cmp(t176, c.r64(7), 64);
            if c.cc_be() { blk = 195; continue; }
            blk = 135; continue;
        }
        135 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008b86cb); }
            let r177 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r177);
            blk = 136; continue;
        }
        136 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b86d2); }
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b86d6); };
            c.w64(7, t178);
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008b86dd); }
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008b86e0); }
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b86e3); };
            c.w64(0, t179);
            if !c.st32(c.r64(2).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x70008b86ea); }
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008b86ed); }
            if !c.st32(c.r64(2).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b86f1); }
            if !c.st32(c.r64(2).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008b86f5); }
            let Some(t180) = c.ld64(c.r64(13).wrapping_add(0x10b0)) else { return RegfileExit::fault(0x70008b86f9); };
            c.w64(12, t180);
            let Some(t181) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8700); };
            c.w32(15, t181);
            let r182 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r182);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(6, c.r64(13).wrapping_add(0x63f0));
            let Some(t183) = c.ld64(c.r64(6).wrapping_add(c.r64(15).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b8718); };
            c.w64(15, t183);
            blk = 157; continue;
        }
        138 => {
            let Some(t184) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8721); };
            c.w32(6, t184);
            let r185 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r185);
            let Some(t186) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8728); };
            c.w32(6, t186);
            c.cmp(c.r32(6), 0x5cu64, 32);
            if c.cc_l() { blk = 145; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(6), 0x85u64, 32);
            if c.cc_l() { blk = 143; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(6), 0x85u64, 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 141; continue;
        }
        141 => {
            c.cmp(c.r32(6), 0x9du64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r32(6), 0x9eu64, 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(6), 0x5cu64, 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 144; continue;
        }
        144 => {
            c.cmp(c.r32(6), 0x5eu64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(6), 0x4au64, 32);
            if c.cc_l() { blk = 148; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(6), 0x4au64, 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(6), 0x53u64, 32);
            if c.cc_e() { blk = 154; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(6), 0x48u64, 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(6), 0x49u64, 32);
            if c.cc_e() { blk = 155; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t187) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8774); };
            c.w32(15, t187);
            blk = 157; continue;
        }
        151 => {
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b877a); };
            c.w64(15, t188);
            blk = 157; continue;
        }
        152 => {
            let Some(t189) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8783); };
            c.w32(15, t189);
            blk = 157; continue;
        }
        153 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b8789); };
            c.w64(15, t190);
            blk = 157; continue;
        }
        154 => {
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b8792); };
            c.w64(15, t191);
            blk = 157; continue;
        }
        155 => {
            let Some(t192) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b879b); };
            c.w32(15, t192);
            blk = 157; continue;
        }
        156 => {
            let Some(t193) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b87a1); };
            c.w64(15, t193);
            blk = 157; continue;
        }
        157 => {
            let Some(t194) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008b87a8); };
            c.w64(6, t194);
            c.w64(11, c.r64(6).wrapping_add(0x1c));
            let Some(t195) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008b87af); };
            c.cmp(t195, c.r64(11), 64);
            if c.cc_be() { blk = 197; continue; }
            blk = 158; continue;
        }
        158 => {
            if !c.st64(c.r64(3), c.r64(11)) { return RegfileExit::fault(0x70008b87b8); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 159; continue;
        }
        159 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b87bf); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008b87c3); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008b87c6); }
            if !c.st32(c.r64(1).wrapping_add(0x13), c.r32(0)) { return RegfileExit::fault(0x70008b87c9); }
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(12)) { return RegfileExit::fault(0x70008b87cc); }
            if !c.st32(c.r64(1).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b87d0); }
            if !c.st32(c.r64(1).wrapping_add(0x17), c.r32(15)) { return RegfileExit::fault(0x70008b87d4); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b87d8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b87d9); }
            let Some(t196) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b87da); };
            c.w64(11, t196);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008b87e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b87e3); }
            c.w32(0, 0x2u64);
            let Some(t197) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b87ec); };
            c.w64(7, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b87f0); };
            c.w64(6, t198);
            return c.call(0x70007f2134, 0x70008b87f9, 0x70008b87f4);
        }
        160 => {
            let Some(t199) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b87f9); };
            c.cmp(t199, 0x0u64, 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x1ec8)) else { return RegfileExit::fault(0x70008b87ff); };
            c.w64(3, t200);
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8808); };
            c.w64(6, t201);
            return c.call(0x700086ed74, 0x70008b8811, 0x70008b880c);
        }
        162 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b8811); };
            c.w64(0, t202);
            blk = 163; continue;
        }
        163 => {
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b8815); };
            c.w64(1, t203);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b881c); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            return c.ret(0x10, 0x70008b8823);
        }
        165 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b8826); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b882d); }
            return c.ret(0x0, 0x70008b882f);
        }
        166 => {
            return RegfileExit::fault(0x70008b8830);
        }
        167 => {
            c.w32(1, 0x14cu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b8836); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b8837); }
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b8838); };
            c.w64(3, t204);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b8849, 0x70008b8844);
        }
        168 => {
            return RegfileExit::fault(0x70008b8849);
        }
        169 => {
            c.w32(1, 0xc0u64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b884f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b8850); }
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008b8851); };
            c.w64(3, t205);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008b8862, 0x70008b885d);
        }
        170 => {
            return RegfileExit::fault(0x70008b8862);
        }
        171 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008b8863); }
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b8867); };
            c.w64(3, t206);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b8875, 0x70008b8870);
        }
        172 => {
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8875); };
            c.w64(2, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8879); };
            c.w64(6, t208);
            blk = 1; continue;
        }
        173 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b8882); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008b8883); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b8887); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0x20f8)) else { return RegfileExit::fault(0x70008b888b); };
            c.w64(3, t209);
            return c.call(0x700086ed74, 0x70008b8897, 0x70008b8892);
        }
        174 => {
            let Some(t210) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008b8897); };
            c.cmp(t210, c.r32(0), 32);
            if c.cc_ne() { blk = 201; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b88a4); };
            c.w64(2, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b88a8); };
            c.w64(6, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b88ac); };
            c.w64(7, t213);
            c.w32(0, 0x1u64);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b88b5); };
            c.w64(1, t214);
            blk = 19; continue;
        }
        176 => {
            c.w32(2, 0x4cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008b88c3); }
            return c.call(0x70007febf4, 0x70008b88cc, 0x70008b88c7);
        }
        177 => {
            c.w64(9, c.r64(0));
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b88cf); };
            c.w64(6, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b88d3); };
            c.w64(7, t216);
            c.w32(0, 0x1u64);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b88dc); };
            c.w64(8, t217);
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b88e0); };
            c.w64(1, t218);
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b88e7); };
            c.w64(12, t219);
            blk = 28; continue;
        }
        178 => {
            return RegfileExit::fault(0x70008b88f3);
        }
        179 => {
            return RegfileExit::fault(0x70008b88f4);
        }
        180 => {
            return RegfileExit::fault(0x70008b88f5);
        }
        181 => {
            return RegfileExit::fault(0x70008b88f6);
        }
        182 => {
            return RegfileExit::fault(0x70008b88f7);
        }
        183 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008b88f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008b88fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008b8900); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x70008b890c, 0x70008b8907);
        }
        184 => {
            c.w64(1, c.r64(0));
            let Some(t220) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b890f); };
            c.w64(7, t220);
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8913); };
            c.w64(2, t221);
            c.w32(0, 0x1u64);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b8922); };
            c.w64(8, t222);
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8926); };
            c.w64(9, t223);
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b892a); };
            c.w64(15, t224);
            blk = 40; continue;
        }
        185 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008b8933); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x70008b8935); }
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x2328)) else { return RegfileExit::fault(0x70008b8939); };
            c.w64(3, t225);
            c.w64(6, c.r64(11));
            return c.call(0x700086ed74, 0x70008b8948, 0x70008b8943);
        }
        186 => {
            let Some(t226) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b8948); };
            c.w64(7, t226);
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b894c); };
            c.w64(2, t227);
            c.w32(0, 0x1u64);
            c.w32(11, 0);
            c.logic(0, 32);
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b895b); };
            c.w64(1, t228);
            blk = 54; continue;
        }
        187 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8964); }
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008b8965); };
            c.w64(3, t229);
            c.w32(0, 0x1u64);
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8971); };
            c.w64(6, t230);
            return c.call(0x700086ed74, 0x70008b897a, 0x70008b8975);
        }
        188 => {
            blk = 65; continue;
        }
        189 => {
            c.w32(2, 0x14u64);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008b8984); }
            return c.call(0x70007febf4, 0x70008b898d, 0x70008b8988);
        }
        190 => {
            c.w64(8, c.r64(0));
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8990); };
            c.w64(6, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b8994); };
            c.w64(7, t232);
            c.w32(0, 0x1u64);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b899d); };
            c.w64(1, t233);
            blk = 74; continue;
        }
        191 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b89a6); }
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008b89a7); };
            c.w64(3, t234);
            c.w32(0, 0x1u64);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b89b3); };
            c.w64(6, t235);
            return c.call(0x700086ed74, 0x70008b89bc, 0x70008b89b7);
        }
        192 => {
            blk = 84; continue;
        }
        193 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008b89c1); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008b89cd, 0x70008b89c8);
        }
        194 => {
            c.w64(9, c.r64(0));
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b89d0); };
            c.w64(6, t236);
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b89d4); };
            c.w64(2, t237);
            c.w32(0, 0x1u64);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008b89e0); };
            c.w64(1, t238);
            c.w32(8, 0);
            c.logic(0, 32);
            c.w32(12, 0x8u64);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b89ed); };
            c.w64(11, t239);
            blk = 98; continue;
        }
        195 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008b89fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008b89ff); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008b8a03); }
            return c.call(0x70007febf4, 0x70008b8a0c, 0x70008b8a07);
        }
        196 => {
            c.w64(2, c.r64(0));
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8a0f); };
            c.w64(6, t240);
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8a13); };
            c.w64(11, t241);
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b8a1a); };
            c.w64(9, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b8a1e); };
            c.w64(8, t243);
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b8a22); };
            c.w64(3, t244);
            let Some(t245) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b8a29); };
            c.w64(1, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b8a30); };
            c.w64(15, t246);
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x10b8)) else { return RegfileExit::fault(0x70008b8a34); };
            c.w64(12, t247);
            blk = 136; continue;
        }
        197 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x70008b8a40); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x70008b8a44); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008b8a48); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x70008b8a4c); }
            c.w32(2, 0x1cu64);
            return c.call(0x70007febf4, 0x70008b8a5a, 0x70008b8a55);
        }
        198 => {
            c.w64(1, c.r64(0));
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b8a5d); };
            c.w64(2, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b8a61); };
            c.w64(15, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b8a65); };
            c.w64(9, t250);
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x10b0)) else { return RegfileExit::fault(0x70008b8a69); };
            c.w64(12, t251);
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b8a70); };
            c.w64(0, t252);
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b8a77); };
            c.w64(7, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b8a7e); };
            c.w64(8, t254);
            blk = 159; continue;
        }
        199 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8a87); }
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8a88); };
            c.w64(0, t255);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8a8f); }
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b8a90); };
            c.w64(7, t256);
            c.w32(0, 0x1u64);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8a99); };
            c.w64(6, t257);
            return c.call(0x70007f2134, 0x70008b8aa2, 0x70008b8a9d);
        }
        200 => {
            blk = 160; continue;
        }
        201 => {
            c.w32(1, 0x4cu64);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b8aac); }
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x2138)) else { return RegfileExit::fault(0x70008b8aad); };
            c.w64(3, t258);
            c.w32(0, 0x1u64);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8ab9); };
            c.w64(6, t259);
            return c.call(0x700086ed74, 0x70008b8ac2, 0x70008b8abd);
        }
        202 => {
            let Some(t260) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8ac2); };
            c.w64(0, t260);
            blk = 163; continue;
        }
        203 => {
            return RegfileExit::jump(0x70008b8ad0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b8ad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b8ad4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b8ad8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b8ad9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8ada); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b8adf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b8ae3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 26; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8aed); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8af3); };
            c.cmp(t4, 0x90u64, 16);
            if c.cc_ne() { blk = 23; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b8aff); };
            c.w64(0, t5);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(1, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8b15); };
            c.w32(1, t6);
            let r7 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r7);
            let Some(t8) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8b1b); };
            c.cmp(t8, 0x83du64, 16);
            if c.cc_ne() { blk = 12; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8b23); };
            c.w32(8, t9);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b8b2a); };
            c.w32(1, t10);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x97u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008b8b39);
        }
        9 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b8b3a); };
            c.cmp(t11, c.r32(1), 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008b8b42); };
            c.w64(1, t12);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b8b4c); };
            c.cmp(t13, c.r32(8), 32);
            c.w8(1, (c.cc_ne() as u64));
            c.w32(1, c.r8(1));
            blk = 13; continue;
        }
        11 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        12 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b8b63); };
            c.w64(3, t14);
            return c.call(0x7000870914, 0x70008b8b6f, 0x70008b8b6a);
        }
        15 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b8b6f); };
            if !c.push64(t15) { return RegfileExit::fault(0x70008b8b6f); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8b72); };
            c.w64(1, t16);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b8b79); }
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008b8b7a); };
            if !c.push64(t17) { return RegfileExit::fault(0x70008b8b7a); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8b7d); };
            c.w64(6, t18);
            c.w64(7, c.r64(0));
            c.w32(0, 0x2u64);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008b8b91, 0x70008b8b8c);
        }
        16 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b8b91); };
            c.w64(1, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b8b98); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return c.ret(0x10, 0x70008b8b9f);
        }
        18 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b8ba2); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b8ba9); }
            return c.ret(0x0, 0x70008b8bab);
        }
        19 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8bac); };
            c.w32(1, t20);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xeu64, 64);
            if c.cc_a() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            return RegfileExit::fault(0x70008b8bbb);
        }
        21 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_add(0x3f)) else { return RegfileExit::fault(0x70008b8bbc); };
            c.w32(7, t21);
            let r22 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b8bc2); };
            if !c.push64(t23) { return RegfileExit::fault(0x70008b8bc2); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8bc5); };
            c.w64(1, t24);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b8bcc); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8bcd); }
            c.w32(0, 0x2u64);
            return c.call(0x70007f2274, 0x70008b8bd8, 0x70008b8bd3);
        }
        22 => {
            blk = 16; continue;
        }
        23 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008b8bda); };
            c.w64(2, t25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x257)) else { return RegfileExit::fault(0x70008b8be1); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            return c.call(0x7000824234, 0x70008b8bef, 0x70008b8bea);
        }
        24 => {
            return RegfileExit::fault(0x70008b8bef);
        }
        25 => {
            return RegfileExit::fault(0x70008b8bf0);
        }
        26 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b8bf3); };
            c.w64(3, t28);
            return c.call(0x700086ed74, 0x70008b8bff, 0x70008b8bfa);
        }
        27 => {
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8bff); };
            c.w64(6, t29);
            blk = 1; continue;
        }
        28 => {
            return RegfileExit::jump(0x70008b8c0c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b8c14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b8c14); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b8c18); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b8c19); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8c1a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b8c1f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8c25); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b8c34);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b8c35); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b8c3b); };
            c.w64(1, t5);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b8c42); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return c.ret(0x8, 0x70008b8c49);
        }
        5 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b8c4c); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b8c53); }
            return c.ret(0x0, 0x70008b8c55);
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b8c56); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b8c5a); };
            c.w64(3, t6);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b8c68, 0x70008b8c63);
        }
        7 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8c68); };
            c.w64(6, t7);
            blk = 1; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008b8c70);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b8c74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b8c74); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b8c78); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b8c79); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8c7a); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b8c7f); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8c85); };
            c.w32(2, t2);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b8c94);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b8c95); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008b8c9b); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x21b8)) else { return RegfileExit::fault(0x70008b8c9c); };
            c.w64(3, t5);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008b8cad, 0x70008b8ca8);
        }
        4 => {
            return RegfileExit::fault(0x70008b8cad);
        }
        5 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008b8cae); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b8cb2); };
            c.w64(3, t6);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b8cc0, 0x70008b8cbb);
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8cc0); };
            c.w64(6, t7);
            blk = 1; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008b8cc8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008b8cd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008b8cd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008b8cd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008b8cd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8cda); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x70008b8cdf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008b8ce3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 68; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8ced); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x2u64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return RegfileExit::fault(0x70008b8cfc);
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b8cfd); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8d03); };
            c.w64(1, t5);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008b8d0a); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2134, 0x70008b8d12, 0x70008b8d0d);
        }
        4 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b8d12); };
            c.w64(6, t6);
            let Some(t7) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8d16); };
            c.w32(1, t7);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x3u64, 64);
            if c.cc_a() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x70008b8d25);
        }
        6 => {
            let Some(t8) = c.ld32(c.r64(6).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b8d26); };
            c.w32(1, t8);
            let r9 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r9);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(1));
            return c.call(0x70008ba474, 0x70008b8d37, 0x70008b8d32);
        }
        7 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b8d37); };
            c.w64(6, t10);
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8d3b); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008b8d41); };
            c.w32(1, t13);
            let r14 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r14);
            let Some(t15) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8d47); };
            c.w32(8, t15);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008b8d51); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008b8d55); }
            c.cmp(c.r64(8), 0x79u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008b8d5f);
        }
        9 => {
            let Some(t16) = c.ld32(c.r64(1).wrapping_add(0x1eb)) else { return RegfileExit::fault(0x70008b8d60); };
            c.w32(8, t16);
            c.w32(9, 0x8u64);
            c.w64(11, c.r64(9));
            let r17 = c.add(c.r64(11), 0xcu64, 64);
            c.w64(11, r17);
            if c.cc_o() { blk = 67; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b8d7a); };
            c.w64(12, t18);
            let Some(t19) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b8d81); };
            c.w64(15, t19);
            c.w64(3, c.r64(15).wrapping_add(c.r64(11)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b8d89); };
            c.w64(2, t20);
            let Some(t21) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b8d90); };
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
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x70008b8da6); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 13; continue;
        }
        13 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b8dae); }
            c.w32(8, 0x6u64);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008b8db8); }
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        14 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008b8dc1); }
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
            let Some(t23) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8dd0); };
            c.w32(9, t23);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x70008b8de0);
        }
        18 => {
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x160)) else { return RegfileExit::fault(0x70008b8de1); };
            c.w64(15, t24);
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008b8de8); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return RegfileExit::fault(0x70008b8df2);
        }
        20 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008b8df3); };
            c.w64(15, t25);
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b8dfa); }
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x70008b8e04);
        }
        22 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008b8e05); };
            c.w64(9, t26);
            if !c.st32(c.r64(11).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008b8e09); }
            let Some(t27) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8e0d); };
            c.w32(9, t27);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0xbeu64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008b8e20);
        }
        24 => {
            let Some(t28) = c.ld32(c.r64(1).wrapping_add(0x2ff)) else { return RegfileExit::fault(0x70008b8e21); };
            c.w32(9, t28);
            let Some(t29) = c.ld64(c.r64(13).wrapping_add(0x10d8)) else { return RegfileExit::fault(0x70008b8e28); };
            c.w64(15, t29);
            let Some(t30) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8e2f); };
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
            let Some(t32) = c.ld64(c.r64(7).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x70008b8e45); };
            c.w64(3, t32);
            blk = 45; continue;
        }
        26 => {
            let Some(t33) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8e4e); };
            c.w32(7, t33);
            let r34 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r34);
            let Some(t35) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8e54); };
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
            let Some(t36) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8ea0); };
            c.w32(3, t36);
            blk = 45; continue;
        }
        39 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x6618)) else { return RegfileExit::fault(0x70008b8ea5); };
            c.w64(3, t37);
            blk = 45; continue;
        }
        40 => {
            let Some(t38) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8eae); };
            c.w32(3, t38);
            blk = 45; continue;
        }
        41 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x6698)) else { return RegfileExit::fault(0x70008b8eb3); };
            c.w64(3, t39);
            blk = 45; continue;
        }
        42 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x6708)) else { return RegfileExit::fault(0x70008b8ebc); };
            c.w64(3, t40);
            blk = 45; continue;
        }
        43 => {
            let Some(t41) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b8ec5); };
            c.w32(3, t41);
            blk = 45; continue;
        }
        44 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x66a8)) else { return RegfileExit::fault(0x70008b8eca); };
            c.w64(3, t42);
            blk = 45; continue;
        }
        45 => {
            let Some(t43) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008b8ed1); };
            c.w64(7, t43);
            c.w64(6, c.r64(7).wrapping_add(0x1c));
            let Some(t44) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008b8ed9); };
            c.cmp(t44, c.r64(6), 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st64(c.r64(12), c.r64(6)) { return RegfileExit::fault(0x70008b8ee2); }
            c.w64(12, c.r64(7).wrapping_add(0x1));
            blk = 47; continue;
        }
        47 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008b8eea); }
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
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008b8efb); }
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008b8f00); };
            c.w64(9, t45);
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008b8f07); }
            if !c.st32(c.r64(12).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008b8f0c); }
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xf90)) else { return RegfileExit::fault(0x70008b8f11); };
            c.w64(9, t46);
            if !c.st32(c.r64(12).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x70008b8f18); }
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008b8f1d); }
            if !c.st32(c.r64(12).wrapping_add(0xf), c.r32(11)) { return RegfileExit::fault(0x70008b8f22); }
            if !c.st32(c.r64(12).wrapping_add(0x17), c.r32(3)) { return RegfileExit::fault(0x70008b8f27); }
            c.w32(2, 0);
            c.logic(0, 32);
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t47) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008b8f33); };
            c.w32(9, t47);
            let r48 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r48);
            let Some(t49) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008b8f3a); };
            c.cmp(t49, 0x83du64, 16);
            if c.cc_ne() { blk = 57; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8f43); };
            c.w32(11, t50);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t51) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008b8f4a); };
            c.w32(9, t51);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x97u64, 64);
            if c.cc_a() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            return RegfileExit::fault(0x70008b8f5a);
        }
        54 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0x263)) else { return RegfileExit::fault(0x70008b8f5b); };
            c.cmp(t52, c.r32(9), 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1010)) else { return RegfileExit::fault(0x70008b8f64); };
            c.w64(9, t53);
            let Some(t54) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008b8f6b); };
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
            let Some(t55) = c.ld64(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008b8f7e); };
            c.w64(3, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b8f85); };
            c.w64(6, t56);
            return c.call(0x7000870914, 0x70008b8f8e, 0x70008b8f89);
        }
        59 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b8f8e); };
            if !c.push64(t57) { return RegfileExit::fault(0x70008b8f8e); }
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b8f91); };
            if !c.push64(t58) { return RegfileExit::fault(0x70008b8f91); }
            c.w64(7, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b8f9c); };
            c.w64(6, t59);
            return c.call(0x70007f2274, 0x70008b8fa5, 0x70008b8fa0);
        }
        60 => {
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008b8fa5); };
            c.w64(1, t60);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008b8fac); }
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_g() { blk = 62; continue; }
            blk = 61; continue;
        }
        61 => {
            return c.ret(0x10, 0x70008b8fb3);
        }
        62 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008b8fb6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008b8fbd); }
            return c.ret(0x0, 0x70008b8fbf);
        }
        63 => {
            c.w64(6, c.r64(1));
            let Some(t61) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008b8fc3); };
            c.w32(1, t61);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xeu64, 64);
            if c.cc_a() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            return RegfileExit::fault(0x70008b8fd2);
        }
        65 => {
            let Some(t62) = c.ld32(c.r64(6).wrapping_add(0x3f)) else { return RegfileExit::fault(0x70008b8fd3); };
            c.w32(7, t62);
            let r63 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r63);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008b8fd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008b8fdb); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x70008b8fe6, 0x70008b8fe1);
        }
        66 => {
            blk = 60; continue;
        }
        67 => {
            return RegfileExit::fault(0x70008b8fe8);
        }
        68 => {
            let Some(t64) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008b8fe9); };
            c.w64(3, t64);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008b8ff7, 0x70008b8ff2);
        }
        69 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b8ff7); };
            c.w64(6, t65);
            blk = 1; continue;
        }
        70 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008b9000); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febb4, 0x70008b900c, 0x70008b9007);
        }
        71 => {
            c.w64(11, c.r64(0));
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b900f); };
            c.w64(0, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b9013); };
            c.w64(1, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008b9017); };
            c.w64(6, t68);
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008b901b); };
            c.w64(12, t69);
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008b9022); };
            c.w64(2, t70);
            c.w32(9, 0x8u64);
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008b902f); };
            c.w64(8, t71);
            blk = 13; continue;
        }
        72 => {
            c.w32(2, 0x1cu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008b903d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008b9041); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008b9045); }
            return c.call(0x70007febf4, 0x70008b904e, 0x70008b9049);
        }
        73 => {
            c.w64(12, c.r64(0));
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008b9051); };
            c.w64(0, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008b9055); };
            c.w64(1, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008b9059); };
            c.w64(3, t74);
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008b905d); };
            c.w64(11, t75);
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x10d8)) else { return RegfileExit::fault(0x70008b9061); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008b9068); };
            c.w64(9, t77);
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        74 => {
            return RegfileExit::jump(0x70008b9078);
        }
            _ => unreachable!(),
        }
    }
}
