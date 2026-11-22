#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000836434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000836434); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000836438); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700083643e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000836442); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x7000836446); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700083644a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700083644e); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836457); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000836460); };
            c.w64(8, t3);
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836467); };
            c.logic(t4 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000836475); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_e() { blk = 646; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(9, c.r64(0).wrapping_add(c.r64(0)));
            c.w32(11, c.r32(8));
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000836489); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let r8 = c.r32(11) | 0x2u64;
            c.w32(11, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(9)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083649e); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            blk = 7; continue;
        }
        7 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 319; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(9, c.r64(2));
            c.w64(11, c.r64(8));
            blk = 9; continue;
        }
        9 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x70008364bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x70008364c0); }
            c.w64(12, c.r64(11));
            let r11 = c.r64(12) & 0xfu64;
            c.w64(12, r11);
            c.logic(r11, 64);
            if c.cc_e() { blk = 99; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(12), 0x1u64, 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(12), 0x4u64, 64);
            if c.cc_e() { blk = 17; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(12), 0x5u64, 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(12), 0xdu64, 64);
            if c.cc_ne() { blk = 404; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008364f1); };
            c.w64(0, t12);
            blk = 15; continue;
        }
        15 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008364fb); }
            return c.ret(0x0, 0x70008364fc);
        }
        16 => {
            c.w64(0, c.r64(9));
            blk = 15; continue;
        }
        17 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_ne() { blk = 31; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.shr(c.r64(8), 0x9u64, 64) { c.w64(8, sr); }
            c.logic(c.r16(11) & 0x100u64, 16);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let r13 = c.r64(8) & 0x7ffu64;
            c.w64(8, r13);
            c.logic(r13, 64);
            c.logic(c.r8(11) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t14) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083652e); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083653b); };
            c.w64(9, t16);
            blk = 21; continue;
        }
        21 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836542); };
            c.w32(8, t17);
            let r18 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r18);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        23 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t19) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083655f); };
            c.w32(9, t19);
            let Some(t20) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000836563); };
            c.cmp(t20, c.r32(9), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 25; continue;
        }
        25 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000836570); }
            blk = 257; continue;
        }
        26 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836581); };
            c.w32(0, t21);
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            blk = 15; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083658e); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008365a0); };
            c.w32(9, t25);
            let Some(t26) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008365a4); };
            c.cmp(t26, c.r32(9), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008365b1); }
            blk = 257; continue;
        }
        31 => {
            c.w64(8, c.r64(11));
            let r27 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r27);
            let r28 = c.r32(8) & 0xfu64;
            c.w32(8, r28);
            c.logic(r28, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 40; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 37; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 571; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 556; continue;
        }
        37 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 556; continue;
        }
        40 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 50; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 556; continue;
        }
        45 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 556; continue;
        }
        48 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r29 = c.r64(11) & 0xfffffu64;
            c.w64(11, r29);
            c.logic(r29, 64);
            let Some(t30) = c.ld8(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836650); };
            c.w32(8, (t30 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        49 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r31 = c.r64(11) & 0xfffffu64;
            c.w64(11, r31);
            c.logic(r31, 64);
            let Some(t32) = c.ld16(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083666a); };
            c.w32(8, (t32 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        50 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r33 = c.r64(11) & 0xfffffu64;
            c.w64(11, r33);
            c.logic(r33, 64);
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836684); };
            c.w32(8, t34);
            c.w64(9, c.r64(8));
            let r35 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r35);
            if c.cc_o() { blk = 557; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        52 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r36 = c.r64(11) & 0xfffffu64;
            c.w64(11, r36);
            c.logic(r36, 64);
            let Some(t37) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008366a8); };
            c.w32(8, t37);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 562; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        54 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r38 = c.r64(11) & 0xfffffu64;
            c.w64(11, r38);
            c.logic(r38, 64);
            let Some(t39) = c.ld64(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008366ce); };
            c.w64(8, t39);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008366d9); };
            c.w64(9, t40);
            let Some(t41) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008366e0); };
            c.w64(11, t41);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008366e7); };
            c.w64(15, t42);
            let Some(t43) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008366ee); };
            c.cmp(t43, c.r64(12), 64);
            if c.cc_be() { blk = 567; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008366f7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 57; continue;
        }
        57 => {
            let Some(t44) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008366fe); };
            c.w64(11, t44);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000836705); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700083670f); }
            let r45 = c.neg(c.r64(8), 64);
            c.w64(8, r45);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700083671a); }
            c.w64(0, c.r64(9));
            blk = 15; continue;
        }
        59 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x7000836726); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700083672e); }
            c.w64(0, c.r64(9));
            blk = 15; continue;
        }
        60 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083673a); };
            c.w64(8, t46);
            let Some(t47) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000836741); };
            c.w64(9, t47);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000836748); };
            c.w64(11, t48);
            let Some(t49) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700083674f); };
            c.cmp(t49, c.r64(12), 64);
            if c.cc_be() { blk = 569; continue; }
            blk = 61; continue;
        }
        61 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000836758); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 62; continue;
        }
        62 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700083675f); };
            c.w64(9, t50);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000836766); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700083676a); }
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        63 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r51 = c.r64(11) & 0xfffffu64;
            c.w64(11, r51);
            c.logic(r51, 64);
            if !c.movss_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000836785); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 257; continue;
        }
        64 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r52 = c.r64(11) & 0xfffffu64;
            c.w64(11, r52);
            c.logic(r52, 64);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008367a0); }
            blk = 257; continue;
        }
        65 => {
            if let Some(sr) = c.shr(c.r64(11), 0xbu64, 64) { c.w64(11, sr); }
            let r53 = c.r64(11) & 0xfffffu64;
            c.w64(11, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008367b7); };
            c.w32(0, t54);
            let r55 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r55);
            blk = 15; continue;
        }
        66 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t56) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008367c9); };
            c.w32(8, t56);
            let r57 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r57);
            let Some(t58) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008367d0); };
            c.cmp(t58, 0x42u64, 16);
            if c.cc_ne() { blk = 278; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008367dc); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 278; continue; }
            blk = 69; continue;
        }
        69 => {
            if c.cc_ne() { blk = 278; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r59 = c.add(c.r64(12), c.r64(8), 64);
            c.w64(12, r59);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 278; continue;
        }
        72 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 73; continue;
        }
        73 => {
            let Some(t60) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836826); };
            c.w32(12, t60);
            c.cmp(c.r64(12), c.r64(8), 64);
            if c.cc_be() { blk = 605; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t61) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836833); };
            c.w32(11, t61);
            let r62 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r62);
            let Some(t63) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083683a); };
            c.w32(11, t63);
            c.logic(c.r8(11) & 0x7u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(15, c.r64(9));
            c.w64(0, c.r64(11));
            c.w32(11, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(11));
            blk = 76; continue;
        }
        76 => {
            c.w64(3, c.r64(0));
            let r64 = c.r32(3) & 0x7u64;
            c.w32(3, r64);
            c.logic(r64, 32);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 574; continue;
        }
        81 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 574; continue;
        }
        84 => {
            let Some(t65) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000836887); };
            c.w32(0, t65);
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700083688b); };
            c.cmp(t66, c.r32(0), 32);
            if c.cc_ne() { blk = 575; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t67) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836898); };
            c.w32(15, t67);
            let r68 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r68);
            let Some(t69) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083689f); };
            c.w32(0, t69);
            let r70 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r70);
            let Some(t71) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008368a6); };
            c.w32(0, t71);
            blk = 76; continue;
        }
        86 => {
            let Some(t72) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008368ac); };
            c.w32(0, t72);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r73 = c.add(c.r64(11), c.r64(0), 64);
            c.w64(11, r73);
            let Some(t74) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008368b9); };
            c.w32(15, t74);
            let r75 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r75);
            let Some(t76) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008368c0); };
            c.w32(0, t76);
            let r77 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r77);
            let Some(t78) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008368c7); };
            c.w32(0, t78);
            blk = 76; continue;
        }
        87 => {
            let Some(t79) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008368cd); };
            c.w32(15, t79);
            let r80 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r80);
            let Some(t81) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008368d4); };
            c.w32(0, t81);
            let r82 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r82);
            let Some(t83) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008368db); };
            c.w32(0, t83);
            blk = 76; continue;
        }
        88 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 91; continue;
        }
        89 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            c.w32(3, 0x1u64);
            blk = 91; continue;
        }
        90 => {
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(9));
            c.w32(3, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(12));
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 576; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(9, c.r64(0).wrapping_add(0xb));
            blk = 93; continue;
        }
        93 => {
            let r84 = c.add(c.r64(8), c.r64(15), 64);
            c.w64(8, r84);
            let r85 = c.r32(11) & 0x8u64;
            c.w32(11, r85);
            c.logic(r85, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t86) = c.ld16(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x7000836938); };
            c.w32(8, t86);
            blk = 96; continue;
        }
        95 => {
            let Some(t87) = c.ld8(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700083693f); };
            c.w32(8, t87);
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(8), 0xffu64, 32);
            if c.cc_g() { blk = 591; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(9, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x7000836959); };
            c.w64(11, t88);
            let Some(t89) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836960); };
            c.w32(0, t89);
            let r90 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000836968); };
            c.cmp(t91, c.r32(0), 32);
            if c.cc_e() { blk = 596; continue; }
            blk = 98; continue;
        }
        98 => {
            blk = 377; continue;
        }
        99 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t92) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836983); };
            c.w32(8, t92);
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            let Some(t94) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083698a); };
            c.w32(8, t94);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 104; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000836996); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(12, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(12) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 104; continue; }
            blk = 102; continue;
        }
        102 => {
            if c.cc_ne() { blk = 104; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t95) = c.ld64(0x7000836801u64) else { return RegfileExit::fault(0x70008369b1); };
            c.w64(15, t95);
            let r96 = c.add(c.r64(15), c.r64(12), 64);
            c.w64(15, r96);
            let Some(t97) = c.ld64(0x700083680eu64) else { return RegfileExit::fault(0x70008369bb); };
            c.w64(7, t97);
            c.cmp(c.r64(15), c.r64(7), 64);
            if c.cc_be() { blk = 109; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 278; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w64(10, c.r64(4));
            let r98 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r98);
            let r99 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r99);
            c.logic(r99, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008369dd); }
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x1478)) else { return RegfileExit::fault(0x70008369e2); };
            c.w64(0, t100);
            c.w64(10, 0x70008369f0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008369f0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008369f4); }
            return c.call(c.r64(0), 0x70008369fa, 0x70008369f8);
        }
        106 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008369fa); }
            let Some(t101) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000836a02); };
            c.w64(4, t101);
            c.cmp(c.r32(0), 0xffffffffffffffffu64, 32);
            if c.cc_e() { blk = 608; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(12, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000836a13); };
            c.w64(11, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000836a17); };
            c.w64(2, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836a1b); };
            c.w64(1, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000836a1f); };
            c.w64(0, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836a23); };
            c.w64(3, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000836a27); };
            c.w64(6, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000836a2b); };
            c.w64(9, t108);
            blk = 109; continue;
        }
        108 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(12, c.r64(8));
            blk = 109; continue;
        }
        109 => {
            c.w64(8, c.r64(11));
            if let Some(sr) = c.shr(c.r64(8), 0xau64, 64) { c.w64(8, sr); }
            let r109 = c.r64(8) & 0xffu64;
            c.w64(8, r109);
            c.logic(r109, 64);
            c.w32(15, c.r32(8));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000836a4b); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000836a4f); }
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_g() { blk = 139; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t110) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836a5d); };
            c.w32(8, t110);
            let r111 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r111);
            c.logic(c.r16(11) & 0x100u64, 16);
            if c.cc_ne() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t112) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000836a6c); };
            c.w32(7, t112);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 113; continue;
        }
        112 => {
            let Some(t113) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836a78); };
            c.w32(7, t113);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_ae() { blk = 615; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(15), 0x6u64, 32);
            if c.cc_l() { blk = 124; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_l() { blk = 120; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(15), 0x9u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(15), 0xau64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(15), 0xbu64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 609; continue;
        }
        120 => {
            c.cmp(c.r32(15), 0x6u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(15), 0x7u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(15), 0x8u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 123; continue;
        }
        123 => {
            blk = 609; continue;
        }
        124 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_l() { blk = 129; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_e() { blk = 136; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 137; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 609; continue;
        }
        129 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            blk = 609; continue;
        }
        133 => {
            let Some(t114) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836b05); };
            c.w32(0, t114);
            let r115 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r115);
            blk = 15; continue;
        }
        134 => {
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836b12); };
            c.w32(8, t116);
            let r117 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r117);
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000836b1a); };
            c.cmp(t118, c.r32(8), 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        136 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000836b2f); }
            blk = 257; continue;
        }
        137 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t119) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836b45); };
            c.cmp(t119, c.r64(9), 64);
            if c.cc_e() { blk = 157; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000836b50); }
            blk = 257; continue;
        }
        139 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_ge() { blk = 191; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(8), 0x11u64, 32);
            if c.cc_ge() { blk = 160; continue; }
            blk = 141; continue;
        }
        141 => {
            c.w32(8, c.r32(11));
            let r120 = c.r32(8) & 0x3fc00u64;
            c.w32(8, r120);
            c.logic(r120, 32);
            c.cmp(c.r32(8), 0x3000u64, 32);
            if c.cc_ne() { blk = 609; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 615; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t121) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836b96); };
            c.w32(8, t121);
            let r122 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r122);
            let Some(t123) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000836b9d); };
            c.w32(9, t123);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r124 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r124);
            c.w32(15, c.r32(12));
            c.w64(10, c.r64(4));
            let r125 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r125);
            let r126 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r126);
            c.logic(r126, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000836bb9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000836bbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000836bc2); }
            c.w64(2, c.r64(15));
            c.w64(1, c.r64(13));
            c.w64(7, c.r64(1));
            let Some(t127) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x7000836bcf); };
            c.w64(0, t127);
            c.w64(10, 0x7000836bddu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000836bdd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000836be1); }
            return c.call(c.r64(0), 0x7000836be7, 0x7000836be5);
        }
        144 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000836be7); }
            let Some(t128) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000836bef); };
            c.w64(4, t128);
            c.w32(8, c.r32(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000836bf7); };
            let r130 = c.r64(8) & t129;
            c.w64(8, r130);
            c.logic(r130, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000836bfe); };
            c.w64(11, t131);
            blk = 146; continue;
        }
        145 => {
            let r132 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r132);
            let r133 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r133);
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000836c0b); };
            c.w64(12, t134);
            let r135 = c.r64(12) & c.r64(8);
            c.w64(12, r135);
            c.logic(r135, 64);
            c.w64(8, c.r64(12));
            blk = 146; continue;
        }
        146 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            c.w64(15, c.r64(12).wrapping_mul(4));
            let Some(t136) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000836c22); };
            c.w32(0, t136);
            let r137 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r137);
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000836c2a); };
            c.cmp(t138, c.r32(0), 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 147; continue;
        }
        147 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000836c37); };
            c.cmp(t139, c.r32(0), 32);
            if c.cc_e() { blk = 145; continue; }
            blk = 149; continue;
        }
        149 => {
            c.xorpd_rr(0, 0);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000836c44); };
            c.cvtsi2sd(0, (t140 as i64));
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000836c4a); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 145; continue; }
            blk = 150; continue;
        }
        150 => {
            if c.cc_e() { blk = 153; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 145; continue;
        }
        152 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000836c5f); };
            c.cmp(c.r64(0), t141, 64);
            if c.cc_ne() { blk = 145; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t142) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000836c65); };
            c.w32(8, t142);
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_ne() { blk = 611; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t143) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000836c74); };
            c.w32(8, t143);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 612; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t144) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836c8c); };
            c.w32(0, t144);
            let r145 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r145);
            blk = 15; continue;
        }
        156 => {
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000836c99); };
            c.w64(2, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836c9d); };
            c.w64(1, t147);
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000836ca1); };
            c.w64(0, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836ca5); };
            c.w64(3, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000836ca9); };
            c.w64(6, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000836cad); };
            c.w64(11, t151);
            blk = 157; continue;
        }
        157 => {
            c.logic(c.r16(11) & 0x200u64, 16);
            if c.cc_e() { blk = 278; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000836cbd); };
            c.w64(8, t152);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836cc7); };
            c.cmp(t153, c.r32(9), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000836cd1); };
            c.w64(0, t154);
            blk = 15; continue;
        }
        160 => {
            let Some(t155) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836cdd); };
            c.w32(8, t155);
            let r156 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r156);
            let Some(t157) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000836ce4); };
            c.logic(t157 & 0x4u64, 8);
            if c.cc_ne() { blk = 278; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t158) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000836cef); };
            c.cmp(t158, c.r64(12), 64);
            if c.cc_be() { blk = 615; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t159) = c.ld64(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000836cf9); };
            c.w64(8, t159);
            let Some(t160) = c.ld32(c.r64(9).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000836cfd); };
            c.w32(9, t160);
            c.cmp(c.r32(15), 0x16u64, 32);
            if c.cc_l() { blk = 172; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(15), 0x19u64, 32);
            if c.cc_l() { blk = 168; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(15), 0x19u64, 32);
            if c.cc_e() { blk = 181; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(15), 0x1au64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(15), 0x1bu64, 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 167; continue;
        }
        167 => {
            blk = 278; continue;
        }
        168 => {
            c.cmp(c.r32(15), 0x16u64, 32);
            if c.cc_e() { blk = 186; continue; }
            blk = 169; continue;
        }
        169 => {
            c.cmp(c.r32(15), 0x17u64, 32);
            if c.cc_e() { blk = 187; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(15), 0x18u64, 32);
            if c.cc_e() { blk = 188; continue; }
            blk = 171; continue;
        }
        171 => {
            blk = 278; continue;
        }
        172 => {
            c.cmp(c.r32(15), 0x13u64, 32);
            if c.cc_l() { blk = 177; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(15), 0x13u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(15), 0x14u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 175; continue;
        }
        175 => {
            c.cmp(c.r32(15), 0x15u64, 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 176; continue;
        }
        176 => {
            blk = 278; continue;
        }
        177 => {
            c.cmp(c.r32(15), 0x11u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(15), 0x12u64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 179; continue;
        }
        179 => {
            blk = 278; continue;
        }
        180 => {
            let r161 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r161);
            blk = 240; continue;
        }
        181 => {
            let r162 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r162);
            blk = 240; continue;
        }
        182 => {
            let r163 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r163);
            blk = 242; continue;
        }
        183 => {
            let r164 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r164);
            blk = 244; continue;
        }
        184 => {
            let r165 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r165);
            blk = 246; continue;
        }
        185 => {
            let r166 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r166);
            blk = 248; continue;
        }
        186 => {
            let r167 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r167);
            blk = 251; continue;
        }
        187 => {
            let r168 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r168);
            blk = 254; continue;
        }
        188 => {
            let r169 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r169);
            blk = 256; continue;
        }
        189 => {
            let r170 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r170);
            blk = 261; continue;
        }
        190 => {
            let r171 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r171);
            blk = 271; continue;
        }
        191 => {
            let Some(t172) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836dd9); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            let Some(t174) = c.ld32(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000836de0); };
            c.w32(7, t174);
            c.logic(c.r8(7) & 0x10u64, 8);
            if c.cc_ne() { blk = 217; continue; }
            blk = 192; continue;
        }
        192 => {
            c.logic(c.r8(7) & 0x4u64, 8);
            if c.cc_ne() { blk = 278; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t175) = c.ld64(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000836df8); };
            c.w64(8, t175);
            let Some(t176) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000836dfc); };
            c.w64(7, t176);
            let Some(t177) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000836e00); };
            c.logic(t177 & 0x1u64, 8);
            if c.cc_ne() { blk = 196; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t178) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000836e07); };
            c.w64(11, t178);
            let r179 = c.add(c.r64(11), c.r64(7), 64);
            c.w64(11, r179);
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_a() { blk = 278; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t180) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000836e17); };
            c.w64(8, t180);
            blk = 219; continue;
        }
        196 => {
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_a() { blk = 278; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t181) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000836e29); };
            c.w32(11, t181);
            let r182 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r182);
            let Some(t183) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000836e30); };
            c.w32(11, t183);
            let r184 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r184);
            let r185 = c.r32(11) & 0x3fu64;
            c.w32(11, r185);
            c.logic(r185, 32);
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_l() { blk = 205; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_l() { blk = 202; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(11), 0x25u64, 32);
            if c.cc_e() { blk = 215; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(11), 0x26u64, 32);
            if c.cc_e() { blk = 215; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_e() { blk = 214; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r32(11), 0x22u64, 32);
            if c.cc_e() { blk = 214; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r32(11), 0x23u64, 32);
            if c.cc_e() { blk = 215; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_l() { blk = 209; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(11), 0x1fu64, 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_e() { blk = 214; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(11), 0x1cu64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(11), 0x1du64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            return RegfileExit::fault(0x7000836e95);
        }
        212 => {
            c.w32(11, 0x1u64);
            blk = 216; continue;
        }
        213 => {
            c.w32(11, 0x2u64);
            blk = 216; continue;
        }
        214 => {
            c.w32(11, 0x4u64);
            blk = 216; continue;
        }
        215 => {
            c.w32(11, 0x8u64);
            blk = 216; continue;
        }
        216 => {
            let r186 = c.sub(c.r64(8), c.r64(7), 64);
            c.w64(8, r186);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(8, c.r64(10));
            c.w64(7, c.r64(2));
            c.cqo();
            c.idiv(c.r64(11), 64);
            c.w64(10, c.r64(8));
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(10));
            c.w64(2, c.r64(7));
            blk = 219; continue;
        }
        217 => {
            c.w64(10, c.r64(4));
            let r187 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r187);
            let r188 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r188);
            c.logic(r188, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000836ee1); }
            c.w64(2, c.r64(9));
            c.w64(1, c.r64(13));
            c.w64(8, c.r64(1));
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000836eef); };
            c.w64(0, t189);
            c.w64(10, 0x7000836efdu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000836efd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000836f01); }
            return c.call(c.r64(0), 0x7000836f07, 0x7000836f05);
        }
        218 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000836f07); }
            let Some(t190) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000836f0f); };
            c.w64(4, t190);
            c.w64(8, c.r64(0));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000836f17); };
            c.w64(2, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000836f1b); };
            c.w64(1, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000836f1f); };
            c.w64(0, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000836f23); };
            c.w64(3, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000836f27); };
            c.w64(6, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000836f2b); };
            c.w64(12, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000836f2f); };
            c.w64(15, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000836f33); };
            c.w64(9, t198);
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r64(12), c.r64(8), 64);
            if c.cc_ae() { blk = 614; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t199) = c.ld64(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000836f40); };
            c.w64(8, t199);
            let Some(t200) = c.ld32(c.r64(9).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000836f44); };
            c.w32(9, t200);
            c.w64(11, c.r64(15));
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_l() { blk = 230; continue; }
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_l() { blk = 226; continue; }
            blk = 222; continue;
        }
        222 => {
            c.cmp(c.r32(11), 0x24u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r32(11), 0x25u64, 32);
            if c.cc_e() { blk = 270; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r32(11), 0x26u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 225; continue;
        }
        225 => {
            blk = 278; continue;
        }
        226 => {
            c.cmp(c.r32(11), 0x21u64, 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(11), 0x22u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(11), 0x23u64, 32);
            if c.cc_e() { blk = 255; continue; }
            blk = 229; continue;
        }
        229 => {
            blk = 278; continue;
        }
        230 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_l() { blk = 235; continue; }
            blk = 231; continue;
        }
        231 => {
            c.cmp(c.r32(11), 0x1eu64, 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(11), 0x1fu64, 32);
            if c.cc_e() { blk = 245; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_e() { blk = 247; continue; }
            blk = 234; continue;
        }
        234 => {
            blk = 278; continue;
        }
        235 => {
            c.cmp(c.r32(11), 0x1cu64, 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 236; continue;
        }
        236 => {
            c.cmp(c.r32(11), 0x1du64, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 237; continue;
        }
        237 => {
            blk = 278; continue;
        }
        238 => {
            let r201 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r201);
            blk = 240; continue;
        }
        239 => {
            let r202 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r202);
            blk = 240; continue;
        }
        240 => {
            let Some(t203) = c.ld8(c.r64(8).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000836fd3); };
            c.w32(8, t203);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        241 => {
            let r204 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r204);
            blk = 242; continue;
        }
        242 => {
            let Some(t205) = c.ld8(c.r64(8).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000836fe4); };
            c.w32(8, (t205 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        243 => {
            let r206 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r206);
            blk = 244; continue;
        }
        244 => {
            let Some(t207) = c.ld16(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(2))) else { return RegfileExit::fault(0x7000836ff5); };
            c.w32(8, t207);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        245 => {
            let r208 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r208);
            blk = 246; continue;
        }
        246 => {
            let Some(t209) = c.ld16(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(2))) else { return RegfileExit::fault(0x7000837006); };
            c.w32(8, (t209 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        247 => {
            let r210 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r210);
            blk = 248; continue;
        }
        248 => {
            let Some(t211) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x7000837017); };
            c.w32(8, t211);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 626; continue; }
            blk = 249; continue;
        }
        249 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        250 => {
            let r212 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r212);
            blk = 251; continue;
        }
        251 => {
            let Some(t213) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4))) else { return RegfileExit::fault(0x7000837034); };
            c.w32(8, t213);
            c.w64(9, c.r64(8));
            let r214 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r214);
            if c.cc_o() { blk = 631; continue; }
            blk = 252; continue;
        }
        252 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 15; continue;
        }
        253 => {
            let r215 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r215);
            blk = 254; continue;
        }
        254 => {
            if !c.movss_rm(0, c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4))) { return RegfileExit::fault(0x700083704f); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 257; continue;
        }
        255 => {
            let r216 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r216);
            blk = 256; continue;
        }
        256 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8))) { return RegfileExit::fault(0x700083705e); }
            blk = 257; continue;
        }
        257 => {
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000837064); };
            c.w64(8, t217);
            let Some(t218) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700083706b); };
            c.w64(9, t218);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000837072); };
            c.w64(12, t219);
            let Some(t220) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000837079); };
            c.cmp(t220, c.r64(11), 64);
            if c.cc_be() { blk = 636; continue; }
            blk = 258; continue;
        }
        258 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000837083); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 259; continue;
        }
        259 => {
            let Some(t221) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083708a); };
            c.w64(9, t221);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000837091); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000837095); }
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        260 => {
            let r222 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r222);
            blk = 261; continue;
        }
        261 => {
            let Some(t223) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8))) else { return RegfileExit::fault(0x70008370a6); };
            c.w64(8, t223);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 267; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008370b0); };
            c.w64(9, t224);
            let Some(t225) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008370b7); };
            c.w64(11, t225);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008370be); };
            c.w64(15, t226);
            let Some(t227) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008370c5); };
            c.cmp(t227, c.r64(12), 64);
            if c.cc_be() { blk = 638; continue; }
            blk = 263; continue;
        }
        263 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008370ce); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 264; continue;
        }
        264 => {
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008370d5); };
            c.w64(11, t228);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008370dc); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 266; continue; }
            blk = 265; continue;
        }
        265 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008370e6); }
            let r229 = c.neg(c.r64(8), 64);
            c.w64(8, r229);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008370f1); }
            c.w64(0, c.r64(9));
            blk = 15; continue;
        }
        266 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008370fd); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000837105); }
            c.w64(0, c.r64(9));
            blk = 15; continue;
        }
        267 => {
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000837111); };
            c.w64(8, t230);
            let Some(t231) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000837118); };
            c.w64(9, t231);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083711f); };
            c.w64(11, t232);
            let Some(t233) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000837126); };
            c.cmp(t233, c.r64(12), 64);
            if c.cc_be() { blk = 640; continue; }
            blk = 268; continue;
        }
        268 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700083712f); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 269; continue;
        }
        269 => {
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000837136); };
            c.w64(9, t234);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700083713d); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x7000837141); }
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        270 => {
            let r235 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r235);
            blk = 271; continue;
        }
        271 => {
            let Some(t236) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(8))) else { return RegfileExit::fault(0x7000837154); };
            c.w64(8, t236);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 275; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083715e); };
            c.w64(9, t237);
            let Some(t238) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000837165); };
            c.w64(11, t238);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083716c); };
            c.w64(15, t239);
            let Some(t240) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000837173); };
            c.cmp(t240, c.r64(12), 64);
            if c.cc_be() { blk = 642; continue; }
            blk = 273; continue;
        }
        273 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083717c); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 274; continue;
        }
        274 => {
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000837183); };
            c.w64(11, t241);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700083718a); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700083718e); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x7000837196); }
            c.w64(0, c.r64(9));
            blk = 15; continue;
        }
        275 => {
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008371a2); };
            c.w64(8, t242);
            let Some(t243) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008371a9); };
            c.w64(9, t243);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008371b0); };
            c.w64(11, t244);
            let Some(t245) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008371b7); };
            c.cmp(t245, c.r64(12), 64);
            if c.cc_be() { blk = 644; continue; }
            blk = 276; continue;
        }
        276 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x70008371c0); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 277; continue;
        }
        277 => {
            let Some(t246) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008371c7); };
            c.w64(9, t246);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008371ce); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008371d2); }
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        278 => {
            let Some(t247) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008371e2); };
            c.w64(5, t247);
            let r248 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r248);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008371ea); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008371eb); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008371ec); }
            let Some(t249) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008371ed); };
            if !c.push64(t249) { return RegfileExit::fault(0x70008371ed); }
            c.w32(0, 0x4u64);
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x2018)) else { return RegfileExit::fault(0x70008371f6); };
            c.w64(3, t250);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x70008371fd); }
            return RegfileExit::jump(0x700086ed74);
        }
        279 => {
            c.w32(9, c.r32(12));
            let r251 = c.r32(9) & 0x3u64;
            c.w32(9, r251);
            c.logic(r251, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700083720e); }
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t252) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837218); };
            c.w32(9, t252);
            let Some(t253) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x700083721d); };
            c.cmp(t253, c.r32(9), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 281; continue;
        }
        281 => {
            let Some(t254) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700083722a); };
            c.cmp(t254, c.r32(12), 32);
            if c.cc_e() { blk = 646; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t255) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x7000837237); };
            c.cmp(t255, c.r32(12), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 283; continue;
        }
        283 => {
            c.cmp(c.r32(12), c.r32(1), 32);
            if c.cc_e() { blk = 311; continue; }
            blk = 284; continue;
        }
        284 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t256) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083724e); };
            c.w32(8, t256);
            let r257 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r257);
            let Some(t258) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837255); };
            c.w32(8, t258);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 289; continue; }
            blk = 286; continue;
        }
        286 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000837261); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 289; continue; }
            blk = 287; continue;
        }
        287 => {
            if c.cc_ne() { blk = 289; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t259) = c.ld64(0x7000836801u64) else { return RegfileExit::fault(0x700083727c); };
            c.w64(11, t259);
            let r260 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r260);
            let Some(t261) = c.ld64(0x700083680eu64) else { return RegfileExit::fault(0x7000837286); };
            c.w64(9, t261);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_be() { blk = 278; continue; }
            blk = 289; continue;
        }
        289 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 308; continue; }
            blk = 290; continue;
        }
        290 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 293; continue; }
            blk = 291; continue;
        }
        291 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 278; continue; }
            blk = 292; continue;
        }
        292 => {
            let Some(t262) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008372b3); };
            c.w32(8, t262);
            let r263 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r263);
            blk = 309; continue;
        }
        293 => {
            let Some(t264) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008372bf); };
            c.w32(9, t264);
            c.logic(c.r32(9) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 294; continue;
        }
        294 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 295; continue;
        }
        295 => {
            c.cmp(c.r16(8), 0x25u64, 16);
            if c.cc_e() { blk = 307; continue; }
            blk = 296; continue;
        }
        296 => {
            c.cmp(c.r16(8), 0x2du64, 16);
            if c.cc_e() { blk = 307; continue; }
            blk = 297; continue;
        }
        297 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(8, c.r64(1));
            blk = 309; continue;
        }
        299 => {
            c.w64(10, c.r64(4));
            let r265 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r265);
            let r266 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r266);
            c.logic(r266, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000837309); }
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(13));
            c.w64(8, c.r64(1));
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x7000837317); };
            c.w64(0, t267);
            c.w64(10, 0x7000837325u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000837325); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000837329); }
            return c.call(c.r64(0), 0x700083732f, 0x700083732d);
        }
        300 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700083732f); }
            let Some(t268) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000837337); };
            c.w64(4, t268);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 306; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 305; continue; }
            blk = 302; continue;
        }
        302 => {
            c.cmp(c.r64(8), 0xfffffffffffffffeu64, 64);
            if c.cc_e() { blk = 304; continue; }
            blk = 303; continue;
        }
        303 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000837352); };
            c.w64(2, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000837356); };
            c.w64(1, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083735a); };
            c.w64(0, t271);
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083735e); };
            c.w64(3, t272);
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000837362); };
            c.w64(6, t273);
            blk = 278; continue;
        }
        304 => {
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083736b); };
            c.w64(2, t274);
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083736f); };
            c.w64(1, t275);
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837373); };
            c.w64(0, t276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837377); };
            c.w64(3, t277);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083737b); };
            c.w64(6, t278);
            blk = 278; continue;
        }
        305 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000837384); };
            c.w64(2, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000837388); };
            c.w64(1, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083738c); };
            c.w64(0, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837390); };
            c.w64(3, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000837394); };
            c.w64(6, t283);
            blk = 278; continue;
        }
        306 => {
            c.w64(8, c.r64(0));
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008373a0); };
            c.w64(2, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008373a4); };
            c.w64(1, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008373a8); };
            c.w64(0, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008373ac); };
            c.w64(3, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008373b0); };
            c.w64(6, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008373b4); };
            c.w64(12, t289);
            blk = 309; continue;
        }
        307 => {
            let Some(t290) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008373ba); };
            c.w32(8, t290);
            let r291 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r291);
            blk = 309; continue;
        }
        308 => {
            c.w64(8, c.r64(1));
            blk = 309; continue;
        }
        309 => {
            c.cmp(c.r32(12), c.r32(8), 32);
            if c.cc_e() { blk = 312; continue; }
            blk = 310; continue;
        }
        310 => {
            blk = 278; continue;
        }
        311 => {
            c.w64(8, c.r64(1));
            blk = 312; continue;
        }
        312 => {
            let Some(t292) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008373d3); };
            c.w64(5, t292);
            c.w64(1, c.r64(8));
            let r293 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r293);
            return RegfileExit::jump(0x70007ffb34);
        }
        313 => {
            blk = 278; continue;
        }
        314 => {
            let Some(t294) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008373e8); };
            c.w32(9, t294);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r295 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r295);
            blk = 315; continue;
        }
        315 => {
            c.w64(11, c.r64(9).wrapping_mul(4));
            c.w32(15, c.r32(8));
            let Some(t296) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837402); };
            c.w32(7, t296);
            let r297 = c.r32(15) | 0x2u64;
            c.w32(15, r297);
            c.logic(r297, 32);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_e() { blk = 318; continue; }
            blk = 316; continue;
        }
        316 => {
            let r298 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r298);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ge() { blk = 315; continue; }
            blk = 317; continue;
        }
        317 => {
            blk = 278; continue;
        }
        318 => {
            let Some(t299) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083741f); };
            c.w32(8, t299);
            let r300 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r300);
            blk = 7; continue;
        }
        319 => {
            let Some(t301) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083742c); };
            c.w32(9, t301);
            let Some(t302) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x7000837430); };
            c.cmp(t302, c.r32(9), 32);
            if c.cc_e() { blk = 400; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t303) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083743d); };
            c.w32(9, t303);
            let r304 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r304);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t305) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083744c); };
            c.cmp(t305, c.r32(11), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 322; continue;
        }
        322 => {
            let Some(t306) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837456); };
            c.w32(9, t306);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000837460); }
            c.logic(c.r8(12) & 0x30u64, 8);
            if c.cc_e() { blk = 393; continue; }
            blk = 323; continue;
        }
        323 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 385; continue; }
            blk = 324; continue;
        }
        324 => {
            let Some(t307) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837478); };
            c.w32(12, t307);
            let r308 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r308);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 326; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000837485); };
            c.w64(12, t309);
            blk = 326; continue;
        }
        326 => {
            let Some(t310) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083748c); };
            c.w32(15, t310);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r311 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r311);
            let Some(t312) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083749b); };
            c.w32(7, t312);
            let r313 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r313);
            let r314 = c.r64(7) & c.r64(15);
            c.w64(7, r314);
            c.logic(r314, 64);
            blk = 327; continue;
        }
        327 => {
            c.w64(0, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(0).wrapping_mul(4));
            let Some(t315) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008374b1); };
            c.w32(9, t315);
            let Some(t316) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008374b6); };
            c.cmp(t316, c.r32(9), 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 328; continue;
        }
        328 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_e() { blk = 330; continue; }
            blk = 329; continue;
        }
        329 => {
            let r317 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r317);
            c.w64(9, c.r64(11).wrapping_add(c.r64(7)));
            c.w64(0, c.r64(15));
            let r318 = c.r64(0) & c.r64(9);
            c.w64(0, r318);
            c.logic(r318, 64);
            c.w64(7, c.r64(0));
            blk = 327; continue;
        }
        330 => {
            let Some(t319) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008374db); };
            c.w32(8, t319);
            let Some(t320) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008374e0); };
            c.w32(9, t320);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 332; continue; }
            blk = 331; continue;
        }
        331 => {
            return RegfileExit::fault(0x70008374f4);
        }
        332 => {
            let Some(t321) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008374f5); };
            c.w32(0, t321);
            let r322 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r322);
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 333; continue;
        }
        333 => {
            let Some(t323) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837507); };
            c.w32(8, t323);
            let Some(t324) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083750b); };
            c.cmp(t324, c.r32(8), 32);
            if c.cc_ne() { blk = 358; continue; }
            blk = 334; continue;
        }
        334 => {
            let Some(t325) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837518); };
            c.w32(8, t325);
            let r326 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r326);
            let Some(t327) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083751f); };
            c.w32(9, t327);
            let r328 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r328);
            let Some(t329) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000837526); };
            c.logic(t329 & 0x2u64, 8);
            if c.cc_ne() { blk = 356; continue; }
            blk = 335; continue;
        }
        335 => {
            let Some(t330) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000837531); };
            c.cmp(t330, c.r32(9), 32);
            if c.cc_e() { blk = 337; continue; }
            blk = 336; continue;
        }
        336 => {
            let Some(t331) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083753a); };
            c.w64(0, t331);
            blk = 15; continue;
        }
        337 => {
            let Some(t332) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000837546); };
            c.w64(9, t332);
            let Some(t333) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x700083754d); };
            c.cmp(t333, 0x20u64, 8);
            if c.cc_e() { blk = 354; continue; }
            blk = 338; continue;
        }
        338 => {
            let Some(t334) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000837557); };
            c.w32(9, t334);
            let Some(t335) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083755b); };
            c.cmp(t335, c.r32(9), 32);
            if c.cc_e() { blk = 340; continue; }
            blk = 339; continue;
        }
        339 => {
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837564); };
            c.w64(0, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837568); };
            c.w64(3, t337);
            blk = 278; continue;
        }
        340 => {
            let Some(t338) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837571); };
            c.w32(9, t338);
            let r339 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r339);
            let Some(t340) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837578); };
            c.w32(11, t340);
            let Some(t341) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083757c); };
            c.cmp(t341, c.r32(11), 32);
            if c.cc_ne() { blk = 342; continue; }
            blk = 341; continue;
        }
        341 => {
            let Some(t342) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000837585); };
            c.w32(9, t342);
            let r343 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r343);
            let Some(t344) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083758c); };
            c.w32(11, t344);
            let Some(t345) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000837590); };
            c.cmp(t345, c.r32(11), 32);
            if c.cc_e() { blk = 341; continue; }
            blk = 342; continue;
        }
        342 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 353; continue; }
            blk = 343; continue;
        }
        343 => {
            let Some(t346) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008375a3); };
            c.w32(11, t346);
            let r347 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r347);
            let Some(t348) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008375aa); };
            c.w32(12, t348);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 349; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 348; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t349) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008375ca); };
            c.cmp(t349, c.r32(11), 32);
            if c.cc_e() { blk = 347; continue; }
            blk = 346; continue;
        }
        346 => {
            c.w64(9, c.r64(2));
            blk = 349; continue;
        }
        347 => {
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008375d8); };
            c.w64(0, t350);
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008375dc); };
            c.w64(3, t351);
            blk = 278; continue;
        }
        348 => {
            let Some(t352) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008375e5); };
            c.w32(9, t352);
            let r353 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r353);
            blk = 349; continue;
        }
        349 => {
            let Some(t354) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008375ec); };
            c.w32(9, t354);
            let r355 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r355);
            let Some(t356) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008375f3); };
            c.w32(9, t356);
            let r357 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r357);
            let Some(t358) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008375fa); };
            c.w32(11, t358);
            let r359 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r359);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 352; continue; }
            blk = 350; continue;
        }
        350 => {
            let Some(t360) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083760c); };
            c.w32(6, t360);
            let r361 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r361);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837613); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007f6fb4, 0x700083761e, 0x7000837619);
        }
        351 => {
            blk = 15; continue;
        }
        352 => {
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837623); };
            c.w64(0, t362);
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837627); };
            c.w64(3, t363);
            blk = 278; continue;
        }
        353 => {
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837630); };
            c.w64(0, t364);
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837634); };
            c.w64(3, t365);
            blk = 278; continue;
        }
        354 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083763d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083763e); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083763f); }
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000837640); };
            c.w64(3, t366);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000837651, 0x700083764c);
        }
        355 => {
            blk = 15; continue;
        }
        356 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837656); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(8));
            c.w64(9, c.r64(0));
            return c.call(0x70007f2274, 0x7000837664, 0x700083765f);
        }
        357 => {
            blk = 15; continue;
        }
        358 => {
            let Some(t367) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837669); };
            c.w32(8, t367);
            let r368 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r368);
            let Some(t369) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837670); };
            c.w32(9, t369);
            c.cmp(c.r16(9), 0x834u64, 16);
            if c.cc_e() { blk = 381; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w32(11, c.r64(9).wrapping_sub(0x80c));
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_be() { blk = 369; continue; }
            blk = 360; continue;
        }
        360 => {
            c.cmp(c.r16(9), 0x411u64, 16);
            if c.cc_e() { blk = 362; continue; }
            blk = 361; continue;
        }
        361 => {
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083769a); };
            c.w64(0, t370);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083769e); };
            c.w64(3, t371);
            blk = 278; continue;
        }
        362 => {
            let Some(t372) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008376a7); };
            c.w32(8, t372);
            let Some(t373) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008376ab); };
            c.cmp(t373, c.r32(8), 32);
            if c.cc_e() { blk = 364; continue; }
            blk = 363; continue;
        }
        363 => {
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008376b4); };
            c.w64(0, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008376b8); };
            c.w64(3, t375);
            blk = 278; continue;
        }
        364 => {
            let Some(t376) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008376c1); };
            c.w32(8, t376);
            let r377 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r377);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 366; continue; }
            blk = 365; continue;
        }
        365 => {
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008376ce); };
            c.w64(0, t378);
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008376d2); };
            c.w64(3, t379);
            blk = 278; continue;
        }
        366 => {
            let Some(t380) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008376db); };
            c.w32(9, t380);
            let r381 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r381);
            let Some(t382) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008376e2); };
            c.cmp(t382, 0x40u64, 16);
            if c.cc_b() { blk = 368; continue; }
            blk = 367; continue;
        }
        367 => {
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008376ea); };
            c.w64(0, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008376ee); };
            c.w64(3, t384);
            blk = 278; continue;
        }
        368 => {
            let Some(t385) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008376f7); };
            c.w32(8, t385);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        369 => {
            let Some(t386) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837704); };
            c.w32(9, t386);
            let Some(t387) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000837708); };
            c.cmp(t387, c.r32(9), 32);
            if c.cc_e() { blk = 371; continue; }
            blk = 370; continue;
        }
        370 => {
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837711); };
            c.w64(0, t388);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837715); };
            c.w64(3, t389);
            blk = 278; continue;
        }
        371 => {
            let Some(t390) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083771e); };
            c.w32(8, t390);
            c.w64(9, c.r64(8));
            let r391 = c.r32(9) & 0xc0u64;
            c.w32(9, r391);
            c.logic(r391, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 374; continue; }
            blk = 372; continue;
        }
        372 => {
            let Some(t392) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837736); };
            c.w32(9, t392);
            let r393 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r393);
            let Some(t394) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083773d); };
            c.w32(9, t394);
            let r395 = c.r32(9) & 0x1fu64;
            c.w32(9, r395);
            c.logic(r395, 32);
            let r396 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r396);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_be() { blk = 374; continue; }
            blk = 373; continue;
        }
        373 => {
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083774f); };
            c.w64(0, t397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837753); };
            c.w64(3, t398);
            blk = 278; continue;
        }
        374 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 380; continue; }
            blk = 375; continue;
        }
        375 => {
            let Some(t399) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000837762); };
            c.w32(0, t399);
            let r400 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r400);
            let Some(t401) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000837768); };
            c.cmp(t401, c.r32(0), 32);
            if c.cc_e() { blk = 379; continue; }
            blk = 376; continue;
        }
        376 => {
            let Some(t402) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837771); };
            c.w32(8, t402);
            let Some(t403) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000837775); };
            c.cmp(t403, c.r32(8), 32);
            if c.cc_e() { blk = 378; continue; }
            blk = 377; continue;
        }
        377 => {
            blk = 15; continue;
        }
        378 => {
            let Some(t404) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837783); };
            c.w32(0, t404);
            let r405 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r405);
            blk = 15; continue;
        }
        379 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083778e); };
            c.w64(0, t406);
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837792); };
            c.w64(3, t407);
            blk = 278; continue;
        }
        380 => {
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083779b); };
            c.w64(0, t408);
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083779f); };
            c.w64(3, t409);
            blk = 278; continue;
        }
        381 => {
            let Some(t410) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008377a8); };
            c.w32(8, t410);
            let Some(t411) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008377ac); };
            c.cmp(t411, c.r32(8), 32);
            if c.cc_e() { blk = 383; continue; }
            blk = 382; continue;
        }
        382 => {
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008377b5); };
            c.w64(0, t412);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008377b9); };
            c.w64(3, t413);
            blk = 278; continue;
        }
        383 => {
            let Some(t414) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008377c2); };
            c.w32(0, t414);
            let r415 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r415);
            blk = 15; continue;
        }
        384 => {
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008377cd); };
            c.w64(0, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008377d1); };
            c.w64(3, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008377d5); };
            c.w64(9, t418);
            blk = 393; continue;
        }
        385 => {
            let Some(t419) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008377de); };
            c.w32(11, t419);
            let r420 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r420);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 386; continue;
        }
        386 => {
            let r421 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r421);
            c.logic(r421, 64);
            let Some(t422) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008377f3); };
            c.w32(12, t422);
            let r423 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r423);
            let Some(t424) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008377fa); };
            c.w32(12, t424);
            let r425 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r425);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 393; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t426) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837807); };
            c.w32(15, t426);
            let r427 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r427);
            let Some(t428) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083780e); };
            c.cmp(t428, 0xbeu64, 16);
            if c.cc_ne() { blk = 278; continue; }
            blk = 388; continue;
        }
        388 => {
            let Some(t429) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083781b); };
            c.w32(15, t429);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 390; continue; }
            blk = 389; continue;
        }
        389 => {
            return RegfileExit::fault(0x700083782e);
        }
        390 => {
            let Some(t430) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083782f); };
            c.w32(15, t430);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t431) = c.ld32(c.r64(11).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x7000837837); };
            c.w32(11, t431);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 392; continue; }
            blk = 391; continue;
        }
        391 => {
            return RegfileExit::fault(0x700083784a);
        }
        392 => {
            let Some(t432) = c.ld32(c.r64(12).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083784b); };
            c.cmp(t432, c.r32(11), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t433) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837859); };
            c.w32(11, t433);
            let r434 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r434);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 399; continue; }
            blk = 394; continue;
        }
        394 => {
            let Some(t435) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000837866); };
            c.cmp(t435, c.r32(11), 32);
            if c.cc_e() { blk = 397; continue; }
            blk = 395; continue;
        }
        395 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 396; continue;
        }
        396 => {
            let r436 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r436);
            c.logic(r436, 64);
            blk = 398; continue;
        }
        397 => {
            c.w64(11, c.r64(2));
            blk = 398; continue;
        }
        398 => {
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(10));
            blk = 9; continue;
        }
        399 => {
            c.w64(0, c.r64(11));
            blk = 15; continue;
        }
        400 => {
            let Some(t437) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000837898); };
            c.w64(5, t437);
            let r438 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r438);
            let Some(t439) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008378a0); };
            c.logic(t439 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 402; continue; }
            blk = 401; continue;
        }
        401 => {
            let r440 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r440);
            blk = 403; continue;
        }
        402 => {
            let Some(t441) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008378b0); };
            c.w32(8, t441);
            let Some(t442) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008378b4); };
            c.w64(8, t442);
            blk = 403; continue;
        }
        403 => {
            return RegfileExit::jump(c.r64(8));
        }
        404 => {
            c.cmp(c.r64(12), 0x2u64, 64);
            if c.cc_e() { blk = 496; continue; }
            blk = 405; continue;
        }
        405 => {
            c.cmp(c.r64(12), 0x6u64, 64);
            if c.cc_e() { blk = 494; continue; }
            blk = 406; continue;
        }
        406 => {
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 491; continue; }
            blk = 407; continue;
        }
        407 => {
            c.cmp(c.r64(12), 0x8u64, 64);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 480; continue; }
            blk = 408; continue;
        }
        408 => {
            c.cmp(c.r64(12), 0x9u64, 64);
            if c.cc_e() { blk = 480; continue; }
            blk = 409; continue;
        }
        409 => {
            c.cmp(c.r64(12), 0x3u64, 64);
            if c.cc_e() { blk = 437; continue; }
            blk = 410; continue;
        }
        410 => {
            c.cmp(c.r64(12), 0xbu64, 64);
            if c.cc_e() { blk = 493; continue; }
            blk = 411; continue;
        }
        411 => {
            c.cmp(c.r64(12), 0xcu64, 64);
            if c.cc_e() { blk = 418; continue; }
            blk = 412; continue;
        }
        412 => {
            c.cmp(c.r64(12), 0xeu64, 64);
            if c.cc_e() { blk = 414; continue; }
            blk = 413; continue;
        }
        413 => {
            let Some(t443) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000837921); };
            c.w64(5, t443);
            let r444 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r444);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837929); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700083792a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083792c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083792d); }
            let Some(t445) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083792e); };
            if !c.push64(t445) { return RegfileExit::fault(0x700083792e); }
            c.w32(0, 0x5u64);
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x7000837937); };
            c.w64(3, t446);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(1)) { return RegfileExit::fault(0x700083793e); }
            return RegfileExit::jump(0x700086ed74);
        }
        414 => {
            let Some(t447) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837948); };
            c.w32(8, t447);
            let r448 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r448);
            let Some(t449) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083794f); };
            c.w32(8, t449);
            let r450 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r450);
            let Some(t451) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837956); };
            c.w32(9, t451);
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r452 = c.r64(11) & 0x1ffffffu64;
            c.w64(11, r452);
            c.logic(r452, 64);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 416; continue; }
            blk = 415; continue;
        }
        415 => {
            return RegfileExit::fault(0x7000837970);
        }
        416 => {
            let Some(t453) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837971); };
            c.w32(8, t453);
            let r454 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r454);
            let Some(t455) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837979); };
            c.w32(0, t455);
            let r456 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r456);
            let Some(t457) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000837980); };
            c.cmp(t457, c.r32(0), 32);
            if c.cc_ne() { blk = 377; continue; }
            blk = 417; continue;
        }
        417 => {
            let Some(t458) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083798d); };
            c.w64(5, t458);
            let r459 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r459);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000837995); }
            let Some(t460) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000837996); };
            if !c.push64(t460) { return RegfileExit::fault(0x7000837996); }
            c.w32(0, 0x2u64);
            let Some(t461) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700083799f); };
            c.w64(3, t461);
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x174u64) { return RegfileExit::fault(0x70008379a6); }
            return RegfileExit::jump(0x700086ed74);
        }
        418 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 435; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t462) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008379bd); };
            c.w32(8, t462);
            let r463 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r463);
            let Some(t464) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008379c4); };
            c.w32(8, t464);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 423; continue; }
            blk = 420; continue;
        }
        420 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008379d0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 423; continue; }
            blk = 421; continue;
        }
        421 => {
            if c.cc_ne() { blk = 423; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t465) = c.ld64(0x7000836801u64) else { return RegfileExit::fault(0x70008379eb); };
            c.w64(12, t465);
            let r466 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r466);
            let Some(t467) = c.ld64(0x700083680eu64) else { return RegfileExit::fault(0x70008379f5); };
            c.w64(11, t467);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 435; continue; }
            blk = 423; continue;
        }
        423 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 434; continue; }
            blk = 424; continue;
        }
        424 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 427; continue; }
            blk = 425; continue;
        }
        425 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 435; continue; }
            blk = 426; continue;
        }
        426 => {
            let Some(t468) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837a1a); };
            c.w32(1, t468);
            let r469 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r469);
            blk = 434; continue;
        }
        427 => {
            let Some(t470) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837a22); };
            c.w32(11, t470);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 435; continue; }
            blk = 428; continue;
        }
        428 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 435; continue; }
            blk = 429; continue;
        }
        429 => {
            c.cmp(c.r16(8), 0x25u64, 16);
            if c.cc_e() { blk = 433; continue; }
            blk = 430; continue;
        }
        430 => {
            c.cmp(c.r16(8), 0x2du64, 16);
            if c.cc_e() { blk = 433; continue; }
            blk = 431; continue;
        }
        431 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 435; continue; }
            blk = 432; continue;
        }
        432 => {
            blk = 434; continue;
        }
        433 => {
            let Some(t471) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837a4b); };
            c.w32(1, t471);
            let r472 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r472);
            blk = 434; continue;
        }
        434 => {
            let Some(t473) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000837a51); };
            c.w64(5, t473);
            c.w64(3, c.r64(1));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            c.w64(0, c.r64(9));
            let r474 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r474);
            return RegfileExit::jump(0x70008bc174);
        }
        435 => {
            let Some(t475) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000837a6c); };
            c.w64(5, t475);
            let r476 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r476);
            let Some(t477) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000837a74); };
            if !c.push64(t477) { return RegfileExit::fault(0x7000837a74); }
            let Some(t478) = c.ld64(c.r64(13).wrapping_add(0x2610)) else { return RegfileExit::fault(0x7000837a78); };
            c.w64(3, t478);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(9)) { return RegfileExit::fault(0x7000837a84); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(1)) { return RegfileExit::fault(0x7000837a89); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x7000837a8e); }
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x2u64) { return RegfileExit::fault(0x7000837a93); }
            return RegfileExit::jump(0x700086ed74);
        }
        436 => {
            blk = 493; continue;
        }
        437 => {
            let Some(t479) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837aa6); };
            c.w32(8, t479);
            let r480 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r480);
            let Some(t481) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837aad); };
            c.w32(11, t481);
            let Some(t482) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000837ab1); };
            c.cmp(t482, c.r32(8), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 438; continue;
        }
        438 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 479; continue; }
            blk = 439; continue;
        }
        439 => {
            let Some(t483) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837ac8); };
            c.w32(11, t483);
            let r484 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r484);
            let Some(t485) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000837acf); };
            c.cmp(t485, c.r32(11), 32);
            if c.cc_ne() { blk = 462; continue; }
            blk = 440; continue;
        }
        440 => {
            let Some(t486) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837adc); };
            c.w32(11, t486);
            let r487 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r487);
            let Some(t488) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837ae3); };
            c.w32(12, t488);
            let r489 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r489);
            let Some(t490) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000837aea); };
            c.logic(t490 & 0x2u64, 8);
            if c.cc_ne() { blk = 460; continue; }
            blk = 441; continue;
        }
        441 => {
            let Some(t491) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000837af6); };
            c.cmp(t491, c.r32(12), 32);
            if c.cc_e() { blk = 443; continue; }
            blk = 442; continue;
        }
        442 => {
            let Some(t492) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000837aff); };
            c.w64(0, t492);
            blk = 15; continue;
        }
        443 => {
            let Some(t493) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000837b0b); };
            c.w64(12, t493);
            let Some(t494) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x7000837b12); };
            c.cmp(t494, 0x20u64, 8);
            if c.cc_e() { blk = 458; continue; }
            blk = 444; continue;
        }
        444 => {
            let Some(t495) = c.ld32(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000837b1d); };
            c.w32(12, t495);
            let Some(t496) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000837b21); };
            c.cmp(t496, c.r32(12), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 445; continue;
        }
        445 => {
            let Some(t497) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837b2e); };
            c.w32(12, t497);
            let r498 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r498);
            let Some(t499) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837b35); };
            c.w32(15, t499);
            let Some(t500) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000837b3a); };
            c.cmp(t500, c.r32(15), 32);
            if c.cc_ne() { blk = 447; continue; }
            blk = 446; continue;
        }
        446 => {
            let Some(t501) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000837b43); };
            c.w32(12, t501);
            let r502 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r502);
            let Some(t503) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837b4b); };
            c.w32(15, t503);
            let Some(t504) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000837b50); };
            c.cmp(t504, c.r32(15), 32);
            if c.cc_e() { blk = 446; continue; }
            blk = 447; continue;
        }
        447 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 448; continue;
        }
        448 => {
            let Some(t505) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837b63); };
            c.w32(15, t505);
            let r506 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r506);
            let Some(t507) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837b6b); };
            c.w32(7, t507);
            c.w32(0, c.r64(7).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 453; continue; }
            blk = 449; continue;
        }
        449 => {
            c.w32(12, c.r16(7));
            c.cmp(c.r32(12), 0x82au64, 32);
            if c.cc_e() { blk = 452; continue; }
            blk = 450; continue;
        }
        450 => {
            let Some(t508) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000837b88); };
            c.cmp(t508, c.r32(15), 32);
            if c.cc_ne() { blk = 454; continue; }
            blk = 451; continue;
        }
        451 => {
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837b91); };
            c.w64(0, t509);
            blk = 278; continue;
        }
        452 => {
            c.w64(9, c.r64(8));
            blk = 454; continue;
        }
        453 => {
            c.w64(9, c.r64(12));
            blk = 454; continue;
        }
        454 => {
            let Some(t510) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837ba2); };
            c.w32(8, t510);
            let r511 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r511);
            let Some(t512) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837ba9); };
            c.w32(8, t512);
            let r513 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r513);
            let Some(t514) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837bb0); };
            c.w32(9, t514);
            let r515 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r515);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 457; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t516) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000837bc2); };
            c.w32(6, t516);
            let r517 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r517);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837bc9); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(11));
            return c.call(0x70007f6fb4, 0x7000837bd4, 0x7000837bcf);
        }
        456 => {
            blk = 15; continue;
        }
        457 => {
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837bd9); };
            c.w64(0, t518);
            blk = 278; continue;
        }
        458 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000837be2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000837be4); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837be5); }
            let Some(t519) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000837be6); };
            c.w64(3, t519);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000837bf7, 0x7000837bf2);
        }
        459 => {
            blk = 15; continue;
        }
        460 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837bfc); }
            c.w64(7, c.r64(11));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000837c07, 0x7000837c02);
        }
        461 => {
            blk = 15; continue;
        }
        462 => {
            let Some(t520) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837c0c); };
            c.w32(12, t520);
            let r521 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r521);
            let Some(t522) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837c13); };
            c.w32(15, t522);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 477; continue; }
            blk = 463; continue;
        }
        463 => {
            c.w32(7, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 469; continue; }
            blk = 464; continue;
        }
        464 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_ne() { blk = 278; continue; }
            blk = 465; continue;
        }
        465 => {
            let Some(t523) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837c3d); };
            c.w32(9, t523);
            let Some(t524) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000837c41); };
            c.cmp(t524, c.r32(9), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 466; continue;
        }
        466 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 467; continue;
        }
        467 => {
            let Some(t525) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837c58); };
            c.cmp(t525, 0x40u64, 16);
            if c.cc_ae() { blk = 278; continue; }
            blk = 468; continue;
        }
        468 => {
            let Some(t526) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837c64); };
            c.w32(8, t526);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        469 => {
            let Some(t527) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837c71); };
            c.w32(11, t527);
            let Some(t528) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000837c75); };
            c.cmp(t528, c.r32(11), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 470; continue;
        }
        470 => {
            let Some(t529) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000837c82); };
            c.w32(11, t529);
            c.w64(12, c.r64(11));
            let r530 = c.r32(12) & 0xc0u64;
            c.w32(12, r530);
            c.logic(r530, 32);
            c.cmp(c.r32(12), 0xc0u64, 32);
            if c.cc_e() { blk = 472; continue; }
            blk = 471; continue;
        }
        471 => {
            let Some(t531) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000837c9b); };
            c.w32(8, t531);
            let r532 = c.r32(8) & 0x1fu64;
            c.w32(8, r532);
            c.logic(r532, 32);
            let r533 = c.sub(c.r32(8), 0x10u64, 32);
            c.w32(8, r533);
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_a() { blk = 278; continue; }
            blk = 472; continue;
        }
        472 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 278; continue; }
            blk = 473; continue;
        }
        473 => {
            let Some(t534) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000837cbb); };
            c.w32(8, t534);
            let r535 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r535);
            let Some(t536) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000837cc2); };
            c.cmp(t536, c.r32(8), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 474; continue;
        }
        474 => {
            let Some(t537) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837ccf); };
            c.w32(9, t537);
            let Some(t538) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000837cd3); };
            c.cmp(t538, c.r32(9), 32);
            if c.cc_e() { blk = 476; continue; }
            blk = 475; continue;
        }
        475 => {
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        476 => {
            let Some(t539) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837ce4); };
            c.w32(0, t539);
            let r540 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r540);
            blk = 15; continue;
        }
        477 => {
            let Some(t541) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837cf0); };
            c.w32(9, t541);
            let Some(t542) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000837cf4); };
            c.cmp(t542, c.r32(9), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 478; continue;
        }
        478 => {
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        479 => {
            c.w64(0, c.r64(8));
            blk = 15; continue;
        }
        480 => {
            let Some(t543) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000837d11); };
            c.w64(12, t543);
            let Some(t544) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x7000837d18); };
            c.cmp(t544, 0x20u64, 8);
            if c.cc_e() { blk = 493; continue; }
            blk = 481; continue;
        }
        481 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 483; continue; }
            blk = 482; continue;
        }
        482 => {
            let Some(t545) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837d29); };
            c.w32(8, t545);
            let r546 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r546);
            blk = 484; continue;
        }
        483 => {
            let Some(t547) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000837d32); };
            c.w32(8, t547);
            let r548 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r548);
            blk = 484; continue;
        }
        484 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 486; continue; }
            blk = 485; continue;
        }
        485 => {
            return RegfileExit::fault(0x7000837d3f);
        }
        486 => {
            let r549 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r549);
            c.logic(r549, 64);
            let Some(t550) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837d44); };
            c.w32(11, t550);
            let r551 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r551);
            let Some(t552) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837d4b); };
            c.w32(3, t552);
            let r553 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r553);
            let Some(t554) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837d52); };
            c.w64(9, t554);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 488; continue; }
            blk = 487; continue;
        }
        487 => {
            let Some(t555) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837d5c); };
            c.w32(11, t555);
            let r556 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r556);
            let Some(t557) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837d63); };
            c.w32(11, t557);
            let r558 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r558);
            blk = 489; continue;
        }
        488 => {
            c.w64(11, c.r64(2));
            blk = 489; continue;
        }
        489 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837d6f); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            c.w64(7, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x7000837d80, 0x7000837d7b);
        }
        490 => {
            blk = 15; continue;
        }
        491 => {
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000837d85); };
            c.w64(8, t559);
            let Some(t560) = c.ld8(c.r64(8)) else { return RegfileExit::fault(0x7000837d8c); };
            c.cmp(t560, 0x20u64, 8);
            if c.cc_e() { blk = 493; continue; }
            blk = 492; continue;
        }
        492 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r561 = c.r64(11) & 0x3ffu64;
            c.w64(11, r561);
            c.logic(r561, 64);
            let r562 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r562);
            let Some(t563) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837da1); };
            c.w32(11, t563);
            let r564 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r564);
            let Some(t565) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000837da8); };
            c.w32(11, t565);
            let r566 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r566);
            let Some(t567) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000837daf); };
            c.w32(3, t567);
            let r568 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r568);
            let Some(t569) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000837db7); };
            c.w64(5, t569);
            c.w64(1, c.r64(9));
            let r570 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r570);
            return RegfileExit::jump(0x70007fea14);
        }
        493 => {
            let Some(t571) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000837dc7); };
            c.w64(5, t571);
            let r572 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r572);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000837dcf); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837dd0); }
            let Some(t573) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000837dd1); };
            if !c.push64(t573) { return RegfileExit::fault(0x7000837dd1); }
            let Some(t574) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000837dd5); };
            c.w64(3, t574);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x7000837de1); }
            return RegfileExit::jump(0x700086ed74);
        }
        494 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r575 = c.r64(11) & 0x3ffu64;
            c.w64(11, r575);
            c.logic(r575, 64);
            let r576 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r576);
            let Some(t577) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837dfa); };
            c.w32(9, t577);
            let r578 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r578);
            let Some(t579) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000837e01); };
            c.w32(9, t579);
            let r580 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r580);
            let Some(t581) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000837e08); };
            c.w32(8, t581);
            let r582 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r582);
            let Some(t583) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837e10); };
            c.w32(7, t583);
            let r584 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r584);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837e17); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000837e1f, 0x7000837e1a);
        }
        495 => {
            blk = 15; continue;
        }
        496 => {
            let Some(t585) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837e24); };
            c.w32(8, t585);
            let r586 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r586);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 498; continue; }
            blk = 497; continue;
        }
        497 => {
            let Some(t587) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000837e31); };
            c.w64(8, t587);
            blk = 498; continue;
        }
        498 => {
            let Some(t588) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837e38); };
            c.w32(11, t588);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r589 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r589);
            let Some(t590) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837e46); };
            c.w32(12, t590);
            let r591 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r591);
            let r592 = c.r64(12) & c.r64(11);
            c.w64(12, r592);
            c.logic(r592, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 499; continue;
        }
        499 => {
            c.w64(7, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(0, c.r64(7).wrapping_mul(4));
            let Some(t593) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837e61); };
            c.w32(3, t593);
            let Some(t594) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000837e66); };
            c.cmp(t594, c.r32(3), 32);
            if c.cc_e() { blk = 555; continue; }
            blk = 500; continue;
        }
        500 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 501; continue;
        }
        501 => {
            let r595 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r595);
            let r596 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r596);
            c.w64(0, c.r64(11));
            let r597 = c.r64(0) & c.r64(12);
            c.w64(0, r597);
            c.logic(r597, 64);
            c.w64(12, c.r64(0));
            blk = 499; continue;
        }
        502 => {
            let Some(t598) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837e89); };
            c.w32(11, t598);
            let Some(t599) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837e8d); };
            c.w32(12, t599);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 504; continue; }
            blk = 503; continue;
        }
        503 => {
            return RegfileExit::fault(0x7000837ea1);
        }
        504 => {
            let Some(t600) = c.ld32(c.r64(8).wrapping_add(c.r64(0)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837ea2); };
            c.w32(0, t600);
            let r601 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r601);
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 505; continue;
        }
        505 => {
            let Some(t602) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837eb4); };
            c.w32(8, t602);
            let Some(t603) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000837eb8); };
            c.cmp(t603, c.r32(8), 32);
            if c.cc_ne() { blk = 530; continue; }
            blk = 506; continue;
        }
        506 => {
            let Some(t604) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000837ec5); };
            c.w32(8, t604);
            let r605 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r605);
            let Some(t606) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837ecc); };
            c.w32(11, t606);
            let r607 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r607);
            let Some(t608) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000837ed3); };
            c.logic(t608 & 0x2u64, 8);
            if c.cc_ne() { blk = 528; continue; }
            blk = 507; continue;
        }
        507 => {
            let Some(t609) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000837ede); };
            c.cmp(t609, c.r32(11), 32);
            if c.cc_e() { blk = 509; continue; }
            blk = 508; continue;
        }
        508 => {
            let Some(t610) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000837ee7); };
            c.w64(0, t610);
            blk = 15; continue;
        }
        509 => {
            let Some(t611) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000837ef3); };
            c.w64(11, t611);
            let Some(t612) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000837efa); };
            c.cmp(t612, 0x20u64, 8);
            if c.cc_e() { blk = 526; continue; }
            blk = 510; continue;
        }
        510 => {
            let Some(t613) = c.ld32(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000837f04); };
            c.w32(11, t613);
            let Some(t614) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000837f08); };
            c.cmp(t614, c.r32(11), 32);
            if c.cc_e() { blk = 512; continue; }
            blk = 511; continue;
        }
        511 => {
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837f11); };
            c.w64(0, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837f15); };
            c.w64(3, t616);
            blk = 278; continue;
        }
        512 => {
            let Some(t617) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837f1e); };
            c.w32(11, t617);
            let r618 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r618);
            let Some(t619) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837f25); };
            c.w32(12, t619);
            let Some(t620) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000837f29); };
            c.cmp(t620, c.r32(12), 32);
            if c.cc_ne() { blk = 514; continue; }
            blk = 513; continue;
        }
        513 => {
            let Some(t621) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000837f32); };
            c.w32(11, t621);
            let r622 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r622);
            let Some(t623) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837f39); };
            c.w32(12, t623);
            let Some(t624) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000837f3d); };
            c.cmp(t624, c.r32(12), 32);
            if c.cc_e() { blk = 513; continue; }
            blk = 514; continue;
        }
        514 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 525; continue; }
            blk = 515; continue;
        }
        515 => {
            let Some(t625) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837f50); };
            c.w32(12, t625);
            let r626 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r626);
            let Some(t627) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837f57); };
            c.w32(15, t627);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 520; continue; }
            blk = 516; continue;
        }
        516 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 519; continue; }
            blk = 517; continue;
        }
        517 => {
            let Some(t628) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000837f76); };
            c.cmp(t628, c.r32(12), 32);
            if c.cc_ne() { blk = 521; continue; }
            blk = 518; continue;
        }
        518 => {
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837f7f); };
            c.w64(0, t629);
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837f83); };
            c.w64(3, t630);
            blk = 278; continue;
        }
        519 => {
            let Some(t631) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000837f8c); };
            c.w32(11, t631);
            let r632 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r632);
            c.w64(9, c.r64(11));
            blk = 521; continue;
        }
        520 => {
            c.w64(9, c.r64(11));
            blk = 521; continue;
        }
        521 => {
            let Some(t633) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000837f9b); };
            c.w32(9, t633);
            let r634 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r634);
            let Some(t635) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000837fa2); };
            c.w32(9, t635);
            let r636 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r636);
            let Some(t637) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000837fa9); };
            c.w32(11, t637);
            let r638 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r638);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 524; continue; }
            blk = 522; continue;
        }
        522 => {
            let Some(t639) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000837fbb); };
            c.w32(6, t639);
            let r640 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r640);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837fc2); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(8));
            return c.call(0x70007f6fb4, 0x7000837fcd, 0x7000837fc8);
        }
        523 => {
            blk = 15; continue;
        }
        524 => {
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837fd2); };
            c.w64(0, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837fd6); };
            c.w64(3, t642);
            blk = 278; continue;
        }
        525 => {
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000837fdf); };
            c.w64(0, t643);
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000837fe3); };
            c.w64(3, t644);
            blk = 278; continue;
        }
        526 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000837fec); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000837fee); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000837fef); }
            let Some(t645) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000837ff0); };
            c.w64(3, t645);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x7000838001, 0x7000837ffc);
        }
        527 => {
            blk = 15; continue;
        }
        528 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000838006); }
            c.w64(7, c.r64(8));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000838011, 0x700083800c);
        }
        529 => {
            blk = 15; continue;
        }
        530 => {
            let Some(t646) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838016); };
            c.w32(8, t646);
            let r647 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r647);
            let Some(t648) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083801d); };
            c.w32(11, t648);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 552; continue; }
            blk = 531; continue;
        }
        531 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 541; continue; }
            blk = 532; continue;
        }
        532 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_e() { blk = 534; continue; }
            blk = 533; continue;
        }
        533 => {
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000838047); };
            c.w64(0, t649);
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083804b); };
            c.w64(3, t650);
            blk = 278; continue;
        }
        534 => {
            let Some(t651) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838054); };
            c.w32(8, t651);
            let Some(t652) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000838058); };
            c.cmp(t652, c.r32(8), 32);
            if c.cc_e() { blk = 536; continue; }
            blk = 535; continue;
        }
        535 => {
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000838061); };
            c.w64(0, t653);
            let Some(t654) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000838065); };
            c.w64(3, t654);
            blk = 278; continue;
        }
        536 => {
            let Some(t655) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083806e); };
            c.w32(8, t655);
            let r656 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r656);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 538; continue; }
            blk = 537; continue;
        }
        537 => {
            let Some(t657) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083807b); };
            c.w64(0, t657);
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083807f); };
            c.w64(3, t658);
            blk = 278; continue;
        }
        538 => {
            let Some(t659) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838088); };
            c.w32(9, t659);
            let r660 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r660);
            let Some(t661) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083808f); };
            c.cmp(t661, 0x40u64, 16);
            if c.cc_b() { blk = 540; continue; }
            blk = 539; continue;
        }
        539 => {
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000838097); };
            c.w64(0, t662);
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083809b); };
            c.w64(3, t663);
            blk = 278; continue;
        }
        540 => {
            let Some(t664) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008380a4); };
            c.w32(8, t664);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 15; continue;
        }
        541 => {
            let Some(t665) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008380b1); };
            c.w32(11, t665);
            let Some(t666) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008380b5); };
            c.cmp(t666, c.r32(11), 32);
            if c.cc_e() { blk = 543; continue; }
            blk = 542; continue;
        }
        542 => {
            let Some(t667) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008380be); };
            c.w64(0, t667);
            let Some(t668) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008380c2); };
            c.w64(3, t668);
            blk = 278; continue;
        }
        543 => {
            let Some(t669) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008380cb); };
            c.w32(8, t669);
            c.w64(11, c.r64(8));
            let r670 = c.r32(11) & 0xc0u64;
            c.w32(11, r670);
            c.logic(r670, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 546; continue; }
            blk = 544; continue;
        }
        544 => {
            let Some(t671) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008380e3); };
            c.w32(11, t671);
            let r672 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r672);
            let Some(t673) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008380ea); };
            c.w32(11, t673);
            let r674 = c.r32(11) & 0x1fu64;
            c.w32(11, r674);
            c.logic(r674, 32);
            let r675 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r675);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 546; continue; }
            blk = 545; continue;
        }
        545 => {
            let Some(t676) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008380fc); };
            c.w64(0, t676);
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000838100); };
            c.w64(3, t677);
            blk = 278; continue;
        }
        546 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 551; continue; }
            blk = 547; continue;
        }
        547 => {
            let Some(t678) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083810f); };
            c.w32(0, t678);
            let r679 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r679);
            let Some(t680) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000838116); };
            c.cmp(t680, c.r32(0), 32);
            if c.cc_e() { blk = 550; continue; }
            blk = 548; continue;
        }
        548 => {
            let Some(t681) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083811f); };
            c.w32(8, t681);
            let Some(t682) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000838123); };
            c.cmp(t682, c.r32(8), 32);
            if c.cc_ne() { blk = 377; continue; }
            blk = 549; continue;
        }
        549 => {
            let Some(t683) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838130); };
            c.w32(0, t683);
            let r684 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r684);
            blk = 15; continue;
        }
        550 => {
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083813b); };
            c.w64(0, t685);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083813f); };
            c.w64(3, t686);
            blk = 278; continue;
        }
        551 => {
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000838148); };
            c.w64(0, t687);
            let Some(t688) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083814c); };
            c.w64(3, t688);
            blk = 278; continue;
        }
        552 => {
            let Some(t689) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838155); };
            c.w32(8, t689);
            let Some(t690) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000838159); };
            c.cmp(t690, c.r32(8), 32);
            if c.cc_e() { blk = 554; continue; }
            blk = 553; continue;
        }
        553 => {
            let Some(t691) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000838162); };
            c.w64(0, t691);
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000838166); };
            c.w64(3, t692);
            blk = 278; continue;
        }
        554 => {
            let Some(t693) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083816f); };
            c.w32(0, t693);
            let r694 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r694);
            blk = 15; continue;
        }
        555 => {
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083817b); };
            c.w64(0, t695);
            let Some(t696) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083817f); };
            c.w64(3, t696);
            blk = 278; continue;
        }
        556 => {
            return RegfileExit::fault(0x7000838188);
        }
        557 => {
            let Some(t697) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000838189); };
            c.w64(9, t697);
            let Some(t698) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000838190); };
            c.w64(11, t698);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000838197); }
            let Some(t699) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083819b); };
            c.w64(15, t699);
            let Some(t700) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008381a2); };
            c.cmp(t700, c.r64(12), 64);
            if c.cc_a() { blk = 560; continue; }
            blk = 558; continue;
        }
        558 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008381b1, 0x70008381ac);
        }
        559 => {
            let Some(t701) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008381b1); };
            c.w64(8, t701);
            blk = 561; continue;
        }
        560 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008381b7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 561; continue;
        }
        561 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t702) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008381ca); };
            c.w64(8, t702);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008381d1); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008381d5); }
            blk = 15; continue;
        }
        562 => {
            let Some(t703) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008381df); };
            c.w64(9, t703);
            let Some(t704) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008381e6); };
            c.w64(11, t704);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008381ed); }
            let Some(t705) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008381f1); };
            c.w64(15, t705);
            let Some(t706) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008381f8); };
            c.cmp(t706, c.r64(12), 64);
            if c.cc_a() { blk = 565; continue; }
            blk = 563; continue;
        }
        563 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000838207, 0x7000838202);
        }
        564 => {
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838207); };
            c.w64(8, t707);
            blk = 566; continue;
        }
        565 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083820d); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 566; continue;
        }
        566 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t708) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000838223); };
            c.w64(8, t708);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083822a); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083822e); }
            blk = 15; continue;
        }
        567 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083823d); }
            return c.call(0x70007febb4, 0x7000838246, 0x7000838241);
        }
        568 => {
            c.w64(9, c.r64(0));
            let Some(t709) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838249); };
            c.w64(8, t709);
            blk = 57; continue;
        }
        569 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700083825c, 0x7000838257);
        }
        570 => {
            c.w64(8, c.r64(0));
            blk = 62; continue;
        }
        571 => {
            let Some(t710) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000838264); };
            c.w64(8, t710);
            let Some(t711) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083826b); };
            c.w32(8, t711);
            let r712 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r712);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000838272); }
            c.w32(0, 0x1u64);
            let Some(t713) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x7000838279); };
            c.w64(3, t713);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700083828a, 0x7000838285);
        }
        572 => {
            return RegfileExit::fault(0x700083828a);
        }
        573 => {
            blk = 278; continue;
        }
        574 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000838290); }
            blk = 578; continue;
        }
        575 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x7000838296); }
            blk = 578; continue;
        }
        576 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700083829c); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 578; continue; }
            blk = 577; continue;
        }
        577 => {
            let Some(t714) = c.ld64(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008382a6); };
            c.w64(9, t714);
            blk = 93; continue;
        }
        578 => {
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 580; continue; }
            blk = 579; continue;
        }
        579 => {
            let r715 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r715);
            blk = 589; continue;
        }
        580 => {
            let Some(t716) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008382bd); };
            c.w64(11, t716);
            let Some(t717) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008382c4); };
            c.w64(15, t717);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t718) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008382cb); };
            c.w64(3, t718);
            let Some(t719) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008382d2); };
            c.cmp(t719, c.r64(0), 64);
            if c.cc_a() { blk = 583; continue; }
            blk = 581; continue;
        }
        581 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008382e1, 0x70008382dc);
        }
        582 => {
            let Some(t720) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008382e1); };
            c.w64(8, t720);
            let Some(t721) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008382e5); };
            c.w64(9, t721);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 584; continue;
        }
        583 => {
            if !c.st64(c.r64(11), c.r64(0)) { return RegfileExit::fault(0x70008382ee); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 584; continue;
        }
        584 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(8) as i64));
            c.logic(c.r64(8) & c.r64(8), 64);
            if c.cc_ns() { blk = 588; continue; }
            blk = 585; continue;
        }
        585 => {
            c.w64(10, c.r64(8));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 587; continue; }
            blk = 586; continue;
        }
        586 => {
            let r722 = c.r64(10) | 0x1u64;
            c.w64(10, r722);
            c.logic(r722, 64);
            blk = 587; continue;
        }
        587 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 588; continue;
        }
        588 => {
            let Some(t723) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083831f); };
            c.w64(8, t723);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000838326); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083832a); }
            c.w64(8, c.r64(0));
            blk = 589; continue;
        }
        589 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000838332); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000838334); }
            let Some(t724) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x7000838336); };
            c.w64(3, t724);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(12));
            return c.call(0x700086ed74, 0x700083834a, 0x7000838345);
        }
        590 => {
            let r725 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r725);
            c.w64(8, c.r64(0));
            blk = 96; continue;
        }
        591 => {
            let Some(t726) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000838354); };
            c.w64(9, t726);
            let Some(t727) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083835b); };
            c.w64(11, t727);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x7000838362); }
            let Some(t728) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000838366); };
            c.w64(15, t728);
            let Some(t729) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083836d); };
            c.cmp(t729, c.r64(12), 64);
            if c.cc_a() { blk = 594; continue; }
            blk = 592; continue;
        }
        592 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x700083837c, 0x7000838377);
        }
        593 => {
            blk = 595; continue;
        }
        594 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083837e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 595; continue;
        }
        595 => {
            let Some(t730) = c.ld64(c.r64(13).wrapping_add(0x2f0)) else { return RegfileExit::fault(0x7000838388); };
            c.w64(8, t730);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083838f); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x7000838393); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700083839a); }
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008383a1); };
            c.w64(8, t731);
            if !c.st16(c.r64(0).wrapping_add(0xb), c.r16(8)) { return RegfileExit::fault(0x70008383a5); }
            blk = 15; continue;
        }
        596 => {
            let Some(t732) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008383af); };
            c.w64(12, t732);
            let Some(t733) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008383b6); };
            c.w64(15, t733);
            c.w64(0, c.r64(15).wrapping_add(0x10));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008383be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x70008383c2); }
            let Some(t734) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008383c6); };
            c.w64(3, t734);
            let Some(t735) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008383cd); };
            c.cmp(t735, c.r64(0), 64);
            if c.cc_a() { blk = 599; continue; }
            blk = 597; continue;
        }
        597 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febf4, 0x70008383dc, 0x70008383d7);
        }
        598 => {
            blk = 600; continue;
        }
        599 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x70008383de); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 600; continue;
        }
        600 => {
            let Some(t736) = c.ld64(c.r64(13).wrapping_add(0x100)) else { return RegfileExit::fault(0x70008383e9); };
            c.w64(8, t736);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008383f0); }
            if !c.st32(c.r64(0).wrapping_add(0x7), 0x1u64) { return RegfileExit::fault(0x70008383f4); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008383fb); }
            let Some(t737) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000838402); };
            c.w64(8, t737);
            if !c.st8(c.r64(0).wrapping_add(0xb), c.r8(8)) { return RegfileExit::fault(0x7000838406); }
            let Some(t738) = c.ld64(c.r64(13).wrapping_add(0x1028)) else { return RegfileExit::fault(0x700083840a); };
            c.w64(11, t738);
            let Some(t739) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838411); };
            c.w32(9, t739);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_b() { blk = 602; continue; }
            blk = 601; continue;
        }
        601 => {
            return RegfileExit::fault(0x7000838420);
        }
        602 => {
            let Some(t740) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838421); };
            c.w64(9, t740);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000838425); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 604; continue; }
            blk = 603; continue;
        }
        603 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r741 = c.r64(8) & c.r64(11);
            c.w64(8, r741);
            c.logic(r741, 64);
            let Some(t742) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000838437); };
            c.logic(t742 & 0x4u64, 8);
            if c.cc_ne() { blk = 648; continue; }
            blk = 604; continue;
        }
        604 => {
            blk = 15; continue;
        }
        605 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 606; continue;
        }
        606 => {
            let Some(t743) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000838451); };
            c.w64(8, t743);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t744) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083845b); };
            c.cmp(t744, c.r32(12), 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 607; continue;
        }
        607 => {
            let Some(t745) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000838465); };
            c.w64(0, t745);
            blk = 15; continue;
        }
        608 => {
            let Some(t746) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838471); };
            c.w64(2, t746);
            let Some(t747) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000838475); };
            c.w64(1, t747);
            let Some(t748) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000838479); };
            c.w64(0, t748);
            let Some(t749) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083847d); };
            c.w64(3, t749);
            let Some(t750) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000838481); };
            c.w64(6, t750);
            blk = 278; continue;
        }
        609 => {
            return RegfileExit::fault(0x700083848a);
        }
        610 => {
            blk = 278; continue;
        }
        611 => {
            let Some(t751) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838490); };
            c.w64(2, t751);
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000838494); };
            c.w64(1, t752);
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000838498); };
            c.w64(0, t753);
            let Some(t754) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083849c); };
            c.w64(3, t754);
            let Some(t755) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008384a0); };
            c.w64(6, t755);
            blk = 278; continue;
        }
        612 => {
            return RegfileExit::fault(0x70008384a9);
        }
        613 => {
            blk = 278; continue;
        }
        614 => {
            let Some(t756) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008384af); };
            c.w64(11, t756);
            blk = 615; continue;
        }
        615 => {
            c.logic(c.r8(11) & 0x40u64, 8);
            if c.cc_e() { blk = 278; continue; }
            blk = 616; continue;
        }
        616 => {
            c.w64(11, c.r64(9));
            let Some(t757) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008384c0); };
            c.w32(8, t757);
            let r758 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r758);
            let Some(t759) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008384c7); };
            c.cmp(t759, 0x82eu64, 16);
            if c.cc_e() { blk = 625; continue; }
            blk = 617; continue;
        }
        617 => {
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_a() { blk = 278; continue; }
            blk = 618; continue;
        }
        618 => {
            let Some(t760) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008384df); };
            c.w32(8, t760);
            let r761 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r761);
            let Some(t762) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008384e6); };
            c.cmp(t762, c.r32(8), 32);
            if c.cc_e() { blk = 625; continue; }
            blk = 619; continue;
        }
        619 => {
            let Some(t763) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008384ef); };
            c.w32(9, t763);
            let r764 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r764);
            let Some(t765) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008384f6); };
            c.w32(11, t765);
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_a() { blk = 622; continue; }
            blk = 620; continue;
        }
        620 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 278; continue; }
            blk = 621; continue;
        }
        621 => {
            let Some(t766) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083850f); };
            c.w32(11, t766);
            let Some(t767) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000838513); };
            c.cmp(t767, c.r32(11), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 622; continue;
        }
        622 => {
            let Some(t768) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838520); };
            c.w32(8, t768);
            let Some(t769) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000838524); };
            c.cmp(t769, c.r32(8), 32);
            if c.cc_e() { blk = 624; continue; }
            blk = 623; continue;
        }
        623 => {
            let Some(t770) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x700083852d); };
            c.cmp(t770, c.r32(8), 32);
            if c.cc_ne() { blk = 278; continue; }
            blk = 624; continue;
        }
        624 => {
            c.w64(8, c.r64(9));
            blk = 618; continue;
        }
        625 => {
            let Some(t771) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083853f); };
            c.w64(0, t771);
            blk = 15; continue;
        }
        626 => {
            let Some(t772) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083854b); };
            c.w64(9, t772);
            let Some(t773) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000838552); };
            c.w64(11, t773);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000838559); }
            let Some(t774) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083855d); };
            c.w64(15, t774);
            let Some(t775) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000838564); };
            c.cmp(t775, c.r64(12), 64);
            if c.cc_a() { blk = 629; continue; }
            blk = 627; continue;
        }
        627 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000838573, 0x700083856e);
        }
        628 => {
            let Some(t776) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838573); };
            c.w64(8, t776);
            blk = 630; continue;
        }
        629 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000838579); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 630; continue;
        }
        630 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t777) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083858f); };
            c.w64(8, t777);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000838596); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083859a); }
            blk = 15; continue;
        }
        631 => {
            let Some(t778) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008385a4); };
            c.w64(9, t778);
            let Some(t779) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008385ab); };
            c.w64(11, t779);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x70008385b2); }
            let Some(t780) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008385b6); };
            c.w64(15, t780);
            let Some(t781) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008385bd); };
            c.cmp(t781, c.r64(12), 64);
            if c.cc_a() { blk = 634; continue; }
            blk = 632; continue;
        }
        632 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008385cc, 0x70008385c7);
        }
        633 => {
            let Some(t782) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008385cc); };
            c.w64(8, t782);
            blk = 635; continue;
        }
        634 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008385d2); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 635; continue;
        }
        635 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t783) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008385e5); };
            c.w64(8, t783);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008385ec); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008385f0); }
            blk = 15; continue;
        }
        636 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008385ff); }
            return c.call(0x70007febf4, 0x7000838609, 0x7000838604);
        }
        637 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700083860c); }
            blk = 259; continue;
        }
        638 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083861b); }
            return c.call(0x70007febb4, 0x7000838624, 0x700083861f);
        }
        639 => {
            c.w64(9, c.r64(0));
            let Some(t784) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838627); };
            c.w64(8, t784);
            blk = 264; continue;
        }
        640 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700083863a, 0x7000838635);
        }
        641 => {
            c.w64(8, c.r64(0));
            blk = 269; continue;
        }
        642 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x7000838647); }
            return c.call(0x70007febb4, 0x7000838650, 0x700083864b);
        }
        643 => {
            c.w64(9, c.r64(0));
            let Some(t785) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838653); };
            c.w64(8, t785);
            blk = 274; continue;
        }
        644 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x7000838666, 0x7000838661);
        }
        645 => {
            c.w64(8, c.r64(0));
            blk = 277; continue;
        }
        646 => {
            let Some(t786) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083866e); };
            c.w64(5, t786);
            let r787 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r787);
            return RegfileExit::jump(0x70008386d4);
        }
        647 => {
            blk = 278; continue;
        }
        648 => {
            c.w32(0, c.r32(0));
            let r788 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r788);
            c.w64(8, 0xfffffffffffc0000u64);
            let r789 = c.r64(8) & c.r64(0);
            c.w64(8, r789);
            c.logic(r789, 64);
            let Some(t790) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083868f); };
            c.logic(t790 & 0x2u64, 8);
            if c.cc_e() { blk = 604; continue; }
            blk = 649; continue;
        }
        649 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083869e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083869f); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008386ac, 0x70008386a7);
        }
        650 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008386ac); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008386ae); }
            blk = 604; continue;
        }
        651 => {
            return RegfileExit::jump(0x70008386b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008386d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008386d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008386d8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008386de); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008386e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008386e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008386ea); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 988; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008386f7); };
            c.cmp(t1, c.r32(2), 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000838706); };
            c.cmp(t2, c.r32(2), 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r3 = c.r32(8) | c.r32(9);
            c.w32(8, r3);
            c.logic(r3, 32);
            if c.cc_ne() { blk = 987; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 476; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838727); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083872e); };
            c.w32(8, t6);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700083873a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(11, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r7);
            c.w64(12, 0x3ffffffffffffeu64);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 477; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 214; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 656; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083878b); };
            c.w32(9, t8);
            c.logic(c.r32(9) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 213; continue; }
            blk = 10; continue;
        }
        10 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 658; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r16(8), 0x25u64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(8), 0x2du64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(8, c.r64(1));
            blk = 215; continue;
        }
        15 => {
            let Some(t9) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008387c2); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            blk = 215; continue;
        }
        16 => {
            c.w64(10, c.r64(4));
            let r11 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            let r12 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r12);
            c.logic(r12, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008387d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008387de); }
            c.w64(2, c.r64(1));
            c.w64(1, c.r64(13));
            c.w64(8, c.r64(1));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x70008387eb); };
            c.w64(0, t13);
            c.w64(10, 0x70008387f9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008387f9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008387fd); }
            return c.call(c.r64(0), 0x7000838803, 0x7000838801);
        }
        17 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000838803); }
            let Some(t14) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083880b); };
            c.w64(4, t14);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 677; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(8), 0xfffffffffffffffeu64, 64);
            if c.cc_e() { blk = 676; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083882e); };
            c.w64(1, t15);
            c.w64(9, c.r64(8));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838835); };
            c.w64(2, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000838839); };
            c.w64(6, t17);
            blk = 477; continue;
        }
        21 => {
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838842); };
            c.w64(8, t18);
            let Some(t19) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838846); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            let Some(t21) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083884d); };
            c.w32(11, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000838852); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 208; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838862); };
            c.w32(12, t22);
            c.logic(c.r32(12) & 0x200000u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000838873); };
            c.w32(15, t23);
            let r24 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r24);
            c.logic(c.r32(12) & 0xffc00u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 24; continue;
        }
        24 => {
            let r25 = c.shr(c.r32(12), 0xau64, 32).unwrap_or(c.r32(12));
            c.w32(12, r25);
            let r26 = c.r32(12) & 0x3ffu64;
            c.w32(12, r26);
            c.logic(r26, 32);
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_be() { blk = 37; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t27) = c.ld16(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x700083889c); };
            c.w32(3, (t27 as u16 as i16 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008388a1); };
            c.w32(2, t28);
            let r29 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r29);
            let r30 = c.sub(c.r32(3), 0x1u64, 32);
            c.w32(3, r30);
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(3));
            c.w64(6, c.r64(1));
            blk = 27; continue;
        }
        26 => {
            c.w64(7, c.r64(11));
            blk = 27; continue;
        }
        27 => {
            c.w64(11, c.r64(7));
            let r31 = c.sub(c.r32(11), c.r32(1), 32);
            c.w32(11, r31);
            let r32 = c.shr(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r32);
            let r33 = c.add(c.r32(11), c.r32(1), 32);
            c.w32(11, r33);
            c.w32(6, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008388ca); };
            c.w32(6, t34);
            let r35 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r35);
            let r36 = c.shr(c.r32(6), 0x9u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r36);
            let r37 = c.r32(6) & 0x3ffu64;
            c.w32(6, r37);
            c.logic(r37, 32);
            c.w32(6, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008388e0); };
            c.w32(6, t38);
            let r39 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r39);
            let Some(t40) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008388e8); };
            c.w32(6, t40);
            let r41 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r41);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_be() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(1, c.r64(11).wrapping_add(0x1));
            c.w64(11, c.r64(7));
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 32; continue;
        }
        31 => {
            let r42 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r42);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_l() { blk = 103; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(11, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000838912); };
            c.w32(11, t43);
            let r44 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r44);
            let r45 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r45);
            let r46 = c.r32(11) & 0x3ffu64;
            c.w32(11, r46);
            c.logic(r46, 32);
            c.w32(6, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t47) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083892c); };
            c.w32(7, t47);
            let r48 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r48);
            let Some(t49) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838934); };
            c.w32(8, t49);
            let r50 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r50);
            c.cmp(c.r32(2), c.r32(8), 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_be() { blk = 103; continue; }
            blk = 36; continue;
        }
        36 => {
            blk = 41; continue;
        }
        37 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r51 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(6, r51);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 103; continue; }
            blk = 38; continue;
        }
        38 => {
            let r52 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r52);
            let Some(t53) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838969); };
            c.cmp(t53, c.r32(0), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 103; continue;
        }
        41 => {
            c.w64(8, c.r64(6).wrapping_mul(4));
            let Some(t54) = c.ld32(c.r64(15).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000838983); };
            c.w32(11, t54);
            c.w32(12, c.r32(11));
            c.w64(3, c.r64(11));
            let r55 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r55);
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t56) = c.ld32(c.r64(15).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000838996); };
            c.w32(8, t56);
            let r57 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083899e); };
            c.w64(12, t58);
            blk = 54; continue;
        }
        43 => {
            c.w64(8, c.r64(3));
            if let Some(sr) = c.shr(c.r64(8), 0x13u64, 64) { c.w64(8, sr); }
            let Some(t59) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008389ae); };
            c.w32(11, t59);
            let r60 = c.r64(8) & 0x3ffu64;
            c.w64(8, r60);
            c.logic(r60, 64);
            let r61 = c.r32(12) & 0x380u64;
            c.w32(12, r61);
            c.logic(r61, 32);
            let r62 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r62);
            c.cmp(c.r32(12), 0x100u64, 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t63) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008389d3); };
            c.w32(9, t63);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_b() { blk = 48; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008389dd); };
            c.w64(12, t64);
            let Some(t65) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008389e1); };
            c.w32(15, t65);
            let r66 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r66);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008389ef); };
            c.w64(15, t67);
            blk = 46; continue;
        }
        46 => {
            let r68 = c.sub(c.r64(8), c.r64(9), 64);
            c.w64(8, r68);
            let Some(t69) = c.ld32(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008389f9); };
            c.w32(8, t69);
            let r70 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r70);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000838a07); }
            blk = 50; continue;
        }
        48 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t71) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838a15); };
            c.w64(12, t71);
            let Some(t72) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838a19); };
            c.w32(8, t72);
            let r73 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r73);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000838a21); }
            blk = 50; continue;
        }
        50 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000838a27); };
            c.w64(8, t74);
            let Some(t75) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000838a2e); };
            c.w64(9, t75);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000838a35); };
            c.w64(15, t76);
            let Some(t77) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000838a3c); };
            c.cmp(t77, c.r64(11), 64);
            if c.cc_be() { blk = 678; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000838a45); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 52; continue;
        }
        52 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000838a4c); };
            c.w64(9, t78);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000838a53); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000838a57); }
            blk = 54; continue;
        }
        53 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000838a5f); };
            c.w64(12, t79);
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838a63); };
            c.w32(8, t80);
            let r81 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r81);
            blk = 54; continue;
        }
        54 => {
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(12));
            blk = 63; continue;
        }
        55 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838a76); };
            c.w32(12, t82);
            let r83 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r83);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000838a83); };
            c.w64(12, t84);
            blk = 57; continue;
        }
        57 => {
            let Some(t85) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838a8a); };
            c.w32(15, t85);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r86 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r86);
            let Some(t87) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838a99); };
            c.w32(3, t87);
            let r88 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r88);
            let r89 = c.r64(3) & c.r64(15);
            c.w64(3, r89);
            c.logic(r89, 64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 59; continue;
        }
        58 => {
            let r90 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r90);
            let r91 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r91);
            c.w64(1, c.r64(15));
            let r92 = c.r64(1) & c.r64(3);
            c.w64(1, r92);
            c.logic(r92, 64);
            c.w64(3, c.r64(1));
            blk = 59; continue;
        }
        59 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(1).wrapping_mul(4));
            let Some(t93) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838ac3); };
            c.w32(7, t93);
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000838ac8); };
            c.cmp(t94, c.r32(7), 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 60; continue;
        }
        60 => {
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t95) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838ad9); };
            c.w32(9, t95);
            let Some(t96) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838ade); };
            c.w32(11, t96);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ae() { blk = 680; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838af6); };
            c.w32(9, t97);
            let r98 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r98);
            let r99 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r99);
            c.w64(3, c.r64(11));
            blk = 63; continue;
        }
        63 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 102; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t100) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838b0d); };
            c.w32(11, t100);
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000838b11); };
            c.cmp(t101, c.r32(11), 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t102) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838b1e); };
            c.w32(2, t102);
            let r103 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r103);
            let Some(t104) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838b25); };
            c.w32(9, t104);
            let r105 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r105);
            let Some(t106) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000838b2c); };
            c.logic(t106 & 0x2u64, 8);
            if c.cc_ne() { blk = 83; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t107) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000838b37); };
            c.cmp(t107, c.r32(9), 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000838b40); };
            c.w64(0, t108);
            blk = 68; continue;
        }
        68 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000838b4a); }
            return c.ret(0x0, 0x7000838b4b);
        }
        69 => {
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000838b4c); };
            c.w64(9, t109);
            let Some(t110) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x7000838b53); };
            c.cmp(t110, 0x20u64, 8);
            if c.cc_e() { blk = 684; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t111) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000838b5d); };
            c.w32(9, t111);
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000838b61); };
            c.cmp(t112, c.r32(9), 32);
            if c.cc_ne() { blk = 681; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t113) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838b6e); };
            c.w32(9, t113);
            let r114 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r114);
            let Some(t115) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838b75); };
            c.w32(11, t115);
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000838b79); };
            c.cmp(t116, c.r32(11), 32);
            if c.cc_ne() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t117) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000838b82); };
            c.w32(9, t117);
            let r118 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r118);
            let Some(t119) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838b89); };
            c.w32(11, t119);
            let Some(t120) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000838b8d); };
            c.cmp(t120, c.r32(11), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 683; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t121) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838ba0); };
            c.w32(11, t121);
            let r122 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r122);
            let Some(t123) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838ba7); };
            c.w32(12, t123);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 79; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t124) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000838bc7); };
            c.cmp(t124, c.r32(11), 32);
            if c.cc_e() { blk = 682; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(8));
            blk = 79; continue;
        }
        78 => {
            let Some(t125) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838bd9); };
            c.w32(9, t125);
            let r126 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r126);
            blk = 79; continue;
        }
        79 => {
            let Some(t127) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838be0); };
            c.w32(9, t127);
            let r128 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r128);
            let Some(t129) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838be7); };
            c.w32(9, t129);
            let r130 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r130);
            let Some(t131) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838bee); };
            c.w32(11, t131);
            let r132 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r132);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t133) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000838c00); };
            c.w32(6, t133);
            let r134 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r134);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000838c07); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000838c10, 0x7000838c0b);
        }
        81 => {
            blk = 68; continue;
        }
        82 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        83 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000838c1d); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000838c24); };
            c.w64(6, t135);
            return c.call(0x70007f2274, 0x7000838c2d, 0x7000838c28);
        }
        84 => {
            blk = 68; continue;
        }
        85 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838c32); };
            c.w32(11, t136);
            let r137 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r137);
            let Some(t138) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838c39); };
            c.w32(12, t138);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 100; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 92; continue; }
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 686; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t139) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838c64); };
            c.w32(9, t139);
            let Some(t140) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000838c68); };
            c.cmp(t140, c.r32(9), 32);
            if c.cc_ne() { blk = 687; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t141) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838c75); };
            c.w32(9, t141);
            let r142 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r142);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 688; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t143) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838c86); };
            c.w32(11, t143);
            let r144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r144);
            let Some(t145) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838c8d); };
            c.cmp(t145, 0x40u64, 16);
            if c.cc_ae() { blk = 689; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t146) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838c99); };
            c.w32(8, t146);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        92 => {
            let Some(t147) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838ca6); };
            c.w32(9, t147);
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000838caa); };
            c.cmp(t148, c.r32(9), 32);
            if c.cc_ne() { blk = 690; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t149) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000838cb7); };
            c.w32(9, t149);
            c.w64(11, c.r64(9));
            let r150 = c.r32(11) & 0xc0u64;
            c.w32(11, r150);
            c.logic(r150, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838ccf); };
            c.w32(11, t151);
            let r152 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r152);
            let Some(t153) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000838cd6); };
            c.w32(11, t153);
            let r154 = c.r32(11) & 0x1fu64;
            c.w32(11, r154);
            c.logic(r154, 32);
            let r155 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r155);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 691; continue; }
            blk = 95; continue;
        }
        95 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 693; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000838cf6); };
            c.w32(0, t156);
            let r157 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r157);
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000838cfd); };
            c.cmp(t158, c.r32(0), 32);
            if c.cc_e() { blk = 692; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t159) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838d0a); };
            c.w32(8, t159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000838d0e); };
            c.cmp(t160, c.r32(8), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            blk = 68; continue;
        }
        99 => {
            let Some(t161) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838d1c); };
            c.w32(0, t161);
            let r162 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r162);
            blk = 68; continue;
        }
        100 => {
            let Some(t163) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838d27); };
            c.w32(9, t163);
            let Some(t164) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000838d2b); };
            c.cmp(t164, c.r32(9), 32);
            if c.cc_ne() { blk = 694; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t165) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838d38); };
            c.w32(0, t165);
            let r166 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r166);
            blk = 68; continue;
        }
        102 => {
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        103 => {
            let Some(t167) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838d4c); };
            c.w32(8, t167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000838d50); };
            c.cmp(t168, c.r32(8), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t169) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838d59); };
            c.w32(8, t169);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 105; continue;
        }
        105 => {
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 695; continue; }
            blk = 106; continue;
        }
        106 => {
            blk = 207; continue;
        }
        107 => {
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000838d72); };
            c.w64(11, t170);
            blk = 108; continue;
        }
        108 => {
            c.cmp(c.r32(11), 0x82eu64, 32);
            if c.cc_e() { blk = 704; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t171) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838d83); };
            c.w32(8, t171);
            let r172 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r172);
            let Some(t173) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000838d8a); };
            c.cmp(t173, c.r32(8), 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t174) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000838d97); };
            c.w32(9, t174);
            let r175 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r175);
            let Some(t176) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838d9e); };
            c.w32(11, t176);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000838da3); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 139; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t177) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838db3); };
            c.w32(12, t177);
            c.logic(c.r32(12) & 0x200000u64, 32);
            if c.cc_ne() { blk = 132; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t178) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000838dc4); };
            c.w32(15, t178);
            let r179 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r179);
            c.logic(c.r32(12) & 0xffc00u64, 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 113; continue;
        }
        113 => {
            let r180 = c.shr(c.r32(12), 0xau64, 32).unwrap_or(c.r32(12));
            c.w32(12, r180);
            let r181 = c.r32(12) & 0x3ffu64;
            c.w32(12, r181);
            c.logic(r181, 32);
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_be() { blk = 128; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t182) = c.ld16(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000838de9); };
            c.w32(3, (t182 as u16 as i16 as i64 as u64));
            let Some(t183) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838dee); };
            c.w32(2, t183);
            let r184 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r184);
            let r185 = c.sub(c.r32(3), 0x1u64, 32);
            c.w32(3, r185);
            c.w64(1, c.r64(3));
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 116; continue;
        }
        115 => {
            c.w64(1, c.r64(7));
            blk = 116; continue;
        }
        116 => {
            c.w64(7, c.r64(1));
            let r186 = c.sub(c.r32(7), c.r32(6), 32);
            c.w32(7, r186);
            let r187 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r187);
            let r188 = c.add(c.r32(7), c.r32(6), 32);
            c.w32(7, r188);
            c.w32(9, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t189) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000838e11); };
            c.w32(9, t189);
            let r190 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r190);
            let r191 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r191);
            let r192 = c.r32(9) & 0x3ffu64;
            c.w32(9, r192);
            c.logic(r192, 32);
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t193) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838e2b); };
            c.w32(9, t193);
            let r194 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r194);
            let Some(t195) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838e33); };
            c.w32(9, t195);
            let r196 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r196);
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_be() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w32(6, c.r64(7).wrapping_add(0x1));
            c.w64(7, c.r64(1));
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 119; continue;
        }
        119 => {
            blk = 121; continue;
        }
        120 => {
            let r197 = c.add(c.r32(6), 0x1u64, 32);
            c.w32(6, r197);
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(3), c.r32(6), 32);
            if c.cc_l() { blk = 127; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w32(9, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t198) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000838e5a); };
            c.w32(9, t198);
            let r199 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r199);
            let r200 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r200);
            let r201 = c.r32(9) & 0x3ffu64;
            c.w32(9, r201);
            c.logic(r201, 32);
            c.w32(1, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t202) = c.ld32(c.r64(15).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838e74); };
            c.w32(7, t202);
            let r203 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r203);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x7000838e7c); }
            let Some(t204) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838e80); };
            c.w32(1, t204);
            let r205 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r205);
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_ne() { blk = 126; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_ne() { blk = 120; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_a() { blk = 151; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000838e97); };
            c.w64(9, t206);
            blk = 108; continue;
        }
        126 => {
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000838ea0); };
            c.w64(9, t207);
            blk = 108; continue;
        }
        127 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000838ea9); };
            c.w64(9, t208);
            blk = 108; continue;
        }
        128 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r209 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(1, r209);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 108; continue; }
            blk = 129; continue;
        }
        129 => {
            let r210 = c.sub(c.r64(1), 0x3u64, 64);
            c.w64(1, r210);
            let Some(t211) = c.ld32(c.r64(15).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838ec7); };
            c.cmp(t211, c.r32(0), 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ne() { blk = 129; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 108; continue;
        }
        132 => {
            let Some(t212) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838edd); };
            c.w32(12, t212);
            let r213 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r213);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000838eea); };
            c.w64(12, t214);
            blk = 134; continue;
        }
        134 => {
            let Some(t215) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838ef1); };
            c.w32(15, t215);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r216 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r216);
            let Some(t217) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838f00); };
            c.w32(3, t217);
            let r218 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r218);
            let r219 = c.r64(3) & c.r64(15);
            c.w64(3, r219);
            c.logic(r219, 64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 136; continue;
        }
        135 => {
            let r220 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r220);
            let r221 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r221);
            c.w64(1, c.r64(15));
            let r222 = c.r64(1) & c.r64(3);
            c.w64(1, r222);
            c.logic(r222, 64);
            c.w64(3, c.r64(1));
            blk = 136; continue;
        }
        136 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(1).wrapping_mul(4));
            let Some(t223) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838f2a); };
            c.w32(7, t223);
            let Some(t224) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000838f2f); };
            c.cmp(t224, c.r32(7), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 138; continue;
        }
        138 => {
            blk = 135; continue;
        }
        139 => {
            c.cmp(c.r16(11), 0xbdu64, 16);
            if c.cc_ne() { blk = 206; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t225) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000838f52); };
            c.logic(t225 & 0x24u64, 8);
            if c.cc_ne() { blk = 206; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t226) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838f5d); };
            c.w32(12, t226);
            let r227 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r227);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000838f6a); };
            c.w64(12, t228);
            blk = 143; continue;
        }
        143 => {
            let Some(t229) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000838f71); };
            c.w32(15, t229);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r230 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r230);
            let Some(t231) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838f80); };
            c.w32(3, t231);
            let r232 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r232);
            let r233 = c.r64(3) & c.r64(15);
            c.w64(3, r233);
            c.logic(r233, 64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 145; continue;
        }
        144 => {
            let r234 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r234);
            let r235 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r235);
            c.w64(1, c.r64(15));
            let r236 = c.r64(1) & c.r64(3);
            c.w64(1, r236);
            c.logic(r236, 64);
            c.w64(3, c.r64(1));
            blk = 145; continue;
        }
        145 => {
            c.w64(1, c.r64(3).wrapping_add(0x5));
            let Some(t237) = c.ld32(c.r64(12).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000838fa1); };
            c.w32(6, t237);
            let r238 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000838fa9); };
            c.cmp(t239, c.r32(6), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t240) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000838fb6); };
            c.cmp(t240, c.r32(6), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 147; continue;
        }
        147 => {
            let Some(t241) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838fbf); };
            c.cmp(t241, c.r32(0), 32);
            if c.cc_ne() { blk = 144; continue; }
            blk = 148; continue;
        }
        148 => {
            let Some(t242) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000838fc4); };
            c.w32(12, t242);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(1), c.r64(12), 64);
            if c.cc_ae() { blk = 700; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t243) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000838fd8); };
            c.w32(12, t243);
            let r244 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r244);
            let Some(t245) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000838fdf); };
            c.cmp(t245, c.r32(12), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 167; continue;
        }
        151 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000838ff1); };
            c.w64(1, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000838ff5); };
            c.w64(9, t247);
            blk = 152; continue;
        }
        152 => {
            c.w64(11, c.r64(1).wrapping_mul(4));
            let Some(t248) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839001); };
            c.w32(12, t248);
            c.w32(3, c.r32(12));
            c.w64(2, c.r64(12));
            let r249 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r249);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t250) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000839014); };
            c.w32(12, t250);
            let r251 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r251);
            c.w64(9, c.r64(12));
            blk = 168; continue;
        }
        154 => {
            c.w64(11, c.r64(2));
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t252) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x700083902b); };
            c.w32(12, t252);
            let r253 = c.r64(11) & 0x3ffu64;
            c.w64(11, r253);
            c.logic(r253, 64);
            let r254 = c.r32(3) & 0x380u64;
            c.w32(3, r254);
            c.logic(r254, 32);
            let r255 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r255);
            c.cmp(c.r32(3), 0x100u64, 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let Some(t256) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083904e); };
            c.w32(9, t256);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 159; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t257) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839058); };
            c.w32(15, t257);
            let r258 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r258);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 157; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000839065); };
            c.w64(15, t259);
            blk = 157; continue;
        }
        157 => {
            let r260 = c.sub(c.r64(11), c.r64(9), 64);
            c.w64(11, r260);
            let Some(t261) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083906f); };
            c.w32(9, t261);
            let r262 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r262);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 168; continue; }
            blk = 158; continue;
        }
        158 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000839081); }
            blk = 161; continue;
        }
        159 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 164; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t263) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083908f); };
            c.w32(9, t263);
            let r264 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r264);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000839097); }
            blk = 161; continue;
        }
        161 => {
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083909d); };
            c.w64(9, t265);
            let Some(t266) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008390a4); };
            c.w64(11, t266);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t267) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008390ab); };
            c.w64(15, t267);
            let Some(t268) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008390b2); };
            c.cmp(t268, c.r64(12), 64);
            if c.cc_be() { blk = 696; continue; }
            blk = 162; continue;
        }
        162 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008390bb); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 163; continue;
        }
        163 => {
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008390c2); };
            c.w64(11, t269);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008390c9); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008390cd); }
            blk = 168; continue;
        }
        164 => {
            let Some(t270) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008390d5); };
            c.w32(12, t270);
            let r271 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r271);
            c.w64(9, c.r64(12));
            blk = 168; continue;
        }
        165 => {
            let Some(t272) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008390e2); };
            c.w32(9, t272);
            let Some(t273) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008390e7); };
            c.w32(11, t273);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(1).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ae() { blk = 698; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t274) = c.ld32(c.r64(12).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008390ff); };
            c.w32(12, t274);
            let r275 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r275);
            let r276 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r276);
            c.w64(9, c.r64(12));
            c.w64(2, c.r64(11));
            blk = 168; continue;
        }
        167 => {
            let Some(t277) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839112); };
            c.w32(9, t277);
            let r278 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r278);
            c.w64(2, c.r64(9));
            c.w64(9, c.r64(12));
            blk = 168; continue;
        }
        168 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t279) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839128); };
            c.w32(11, t279);
            let Some(t280) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083912c); };
            c.cmp(t280, c.r32(11), 32);
            if c.cc_ne() { blk = 189; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t281) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839139); };
            c.w32(2, t281);
            let r282 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r282);
            let Some(t283) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839140); };
            c.w32(9, t283);
            let r284 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r284);
            let Some(t285) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000839147); };
            c.logic(t285 & 0x2u64, 8);
            if c.cc_ne() { blk = 187; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t286) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000839152); };
            c.cmp(t286, c.r32(9), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t287) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083915b); };
            c.w64(0, t287);
            blk = 68; continue;
        }
        173 => {
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000839167); };
            c.w64(9, t288);
            let Some(t289) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x700083916e); };
            c.cmp(t289, 0x20u64, 8);
            if c.cc_e() { blk = 702; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t290) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000839178); };
            c.w32(9, t290);
            let Some(t291) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083917c); };
            c.cmp(t291, c.r32(9), 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t292) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839189); };
            c.w32(9, t292);
            let r293 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r293);
            let Some(t294) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839190); };
            c.w32(11, t294);
            let Some(t295) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000839194); };
            c.cmp(t295, c.r32(11), 32);
            if c.cc_ne() { blk = 177; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t296) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083919d); };
            c.w32(9, t296);
            let r297 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r297);
            let Some(t298) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008391a4); };
            c.w32(11, t298);
            let Some(t299) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008391a8); };
            c.cmp(t299, c.r32(11), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 177; continue;
        }
        177 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 206; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t300) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008391bb); };
            c.w32(11, t300);
            let r301 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r301);
            let Some(t302) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008391c2); };
            c.w32(12, t302);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 183; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t303) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008391e2); };
            c.cmp(t303, c.r32(11), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 181; continue;
        }
        181 => {
            blk = 184; continue;
        }
        182 => {
            let Some(t304) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008391f1); };
            c.w32(8, t304);
            let r305 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r305);
            blk = 184; continue;
        }
        183 => {
            c.w64(8, c.r64(9));
            blk = 184; continue;
        }
        184 => {
            let Some(t306) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008391fd); };
            c.w32(8, t306);
            let r307 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r307);
            let Some(t308) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839204); };
            c.w32(8, t308);
            let r309 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r309);
            let Some(t310) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083920b); };
            c.w32(9, t310);
            let r311 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r311);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 206; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t312) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839221); };
            c.w32(6, t312);
            let r313 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r313);
            let Some(t314) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839228); };
            if !c.push64(t314) { return RegfileExit::fault(0x7000839228); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x7000839232, 0x700083922d);
        }
        186 => {
            blk = 68; continue;
        }
        187 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839237); };
            if !c.push64(t315) { return RegfileExit::fault(0x7000839237); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083923f); };
            c.w64(6, t316);
            return c.call(0x70007f2274, 0x7000839248, 0x7000839243);
        }
        188 => {
            blk = 68; continue;
        }
        189 => {
            let Some(t317) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083924d); };
            c.w32(11, t317);
            let r318 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r318);
            let Some(t319) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839254); };
            c.w32(12, t319);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 203; continue; }
            blk = 190; continue;
        }
        190 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 196; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 206; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t320) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083927f); };
            c.w32(9, t320);
            let Some(t321) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000839283); };
            c.cmp(t321, c.r32(9), 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t322) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839290); };
            c.w32(8, t322);
            let r323 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r323);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 206; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t324) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008392a1); };
            c.w32(9, t324);
            let r325 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r325);
            let Some(t326) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008392a8); };
            c.cmp(t326, 0x40u64, 16);
            if c.cc_ae() { blk = 206; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t327) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008392b4); };
            c.w32(8, t327);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        196 => {
            let Some(t328) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008392c1); };
            c.w32(9, t328);
            let Some(t329) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008392c5); };
            c.cmp(t329, c.r32(9), 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t330) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008392d2); };
            c.w32(9, t330);
            c.w64(11, c.r64(9));
            let r331 = c.r32(11) & 0xc0u64;
            c.w32(11, r331);
            c.logic(r331, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t332) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008392ea); };
            c.w32(11, t332);
            let r333 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r333);
            let Some(t334) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008392f1); };
            c.w32(11, t334);
            let r335 = c.r32(11) & 0x1fu64;
            c.w32(11, r335);
            c.logic(r335, 32);
            let r336 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r336);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 206; continue; }
            blk = 199; continue;
        }
        199 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 206; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t337) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000839309); };
            c.w32(0, t337);
            let r338 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r338);
            let Some(t339) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000839310); };
            c.cmp(t339, c.r32(0), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t340) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839319); };
            c.w32(8, t340);
            let Some(t341) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083931d); };
            c.cmp(t341, c.r32(8), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t342) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083932a); };
            c.w32(0, t342);
            let r343 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r343);
            blk = 68; continue;
        }
        203 => {
            let Some(t344) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839335); };
            c.w32(9, t344);
            let Some(t345) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000839339); };
            c.cmp(t345, c.r32(9), 32);
            if c.cc_ne() { blk = 206; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t346) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839342); };
            c.w32(0, t346);
            let r347 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r347);
            blk = 68; continue;
        }
        205 => {
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        206 => {
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839356); };
            c.w64(2, t348);
            blk = 655; continue;
        }
        207 => {
            let Some(t349) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083935f); };
            c.w64(0, t349);
            blk = 68; continue;
        }
        208 => {
            c.cmp(c.r16(11), 0xbcu64, 16);
            if c.cc_ne() { blk = 705; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t350) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839377); };
            c.w32(9, t350);
            let Some(t351) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700083937b); };
            c.cmp(t351, c.r32(9), 32);
            if c.cc_ne() { blk = 212; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t352) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839384); };
            c.logic(t352 & 0x1u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        212 => {
            let Some(t353) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000839392); };
            c.w64(5, t353);
            c.w32(2, 0x2u64);
            c.w64(3, c.r64(0));
            c.w64(0, c.r64(8));
            c.w64(1, c.r64(8));
            let Some(t354) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008393a4); };
            c.w64(6, t354);
            let r355 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r355);
            return RegfileExit::jump(0x70008bc174);
        }
        213 => {
            if let Some(sr) = c.shr(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let r356 = c.r64(9) & 0xffffffu64;
            c.w64(9, r356);
            c.logic(r356, 64);
            blk = 477; continue;
        }
        214 => {
            c.w64(8, c.r64(1));
            blk = 215; continue;
        }
        215 => {
            let Some(t357) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008393c5); };
            c.w32(9, t357);
            let r358 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r358);
            let Some(t359) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008393cc); };
            c.w32(11, t359);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008393d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008393d5); }
            c.cmp(c.r16(11), 0x410u64, 16);
            if c.cc_be() { blk = 470; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t360) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008393e5); };
            c.w32(12, t360);
            c.logic(c.r32(12) & 0x200000u64, 32);
            if c.cc_ne() { blk = 319; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t361) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008393f6); };
            c.w32(15, t361);
            let r362 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r362);
            c.logic(c.r32(12) & 0xffc00u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 218; continue;
        }
        218 => {
            let r363 = c.shr(c.r32(12), 0xau64, 32).unwrap_or(c.r32(12));
            c.w32(12, r363);
            let r364 = c.r32(12) & 0x3ffu64;
            c.w32(12, r364);
            c.logic(r364, 32);
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_be() { blk = 231; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t365) = c.ld16(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x700083941f); };
            c.w32(7, (t365 as u16 as i16 as i64 as u64));
            let Some(t366) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839424); };
            c.w32(1, t366);
            let r367 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r367);
            let r368 = c.sub(c.r32(7), 0x1u64, 32);
            c.w32(7, r368);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700083942e); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 221; continue;
        }
        220 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(6)) { return RegfileExit::fault(0x7000839436); }
            blk = 221; continue;
        }
        221 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083943a); };
            c.w64(6, t369);
            let r370 = c.sub(c.r32(6), c.r32(0), 32);
            c.w32(6, r370);
            let r371 = c.shr(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r371);
            let r372 = c.add(c.r32(6), c.r32(0), 32);
            c.w32(6, r372);
            c.w32(11, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t373) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083944b); };
            c.w32(11, t373);
            let r374 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r374);
            let r375 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r375);
            let r376 = c.r32(11) & 0x3ffu64;
            c.w32(11, r376);
            c.logic(r376, 32);
            c.w32(11, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t377) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839465); };
            c.w32(11, t377);
            let r378 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r378);
            let Some(t379) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083946d); };
            c.w32(11, t379);
            let r380 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r380);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_be() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(0, c.r64(6).wrapping_add(0x1));
            let Some(t381) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083947d); };
            c.w64(6, t381);
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r32(0), c.r32(6), 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 224; continue;
        }
        224 => {
            blk = 226; continue;
        }
        225 => {
            let r382 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r382);
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_l() { blk = 249; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t383) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839499); };
            c.w32(11, t383);
            let r384 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r384);
            let r385 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r385);
            let r386 = c.r32(11) & 0x3ffu64;
            c.w32(11, r386);
            c.logic(r386, 32);
            c.w32(6, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t387) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008394b3); };
            c.w32(3, t387);
            let r388 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r388);
            let Some(t389) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008394bb); };
            c.w32(2, t389);
            let r390 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r390);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_ne() { blk = 249; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 229; continue;
        }
        229 => {
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_be() { blk = 249; continue; }
            blk = 230; continue;
        }
        230 => {
            c.w64(12, c.r64(6));
            blk = 235; continue;
        }
        231 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let r391 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r391);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 249; continue; }
            blk = 232; continue;
        }
        232 => {
            let r392 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r392);
            let Some(t393) = c.ld32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008394f1); };
            c.cmp(t393, c.r32(8), 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 232; continue; }
            blk = 234; continue;
        }
        234 => {
            blk = 249; continue;
        }
        235 => {
            c.w64(11, c.r64(12).wrapping_mul(4));
            let Some(t394) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083950b); };
            c.w32(12, t394);
            c.w32(0, c.r32(12));
            c.w64(3, c.r64(12));
            let r395 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r395);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t396) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083951e); };
            c.w32(9, t396);
            let r397 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839526); };
            c.w64(2, t398);
            blk = 248; continue;
        }
        237 => {
            c.w64(11, c.r64(3));
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t399) = c.ld8(c.r64(9).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000839536); };
            c.w32(12, t399);
            let r400 = c.r64(11) & 0x3ffu64;
            c.w64(11, r400);
            c.logic(r400, 64);
            let r401 = c.r32(0) & 0x380u64;
            c.w32(0, r401);
            c.logic(r401, 32);
            let r402 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r402);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(12, (c.cc_ne() as u64));
            c.w32(12, c.r8(12));
            let Some(t403) = c.ld8(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839557); };
            c.w32(9, t403);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 242; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839561); };
            c.w64(2, t404);
            let Some(t405) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839565); };
            c.w32(15, t405);
            let r406 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r406);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 240; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t407) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000839572); };
            c.w64(15, t407);
            blk = 240; continue;
        }
        240 => {
            let r408 = c.sub(c.r64(11), c.r64(9), 64);
            c.w64(11, r408);
            let Some(t409) = c.ld32(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083957c); };
            c.w32(9, t409);
            let r410 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r410);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 248; continue; }
            blk = 241; continue;
        }
        241 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700083958a); }
            blk = 244; continue;
        }
        242 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 247; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839598); };
            c.w64(2, t411);
            let Some(t412) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083959c); };
            c.w32(9, t412);
            let r413 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r413);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008395a4); }
            blk = 244; continue;
        }
        244 => {
            let Some(t414) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008395aa); };
            c.w64(9, t414);
            let Some(t415) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008395b1); };
            c.w64(11, t415);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008395b8); };
            c.w64(15, t416);
            let Some(t417) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008395bf); };
            c.cmp(t417, c.r64(12), 64);
            if c.cc_be() { blk = 712; continue; }
            blk = 245; continue;
        }
        245 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008395c8); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 246; continue;
        }
        246 => {
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008395cf); };
            c.w64(11, t418);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008395d6); }
            if !c.movsd_mr(c.r64(9).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008395da); }
            blk = 248; continue;
        }
        247 => {
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008395e2); };
            c.w64(2, t419);
            let Some(t420) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008395e6); };
            c.w32(9, t420);
            let r421 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r421);
            blk = 248; continue;
        }
        248 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008395ee); };
            c.w64(6, t422);
            blk = 327; continue;
        }
        249 => {
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008395f7); };
            c.w64(3, t423);
            let Some(t424) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008395fb); };
            c.cmp(t424, c.r32(3), 32);
            if c.cc_e() { blk = 315; continue; }
            blk = 250; continue;
        }
        250 => {
            c.w64(11, c.r64(9));
            if let Some(sr) = c.shr(c.r64(11), 0xdu64, 64) { c.w64(11, sr); }
            let Some(t425) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083960f); };
            c.w32(12, t425);
            let r426 = c.r64(11) ^ c.r64(9);
            c.w64(11, r426);
            c.logic(r426, 64);
            let r427 = c.add(c.r32(11), c.r32(12), 32);
            c.w32(11, r427);
            let r428 = c.r32(11) & 0x1ffcu64;
            c.w32(11, r428);
            c.logic(r428, 32);
            let r429 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(15, r429);
            let Some(t430) = c.ld64(c.r64(13).wrapping_add(0x2f18)) else { return RegfileExit::fault(0x7000839624); };
            c.w64(0, t430);
            let Some(t431) = c.ld32(c.r64(0).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x700083962b); };
            c.cmp(t431, c.r32(8), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t432) = c.ld32(c.r64(0).wrapping_add(c.r64(15)).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000839631); };
            c.cmp(t432, c.r32(9), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let Some(t433) = c.ld32(c.r64(0).wrapping_add(c.r64(15)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000839638); };
            c.w32(11, t433);
            let r434 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r434);
            blk = 256; continue;
        }
        253 => {
            c.w32(15, c.r32(8));
            c.w32(11, c.r32(11));
            let r435 = c.sub(c.r32(11), c.r32(15), 32);
            c.w32(11, r435);
            let r436 = c.sub(c.r32(11), 0x4e93591bu64, 32);
            c.w32(11, r436);
            let r437 = c.r32(11) & 0x7fcu64;
            c.w32(11, r437);
            c.logic(r437, 32);
            let r438 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r438);
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x2f30)) else { return RegfileExit::fault(0x700083965d); };
            c.w64(15, t439);
            let Some(t440) = c.ld32(c.r64(15).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x7000839664); };
            c.cmp(t440, c.r32(8), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t441) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083966e); };
            c.cmp(t441, c.r32(9), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t442) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000839679); };
            c.w32(11, t442);
            let r443 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r443);
            blk = 256; continue;
        }
        256 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 714; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083968b); };
            c.w64(2, t444);
            c.w64(15, c.r64(11));
            blk = 258; continue;
        }
        258 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(0, c.r64(15));
            let r445 = c.r64(0) & 0xfu64;
            c.w64(0, r445);
            c.logic(r445, 64);
            c.cmp(c.r64(0), 0x4u64, 64);
            if c.cc_e() { blk = 263; continue; }
            blk = 259; continue;
        }
        259 => {
            c.cmp(c.r64(0), 0x5u64, 64);
            if c.cc_e() { blk = 262; continue; }
            blk = 260; continue;
        }
        260 => {
            c.cmp(c.r64(0), 0xdu64, 64);
            if c.cc_ne() { blk = 796; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008396b5); };
            c.w64(0, t446);
            blk = 68; continue;
        }
        262 => {
            c.w64(0, c.r64(2));
            blk = 68; continue;
        }
        263 => {
            c.logic(c.r8(15) & 0x40u64, 8);
            if c.cc_ne() { blk = 276; continue; }
            blk = 264; continue;
        }
        264 => {
            c.w64(9, c.r64(15));
            if let Some(sr) = c.shr(c.r64(9), 0x9u64, 64) { c.w64(9, sr); }
            c.logic(c.r16(15) & 0x100u64, 16);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let r447 = c.r64(9) & 0x7ffu64;
            c.w64(9, r447);
            c.logic(r447, 64);
            c.logic(c.r8(15) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 271; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t448) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008396f5); };
            c.w32(12, t448);
            let r449 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r449);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 267; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t450) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000839702); };
            c.w64(12, t450);
            blk = 267; continue;
        }
        267 => {
            let Some(t451) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839709); };
            c.w32(0, t451);
            let r452 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r452);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 268; continue;
        }
        268 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t453) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839723); };
            c.w32(9, t453);
            let Some(t454) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000839727); };
            c.cmp(t454, c.r32(9), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 270; continue;
        }
        270 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000839734); }
            blk = 310; continue;
        }
        271 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 273; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t455) = c.ld32(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839744); };
            c.w32(0, t455);
            let r456 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r456);
            blk = 68; continue;
        }
        273 => {
            let Some(t457) = c.ld32(c.r64(2).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839751); };
            c.w32(9, t457);
            let r458 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r458);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 274; continue;
        }
        274 => {
            let Some(t459) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839763); };
            c.w32(11, t459);
            let Some(t460) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000839767); };
            c.cmp(t460, c.r32(11), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 275; continue;
        }
        275 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000839774); }
            blk = 310; continue;
        }
        276 => {
            c.w64(8, c.r64(15));
            let r461 = c.shr(c.r32(8), 0x7u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r461);
            let r462 = c.r32(8) & 0xfu64;
            c.w32(8, r462);
            c.logic(r462, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_l() { blk = 285; continue; }
            blk = 277; continue;
        }
        277 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_l() { blk = 282; continue; }
            blk = 278; continue;
        }
        278 => {
            c.cmp(c.r32(8), 0x7u64, 32);
            if c.cc_e() { blk = 932; continue; }
            blk = 279; continue;
        }
        279 => {
            c.cmp(c.r32(8), 0x8u64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 280; continue;
        }
        280 => {
            c.cmp(c.r32(8), 0x9u64, 32);
            if c.cc_e() { blk = 313; continue; }
            blk = 281; continue;
        }
        281 => {
            blk = 915; continue;
        }
        282 => {
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 283; continue;
        }
        283 => {
            c.cmp(c.r32(8), 0x6u64, 32);
            if c.cc_e() { blk = 309; continue; }
            blk = 284; continue;
        }
        284 => {
            blk = 915; continue;
        }
        285 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_l() { blk = 290; continue; }
            blk = 286; continue;
        }
        286 => {
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 295; continue; }
            blk = 287; continue;
        }
        287 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 297; continue; }
            blk = 288; continue;
        }
        288 => {
            c.cmp(c.r32(8), 0x4u64, 32);
            if c.cc_e() { blk = 299; continue; }
            blk = 289; continue;
        }
        289 => {
            blk = 915; continue;
        }
        290 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 293; continue; }
            blk = 291; continue;
        }
        291 => {
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 294; continue; }
            blk = 292; continue;
        }
        292 => {
            blk = 915; continue;
        }
        293 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r463 = c.r64(15) & 0xfffffu64;
            c.w64(15, r463);
            c.logic(r463, 64);
            let Some(t464) = c.ld8(c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839813); };
            c.w32(8, (t464 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        294 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r465 = c.r64(15) & 0xfffffu64;
            c.w64(15, r465);
            c.logic(r465, 64);
            let Some(t466) = c.ld16(c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083982d); };
            c.w32(8, (t466 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        295 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r467 = c.r64(15) & 0xfffffu64;
            c.w64(15, r467);
            c.logic(r467, 64);
            let Some(t468) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839847); };
            c.w32(8, t468);
            c.w64(9, c.r64(8));
            let r469 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r469);
            if c.cc_o() { blk = 916; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 68; continue;
        }
        297 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r470 = c.r64(15) & 0xfffffu64;
            c.w64(15, r470);
            c.logic(r470, 64);
            let Some(t471) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083986b); };
            c.w32(8, t471);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 921; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        299 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r472 = c.r64(15) & 0xfffffu64;
            c.w64(15, r472);
            c.logic(r472, 64);
            let Some(t473) = c.ld64(c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839891); };
            c.w64(8, t473);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 305; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083989c); };
            c.w64(9, t474);
            let Some(t475) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008398a3); };
            c.w64(11, t475);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t476) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008398aa); };
            c.w64(15, t476);
            let Some(t477) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008398b1); };
            c.cmp(t477, c.r64(12), 64);
            if c.cc_be() { blk = 926; continue; }
            blk = 301; continue;
        }
        301 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008398ba); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 302; continue;
        }
        302 => {
            let Some(t478) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008398c1); };
            c.w64(11, t478);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008398c8); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 304; continue; }
            blk = 303; continue;
        }
        303 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008398d2); }
            let r479 = c.neg(c.r64(8), 64);
            c.w64(8, r479);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008398dd); }
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        304 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008398e9); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008398f1); }
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        305 => {
            let Some(t480) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008398fd); };
            c.w64(8, t480);
            let Some(t481) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000839904); };
            c.w64(9, t481);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t482) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083990b); };
            c.w64(11, t482);
            let Some(t483) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000839912); };
            c.cmp(t483, c.r64(12), 64);
            if c.cc_be() { blk = 928; continue; }
            blk = 306; continue;
        }
        306 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700083991b); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 307; continue;
        }
        307 => {
            let Some(t484) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x7000839922); };
            c.w64(9, t484);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000839929); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700083992d); }
            c.w64(0, c.r64(8));
            blk = 68; continue;
        }
        308 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r485 = c.r64(15) & 0xfffffu64;
            c.w64(15, r485);
            c.logic(r485, 64);
            if !c.movss_rm(0, c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000839948); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 310; continue;
        }
        309 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r486 = c.r64(15) & 0xfffffu64;
            c.w64(15, r486);
            c.logic(r486, 64);
            if !c.movsd_rm(0, c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) { return RegfileExit::fault(0x7000839960); }
            blk = 310; continue;
        }
        310 => {
            let Some(t487) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000839967); };
            c.w64(8, t487);
            let Some(t488) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700083996e); };
            c.w64(9, t488);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t489) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000839975); };
            c.w64(12, t489);
            let Some(t490) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700083997c); };
            c.cmp(t490, c.r64(11), 64);
            if c.cc_be() { blk = 930; continue; }
            blk = 311; continue;
        }
        311 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000839986); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 312; continue;
        }
        312 => {
            let Some(t491) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083998d); };
            c.w64(9, t491);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000839994); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000839998); }
            c.w64(0, c.r64(8));
            blk = 68; continue;
        }
        313 => {
            if let Some(sr) = c.shr(c.r64(15), 0xbu64, 64) { c.w64(15, sr); }
            let r492 = c.r64(15) & 0xfffffu64;
            c.w64(15, r492);
            c.logic(r492, 64);
            let Some(t493) = c.ld32(c.r64(2).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008399b1); };
            c.w32(0, t493);
            let r494 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r494);
            blk = 68; continue;
        }
        314 => {
            let Some(t495) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008399be); };
            c.w64(5, t495);
            let Some(t496) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008399c2); };
            c.w64(6, t496);
            let Some(t497) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x70008399c7); };
            c.w64(10, t497);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(10)) { return RegfileExit::fault(0x70008399cc); }
            c.w32(0, 0x4u64);
            let Some(t498) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x70008399d6); };
            c.w64(10, t498);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x70008399db); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(8)) { return RegfileExit::fault(0x70008399e0); }
            let Some(t499) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008399e5); };
            c.w64(10, t499);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x70008399ea); }
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(3)) { return RegfileExit::fault(0x70008399ef); }
            let Some(t500) = c.ld64(c.r64(13).wrapping_add(0x2018)) else { return RegfileExit::fault(0x70008399f4); };
            c.w64(3, t500);
            let r501 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r501);
            return RegfileExit::jump(0x700086ed74);
        }
        315 => {
            let Some(t502) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839a04); };
            c.w32(11, t502);
            let Some(t503) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000839a08); };
            c.cmp(t503, c.r32(11), 32);
            if c.cc_ne() { blk = 318; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t504) = c.ld8(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839a11); };
            c.logic(t504 & 0x1u64, 8);
            if c.cc_e() { blk = 318; continue; }
            blk = 317; continue;
        }
        317 => {
            let Some(t505) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839a18); };
            c.w64(2, t505);
            blk = 655; continue;
        }
        318 => {
            let Some(t506) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000839a21); };
            c.w64(2, t506);
            let Some(t507) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000839a25); };
            c.w64(6, t507);
            blk = 364; continue;
        }
        319 => {
            let Some(t508) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839a2e); };
            c.w32(12, t508);
            let r509 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r509);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 321; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t510) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000839a3b); };
            c.w64(12, t510);
            blk = 321; continue;
        }
        321 => {
            let Some(t511) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839a42); };
            c.w32(15, t511);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r512 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r512);
            let Some(t513) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839a51); };
            c.w32(0, t513);
            let r514 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r514);
            let r515 = c.r64(0) & c.r64(15);
            c.w64(0, r515);
            c.logic(r515, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 323; continue;
        }
        322 => {
            let r516 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r516);
            c.w64(11, c.r64(3).wrapping_add(c.r64(0)));
            c.w64(0, c.r64(15));
            let r517 = c.r64(0) & c.r64(11);
            c.w64(0, r517);
            c.logic(r517, 64);
            blk = 323; continue;
        }
        323 => {
            c.w64(7, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(7).wrapping_mul(4));
            let Some(t518) = c.ld32(c.r64(12).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839a7a); };
            c.w32(11, t518);
            let Some(t519) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000839a7f); };
            c.cmp(t519, c.r32(11), 32);
            if c.cc_e() { blk = 364; continue; }
            blk = 324; continue;
        }
        324 => {
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_ne() { blk = 322; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t520) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839a91); };
            c.w32(9, t520);
            let Some(t521) = c.ld32(c.r64(12).wrapping_add(c.r64(1)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839a96); };
            c.w32(11, t521);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(7).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ae() { blk = 934; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t522) = c.ld32(c.r64(12).wrapping_add(c.r64(1)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839aae); };
            c.w32(9, t522);
            let r523 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r523);
            let r524 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r524);
            c.w64(3, c.r64(11));
            blk = 327; continue;
        }
        327 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 363; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t525) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839ac5); };
            c.w32(11, t525);
            let Some(t526) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000839ac9); };
            c.cmp(t526, c.r32(11), 32);
            if c.cc_ne() { blk = 347; continue; }
            blk = 329; continue;
        }
        329 => {
            let Some(t527) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839ad6); };
            c.w32(9, t527);
            let r528 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r528);
            let Some(t529) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839add); };
            c.w32(11, t529);
            let r530 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r530);
            let Some(t531) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000839ae4); };
            c.logic(t531 & 0x2u64, 8);
            if c.cc_ne() { blk = 345; continue; }
            blk = 330; continue;
        }
        330 => {
            let Some(t532) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000839aef); };
            c.cmp(t532, c.r32(11), 32);
            if c.cc_e() { blk = 332; continue; }
            blk = 331; continue;
        }
        331 => {
            let Some(t533) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000839af8); };
            c.w64(0, t533);
            blk = 68; continue;
        }
        332 => {
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000839b04); };
            c.w64(11, t534);
            let Some(t535) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x7000839b0b); };
            c.cmp(t535, 0x20u64, 8);
            if c.cc_e() { blk = 936; continue; }
            blk = 333; continue;
        }
        333 => {
            let Some(t536) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000839b15); };
            c.w32(8, t536);
            let Some(t537) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000839b19); };
            c.cmp(t537, c.r32(8), 32);
            if c.cc_ne() { blk = 655; continue; }
            blk = 334; continue;
        }
        334 => {
            let Some(t538) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839b26); };
            c.w32(8, t538);
            let r539 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r539);
            let Some(t540) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839b2d); };
            c.w32(11, t540);
            let Some(t541) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000839b31); };
            c.cmp(t541, c.r32(11), 32);
            if c.cc_ne() { blk = 336; continue; }
            blk = 335; continue;
        }
        335 => {
            let Some(t542) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839b3a); };
            c.w32(8, t542);
            let r543 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r543);
            let Some(t544) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839b41); };
            c.w32(11, t544);
            let Some(t545) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000839b45); };
            c.cmp(t545, c.r32(11), 32);
            if c.cc_e() { blk = 335; continue; }
            blk = 336; continue;
        }
        336 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 655; continue; }
            blk = 337; continue;
        }
        337 => {
            let Some(t546) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839b58); };
            c.w32(11, t546);
            let r547 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r547);
            let Some(t548) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839b5f); };
            c.w32(12, t548);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 342; continue; }
            blk = 338; continue;
        }
        338 => {
            c.w32(8, c.r16(12));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 341; continue; }
            blk = 339; continue;
        }
        339 => {
            let Some(t549) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000839b7f); };
            c.cmp(t549, c.r32(11), 32);
            if c.cc_e() { blk = 655; continue; }
            blk = 340; continue;
        }
        340 => {
            c.w64(8, c.r64(2));
            blk = 342; continue;
        }
        341 => {
            let Some(t550) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839b91); };
            c.w32(8, t550);
            let r551 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r551);
            blk = 342; continue;
        }
        342 => {
            let Some(t552) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839b98); };
            c.w32(8, t552);
            let r553 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r553);
            let Some(t554) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839b9f); };
            c.w32(8, t554);
            let r555 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r555);
            let Some(t556) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839ba6); };
            c.w32(11, t556);
            let r557 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r557);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 655; continue; }
            blk = 343; continue;
        }
        343 => {
            let Some(t558) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839bbc); };
            c.w32(6, t558);
            let r559 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r559);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000839bc3); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x7000839bce, 0x7000839bc9);
        }
        344 => {
            blk = 68; continue;
        }
        345 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000839bd3); }
            c.w64(7, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x7000839bde, 0x7000839bd9);
        }
        346 => {
            blk = 68; continue;
        }
        347 => {
            let Some(t560) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839be3); };
            c.w32(8, t560);
            let r561 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r561);
            let Some(t562) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839bea); };
            c.w32(11, t562);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 361; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 354; continue; }
            blk = 349; continue;
        }
        349 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 655; continue; }
            blk = 350; continue;
        }
        350 => {
            let Some(t563) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839c14); };
            c.w32(8, t563);
            let Some(t564) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000839c18); };
            c.cmp(t564, c.r32(8), 32);
            if c.cc_ne() { blk = 655; continue; }
            blk = 351; continue;
        }
        351 => {
            let Some(t565) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839c25); };
            c.w32(8, t565);
            let r566 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r566);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 655; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t567) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839c36); };
            c.w32(9, t567);
            let r568 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r568);
            let Some(t569) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839c3d); };
            c.cmp(t569, 0x40u64, 16);
            if c.cc_ae() { blk = 655; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t570) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839c49); };
            c.w32(8, t570);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        354 => {
            let Some(t571) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839c56); };
            c.w32(9, t571);
            let Some(t572) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x7000839c5a); };
            c.cmp(t572, c.r32(9), 32);
            if c.cc_ne() { blk = 655; continue; }
            blk = 355; continue;
        }
        355 => {
            let Some(t573) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000839c67); };
            c.w32(8, t573);
            c.w64(9, c.r64(8));
            let r574 = c.r32(9) & 0xc0u64;
            c.w32(9, r574);
            c.logic(r574, 32);
            c.cmp(c.r32(9), 0xc0u64, 32);
            if c.cc_e() { blk = 357; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t575) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839c7f); };
            c.w32(9, t575);
            let r576 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r576);
            let Some(t577) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000839c86); };
            c.w32(9, t577);
            let r578 = c.r32(9) & 0x1fu64;
            c.w32(9, r578);
            c.logic(r578, 32);
            let r579 = c.sub(c.r32(9), 0x10u64, 32);
            c.w32(9, r579);
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_a() { blk = 655; continue; }
            blk = 357; continue;
        }
        357 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 655; continue; }
            blk = 358; continue;
        }
        358 => {
            let Some(t580) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000839ca6); };
            c.w32(0, t580);
            let r581 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r581);
            let Some(t582) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000839cac); };
            c.cmp(t582, c.r32(0), 32);
            if c.cc_e() { blk = 655; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t583) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839cb9); };
            c.w32(8, t583);
            let Some(t584) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000839cbd); };
            c.cmp(t584, c.r32(8), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t585) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839cca); };
            c.w32(0, t585);
            let r586 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r586);
            blk = 68; continue;
        }
        361 => {
            let Some(t587) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839cd5); };
            c.w32(8, t587);
            let Some(t588) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000839cd9); };
            c.cmp(t588, c.r32(8), 32);
            if c.cc_ne() { blk = 655; continue; }
            blk = 362; continue;
        }
        362 => {
            let Some(t589) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839ce6); };
            c.w32(0, t589);
            let r590 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r590);
            blk = 68; continue;
        }
        363 => {
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        364 => {
            let Some(t591) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839cf9); };
            c.w32(11, t591);
            let Some(t592) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000839cfd); };
            c.cmp(t592, c.r32(11), 32);
            if c.cc_ne() { blk = 368; continue; }
            blk = 365; continue;
        }
        365 => {
            let Some(t593) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839d06); };
            c.w32(11, t593);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 368; continue; }
            blk = 366; continue;
        }
        366 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 655; continue; }
            blk = 367; continue;
        }
        367 => {
            blk = 469; continue;
        }
        368 => {
            let Some(t594) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000839d1f); };
            c.w64(11, t594);
            blk = 370; continue;
        }
        369 => {
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(12));
            blk = 370; continue;
        }
        370 => {
            c.cmp(c.r32(11), 0x82eu64, 32);
            if c.cc_e() { blk = 946; continue; }
            blk = 371; continue;
        }
        371 => {
            let Some(t595) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839d38); };
            c.w32(9, t595);
            let r596 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r596);
            let Some(t597) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000839d3f); };
            c.cmp(t597, c.r32(9), 32);
            if c.cc_e() { blk = 469; continue; }
            blk = 372; continue;
        }
        372 => {
            let Some(t598) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000839d4c); };
            c.w32(11, t598);
            let r599 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r599);
            let Some(t600) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839d53); };
            c.w32(12, t600);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000839d58); }
            c.cmp(c.r16(12), 0x410u64, 16);
            if c.cc_be() { blk = 401; continue; }
            blk = 373; continue;
        }
        373 => {
            let Some(t601) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839d68); };
            c.w32(15, t601);
            c.logic(c.r32(15) & 0x200000u64, 32);
            if c.cc_ne() { blk = 394; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t602) = c.ld32(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000839d79); };
            c.w32(0, t602);
            let r603 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r603);
            c.logic(c.r32(15) & 0xffc00u64, 32);
            if c.cc_e() { blk = 369; continue; }
            blk = 375; continue;
        }
        375 => {
            let r604 = c.shr(c.r32(15), 0xau64, 32).unwrap_or(c.r32(15));
            c.w32(15, r604);
            let r605 = c.r32(15) & 0x3ffu64;
            c.w32(15, r605);
            c.logic(r605, 32);
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_be() { blk = 390; continue; }
            blk = 376; continue;
        }
        376 => {
            let Some(t606) = c.ld16(c.r64(0).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000839d9e); };
            c.w32(3, (t606 as u16 as i16 as i64 as u64));
            let Some(t607) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839da2); };
            c.w32(1, t607);
            let r608 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r608);
            let r609 = c.sub(c.r32(3), 0x1u64, 32);
            c.w32(3, r609);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x7000839dac); }
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 378; continue;
        }
        377 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x7000839db4); }
            blk = 378; continue;
        }
        378 => {
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000839db8); };
            c.w64(2, t610);
            let r611 = c.sub(c.r32(2), c.r32(7), 32);
            c.w32(2, r611);
            let r612 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r612);
            let r613 = c.add(c.r32(2), c.r32(7), 32);
            c.w32(2, r613);
            c.w32(6, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t614) = c.ld32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839dc8); };
            c.w32(6, t614);
            let r615 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r615);
            let r616 = c.shr(c.r32(6), 0x9u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r616);
            let r617 = c.r32(6) & 0x3ffu64;
            c.w32(6, r617);
            c.logic(r617, 32);
            c.w32(6, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t618) = c.ld32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839ddd); };
            c.w32(6, t618);
            let r619 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r619);
            let Some(t620) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839de4); };
            c.w32(6, t620);
            let r621 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r621);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_be() { blk = 380; continue; }
            blk = 379; continue;
        }
        379 => {
            c.w32(7, c.r64(2).wrapping_add(0x1));
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000839df1); };
            c.w64(2, t622);
            blk = 380; continue;
        }
        380 => {
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_ne() { blk = 377; continue; }
            blk = 381; continue;
        }
        381 => {
            blk = 383; continue;
        }
        382 => {
            let r623 = c.add(c.r32(7), 0x1u64, 32);
            c.w32(7, r623);
            blk = 383; continue;
        }
        383 => {
            c.cmp(c.r32(3), c.r32(7), 32);
            if c.cc_l() { blk = 389; continue; }
            blk = 384; continue;
        }
        384 => {
            c.w32(2, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t624) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839e08); };
            c.w32(2, t624);
            let r625 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r625);
            let r626 = c.shr(c.r32(2), 0x9u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r626);
            let r627 = c.r32(2) & 0x3ffu64;
            c.w32(2, r627);
            c.logic(r627, 32);
            c.w32(6, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t628) = c.ld32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839e1d); };
            c.w32(11, t628);
            let r629 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r629);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000839e25); }
            let Some(t630) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839e29); };
            c.w32(6, t630);
            let r631 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r631);
            c.cmp(c.r32(1), c.r32(6), 32);
            if c.cc_ne() { blk = 388; continue; }
            blk = 385; continue;
        }
        385 => {
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_ne() { blk = 382; continue; }
            blk = 386; continue;
        }
        386 => {
            c.cmp(c.r32(15), c.r32(2), 32);
            if c.cc_a() { blk = 413; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000839e42); };
            c.w64(11, t632);
            blk = 369; continue;
        }
        388 => {
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000839e4b); };
            c.w64(11, t633);
            blk = 369; continue;
        }
        389 => {
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000839e54); };
            c.w64(11, t634);
            blk = 369; continue;
        }
        390 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r635 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(6, r635);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 369; continue; }
            blk = 391; continue;
        }
        391 => {
            let r636 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r636);
            let Some(t637) = c.ld32(c.r64(0).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839e72); };
            c.cmp(t637, c.r32(8), 32);
            if c.cc_e() { blk = 414; continue; }
            blk = 392; continue;
        }
        392 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 391; continue; }
            blk = 393; continue;
        }
        393 => {
            blk = 369; continue;
        }
        394 => {
            let Some(t638) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839e88); };
            c.w32(15, t638);
            let r639 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r639);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 396; continue; }
            blk = 395; continue;
        }
        395 => {
            let Some(t640) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000839e95); };
            c.w64(15, t640);
            blk = 396; continue;
        }
        396 => {
            let Some(t641) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839e9c); };
            c.w32(0, t641);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r642 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r642);
            let Some(t643) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839eaa); };
            c.w32(3, t643);
            let r644 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r644);
            let r645 = c.r64(3) & c.r64(0);
            c.w64(3, r645);
            c.logic(r645, 64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 398; continue;
        }
        397 => {
            let r646 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r646);
            let r647 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r647);
            c.w64(1, c.r64(0));
            let r648 = c.r64(1) & c.r64(3);
            c.w64(1, r648);
            c.logic(r648, 64);
            c.w64(3, c.r64(1));
            blk = 398; continue;
        }
        398 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(6, c.r64(1).wrapping_mul(4));
            let Some(t649) = c.ld32(c.r64(15).wrapping_add(c.r64(6)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839ed5); };
            c.w32(7, t649);
            let Some(t650) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000839eda); };
            c.cmp(t650, c.r32(7), 32);
            if c.cc_e() { blk = 369; continue; }
            blk = 399; continue;
        }
        399 => {
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_e() { blk = 427; continue; }
            blk = 400; continue;
        }
        400 => {
            blk = 397; continue;
        }
        401 => {
            c.cmp(c.r16(12), 0xbdu64, 16);
            if c.cc_ne() { blk = 468; continue; }
            blk = 402; continue;
        }
        402 => {
            let Some(t651) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000839efe); };
            c.logic(t651 & 0x24u64, 8);
            if c.cc_ne() { blk = 468; continue; }
            blk = 403; continue;
        }
        403 => {
            let Some(t652) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839f09); };
            c.w32(15, t652);
            let r653 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r653);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 405; continue; }
            blk = 404; continue;
        }
        404 => {
            let Some(t654) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000839f16); };
            c.w64(15, t654);
            blk = 405; continue;
        }
        405 => {
            let Some(t655) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000839f1d); };
            c.w32(0, t655);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r656 = c.sub(c.r64(0), 0x1u64, 64);
            c.w64(0, r656);
            let Some(t657) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839f2b); };
            c.w32(3, t657);
            let r658 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r658);
            let r659 = c.r64(3) & c.r64(0);
            c.w64(3, r659);
            c.logic(r659, 64);
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 407; continue;
        }
        406 => {
            let r660 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r660);
            let r661 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r661);
            c.w64(1, c.r64(0));
            let r662 = c.r64(1) & c.r64(3);
            c.w64(1, r662);
            c.logic(r662, 64);
            c.w64(3, c.r64(1));
            blk = 407; continue;
        }
        407 => {
            c.w64(1, c.r64(3).wrapping_add(0x5));
            let Some(t663) = c.ld32(c.r64(15).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000839f4d); };
            c.w32(6, t663);
            let r664 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r664);
            let Some(t665) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000839f55); };
            c.cmp(t665, c.r32(6), 32);
            if c.cc_e() { blk = 369; continue; }
            blk = 408; continue;
        }
        408 => {
            let Some(t666) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000839f62); };
            c.cmp(t666, c.r32(6), 32);
            if c.cc_e() { blk = 406; continue; }
            blk = 409; continue;
        }
        409 => {
            let Some(t667) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839f6b); };
            c.cmp(t667, c.r32(8), 32);
            if c.cc_ne() { blk = 406; continue; }
            blk = 410; continue;
        }
        410 => {
            let Some(t668) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839f71); };
            c.w32(15, t668);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(1), c.r64(15), 64);
            if c.cc_ae() { blk = 942; continue; }
            blk = 411; continue;
        }
        411 => {
            let Some(t669) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000839f84); };
            c.w32(15, t669);
            let r670 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r670);
            let Some(t671) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000839f8b); };
            c.cmp(t671, c.r32(15), 32);
            if c.cc_e() { blk = 369; continue; }
            blk = 412; continue;
        }
        412 => {
            blk = 429; continue;
        }
        413 => {
            let Some(t672) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000839f9d); };
            c.w64(6, t672);
            let Some(t673) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000839fa1); };
            c.w64(11, t673);
            blk = 414; continue;
        }
        414 => {
            c.w64(12, c.r64(6).wrapping_mul(4));
            let Some(t674) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000839fad); };
            c.w32(15, t674);
            c.w32(3, c.r32(15));
            c.w64(2, c.r64(15));
            let r675 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r675);
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 416; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t676) = c.ld32(c.r64(0).wrapping_add(c.r64(12)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000839fc0); };
            c.w32(15, t676);
            let r677 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r677);
            c.w64(11, c.r64(15));
            blk = 430; continue;
        }
        416 => {
            c.w64(12, c.r64(2));
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t678) = c.ld8(c.r64(11).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000839fd7); };
            c.w32(15, t678);
            let r679 = c.r64(12) & 0x3ffu64;
            c.w64(12, r679);
            c.logic(r679, 64);
            let r680 = c.r32(3) & 0x380u64;
            c.w32(3, r680);
            c.logic(r680, 32);
            let r681 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r681);
            c.cmp(c.r32(3), 0x100u64, 32);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            let Some(t682) = c.ld8(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000839ffa); };
            c.w32(11, t682);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_b() { blk = 421; continue; }
            blk = 417; continue;
        }
        417 => {
            let Some(t683) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a004); };
            c.w32(0, t683);
            let r684 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r684);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 419; continue; }
            blk = 418; continue;
        }
        418 => {
            let Some(t685) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083a00f); };
            c.w64(0, t685);
            blk = 419; continue;
        }
        419 => {
            let r686 = c.sub(c.r64(12), c.r64(11), 64);
            c.w64(12, r686);
            let Some(t687) = c.ld32(c.r64(0).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a019); };
            c.w32(11, t687);
            let r688 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r688);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 430; continue; }
            blk = 420; continue;
        }
        420 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700083a02b); }
            blk = 423; continue;
        }
        421 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 426; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t689) = c.ld32(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a039); };
            c.w32(11, t689);
            let r690 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r690);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700083a041); }
            blk = 423; continue;
        }
        423 => {
            let Some(t691) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083a047); };
            c.w64(11, t691);
            let Some(t692) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700083a04e); };
            c.w64(12, t692);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t693) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083a056); };
            c.w64(0, t693);
            let Some(t694) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700083a05d); };
            c.cmp(t694, c.r64(15), 64);
            if c.cc_be() { blk = 938; continue; }
            blk = 424; continue;
        }
        424 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x700083a066); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 425; continue;
        }
        425 => {
            let Some(t695) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083a06e); };
            c.w64(12, t695);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700083a075); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083a079); }
            blk = 430; continue;
        }
        426 => {
            let Some(t696) = c.ld32(c.r64(9).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a081); };
            c.w32(15, t696);
            let r697 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r697);
            c.w64(11, c.r64(15));
            blk = 430; continue;
        }
        427 => {
            let Some(t698) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a08e); };
            c.w32(11, t698);
            let Some(t699) = c.ld32(c.r64(15).wrapping_add(c.r64(6)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083a092); };
            c.w32(12, t699);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_ae() { blk = 940; continue; }
            blk = 428; continue;
        }
        428 => {
            let Some(t700) = c.ld32(c.r64(15).wrapping_add(c.r64(6)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a0aa); };
            c.w32(15, t700);
            let r701 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r701);
            let r702 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r702);
            c.w64(11, c.r64(15));
            c.w64(2, c.r64(12));
            blk = 430; continue;
        }
        429 => {
            let Some(t703) = c.ld32(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a0bd); };
            c.w32(11, t703);
            let r704 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r704);
            c.w64(2, c.r64(11));
            c.w64(11, c.r64(15));
            blk = 430; continue;
        }
        430 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 467; continue; }
            blk = 431; continue;
        }
        431 => {
            let Some(t705) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a0d3); };
            c.w32(12, t705);
            let Some(t706) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083a0d7); };
            c.cmp(t706, c.r32(12), 32);
            if c.cc_ne() { blk = 451; continue; }
            blk = 432; continue;
        }
        432 => {
            let Some(t707) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a0e4); };
            c.w32(2, t707);
            let r708 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r708);
            let Some(t709) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a0eb); };
            c.w32(11, t709);
            let r710 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r710);
            let Some(t711) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083a0f2); };
            c.logic(t711 & 0x2u64, 8);
            if c.cc_ne() { blk = 449; continue; }
            blk = 433; continue;
        }
        433 => {
            let Some(t712) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083a0fd); };
            c.cmp(t712, c.r32(11), 32);
            if c.cc_e() { blk = 435; continue; }
            blk = 434; continue;
        }
        434 => {
            let Some(t713) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083a106); };
            c.w64(0, t713);
            blk = 68; continue;
        }
        435 => {
            let Some(t714) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083a112); };
            c.w64(11, t714);
            let Some(t715) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700083a119); };
            c.cmp(t715, 0x20u64, 8);
            if c.cc_e() { blk = 944; continue; }
            blk = 436; continue;
        }
        436 => {
            let Some(t716) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x700083a123); };
            c.w32(8, t716);
            let Some(t717) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083a127); };
            c.cmp(t717, c.r32(8), 32);
            if c.cc_ne() { blk = 468; continue; }
            blk = 437; continue;
        }
        437 => {
            let Some(t718) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a134); };
            c.w32(8, t718);
            let r719 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r719);
            let Some(t720) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a13b); };
            c.w32(11, t720);
            let Some(t721) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083a13f); };
            c.cmp(t721, c.r32(11), 32);
            if c.cc_ne() { blk = 439; continue; }
            blk = 438; continue;
        }
        438 => {
            let Some(t722) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083a148); };
            c.w32(8, t722);
            let r723 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r723);
            let Some(t724) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a14f); };
            c.w32(11, t724);
            let Some(t725) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083a153); };
            c.cmp(t725, c.r32(11), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 439; continue;
        }
        439 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 468; continue; }
            blk = 440; continue;
        }
        440 => {
            let Some(t726) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a166); };
            c.w32(11, t726);
            let r727 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r727);
            let Some(t728) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a16d); };
            c.w32(12, t728);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 445; continue; }
            blk = 441; continue;
        }
        441 => {
            c.w32(8, c.r16(12));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 444; continue; }
            blk = 442; continue;
        }
        442 => {
            let Some(t729) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083a18d); };
            c.cmp(t729, c.r32(11), 32);
            if c.cc_e() { blk = 468; continue; }
            blk = 443; continue;
        }
        443 => {
            blk = 446; continue;
        }
        444 => {
            let Some(t730) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a19c); };
            c.w32(9, t730);
            let r731 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r731);
            blk = 446; continue;
        }
        445 => {
            c.w64(9, c.r64(8));
            blk = 446; continue;
        }
        446 => {
            let Some(t732) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083a1a8); };
            c.w32(8, t732);
            let r733 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r733);
            let Some(t734) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a1af); };
            c.w32(8, t734);
            let r735 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r735);
            let Some(t736) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a1b6); };
            c.w32(9, t736);
            let r737 = c.sub(c.r32(9), 0x89u64, 32);
            c.w32(9, r737);
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_a() { blk = 468; continue; }
            blk = 447; continue;
        }
        447 => {
            let Some(t738) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083a1cc); };
            c.w32(6, t738);
            let r739 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r739);
            let Some(t740) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083a1d3); };
            if !c.push64(t740) { return RegfileExit::fault(0x700083a1d3); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x700083a1dd, 0x700083a1d8);
        }
        448 => {
            blk = 68; continue;
        }
        449 => {
            let Some(t741) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083a1e2); };
            if !c.push64(t741) { return RegfileExit::fault(0x700083a1e2); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t742) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083a1ea); };
            c.w64(6, t742);
            return c.call(0x70007f2274, 0x700083a1f3, 0x700083a1ee);
        }
        450 => {
            blk = 68; continue;
        }
        451 => {
            let Some(t743) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a1f8); };
            c.w32(8, t743);
            let r744 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r744);
            let Some(t745) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a1ff); };
            c.w32(12, t745);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 465; continue; }
            blk = 452; continue;
        }
        452 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 458; continue; }
            blk = 453; continue;
        }
        453 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 468; continue; }
            blk = 454; continue;
        }
        454 => {
            let Some(t746) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a22a); };
            c.w32(8, t746);
            let Some(t747) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083a22e); };
            c.cmp(t747, c.r32(8), 32);
            if c.cc_ne() { blk = 468; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t748) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a23b); };
            c.w32(8, t748);
            let r749 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r749);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 468; continue; }
            blk = 456; continue;
        }
        456 => {
            let Some(t750) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a24c); };
            c.w32(9, t750);
            let r751 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r751);
            let Some(t752) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a253); };
            c.cmp(t752, 0x40u64, 16);
            if c.cc_ae() { blk = 468; continue; }
            blk = 457; continue;
        }
        457 => {
            let Some(t753) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a25f); };
            c.w32(8, t753);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        458 => {
            let Some(t754) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a26c); };
            c.w32(11, t754);
            let Some(t755) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700083a270); };
            c.cmp(t755, c.r32(11), 32);
            if c.cc_ne() { blk = 468; continue; }
            blk = 459; continue;
        }
        459 => {
            let Some(t756) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083a27d); };
            c.w32(8, t756);
            c.w64(11, c.r64(8));
            let r757 = c.r32(11) & 0xc0u64;
            c.w32(11, r757);
            c.logic(r757, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 461; continue; }
            blk = 460; continue;
        }
        460 => {
            let Some(t758) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a295); };
            c.w32(11, t758);
            let r759 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r759);
            let Some(t760) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083a29c); };
            c.w32(11, t760);
            let r761 = c.r32(11) & 0x1fu64;
            c.w32(11, r761);
            c.logic(r761, 32);
            let r762 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r762);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 468; continue; }
            blk = 461; continue;
        }
        461 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 468; continue; }
            blk = 462; continue;
        }
        462 => {
            let Some(t763) = c.ld32(c.r64(9).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083a2b4); };
            c.w32(0, t763);
            let r764 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r764);
            let Some(t765) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083a2bb); };
            c.cmp(t765, c.r32(0), 32);
            if c.cc_e() { blk = 468; continue; }
            blk = 463; continue;
        }
        463 => {
            let Some(t766) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a2c4); };
            c.w32(8, t766);
            let Some(t767) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083a2c8); };
            c.cmp(t767, c.r32(8), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 464; continue;
        }
        464 => {
            let Some(t768) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083a2d5); };
            c.w32(0, t768);
            let r769 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r769);
            blk = 68; continue;
        }
        465 => {
            let Some(t770) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a2e0); };
            c.w32(8, t770);
            let Some(t771) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083a2e4); };
            c.cmp(t771, c.r32(8), 32);
            if c.cc_ne() { blk = 468; continue; }
            blk = 466; continue;
        }
        466 => {
            let Some(t772) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a2ed); };
            c.w32(0, t772);
            let r773 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r773);
            blk = 68; continue;
        }
        467 => {
            c.w64(0, c.r64(11));
            blk = 68; continue;
        }
        468 => {
            let Some(t774) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083a301); };
            c.w64(2, t774);
            blk = 655; continue;
        }
        469 => {
            let Some(t775) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083a30a); };
            c.w64(0, t775);
            blk = 68; continue;
        }
        470 => {
            c.cmp(c.r16(11), 0xbcu64, 16);
            if c.cc_ne() { blk = 947; continue; }
            blk = 471; continue;
        }
        471 => {
            let Some(t776) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a322); };
            c.w32(9, t776);
            let Some(t777) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700083a326); };
            c.cmp(t777, c.r32(9), 32);
            if c.cc_ne() { blk = 474; continue; }
            blk = 472; continue;
        }
        472 => {
            let Some(t778) = c.ld8(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a32f); };
            c.logic(t778 & 0x1u64, 8);
            if c.cc_e() { blk = 474; continue; }
            blk = 473; continue;
        }
        473 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083a336); }
            blk = 655; continue;
        }
        474 => {
            let Some(t779) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083a33f); };
            c.w64(5, t779);
            c.w64(0, c.r64(2));
            c.w64(1, c.r64(2));
            c.w32(2, 0x2u64);
            c.w64(3, c.r64(8));
            let r780 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r780);
            return RegfileExit::jump(0x70008bc174);
        }
        475 => {
            blk = 477; continue;
        }
        476 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            blk = 477; continue;
        }
        477 => {
            c.w64(8, c.r64(2));
            let Some(t781) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a368); };
            c.w32(11, t781);
            let r782 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r782);
            let Some(t783) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a36f); };
            c.w32(12, t783);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083a374); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700083a378); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700083a37c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x700083a380); }
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_be() { blk = 652; continue; }
            blk = 478; continue;
        }
        478 => {
            let Some(t784) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x700083a390); };
            c.w32(15, t784);
            c.w64(0, c.r64(15));
            let r785 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r785);
            c.w64(3, c.r64(0));
            let r786 = c.r32(3) & 0x3fu64;
            c.w32(3, r786);
            c.logic(r786, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700083a3a1); }
            c.cmp(c.r32(3), 0xbu64, 32);
            if c.cc_g() { blk = 508; continue; }
            blk = 479; continue;
        }
        479 => {
            let Some(t787) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a3ae); };
            c.w32(15, t787);
            let r788 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r788);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 481; continue; }
            blk = 480; continue;
        }
        480 => {
            let Some(t789) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a3bd); };
            c.w32(0, t789);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            blk = 482; continue;
        }
        481 => {
            let Some(t790) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a3c9); };
            c.w32(0, t790);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            blk = 482; continue;
        }
        482 => {
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_ae() { blk = 586; continue; }
            blk = 483; continue;
        }
        483 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_l() { blk = 493; continue; }
            blk = 484; continue;
        }
        484 => {
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_l() { blk = 489; continue; }
            blk = 485; continue;
        }
        485 => {
            c.cmp(c.r32(3), 0x9u64, 32);
            if c.cc_e() { blk = 503; continue; }
            blk = 486; continue;
        }
        486 => {
            c.cmp(c.r32(3), 0xau64, 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 487; continue;
        }
        487 => {
            c.cmp(c.r32(3), 0xbu64, 32);
            if c.cc_e() { blk = 503; continue; }
            blk = 488; continue;
        }
        488 => {
            blk = 948; continue;
        }
        489 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 490; continue;
        }
        490 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_e() { blk = 503; continue; }
            blk = 491; continue;
        }
        491 => {
            c.cmp(c.r32(3), 0x8u64, 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 492; continue;
        }
        492 => {
            blk = 948; continue;
        }
        493 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_l() { blk = 498; continue; }
            blk = 494; continue;
        }
        494 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 503; continue; }
            blk = 495; continue;
        }
        495 => {
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_e() { blk = 505; continue; }
            blk = 496; continue;
        }
        496 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 506; continue; }
            blk = 497; continue;
        }
        497 => {
            blk = 948; continue;
        }
        498 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 499; continue;
        }
        499 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 503; continue; }
            blk = 500; continue;
        }
        500 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 502; continue; }
            blk = 501; continue;
        }
        501 => {
            blk = 948; continue;
        }
        502 => {
            let Some(t791) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a43f); };
            c.w32(0, t791);
            let r792 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r792);
            blk = 68; continue;
        }
        503 => {
            let Some(t793) = c.ld32(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a44c); };
            c.w32(0, t793);
            let r794 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r794);
            let Some(t795) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083a454); };
            c.cmp(t795, c.r32(0), 32);
            if c.cc_e() { blk = 588; continue; }
            blk = 504; continue;
        }
        504 => {
            blk = 98; continue;
        }
        505 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x700083a466); }
            blk = 631; continue;
        }
        506 => {
            c.w64(12, 0xfff7fffffff7ffffu64);
            let Some(t796) = c.ld64(c.r64(15).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a47c); };
            c.cmp(t796, c.r64(12), 64);
            if c.cc_e() { blk = 588; continue; }
            blk = 507; continue;
        }
        507 => {
            if !c.movsd_rm(0, c.r64(15).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7)) { return RegfileExit::fault(0x700083a487); }
            blk = 631; continue;
        }
        508 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_ge() { blk = 557; continue; }
            blk = 509; continue;
        }
        509 => {
            c.cmp(c.r32(3), 0x11u64, 32);
            if c.cc_ge() { blk = 526; continue; }
            blk = 510; continue;
        }
        510 => {
            let r797 = c.r32(15) & 0xfcu64;
            c.w32(15, r797);
            c.logic(r797, 32);
            c.cmp(c.r32(15), 0x30u64, 32);
            if c.cc_ne() { blk = 949; continue; }
            blk = 511; continue;
        }
        511 => {
            c.w32(15, 0xfffffffeu64);
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_b() { blk = 586; continue; }
            blk = 512; continue;
        }
        512 => {
            let Some(t798) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a4c5); };
            c.w32(12, t798);
            let r799 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r799);
            let Some(t800) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083a4cc); };
            c.w32(15, t800);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            let r801 = c.sub(c.r64(15), 0x1u64, 64);
            c.w64(15, r801);
            c.w32(2, c.r32(9));
            c.w64(10, c.r64(4));
            let r802 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r802);
            let r803 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r803);
            c.logic(r803, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700083a4e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(12)) { return RegfileExit::fault(0x700083a4ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700083a4f2); }
            c.w64(0, c.r64(13));
            c.w64(1, c.r64(0));
            let Some(t804) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700083a4fc); };
            c.w64(0, t804);
            c.w64(10, 0x700083a50au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700083a50a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700083a50e); }
            return c.call(c.r64(0), 0x700083a514, 0x700083a512);
        }
        513 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700083a514); }
            let Some(t805) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083a51c); };
            c.w64(4, t805);
            c.w32(8, c.r32(0));
            let Some(t806) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083a524); };
            let r807 = c.r64(8) & t806;
            c.w64(8, r807);
            c.logic(r807, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t808) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083a52b); };
            c.w64(11, t808);
            blk = 515; continue;
        }
        514 => {
            let r809 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r809);
            let r810 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r810);
            let Some(t811) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083a538); };
            c.w64(12, t811);
            let r812 = c.r64(12) & c.r64(8);
            c.w64(12, r812);
            c.logic(r812, 64);
            c.w64(8, c.r64(12));
            blk = 515; continue;
        }
        515 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            c.w64(15, c.r64(12).wrapping_mul(4));
            let Some(t813) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a54f); };
            c.w32(0, t813);
            let r814 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r814);
            let Some(t815) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083a557); };
            c.cmp(t815, c.r32(0), 32);
            if c.cc_e() { blk = 525; continue; }
            blk = 516; continue;
        }
        516 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 521; continue; }
            blk = 517; continue;
        }
        517 => {
            let Some(t816) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083a564); };
            c.cmp(t816, c.r32(0), 32);
            if c.cc_e() { blk = 514; continue; }
            blk = 518; continue;
        }
        518 => {
            c.xorpd_rr(0, 0);
            let Some(t817) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083a571); };
            c.cvtsi2sd(0, (t817 as i64));
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700083a577); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 514; continue; }
            blk = 519; continue;
        }
        519 => {
            if c.cc_e() { blk = 522; continue; }
            blk = 520; continue;
        }
        520 => {
            blk = 514; continue;
        }
        521 => {
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let Some(t818) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083a58c); };
            c.cmp(c.r64(0), t818, 64);
            if c.cc_ne() { blk = 514; continue; }
            blk = 522; continue;
        }
        522 => {
            let Some(t819) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083a592); };
            c.w32(8, t819);
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_ne() { blk = 950; continue; }
            blk = 523; continue;
        }
        523 => {
            let Some(t820) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083a5a1); };
            c.w32(8, t820);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 951; continue; }
            blk = 524; continue;
        }
        524 => {
            let Some(t821) = c.ld32(c.r64(11).wrapping_add(c.r64(15)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a5b9); };
            c.w32(0, t821);
            let r822 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r822);
            blk = 68; continue;
        }
        525 => {
            let Some(t823) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083a5c6); };
            c.w64(8, t823);
            let Some(t824) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083a5ca); };
            c.w64(6, t824);
            let Some(t825) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083a5ce); };
            c.w64(1, t825);
            let Some(t826) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083a5d2); };
            c.w64(11, t826);
            blk = 588; continue;
        }
        526 => {
            let Some(t827) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a5db); };
            c.w32(15, t827);
            let r828 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r828);
            let Some(t829) = c.ld8(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700083a5e2); };
            c.logic(t829 & 0x4u64, 8);
            if c.cc_ne() { blk = 953; continue; }
            blk = 527; continue;
        }
        527 => {
            let Some(t830) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083a5ed); };
            c.cmp(t830, c.r64(9), 64);
            if c.cc_be() { blk = 586; continue; }
            blk = 528; continue;
        }
        528 => {
            let Some(t831) = c.ld64(c.r64(8).wrapping_add(0x27)) else { return RegfileExit::fault(0x700083a5f7); };
            c.w64(11, t831);
            let Some(t832) = c.ld32(c.r64(8).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700083a5fb); };
            c.w32(12, t832);
            c.cmp(c.r32(3), 0x16u64, 32);
            if c.cc_l() { blk = 538; continue; }
            blk = 529; continue;
        }
        529 => {
            c.cmp(c.r32(3), 0x19u64, 32);
            if c.cc_l() { blk = 534; continue; }
            blk = 530; continue;
        }
        530 => {
            c.cmp(c.r32(3), 0x19u64, 32);
            if c.cc_e() { blk = 547; continue; }
            blk = 531; continue;
        }
        531 => {
            c.cmp(c.r32(3), 0x1au64, 32);
            if c.cc_e() { blk = 556; continue; }
            blk = 532; continue;
        }
        532 => {
            c.cmp(c.r32(3), 0x1bu64, 32);
            if c.cc_e() { blk = 555; continue; }
            blk = 533; continue;
        }
        533 => {
            blk = 952; continue;
        }
        534 => {
            c.cmp(c.r32(3), 0x16u64, 32);
            if c.cc_e() { blk = 552; continue; }
            blk = 535; continue;
        }
        535 => {
            c.cmp(c.r32(3), 0x17u64, 32);
            if c.cc_e() { blk = 553; continue; }
            blk = 536; continue;
        }
        536 => {
            c.cmp(c.r32(3), 0x18u64, 32);
            if c.cc_e() { blk = 554; continue; }
            blk = 537; continue;
        }
        537 => {
            blk = 952; continue;
        }
        538 => {
            c.cmp(c.r32(3), 0x13u64, 32);
            if c.cc_l() { blk = 543; continue; }
            blk = 539; continue;
        }
        539 => {
            c.cmp(c.r32(3), 0x13u64, 32);
            if c.cc_e() { blk = 549; continue; }
            blk = 540; continue;
        }
        540 => {
            c.cmp(c.r32(3), 0x14u64, 32);
            if c.cc_e() { blk = 550; continue; }
            blk = 541; continue;
        }
        541 => {
            c.cmp(c.r32(3), 0x15u64, 32);
            if c.cc_e() { blk = 551; continue; }
            blk = 542; continue;
        }
        542 => {
            blk = 952; continue;
        }
        543 => {
            c.cmp(c.r32(3), 0x11u64, 32);
            if c.cc_e() { blk = 546; continue; }
            blk = 544; continue;
        }
        544 => {
            c.cmp(c.r32(3), 0x12u64, 32);
            if c.cc_e() { blk = 548; continue; }
            blk = 545; continue;
        }
        545 => {
            blk = 952; continue;
        }
        546 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 614; continue;
        }
        547 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 614; continue;
        }
        548 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 616; continue;
        }
        549 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 618; continue;
        }
        550 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 620; continue;
        }
        551 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 622; continue;
        }
        552 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 625; continue;
        }
        553 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 628; continue;
        }
        554 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 630; continue;
        }
        555 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 635; continue;
        }
        556 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(12)));
            blk = 645; continue;
        }
        557 => {
            let Some(t833) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a6d4); };
            c.w32(0, t833);
            let r834 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r834);
            let Some(t835) = c.ld32(c.r64(0).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700083a6db); };
            c.w32(2, t835);
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 583; continue; }
            blk = 558; continue;
        }
        558 => {
            c.logic(c.r8(2) & 0x4u64, 8);
            if c.cc_ne() { blk = 956; continue; }
            blk = 559; continue;
        }
        559 => {
            let Some(t836) = c.ld64(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083a6f0); };
            c.w64(0, t836);
            let Some(t837) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083a6f4); };
            c.w64(2, t837);
            let Some(t838) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700083a6f8); };
            c.logic(t838 & 0x1u64, 8);
            if c.cc_ne() { blk = 562; continue; }
            blk = 560; continue;
        }
        560 => {
            let Some(t839) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083a6ff); };
            c.w64(15, t839);
            let r840 = c.add(c.r64(15), c.r64(2), 64);
            c.w64(15, r840);
            c.cmp(c.r64(15), c.r64(0), 64);
            if c.cc_a() { blk = 954; continue; }
            blk = 561; continue;
        }
        561 => {
            let Some(t841) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083a70f); };
            c.w64(15, t841);
            c.w64(0, c.r64(15));
            blk = 585; continue;
        }
        562 => {
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_a() { blk = 955; continue; }
            blk = 563; continue;
        }
        563 => {
            c.w32(15, c.r8(15));
            let r842 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r842);
            let r843 = c.r32(15) & 0x3fu64;
            c.w32(15, r843);
            c.logic(r843, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 571; continue; }
            blk = 564; continue;
        }
        564 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 568; continue; }
            blk = 565; continue;
        }
        565 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 578; continue; }
            blk = 566; continue;
        }
        566 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 581; continue; }
            blk = 567; continue;
        }
        567 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 581; continue; }
            blk = 568; continue;
        }
        568 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 580; continue; }
            blk = 569; continue;
        }
        569 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 580; continue; }
            blk = 570; continue;
        }
        570 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 581; continue; }
            blk = 571; continue;
        }
        571 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 575; continue; }
            blk = 572; continue;
        }
        572 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 579; continue; }
            blk = 573; continue;
        }
        573 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 579; continue; }
            blk = 574; continue;
        }
        574 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 580; continue; }
            blk = 575; continue;
        }
        575 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 578; continue; }
            blk = 576; continue;
        }
        576 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 578; continue; }
            blk = 577; continue;
        }
        577 => {
            return RegfileExit::fault(0x700083a788);
        }
        578 => {
            c.w32(15, 0x1u64);
            blk = 582; continue;
        }
        579 => {
            c.w32(15, 0x2u64);
            blk = 582; continue;
        }
        580 => {
            c.w32(15, 0x4u64);
            blk = 582; continue;
        }
        581 => {
            c.w32(15, 0x8u64);
            blk = 582; continue;
        }
        582 => {
            let r844 = c.sub(c.r64(0), c.r64(2), 64);
            c.w64(0, r844);
            c.cqo();
            c.idiv(c.r64(15), 64);
            blk = 585; continue;
        }
        583 => {
            c.w64(10, c.r64(4));
            let r845 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r845);
            let r846 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r846);
            c.logic(r846, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700083a7bc); }
            c.w64(2, c.r64(8));
            c.w64(1, c.r64(13));
            c.w64(15, c.r64(1));
            let Some(t847) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700083a7ca); };
            c.w64(0, t847);
            c.w64(10, 0x700083a7d8u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700083a7d8); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700083a7dc); }
            return c.call(c.r64(0), 0x700083a7e2, 0x700083a7e0);
        }
        584 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700083a7e2); }
            let Some(t848) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083a7ea); };
            c.w64(4, t848);
            let Some(t849) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083a7ef); };
            c.w64(11, t849);
            let Some(t850) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083a7f3); };
            c.w64(8, t850);
            let Some(t851) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083a7f7); };
            c.w64(6, t851);
            let Some(t852) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083a7fb); };
            c.w64(1, t852);
            let Some(t853) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083a7ff); };
            c.w64(9, t853);
            let Some(t854) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083a803); };
            c.w64(12, t854);
            let Some(t855) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083a807); };
            c.w64(3, t855);
            blk = 585; continue;
        }
        585 => {
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_b() { blk = 594; continue; }
            blk = 586; continue;
        }
        586 => {
            c.cmp(c.r16(12), 0x82eu64, 16);
            if c.cc_e() { blk = 593; continue; }
            blk = 587; continue;
        }
        587 => {
            c.w32(12, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_a() { blk = 957; continue; }
            blk = 588; continue;
        }
        588 => {
            let Some(t856) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083a827); };
            c.w32(9, t856);
            let r857 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r857);
            let Some(t858) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700083a82e); };
            c.cmp(t858, c.r32(9), 32);
            if c.cc_e() { blk = 593; continue; }
            blk = 589; continue;
        }
        589 => {
            let Some(t859) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083a837); };
            c.w32(11, t859);
            let r860 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r860);
            let Some(t861) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a83e); };
            c.w32(12, t861);
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_be() { blk = 958; continue; }
            blk = 590; continue;
        }
        590 => {
            let Some(t862) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083a84f); };
            c.w32(9, t862);
            let Some(t863) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083a853); };
            c.cmp(t863, c.r32(9), 32);
            if c.cc_e() { blk = 588; continue; }
            blk = 591; continue;
        }
        591 => {
            let Some(t864) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x700083a85c); };
            c.cmp(t864, c.r32(9), 32);
            if c.cc_e() { blk = 588; continue; }
            blk = 592; continue;
        }
        592 => {
            blk = 963; continue;
        }
        593 => {
            let Some(t865) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083a86a); };
            c.w64(0, t865);
            blk = 68; continue;
        }
        594 => {
            c.w64(0, c.r64(8));
            let Some(t866) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x700083a879); };
            c.w64(8, t866);
            let Some(t867) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700083a87d); };
            c.w32(11, t867);
            c.w64(12, c.r64(3));
            c.cmp(c.r32(12), 0x21u64, 32);
            if c.cc_l() { blk = 604; continue; }
            blk = 595; continue;
        }
        595 => {
            c.cmp(c.r32(12), 0x24u64, 32);
            if c.cc_l() { blk = 600; continue; }
            blk = 596; continue;
        }
        596 => {
            c.cmp(c.r32(12), 0x24u64, 32);
            if c.cc_e() { blk = 613; continue; }
            blk = 597; continue;
        }
        597 => {
            c.cmp(c.r32(12), 0x25u64, 32);
            if c.cc_e() { blk = 644; continue; }
            blk = 598; continue;
        }
        598 => {
            c.cmp(c.r32(12), 0x26u64, 32);
            if c.cc_e() { blk = 634; continue; }
            blk = 599; continue;
        }
        599 => {
            blk = 964; continue;
        }
        600 => {
            c.cmp(c.r32(12), 0x21u64, 32);
            if c.cc_e() { blk = 624; continue; }
            blk = 601; continue;
        }
        601 => {
            c.cmp(c.r32(12), 0x22u64, 32);
            if c.cc_e() { blk = 627; continue; }
            blk = 602; continue;
        }
        602 => {
            c.cmp(c.r32(12), 0x23u64, 32);
            if c.cc_e() { blk = 629; continue; }
            blk = 603; continue;
        }
        603 => {
            blk = 964; continue;
        }
        604 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_l() { blk = 609; continue; }
            blk = 605; continue;
        }
        605 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_e() { blk = 617; continue; }
            blk = 606; continue;
        }
        606 => {
            c.cmp(c.r32(12), 0x1fu64, 32);
            if c.cc_e() { blk = 619; continue; }
            blk = 607; continue;
        }
        607 => {
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_e() { blk = 621; continue; }
            blk = 608; continue;
        }
        608 => {
            blk = 964; continue;
        }
        609 => {
            c.cmp(c.r32(12), 0x1cu64, 32);
            if c.cc_e() { blk = 612; continue; }
            blk = 610; continue;
        }
        610 => {
            c.cmp(c.r32(12), 0x1du64, 32);
            if c.cc_e() { blk = 615; continue; }
            blk = 611; continue;
        }
        611 => {
            blk = 964; continue;
        }
        612 => {
            let r868 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r868);
            blk = 614; continue;
        }
        613 => {
            let r869 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r869);
            blk = 614; continue;
        }
        614 => {
            let Some(t870) = c.ld8(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700083a90c); };
            c.w32(8, t870);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        615 => {
            let r871 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r871);
            blk = 616; continue;
        }
        616 => {
            let Some(t872) = c.ld8(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700083a91d); };
            c.w32(8, (t872 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        617 => {
            let r873 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r873);
            blk = 618; continue;
        }
        618 => {
            let Some(t874) = c.ld16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x700083a92e); };
            c.w32(8, t874);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        619 => {
            let r875 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r875);
            blk = 620; continue;
        }
        620 => {
            let Some(t876) = c.ld16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2))) else { return RegfileExit::fault(0x700083a93f); };
            c.w32(8, (t876 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        621 => {
            let r877 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r877);
            blk = 622; continue;
        }
        622 => {
            let Some(t878) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700083a950); };
            c.w32(8, t878);
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 965; continue; }
            blk = 623; continue;
        }
        623 => {
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        624 => {
            let r879 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r879);
            blk = 625; continue;
        }
        625 => {
            let Some(t880) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4))) else { return RegfileExit::fault(0x700083a96d); };
            c.w32(8, t880);
            c.w64(9, c.r64(8));
            let r881 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r881);
            if c.cc_o() { blk = 970; continue; }
            blk = 626; continue;
        }
        626 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 68; continue;
        }
        627 => {
            let r882 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r882);
            blk = 628; continue;
        }
        628 => {
            if !c.movss_rm(0, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4))) { return RegfileExit::fault(0x700083a988); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 631; continue;
        }
        629 => {
            let r883 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r883);
            blk = 630; continue;
        }
        630 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8))) { return RegfileExit::fault(0x700083a997); }
            blk = 631; continue;
        }
        631 => {
            let Some(t884) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083a99d); };
            c.w64(8, t884);
            let Some(t885) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700083a9a4); };
            c.w64(9, t885);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t886) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083a9ab); };
            c.w64(12, t886);
            let Some(t887) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700083a9b2); };
            c.cmp(t887, c.r64(11), 64);
            if c.cc_be() { blk = 975; continue; }
            blk = 632; continue;
        }
        632 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700083a9bc); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 633; continue;
        }
        633 => {
            let Some(t888) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083a9c3); };
            c.w64(9, t888);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700083a9ca); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083a9ce); }
            c.w64(0, c.r64(8));
            blk = 68; continue;
        }
        634 => {
            let r889 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r889);
            blk = 635; continue;
        }
        635 => {
            let Some(t890) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700083a9df); };
            c.w64(8, t890);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 641; continue; }
            blk = 636; continue;
        }
        636 => {
            let Some(t891) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083a9e9); };
            c.w64(9, t891);
            let Some(t892) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083a9f0); };
            c.w64(11, t892);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t893) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083a9f7); };
            c.w64(15, t893);
            let Some(t894) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083a9fe); };
            c.cmp(t894, c.r64(12), 64);
            if c.cc_be() { blk = 977; continue; }
            blk = 637; continue;
        }
        637 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083aa07); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 638; continue;
        }
        638 => {
            let Some(t895) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700083aa0e); };
            c.w64(11, t895);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700083aa15); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 640; continue; }
            blk = 639; continue;
        }
        639 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x700083aa1f); }
            let r896 = c.neg(c.r64(8), 64);
            c.w64(8, r896);
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700083aa2a); }
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        640 => {
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700083aa36); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700083aa3e); }
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        641 => {
            let Some(t897) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083aa4a); };
            c.w64(8, t897);
            let Some(t898) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700083aa51); };
            c.w64(9, t898);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t899) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083aa58); };
            c.w64(11, t899);
            let Some(t900) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700083aa5f); };
            c.cmp(t900, c.r64(12), 64);
            if c.cc_be() { blk = 979; continue; }
            blk = 642; continue;
        }
        642 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700083aa68); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 643; continue;
        }
        643 => {
            let Some(t901) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700083aa6f); };
            c.w64(9, t901);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700083aa76); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700083aa7a); }
            c.w64(0, c.r64(8));
            blk = 68; continue;
        }
        644 => {
            let r902 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r902);
            blk = 645; continue;
        }
        645 => {
            let Some(t903) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8))) else { return RegfileExit::fault(0x700083aa8d); };
            c.w64(8, t903);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 649; continue; }
            blk = 646; continue;
        }
        646 => {
            let Some(t904) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083aa97); };
            c.w64(9, t904);
            let Some(t905) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083aa9e); };
            c.w64(11, t905);
            c.w64(12, c.r64(11).wrapping_add(0x10));
            let Some(t906) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083aaa5); };
            c.w64(15, t906);
            let Some(t907) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083aaac); };
            c.cmp(t907, c.r64(12), 64);
            if c.cc_be() { blk = 981; continue; }
            blk = 647; continue;
        }
        647 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083aab5); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 648; continue;
        }
        648 => {
            let Some(t908) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700083aabc); };
            c.w64(11, t908);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700083aac3); }
            if !c.st32(c.r64(9).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x700083aac7); }
            if !c.st64(c.r64(9).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x700083aacf); }
            c.w64(0, c.r64(9));
            blk = 68; continue;
        }
        649 => {
            let Some(t909) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083aadb); };
            c.w64(8, t909);
            let Some(t910) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700083aae2); };
            c.w64(9, t910);
            c.w64(12, c.r64(9).wrapping_add(0x8));
            let Some(t911) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083aae9); };
            c.w64(11, t911);
            let Some(t912) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700083aaf0); };
            c.cmp(t912, c.r64(12), 64);
            if c.cc_be() { blk = 983; continue; }
            blk = 650; continue;
        }
        650 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700083aaf9); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 651; continue;
        }
        651 => {
            let Some(t913) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x700083ab00); };
            c.w64(9, t913);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700083ab07); }
            if !c.st32(c.r64(8).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x700083ab0b); }
            c.w64(0, c.r64(8));
            blk = 68; continue;
        }
        652 => {
            c.cmp(c.r16(12), 0x40u64, 16);
            if c.cc_ae() { blk = 985; continue; }
            blk = 653; continue;
        }
        653 => {
            let Some(t914) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ab26); };
            c.w32(11, t914);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ae() { blk = 986; continue; }
            blk = 654; continue;
        }
        654 => {
            let Some(t915) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083ab33); };
            c.w64(5, t915);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(9));
            let r916 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r916);
            return RegfileExit::jump(0x700089f714);
        }
        655 => {
            let Some(t917) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083ab48); };
            c.w64(5, t917);
            let Some(t918) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083ab4c); };
            c.w64(3, t918);
            c.w32(0, 0x2u64);
            let Some(t919) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x700083ab58); };
            c.w64(10, t919);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700083ab5d); }
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(2)) { return RegfileExit::fault(0x700083ab62); }
            let Some(t920) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x700083ab67); };
            c.w64(10, t920);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x700083ab6c); }
            let Some(t921) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083ab71); };
            c.w64(6, t921);
            let r922 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r922);
            return RegfileExit::jump(0x700086ed74);
        }
        656 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 658; continue; }
            blk = 657; continue;
        }
        657 => {
            let Some(t923) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ab86); };
            c.w32(8, t923);
            let r924 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r924);
            blk = 215; continue;
        }
        658 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1af4, 0x700083ab9a, 0x700083ab95);
        }
        659 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 710; continue; }
            blk = 660; continue;
        }
        660 => {
            let Some(t925) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083aba2); };
            c.w32(8, t925);
            let r926 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r926);
            let Some(t927) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083aba9); };
            c.w32(8, t927);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 664; continue; }
            blk = 661; continue;
        }
        661 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700083abb5); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 664; continue; }
            blk = 662; continue;
        }
        662 => {
            if c.cc_ne() { blk = 664; continue; }
            blk = 663; continue;
        }
        663 => {
            let Some(t928) = c.ld64(0x7000838757u64) else { return RegfileExit::fault(0x700083abd0); };
            c.w64(11, t928);
            let r929 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r929);
            let Some(t930) = c.ld64(0x7000838764u64) else { return RegfileExit::fault(0x700083abda); };
            c.w64(12, t930);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 711; continue; }
            blk = 664; continue;
        }
        664 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 709; continue; }
            blk = 665; continue;
        }
        665 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_b() { blk = 669; continue; }
            blk = 666; continue;
        }
        666 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_e() { blk = 668; continue; }
            blk = 667; continue;
        }
        667 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700083ac03); }
            let Some(t931) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ac07); };
            c.w64(2, t931);
            blk = 655; continue;
        }
        668 => {
            let Some(t932) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ac10); };
            c.w32(8, t932);
            let r933 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r933);
            c.w64(1, c.r64(0));
            let Some(t934) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083ac1a); };
            c.w64(3, t934);
            let Some(t935) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ac1e); };
            c.w64(2, t935);
            let Some(t936) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ac22); };
            c.w64(6, t936);
            let Some(t937) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083ac26); };
            c.w64(0, t937);
            blk = 215; continue;
        }
        669 => {
            let Some(t938) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ac2f); };
            c.w32(9, t938);
            c.logic(c.r32(9) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 708; continue; }
            blk = 670; continue;
        }
        670 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 707; continue; }
            blk = 671; continue;
        }
        671 => {
            c.cmp(c.r16(8), 0x25u64, 16);
            if c.cc_e() { blk = 706; continue; }
            blk = 672; continue;
        }
        672 => {
            c.cmp(c.r16(8), 0x2du64, 16);
            if c.cc_e() { blk = 706; continue; }
            blk = 673; continue;
        }
        673 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 675; continue; }
            blk = 674; continue;
        }
        674 => {
            c.w64(1, c.r64(0));
            c.w64(8, c.r64(0));
            let Some(t939) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ac6c); };
            c.w64(2, t939);
            let Some(t940) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ac70); };
            c.w64(6, t940);
            let Some(t941) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083ac74); };
            c.w64(0, t941);
            let Some(t942) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083ac78); };
            c.w64(3, t942);
            blk = 215; continue;
        }
        675 => {
            c.w64(1, c.r64(0));
            blk = 16; continue;
        }
        676 => {
            let Some(t943) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ac89); };
            c.w64(2, t943);
            blk = 655; continue;
        }
        677 => {
            let Some(t944) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ac92); };
            c.w64(2, t944);
            blk = 655; continue;
        }
        678 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700083aca0); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700083aca4); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700083aca8); }
            return c.call(0x70007febf4, 0x700083acb2, 0x700083acad);
        }
        679 => {
            c.w64(8, c.r64(0));
            let Some(t945) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083acb5); };
            c.w64(12, t945);
            let Some(t946) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083acb9); };
            c.w64(0, t946);
            let Some(t947) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083acbd); };
            c.w64(3, t947);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700083acc1); }
            blk = 52; continue;
        }
        680 => {
            return RegfileExit::fault(0x700083accb);
        }
        681 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        682 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        683 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        684 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083ace4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083ace6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083ace7); }
            let Some(t948) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083ace9); };
            c.w64(3, t948);
            c.w32(0, 0x3u64);
            let Some(t949) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083acf5); };
            c.w64(6, t949);
            return c.call(0x700086ed74, 0x700083acfe, 0x700083acf9);
        }
        685 => {
            blk = 68; continue;
        }
        686 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        687 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        688 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        689 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        690 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        691 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        692 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        693 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        694 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        695 => {
            let Some(t950) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ad4b); };
            c.w64(2, t950);
            blk = 655; continue;
        }
        696 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700083ad54); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x700083ad5d); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700083ad61); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700083ad65); }
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700083ad72, 0x700083ad6d);
        }
        697 => {
            c.w64(9, c.r64(0));
            let Some(t951) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083ad75); };
            c.w64(0, t951);
            let Some(t952) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083ad79); };
            c.w64(8, t952);
            let Some(t953) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083ad7d); };
            c.w64(2, t953);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700083ad81); }
            blk = 163; continue;
        }
        698 => {
            return RegfileExit::fault(0x700083ad8b);
        }
        699 => {
            blk = 206; continue;
        }
        700 => {
            return RegfileExit::fault(0x700083ad91);
        }
        701 => {
            blk = 206; continue;
        }
        702 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083ad97); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083ad99); }
            let Some(t954) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ad9a); };
            if !c.push64(t954) { return RegfileExit::fault(0x700083ad9a); }
            let Some(t955) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083ad9d); };
            c.w64(3, t955);
            c.w32(0, 0x3u64);
            let Some(t956) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ada9); };
            c.w64(6, t956);
            return c.call(0x700086ed74, 0x700083adb2, 0x700083adad);
        }
        703 => {
            blk = 68; continue;
        }
        704 => {
            let Some(t957) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083adb7); };
            c.w64(2, t957);
            blk = 655; continue;
        }
        705 => {
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        706 => {
            let Some(t958) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083adc8); };
            c.w32(8, t958);
            let r959 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r959);
            c.w64(1, c.r64(0));
            let Some(t960) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083add2); };
            c.w64(3, t960);
            let Some(t961) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083add6); };
            c.w64(2, t961);
            let Some(t962) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083adda); };
            c.w64(6, t962);
            let Some(t963) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083adde); };
            c.w64(0, t963);
            blk = 215; continue;
        }
        707 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700083ade7); }
            let Some(t964) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083adeb); };
            c.w64(2, t964);
            blk = 655; continue;
        }
        708 => {
            if let Some(sr) = c.shr(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let r965 = c.r64(9) & 0xffffffu64;
            c.w64(9, r965);
            c.logic(r965, 64);
            c.w64(1, c.r64(0));
            let Some(t966) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ae02); };
            c.w64(6, t966);
            let Some(t967) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ae06); };
            c.w64(2, t967);
            blk = 477; continue;
        }
        709 => {
            c.w64(1, c.r64(0));
            c.w64(8, c.r64(0));
            let Some(t968) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ae15); };
            c.w64(2, t968);
            let Some(t969) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ae19); };
            c.w64(6, t969);
            let Some(t970) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083ae1d); };
            c.w64(0, t970);
            let Some(t971) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083ae21); };
            c.w64(3, t971);
            blk = 215; continue;
        }
        710 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            blk = 711; continue;
        }
        711 => {
            c.w64(1, c.r64(0));
            let Some(t972) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083ae36); };
            c.w64(6, t972);
            let Some(t973) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ae3a); };
            c.w64(2, t973);
            blk = 477; continue;
        }
        712 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700083ae48); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700083ae4c); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x700083ae50); }
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700083ae5d, 0x700083ae58);
        }
        713 => {
            c.w64(9, c.r64(0));
            let Some(t974) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083ae60); };
            c.w64(2, t974);
            let Some(t975) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083ae64); };
            c.w64(8, t975);
            let Some(t976) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083ae68); };
            c.w64(3, t976);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x700083ae6c); }
            blk = 246; continue;
        }
        714 => {
            let Some(t977) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ae76); };
            c.w32(15, t977);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(12)) { return RegfileExit::fault(0x700083ae7a); }
            let Some(t978) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700083ae7e); };
            c.cmp(t978, c.r32(15), 32);
            if c.cc_e() { blk = 792; continue; }
            blk = 715; continue;
        }
        715 => {
            let Some(t979) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ae8b); };
            c.w32(15, t979);
            let r980 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r980);
            c.w32(0, 0);
            c.logic(0, 32);
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_e() { blk = 717; continue; }
            blk = 716; continue;
        }
        716 => {
            let Some(t981) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ae99); };
            c.cmp(t981, c.r32(0), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 717; continue;
        }
        717 => {
            let Some(t982) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083aea3); };
            c.w32(15, t982);
            c.w64(2, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700083aead); }
            c.logic(c.r8(2) & 0x30u64, 8);
            if c.cc_e() { blk = 787; continue; }
            blk = 718; continue;
        }
        718 => {
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 779; continue; }
            blk = 719; continue;
        }
        719 => {
            c.w64(1, c.r64(12));
            let r983 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r983);
            let Some(t984) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083aec9); };
            c.w64(2, t984);
            let Some(t985) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083aecd); };
            c.w32(6, t985);
            let r986 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r986);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_ne() { blk = 721; continue; }
            blk = 720; continue;
        }
        720 => {
            let Some(t987) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083aed9); };
            c.w64(6, t987);
            blk = 721; continue;
        }
        721 => {
            let Some(t988) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083aee0); };
            c.w32(7, t988);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            let r989 = c.sub(c.r64(7), 0x1u64, 64);
            c.w64(7, r989);
            let r990 = c.r64(1) & c.r64(7);
            c.w64(1, r990);
            c.logic(r990, 64);
            blk = 722; continue;
        }
        722 => {
            c.w64(3, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(12, c.r64(3).wrapping_mul(4));
            let Some(t991) = c.ld32(c.r64(6).wrapping_add(c.r64(12)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083aefd); };
            c.w32(15, t991);
            let Some(t992) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083af02); };
            c.cmp(t992, c.r32(15), 32);
            if c.cc_e() { blk = 778; continue; }
            blk = 723; continue;
        }
        723 => {
            c.cmp(c.r32(15), c.r32(8), 32);
            if c.cc_e() { blk = 725; continue; }
            blk = 724; continue;
        }
        724 => {
            let r993 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r993);
            c.w64(12, c.r64(0).wrapping_add(c.r64(1)));
            c.w64(15, c.r64(7));
            let r994 = c.r64(15) & c.r64(12);
            c.w64(15, r994);
            c.logic(r994, 64);
            c.w64(1, c.r64(15));
            blk = 722; continue;
        }
        725 => {
            let Some(t995) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083af27); };
            c.w32(11, t995);
            let Some(t996) = c.ld32(c.r64(6).wrapping_add(c.r64(12)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083af2b); };
            c.w32(15, t996);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_b() { blk = 727; continue; }
            blk = 726; continue;
        }
        726 => {
            return RegfileExit::fault(0x700083af3f);
        }
        727 => {
            let Some(t997) = c.ld32(c.r64(6).wrapping_add(c.r64(12)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083af40); };
            c.w32(0, t997);
            let r998 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r998);
            c.logic(c.r8(15) & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 728; continue;
        }
        728 => {
            let Some(t999) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083af52); };
            c.w32(11, t999);
            let Some(t1000) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083af56); };
            c.cmp(t1000, c.r32(11), 32);
            if c.cc_ne() { blk = 753; continue; }
            blk = 729; continue;
        }
        729 => {
            let Some(t1001) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083af63); };
            c.w32(11, t1001);
            let r1002 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1002);
            let Some(t1003) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083af6a); };
            c.w32(12, t1003);
            let r1004 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r1004);
            let Some(t1005) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083af71); };
            c.logic(t1005 & 0x2u64, 8);
            if c.cc_ne() { blk = 751; continue; }
            blk = 730; continue;
        }
        730 => {
            let Some(t1006) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083af7d); };
            c.cmp(t1006, c.r32(12), 32);
            if c.cc_e() { blk = 732; continue; }
            blk = 731; continue;
        }
        731 => {
            let Some(t1007) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083af86); };
            c.w64(0, t1007);
            blk = 68; continue;
        }
        732 => {
            let Some(t1008) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083af92); };
            c.w64(12, t1008);
            let Some(t1009) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x700083af99); };
            c.cmp(t1009, 0x20u64, 8);
            if c.cc_e() { blk = 749; continue; }
            blk = 733; continue;
        }
        733 => {
            let Some(t1010) = c.ld32(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x700083afa4); };
            c.w32(12, t1010);
            let Some(t1011) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083afa8); };
            c.cmp(t1011, c.r32(12), 32);
            if c.cc_e() { blk = 735; continue; }
            blk = 734; continue;
        }
        734 => {
            let Some(t1012) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083afb1); };
            c.w64(3, t1012);
            blk = 314; continue;
        }
        735 => {
            let Some(t1013) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083afba); };
            c.w32(12, t1013);
            let Some(t1014) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083afbe); };
            c.cmp(t1014, c.r32(12), 32);
            if c.cc_ne() { blk = 737; continue; }
            blk = 736; continue;
        }
        736 => {
            let Some(t1015) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083afc7); };
            c.w32(9, t1015);
            let r1016 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1016);
            let Some(t1017) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083afce); };
            c.w32(12, t1017);
            let Some(t1018) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083afd2); };
            c.cmp(t1018, c.r32(12), 32);
            if c.cc_e() { blk = 736; continue; }
            blk = 737; continue;
        }
        737 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 748; continue; }
            blk = 738; continue;
        }
        738 => {
            let Some(t1019) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083afe5); };
            c.w32(12, t1019);
            let r1020 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r1020);
            let Some(t1021) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083afec); };
            c.w32(15, t1021);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 744; continue; }
            blk = 739; continue;
        }
        739 => {
            c.w32(9, c.r16(15));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 743; continue; }
            blk = 740; continue;
        }
        740 => {
            let Some(t1022) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083b00b); };
            c.cmp(t1022, c.r32(12), 32);
            if c.cc_e() { blk = 742; continue; }
            blk = 741; continue;
        }
        741 => {
            c.w64(9, c.r64(2));
            blk = 744; continue;
        }
        742 => {
            let Some(t1023) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b019); };
            c.w64(3, t1023);
            blk = 314; continue;
        }
        743 => {
            let Some(t1024) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b022); };
            c.w32(9, t1024);
            let r1025 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1025);
            blk = 744; continue;
        }
        744 => {
            let Some(t1026) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b029); };
            c.w32(9, t1026);
            let r1027 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1027);
            let Some(t1028) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b030); };
            c.w32(9, t1028);
            let r1029 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1029);
            let Some(t1030) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b037); };
            c.w32(12, t1030);
            let r1031 = c.sub(c.r32(12), 0x89u64, 32);
            c.w32(12, r1031);
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_a() { blk = 747; continue; }
            blk = 745; continue;
        }
        745 => {
            let Some(t1032) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083b049); };
            c.w32(6, t1032);
            let r1033 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r1033);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083b050); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(11));
            return c.call(0x70007f6fb4, 0x700083b05b, 0x700083b056);
        }
        746 => {
            blk = 68; continue;
        }
        747 => {
            let Some(t1034) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b060); };
            c.w64(3, t1034);
            blk = 314; continue;
        }
        748 => {
            let Some(t1035) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b069); };
            c.w64(3, t1035);
            blk = 314; continue;
        }
        749 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083b072); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083b073); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083b075); }
            let Some(t1036) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083b076); };
            c.w64(3, t1036);
            c.w32(0, 0x3u64);
            let Some(t1037) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b082); };
            c.w64(6, t1037);
            return c.call(0x700086ed74, 0x700083b08b, 0x700083b086);
        }
        750 => {
            blk = 68; continue;
        }
        751 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083b090); }
            c.w64(7, c.r64(11));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t1038) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b096); };
            c.w64(6, t1038);
            return c.call(0x70007f2274, 0x700083b09f, 0x700083b09a);
        }
        752 => {
            blk = 68; continue;
        }
        753 => {
            let Some(t1039) = c.ld16(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083b0a4); };
            c.w32(11, t1039);
            c.cmp(c.r32(11), 0x834u64, 32);
            if c.cc_e() { blk = 775; continue; }
            blk = 754; continue;
        }
        754 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 764; continue; }
            blk = 755; continue;
        }
        755 => {
            c.cmp(c.r32(11), 0x411u64, 32);
            if c.cc_e() { blk = 757; continue; }
            blk = 756; continue;
        }
        756 => {
            let Some(t1040) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b0cc); };
            c.w64(3, t1040);
            blk = 314; continue;
        }
        757 => {
            let Some(t1041) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b0d5); };
            c.w32(9, t1041);
            let Some(t1042) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083b0d9); };
            c.cmp(t1042, c.r32(9), 32);
            if c.cc_e() { blk = 759; continue; }
            blk = 758; continue;
        }
        758 => {
            let Some(t1043) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b0e2); };
            c.w64(3, t1043);
            blk = 314; continue;
        }
        759 => {
            let Some(t1044) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b0eb); };
            c.w32(9, t1044);
            let r1045 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1045);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 761; continue; }
            blk = 760; continue;
        }
        760 => {
            let Some(t1046) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b0f8); };
            c.w64(3, t1046);
            blk = 314; continue;
        }
        761 => {
            let Some(t1047) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b101); };
            c.w32(11, t1047);
            let r1048 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1048);
            let Some(t1049) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b108); };
            c.cmp(t1049, 0x40u64, 16);
            if c.cc_b() { blk = 763; continue; }
            blk = 762; continue;
        }
        762 => {
            let Some(t1050) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b110); };
            c.w64(3, t1050);
            blk = 314; continue;
        }
        763 => {
            let Some(t1051) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b119); };
            c.w32(8, t1051);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        764 => {
            let Some(t1052) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b126); };
            c.w32(11, t1052);
            let Some(t1053) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700083b12a); };
            c.cmp(t1053, c.r32(11), 32);
            if c.cc_e() { blk = 766; continue; }
            blk = 765; continue;
        }
        765 => {
            let Some(t1054) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b133); };
            c.w64(3, t1054);
            blk = 314; continue;
        }
        766 => {
            let Some(t1055) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083b13c); };
            c.w32(9, t1055);
            c.w64(11, c.r64(9));
            let r1056 = c.r32(11) & 0xc0u64;
            c.w32(11, r1056);
            c.logic(r1056, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 769; continue; }
            blk = 767; continue;
        }
        767 => {
            let Some(t1057) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b154); };
            c.w32(11, t1057);
            let r1058 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1058);
            let Some(t1059) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083b15b); };
            c.w32(11, t1059);
            let r1060 = c.r32(11) & 0x1fu64;
            c.w32(11, r1060);
            c.logic(r1060, 32);
            let r1061 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r1061);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 769; continue; }
            blk = 768; continue;
        }
        768 => {
            let Some(t1062) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b16d); };
            c.w64(3, t1062);
            blk = 314; continue;
        }
        769 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 774; continue; }
            blk = 770; continue;
        }
        770 => {
            let Some(t1063) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083b17c); };
            c.w32(0, t1063);
            let r1064 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1064);
            let Some(t1065) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083b182); };
            c.cmp(t1065, c.r32(0), 32);
            if c.cc_e() { blk = 773; continue; }
            blk = 771; continue;
        }
        771 => {
            let Some(t1066) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b18b); };
            c.w32(8, t1066);
            let Some(t1067) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083b18f); };
            c.cmp(t1067, c.r32(8), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 772; continue;
        }
        772 => {
            let Some(t1068) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b19c); };
            c.w32(0, t1068);
            let r1069 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1069);
            blk = 68; continue;
        }
        773 => {
            let Some(t1070) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b1a7); };
            c.w64(3, t1070);
            blk = 314; continue;
        }
        774 => {
            let Some(t1071) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b1b0); };
            c.w64(3, t1071);
            blk = 314; continue;
        }
        775 => {
            let Some(t1072) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b1b9); };
            c.w32(9, t1072);
            let Some(t1073) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083b1bd); };
            c.cmp(t1073, c.r32(9), 32);
            if c.cc_e() { blk = 777; continue; }
            blk = 776; continue;
        }
        776 => {
            let Some(t1074) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b1c6); };
            c.w64(3, t1074);
            blk = 314; continue;
        }
        777 => {
            let Some(t1075) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b1cf); };
            c.w32(0, t1075);
            let r1076 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1076);
            blk = 68; continue;
        }
        778 => {
            let Some(t1077) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083b1da); };
            c.w64(3, t1077);
            let Some(t1078) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083b1de); };
            c.w64(12, t1078);
            let Some(t1079) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083b1e2); };
            c.w64(15, t1079);
            blk = 787; continue;
        }
        779 => {
            let Some(t1080) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b1e8); };
            c.w32(0, t1080);
            let r1081 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1081);
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 780; continue;
        }
        780 => {
            let r1082 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r1082);
            c.logic(r1082, 64);
            let Some(t1083) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b1fc); };
            c.w64(6, t1083);
            let Some(t1084) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b200); };
            c.w32(2, t1084);
            let r1085 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1085);
            let Some(t1086) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083b206); };
            c.w32(2, t1086);
            let r1087 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1087);
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_e() { blk = 787; continue; }
            blk = 781; continue;
        }
        781 => {
            let Some(t1088) = c.ld16(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083b210); };
            c.w32(1, t1088);
            c.cmp(c.r32(1), 0xbeu64, 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 782; continue;
        }
        782 => {
            let Some(t1089) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b220); };
            c.w32(1, t1089);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xa9u64, 64);
            if c.cc_a() { blk = 784; continue; }
            blk = 783; continue;
        }
        783 => {
            return RegfileExit::fault(0x700083b232);
        }
        784 => {
            let Some(t1090) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b233); };
            c.w32(1, t1090);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t1091) = c.ld32(c.r64(0).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083b239); };
            c.w32(0, t1091);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0xa9u64, 64);
            if c.cc_a() { blk = 786; continue; }
            blk = 785; continue;
        }
        785 => {
            return RegfileExit::fault(0x700083b24b);
        }
        786 => {
            let Some(t1092) = c.ld32(c.r64(2).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083b24c); };
            c.cmp(t1092, c.r32(0), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 787; continue;
        }
        787 => {
            let Some(t1093) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b258); };
            c.w32(0, t1093);
            let r1094 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1094);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 788; continue;
        }
        788 => {
            let Some(t1095) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700083b267); };
            c.cmp(t1095, c.r32(0), 32);
            if c.cc_e() { blk = 791; continue; }
            blk = 789; continue;
        }
        789 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 790; continue;
        }
        790 => {
            let r1096 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r1096);
            c.logic(r1096, 64);
            c.w64(2, c.r64(0));
            blk = 258; continue;
        }
        791 => {
            let Some(t1097) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b285); };
            c.w64(2, t1097);
            blk = 258; continue;
        }
        792 => {
            let Some(t1098) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083b28e); };
            c.w64(5, t1098);
            let Some(t1099) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700083b292); };
            c.w64(2, t1099);
            c.w64(1, c.r64(8));
            let Some(t1100) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083b29a); };
            c.w64(0, t1100);
            let Some(t1101) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083b29f); };
            c.w64(6, t1101);
            let r1102 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r1102);
            let Some(t1103) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083b2a8); };
            c.logic(t1103 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 794; continue; }
            blk = 793; continue;
        }
        793 => {
            let r1104 = c.add(c.r64(11), 0x3fu64, 64);
            c.w64(11, r1104);
            blk = 795; continue;
        }
        794 => {
            let Some(t1105) = c.ld32(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083b2b8); };
            c.w32(11, t1105);
            let Some(t1106) = c.ld64(c.r64(13).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700083b2bc); };
            c.w64(11, t1106);
            blk = 795; continue;
        }
        795 => {
            return RegfileExit::jump(c.r64(11));
        }
        796 => {
            c.cmp(c.r64(0), 0x2u64, 64);
            if c.cc_e() { blk = 868; continue; }
            blk = 797; continue;
        }
        797 => {
            c.cmp(c.r64(0), 0x6u64, 64);
            if c.cc_e() { blk = 866; continue; }
            blk = 798; continue;
        }
        798 => {
            c.cmp(c.r64(0), 0x7u64, 64);
            if c.cc_e() { blk = 863; continue; }
            blk = 799; continue;
        }
        799 => {
            c.cmp(c.r64(0), 0x8u64, 64);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 852; continue; }
            blk = 800; continue;
        }
        800 => {
            c.cmp(c.r64(0), 0x9u64, 64);
            if c.cc_e() { blk = 852; continue; }
            blk = 801; continue;
        }
        801 => {
            c.cmp(c.r64(0), 0x3u64, 64);
            if c.cc_e() { blk = 812; continue; }
            blk = 802; continue;
        }
        802 => {
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_e() { blk = 865; continue; }
            blk = 803; continue;
        }
        803 => {
            c.cmp(c.r64(0), 0xcu64, 64);
            if c.cc_e() { blk = 810; continue; }
            blk = 804; continue;
        }
        804 => {
            c.cmp(c.r64(0), 0xeu64, 64);
            if c.cc_e() { blk = 806; continue; }
            blk = 805; continue;
        }
        805 => {
            let Some(t1107) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083b329); };
            c.w64(5, t1107);
            let Some(t1108) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x700083b32d); };
            c.w64(10, t1108);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700083b332); }
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(8)) { return RegfileExit::fault(0x700083b33c); }
            let Some(t1109) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700083b341); };
            c.w64(10, t1109);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x700083b346); }
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(2)) { return RegfileExit::fault(0x700083b34b); }
            let Some(t1110) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083b350); };
            c.w64(10, t1110);
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(10)) { return RegfileExit::fault(0x700083b355); }
            let Some(t1111) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083b35a); };
            c.w64(6, t1111);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(3)) { return RegfileExit::fault(0x700083b35f); }
            let Some(t1112) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x700083b364); };
            c.w64(3, t1112);
            let r1113 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r1113);
            return RegfileExit::jump(0x700086ed74);
        }
        806 => {
            let Some(t1114) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b374); };
            c.w32(9, t1114);
            let r1115 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1115);
            let Some(t1116) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b37b); };
            c.w32(9, t1116);
            let r1117 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1117);
            let Some(t1118) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b382); };
            c.w32(11, t1118);
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r1119 = c.r64(15) & 0x1ffffffu64;
            c.w64(15, r1119);
            c.logic(r1119, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 808; continue; }
            blk = 807; continue;
        }
        807 => {
            return RegfileExit::fault(0x700083b39c);
        }
        808 => {
            let Some(t1120) = c.ld32(c.r64(9).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b39d); };
            c.w32(9, t1120);
            let r1121 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1121);
            let Some(t1122) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b3a5); };
            c.w32(0, t1122);
            let r1123 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1123);
            let Some(t1124) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083b3ac); };
            c.cmp(t1124, c.r32(0), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 809; continue;
        }
        809 => {
            let Some(t1125) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083b3b9); };
            c.w64(5, t1125);
            let Some(t1126) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x700083b3bd); };
            c.w64(3, t1126);
            c.w32(0, 0x2u64);
            let Some(t1127) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x700083b3c9); };
            c.w64(10, t1127);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700083b3ce); }
            if !c.st64(c.r64(4).wrapping_add(0x50), 0x174u64) { return RegfileExit::fault(0x700083b3d3); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(8)) { return RegfileExit::fault(0x700083b3dc); }
            let Some(t1128) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083b3e1); };
            c.w64(6, t1128);
            let r1129 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r1129);
            return RegfileExit::jump(0x700086ed74);
        }
        810 => {
            let Some(t1130) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083b3ef); };
            c.w64(5, t1130);
            let Some(t1131) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083b3f3); };
            c.w64(6, t1131);
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(8));
            let Some(t1132) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700083b3fe); };
            c.w64(1, t1132);
            c.w32(2, 0x2u64);
            let r1133 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r1133);
            return RegfileExit::jump(0x70008bc174);
        }
        811 => {
            blk = 865; continue;
        }
        812 => {
            let Some(t1134) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b416); };
            c.w32(0, t1134);
            let r1135 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1135);
            let Some(t1136) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b41c); };
            c.w32(9, t1136);
            let Some(t1137) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083b420); };
            c.cmp(t1137, c.r32(0), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 813; continue;
        }
        813 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 814; continue;
        }
        814 => {
            let Some(t1138) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b437); };
            c.w32(9, t1138);
            let r1139 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1139);
            let Some(t1140) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083b43e); };
            c.cmp(t1140, c.r32(9), 32);
            if c.cc_ne() { blk = 836; continue; }
            blk = 815; continue;
        }
        815 => {
            let Some(t1141) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b44b); };
            c.w32(9, t1141);
            let r1142 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1142);
            let Some(t1143) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b452); };
            c.w32(11, t1143);
            let r1144 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1144);
            let Some(t1145) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083b459); };
            c.logic(t1145 & 0x2u64, 8);
            if c.cc_ne() { blk = 834; continue; }
            blk = 816; continue;
        }
        816 => {
            let Some(t1146) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083b464); };
            c.cmp(t1146, c.r32(11), 32);
            if c.cc_e() { blk = 818; continue; }
            blk = 817; continue;
        }
        817 => {
            let Some(t1147) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083b46d); };
            c.w64(0, t1147);
            blk = 68; continue;
        }
        818 => {
            let Some(t1148) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083b479); };
            c.w64(11, t1148);
            let Some(t1149) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700083b480); };
            c.cmp(t1149, 0x20u64, 8);
            if c.cc_e() { blk = 832; continue; }
            blk = 819; continue;
        }
        819 => {
            let Some(t1150) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x700083b48a); };
            c.w32(11, t1150);
            let Some(t1151) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083b48e); };
            c.cmp(t1151, c.r32(11), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 820; continue;
        }
        820 => {
            let Some(t1152) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b49b); };
            c.w32(11, t1152);
            let r1153 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1153);
            let Some(t1154) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b4a2); };
            c.w32(12, t1154);
            let Some(t1155) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083b4a6); };
            c.cmp(t1155, c.r32(12), 32);
            if c.cc_ne() { blk = 822; continue; }
            blk = 821; continue;
        }
        821 => {
            let Some(t1156) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083b4af); };
            c.w32(11, t1156);
            let r1157 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1157);
            let Some(t1158) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b4b6); };
            c.w32(12, t1158);
            let Some(t1159) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083b4ba); };
            c.cmp(t1159, c.r32(12), 32);
            if c.cc_e() { blk = 821; continue; }
            blk = 822; continue;
        }
        822 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 823; continue;
        }
        823 => {
            let Some(t1160) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b4cd); };
            c.w32(12, t1160);
            let r1161 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r1161);
            let Some(t1162) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b4d4); };
            c.w32(15, t1162);
            c.w32(1, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 828; continue; }
            blk = 824; continue;
        }
        824 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 827; continue; }
            blk = 825; continue;
        }
        825 => {
            let Some(t1163) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083b4f3); };
            c.cmp(t1163, c.r32(12), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 826; continue;
        }
        826 => {
            blk = 829; continue;
        }
        827 => {
            c.w64(2, c.r64(0));
            blk = 829; continue;
        }
        828 => {
            c.w64(2, c.r64(11));
            blk = 829; continue;
        }
        829 => {
            let Some(t1164) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b50a); };
            c.w32(11, t1164);
            let r1165 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1165);
            let Some(t1166) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b511); };
            c.w32(11, t1166);
            let r1167 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1167);
            let Some(t1168) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b518); };
            c.w32(12, t1168);
            let r1169 = c.sub(c.r32(12), 0x89u64, 32);
            c.w32(12, r1169);
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_a() { blk = 314; continue; }
            blk = 830; continue;
        }
        830 => {
            let Some(t1170) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083b52e); };
            c.w32(6, t1170);
            let r1171 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r1171);
            let Some(t1172) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b535); };
            if !c.push64(t1172) { return RegfileExit::fault(0x700083b535); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x700083b542, 0x700083b53d);
        }
        831 => {
            blk = 68; continue;
        }
        832 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083b547); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083b548); }
            let Some(t1173) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b54a); };
            if !c.push64(t1173) { return RegfileExit::fault(0x700083b54a); }
            let Some(t1174) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083b54d); };
            c.w64(3, t1174);
            c.w32(0, 0x3u64);
            let Some(t1175) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b559); };
            c.w64(6, t1175);
            return c.call(0x700086ed74, 0x700083b562, 0x700083b55d);
        }
        833 => {
            blk = 68; continue;
        }
        834 => {
            let Some(t1176) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b567); };
            if !c.push64(t1176) { return RegfileExit::fault(0x700083b567); }
            c.w64(7, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t1177) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b56f); };
            c.w64(6, t1177);
            return c.call(0x70007f2274, 0x700083b578, 0x700083b573);
        }
        835 => {
            blk = 68; continue;
        }
        836 => {
            let Some(t1178) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b57d); };
            c.w32(11, t1178);
            let r1179 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1179);
            let Some(t1180) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b584); };
            c.w32(12, t1180);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 850; continue; }
            blk = 837; continue;
        }
        837 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 843; continue; }
            blk = 838; continue;
        }
        838 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 314; continue; }
            blk = 839; continue;
        }
        839 => {
            let Some(t1181) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b5af); };
            c.w32(11, t1181);
            let Some(t1182) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083b5b3); };
            c.cmp(t1182, c.r32(11), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 840; continue;
        }
        840 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 841; continue;
        }
        841 => {
            let Some(t1183) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b5c8); };
            c.cmp(t1183, 0x40u64, 16);
            if c.cc_ae() { blk = 314; continue; }
            blk = 842; continue;
        }
        842 => {
            let Some(t1184) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b5d4); };
            c.w32(8, t1184);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        843 => {
            let Some(t1185) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b5e1); };
            c.w32(9, t1185);
            let Some(t1186) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700083b5e5); };
            c.cmp(t1186, c.r32(9), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 844; continue;
        }
        844 => {
            let Some(t1187) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083b5f2); };
            c.w32(9, t1187);
            c.w64(11, c.r64(9));
            let r1188 = c.r32(11) & 0xc0u64;
            c.w32(11, r1188);
            c.logic(r1188, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 846; continue; }
            blk = 845; continue;
        }
        845 => {
            let Some(t1189) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083b60a); };
            c.w32(11, t1189);
            let r1190 = c.r32(11) & 0x1fu64;
            c.w32(11, r1190);
            c.logic(r1190, 32);
            let r1191 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r1191);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 314; continue; }
            blk = 846; continue;
        }
        846 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 314; continue; }
            blk = 847; continue;
        }
        847 => {
            let Some(t1192) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083b62a); };
            c.w32(0, t1192);
            let r1193 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1193);
            let Some(t1194) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083b630); };
            c.cmp(t1194, c.r32(0), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 848; continue;
        }
        848 => {
            let Some(t1195) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b63d); };
            c.w32(8, t1195);
            let Some(t1196) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083b641); };
            c.cmp(t1196, c.r32(8), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 849; continue;
        }
        849 => {
            let Some(t1197) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b64e); };
            c.w32(0, t1197);
            let r1198 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1198);
            blk = 68; continue;
        }
        850 => {
            let Some(t1199) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b659); };
            c.w32(9, t1199);
            let Some(t1200) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083b65d); };
            c.cmp(t1200, c.r32(9), 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 851; continue;
        }
        851 => {
            blk = 314; continue;
        }
        852 => {
            let Some(t1201) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083b66f); };
            c.w64(0, t1201);
            let Some(t1202) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x700083b676); };
            c.cmp(t1202, 0x20u64, 8);
            if c.cc_e() { blk = 865; continue; }
            blk = 853; continue;
        }
        853 => {
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 855; continue; }
            blk = 854; continue;
        }
        854 => {
            let Some(t1203) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b685); };
            c.w32(8, t1203);
            let r1204 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1204);
            blk = 856; continue;
        }
        855 => {
            let Some(t1205) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083b68e); };
            c.w32(8, t1205);
            let r1206 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1206);
            blk = 856; continue;
        }
        856 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_ne() { blk = 858; continue; }
            blk = 857; continue;
        }
        857 => {
            return RegfileExit::fault(0x700083b69b);
        }
        858 => {
            let r1207 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r1207);
            c.logic(r1207, 64);
            let Some(t1208) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b6a0); };
            c.w32(11, t1208);
            let r1209 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1209);
            let Some(t1210) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b6a7); };
            c.w32(3, t1210);
            let r1211 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1211);
            let Some(t1212) = c.ld64(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b6ad); };
            c.w64(2, t1212);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 860; continue; }
            blk = 859; continue;
        }
        859 => {
            let Some(t1213) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b6b7); };
            c.w32(9, t1213);
            let r1214 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1214);
            blk = 861; continue;
        }
        860 => {
            let Some(t1215) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b6c0); };
            c.w64(9, t1215);
            blk = 861; continue;
        }
        861 => {
            let Some(t1216) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b6c4); };
            if !c.push64(t1216) { return RegfileExit::fault(0x700083b6c4); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(9));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x700083b6d4, 0x700083b6cf);
        }
        862 => {
            blk = 68; continue;
        }
        863 => {
            let Some(t1217) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083b6d9); };
            c.w64(9, t1217);
            let Some(t1218) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x700083b6e0); };
            c.cmp(t1218, 0x20u64, 8);
            if c.cc_e() { blk = 865; continue; }
            blk = 864; continue;
        }
        864 => {
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r1219 = c.r64(15) & 0x3ffu64;
            c.w64(15, r1219);
            c.logic(r1219, 64);
            let r1220 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(8, r1220);
            let Some(t1221) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b6f5); };
            c.w32(9, t1221);
            let r1222 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1222);
            let Some(t1223) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083b6fc); };
            c.w32(9, t1223);
            let r1224 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1224);
            let Some(t1225) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083b703); };
            c.w32(3, t1225);
            let r1226 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1226);
            let Some(t1227) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083b70b); };
            c.w64(5, t1227);
            c.w64(1, c.r64(2));
            let Some(t1228) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700083b712); };
            c.w64(2, t1228);
            let Some(t1229) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083b717); };
            c.w64(6, t1229);
            let r1230 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r1230);
            return RegfileExit::jump(0x70007fea14);
        }
        865 => {
            let Some(t1231) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083b725); };
            c.w64(5, t1231);
            let Some(t1232) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083b729); };
            c.w64(3, t1232);
            c.w32(0, 0x3u64);
            let Some(t1233) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700083b735); };
            c.w64(10, t1233);
            if !c.st64(c.r64(4).wrapping_add(0x40), c.r64(10)) { return RegfileExit::fault(0x700083b73a); }
            let Some(t1234) = c.ld64(c.r64(4).wrapping_add(0x50)) else { return RegfileExit::fault(0x700083b73f); };
            c.w64(10, t1234);
            let Some(t1235) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700083b744); };
            if !c.push64(t1235) { return RegfileExit::fault(0x700083b744); }
            let sp1236 = c.r64(4);
            let Some(pv1237) = c.ld64(sp1236) else { return RegfileExit::fault(0x700083b748); };
            c.set_rsp(sp1236.wrapping_add(8));
            let pea1238 = c.r64(4).wrapping_add(0x50);
            if !c.st64(pea1238, pv1237) { c.set_rsp(sp1236); return RegfileExit::fault(0x700083b748); }
            if !c.st64(c.r64(4).wrapping_add(0x38), c.r64(10)) { return RegfileExit::fault(0x700083b74c); }
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(8)) { return RegfileExit::fault(0x700083b751); }
            let Some(t1239) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083b756); };
            c.w64(6, t1239);
            let r1240 = c.add(c.r64(4), 0x38u64, 64);
            c.w64(4, r1240);
            return RegfileExit::jump(0x700086ed74);
        }
        866 => {
            if let Some(sr) = c.shr(c.r64(15), 0x6u64, 64) { c.w64(15, sr); }
            let r1241 = c.r64(15) & 0x3ffu64;
            c.w64(15, r1241);
            c.logic(r1241, 64);
            let r1242 = c.imul2(c.r64(15), 0x3u64, 64);
            c.w64(8, r1242);
            let Some(t1243) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b773); };
            c.w32(9, t1243);
            let r1244 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1244);
            let Some(t1245) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083b77a); };
            c.w32(9, t1245);
            let r1246 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1246);
            let Some(t1247) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083b781); };
            c.w32(8, t1247);
            let r1248 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1248);
            let Some(t1249) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b789); };
            c.w32(7, t1249);
            let r1250 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1250);
            let Some(t1251) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b790); };
            if !c.push64(t1251) { return RegfileExit::fault(0x700083b790); }
            let Some(t1252) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b793); };
            c.w64(6, t1252);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2274, 0x700083b79e, 0x700083b799);
        }
        867 => {
            blk = 68; continue;
        }
        868 => {
            let Some(t1253) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b7a3); };
            c.w32(9, t1253);
            let r1254 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1254);
            let r1255 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r1255);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 870; continue; }
            blk = 869; continue;
        }
        869 => {
            let Some(t1256) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083b7b4); };
            c.w64(9, t1256);
            blk = 870; continue;
        }
        870 => {
            let Some(t1257) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b7bb); };
            c.w32(11, t1257);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r1258 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r1258);
            let r1259 = c.r64(12) & c.r64(11);
            c.w64(12, r1259);
            c.logic(r1259, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 871; continue;
        }
        871 => {
            c.w64(0, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(1, c.r64(0).wrapping_mul(4));
            let Some(t1260) = c.ld32(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b7dc); };
            c.w32(6, t1260);
            let Some(t1261) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083b7e1); };
            c.cmp(t1261, c.r32(6), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 872; continue;
        }
        872 => {
            c.cmp(c.r32(6), c.r32(8), 32);
            if c.cc_e() { blk = 874; continue; }
            blk = 873; continue;
        }
        873 => {
            let r1262 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r1262);
            let r1263 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r1263);
            c.w64(0, c.r64(11));
            let r1264 = c.r64(0) & c.r64(12);
            c.w64(0, r1264);
            c.logic(r1264, 64);
            c.w64(12, c.r64(0));
            blk = 871; continue;
        }
        874 => {
            let Some(t1265) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b805); };
            c.w32(11, t1265);
            let Some(t1266) = c.ld32(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b809); };
            c.w32(12, t1266);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_b() { blk = 876; continue; }
            blk = 875; continue;
        }
        875 => {
            return RegfileExit::fault(0x700083b81d);
        }
        876 => {
            let Some(t1267) = c.ld32(c.r64(9).wrapping_add(c.r64(1)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b81e); };
            c.w32(0, t1267);
            let r1268 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1268);
            c.logic(c.r8(12) & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 877; continue;
        }
        877 => {
            let Some(t1269) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b830); };
            c.w32(9, t1269);
            let Some(t1270) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700083b834); };
            c.cmp(t1270, c.r32(9), 32);
            if c.cc_ne() { blk = 899; continue; }
            blk = 878; continue;
        }
        878 => {
            let Some(t1271) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b841); };
            c.w32(9, t1271);
            let r1272 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1272);
            let Some(t1273) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b848); };
            c.w32(11, t1273);
            let r1274 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1274);
            let Some(t1275) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083b84f); };
            c.logic(t1275 & 0x2u64, 8);
            if c.cc_ne() { blk = 897; continue; }
            blk = 879; continue;
        }
        879 => {
            let Some(t1276) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083b85a); };
            c.cmp(t1276, c.r32(11), 32);
            if c.cc_e() { blk = 881; continue; }
            blk = 880; continue;
        }
        880 => {
            let Some(t1277) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083b863); };
            c.w64(0, t1277);
            blk = 68; continue;
        }
        881 => {
            let Some(t1278) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083b86f); };
            c.w64(11, t1278);
            let Some(t1279) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x700083b876); };
            c.cmp(t1279, 0x20u64, 8);
            if c.cc_e() { blk = 895; continue; }
            blk = 882; continue;
        }
        882 => {
            let Some(t1280) = c.ld32(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x700083b880); };
            c.w32(11, t1280);
            let Some(t1281) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083b884); };
            c.cmp(t1281, c.r32(11), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 883; continue;
        }
        883 => {
            let Some(t1282) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b891); };
            c.w32(11, t1282);
            let r1283 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1283);
            let Some(t1284) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b898); };
            c.w32(12, t1284);
            let Some(t1285) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083b89c); };
            c.cmp(t1285, c.r32(12), 32);
            if c.cc_ne() { blk = 885; continue; }
            blk = 884; continue;
        }
        884 => {
            let Some(t1286) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083b8a5); };
            c.w32(11, t1286);
            let r1287 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1287);
            let Some(t1288) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b8ac); };
            c.w32(12, t1288);
            let Some(t1289) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083b8b0); };
            c.cmp(t1289, c.r32(12), 32);
            if c.cc_e() { blk = 884; continue; }
            blk = 885; continue;
        }
        885 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 886; continue;
        }
        886 => {
            let Some(t1290) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b8c3); };
            c.w32(12, t1290);
            let r1291 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r1291);
            let Some(t1292) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b8ca); };
            c.w32(15, t1292);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 891; continue; }
            blk = 887; continue;
        }
        887 => {
            c.w32(11, c.r16(15));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 890; continue; }
            blk = 888; continue;
        }
        888 => {
            let Some(t1293) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x700083b8e9); };
            c.cmp(t1293, c.r32(12), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 889; continue;
        }
        889 => {
            blk = 892; continue;
        }
        890 => {
            let Some(t1294) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b8f8); };
            c.w32(0, t1294);
            let r1295 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1295);
            c.w64(2, c.r64(0));
            blk = 892; continue;
        }
        891 => {
            c.w64(2, c.r64(11));
            blk = 892; continue;
        }
        892 => {
            let Some(t1296) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083b906); };
            c.w32(11, t1296);
            let r1297 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1297);
            let Some(t1298) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b90d); };
            c.w32(11, t1298);
            let r1299 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1299);
            let Some(t1300) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b914); };
            c.w32(12, t1300);
            let r1301 = c.sub(c.r32(12), 0x89u64, 32);
            c.w32(12, r1301);
            c.cmp(c.r32(12), 0x9u64, 32);
            if c.cc_a() { blk = 314; continue; }
            blk = 893; continue;
        }
        893 => {
            let Some(t1302) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083b92a); };
            c.w32(6, t1302);
            let r1303 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r1303);
            let Some(t1304) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b931); };
            if !c.push64(t1304) { return RegfileExit::fault(0x700083b931); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(2, c.r64(9));
            return c.call(0x70007f6fb4, 0x700083b93e, 0x700083b939);
        }
        894 => {
            blk = 68; continue;
        }
        895 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083b943); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083b944); }
            let Some(t1305) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b946); };
            if !c.push64(t1305) { return RegfileExit::fault(0x700083b946); }
            let Some(t1306) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083b949); };
            c.w64(3, t1306);
            c.w32(0, 0x3u64);
            let Some(t1307) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b955); };
            c.w64(6, t1307);
            return c.call(0x700086ed74, 0x700083b95e, 0x700083b959);
        }
        896 => {
            blk = 68; continue;
        }
        897 => {
            let Some(t1308) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083b963); };
            if !c.push64(t1308) { return RegfileExit::fault(0x700083b963); }
            c.w64(7, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t1309) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083b96b); };
            c.w64(6, t1309);
            return c.call(0x70007f2274, 0x700083b974, 0x700083b96f);
        }
        898 => {
            blk = 68; continue;
        }
        899 => {
            let Some(t1310) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b979); };
            c.w32(9, t1310);
            let r1311 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1311);
            let Some(t1312) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b980); };
            c.w32(11, t1312);
            c.cmp(c.r16(11), 0x834u64, 16);
            if c.cc_e() { blk = 913; continue; }
            blk = 900; continue;
        }
        900 => {
            c.w32(12, c.r64(11).wrapping_sub(0x80c));
            c.cmp(c.r32(12), 0xeu64, 32);
            if c.cc_be() { blk = 906; continue; }
            blk = 901; continue;
        }
        901 => {
            c.cmp(c.r16(11), 0x411u64, 16);
            if c.cc_ne() { blk = 314; continue; }
            blk = 902; continue;
        }
        902 => {
            let Some(t1313) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b9aa); };
            c.w32(9, t1313);
            let Some(t1314) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083b9ae); };
            c.cmp(t1314, c.r32(9), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 903; continue;
        }
        903 => {
            let Some(t1315) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083b9bb); };
            c.w32(9, t1315);
            let r1316 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1316);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 904; continue;
        }
        904 => {
            let Some(t1317) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083b9cc); };
            c.w32(11, t1317);
            let r1318 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1318);
            let Some(t1319) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b9d3); };
            c.cmp(t1319, 0x40u64, 16);
            if c.cc_ae() { blk = 314; continue; }
            blk = 905; continue;
        }
        905 => {
            let Some(t1320) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083b9df); };
            c.w32(8, t1320);
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            blk = 68; continue;
        }
        906 => {
            let Some(t1321) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083b9ec); };
            c.w32(11, t1321);
            let Some(t1322) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700083b9f0); };
            c.cmp(t1322, c.r32(11), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 907; continue;
        }
        907 => {
            let Some(t1323) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700083b9fd); };
            c.w32(9, t1323);
            c.w64(11, c.r64(9));
            let r1324 = c.r32(11) & 0xc0u64;
            c.w32(11, r1324);
            c.logic(r1324, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 909; continue; }
            blk = 908; continue;
        }
        908 => {
            let Some(t1325) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ba15); };
            c.w32(11, t1325);
            let r1326 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1326);
            let Some(t1327) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083ba1c); };
            c.w32(11, t1327);
            let r1328 = c.r32(11) & 0x1fu64;
            c.w32(11, r1328);
            c.logic(r1328, 32);
            let r1329 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r1329);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 314; continue; }
            blk = 909; continue;
        }
        909 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 314; continue; }
            blk = 910; continue;
        }
        910 => {
            let Some(t1330) = c.ld32(c.r64(2).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083ba3c); };
            c.w32(0, t1330);
            let r1331 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1331);
            let Some(t1332) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083ba42); };
            c.cmp(t1332, c.r32(0), 32);
            if c.cc_e() { blk = 314; continue; }
            blk = 911; continue;
        }
        911 => {
            let Some(t1333) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ba4f); };
            c.w32(8, t1333);
            let Some(t1334) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083ba53); };
            c.cmp(t1334, c.r32(8), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 912; continue;
        }
        912 => {
            let Some(t1335) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083ba60); };
            c.w32(0, t1335);
            let r1336 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1336);
            blk = 68; continue;
        }
        913 => {
            let Some(t1337) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ba6b); };
            c.w32(9, t1337);
            let Some(t1338) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700083ba6f); };
            c.cmp(t1338, c.r32(9), 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 914; continue;
        }
        914 => {
            let Some(t1339) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083ba7c); };
            c.w32(0, t1339);
            let r1340 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1340);
            blk = 68; continue;
        }
        915 => {
            return RegfileExit::fault(0x700083ba87);
        }
        916 => {
            let Some(t1341) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083ba88); };
            c.w64(9, t1341);
            let Some(t1342) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083ba8f); };
            c.w64(11, t1342);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083ba96); }
            let Some(t1343) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083ba9a); };
            c.w64(15, t1343);
            let Some(t1344) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083baa1); };
            c.cmp(t1344, c.r64(12), 64);
            if c.cc_a() { blk = 919; continue; }
            blk = 917; continue;
        }
        917 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700083bab0, 0x700083baab);
        }
        918 => {
            let Some(t1345) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bab0); };
            c.w64(8, t1345);
            blk = 920; continue;
        }
        919 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083bab6); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 920; continue;
        }
        920 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t1346) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083bac9); };
            c.w64(8, t1346);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083bad0); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083bad4); }
            blk = 68; continue;
        }
        921 => {
            let Some(t1347) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083bade); };
            c.w64(9, t1347);
            let Some(t1348) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083bae5); };
            c.w64(11, t1348);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083baec); }
            let Some(t1349) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083baf0); };
            c.w64(15, t1349);
            let Some(t1350) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083baf7); };
            c.cmp(t1350, c.r64(12), 64);
            if c.cc_a() { blk = 924; continue; }
            blk = 922; continue;
        }
        922 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700083bb06, 0x700083bb01);
        }
        923 => {
            let Some(t1351) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bb06); };
            c.w64(8, t1351);
            blk = 925; continue;
        }
        924 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083bb0c); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 925; continue;
        }
        925 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t1352) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083bb22); };
            c.w64(8, t1352);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083bb29); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083bb2d); }
            blk = 68; continue;
        }
        926 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083bb3c); }
            return c.call(0x70007febb4, 0x700083bb45, 0x700083bb40);
        }
        927 => {
            c.w64(9, c.r64(0));
            let Some(t1353) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bb48); };
            c.w64(8, t1353);
            blk = 302; continue;
        }
        928 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700083bb5b, 0x700083bb56);
        }
        929 => {
            c.w64(8, c.r64(0));
            blk = 307; continue;
        }
        930 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x700083bb68); }
            return c.call(0x70007febf4, 0x700083bb72, 0x700083bb6d);
        }
        931 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700083bb75); }
            blk = 312; continue;
        }
        932 => {
            let Some(t1354) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700083bb7f); };
            c.w64(8, t1354);
            let Some(t1355) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083bb86); };
            c.w32(8, t1355);
            let r1356 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1356);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083bb8d); }
            let Some(t1357) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x700083bb8f); };
            c.w64(3, t1357);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700083bba2, 0x700083bb9d);
        }
        933 => {
            return RegfileExit::fault(0x700083bba2);
        }
        934 => {
            return RegfileExit::fault(0x700083bba3);
        }
        935 => {
            blk = 655; continue;
        }
        936 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083bba9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083bbaa); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083bbac); }
            let Some(t1358) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083bbad); };
            c.w64(3, t1358);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x700083bbbe, 0x700083bbb9);
        }
        937 => {
            blk = 68; continue;
        }
        938 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(2)) { return RegfileExit::fault(0x700083bbc3); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700083bbcc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x700083bbd0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700083bbd4); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700083bbe1, 0x700083bbdc);
        }
        939 => {
            c.w64(11, c.r64(0));
            let Some(t1359) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083bbe4); };
            c.w64(8, t1359);
            let Some(t1360) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083bbe8); };
            c.w64(9, t1360);
            let Some(t1361) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083bbec); };
            c.w64(2, t1361);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700083bbf0); }
            blk = 425; continue;
        }
        940 => {
            return RegfileExit::fault(0x700083bbfa);
        }
        941 => {
            blk = 468; continue;
        }
        942 => {
            return RegfileExit::fault(0x700083bc00);
        }
        943 => {
            blk = 468; continue;
        }
        944 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700083bc06); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083bc08); }
            let Some(t1362) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bc0a); };
            if !c.push64(t1362) { return RegfileExit::fault(0x700083bc0a); }
            let Some(t1363) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x700083bc0d); };
            c.w64(3, t1363);
            c.w32(0, 0x3u64);
            let Some(t1364) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083bc19); };
            c.w64(6, t1364);
            return c.call(0x700086ed74, 0x700083bc22, 0x700083bc1d);
        }
        945 => {
            blk = 68; continue;
        }
        946 => {
            let Some(t1365) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bc27); };
            c.w64(2, t1365);
            blk = 655; continue;
        }
        947 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc30); }
            blk = 655; continue;
        }
        948 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc39); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        949 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc45); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        950 => {
            let Some(t1366) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bc51); };
            c.w64(2, t1366);
            blk = 655; continue;
        }
        951 => {
            return RegfileExit::fault(0x700083bc5a);
        }
        952 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc5b); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        953 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc67); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        954 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc73); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        955 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc7f); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        956 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc8b); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        957 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bc97); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        958 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 961; continue; }
            blk = 959; continue;
        }
        959 => {
            let Some(t1367) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083bcab); };
            c.w32(12, t1367);
            let Some(t1368) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700083bcaf); };
            c.cmp(t1368, c.r32(12), 32);
            if c.cc_e() { blk = 590; continue; }
            blk = 960; continue;
        }
        960 => {
            blk = 962; continue;
        }
        961 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bcbe); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        962 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bcca); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        963 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bcd6); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        964 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083bce2); }
            c.w64(2, c.r64(0));
            blk = 655; continue;
        }
        965 => {
            let Some(t1369) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083bcee); };
            c.w64(9, t1369);
            let Some(t1370) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083bcf5); };
            c.w64(11, t1370);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083bcfc); }
            let Some(t1371) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083bd00); };
            c.w64(15, t1371);
            let Some(t1372) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083bd07); };
            c.cmp(t1372, c.r64(12), 64);
            if c.cc_a() { blk = 968; continue; }
            blk = 966; continue;
        }
        966 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700083bd16, 0x700083bd11);
        }
        967 => {
            let Some(t1373) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bd16); };
            c.w64(8, t1373);
            blk = 969; continue;
        }
        968 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083bd1c); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 969; continue;
        }
        969 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t1374) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083bd32); };
            c.w64(8, t1374);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083bd39); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083bd3d); }
            blk = 68; continue;
        }
        970 => {
            let Some(t1375) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083bd47); };
            c.w64(9, t1375);
            let Some(t1376) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700083bd4e); };
            c.w64(11, t1376);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083bd55); }
            let Some(t1377) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083bd59); };
            c.w64(15, t1377);
            let Some(t1378) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083bd60); };
            c.cmp(t1378, c.r64(12), 64);
            if c.cc_a() { blk = 973; continue; }
            blk = 971; continue;
        }
        971 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700083bd6f, 0x700083bd6a);
        }
        972 => {
            let Some(t1379) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bd6f); };
            c.w64(8, t1379);
            blk = 974; continue;
        }
        973 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700083bd75); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 974; continue;
        }
        974 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t1380) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083bd88); };
            c.w64(8, t1380);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083bd8f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083bd93); }
            blk = 68; continue;
        }
        975 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x700083bda2); }
            return c.call(0x70007febf4, 0x700083bdac, 0x700083bda7);
        }
        976 => {
            c.w64(8, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x700083bdaf); }
            blk = 633; continue;
        }
        977 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083bdbe); }
            return c.call(0x70007febb4, 0x700083bdc7, 0x700083bdc2);
        }
        978 => {
            c.w64(9, c.r64(0));
            let Some(t1381) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bdca); };
            c.w64(8, t1381);
            blk = 638; continue;
        }
        979 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700083bddd, 0x700083bdd8);
        }
        980 => {
            c.w64(8, c.r64(0));
            blk = 643; continue;
        }
        981 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(8)) { return RegfileExit::fault(0x700083bdea); }
            return c.call(0x70007febb4, 0x700083bdf3, 0x700083bdee);
        }
        982 => {
            c.w64(9, c.r64(0));
            let Some(t1382) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083bdf6); };
            c.w64(8, t1382);
            blk = 648; continue;
        }
        983 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x700083be09, 0x700083be04);
        }
        984 => {
            c.w64(8, c.r64(0));
            blk = 651; continue;
        }
        985 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083be11); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        986 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083be1d); }
            c.w64(2, c.r64(8));
            blk = 655; continue;
        }
        987 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083be29); }
            blk = 655; continue;
        }
        988 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700083be32); }
            blk = 655; continue;
        }
        989 => {
            return RegfileExit::jump(0x700083be3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083be54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083be57); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083be5b); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083be61); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083be67); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700083be6b); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083be74); };
            c.w64(7, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(7));
            return RegfileExit::jump(0x7000836434);
        }
        3 => {
            return RegfileExit::jump(0x700083be84);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083be94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083be97); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(7).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700083be9b); };
            c.w64(7, t1);
            c.w64(6, c.r64(7));
            c.w64(3, c.r64(8));
            return RegfileExit::jump(0x7000836434);
        }
        1 => {
            return RegfileExit::jump(0x700083beac);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083beb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083beb7); };
            c.w64(7, t0);
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083bebb); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083bec1); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083bec7); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700083becb); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083bed4); };
            c.w64(7, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(7));
            return RegfileExit::jump(0x70008386d4);
        }
        3 => {
            return RegfileExit::jump(0x700083bee4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083bef4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700083bef4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700083bef8); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700083befe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700083bf02); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700083bf06); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700083bf0a); }
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083bf0e); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_e() { blk = 294; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083bf1f); };
            c.w32(9, t2);
            let r3 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r3);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let r4 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r4);
            c.logic(r4, 64);
            let Some(t5) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083bf3f); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083bf43); };
            c.w32(11, t6);
            let r7 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r7);
            c.w32(12, c.r32(8));
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 252; continue; }
            blk = 4; continue;
        }
        4 => {
            let r8 = c.r32(12) & 0x180u64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(12), 0x100u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083bf87); };
            c.w32(8, t9);
            let Some(t10) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083bf8b); };
            c.cmp(t10, c.r32(8), 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 252; continue;
        }
        10 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700083bfa0); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r11 = c.r64(8) & c.r64(9);
            c.w64(8, r11);
            c.logic(r11, 64);
            let Some(t12) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083bfb2); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 343; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083bfc0); }
            return c.ret(0x0, 0x700083bfc1);
        }
        14 => {
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 13; continue;
        }
        16 => {
            let Some(t13) = c.ld32(c.r64(3).wrapping_add(c.r64(8)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083bfcd); };
            c.w32(8, t13);
            let r14 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r14);
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x700083bfd5); };
            c.cmp(t15, c.r32(8), 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t16) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083bfe2); };
            c.w32(9, t16);
            let r17 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r17);
            let Some(t18) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083bfe9); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083bff0); };
            c.w32(11, t20);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x3u64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x700083c000);
        }
        19 => {
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083c001); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r22);
            c.w32(9, c.r32(8));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700083c00b); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 20; continue;
        }
        20 => {
            let r23 = c.r32(9) & 0x3u64;
            c.w32(9, r23);
            c.logic(r23, 32);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t24) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c027); };
            c.w32(9, t24);
            let Some(t25) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700083c02b); };
            c.cmp(t25, c.r32(9), 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c038); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t28) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c047); };
            c.cmp(t28, c.r32(11), 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c051); };
            c.w32(9, t29);
            c.w32(11, c.r32(9));
            let r30 = c.r32(11) & 0x1eu64;
            c.w32(11, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(11), 0xeu64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(11), 0x12u64, 32);
            if c.cc_e() { blk = 62; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083c070); };
            c.w32(12, t31);
            let r32 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r32);
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 27; continue;
        }
        27 => {
            let r33 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r33);
            c.logic(r33, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x700083c085); }
            c.cmp(c.r32(11), 0xcu64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 46; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(11), 0xau64, 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1af4, 0x700083c0c7, 0x700083c0c2);
        }
        33 => {
            let Some(t34) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083c0c7); };
            c.w64(5, t34);
            c.w64(3, c.r64(0));
            let Some(t35) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083c0ce); };
            c.w64(0, t35);
            let Some(t36) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x700083c0d3); };
            c.w64(1, t36);
            let Some(t37) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083c0d8); };
            c.w64(2, t37);
            let Some(t38) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083c0dd); };
            c.w64(6, t38);
            let r39 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r39);
            return RegfileExit::jump(0x70008be194);
        }
        34 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t40) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083c0f1); };
            c.w32(8, t40);
            let r41 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r41);
            blk = 37; continue;
        }
        36 => {
            let Some(t42) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083c0fa); };
            c.w32(8, t42);
            let r43 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r43);
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r44 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r44);
            c.logic(r44, 64);
            blk = 40; continue;
        }
        39 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 40; continue;
        }
        40 => {
            c.w64(11, c.r64(12));
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c113); };
            c.w32(9, t45);
            let r46 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r46);
            let Some(t47) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083c11a); };
            c.w32(11, t47);
            let r48 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r48);
            let Some(t49) = c.ld64(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c121); };
            c.w64(9, t49);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t50) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c12b); };
            c.w32(12, t50);
            let r51 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r51);
            let Some(t52) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083c132); };
            c.w32(12, t52);
            let r53 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r53);
            blk = 43; continue;
        }
        42 => {
            c.w64(12, c.r64(2));
            blk = 43; continue;
        }
        43 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083c13f); };
            c.w64(15, t54);
            let Some(t55) = c.ld8(c.r64(15)) else { return RegfileExit::fault(0x700083c146); };
            c.cmp(t55, 0x20u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 44; continue;
        }
        44 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083c150); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083c151); }
            c.w32(1, 0x1u64);
            c.w64(2, c.r64(9));
            c.w64(3, c.r64(11));
            c.w64(7, c.r64(12));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x700083c168, 0x700083c163);
        }
        45 => {
            blk = 13; continue;
        }
        46 => {
            let r56 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r56);
            if let Some(sr) = c.shr(c.r64(9), 0x6u64, 64) { c.w64(9, sr); }
            let r57 = c.r64(9) & 0x3ffu64;
            c.w64(9, r57);
            c.logic(r57, 64);
            let r58 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(8, r58);
            let Some(t59) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c17f); };
            c.w32(9, t59);
            let r60 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r60);
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083c187); };
            c.w32(9, t61);
            let r62 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r62);
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083c18e); };
            c.w32(8, t63);
            let r64 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r64);
            let Some(t65) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083c196); };
            c.w64(5, t65);
            let r66 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r66);
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700083c19e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083c1a0); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083c1a2); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083c1a3); }
            let Some(t67) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083c1a4); };
            if !c.push64(t67) { return RegfileExit::fault(0x700083c1a4); }
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2078)) else { return RegfileExit::fault(0x700083c1a8); };
            c.w64(3, t68);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x700083c1b4); }
            return RegfileExit::jump(0x700086ed74);
        }
        47 => {
            let r69 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r69);
            if let Some(sr) = c.shr(c.r64(9), 0x6u64, 64) { c.w64(9, sr); }
            let r70 = c.r64(9) & 0x3ffu64;
            c.w64(9, r70);
            c.logic(r70, 64);
            let r71 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(8, r71);
            let Some(t72) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c1d0); };
            c.w32(9, t72);
            let r73 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r73);
            let Some(t74) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083c1d8); };
            c.w32(9, t74);
            let r75 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r75);
            let Some(t76) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083c1df); };
            c.w32(8, t76);
            let r77 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r77);
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c1e7); };
            c.w32(7, t78);
            let r79 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r79);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083c1ee); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083c1ef); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700083c1fa, 0x700083c1f5);
        }
        48 => {
            blk = 13; continue;
        }
        49 => {
            let Some(t80) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c1ff); };
            c.w32(8, t80);
            let Some(t81) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083c204); };
            c.w32(9, t81);
            let r82 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r82);
            c.w32(11, c.r32(8));
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 252; continue; }
            blk = 50; continue;
        }
        50 => {
            let r83 = c.r32(11) & 0x180u64;
            c.w32(11, r83);
            c.logic(r83, 32);
            c.cmp(c.r32(11), 0x100u64, 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 52; continue;
        }
        52 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t84) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c249); };
            c.w32(8, t84);
            let Some(t85) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c24d); };
            c.cmp(t85, c.r32(8), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 252; continue;
        }
        56 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st32(c.r64(12).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700083c262); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r86 = c.r64(8) & c.r64(12);
            c.w64(8, r86);
            c.logic(r86, 64);
            let Some(t87) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c275); };
            c.logic(t87 & 0x4u64, 8);
            if c.cc_ne() { blk = 340; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 13; continue;
        }
        60 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 61; continue;
        }
        61 => {
            blk = 252; continue;
        }
        62 => {
            let Some(t88) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083c293); };
            c.w64(5, t88);
            let r89 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r89);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083c29b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083c29c); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083c29d); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083c29e); }
            let Some(t90) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083c29f); };
            if !c.push64(t90) { return RegfileExit::fault(0x700083c29f); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700083c2a3); }
            c.w32(0, 0x5u64);
            let Some(t91) = c.ld64(c.r64(13).wrapping_add(0x2090)) else { return RegfileExit::fault(0x700083c2ad); };
            c.w64(3, t91);
            return RegfileExit::jump(0x700086ed74);
        }
        63 => {
            let Some(t92) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c2b9); };
            c.w32(8, t92);
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            let Some(t94) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083c2c0); };
            c.logic(t94 & 0x100000u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t95) = c.ld32(c.r64(8).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083c2ca); };
            c.w32(9, t95);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(10, c.r64(4));
            let r96 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r96);
            let r97 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r97);
            c.logic(r97, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700083c2df); }
            c.w64(1, c.r64(8));
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x700083c2e7); };
            c.w64(0, t98);
            c.w64(10, 0x700083c2f5u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700083c2f5); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700083c2f9); }
            return c.call(c.r64(0), 0x700083c2ff, 0x700083c2fd);
        }
        66 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700083c2ff); }
            let Some(t99) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083c307); };
            c.w64(4, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083c30c); };
            c.w64(0, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083c310); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083c314); };
            c.w64(6, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083c318); };
            c.w64(2, t103);
            blk = 67; continue;
        }
        67 => {
            let Some(t104) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c31c); };
            c.w32(8, t104);
            let r105 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r105);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083c329); };
            c.w64(8, t106);
            blk = 69; continue;
        }
        69 => {
            let Some(t107) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c330); };
            c.w32(9, t107);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t108) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083c337); };
            c.w32(11, t108);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 262; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t109) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c348); };
            c.w32(12, t109);
            let r110 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r110);
            c.w64(15, c.r64(12));
            let r111 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r111);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r112 = c.r64(15) & 0xfffffffffffffffeu64;
            c.w64(15, r112);
            c.logic(r112, 64);
            let r113 = c.add(c.r32(15), c.r32(12), 32);
            c.w32(15, r113);
            c.w32(3, c.r32(11));
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_b() { blk = 92; continue; }
            blk = 71; continue;
        }
        71 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 263; continue; }
            blk = 72; continue;
        }
        72 => {
            let r114 = c.sub(c.r32(3), c.r32(12), 32);
            c.w32(3, r114);
            let r115 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r115);
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083c37e); };
            c.w32(15, t116);
            let r117 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r117);
            c.logic(r117, 64);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_b() { blk = 92; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_be() { blk = 264; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083c399); };
            c.w32(9, t118);
            let r119 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r119);
            c.w32(11, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r32(11), 0xfffffeu64, 32);
            if c.cc_a() { blk = 92; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x700083c3b4); }
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c3b8); };
            c.w32(11, t120);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 265; continue; }
            blk = 76; continue;
        }
        76 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700083c3cf); }
            let Some(t121) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083c3d3); };
            c.w32(11, t121);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r122 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r122);
            let Some(t123) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c3e1); };
            c.w32(12, t123);
            let r124 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r124);
            let r125 = c.r64(12) & c.r64(11);
            c.w64(12, r125);
            c.logic(r125, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 78; continue;
        }
        77 => {
            let r126 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r126);
            let r127 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r127);
            c.w64(3, c.r64(11));
            let r128 = c.r64(3) & c.r64(12);
            c.w64(3, r128);
            c.logic(r128, 64);
            c.w64(12, c.r64(3));
            blk = 78; continue;
        }
        78 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            c.w64(6, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t129) = c.ld32(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c416); };
            c.w32(7, t129);
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083c41b); };
            c.cmp(t130, c.r32(7), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t131) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083c424); };
            c.cmp(t131, c.r32(7), 32);
            if c.cc_ne() { blk = 77; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t132) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c42d); };
            c.w32(11, t132);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 266; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x700083c440); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r133 = c.r64(11) & c.r64(8);
            c.w64(11, r133);
            c.logic(r133, 64);
            let Some(t134) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c453); };
            c.logic(t134 & 0x4u64, 8);
            if c.cc_ne() { blk = 337; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t135) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c45e); };
            c.w32(11, t135);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 267; continue; }
            blk = 84; continue;
        }
        84 => {
            if let Some(sr) = c.shl(c.r64(9), 0x8u64, 64) { c.w64(9, sr); }
            c.w32(11, c.r32(9));
            c.w64(12, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            c.w64(15, c.r64(0));
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x700083c487); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r136 = c.r64(0) & c.r64(8);
            c.w64(0, r136);
            c.logic(r136, 64);
            let Some(t137) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c49b); };
            c.logic(t137 & 0x4u64, 8);
            if c.cc_ne() { blk = 334; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t138) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c4a5); };
            c.w32(12, t138);
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700083c4a9); };
            c.cmp(t139, c.r32(12), 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t140) = c.ld8(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c4b2); };
            c.logic(t140 & 0x1u64, 8);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            let r141 = c.r32(11) | 0x20u64;
            c.w32(11, r141);
            c.logic(r141, 32);
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 90; continue;
        }
        89 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 90; continue;
        }
        90 => {
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c4c4); };
            c.w32(11, t142);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(3).wrapping_add(0x2));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 268; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700083c4db); }
            c.w64(0, c.r64(15));
            blk = 13; continue;
        }
        92 => {
            let Some(t143) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083c4e8); };
            c.w64(5, t143);
            let r144 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r144);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083c4f0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083c4f1); }
            let Some(t145) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083c4f2); };
            if !c.push64(t145) { return RegfileExit::fault(0x700083c4f2); }
            c.w32(0, 0x3u64);
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x700083c4fb); };
            c.w64(3, t146);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700083c502); }
            return RegfileExit::jump(0x700086ed74);
        }
        93 => {
            let Some(t147) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083c50c); };
            c.w64(5, t147);
            let r148 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r148);
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083c514); };
            c.logic(t149 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let r150 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r150);
            blk = 96; continue;
        }
        95 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083c524); };
            c.w32(8, t151);
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700083c528); };
            c.w64(8, t152);
            blk = 96; continue;
        }
        96 => {
            return RegfileExit::jump(c.r64(8));
        }
        97 => {
            c.w64(11, c.r64(0));
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 289; continue; }
            blk = 98; continue;
        }
        98 => {
            let r153 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r153);
            c.logic(r153, 64);
            let Some(t154) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c544); };
            c.w32(9, t154);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700083c548); }
            let Some(t155) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083c54c); };
            c.cmp(t155, c.r32(9), 32);
            if c.cc_e() { blk = 112; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t156) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083c559); };
            c.w32(9, t156);
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083c55d); };
            c.w32(12, t157);
            let r158 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r158);
            c.w32(15, c.r32(9));
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 273; continue; }
            blk = 100; continue;
        }
        100 => {
            let r159 = c.r32(15) & 0x180u64;
            c.w32(15, r159);
            c.logic(r159, 32);
            c.cmp(c.r32(15), 0x100u64, 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 102; continue;
        }
        102 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 106; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 270; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t160) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c5a3); };
            c.w32(9, t160);
            let Some(t161) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c5a7); };
            c.cmp(t161, c.r32(9), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 105; continue;
        }
        105 => {
            blk = 269; continue;
        }
        106 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 271; continue; }
            blk = 107; continue;
        }
        107 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700083c5bd); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r162 = c.r64(9) & c.r64(8);
            c.w64(9, r162);
            c.logic(r162, 64);
            let Some(t163) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c5d1); };
            c.logic(t163 & 0x4u64, 8);
            if c.cc_ne() { blk = 331; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(0, c.r64(11));
            blk = 13; continue;
        }
        110 => {
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ne() { blk = 272; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(0, c.r64(11));
            blk = 13; continue;
        }
        112 => {
            let Some(t164) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083c5f5); };
            c.w32(9, t164);
            let r165 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r165);
            c.w32(12, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t166) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c604); };
            c.cmp(t166, c.r32(12), 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t167) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083c60e); };
            c.w32(9, t167);
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 288; continue; }
            blk = 115; continue;
        }
        115 => {
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r168 = c.r64(9) & 0x3ffu64;
            c.w64(9, r168);
            c.logic(r168, 64);
            c.w64(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t169) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083c633); };
            c.w32(12, t169);
            let r170 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r170);
            let Some(t171) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083c63a); };
            c.w32(15, t171);
            c.w32(0, c.r32(15));
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t172) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083c648); };
            c.cmp(t172, c.r32(11), 32);
            if c.cc_ne() { blk = 275; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083c653); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r173 = c.r64(9) & c.r64(2);
            c.w64(9, r173);
            c.logic(r173, 64);
            let Some(t174) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c661); };
            c.logic(t174 & 0x4u64, 8);
            if c.cc_ne() { blk = 328; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 174; continue;
        }
        119 => {
            let r175 = c.r32(0) & 0x380u64;
            c.w32(0, r175);
            c.logic(r175, 32);
            c.cmp(c.r32(0), 0x80u64, 32);
            if c.cc_e() { blk = 133; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(0), 0x100u64, 32);
            if c.cc_e() { blk = 130; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(0), 0x180u64, 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 276; continue; }
            blk = 123; continue;
        }
        123 => {
            blk = 134; continue;
        }
        124 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 280; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t176) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083c6a4); };
            c.w32(9, t176);
            let r177 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r177);
            c.w32(12, 0x4u64);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 279; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w32(12, 0x2u64);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 128; continue;
        }
        128 => {
            let r178 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r178);
            c.logic(r178, 64);
            let Some(t179) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c6d4); };
            c.cmp(t179, c.r32(9), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 129; continue;
        }
        129 => {
            blk = 277; continue;
        }
        130 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t180) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083c6e5); };
            c.w32(9, t180);
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083c6e9); };
            c.cmp(t181, c.r32(9), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        133 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 281; continue; }
            blk = 134; continue;
        }
        134 => {
            let r182 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r182);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t183) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x700083c70b); };
            c.w32(9, t183);
            let Some(t184) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c710); };
            c.w32(12, t184);
            let r185 = c.r64(15) & 0x3ffu64;
            c.w64(15, r185);
            c.logic(r185, 64);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r186 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r186);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_b() { blk = 161; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 136; continue;
        }
        136 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700083c73e); }
            blk = 139; continue;
        }
        138 => {
            c.w64(15, c.r64(11));
            let r187 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r187);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 139; continue;
        }
        139 => {
            let Some(t188) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083c755); };
            c.w64(15, t188);
            let Some(t189) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083c75c); };
            c.w64(0, t189);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083c763); };
            c.w64(1, t190);
            let Some(t191) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700083c76a); };
            c.cmp(t191, c.r64(3), 64);
            if c.cc_be() { blk = 282; continue; }
            blk = 140; continue;
        }
        140 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700083c773); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 141; continue;
        }
        141 => {
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083c77a); };
            c.w64(0, t192);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700083c781); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083c785); }
            blk = 143; continue;
        }
        142 => {
            c.w64(15, c.r64(11));
            blk = 143; continue;
        }
        143 => {
            let Some(t193) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c790); };
            c.w32(0, t193);
            let r194 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r194);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700083c796); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 145; continue; }
            blk = 144; continue;
        }
        144 => {
            let Some(t195) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c79e); };
            c.w32(3, t195);
            let r196 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r196);
            c.w64(1, c.r64(3));
            let r197 = c.r32(1) & 0x3ffu64;
            c.w32(1, r197);
            c.logic(r197, 32);
            let r198 = c.r32(3) & 0x3ffffc00u64;
            c.w32(3, r198);
            c.logic(r198, 32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 146; continue;
        }
        145 => {
            let r199 = c.shl(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r199);
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083c7c3); };
            c.w64(3, t200);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 146; continue;
        }
        146 => {
            let r201 = c.sub(c.r64(9), c.r64(12), 64);
            c.w64(9, r201);
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_b() { blk = 157; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(12, c.r64(1).wrapping_add(0x3));
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083c7dc); };
            c.w64(6, t202);
            let Some(t203) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700083c7e3); };
            c.w64(7, t203);
            c.w64(11, c.r64(12).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083c7f2); };
            c.w64(15, t204);
            let Some(t205) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083c7f9); };
            c.cmp(t205, c.r64(8), 64);
            if c.cc_be() { blk = 284; continue; }
            blk = 148; continue;
        }
        148 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x700083c802); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 149; continue;
        }
        149 => {
            c.w32(11, c.r32(12));
            c.w32(15, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x700083c810); };
            c.w64(6, t206);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700083c817); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700083c81b); }
            c.w64(15, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.w64(12, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_e() { blk = 152; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083c834); };
            c.w64(1, t207);
            blk = 151; continue;
        }
        151 => {
            let r208 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r208);
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700083c83f); }
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_ne() { blk = 151; continue; }
            blk = 152; continue;
        }
        152 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let r209 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r209);
            let Some(t210) = c.ld32(c.r64(15).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700083c852); };
            c.w32(12, t210);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x700083c856); }
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 153; continue; }
            blk = 154; continue;
        }
        154 => {
            let r211 = c.r32(11) | c.r32(0);
            c.w32(11, r211);
            c.logic(r211, 32);
            let r212 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r212);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700083c866); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700083c86a); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r213 = c.r64(11) & c.r64(2);
            c.w64(11, r213);
            c.logic(r213, 64);
            let Some(t214) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c87e); };
            c.logic(t214 & 0x4u64, 8);
            if c.cc_ne() { blk = 325; continue; }
            blk = 156; continue;
        }
        156 => {
            c.w64(3, c.r64(8));
            blk = 157; continue;
        }
        157 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083c894); };
            c.w64(0, t215);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700083c898); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r216 = c.r64(9) & c.r64(3);
            c.w64(9, r216);
            c.logic(r216, 64);
            let Some(t217) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c8aa); };
            c.logic(t217 & 0x4u64, 8);
            if c.cc_ne() { blk = 322; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083c8b5); };
            c.w64(8, t218);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083c8b9); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r219 = c.r64(9) & c.r64(2);
            c.w64(9, r219);
            c.logic(r219, 64);
            let Some(t220) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c8c7); };
            c.logic(t220 & 0x4u64, 8);
            if c.cc_ne() { blk = 319; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083c8d2); };
            c.w64(11, t221);
            blk = 174; continue;
        }
        161 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 166; continue; }
            blk = 162; continue;
        }
        162 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083c8e8); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r222 = c.r64(12) & c.r64(2);
            c.w64(12, r222);
            c.logic(r222, 64);
            let Some(t223) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c8f6); };
            c.logic(t223 & 0x4u64, 8);
            if c.cc_ne() { blk = 316; continue; }
            blk = 163; continue;
        }
        163 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x700083c902); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r224 = c.r64(8) & c.r64(2);
            c.w64(8, r224);
            c.logic(r224, 64);
            let Some(t225) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c916); };
            c.logic(t225 & 0x4u64, 8);
            if c.cc_ne() { blk = 313; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 174; continue;
        }
        166 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700083c92c); }
            blk = 169; continue;
        }
        168 => {
            c.w64(12, c.r64(11));
            let r226 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r226);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 169; continue;
        }
        169 => {
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083c943); };
            c.w64(12, t227);
            let Some(t228) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700083c94a); };
            c.w64(15, t228);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083c952); };
            c.w64(3, t229);
            let Some(t230) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700083c959); };
            c.cmp(t230, c.r64(0), 64);
            if c.cc_be() { blk = 286; continue; }
            blk = 170; continue;
        }
        170 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700083c962); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 171; continue;
        }
        171 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083c96a); };
            c.w64(15, t231);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700083c971); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083c976); }
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083c97d); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r232 = c.r64(15) & c.r64(2);
            c.w64(15, r232);
            c.logic(r232, 64);
            let Some(t233) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c98b); };
            c.logic(t233 & 0x4u64, 8);
            if c.cc_ne() { blk = 310; continue; }
            blk = 172; continue;
        }
        172 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x700083c996); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r234 = c.r64(8) & c.r64(2);
            c.w64(8, r234);
            c.logic(r234, 64);
            let Some(t235) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083c9aa); };
            c.logic(t235 & 0x4u64, 8);
            if c.cc_ne() { blk = 307; continue; }
            blk = 174; continue;
        }
        174 => {
            c.w64(0, c.r64(11));
            blk = 13; continue;
        }
        175 => {
            c.w64(11, c.r64(8));
            let r236 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r236);
            let r237 = c.r32(9) & 0x1eu64;
            c.w32(9, r237);
            c.logic(r237, 32);
            c.w64(12, c.r64(11));
            let r238 = c.r32(12) & 0xfu64;
            c.w32(12, r238);
            c.logic(r238, 32);
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_l() { blk = 194; continue; }
            blk = 176; continue;
        }
        176 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t239) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083c9f6); };
            c.w32(8, t239);
            let r240 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r240);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083ca03); };
            c.w64(8, t241);
            blk = 181; continue;
        }
        181 => {
            let Some(t242) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083ca0a); };
            c.w32(9, t242);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r243 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r243);
            let Some(t244) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ca18); };
            c.w32(11, t244);
            let r245 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r245);
            let r246 = c.r64(11) & c.r64(9);
            c.w64(11, r246);
            c.logic(r246, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 183; continue;
        }
        182 => {
            let r247 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r247);
            let r248 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r248);
            c.w64(15, c.r64(9));
            let r249 = c.r64(15) & c.r64(11);
            c.w64(15, r249);
            c.logic(r249, 64);
            c.w64(11, c.r64(15));
            blk = 183; continue;
        }
        183 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(15).wrapping_mul(4));
            let Some(t250) = c.ld32(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ca45); };
            c.w32(3, t250);
            let Some(t251) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083ca4a); };
            c.cmp(t251, c.r32(3), 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 184; continue;
        }
        184 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t252) = c.ld32(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083ca5b); };
            c.w32(9, t252);
            c.logic(c.r8(9) & 0x12u64, 8);
            if c.cc_ne() { blk = 291; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t253) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ca6a); };
            c.w32(9, t253);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 290; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w64(9, c.r64(15).wrapping_mul(4).wrapping_add(0xb));
            c.w64(11, c.r64(0));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x700083ca8c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r254 = c.r64(12) & c.r64(8);
            c.w64(12, r254);
            c.logic(r254, 64);
            let Some(t255) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083caa0); };
            c.logic(t255 & 0x4u64, 8);
            if c.cc_ne() { blk = 304; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w64(0, c.r64(11));
            blk = 13; continue;
        }
        190 => {
            let Some(t256) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083cab4); };
            c.w64(5, t256);
            let r257 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r257);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083cabc); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083cabd); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083cabe); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083cabf); }
            let Some(t258) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083cac0); };
            if !c.push64(t258) { return RegfileExit::fault(0x700083cac0); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700083cac4); }
            c.w32(0, 0x5u64);
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2090)) else { return RegfileExit::fault(0x700083cace); };
            c.w64(3, t259);
            return RegfileExit::jump(0x700086ed74);
        }
        191 => {
            let Some(t260) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083cada); };
            c.w64(5, t260);
            let r261 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r261);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083cae2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083cae3); }
            let Some(t262) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083cae4); };
            if !c.push64(t262) { return RegfileExit::fault(0x700083cae4); }
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x20a8)) else { return RegfileExit::fault(0x700083cae8); };
            c.w64(3, t263);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700083caef); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        192 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1af4, 0x700083cb06, 0x700083cb01);
        }
        193 => {
            let Some(t264) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083cb06); };
            c.w64(5, t264);
            c.w64(3, c.r64(0));
            let Some(t265) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083cb0d); };
            c.w64(0, t265);
            let Some(t266) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x700083cb12); };
            c.w64(1, t266);
            let Some(t267) = c.ld64(c.r64(4).wrapping_add(0x30)) else { return RegfileExit::fault(0x700083cb17); };
            c.w64(6, t267);
            c.w64(2, c.r64(0));
            let r268 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r268);
            return RegfileExit::jump(0x70008be194);
        }
        194 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(12, c.r64(11));
            let r269 = c.shr(c.r32(12), 0x11u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r269);
            let r270 = c.r32(12) & 0x7u64;
            c.w32(12, r270);
            c.logic(r270, 32);
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 218; continue; }
            blk = 199; continue;
        }
        199 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t271) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083cb69); };
            c.w32(12, t271);
            let Some(t272) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083cb6d); };
            c.cmp(t272, c.r32(12), 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 201; continue;
        }
        201 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700083cb7a); }
            blk = 203; continue;
        }
        202 => {
            c.w64(12, c.r64(0));
            let r273 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r273);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 203; continue;
        }
        203 => {
            c.w64(12, c.r64(11));
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r274 = c.r64(12) & 0x3ffu64;
            c.w64(12, r274);
            c.logic(r274, 64);
            let r275 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r275);
            let Some(t276) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083cba2); };
            c.w32(15, t276);
            let r277 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r277);
            let Some(t278) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083cba9); };
            c.w32(15, t278);
            let r279 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r279);
            let Some(t280) = c.ld32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083cbb0); };
            c.w32(12, t280);
            c.w32(12, c.r32(12));
            let r281 = c.r32(12) & 0x380u64;
            c.w32(12, r281);
            c.logic(r281, 32);
            c.cmp(c.r32(12), 0x100u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 204; continue;
        }
        204 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(9), 0x2u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r282 = c.r64(11) & 0x7ffu64;
            c.w64(11, r282);
            c.logic(r282, 64);
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t283) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083cbec); };
            c.w32(2, t283);
            let r284 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r284);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083cbf7); };
            c.w64(2, t285);
            blk = 207; continue;
        }
        207 => {
            let Some(t286) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083cbfe); };
            c.w32(8, t286);
            let r287 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r287);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083cc0c); }
            blk = 13; continue;
        }
        209 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700083cc17); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            if c.cc_e() { blk = 216; continue; }
            blk = 211; continue;
        }
        211 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            if c.cc_e() { blk = 252; continue; }
            blk = 213; continue;
        }
        213 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 15; continue; }
            blk = 214; continue;
        }
        214 => {
            if c.cc_e() { blk = 252; continue; }
            blk = 215; continue;
        }
        215 => {
            blk = 15; continue;
        }
        216 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 217; continue;
        }
        217 => {
            blk = 252; continue;
        }
        218 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            let r288 = c.r64(11) & 0x7ffu64;
            c.w64(11, r288);
            c.logic(r288, 64);
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t289) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083cc7a); };
            c.w32(2, t289);
            let r290 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r290);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083cc85); };
            c.w64(2, t291);
            blk = 221; continue;
        }
        221 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 222; continue;
        }
        222 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700083cc9a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r292 = c.r64(9) & c.r64(2);
            c.w64(9, r292);
            c.logic(r292, 64);
            let Some(t293) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083ccac); };
            c.logic(t293 & 0x4u64, 8);
            if c.cc_ne() { blk = 301; continue; }
            blk = 224; continue;
        }
        224 => {
            blk = 13; continue;
        }
        225 => {
            let Some(t294) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ccbc); };
            c.cmp(t294, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 226; continue;
        }
        226 => {
            blk = 252; continue;
        }
        227 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 252; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 232; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(12, c.r64(11));
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r295 = c.r64(12) & 0x3ffu64;
            c.w64(12, r295);
            c.logic(r295, 64);
            let r296 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r296);
            let Some(t297) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ccf8); };
            c.w32(15, t297);
            let r298 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r298);
            let Some(t299) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083ccff); };
            c.w32(15, t299);
            let r300 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r300);
            let Some(t301) = c.ld32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083cd06); };
            c.w32(12, t301);
            let r302 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r302);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 252; continue; }
            blk = 231; continue;
        }
        231 => {
            let r303 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r303);
            c.logic(r303, 64);
            let Some(t304) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083cd22); };
            c.cmp(t304, c.r32(12), 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 232; continue;
        }
        232 => {
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 235; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t305) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083cd35); };
            c.w32(2, t305);
            let r306 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r306);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 235; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083cd40); };
            c.w64(2, t307);
            blk = 235; continue;
        }
        235 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            let r308 = c.r64(11) & 0x7ffu64;
            c.w64(11, r308);
            c.logic(r308, 64);
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 239; continue; }
            blk = 236; continue;
        }
        236 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700083cd60); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r309 = c.r64(9) & c.r64(2);
            c.w64(9, r309);
            c.logic(r309, 64);
            let Some(t310) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083cd72); };
            c.logic(t310 & 0x4u64, 8);
            if c.cc_ne() { blk = 298; continue; }
            blk = 238; continue;
        }
        238 => {
            blk = 13; continue;
        }
        239 => {
            let Some(t311) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083cd82); };
            c.cmp(t311, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 240; continue;
        }
        240 => {
            blk = 252; continue;
        }
        241 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 252; continue; }
            blk = 242; continue;
        }
        242 => {
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 245; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t312) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083cda3); };
            c.w32(2, t312);
            let r313 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r313);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 245; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083cdae); };
            c.w64(2, t314);
            blk = 245; continue;
        }
        245 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            let r315 = c.r64(11) & 0x7ffu64;
            c.w64(11, r315);
            c.logic(r315, 64);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700083cdc6); }
            blk = 13; continue;
        }
        247 => {
            let Some(t316) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083cdd0); };
            c.cmp(t316, c.r32(0), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 248; continue;
        }
        248 => {
            blk = 252; continue;
        }
        249 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r317 = c.r64(11) & 0x3ffu64;
            c.w64(11, r317);
            c.logic(r317, 64);
            let r318 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r318);
            let Some(t319) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083cdef); };
            c.w32(9, t319);
            let r320 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r320);
            let Some(t321) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083cdf6); };
            c.w32(9, t321);
            let r322 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r322);
            let Some(t323) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083cdfd); };
            c.w32(8, t323);
            let r324 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r324);
            let Some(t325) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083ce05); };
            c.w64(5, t325);
            let r326 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r326);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083ce0d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083ce0e); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083ce10); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083ce11); }
            let Some(t327) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083ce12); };
            if !c.push64(t327) { return RegfileExit::fault(0x700083ce12); }
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2078)) else { return RegfileExit::fault(0x700083ce16); };
            c.w64(3, t328);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x700083ce22); }
            return RegfileExit::jump(0x700086ed74);
        }
        250 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r329 = c.r64(11) & 0x3ffu64;
            c.w64(11, r329);
            c.logic(r329, 64);
            let r330 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r330);
            let Some(t331) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ce3b); };
            c.w32(9, t331);
            let r332 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r332);
            let Some(t333) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083ce42); };
            c.w32(9, t333);
            let r334 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r334);
            let Some(t335) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083ce49); };
            c.w32(8, t335);
            let r336 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r336);
            let Some(t337) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083ce51); };
            c.w32(7, t337);
            let r338 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r338);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083ce58); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083ce59); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700083ce64, 0x700083ce5f);
        }
        251 => {
            blk = 13; continue;
        }
        252 => {
            let Some(t339) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083ce69); };
            c.w64(5, t339);
            let r340 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r340);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083ce71); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083ce72); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083ce73); }
            let Some(t341) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083ce74); };
            if !c.push64(t341) { return RegfileExit::fault(0x700083ce74); }
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(0)) { return RegfileExit::fault(0x700083ce78); }
            c.w32(0, 0x4u64);
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0x2080)) else { return RegfileExit::fault(0x700083ce82); };
            c.w64(3, t342);
            return RegfileExit::jump(0x700086ed74);
        }
        253 => {
            let Some(t343) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083ce8e); };
            c.w32(8, t343);
            let r344 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r344);
            let Some(t345) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083ce95); };
            c.w32(8, t345);
            let r346 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r346);
            let Some(t347) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ce9c); };
            c.w32(11, t347);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            return RegfileExit::fault(0x700083ceaf);
        }
        255 => {
            let Some(t348) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x700083ceb0); };
            c.w32(8, t348);
            let r349 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r349);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t350) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083cebd); };
            c.w32(11, t350);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            let r351 = c.r64(12) & 0xfffu64;
            c.w64(12, r351);
            c.logic(r351, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(15, c.r64(12).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ae() { blk = 293; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t352) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083cee2); };
            c.w32(8, t352);
            let r353 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r353);
            let Some(t354) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ceea); };
            c.w32(11, t354);
            if let Some(sr) = c.shr(c.r64(9), 0xcu64, 64) { c.w64(9, sr); }
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let r355 = c.r64(9) & 0x3ffffu64;
            c.w64(9, r355);
            c.logic(r355, 64);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 258; continue; }
            blk = 257; continue;
        }
        257 => {
            return RegfileExit::fault(0x700083cf04);
        }
        258 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x700083cf0d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 260; continue; }
            blk = 259; continue;
        }
        259 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r356 = c.r64(11) & c.r64(8);
            c.w64(11, r356);
            c.logic(r356, 64);
            let Some(t357) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083cf1f); };
            c.logic(t357 & 0x4u64, 8);
            if c.cc_ne() { blk = 295; continue; }
            blk = 260; continue;
        }
        260 => {
            blk = 13; continue;
        }
        261 => {
            blk = 252; continue;
        }
        262 => {
            return RegfileExit::fault(0x700083cf34);
        }
        263 => {
            return RegfileExit::fault(0x700083cf35);
        }
        264 => {
            return RegfileExit::fault(0x700083cf36);
        }
        265 => {
            return RegfileExit::fault(0x700083cf37);
        }
        266 => {
            return RegfileExit::fault(0x700083cf38);
        }
        267 => {
            return RegfileExit::fault(0x700083cf39);
        }
        268 => {
            return RegfileExit::fault(0x700083cf3a);
        }
        269 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        270 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        271 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        272 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        273 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        274 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        275 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        276 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        277 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        278 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        279 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        280 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        281 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        282 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083cfa8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700083cfac); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700083cfb0); }
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x700083cfbd, 0x700083cfb8);
        }
        283 => {
            c.w64(15, c.r64(0));
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083cfc0); };
            c.w64(11, t358);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083cfc4); };
            c.w64(2, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083cfc8); };
            c.w64(8, t360);
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083cfcc); };
            c.w64(9, t361);
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083cfd0); };
            c.w64(12, t362);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700083cfd4); }
            blk = 141; continue;
        }
        284 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083cfde); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700083cfe2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x700083cfe6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700083cfea); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x700083cfee); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700083cffa, 0x700083cff5);
        }
        285 => {
            c.w64(8, c.r64(0));
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083cffd); };
            c.w64(2, t363);
            let Some(t364) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083d001); };
            c.w64(9, t364);
            let Some(t365) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083d005); };
            c.w64(3, t365);
            let Some(t366) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083d009); };
            c.w64(0, t366);
            let Some(t367) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083d00d); };
            c.w64(12, t367);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700083d011); };
            c.w64(1, t368);
            blk = 149; continue;
        }
        286 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083d01f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700083d023); }
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x700083d030, 0x700083d02b);
        }
        287 => {
            c.w64(12, c.r64(0));
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083d033); };
            c.w64(11, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083d037); };
            c.w64(2, t370);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083d03b); };
            c.w64(9, t371);
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083d03f); };
            c.w64(8, t372);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700083d043); }
            blk = 171; continue;
        }
        288 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        289 => {
            c.w64(0, c.r64(11));
            blk = 252; continue;
        }
        290 => {
            return RegfileExit::fault(0x700083d05d);
        }
        291 => {
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083d05e); };
            c.w64(3, t373);
            blk = 252; continue;
        }
        292 => {
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083d067); };
            c.w64(3, t374);
            blk = 252; continue;
        }
        293 => {
            return RegfileExit::fault(0x700083d070);
        }
        294 => {
            let Some(t375) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083d071); };
            c.w64(5, t375);
            let r376 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r376);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083d079); }
            let Some(t377) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d07a); };
            if !c.push64(t377) { return RegfileExit::fault(0x700083d07a); }
            let Some(t378) = c.ld64(c.r64(13).wrapping_add(0x2088)) else { return RegfileExit::fault(0x700083d07e); };
            c.w64(3, t378);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x700083d085); }
            c.w32(0, 0x2u64);
            return RegfileExit::jump(0x700086ed74);
        }
        295 => {
            c.w32(0, c.r32(0));
            let r379 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r379);
            c.w64(11, 0xfffffffffffc0000u64);
            let r380 = c.r64(11) & c.r64(0);
            c.w64(11, r380);
            c.logic(r380, 64);
            let Some(t381) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d0a3); };
            c.logic(t381 & 0x2u64, 8);
            if c.cc_e() { blk = 260; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d0b2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d0b3); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083d0c0, 0x700083d0bb);
        }
        297 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d0c0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d0c2); }
            blk = 260; continue;
        }
        298 => {
            c.w32(0, c.r32(0));
            let r382 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r382);
            c.w64(9, 0xfffffffffffc0000u64);
            let r383 = c.r64(9) & c.r64(0);
            c.w64(9, r383);
            c.logic(r383, 64);
            let Some(t384) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d0d7); };
            c.logic(t384 & 0x2u64, 8);
            if c.cc_e() { blk = 238; continue; }
            blk = 299; continue;
        }
        299 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d0e6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d0e7); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083d0f4, 0x700083d0ef);
        }
        300 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d0f4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d0f6); }
            blk = 238; continue;
        }
        301 => {
            c.w32(0, c.r32(0));
            let r385 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r385);
            c.w64(9, 0xfffffffffffc0000u64);
            let r386 = c.r64(9) & c.r64(0);
            c.w64(9, r386);
            c.logic(r386, 64);
            let Some(t387) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d10b); };
            c.logic(t387 & 0x2u64, 8);
            if c.cc_e() { blk = 224; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(11, c.r64(2).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d11a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d11b); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083d128, 0x700083d123);
        }
        303 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d128); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d12a); }
            blk = 224; continue;
        }
        304 => {
            c.w32(11, c.r32(11));
            let r388 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r388);
            c.w64(12, 0xfffffffffffc0000u64);
            let r389 = c.r64(12) & c.r64(11);
            c.w64(12, r389);
            c.logic(r389, 64);
            let Some(t390) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d140); };
            c.logic(t390 & 0x2u64, 8);
            if c.cc_e() { blk = 189; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d150); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d151); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700083d15e, 0x700083d159);
        }
        306 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d15e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d160); }
            blk = 189; continue;
        }
        307 => {
            c.w32(12, c.r32(12));
            let r391 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r391);
            c.w64(8, 0xfffffffffffc0000u64);
            let r392 = c.r64(8) & c.r64(12);
            c.w64(8, r392);
            c.logic(r392, 64);
            let Some(t393) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d176); };
            c.logic(t393 & 0x2u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d185); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d186); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700083d193, 0x700083d18e);
        }
        309 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d193); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d195); }
            blk = 174; continue;
        }
        310 => {
            c.w32(8, c.r32(8));
            let r394 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r394);
            c.w64(15, 0xfffffffffffc0000u64);
            let r395 = c.r64(15) & c.r64(8);
            c.w64(15, r395);
            c.logic(r395, 64);
            let Some(t396) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d1ab); };
            c.logic(t396 & 0x2u64, 8);
            if c.cc_e() { blk = 172; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(0, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d1ba); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d1bb); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x700083d1c8, 0x700083d1c3);
        }
        312 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d1c8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d1ca); }
            blk = 172; continue;
        }
        313 => {
            c.w32(11, c.r32(11));
            let r397 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r397);
            c.w64(8, 0xfffffffffffc0000u64);
            let r398 = c.r64(8) & c.r64(11);
            c.w64(8, r398);
            c.logic(r398, 64);
            let Some(t399) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d1e0); };
            c.logic(t399 & 0x2u64, 8);
            if c.cc_e() { blk = 165; continue; }
            blk = 314; continue;
        }
        314 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d1ef); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d1f0); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083d1fd, 0x700083d1f8);
        }
        315 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d1fd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d1ff); }
            blk = 165; continue;
        }
        316 => {
            c.w32(8, c.r32(8));
            let r400 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r400);
            c.w64(12, 0xfffffffffffc0000u64);
            let r401 = c.r64(12) & c.r64(8);
            c.w64(12, r401);
            c.logic(r401, 64);
            let Some(t402) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d215); };
            c.logic(t402 & 0x2u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(15, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d225); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d226); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x700083d233, 0x700083d22e);
        }
        318 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d233); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d235); }
            blk = 163; continue;
        }
        319 => {
            c.w32(8, c.r32(8));
            let r403 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r403);
            c.w64(9, 0xfffffffffffc0000u64);
            let r404 = c.r64(9) & c.r64(8);
            c.w64(9, r404);
            c.logic(r404, 64);
            let Some(t405) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d24b); };
            c.logic(t405 & 0x2u64, 8);
            if c.cc_e() { blk = 160; continue; }
            blk = 320; continue;
        }
        320 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d25a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d25b); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x700083d268, 0x700083d263);
        }
        321 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d268); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d26a); }
            blk = 160; continue;
        }
        322 => {
            c.w32(0, c.r32(0));
            let r406 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r406);
            c.w64(9, 0xfffffffffffc0000u64);
            let r407 = c.r64(9) & c.r64(0);
            c.w64(9, r407);
            c.logic(r407, 64);
            let Some(t408) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d27f); };
            c.logic(t408 & 0x2u64, 8);
            if c.cc_e() { blk = 159; continue; }
            blk = 323; continue;
        }
        323 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d28e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d28f); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083d29c, 0x700083d297);
        }
        324 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d29c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d29e); }
            blk = 159; continue;
        }
        325 => {
            c.w32(8, c.r32(8));
            let r409 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r409);
            c.w64(11, 0xfffffffffffc0000u64);
            let r410 = c.r64(11) & c.r64(8);
            c.w64(11, r410);
            c.logic(r410, 64);
            let Some(t411) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d2b4); };
            c.logic(t411 & 0x2u64, 8);
            if c.cc_e() { blk = 156; continue; }
            blk = 326; continue;
        }
        326 => {
            c.w64(12, c.r64(2).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d2c3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d2c4); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083d2d1, 0x700083d2cc);
        }
        327 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d2d1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d2d3); }
            blk = 156; continue;
        }
        328 => {
            c.w32(8, c.r32(8));
            let r412 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r412);
            c.w64(9, 0xfffffffffffc0000u64);
            let r413 = c.r64(9) & c.r64(8);
            c.w64(9, r413);
            c.logic(r413, 64);
            let Some(t414) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d2e9); };
            c.logic(t414 & 0x2u64, 8);
            if c.cc_e() { blk = 118; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w64(12, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d2f8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d2f9); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x700083d306, 0x700083d301);
        }
        330 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d306); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d308); }
            blk = 118; continue;
        }
        331 => {
            c.w32(11, c.r32(11));
            let r415 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r415);
            c.w64(9, 0xfffffffffffc0000u64);
            let r416 = c.r64(9) & c.r64(11);
            c.w64(9, r416);
            c.logic(r416, 64);
            let Some(t417) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d31e); };
            c.logic(t417 & 0x2u64, 8);
            if c.cc_e() { blk = 109; continue; }
            blk = 332; continue;
        }
        332 => {
            c.w64(12, c.r64(8).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d32d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d32e); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083d33b, 0x700083d336);
        }
        333 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d33b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d33d); }
            blk = 109; continue;
        }
        334 => {
            c.w32(15, c.r32(15));
            let r418 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r418);
            c.w64(0, 0xfffffffffffc0000u64);
            let r419 = c.r64(0) & c.r64(15);
            c.w64(0, r419);
            c.logic(r419, 64);
            let Some(t420) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d353); };
            c.logic(t420 & 0x2u64, 8);
            if c.cc_e() { blk = 86; continue; }
            blk = 335; continue;
        }
        335 => {
            c.w64(6, c.r64(8).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d361); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d362); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x700083d36f, 0x700083d36a);
        }
        336 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d36f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d371); }
            blk = 86; continue;
        }
        337 => {
            c.w32(1, c.r32(1));
            let r421 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r421);
            c.w64(11, 0xfffffffffffc0000u64);
            let r422 = c.r64(11) & c.r64(1);
            c.w64(11, r422);
            c.logic(r422, 64);
            let Some(t423) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d386); };
            c.logic(t423 & 0x2u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 338; continue;
        }
        338 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d395); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d396); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083d3a3, 0x700083d39e);
        }
        339 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d3a3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d3a5); }
            blk = 83; continue;
        }
        340 => {
            c.w32(0, c.r32(0));
            let r424 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r424);
            c.w64(8, 0xfffffffffffc0000u64);
            let r425 = c.r64(8) & c.r64(0);
            c.w64(8, r425);
            c.logic(r425, 64);
            let Some(t426) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d3ba); };
            c.logic(t426 & 0x2u64, 8);
            if c.cc_e() { blk = 59; continue; }
            blk = 341; continue;
        }
        341 => {
            c.w64(9, c.r64(12).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d3ca); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d3cb); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700083d3d8, 0x700083d3d3);
        }
        342 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d3d8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d3da); }
            blk = 59; continue;
        }
        343 => {
            c.w32(0, c.r32(0));
            let r427 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r427);
            c.w64(8, 0xfffffffffffc0000u64);
            let r428 = c.r64(8) & c.r64(0);
            c.w64(8, r428);
            c.logic(r428, 64);
            let Some(t429) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d3ef); };
            c.logic(t429 & 0x2u64, 8);
            if c.cc_e() { blk = 13; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w64(11, c.r64(9).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083d3fe); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d3ff); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083d40c, 0x700083d407);
        }
        345 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083d40c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083d40e); }
            blk = 13; continue;
        }
        346 => {
            return RegfileExit::jump(0x700083d418);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083d434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083d437); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083d43b); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d441); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d447); };
            c.w32(8, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700083d44b); };
            c.cmp(t6, c.r32(8), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083d454); };
            c.w64(2, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(2));
            return RegfileExit::jump(0x700083bef4);
        }
        3 => {
            return RegfileExit::jump(0x700083d464);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083d474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083d477); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700083d47b); };
            c.w64(2, t1);
            c.w64(6, c.r64(2));
            c.w64(3, c.r64(8));
            return RegfileExit::jump(0x700083bef4);
        }
        1 => {
            return RegfileExit::jump(0x700083d48c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083d494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700083d494); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700083d498); }
            let r0 = c.sub(c.r64(4), 0x48u64, 64);
            c.w64(4, r0);
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700083d4a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700083d4ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700083d4b1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700083d4b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x700083d4b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x700083d4bd); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d4c7); };
            c.w32(9, t1);
            let r2 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083d4d0); };
            c.w64(9, t3);
            blk = 3; continue;
        }
        3 => {
            let Some(t4) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083d4d7); };
            c.logic(t4 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083d4e5); };
            c.cmp(t5, c.r32(3), 32);
            if c.cc_e() { blk = 323; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7)));
            c.w32(12, c.r32(9));
            let Some(t6) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083d4f9); };
            c.w32(15, t6);
            let r7 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r7);
            let r8 = c.r32(12) | 0x2u64;
            c.w32(12, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_ne() { blk = 267; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(11)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083d50e); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            blk = 7; continue;
        }
        7 => {
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 185; continue; }
            blk = 8; continue;
        }
        8 => {
            let r11 = c.r32(9) & 0x3u64;
            c.w32(9, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d531); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700083d535); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083d542); };
            c.w32(9, t14);
            let r15 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r15);
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t16) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d551); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t17) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d55b); };
            c.w32(9, t17);
            c.w64(12, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(15, c.r32(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700083d568); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x700083d56c); }
            c.logic(c.r8(12) & 0x30u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 13; continue;
        }
        13 => {
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d584); };
            c.w32(12, t18);
            let r19 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r19);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083d591); };
            c.w64(12, t20);
            blk = 16; continue;
        }
        16 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083d598); };
            c.w32(3, t21);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            let r22 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r22);
            let Some(t23) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d5a7); };
            c.w32(7, t23);
            let r24 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r24);
            let r25 = c.r64(7) & c.r64(3);
            c.w64(7, r25);
            c.logic(r25, 64);
            blk = 18; continue;
        }
        17 => {
            let r26 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r26);
            c.w64(9, c.r64(11).wrapping_add(c.r64(7)));
            c.w64(15, c.r64(3));
            let r27 = c.r64(15) & c.r64(9);
            c.w64(15, r27);
            c.logic(r27, 64);
            c.w64(7, c.r64(15));
            blk = 18; continue;
        }
        18 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(15, c.r64(9).wrapping_mul(4));
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083d5d0); };
            c.w32(2, t28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083d5d5); };
            c.cmp(t29, c.r32(2), 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld32(c.r64(12).wrapping_add(c.r64(15)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083d5e2); };
            c.w32(8, t30);
            c.logic(c.r8(8) & 0x12u64, 8);
            if c.cc_ne() { blk = 283; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t31) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d5f1); };
            c.w32(8, t31);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(11, c.r64(9).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 282; continue; }
            blk = 22; continue;
        }
        22 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700083d611); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r32 = c.r64(9) & c.r64(12);
            c.w64(9, r32);
            c.logic(r32, 64);
            let Some(t33) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d623); };
            c.logic(t33 & 0x4u64, 8);
            if c.cc_ne() { blk = 370; continue; }
            blk = 24; continue;
        }
        24 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700083d631); }
            return c.ret(0x0, 0x700083d632);
        }
        25 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083d633); };
            c.w64(7, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083d637); };
            c.w64(2, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083d63b); };
            c.w64(15, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083d63f); };
            c.w64(9, t37);
            blk = 34; continue;
        }
        26 => {
            let Some(t38) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083d648); };
            c.w32(11, t38);
            let r39 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r39);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 27; continue;
        }
        27 => {
            let r40 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r40);
            c.logic(r40, 64);
            let Some(t41) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d65d); };
            c.w32(12, t41);
            let r42 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r42);
            let Some(t43) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083d664); };
            c.w32(12, t43);
            let r44 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r44);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d671); };
            c.w32(3, t45);
            let r46 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r46);
            let Some(t47) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083d677); };
            c.cmp(t47, 0xbeu64, 16);
            if c.cc_ne() { blk = 284; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t48) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d683); };
            c.w32(3, t48);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xa9u64, 64);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            return RegfileExit::fault(0x700083d696);
        }
        31 => {
            let Some(t49) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d697); };
            c.w32(3, t49);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t50) = c.ld32(c.r64(11).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083d69f); };
            c.w32(11, t50);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xa9u64, 64);
            if c.cc_a() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            return RegfileExit::fault(0x700083d6b2);
        }
        33 => {
            let Some(t51) = c.ld32(c.r64(12).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700083d6b3); };
            c.cmp(t51, c.r32(11), 32);
            if c.cc_ne() { blk = 285; continue; }
            blk = 34; continue;
        }
        34 => {
            let r52 = c.r32(15) & 0x1eu64;
            c.w32(15, r52);
            c.logic(r52, 32);
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_e() { blk = 73; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(15), 0x12u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t53) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083d6d9); };
            c.w32(11, t53);
            let r54 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r54);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 291; continue; }
            blk = 37; continue;
        }
        37 => {
            let r55 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r55);
            c.logic(r55, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700083d6ee); }
            c.cmp(c.r32(15), 0xcu64, 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r32(15), 0x4u64, 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(15), 0x6u64, 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(15), 0x8u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 44; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(15), 0xau64, 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1af4, 0x700083d730, 0x700083d72b);
        }
        43 => {
            let Some(t56) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083d730); };
            c.w64(5, t56);
            c.w64(3, c.r64(0));
            let Some(t57) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083d737); };
            c.w64(0, t57);
            let Some(t58) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x700083d73c); };
            c.w64(1, t58);
            let Some(t59) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x700083d741); };
            c.w64(2, t59);
            let Some(t60) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083d746); };
            c.w64(6, t60);
            let r61 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r61);
            return RegfileExit::jump(0x70008be194);
        }
        44 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083d75a); };
            c.w32(8, t62);
            let r63 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r63);
            blk = 47; continue;
        }
        46 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083d763); };
            c.w32(8, t64);
            let r65 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r65);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let r66 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r66);
            c.logic(r66, 64);
            blk = 50; continue;
        }
        49 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 50; continue;
        }
        50 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083d779); };
            c.w32(9, t67);
            let r68 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r68);
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083d780); };
            c.w32(3, t69);
            let r70 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r70);
            let Some(t71) = c.ld64(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d787); };
            c.w64(9, t71);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t72) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d791); };
            c.w32(11, t72);
            let r73 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r73);
            let Some(t74) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083d798); };
            c.w32(11, t74);
            let r75 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r75);
            blk = 53; continue;
        }
        52 => {
            c.w64(11, c.r64(2));
            blk = 53; continue;
        }
        53 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x700083d7a4); };
            c.w64(12, t76);
            let Some(t77) = c.ld8(c.r64(12)) else { return RegfileExit::fault(0x700083d7ab); };
            c.cmp(t77, 0x20u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083d7b6); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083d7b7); }
            c.w32(1, 0x1u64);
            c.w64(2, c.r64(9));
            c.w64(7, c.r64(11));
            c.w64(6, c.r64(8));
            return c.call(0x70007fe8d4, 0x700083d7cb, 0x700083d7c6);
        }
        55 => {
            blk = 24; continue;
        }
        56 => {
            let r78 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r78);
            if let Some(sr) = c.shr(c.r64(9), 0x6u64, 64) { c.w64(9, sr); }
            let r79 = c.r64(9) & 0x3ffu64;
            c.w64(9, r79);
            c.logic(r79, 64);
            let r80 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(8, r80);
            let Some(t81) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d7e2); };
            c.w32(9, t81);
            let r82 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r82);
            let Some(t83) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083d7e9); };
            c.w32(9, t83);
            let r84 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r84);
            let Some(t85) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083d7f0); };
            c.w32(8, t85);
            let r86 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r86);
            let Some(t87) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083d7f8); };
            c.w64(5, t87);
            let r88 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r88);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700083d800); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083d802); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083d804); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083d805); }
            let Some(t89) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083d806); };
            if !c.push64(t89) { return RegfileExit::fault(0x700083d806); }
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2078)) else { return RegfileExit::fault(0x700083d80a); };
            c.w64(3, t90);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x700083d816); }
            return RegfileExit::jump(0x700086ed74);
        }
        57 => {
            let r91 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r91);
            if let Some(sr) = c.shr(c.r64(9), 0x6u64, 64) { c.w64(9, sr); }
            let r92 = c.r64(9) & 0x3ffu64;
            c.w64(9, r92);
            c.logic(r92, 64);
            let r93 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(8, r93);
            let Some(t94) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d832); };
            c.w32(9, t94);
            let r95 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r95);
            let Some(t96) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083d839); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083d840); };
            c.w32(8, t98);
            let r99 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r99);
            let Some(t100) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083d848); };
            c.w32(7, t100);
            let r101 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r101);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083d84f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083d850); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700083d85b, 0x700083d856);
        }
        58 => {
            blk = 24; continue;
        }
        59 => {
            let Some(t102) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083d860); };
            c.w32(8, t102);
            let Some(t103) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083d864); };
            c.w32(9, t103);
            let r104 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r104);
            c.w32(12, c.r32(8));
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 290; continue; }
            blk = 60; continue;
        }
        60 => {
            let r105 = c.r32(12) & 0x180u64;
            c.w32(12, r105);
            c.logic(r105, 32);
            c.cmp(c.r32(12), 0x100u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 62; continue;
        }
        62 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 66; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 287; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t106) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d8a8); };
            c.w32(8, t106);
            let Some(t107) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d8ac); };
            c.cmp(t107, c.r32(8), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 65; continue;
        }
        65 => {
            blk = 286; continue;
        }
        66 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 288; continue; }
            blk = 67; continue;
        }
        67 => {
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x700083d8c1); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r108 = c.r64(8) & c.r64(11);
            c.w64(8, r108);
            c.logic(r108, 64);
            let Some(t109) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083d8d3); };
            c.logic(t109 & 0x4u64, 8);
            if c.cc_ne() { blk = 367; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 24; continue;
        }
        70 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_ne() { blk = 289; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 24; continue;
        }
        72 => {
            let Some(t110) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083d8f1); };
            c.w64(5, t110);
            let r111 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r111);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083d8f9); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083d8fa); }
            let Some(t112) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083d8fb); };
            if !c.push64(t112) { return RegfileExit::fault(0x700083d8fb); }
            let Some(t113) = c.ld64(c.r64(13).wrapping_add(0x2030)) else { return RegfileExit::fault(0x700083d8ff); };
            c.w64(3, t113);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700083d906); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        73 => {
            let Some(t114) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083d915); };
            c.w32(8, t114);
            let r115 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r115);
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083d91c); };
            c.logic(t116 & 0x100000u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t117) = c.ld32(c.r64(8).wrapping_add(0x23)) else { return RegfileExit::fault(0x700083d926); };
            c.w32(9, t117);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(10, c.r64(4));
            let r118 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r118);
            let r119 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r119);
            c.logic(r119, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700083d93b); }
            c.w64(1, c.r64(8));
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x700083d943); };
            c.w64(0, t120);
            c.w64(10, 0x700083d951u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700083d951); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700083d955); }
            return c.call(c.r64(0), 0x700083d95b, 0x700083d959);
        }
        76 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700083d95b); }
            let Some(t121) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083d963); };
            c.w64(4, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083d968); };
            c.w64(0, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083d96c); };
            c.w64(2, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083d970); };
            c.w64(1, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083d974); };
            c.w64(6, t125);
            blk = 77; continue;
        }
        77 => {
            let Some(t126) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d978); };
            c.w32(8, t126);
            let r127 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r127);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083d985); };
            c.w64(8, t128);
            blk = 79; continue;
        }
        79 => {
            let Some(t129) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083d98c); };
            c.w32(9, t129);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t130) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083d993); };
            c.w32(11, t130);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 292; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t131) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083d9a4); };
            c.w32(12, t131);
            let r132 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r132);
            c.w64(15, c.r64(12));
            let r133 = c.shr(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r133);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let r134 = c.r64(15) & 0xfffffffffffffffeu64;
            c.w64(15, r134);
            c.logic(r134, 64);
            let r135 = c.add(c.r32(15), c.r32(12), 32);
            c.w32(15, r135);
            c.w32(3, c.r32(11));
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_b() { blk = 102; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 293; continue; }
            blk = 82; continue;
        }
        82 => {
            let r136 = c.sub(c.r32(3), c.r32(12), 32);
            c.w32(3, r136);
            let r137 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r137);
            c.w64(11, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083d9da); };
            c.w32(15, t138);
            let r139 = c.r64(11) & 0xfffffffffffffffeu64;
            c.w64(11, r139);
            c.logic(r139, 64);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_b() { blk = 102; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r64(9), 0x3u64, 64);
            if c.cc_be() { blk = 294; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083d9f5); };
            c.w32(9, t140);
            let r141 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r141);
            c.w32(11, c.r64(9).wrapping_add(0x2));
            c.cmp(c.r32(11), 0xfffffeu64, 32);
            if c.cc_a() { blk = 102; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x13), c.r32(11)) { return RegfileExit::fault(0x700083da10); }
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083da14); };
            c.w32(11, t142);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 295; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(11, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x700083da2b); }
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083da2f); };
            c.w32(11, t143);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r144 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r144);
            let Some(t145) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083da3d); };
            c.w32(12, t145);
            let r146 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r146);
            let r147 = c.r64(12) & c.r64(11);
            c.w64(12, r147);
            c.logic(r147, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 88; continue;
        }
        87 => {
            let r148 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r148);
            let r149 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r149);
            c.w64(3, c.r64(11));
            let r150 = c.r64(3) & c.r64(12);
            c.w64(3, r150);
            c.logic(r150, 64);
            c.w64(12, c.r64(3));
            blk = 88; continue;
        }
        88 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            c.w64(6, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083da72); };
            c.w32(7, t151);
            let Some(t152) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083da77); };
            c.cmp(t152, c.r32(7), 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t153) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700083da80); };
            c.cmp(t153, c.r32(7), 32);
            if c.cc_ne() { blk = 87; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t154) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083da89); };
            c.w32(11, t154);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_ae() { blk = 296; continue; }
            blk = 91; continue;
        }
        91 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x700083da9c); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r155 = c.r64(11) & c.r64(8);
            c.w64(11, r155);
            c.logic(r155, 64);
            let Some(t156) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083daaf); };
            c.logic(t156 & 0x4u64, 8);
            if c.cc_ne() { blk = 364; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083daba); };
            c.w32(11, t157);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 297; continue; }
            blk = 94; continue;
        }
        94 => {
            if let Some(sr) = c.shl(c.r64(9), 0x8u64, 64) { c.w64(9, sr); }
            c.w32(11, c.r32(9));
            c.w64(12, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            c.w64(15, c.r64(0));
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(15)) { return RegfileExit::fault(0x700083dae3); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r158 = c.r64(0) & c.r64(8);
            c.w64(0, r158);
            c.logic(r158, 64);
            let Some(t159) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083daf7); };
            c.logic(t159 & 0x4u64, 8);
            if c.cc_ne() { blk = 361; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t160) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083db01); };
            c.w32(12, t160);
            let Some(t161) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700083db05); };
            c.cmp(t161, c.r32(12), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t162) = c.ld8(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083db0e); };
            c.logic(t162 & 0x1u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let r163 = c.r32(11) | 0x20u64;
            c.w32(11, r163);
            c.logic(r163, 32);
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 100; continue;
        }
        99 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 100; continue;
        }
        100 => {
            let Some(t164) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083db20); };
            c.w32(11, t164);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(12, c.r64(3).wrapping_add(0x2));
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_ae() { blk = 298; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(2)).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x700083db37); }
            c.w64(0, c.r64(15));
            blk = 24; continue;
        }
        102 => {
            let Some(t165) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083db44); };
            c.w64(5, t165);
            let r166 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r166);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083db4c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083db4d); }
            let Some(t167) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083db4e); };
            if !c.push64(t167) { return RegfileExit::fault(0x700083db4e); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x700083db52); }
            let Some(t168) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x700083db57); };
            c.w64(3, t168);
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        103 => {
            let Some(t169) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083db68); };
            c.w64(5, t169);
            let r170 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r170);
            let Some(t171) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700083db70); };
            c.logic(t171 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let r172 = c.add(c.r64(8), 0x3fu64, 64);
            c.w64(8, r172);
            blk = 106; continue;
        }
        105 => {
            let Some(t173) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083db80); };
            c.w32(8, t173);
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700083db84); };
            c.w64(8, t174);
            blk = 106; continue;
        }
        106 => {
            return RegfileExit::jump(c.r64(8));
        }
        107 => {
            c.w64(11, c.r64(0));
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 319; continue; }
            blk = 108; continue;
        }
        108 => {
            let r175 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r175);
            c.logic(r175, 64);
            let Some(t176) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083dba0); };
            c.w32(9, t176);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700083dba4); }
            let Some(t177) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x700083dba8); };
            c.cmp(t177, c.r32(9), 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083dbb5); };
            c.w32(9, t178);
            let Some(t179) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083dbb9); };
            c.w32(12, t179);
            let r180 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r180);
            c.w32(15, c.r32(9));
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 110; continue;
        }
        110 => {
            let r181 = c.r32(15) & 0x180u64;
            c.w32(15, r181);
            c.logic(r181, 32);
            c.cmp(c.r32(15), 0x100u64, 32);
            if c.cc_e() { blk = 120; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 112; continue;
        }
        112 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 300; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t182) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083dbff); };
            c.w32(9, t182);
            let Some(t183) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083dc03); };
            c.cmp(t183, c.r32(9), 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 299; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.st32(c.r64(8).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x700083dc19); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r184 = c.r64(9) & c.r64(8);
            c.w64(9, r184);
            c.logic(r184, 64);
            let Some(t185) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083dc2d); };
            c.logic(t185 & 0x4u64, 8);
            if c.cc_ne() { blk = 358; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w64(0, c.r64(11));
            blk = 24; continue;
        }
        120 => {
            c.cmp(c.r32(12), c.r32(11), 32);
            if c.cc_ne() { blk = 302; continue; }
            blk = 121; continue;
        }
        121 => {
            c.w64(0, c.r64(11));
            blk = 24; continue;
        }
        122 => {
            let Some(t186) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700083dc51); };
            c.w32(9, t186);
            let r187 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r187);
            c.w32(12, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083dc60); };
            c.cmp(t188, c.r32(12), 32);
            if c.cc_ne() { blk = 304; continue; }
            blk = 124; continue;
        }
        124 => {
            let Some(t189) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083dc6a); };
            c.w32(9, t189);
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 318; continue; }
            blk = 125; continue;
        }
        125 => {
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r190 = c.r64(9) & 0x3ffu64;
            c.w64(9, r190);
            c.logic(r190, 64);
            c.w64(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t191) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083dc8f); };
            c.w32(12, t191);
            let r192 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r192);
            let Some(t193) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083dc96); };
            c.w32(15, t193);
            c.w32(0, c.r32(15));
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 129; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t194) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083dca4); };
            c.cmp(t194, c.r32(11), 32);
            if c.cc_ne() { blk = 305; continue; }
            blk = 127; continue;
        }
        127 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083dcaf); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r195 = c.r64(9) & c.r64(2);
            c.w64(9, r195);
            c.logic(r195, 64);
            let Some(t196) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083dcbd); };
            c.logic(t196 & 0x4u64, 8);
            if c.cc_ne() { blk = 355; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 184; continue;
        }
        129 => {
            let r197 = c.r32(0) & 0x380u64;
            c.w32(0, r197);
            c.logic(r197, 32);
            c.cmp(c.r32(0), 0x80u64, 32);
            if c.cc_e() { blk = 143; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(0), 0x100u64, 32);
            if c.cc_e() { blk = 140; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(0), 0x180u64, 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 306; continue; }
            blk = 133; continue;
        }
        133 => {
            blk = 144; continue;
        }
        134 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 310; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t198) = c.ld32(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083dd00); };
            c.w32(9, t198);
            let r199 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r199);
            c.w32(12, 0x4u64);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 309; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w32(12, 0x2u64);
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 138; continue;
        }
        138 => {
            let r200 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r200);
            c.logic(r200, 64);
            let Some(t201) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083dd30); };
            c.cmp(t201, c.r32(9), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 139; continue;
        }
        139 => {
            blk = 307; continue;
        }
        140 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t202) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083dd41); };
            c.w32(9, t202);
            let Some(t203) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083dd45); };
            c.cmp(t203, c.r32(9), 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        143 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 311; continue; }
            blk = 144; continue;
        }
        144 => {
            let r204 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r204);
            if let Some(sr) = c.shr(c.r64(15), 0x13u64, 64) { c.w64(15, sr); }
            let Some(t205) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x700083dd67); };
            c.w32(9, t205);
            let Some(t206) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083dd6c); };
            c.w32(12, t206);
            let r207 = c.r64(15) & 0x3ffu64;
            c.w64(15, r207);
            c.logic(r207, 64);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            let r208 = c.add(c.r64(9), c.r64(15), 64);
            c.w64(9, r208);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_b() { blk = 171; continue; }
            blk = 145; continue;
        }
        145 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 152; continue; }
            blk = 146; continue;
        }
        146 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 147; continue;
        }
        147 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700083dd9a); }
            blk = 149; continue;
        }
        148 => {
            c.w64(15, c.r64(11));
            let r209 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r209);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 149; continue;
        }
        149 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083ddb1); };
            c.w64(15, t210);
            let Some(t211) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083ddb8); };
            c.w64(0, t211);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t212) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083ddbf); };
            c.w64(1, t212);
            let Some(t213) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700083ddc6); };
            c.cmp(t213, c.r64(3), 64);
            if c.cc_be() { blk = 312; continue; }
            blk = 150; continue;
        }
        150 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700083ddcf); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 151; continue;
        }
        151 => {
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083ddd6); };
            c.w64(0, t214);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700083dddd); }
            if !c.movsd_mr(c.r64(15).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083dde1); }
            blk = 153; continue;
        }
        152 => {
            c.w64(15, c.r64(11));
            blk = 153; continue;
        }
        153 => {
            let Some(t215) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ddec); };
            c.w32(0, t215);
            let r216 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r216);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(15)) { return RegfileExit::fault(0x700083ddf2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t217) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083ddfa); };
            c.w32(3, t217);
            let r218 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r218);
            c.w64(1, c.r64(3));
            let r219 = c.r32(1) & 0x3ffu64;
            c.w32(1, r219);
            c.logic(r219, 32);
            let r220 = c.r32(3) & 0x3ffffc00u64;
            c.w32(3, r220);
            c.logic(r220, 32);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(10));
            blk = 156; continue;
        }
        155 => {
            let r221 = c.shl(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r221);
            let Some(t222) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083de1f); };
            c.w64(3, t222);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 156; continue;
        }
        156 => {
            let r223 = c.sub(c.r64(9), c.r64(12), 64);
            c.w64(9, r223);
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_b() { blk = 167; continue; }
            blk = 157; continue;
        }
        157 => {
            c.w64(12, c.r64(1).wrapping_add(0x3));
            let Some(t224) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083de38); };
            c.w64(6, t224);
            let Some(t225) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700083de3f); };
            c.w64(7, t225);
            c.w64(11, c.r64(12).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083de4e); };
            c.w64(15, t226);
            let Some(t227) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700083de55); };
            c.cmp(t227, c.r64(8), 64);
            if c.cc_be() { blk = 314; continue; }
            blk = 158; continue;
        }
        158 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x700083de5e); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            blk = 159; continue;
        }
        159 => {
            c.w32(11, c.r32(12));
            c.w32(15, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x700083de6c); };
            c.w64(6, t228);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x700083de73); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700083de77); }
            c.w64(15, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.w64(12, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_e() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t229) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083de90); };
            c.w64(1, t229);
            blk = 161; continue;
        }
        161 => {
            let r230 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r230);
            if !c.st32(c.r64(12).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700083de9b); }
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_ne() { blk = 161; continue; }
            blk = 162; continue;
        }
        162 => {
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_e() { blk = 164; continue; }
            blk = 163; continue;
        }
        163 => {
            let r231 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r231);
            let Some(t232) = c.ld32(c.r64(15).wrapping_add(c.r64(3))) else { return RegfileExit::fault(0x700083deae); };
            c.w32(12, t232);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x700083deb2); }
            c.cmp(c.r64(15), 0x7u64, 64);
            if c.cc_ne() { blk = 163; continue; }
            blk = 164; continue;
        }
        164 => {
            let r233 = c.r32(11) | c.r32(0);
            c.w32(11, r233);
            c.logic(r233, 32);
            let r234 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r234);
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700083dec2); }
            if !c.st32(c.r64(2).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700083dec6); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r235 = c.r64(11) & c.r64(2);
            c.w64(11, r235);
            c.logic(r235, 64);
            let Some(t236) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083deda); };
            c.logic(t236 & 0x4u64, 8);
            if c.cc_ne() { blk = 352; continue; }
            blk = 166; continue;
        }
        166 => {
            c.w64(3, c.r64(8));
            blk = 167; continue;
        }
        167 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083def0); };
            c.w64(0, t237);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700083def4); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r238 = c.r64(9) & c.r64(3);
            c.w64(9, r238);
            c.logic(r238, 64);
            let Some(t239) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083df06); };
            c.logic(t239 & 0x4u64, 8);
            if c.cc_ne() { blk = 349; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083df11); };
            c.w64(8, t240);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083df15); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r241 = c.r64(9) & c.r64(2);
            c.w64(9, r241);
            c.logic(r241, 64);
            let Some(t242) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083df23); };
            c.logic(t242 & 0x4u64, 8);
            if c.cc_ne() { blk = 346; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083df2e); };
            c.w64(11, t243);
            blk = 184; continue;
        }
        171 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 172; continue;
        }
        172 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083df44); }
            c.w64(12, 0xfffffffffffc0000u64);
            let r244 = c.r64(12) & c.r64(2);
            c.w64(12, r244);
            c.logic(r244, 64);
            let Some(t245) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083df52); };
            c.logic(t245 & 0x4u64, 8);
            if c.cc_ne() { blk = 343; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x700083df5e); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r246 = c.r64(8) & c.r64(2);
            c.w64(8, r246);
            c.logic(r246, 64);
            let Some(t247) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083df72); };
            c.logic(t247 & 0x4u64, 8);
            if c.cc_ne() { blk = 340; continue; }
            blk = 175; continue;
        }
        175 => {
            blk = 184; continue;
        }
        176 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700083df88); }
            blk = 179; continue;
        }
        178 => {
            c.w64(12, c.r64(11));
            let r248 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r248);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 179; continue;
        }
        179 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700083df9f); };
            c.w64(12, t249);
            let Some(t250) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700083dfa6); };
            c.w64(15, t250);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700083dfae); };
            c.w64(3, t251);
            let Some(t252) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700083dfb5); };
            c.cmp(t252, c.r64(0), 64);
            if c.cc_be() { blk = 316; continue; }
            blk = 180; continue;
        }
        180 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x700083dfbe); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 181; continue;
        }
        181 => {
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083dfc6); };
            c.w64(15, t253);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700083dfcd); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083dfd2); }
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700083dfd9); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r254 = c.r64(15) & c.r64(2);
            c.w64(15, r254);
            c.logic(r254, 64);
            let Some(t255) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083dfe7); };
            c.logic(t255 & 0x4u64, 8);
            if c.cc_ne() { blk = 337; continue; }
            blk = 182; continue;
        }
        182 => {
            if !c.st32(c.r64(2).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x700083dff2); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r256 = c.r64(8) & c.r64(2);
            c.w64(8, r256);
            c.logic(r256, 64);
            let Some(t257) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e006); };
            c.logic(t257 & 0x4u64, 8);
            if c.cc_ne() { blk = 334; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(0, c.r64(11));
            blk = 24; continue;
        }
        185 => {
            c.w64(11, c.r64(8));
            let r258 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r258);
            let r259 = c.r32(9) & 0x1eu64;
            c.w32(9, r259);
            c.logic(r259, 32);
            c.w64(12, c.r64(11));
            let r260 = c.r32(12) & 0xfu64;
            c.w32(12, r260);
            c.logic(r260, 32);
            c.cmp(c.r32(12), 0x6u64, 32);
            if c.cc_l() { blk = 204; continue; }
            blk = 186; continue;
        }
        186 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 200; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t261) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e052); };
            c.w32(8, t261);
            let r262 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r262);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700083e05f); };
            c.w64(8, t263);
            blk = 191; continue;
        }
        191 => {
            let Some(t264) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083e066); };
            c.w32(9, t264);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r265 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r265);
            let Some(t266) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e074); };
            c.w32(11, t266);
            let r267 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r267);
            let r268 = c.r64(11) & c.r64(9);
            c.w64(11, r268);
            c.logic(r268, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 193; continue;
        }
        192 => {
            let r269 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r269);
            let r270 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r270);
            c.w64(15, c.r64(9));
            let r271 = c.r64(15) & c.r64(11);
            c.w64(15, r271);
            c.logic(r271, 64);
            c.w64(11, c.r64(15));
            blk = 193; continue;
        }
        193 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(15).wrapping_mul(4));
            let Some(t272) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083e0a1); };
            c.w32(7, t272);
            let Some(t273) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083e0a6); };
            c.cmp(t273, c.r32(7), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r32(7), c.r32(1), 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t274) = c.ld32(c.r64(8).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700083e0b7); };
            c.w32(9, t274);
            c.logic(c.r8(9) & 0x12u64, 8);
            if c.cc_ne() { blk = 321; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t275) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e0c6); };
            c.w32(9, t275);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 320; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w64(9, c.r64(15).wrapping_mul(4).wrapping_add(0xb));
            c.w64(11, c.r64(0));
            if !c.st32(c.r64(8).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x700083e0e8); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r276 = c.r64(12) & c.r64(8);
            c.w64(12, r276);
            c.logic(r276, 64);
            let Some(t277) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e0fc); };
            c.logic(t277 & 0x4u64, 8);
            if c.cc_ne() { blk = 331; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(0, c.r64(11));
            blk = 24; continue;
        }
        200 => {
            let Some(t278) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083e110); };
            c.w64(5, t278);
            let r279 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r279);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083e118); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083e119); }
            let Some(t280) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083e11a); };
            if !c.push64(t280) { return RegfileExit::fault(0x700083e11a); }
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0x2030)) else { return RegfileExit::fault(0x700083e11e); };
            c.w64(3, t281);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700083e125); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        201 => {
            let Some(t282) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083e134); };
            c.w64(5, t282);
            let r283 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r283);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083e13c); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083e13d); }
            let Some(t284) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700083e13e); };
            if !c.push64(t284) { return RegfileExit::fault(0x700083e13e); }
            let Some(t285) = c.ld64(c.r64(13).wrapping_add(0x20a8)) else { return RegfileExit::fault(0x700083e142); };
            c.w64(3, t285);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x700083e149); }
            c.w32(0, 0x3u64);
            return RegfileExit::jump(0x700086ed74);
        }
        202 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1af4, 0x700083e160, 0x700083e15b);
        }
        203 => {
            let Some(t286) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083e160); };
            c.w64(5, t286);
            c.w64(3, c.r64(0));
            let Some(t287) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x700083e167); };
            c.w64(0, t287);
            let Some(t288) = c.ld64(c.r64(4).wrapping_add(0x40)) else { return RegfileExit::fault(0x700083e16c); };
            c.w64(1, t288);
            let Some(t289) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700083e171); };
            c.w64(6, t289);
            c.w64(2, c.r64(0));
            let r290 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r290);
            return RegfileExit::jump(0x70008be194);
        }
        204 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 264; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(12, c.r64(11));
            let r291 = c.shr(c.r32(12), 0x11u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r291);
            let r292 = c.r32(12) & 0x7u64;
            c.w32(12, r292);
            c.logic(r292, 32);
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 239; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 209; continue;
        }
        209 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 212; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t293) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e1c3); };
            c.w32(12, t293);
            let Some(t294) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700083e1c7); };
            c.cmp(t294, c.r32(12), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 211; continue;
        }
        211 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700083e1d4); }
            blk = 213; continue;
        }
        212 => {
            c.w64(12, c.r64(0));
            let r295 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r295);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 213; continue;
        }
        213 => {
            c.w64(12, c.r64(11));
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r296 = c.r64(12) & 0x3ffu64;
            c.w64(12, r296);
            c.logic(r296, 64);
            let r297 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r297);
            let Some(t298) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e1fc); };
            c.w32(15, t298);
            let r299 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r299);
            let Some(t300) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083e203); };
            c.w32(15, t300);
            let r301 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r301);
            let Some(t302) = c.ld32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083e20a); };
            c.w32(12, t302);
            c.w32(12, c.r32(12));
            let r303 = c.r32(12) & 0x380u64;
            c.w32(12, r303);
            c.logic(r303, 32);
            c.cmp(c.r32(12), 0x100u64, 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 214; continue;
        }
        214 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(9), 0x2u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            let r304 = c.r64(11) & 0x7ffu64;
            c.w64(11, r304);
            c.logic(r304, 64);
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 217; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t305) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e246); };
            c.w32(8, t305);
            let r306 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r306);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083e253); };
            c.w64(8, t307);
            blk = 218; continue;
        }
        217 => {
            c.w64(8, c.r64(2));
            blk = 218; continue;
        }
        218 => {
            let Some(t308) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e25f); };
            c.w32(8, t308);
            let r309 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r309);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700083e26d); }
            blk = 24; continue;
        }
        220 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700083e278); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 222; continue; }
            blk = 221; continue;
        }
        221 => {
            if c.cc_e() { blk = 227; continue; }
            blk = 222; continue;
        }
        222 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            if c.cc_e() { blk = 266; continue; }
            blk = 224; continue;
        }
        224 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 71; continue; }
            blk = 225; continue;
        }
        225 => {
            if c.cc_e() { blk = 266; continue; }
            blk = 226; continue;
        }
        226 => {
            blk = 71; continue;
        }
        227 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 228; continue;
        }
        228 => {
            blk = 266; continue;
        }
        229 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            let r310 = c.r64(11) & 0x7ffu64;
            c.w64(11, r310);
            c.logic(r310, 64);
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t311) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e2db); };
            c.w32(8, t311);
            let r312 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r312);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 233; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083e2e8); };
            c.w64(8, t313);
            blk = 233; continue;
        }
        232 => {
            c.w64(8, c.r64(2));
            blk = 233; continue;
        }
        233 => {
            c.w64(12, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 234; continue;
        }
        234 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x700083e302); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r314 = c.r64(9) & c.r64(8);
            c.w64(9, r314);
            c.logic(r314, 64);
            let Some(t315) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e314); };
            c.logic(t315 & 0x4u64, 8);
            if c.cc_ne() { blk = 328; continue; }
            blk = 236; continue;
        }
        236 => {
            blk = 24; continue;
        }
        237 => {
            let Some(t316) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e324); };
            c.cmp(t316, c.r32(0), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 238; continue;
        }
        238 => {
            blk = 266; continue;
        }
        239 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 240; continue;
        }
        240 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 244; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w64(12, c.r64(11));
            if let Some(sr) = c.shr(c.r64(12), 0x6u64, 64) { c.w64(12, sr); }
            let r317 = c.r64(12) & 0x3ffu64;
            c.w64(12, r317);
            c.logic(r317, 64);
            let r318 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r318);
            let Some(t319) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e360); };
            c.w32(15, t319);
            let r320 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r320);
            let Some(t321) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083e367); };
            c.w32(15, t321);
            let r322 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r322);
            let Some(t323) = c.ld32(c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083e36e); };
            c.w32(12, t323);
            let r324 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r324);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 244; continue; }
            blk = 242; continue;
        }
        242 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 266; continue; }
            blk = 243; continue;
        }
        243 => {
            let r325 = c.r64(12) & 0xfffffffffffffffdu64;
            c.w64(12, r325);
            c.logic(r325, 64);
            let Some(t326) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e38a); };
            c.cmp(t326, c.r32(12), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 244; continue;
        }
        244 => {
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 247; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t327) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e39d); };
            c.w32(8, t327);
            let r328 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r328);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 248; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t329) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083e3aa); };
            c.w64(8, t329);
            blk = 248; continue;
        }
        247 => {
            c.w64(8, c.r64(2));
            blk = 248; continue;
        }
        248 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            let r330 = c.r64(11) & 0x7ffu64;
            c.w64(11, r330);
            c.logic(r330, 64);
            c.w64(12, c.r64(11).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 252; continue; }
            blk = 249; continue;
        }
        249 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(12)), c.r32(0)) { return RegfileExit::fault(0x700083e3cf); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r331 = c.r64(9) & c.r64(8);
            c.w64(9, r331);
            c.logic(r331, 64);
            let Some(t332) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e3e1); };
            c.logic(t332 & 0x4u64, 8);
            if c.cc_ne() { blk = 325; continue; }
            blk = 251; continue;
        }
        251 => {
            blk = 24; continue;
        }
        252 => {
            let Some(t333) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e3f1); };
            c.cmp(t333, c.r32(0), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 253; continue;
        }
        253 => {
            blk = 266; continue;
        }
        254 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 266; continue; }
            blk = 255; continue;
        }
        255 => {
            c.logic(c.r32(8) & 0x20000u64, 32);
            if c.cc_ne() { blk = 258; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t334) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e412); };
            c.w32(8, t334);
            let r335 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r335);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 259; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t336) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700083e41f); };
            c.w64(8, t336);
            blk = 259; continue;
        }
        258 => {
            c.w64(8, c.r64(2));
            blk = 259; continue;
        }
        259 => {
            if let Some(sr) = c.shr(c.r64(11), 0x14u64, 64) { c.w64(11, sr); }
            let r337 = c.r64(11) & 0x7ffu64;
            c.w64(11, r337);
            c.logic(r337, 64);
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 261; continue; }
            blk = 260; continue;
        }
        260 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700083e43c); }
            blk = 24; continue;
        }
        261 => {
            let Some(t338) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e446); };
            c.cmp(t338, c.r32(0), 32);
            if c.cc_e() { blk = 71; continue; }
            blk = 262; continue;
        }
        262 => {
            blk = 266; continue;
        }
        263 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r339 = c.r64(11) & 0x3ffu64;
            c.w64(11, r339);
            c.logic(r339, 64);
            let r340 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r340);
            let Some(t341) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e465); };
            c.w32(9, t341);
            let r342 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r342);
            let Some(t343) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083e46c); };
            c.w32(9, t343);
            let r344 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r344);
            let Some(t345) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083e473); };
            c.w32(8, t345);
            let r346 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r346);
            let Some(t347) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083e47b); };
            c.w64(5, t347);
            let r348 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r348);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083e483); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e484); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083e486); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083e487); }
            let Some(t349) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083e488); };
            if !c.push64(t349) { return RegfileExit::fault(0x700083e488); }
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0x2078)) else { return RegfileExit::fault(0x700083e48c); };
            c.w64(3, t350);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x700083e498); }
            return RegfileExit::jump(0x700086ed74);
        }
        264 => {
            if let Some(sr) = c.shr(c.r64(11), 0x6u64, 64) { c.w64(11, sr); }
            let r351 = c.r64(11) & 0x3ffu64;
            c.w64(11, r351);
            c.logic(r351, 64);
            let r352 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(8, r352);
            let Some(t353) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e4b1); };
            c.w32(9, t353);
            let r354 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r354);
            let Some(t355) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083e4b8); };
            c.w32(9, t355);
            let r356 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r356);
            let Some(t357) = c.ld32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700083e4bf); };
            c.w32(8, t357);
            let r358 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r358);
            let Some(t359) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083e4c7); };
            c.w32(7, t359);
            let r360 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r360);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700083e4ce); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083e4cf); }
            c.w32(0, 0x1u64);
            return c.call(0x70007f2274, 0x700083e4da, 0x700083e4d5);
        }
        265 => {
            blk = 24; continue;
        }
        266 => {
            let Some(t361) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083e4df); };
            c.w64(5, t361);
            let r362 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r362);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e4e7); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700083e4e8); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700083e4e9); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700083e4ea); }
            let Some(t363) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700083e4eb); };
            if !c.push64(t363) { return RegfileExit::fault(0x700083e4eb); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700083e4ef); }
            c.w32(0, 0x5u64);
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0x2098)) else { return RegfileExit::fault(0x700083e4f9); };
            c.w64(3, t364);
            return RegfileExit::jump(0x700086ed74);
        }
        267 => {
            c.w32(11, c.r32(15));
            let r365 = c.r32(11) & 0x3u64;
            c.w32(11, r365);
            c.logic(r365, 32);
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t366) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e512); };
            c.w32(11, t366);
            let Some(t367) = c.ld32(c.r64(13).wrapping_add(0x2c8)) else { return RegfileExit::fault(0x700083e516); };
            c.cmp(t367, c.r32(11), 32);
            if c.cc_e() { blk = 277; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xc08)) else { return RegfileExit::fault(0x700083e523); };
            c.cmp(t368, c.r32(15), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 270; continue;
        }
        270 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t369) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083e532); };
            c.w32(8, t369);
            let r370 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r370);
            c.w64(9, c.r64(8));
            if let Some(sr) = c.shr(c.r64(9), 0xdu64, 64) { c.w64(9, sr); }
            let Some(t371) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e540); };
            c.w32(11, t371);
            let r372 = c.r64(9) ^ c.r64(8);
            c.w64(9, r372);
            c.logic(r372, 64);
            let r373 = c.add(c.r32(9), c.r32(11), 32);
            c.w32(9, r373);
            let r374 = c.r32(9) & 0x1ffcu64;
            c.w32(9, r374);
            c.logic(r374, 32);
            let r375 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(11, r375);
            let Some(t376) = c.ld64(c.r64(13).wrapping_add(0x2f48)) else { return RegfileExit::fault(0x700083e555); };
            c.w64(12, t376);
            let Some(t377) = c.ld32(c.r64(12).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x700083e55c); };
            c.cmp(t377, c.r32(1), 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t378) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e562); };
            c.cmp(t378, c.r32(8), 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t379) = c.ld32(c.r64(12).wrapping_add(c.r64(11)).wrapping_add(0x4)) else { return RegfileExit::fault(0x700083e569); };
            c.w32(8, t379);
            let r380 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r380);
            blk = 7; continue;
        }
        274 => {
            c.w32(11, c.r32(1));
            c.w32(9, c.r32(9));
            let r381 = c.sub(c.r32(9), c.r32(11), 32);
            c.w32(9, r381);
            let r382 = c.sub(c.r32(9), 0x4e93591bu64, 32);
            c.w32(9, r382);
            let r383 = c.r32(9) & 0x7fcu64;
            c.w32(9, r383);
            c.logic(r383, 32);
            let r384 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(9, r384);
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x2f60)) else { return RegfileExit::fault(0x700083e591); };
            c.w64(11, t385);
            let Some(t386) = c.ld32(c.r64(11).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700083e598); };
            c.cmp(t386, c.r32(1), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t387) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e5a2); };
            c.cmp(t387, c.r32(8), 32);
            if c.cc_ne() { blk = 266; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t388) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x4)) else { return RegfileExit::fault(0x700083e5ad); };
            c.w32(8, t388);
            let r389 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r389);
            blk = 7; continue;
        }
        277 => {
            let Some(t390) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083e5ba); };
            c.w32(8, t390);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r391 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r391);
            blk = 278; continue;
        }
        278 => {
            c.w64(11, c.r64(8).wrapping_mul(4));
            c.w32(12, c.r32(9));
            let Some(t392) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700083e5d3); };
            c.w32(7, t392);
            let r393 = c.r32(12) | 0x2u64;
            c.w32(12, r393);
            c.logic(r393, 32);
            c.cmp(c.r32(12), c.r32(7), 32);
            if c.cc_e() { blk = 281; continue; }
            blk = 279; continue;
        }
        279 => {
            let r394 = c.sub(c.r64(8), 0x2u64, 64);
            c.w64(8, r394);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ge() { blk = 278; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e5eb); };
            c.w64(7, t395);
            blk = 266; continue;
        }
        281 => {
            let Some(t396) = c.ld32(c.r64(15).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700083e5f4); };
            c.w32(8, t396);
            let r397 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r397);
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e5fc); };
            c.w64(7, t398);
            blk = 7; continue;
        }
        282 => {
            return RegfileExit::fault(0x700083e605);
        }
        283 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e606); };
            c.w64(7, t399);
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e60a); };
            c.w64(3, t400);
            let Some(t401) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083e60e); };
            c.w64(2, t401);
            blk = 266; continue;
        }
        284 => {
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e617); };
            c.w64(3, t402);
            blk = 266; continue;
        }
        285 => {
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e620); };
            c.w64(3, t403);
            blk = 266; continue;
        }
        286 => {
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e629); };
            c.w64(3, t404);
            blk = 266; continue;
        }
        287 => {
            let Some(t405) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e632); };
            c.w64(3, t405);
            blk = 266; continue;
        }
        288 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e63b); };
            c.w64(3, t406);
            blk = 266; continue;
        }
        289 => {
            let Some(t407) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e644); };
            c.w64(3, t407);
            blk = 266; continue;
        }
        290 => {
            let Some(t408) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e64d); };
            c.w64(3, t408);
            blk = 266; continue;
        }
        291 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e656); };
            c.w64(3, t409);
            blk = 266; continue;
        }
        292 => {
            return RegfileExit::fault(0x700083e65f);
        }
        293 => {
            return RegfileExit::fault(0x700083e660);
        }
        294 => {
            return RegfileExit::fault(0x700083e661);
        }
        295 => {
            return RegfileExit::fault(0x700083e662);
        }
        296 => {
            return RegfileExit::fault(0x700083e663);
        }
        297 => {
            return RegfileExit::fault(0x700083e664);
        }
        298 => {
            return RegfileExit::fault(0x700083e665);
        }
        299 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        300 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        301 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        302 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        303 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        304 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        305 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        306 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        307 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        308 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        309 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        310 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        311 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        312 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083e6d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700083e6d7); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700083e6db); }
            c.w64(15, c.r64(2));
            return c.call(0x70007febf4, 0x700083e6e8, 0x700083e6e3);
        }
        313 => {
            c.w64(15, c.r64(0));
            let Some(t410) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083e6eb); };
            c.w64(11, t410);
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083e6ef); };
            c.w64(2, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083e6f3); };
            c.w64(8, t412);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e6f7); };
            c.w64(9, t413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083e6fb); };
            c.w64(12, t414);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700083e6ff); }
            blk = 151; continue;
        }
        314 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083e709); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700083e70d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x700083e711); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x700083e715); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x700083e719); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700083e725, 0x700083e720);
        }
        315 => {
            c.w64(8, c.r64(0));
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083e728); };
            c.w64(2, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e72c); };
            c.w64(9, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700083e730); };
            c.w64(3, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700083e734); };
            c.w64(0, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700083e738); };
            c.w64(12, t419);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700083e73c); };
            c.w64(1, t420);
            blk = 159; continue;
        }
        316 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x700083e74a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700083e74e); }
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x700083e75b, 0x700083e756);
        }
        317 => {
            c.w64(12, c.r64(0));
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083e75e); };
            c.w64(11, t421);
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083e762); };
            c.w64(2, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e766); };
            c.w64(9, t423);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700083e76a); };
            c.w64(8, t424);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700083e76e); }
            blk = 181; continue;
        }
        318 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        319 => {
            c.w64(0, c.r64(11));
            blk = 266; continue;
        }
        320 => {
            return RegfileExit::fault(0x700083e788);
        }
        321 => {
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e789); };
            c.w64(7, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e78d); };
            c.w64(3, t426);
            blk = 266; continue;
        }
        322 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700083e796); };
            c.w64(7, t427);
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700083e79a); };
            c.w64(3, t428);
            blk = 266; continue;
        }
        323 => {
            let Some(t429) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700083e7a3); };
            c.w64(5, t429);
            let r430 = c.add(c.r64(4), 0x58u64, 64);
            c.w64(4, r430);
            return RegfileExit::jump(0x7000805d94);
        }
        324 => {
            blk = 266; continue;
        }
        325 => {
            c.w32(0, c.r32(0));
            let r431 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r431);
            c.w64(9, 0xfffffffffffc0000u64);
            let r432 = c.r64(9) & c.r64(0);
            c.w64(9, r432);
            c.logic(r432, 64);
            let Some(t433) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e7c4); };
            c.logic(t433 & 0x2u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 326; continue;
        }
        326 => {
            c.w64(11, c.r64(8).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e7d3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e7d4); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083e7e1, 0x700083e7dc);
        }
        327 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e7e1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e7e3); }
            blk = 251; continue;
        }
        328 => {
            c.w32(0, c.r32(0));
            let r434 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r434);
            c.w64(9, 0xfffffffffffc0000u64);
            let r435 = c.r64(9) & c.r64(0);
            c.w64(9, r435);
            c.logic(r435, 64);
            let Some(t436) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e7f8); };
            c.logic(t436 & 0x2u64, 8);
            if c.cc_e() { blk = 236; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w64(11, c.r64(8).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e807); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e808); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083e815, 0x700083e810);
        }
        330 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e815); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e817); }
            blk = 236; continue;
        }
        331 => {
            c.w32(11, c.r32(11));
            let r437 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r437);
            c.w64(12, 0xfffffffffffc0000u64);
            let r438 = c.r64(12) & c.r64(11);
            c.w64(12, r438);
            c.logic(r438, 64);
            let Some(t439) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e82d); };
            c.logic(t439 & 0x2u64, 8);
            if c.cc_e() { blk = 199; continue; }
            blk = 332; continue;
        }
        332 => {
            c.w64(15, c.r64(8).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e83d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e83e); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700083e84b, 0x700083e846);
        }
        333 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e84b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e84d); }
            blk = 199; continue;
        }
        334 => {
            c.w32(12, c.r32(12));
            let r440 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r440);
            c.w64(8, 0xfffffffffffc0000u64);
            let r441 = c.r64(8) & c.r64(12);
            c.w64(8, r441);
            c.logic(r441, 64);
            let Some(t442) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e863); };
            c.logic(t442 & 0x2u64, 8);
            if c.cc_e() { blk = 184; continue; }
            blk = 335; continue;
        }
        335 => {
            c.w64(15, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e872); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e873); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700083e880, 0x700083e87b);
        }
        336 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e880); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e882); }
            blk = 184; continue;
        }
        337 => {
            c.w32(8, c.r32(8));
            let r443 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r443);
            c.w64(15, 0xfffffffffffc0000u64);
            let r444 = c.r64(15) & c.r64(8);
            c.w64(15, r444);
            c.logic(r444, 64);
            let Some(t445) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e898); };
            c.logic(t445 & 0x2u64, 8);
            if c.cc_e() { blk = 182; continue; }
            blk = 338; continue;
        }
        338 => {
            c.w64(0, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e8a7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e8a8); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x700083e8b5, 0x700083e8b0);
        }
        339 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e8b5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e8b7); }
            blk = 182; continue;
        }
        340 => {
            c.w32(11, c.r32(11));
            let r446 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r446);
            c.w64(8, 0xfffffffffffc0000u64);
            let r447 = c.r64(8) & c.r64(11);
            c.w64(8, r447);
            c.logic(r447, 64);
            let Some(t448) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e8cd); };
            c.logic(t448 & 0x2u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 341; continue;
        }
        341 => {
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e8dc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e8dd); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083e8ea, 0x700083e8e5);
        }
        342 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e8ea); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e8ec); }
            blk = 175; continue;
        }
        343 => {
            c.w32(8, c.r32(8));
            let r449 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r449);
            c.w64(12, 0xfffffffffffc0000u64);
            let r450 = c.r64(12) & c.r64(8);
            c.w64(12, r450);
            c.logic(r450, 64);
            let Some(t451) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e902); };
            c.logic(t451 & 0x2u64, 8);
            if c.cc_e() { blk = 173; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w64(15, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e912); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e913); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(15));
            return c.call(0x70007f13b4, 0x700083e920, 0x700083e91b);
        }
        345 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e920); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e922); }
            blk = 173; continue;
        }
        346 => {
            c.w32(8, c.r32(8));
            let r452 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r452);
            c.w64(9, 0xfffffffffffc0000u64);
            let r453 = c.r64(9) & c.r64(8);
            c.w64(9, r453);
            c.logic(r453, 64);
            let Some(t454) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e938); };
            c.logic(t454 & 0x2u64, 8);
            if c.cc_e() { blk = 170; continue; }
            blk = 347; continue;
        }
        347 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e947); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e948); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x700083e955, 0x700083e950);
        }
        348 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e955); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e957); }
            blk = 170; continue;
        }
        349 => {
            c.w32(0, c.r32(0));
            let r455 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r455);
            c.w64(9, 0xfffffffffffc0000u64);
            let r456 = c.r64(9) & c.r64(0);
            c.w64(9, r456);
            c.logic(r456, 64);
            let Some(t457) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e96c); };
            c.logic(t457 & 0x2u64, 8);
            if c.cc_e() { blk = 169; continue; }
            blk = 350; continue;
        }
        350 => {
            c.w64(11, c.r64(3).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e97b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e97c); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083e989, 0x700083e984);
        }
        351 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e989); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e98b); }
            blk = 169; continue;
        }
        352 => {
            c.w32(8, c.r32(8));
            let r458 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r458);
            c.w64(11, 0xfffffffffffc0000u64);
            let r459 = c.r64(11) & c.r64(8);
            c.w64(11, r459);
            c.logic(r459, 64);
            let Some(t460) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e9a1); };
            c.logic(t460 & 0x2u64, 8);
            if c.cc_e() { blk = 166; continue; }
            blk = 353; continue;
        }
        353 => {
            c.w64(12, c.r64(2).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e9b0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e9b1); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083e9be, 0x700083e9b9);
        }
        354 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e9be); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e9c0); }
            blk = 166; continue;
        }
        355 => {
            c.w32(8, c.r32(8));
            let r461 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r461);
            c.w64(9, 0xfffffffffffc0000u64);
            let r462 = c.r64(9) & c.r64(8);
            c.w64(9, r462);
            c.logic(r462, 64);
            let Some(t463) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083e9d6); };
            c.logic(t463 & 0x2u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 356; continue;
        }
        356 => {
            c.w64(12, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083e9e5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083e9e6); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(12));
            return c.call(0x70007f13b4, 0x700083e9f3, 0x700083e9ee);
        }
        357 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083e9f3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083e9f5); }
            blk = 128; continue;
        }
        358 => {
            c.w32(11, c.r32(11));
            let r464 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r464);
            c.w64(9, 0xfffffffffffc0000u64);
            let r465 = c.r64(9) & c.r64(11);
            c.w64(9, r465);
            c.logic(r465, 64);
            let Some(t466) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083ea0b); };
            c.logic(t466 & 0x2u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(12, c.r64(8).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083ea1a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083ea1b); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083ea28, 0x700083ea23);
        }
        360 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083ea28); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083ea2a); }
            blk = 119; continue;
        }
        361 => {
            c.w32(15, c.r32(15));
            let r467 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r467);
            c.w64(0, 0xfffffffffffc0000u64);
            let r468 = c.r64(0) & c.r64(15);
            c.w64(0, r468);
            c.logic(r468, 64);
            let Some(t469) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083ea40); };
            c.logic(t469 & 0x2u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 362; continue;
        }
        362 => {
            c.w64(6, c.r64(8).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083ea4e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083ea4f); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x700083ea5c, 0x700083ea57);
        }
        363 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083ea5c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083ea5e); }
            blk = 96; continue;
        }
        364 => {
            c.w32(1, c.r32(1));
            let r470 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r470);
            c.w64(11, 0xfffffffffffc0000u64);
            let r471 = c.r64(11) & c.r64(1);
            c.w64(11, r471);
            c.logic(r471, 64);
            let Some(t472) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083ea73); };
            c.logic(t472 & 0x2u64, 8);
            if c.cc_e() { blk = 93; continue; }
            blk = 365; continue;
        }
        365 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083ea82); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083ea83); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700083ea90, 0x700083ea8b);
        }
        366 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083ea90); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083ea92); }
            blk = 93; continue;
        }
        367 => {
            c.w32(0, c.r32(0));
            let r473 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r473);
            c.w64(8, 0xfffffffffffc0000u64);
            let r474 = c.r64(8) & c.r64(0);
            c.w64(8, r474);
            c.logic(r474, 64);
            let Some(t475) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083eaa7); };
            c.logic(t475 & 0x2u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 368; continue;
        }
        368 => {
            c.w64(9, c.r64(11).wrapping_add(0xb));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083eab6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083eab7); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700083eac4, 0x700083eabf);
        }
        369 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083eac4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083eac6); }
            blk = 69; continue;
        }
        370 => {
            c.w32(0, c.r32(0));
            let r476 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r476);
            c.w64(9, 0xfffffffffffc0000u64);
            let r477 = c.r64(9) & c.r64(0);
            c.w64(9, r477);
            c.logic(r477, 64);
            let Some(t478) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700083eadb); };
            c.logic(t478 & 0x2u64, 8);
            if c.cc_e() { blk = 24; continue; }
            blk = 371; continue;
        }
        371 => {
            c.w64(11, c.r64(12).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700083eaea); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700083eaeb); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700083eaf8, 0x700083eaf3);
        }
        372 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700083eaf8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700083eafa); }
            blk = 24; continue;
        }
        373 => {
            return RegfileExit::jump(0x700083eb04);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083eb14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700083eb17); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700083eb1b); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700083eb22); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700083eb29); };
            c.w32(9, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700083eb2d); };
            c.cmp(t6, c.r32(9), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700083eb36); };
            c.w64(8, t7);
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(8));
            return RegfileExit::jump(0x700083d494);
        }
        3 => {
            return RegfileExit::jump(0x700083eb48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700083eb54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700083eb57); };
            c.w64(9, t0);
            let Some(t1) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x700083eb5b); };
            c.w64(8, t1);
            c.w64(6, c.r64(8));
            c.w64(3, c.r64(9));
            return RegfileExit::jump(0x700083d494);
        }
        1 => {
            return RegfileExit::jump(0x700083eb6c);
        }
            _ => unreachable!(),
        }
    }
}
