#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700088f014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088f014); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700088f018); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088f01e); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(6)) { return RegfileExit::fault(0x700088f02e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x700088f032); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(3)) { return RegfileExit::fault(0x700088f036); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700088f03a); }
            blk = 2; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x700088f040); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088f044); };
            c.w64(2, t3);
            blk = 2; continue;
        }
        2 => {
            c.w32(15, c.r32(7));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x700088f04b); }
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700088f05b); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x700088f05f); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088f069); }
            let Some(t4) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088f06e); };
            c.cmp(t4, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088f074); }
            blk = 8; continue;
        }
        5 => {
            c.w64(3, c.r64(8));
            let r5 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r5);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700088f08b); };
            c.cmp(t6, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(3, c.r64(7));
            let r7 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r7);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700088f096); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 253; continue;
        }
        10 => {
            c.cmp(c.r32(7), c.r32(8), 32);
            if c.cc_ge() { blk = 253; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700088f0b8); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 279; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f0c5); };
            c.w32(3, t8);
            let r9 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r9);
            let Some(t10) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f0cb); };
            c.w32(8, t10);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 279; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 273; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f0f2); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            let Some(t13) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f0f9); };
            c.w32(9, t13);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088f105); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(2) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(1, 0x1fffffffffffffu64);
            let r14 = c.add(c.r64(1), c.r64(2), 64);
            c.w64(1, r14);
            c.w64(6, 0x3ffffffffffffeu64);
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_be() { blk = 107; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088f140); };
            c.w64(6, t15);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 259; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f156); };
            c.w32(2, t16);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 279; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 279; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(9, c.r64(7));
            blk = 30; continue;
        }
        27 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f187); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            blk = 30; continue;
        }
        28 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r19 = c.r64(2) & 0xffffffu64;
            c.w64(2, r19);
            c.logic(r19, 64);
            c.w64(9, c.r64(2));
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088f19e); };
            c.w64(2, t20);
            blk = 109; continue;
        }
        29 => {
            c.w64(9, c.r64(7));
            blk = 30; continue;
        }
        30 => {
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700088f1aa); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088f1b1); };
            c.w64(2, t22);
            blk = 32; continue;
        }
        31 => {
            let Some(t23) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f1b7); };
            c.w32(3, t23);
            let r24 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r24);
            let Some(t25) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f1bd); };
            c.w32(8, t25);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 61; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f1cf); };
            c.w32(6, t26);
            c.logic(c.r32(6) & 0x200000u64, 32);
            if c.cc_ne() { blk = 54; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t27) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x700088f1de); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            c.logic(c.r32(6) & 0xffc00u64, 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 35; continue;
        }
        35 => {
            let r29 = c.shr(c.r32(6), 0xau64, 32).unwrap_or(c.r32(6));
            c.w32(6, r29);
            let r30 = c.r32(6) & 0x3ffu64;
            c.w32(6, r30);
            c.logic(r30, 32);
            c.cmp(c.r32(6), 0x20u64, 32);
            if c.cc_be() { blk = 50; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t31) = c.ld16(c.r64(2).wrapping_add(0x5)) else { return RegfileExit::fault(0x700088f202); };
            c.w32(15, (t31 as u16 as i16 as i64 as u64));
            let Some(t32) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f207); };
            c.w32(7, t32);
            let r33 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r33);
            let r34 = c.sub(c.r32(15), 0x1u64, 32);
            c.w32(15, r34);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700088f212); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700088f21a); }
            blk = 38; continue;
        }
        38 => {
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088f21e); };
            c.w64(12, t35);
            let r36 = c.sub(c.r32(12), c.r32(0), 32);
            c.w32(12, r36);
            let r37 = c.shr(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r37);
            let r38 = c.add(c.r32(12), c.r32(0), 32);
            c.w32(12, r38);
            c.w32(11, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088f232); };
            c.w32(11, t39);
            let r40 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r40);
            let r41 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r41);
            let r42 = c.r32(11) & 0x3ffu64;
            c.w32(11, r42);
            c.logic(r42, 32);
            c.w32(11, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f24c); };
            c.w32(11, t43);
            let r44 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r44);
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f254); };
            c.w32(11, t45);
            let r46 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r46);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_be() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(0, c.r64(12).wrapping_add(0x1));
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088f266); };
            c.w64(12, t47);
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(0), c.r32(12), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700088f26f); }
            blk = 43; continue;
        }
        42 => {
            let r48 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r48);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_l() { blk = 49; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088f284); };
            c.w32(11, t49);
            let r50 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r50);
            let r51 = c.shr(c.r32(11), 0x9u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r51);
            let r52 = c.r32(11) & 0x3ffu64;
            c.w32(11, r52);
            c.logic(r52, 32);
            c.w32(12, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f29e); };
            c.w32(12, t53);
            let r54 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r54);
            let Some(t55) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f2a6); };
            c.w32(3, t55);
            let r56 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r56);
            c.cmp(c.r32(7), c.r32(3), 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(12), c.r32(9), 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(6), c.r32(11), 32);
            if c.cc_a() { blk = 242; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088f2c0); };
            c.w64(3, t57);
            blk = 72; continue;
        }
        48 => {
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088f2c9); };
            c.w64(3, t58);
            blk = 72; continue;
        }
        49 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088f2d2); };
            c.w64(3, t59);
            blk = 72; continue;
        }
        50 => {
            c.w64(11, (c.r32(6) as u32 as i32 as i64 as u64));
            let r60 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r60);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 51; continue;
        }
        51 => {
            let r61 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r61);
            let Some(t62) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f2f0); };
            c.cmp(t62, c.r32(9), 32);
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
            let Some(t63) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f306); };
            c.w32(11, t63);
            let r64 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r64);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(11, c.r64(1));
            blk = 56; continue;
        }
        56 => {
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f316); };
            c.w32(12, t65);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r66 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r66);
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f324); };
            c.w32(15, t67);
            let r68 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r68);
            let r69 = c.r64(15) & c.r64(12);
            c.w64(15, r69);
            c.logic(r69, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 58; continue;
        }
        57 => {
            let r70 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r70);
            let r71 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r71);
            c.w64(2, c.r64(12));
            let r72 = c.r64(2) & c.r64(15);
            c.w64(2, r72);
            c.logic(r72, 64);
            c.w64(15, c.r64(2));
            blk = 58; continue;
        }
        58 => {
            c.w64(2, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t73) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f348); };
            c.w32(2, t73);
            let Some(t74) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088f34d); };
            c.cmp(t74, c.r32(2), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 59; continue;
        }
        59 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 60; continue;
        }
        60 => {
            blk = 57; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 267; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t75) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088f372); };
            c.logic(t75 & 0x24u64, 8);
            if c.cc_ne() { blk = 272; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t76) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f37c); };
            c.w32(11, t76);
            let r77 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r77);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(11, c.r64(1));
            blk = 65; continue;
        }
        65 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f38c); };
            c.w32(12, t78);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r79 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r79);
            let Some(t80) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f39a); };
            c.w32(15, t80);
            let r81 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r81);
            let r82 = c.r64(15) & c.r64(12);
            c.w64(15, r82);
            c.logic(r82, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 67; continue;
        }
        66 => {
            let r83 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r83);
            let r84 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r84);
            c.w64(2, c.r64(12));
            let r85 = c.r64(2) & c.r64(15);
            c.w64(2, r85);
            c.logic(r85, 64);
            c.w64(15, c.r64(2));
            blk = 67; continue;
        }
        67 => {
            c.w64(2, c.r64(15).wrapping_add(0x5));
            let Some(t86) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f3bd); };
            c.w32(6, t86);
            let r87 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r87);
            let Some(t88) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088f3c5); };
            c.cmp(t88, c.r32(6), 32);
            if c.cc_e() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088f3ce); };
            c.cmp(t89, c.r32(6), 32);
            if c.cc_e() { blk = 66; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t90) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f3d7); };
            c.cmp(t90, c.r32(9), 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t91) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f3dd); };
            c.w32(11, t91);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 296; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t92) = c.ld32(c.r64(6).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f3f0); };
            c.w32(11, t92);
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088f3f4); };
            c.cmp(t93, c.r32(11), 32);
            if c.cc_ne() { blk = 242; continue; }
            blk = 72; continue;
        }
        72 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t94) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f40e); };
            c.w32(8, t94);
            let r95 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r95);
            let Some(t96) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f415); };
            c.w32(8, t96);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 103; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t97) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f425); };
            c.w32(11, t97);
            let r98 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r98);
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
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 78; continue;
        }
        78 => {
            c.w64(15, c.r64(11));
            let r99 = c.r32(15) & 0x7u64;
            c.w32(15, r99);
            c.logic(r99, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 83; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 81; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 82; continue;
        }
        82 => {
            blk = 261; continue;
        }
        83 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            blk = 261; continue;
        }
        86 => {
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f492); };
            c.w32(11, t100);
            let Some(t101) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700088f497); };
            c.cmp(t101, c.r32(11), 32);
            if c.cc_ne() { blk = 262; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t102) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f4a4); };
            c.w32(12, t102);
            let r103 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r103);
            let Some(t104) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f4ac); };
            c.w32(11, t104);
            let r105 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r105);
            let Some(t106) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f4b4); };
            c.w32(11, t106);
            blk = 78; continue;
        }
        88 => {
            let Some(t107) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f4bb); };
            c.w32(11, t107);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r108 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f4c9); };
            c.w32(12, t109);
            let r110 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r110);
            let Some(t111) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f4d1); };
            c.w32(11, t111);
            let r112 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r112);
            let Some(t113) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f4d9); };
            c.w32(11, t113);
            blk = 78; continue;
        }
        89 => {
            let Some(t114) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f4e3); };
            c.w32(12, t114);
            let r115 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r115);
            let Some(t116) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f4eb); };
            c.w32(11, t116);
            let r117 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r117);
            let Some(t118) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f4f3); };
            c.w32(11, t118);
            blk = 78; continue;
        }
        90 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 93; continue;
        }
        91 => {
            c.w32(15, 0x1u64);
            blk = 93; continue;
        }
        92 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 93; continue;
        }
        93 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 263; continue; }
            blk = 94; continue;
        }
        94 => {
            let r119 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r119);
            blk = 95; continue;
        }
        95 => {
            let r120 = c.r32(11) & 0x8u64;
            c.w32(11, r120);
            c.logic(r120, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t121) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x700088f52d); };
            c.w32(8, t121);
            blk = 98; continue;
        }
        97 => {
            let Some(t122) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700088f534); };
            c.w32(8, t122);
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 271; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 270; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 268; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088f567); };
            c.w64(7, t123);
            let Some(t124) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f56b); };
            c.w32(8, t124);
            let Some(t125) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700088f56f); };
            c.cmp(t125, c.r32(8), 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t126) = c.ld8(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f578); };
            c.logic(t126 & 0x1u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t127) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f582); };
            c.w32(2, t127);
            let r128 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088f588); };
            c.cmp(t129, c.r32(2), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 106; continue;
        }
        106 => {
            blk = 31; continue;
        }
        107 => {
            c.w64(9, c.r64(2));
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088f59d); };
            c.w64(2, t130);
            blk = 109; continue;
        }
        108 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 109; continue;
        }
        109 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700088f5a9); }
            c.w64(11, c.r64(8));
            c.w64(8, c.r64(2));
            blk = 111; continue;
        }
        110 => {
            let Some(t131) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088f5b5); };
            c.w32(3, t131);
            let r132 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r132);
            let Some(t133) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f5bc); };
            c.w32(11, t133);
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(11), 0x410u64, 32);
            if c.cc_le() { blk = 279; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t134) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088f5ce); };
            c.w32(11, t134);
            let r135 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r135);
            let r136 = c.r32(11) & 0x3fu64;
            c.w32(11, r136);
            c.logic(r136, 32);
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
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
            blk = 279; continue;
        }
        181 => {
            blk = 186; continue;
        }
        182 => {
            let Some(t137) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f7f7); };
            c.w32(8, t137);
            let r138 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r138);
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f7fe); };
            c.w32(11, t139);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ae() { blk = 187; continue; }
            blk = 183; continue;
        }
        183 => {
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t140) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f817); };
            c.cmp(t140, c.r64(11), 64);
            if c.cc_e() { blk = 205; continue; }
            blk = 184; continue;
        }
        184 => {
            blk = 242; continue;
        }
        185 => {
            let Some(t141) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f827); };
            c.w32(11, t141);
            let r142 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r142);
            let Some(t143) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f82e); };
            c.w32(11, t143);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 242; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t144) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f83b); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088f842); };
            c.w32(11, t146);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 189; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_b() { blk = 279; continue; }
            blk = 188; continue;
        }
        188 => {
            blk = 205; continue;
        }
        189 => {
            let Some(t147) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f865); };
            c.w32(8, t147);
            let Some(t148) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088f86a); };
            c.cmp(t148, c.r32(8), 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 190; continue;
        }
        190 => {
            blk = 242; continue;
        }
        191 => {
            let Some(t149) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f87c); };
            c.w32(11, t149);
            let r150 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r150);
            let Some(t151) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f883); };
            c.w32(11, t151);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 242; continue; }
            blk = 192; continue;
        }
        192 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700088f890); }
            c.w32(11, 0xfffffffeu64);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 279; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t152) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f8a3); };
            c.w32(8, t152);
            let r153 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r153);
            let Some(t154) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f8aa); };
            c.w32(11, t154);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r155 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r155);
            c.w32(15, c.r32(9));
            c.w64(10, c.r64(4));
            let r156 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r156);
            let r157 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r157);
            c.logic(r157, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700088f8c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700088f8cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700088f8cf); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(15));
            let Some(t158) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x700088f8d9); };
            c.w64(0, t158);
            c.w64(10, 0x700088f8e7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700088f8e7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700088f8eb); }
            return c.call(c.r64(0), 0x700088f8f1, 0x700088f8ef);
        }
        194 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700088f8f1); }
            let Some(t159) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088f8f9); };
            c.w64(4, t159);
            c.w32(8, c.r32(0));
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088f901); };
            let r161 = c.r64(8) & t160;
            c.w64(8, r161);
            c.logic(r161, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700088f908); };
            c.w64(11, t162);
            blk = 196; continue;
        }
        195 => {
            let r163 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r163);
            let r164 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088f915); };
            c.w64(12, t165);
            let r166 = c.r64(12) & c.r64(8);
            c.w64(12, r166);
            c.logic(r166, 64);
            c.w64(8, c.r64(12));
            blk = 196; continue;
        }
        196 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t167) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088f924); };
            c.w32(12, t167);
            let r168 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088f92c); };
            c.cmp(t169, c.r32(12), 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 197; continue;
        }
        197 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t170) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088f93b); };
            c.cmp(t170, c.r32(12), 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 199; continue;
        }
        199 => {
            c.xorpd_rr(0, 0);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088f948); };
            c.cvtsi2sd(0, (t171 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700088f94e); }
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
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088f969); };
            c.cmp(c.r64(12), t172, 64);
            if c.cc_e() { blk = 242; continue; }
            blk = 203; continue;
        }
        203 => {
            blk = 195; continue;
        }
        204 => {
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088f975); };
            c.w64(7, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088f979); };
            c.w64(3, t174);
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088f97d); };
            c.w64(9, t175);
            blk = 205; continue;
        }
        205 => {
            let Some(t176) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f981); };
            c.w32(8, t176);
            let r177 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088f988); };
            c.w64(2, t178);
            let Some(t179) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088f98c); };
            c.cmp(t179, c.r32(8), 32);
            if c.cc_e() { blk = 243; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 110; continue;
        }
        207 => {
            let Some(t180) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f99e); };
            c.w32(11, t180);
            let r181 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r181);
            let Some(t182) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700088f9a5); };
            c.logic(t182 & 0x4u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t183) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088f9b0); };
            c.cmp(t183, c.r64(9), 64);
            if c.cc_a() { blk = 242; continue; }
            blk = 209; continue;
        }
        209 => {
            blk = 243; continue;
        }
        210 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f9bf); };
            c.w32(11, t184);
            let r185 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r185);
            let Some(t186) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700088f9c6); };
            c.w32(12, t186);
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 239; continue; }
            blk = 211; continue;
        }
        211 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 243; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t187) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088f9de); };
            c.w64(11, t187);
            let Some(t188) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700088f9e2); };
            c.w64(12, t188);
            let Some(t189) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700088f9e6); };
            c.logic(t189 & 0x1u64, 8);
            if c.cc_ne() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t190) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700088f9ed); };
            c.w64(15, t190);
            let r191 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r191);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t192) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700088f9fd); };
            c.w64(8, t192);
            c.w64(0, c.r64(8));
            blk = 241; continue;
        }
        215 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 243; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t193) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088fa12); };
            c.w32(8, t193);
            let r194 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r194);
            let Some(t195) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700088fa19); };
            c.w32(8, t195);
            let r196 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r196);
            let r197 = c.r32(8) & 0x3fu64;
            c.w32(8, r197);
            c.logic(r197, 32);
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
            blk = 257; continue;
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
            blk = 257; continue;
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
            blk = 257; continue;
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
            blk = 257; continue;
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
            let r198 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r198);
            c.w64(0, c.r64(11));
            c.cqo();
            c.idiv(c.r64(8), 64);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088fac3); };
            c.w64(11, t199);
            blk = 241; continue;
        }
        239 => {
            c.w64(10, c.r64(4));
            let r200 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r200);
            let r201 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r201);
            c.logic(r201, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700088fad4); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700088fadf); };
            c.w64(0, t202);
            c.w64(10, 0x700088faedu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700088faed); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700088faf1); }
            return c.call(c.r64(0), 0x700088faf7, 0x700088faf5);
        }
        240 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700088faf7); }
            let Some(t203) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700088faff); };
            c.w64(4, t203);
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088fb04); };
            c.w64(9, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fb08); };
            c.w64(7, t205);
            blk = 241; continue;
        }
        241 => {
            c.w64(2, c.r64(9));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088fb14); };
            c.w64(8, t206);
            blk = 244; continue;
        }
        243 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700088fb1d); };
            c.w64(8, t207);
            blk = 244; continue;
        }
        244 => {
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700088fb24); };
            c.cmp(t208, c.r32(8), 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088fb2d); };
            c.w64(1, t209);
            blk = 250; continue;
        }
        246 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088fb33); };
            c.w64(0, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fb37); };
            c.w64(3, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088fb3b); };
            c.w64(6, t212);
            return c.call(0x7000870914, 0x700088fb44, 0x700088fb3f);
        }
        247 => {
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088fb44); };
            c.w64(8, t213);
            let Some(t214) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088fb48); };
            c.cmp(t214, c.r32(8), 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088fb51); };
            if !c.push64(t215) { return RegfileExit::fault(0x700088fb51); }
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fb54); };
            if !c.push64(t216) { return RegfileExit::fault(0x700088fb54); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088fb57); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088fb58); }
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088fb5a); };
            c.w64(8, t217);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088fb61); }
            c.w32(0, 0x4u64);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088fb68); };
            c.w64(7, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088fb6c); };
            c.w64(6, t219);
            return c.call(0x70007f2134, 0x700088fb75, 0x700088fb70);
        }
        249 => {
            c.w64(1, c.r64(0));
            blk = 250; continue;
        }
        250 => {
            let Some(t220) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088fb78); };
            c.cmp(t220, 0x0u64, 32);
            if c.cc_e() { blk = 281; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088fb82); };
            c.w64(8, t221);
            let r222 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r222);
            if c.cc_o() { blk = 281; continue; }
            blk = 252; continue;
        }
        252 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        253 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088fb98); };
            c.w64(0, t223);
            let Some(t224) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088fb9c); };
            c.cmp(t224, c.r32(0), 32);
            if c.cc_e() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700088fba8); }
            return c.ret(0x8, 0x700088fba9);
        }
        255 => {
            c.w32(8, 0x140u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088fbb2); }
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x700088fbb4); };
            c.w64(8, t225);
            let Some(t226) = c.ld32(c.r64(8).wrapping_add(0x153)) else { return RegfileExit::fault(0x700088fbbb); };
            c.w32(8, t226);
            let r227 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r227);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700088fbc5); }
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700088fbc7); };
            c.w64(3, t228);
            c.w32(0, 0x2u64);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088fbd3); };
            c.w64(6, t229);
            return c.call(0x700086ed74, 0x700088fbdc, 0x700088fbd7);
        }
        256 => {
            return RegfileExit::fault(0x700088fbdc);
        }
        257 => {
            return RegfileExit::fault(0x700088fbdd);
        }
        258 => {
            blk = 279; continue;
        }
        259 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 279; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t230) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088fbee); };
            c.w32(9, t230);
            let r231 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r231);
            blk = 30; continue;
        }
        261 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700088fbfa); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700088fbfe); }
            blk = 265; continue;
        }
        262 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700088fc04); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700088fc08); }
            blk = 265; continue;
        }
        263 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700088fc0e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700088fc12); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t232) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088fc1c); };
            c.w64(12, t232);
            blk = 95; continue;
        }
        265 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700088fc26); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088fc28); }
            c.w32(0, 0x2u64);
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x700088fc2e); };
            c.w64(3, t233);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700088fc3d, 0x700088fc38);
        }
        266 => {
            let r234 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r234);
            c.w64(8, c.r64(0));
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700088fc42); };
            c.w64(3, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700088fc46); };
            c.w64(9, t236);
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700088fc4a); };
            c.w64(1, t237);
            blk = 98; continue;
        }
        267 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fc56); };
            c.w64(7, t238);
            blk = 279; continue;
        }
        268 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fc5c); };
            c.w64(7, t239);
            blk = 279; continue;
        }
        269 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fc62); };
            c.w64(7, t240);
            blk = 279; continue;
        }
        270 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fc68); };
            c.w64(7, t241);
            blk = 279; continue;
        }
        271 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fc6e); };
            c.w64(7, t242);
            blk = 279; continue;
        }
        272 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fc74); };
            c.w64(7, t243);
            blk = 279; continue;
        }
        273 => {
            c.w64(0, c.r64(7));
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088fc7d); };
            c.w64(6, t244);
            return c.call(0x70008a1af4, 0x700088fc86, 0x700088fc81);
        }
        274 => {
            let Some(t245) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088fc86); };
            c.w32(8, t245);
            let Some(t246) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x700088fc8a); };
            c.cmp(t246, c.r32(8), 32);
            if c.cc_ne() { blk = 277; continue; }
            blk = 275; continue;
        }
        275 => {
            let Some(t247) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088fc93); };
            c.logic(t247 & 0x1u64, 8);
            if c.cc_e() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fc99); };
            c.w64(7, t248);
            blk = 243; continue;
        }
        277 => {
            c.w64(3, c.r64(0));
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088fca5); };
            c.w64(0, t249);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088fca9); };
            c.w64(6, t250);
            return c.call(0x70008bced4, 0x700088fcb2, 0x700088fcad);
        }
        278 => {
            c.w64(8, c.r64(0));
            blk = 244; continue;
        }
        279 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700088fcba); };
            if !c.push64(t251) { return RegfileExit::fault(0x700088fcba); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088fcbd); }
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700088fcbe); };
            c.w64(3, t252);
            c.w32(0, 0x2u64);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700088fcca); };
            c.w64(6, t253);
            return c.call(0x700086ed74, 0x700088fcd3, 0x700088fcce);
        }
        280 => {
            c.w64(8, c.r64(0));
            blk = 244; continue;
        }
        281 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700088fcdb); }
            let Some(t254) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088fcdf); };
            c.cmp(t254, 0x0u64, 32);
            if c.cc_ne() { blk = 283; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fce5); };
            c.w64(7, t255);
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700088fce9); }
            blk = 284; continue;
        }
        283 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088fcf0); };
            c.w64(7, t256);
            let r257 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r257);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 284; continue;
        }
        284 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700088fd1e); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 290; continue; }
            blk = 285; continue;
        }
        285 => {
            if c.cc_ne() { blk = 290; continue; }
            blk = 286; continue;
        }
        286 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 288; continue; }
            blk = 287; continue;
        }
        287 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 290; continue; }
            blk = 288; continue;
        }
        288 => {
            c.w64(9, c.r64(8));
            let r258 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r258);
            if c.cc_o() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w64(7, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(7));
            blk = 295; continue;
        }
        290 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700088fd50); };
            c.w64(8, t259);
            let Some(t260) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700088fd57); };
            c.w64(9, t260);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700088fd5e); };
            c.w64(12, t261);
            let Some(t262) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700088fd65); };
            c.cmp(t262, c.r64(11), 64);
            if c.cc_a() { blk = 293; continue; }
            blk = 291; continue;
        }
        291 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700088fd75, 0x700088fd70);
        }
        292 => {
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700088fd75); };
            c.w64(1, t263);
            blk = 294; continue;
        }
        293 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700088fd7b); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 294; continue;
        }
        294 => {
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700088fd85); };
            c.w64(8, t264);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700088fd8c); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700088fd90); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700088fd95); }
            blk = 295; continue;
        }
        295 => {
            c.w64(7, c.r64(0));
            blk = 1; continue;
        }
        296 => {
            return RegfileExit::fault(0x700088fda2);
        }
        297 => {
            return RegfileExit::jump(0x700088fda4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700088fdb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700088fdb4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700088fdb8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700088fdb9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700088fdba); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x700088fdbf); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700088fdc3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x700088fdc7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 105; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088fddf); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700088fde4); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088fdf3); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700088fe02); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700088fe06); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088fe1d); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088fe24); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 107; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700088fe3e); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088fe48); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088fe4f); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088fe58); };
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
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088fe78); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088fe7f); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088fe86); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088fe8d); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x700088fe9d);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700088fe9e); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088feb5); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700088febc); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700088fec4); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088fecc); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x700088fed9);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x700088feda); };
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
            return RegfileExit::fault(0x700088feed);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700088feee); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 110; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x700088ff05);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700088ff06); };
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
            return c.call(0x70008a17f4, 0x700088ff1b, 0x700088ff16);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700088ff1e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700088ff22); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700088ff26); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700088ff2a); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700088ff2e); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x700088ff52); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x700088ff56); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 115; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x700088ff69); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088ff72); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700088ff79); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700088ff80); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ff8d); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700088ff93); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_g() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t42) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700088ffa3); };
            c.w64(8, t42);
            blk = 40; continue;
        }
        37 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(8, c.r64(9));
            let Some(t43) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700088ffb5); };
            c.w64(8, t43);
            blk = 40; continue;
        }
        39 => {
            c.w64(8, c.r64(2));
            blk = 40; continue;
        }
        40 => {
            c.logic(c.r8(15) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700088ffca); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(9)) { return RegfileExit::fault(0x700088ffce); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700088ffdc); };
            c.w32(3, t44);
            let r45 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r45);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(3)) { return RegfileExit::fault(0x700088ffe3); }
            let Some(t46) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700088ffe7); };
            c.cmp(t46, 0x834u64, 16);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(7, 0x1u64);
            blk = 44; continue;
        }
        43 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t47) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000890001); };
            c.w32(7, t47);
            c.w64(8, c.r64(7));
            let r48 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r48);
            let r49 = c.r32(8) & 0x3fu64;
            c.w32(8, r49);
            c.logic(r49, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x7000890010); }
            c.cmp(c.r32(8), 0xbu64, 32);
            if c.cc_g() { blk = 100; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t50) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089001e); };
            c.w32(8, t50);
            let r51 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r51);
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000890025); };
            c.w32(8, t52);
            let r53 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r53);
            let Some(t54) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089002c); };
            c.w32(0, t54);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0x4cu64, 64);
            if c.cc_a() { blk = 48; continue; }
            blk = 47; continue;
        }
        47 => {
            return RegfileExit::fault(0x700089003c);
        }
        48 => {
            let Some(t55) = c.ld32(c.r64(8).wrapping_add(0x137)) else { return RegfileExit::fault(0x700089003d); };
            c.w32(8, t55);
            let Some(t56) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000890044); };
            c.cmp(t56, c.r32(8), 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700089004e); };
            c.w64(8, t57);
            let Some(t58) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000890055); };
            c.cmp(t58, c.r32(12), 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(0, c.r64(12));
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000890062); };
            c.w64(1, t59);
            blk = 52; continue;
        }
        51 => {
            c.w32(9, c.r64(0).wrapping_add(0x2));
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 52; continue;
        }
        52 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 53; continue;
        }
        53 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089007e); }
            let Some(t60) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000890083); };
            c.cmp(t60, 0x0u64, 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890089); }
            blk = 58; continue;
        }
        55 => {
            c.w64(2, c.r64(15));
            let r61 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r61);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 58; continue;
        }
        56 => {
            let Some(t62) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008900a0); };
            c.cmp(t62, 0x0u64, 32);
            if c.cc_ne() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(2, c.r64(0));
            let r63 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r63);
            if !c.movsd_rm(1, c.r64(15).wrapping_add(0x3)) { return RegfileExit::fault(0x70008900ab); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 58; continue;
        }
        58 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 61; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 89; continue;
        }
        60 => {
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_ge() { blk = 89; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t64) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008900cd); };
            c.cmp(t64, c.r32(3), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008900d7); };
            c.cmp(t65, c.r32(12), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008900e1); };
            c.w32(2, t66);
            let r67 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r67);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008900ed); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008900f7); }
            blk = 69; continue;
        }
        66 => {
            c.w64(2, c.r64(0));
            let r68 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r68);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 69; continue;
        }
        67 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 68; continue;
        }
        68 => {
            let r69 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r69);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890114); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 69; continue;
        }
        69 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 98; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 72; continue;
        }
        71 => {
            c.cmp(c.r32(2), c.r32(0), 32);
            if c.cc_le() { blk = 98; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t70) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890135); };
            c.w32(2, t70);
            let r71 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r71);
            let r72 = c.r32(2) & 0x3fu64;
            c.w32(2, r72);
            c.logic(r72, 32);
            let r73 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r73);
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t74) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890146); };
            c.w32(2, t74);
            let r75 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r75);
            let Some(t76) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089014d); };
            c.w32(6, t76);
            let r77 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r77);
            let Some(t78) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890153); };
            c.w32(6, t78);
            let r79 = c.sub(c.r32(6), 0x77u64, 32);
            c.w32(6, r79);
            c.cmp(c.r32(6), 0xcu64, 32);
            if c.cc_a() { blk = 95; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000890163); };
            c.w32(6, t80);
            c.cmp(c.r32(0), c.r32(6), 32);
            if c.cc_ae() { blk = 94; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(6, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t81) = c.ld32(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890171); };
            c.w32(2, t81);
            let r82 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000890178); };
            c.cmp(t83, c.r32(2), 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 83; continue;
        }
        77 => {
            let Some(t84) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089018a); };
            c.w32(2, t84);
            let r85 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r85);
            let Some(t86) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000890191); };
            c.w32(6, t86);
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000890194); };
            c.cmp(t87, c.r32(6), 32);
            if c.cc_ne() { blk = 97; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t88) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008901a1); };
            c.w32(6, t88);
            c.cmp(c.r32(0), c.r32(6), 32);
            if c.cc_ae() { blk = 96; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(6, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x70008901af); }
            c.w64(8, 0xfff7fffffff7ffffu64);
            let Some(t89) = c.ld64(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008901bf); };
            c.cmp(t89, c.r64(8), 64);
            if c.cc_e() { blk = 88; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008901ca); };
            c.w64(8, t90);
            let Some(t91) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008901d1); };
            c.w64(2, t91);
            c.w64(6, c.r64(2).wrapping_add(0xc));
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008901d8); };
            c.w64(9, t92);
            let Some(t93) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008901df); };
            c.cmp(t93, c.r64(6), 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.st64(c.r64(8), c.r64(6)) { return RegfileExit::fault(0x70008901e8); }
            c.w64(8, c.r64(2).wrapping_add(0x1));
            blk = 82; continue;
        }
        82 => {
            let Some(t94) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008901ef); };
            c.w64(9, t94);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008901f6); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008901fa); }
            c.w64(2, c.r64(8));
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000890203); };
            c.w64(8, t95);
            blk = 83; continue;
        }
        83 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089020a); };
            c.cmp(t96, c.r32(1), 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000890213); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000890215); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000890216); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000890217); }
            let Some(t97) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000890218); };
            c.w64(2, t97);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089021f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000890220); }
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890224); };
            c.w64(6, t98);
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890228); };
            c.w64(7, t99);
            c.w32(0, 0x4u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x7000890239, 0x7000890234);
        }
        85 => {
            c.w64(9, c.r64(0));
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089023c); };
            c.w64(11, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000890240); };
            c.w64(15, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890244); };
            c.w64(0, t102);
            c.w32(12, 0);
            c.logic(0, 32);
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700089024b); };
            c.w64(8, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000890252); };
            c.w64(3, t104);
            blk = 87; continue;
        }
        86 => {
            c.w64(9, c.r64(2));
            blk = 87; continue;
        }
        87 => {
            c.w64(1, c.r64(9));
            blk = 51; continue;
        }
        88 => {
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000890263); };
            c.w64(8, t105);
            blk = 51; continue;
        }
        89 => {
            let Some(t106) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089026f); };
            c.cmp(t106, c.r32(1), 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000890278); };
            c.w64(8, t107);
            let r108 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r108);
            c.w64(0, c.r64(1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000890286); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000890287); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089028d); }
            return c.ret(0x0, 0x700089028f);
        }
        91 => {
            c.w32(8, 0x140u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000890296); }
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000890298); };
            c.w64(8, t109);
            let Some(t110) = c.ld32(c.r64(8).wrapping_add(0x153)) else { return RegfileExit::fault(0x700089029f); };
            c.w32(8, t110);
            let r111 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r111);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008902a9); }
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008902ab); };
            c.w64(3, t112);
            c.w32(0, 0x2u64);
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008902b7); };
            c.w64(6, t113);
            return c.call(0x700086ed74, 0x70008902c0, 0x70008902bb);
        }
        92 => {
            return RegfileExit::fault(0x70008902c0);
        }
        93 => {
            blk = 98; continue;
        }
        94 => {
            return RegfileExit::fault(0x70008902c3);
        }
        95 => {
            return RegfileExit::fault(0x70008902c4);
        }
        96 => {
            return RegfileExit::fault(0x70008902c5);
        }
        97 => {
            return RegfileExit::fault(0x70008902c6);
        }
        98 => {
            c.w64(12, c.r64(0));
            c.w64(8, c.r64(1));
            blk = 101; continue;
        }
        99 => {
            c.w64(12, c.r64(0));
            c.w64(8, c.r64(1));
            blk = 101; continue;
        }
        100 => {
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008902d7); };
            c.w64(8, t114);
            blk = 101; continue;
        }
        101 => {
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008902db); }
            c.w64(0, c.r64(11));
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008902e0); };
            c.w64(3, t115);
            c.w64(1, c.r64(8));
            c.w64(2, c.r64(11));
            c.w64(7, c.r64(12));
            let Some(t116) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008902ed); };
            c.w64(6, t116);
            return c.call(0x700088f014, 0x70008902f6, 0x70008902f1);
        }
        102 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008902f6); };
            c.w64(8, t117);
            let r118 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r118);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000890301); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000890302); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000890308); }
            return c.ret(0x0, 0x700089030a);
        }
        103 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000890311); }
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000890313); };
            c.w64(8, t119);
            let Some(t120) = c.ld32(c.r64(8).wrapping_add(0x153)) else { return RegfileExit::fault(0x700089031a); };
            c.w32(8, t120);
            let r121 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r121);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000890324); }
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000890326); };
            c.w64(3, t122);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000890337, 0x7000890332);
        }
        104 => {
            return RegfileExit::fault(0x7000890337);
        }
        105 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000890338); };
            c.w64(3, t123);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000890349, 0x7000890344);
        }
        106 => {
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890349); };
            c.w64(6, t124);
            let Some(t125) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089034d); };
            c.w64(0, t125);
            blk = 1; continue;
        }
        107 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x700089035e, 0x7000890359);
        }
        108 => {
            c.w64(11, c.r64(0));
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890361); };
            c.w64(6, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890365); };
            c.w64(8, t127);
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000890369); };
            c.w64(9, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089036d); };
            c.w64(0, t129);
            blk = 4; continue;
        }
        109 => {
            let Some(t130) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000890376); };
            c.w32(12, t130);
            let r131 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r131);
            blk = 23; continue;
        }
        110 => {
            let Some(t132) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000890382); };
            c.cmp(t132, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t133) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000890390); };
            c.w64(3, t133);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x700089039f, 0x700089039a);
        }
        112 => {
            c.w64(12, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008903a2); };
            c.w64(6, t134);
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008903a6); };
            c.w64(8, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008903aa); };
            c.w64(11, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008903ae); };
            c.w64(9, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008903b2); };
            c.w64(0, t138);
            blk = 23; continue;
        }
        113 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008903c0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x70008903c4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008903c9); }
            return c.call(0x70007febf4, 0x70008903d2, 0x70008903cd);
        }
        114 => {
            c.w64(8, c.r64(0));
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008903d5); };
            c.w64(11, t139);
            let Some(t140) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008903d9); };
            c.w64(15, t140);
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008903dd); };
            c.w64(0, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008903e1); };
            c.w64(1, t142);
            c.w32(12, 0);
            c.logic(0, 32);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x70008903e8); }
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008903ed); };
            c.w64(3, t143);
            blk = 82; continue;
        }
        115 => {
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008903f6); };
            c.w64(2, t144);
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t145) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000890402); };
            c.w64(8, t145);
            c.w64(2, c.r64(8));
            blk = 118; continue;
        }
        118 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089040f); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000890411); }
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000890412); };
            c.w64(3, t146);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000890423, 0x700089041e);
        }
        119 => {
            return RegfileExit::fault(0x7000890423);
        }
        120 => {
            return RegfileExit::jump(0x7000890428);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000890434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000890434); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000890438); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000890439); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089043a); }
            let r0 = c.sub(c.r64(4), 0x70u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(6)) { return RegfileExit::fault(0x700089043f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000890443); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000890447); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 178; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(8, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089045b); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000890476); };
            c.w32(9, t3);
            let r4 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r4);
            let Some(t5) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089047d); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(12, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.w32(11, 0x1u64);
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008904a1); };
            c.w32(12, t6);
            c.w64(15, c.r64(12));
            let r7 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r7);
            let r8 = c.r32(15) & 0x3fu64;
            c.w32(15, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_a() { blk = 108; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008904bb); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008904c2); };
            c.w32(15, t11);
            let r12 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r12);
            let Some(t13) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008904c9); };
            c.w32(3, t13);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x4cu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008904d9);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(15).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008904da); };
            c.w32(15, t14);
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008904e1); };
            c.cmp(t15, c.r32(15), 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008904ee); };
            c.w64(15, t16);
            let Some(t17) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008904f5); };
            c.cmp(t17, c.r32(9), 32);
            if c.cc_e() { blk = 108; continue; }
            blk = 11; continue;
        }
        11 => {
            let r18 = c.r32(12) & 0xfcu64;
            c.w32(12, r18);
            c.logic(r18, 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(12), 0x8u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x10u64, 32);
            if c.cc_ne() { blk = 109; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890520); };
            c.w32(8, t19);
            let r20 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r20);
            let Some(t21) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000890527); };
            c.w32(11, t21);
            let r22 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r22);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 16; continue;
        }
        15 => {
            let r23 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r23);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r24 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r24);
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700089054c); }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ge() { blk = 103; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089055c); };
            c.w32(12, t25);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_ae() { blk = 23; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000890568); }
            c.w64(3, 0xfff7fffffff7ffffu64);
            let Some(t26) = c.ld64(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890579); };
            c.cmp(t26, c.r64(3), 64);
            if c.cc_e() { blk = 26; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ae() { blk = 24; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(1, c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x7000890588); }
            let Some(t27) = c.ld64(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089058f); };
            c.cmp(t27, c.r64(3), 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 21; continue;
        }
        21 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 1, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x7), 1) { return RegfileExit::fault(0x70008905a0); }
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008905a7); };
            c.w32(15, t28);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_b() { blk = 15; continue; }
            blk = 22; continue;
        }
        22 => {
            blk = 180; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008905b5);
        }
        24 => {
            return RegfileExit::fault(0x70008905b6);
        }
        25 => {
            return RegfileExit::fault(0x70008905b7);
        }
        26 => {
            return RegfileExit::fault(0x70008905b8);
        }
        27 => {
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008905b9); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008905c0); };
            c.w32(11, t31);
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008905c4); };
            c.cmp(t32, c.r32(11), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t33) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008905d1); };
            c.w32(11, t33);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008905df); };
            c.w32(12, t34);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_a() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ae() { blk = 34; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008905f7); };
            c.w64(2, t35);
            let Some(t36) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008905fe); };
            c.w32(2, t36);
            let r37 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r37);
            return c.call(0x7000824234, 0x700089060c, 0x7000890607);
        }
        32 => {
            return RegfileExit::fault(0x700089060c);
        }
        33 => {
            return RegfileExit::fault(0x700089060d);
        }
        34 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 184; continue; }
            blk = 36; continue;
        }
        36 => {
            let r38 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r38);
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(3, c.r32(3));
            let r39 = c.imul2(c.r32(3), 0x4u64, 32);
            c.w32(3, r39);
            if c.cc_o() { blk = 49; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r40 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r40);
            if c.cc_o() { blk = 48; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089064d); };
            c.w64(1, t41);
            let Some(t42) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000890654); };
            c.w64(1, t42);
            c.w64(6, c.r64(1).wrapping_add(c.r64(3)));
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089065b); };
            c.w64(0, t43);
            let Some(t44) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000890662); };
            c.cmp(t44, c.r64(6), 64);
            if c.cc_be() { blk = 182; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 182; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000890678); };
            c.w64(0, t45);
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x700089067f); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 41; continue;
        }
        41 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000890686); };
            c.w64(3, t46);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700089068d); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000890690); }
            c.w64(12, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(15, c.r64(2));
            blk = 43; continue;
        }
        42 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008906a1); }
            let r47 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r47);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(3, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t48) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008906bb); };
            c.w32(3, t48);
            let r49 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r49);
            blk = 42; continue;
        }
        46 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008906c6); };
            c.w64(3, t50);
            blk = 42; continue;
        }
        47 => {
            c.w64(8, c.r64(0));
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008906d2); };
            c.w64(0, t51);
            blk = 51; continue;
        }
        48 => {
            return RegfileExit::fault(0x70008906d8);
        }
        49 => {
            return RegfileExit::fault(0x70008906d9);
        }
        50 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008906da); };
            c.w64(8, t52);
            blk = 51; continue;
        }
        51 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008906e1); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r53 = c.r64(11) & c.r64(7);
            c.w64(11, r53);
            c.logic(r53, 64);
            let Some(t54) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008906f5); };
            c.logic(t54 & 0x4u64, 8);
            if c.cc_ne() { blk = 251; continue; }
            blk = 53; continue;
        }
        53 => {
            blk = 55; continue;
        }
        54 => {
            return RegfileExit::fault(0x7000890702);
        }
        55 => {
            let Some(t55) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890703); };
            c.w32(8, t55);
            let r56 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r56);
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089070a); };
            c.w32(11, t57);
            let r58 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r58);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 58; continue;
        }
        56 => {
            let r59 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r59);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r60 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r60);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(2)), c.r32(15)) { return RegfileExit::fault(0x7000890725); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r61 = c.r64(12) & c.r64(8);
            c.w64(12, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000890739); };
            c.logic(t62 & 0x4u64, 8);
            if c.cc_ne() { blk = 248; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ge() { blk = 103; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t63) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089074e); };
            c.w32(12, t63);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_ae() { blk = 65; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890762); };
            c.w32(15, t64);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ae() { blk = 66; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            let Some(t65) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890777); };
            c.w32(12, t65);
            if !c.st32(c.r64(8).wrapping_add(c.r64(3)), c.r32(12)) { return RegfileExit::fault(0x700089077c); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r66 = c.r64(1) & c.r64(8);
            c.w64(1, r66);
            c.logic(r66, 64);
            let Some(t67) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000890790); };
            c.logic(t67 & 0x4u64, 8);
            if c.cc_ne() { blk = 245; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t68) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089079a); };
            c.w32(12, t68);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_b() { blk = 56; continue; }
            blk = 64; continue;
        }
        64 => {
            blk = 67; continue;
        }
        65 => {
            return RegfileExit::fault(0x70008907a9);
        }
        66 => {
            return RegfileExit::fault(0x70008907aa);
        }
        67 => {
            return RegfileExit::fault(0x70008907ab);
        }
        68 => {
            let Some(t69) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008907ac); };
            c.w32(8, t69);
            let r70 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r70);
            let Some(t71) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008907b3); };
            c.w32(11, t71);
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008907b7); };
            c.cmp(t72, c.r32(11), 32);
            if c.cc_ne() { blk = 96; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t73) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008907c4); };
            c.w32(11, t73);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 95; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t74) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008907d2); };
            c.w32(12, t74);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_a() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ae() { blk = 75; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008907ea); };
            c.w64(2, t75);
            let Some(t76) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008907f1); };
            c.w32(2, t76);
            let r77 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r77);
            return c.call(0x7000824234, 0x70008907ff, 0x70008907fa);
        }
        73 => {
            return RegfileExit::fault(0x70008907ff);
        }
        74 => {
            return RegfileExit::fault(0x7000890800);
        }
        75 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 91; continue; }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 189; continue; }
            blk = 77; continue;
        }
        77 => {
            let r78 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r78);
            c.w64(15, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(3, c.r32(3));
            let r79 = c.imul2(c.r32(3), 0x4u64, 32);
            c.w32(3, r79);
            if c.cc_o() { blk = 90; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w32(2, 0x8u64);
            let r80 = c.add(c.r64(3), c.r64(2), 64);
            c.w64(3, r80);
            if c.cc_o() { blk = 89; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000890840); };
            c.w64(1, t81);
            let Some(t82) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000890847); };
            c.w64(1, t82);
            c.w64(6, c.r64(1).wrapping_add(c.r64(3)));
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089084e); };
            c.w64(0, t83);
            let Some(t84) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000890855); };
            c.cmp(t84, c.r64(6), 64);
            if c.cc_be() { blk = 187; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 187; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089086b); };
            c.w64(0, t85);
            if !c.st64(c.r64(0), c.r64(6)) { return RegfileExit::fault(0x7000890872); }
            c.w64(0, c.r64(1).wrapping_add(0x1));
            blk = 82; continue;
        }
        82 => {
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000890879); };
            c.w64(3, t86);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000890880); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000890883); }
            c.w64(12, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(15, c.r64(2));
            blk = 84; continue;
        }
        83 => {
            if !c.st32(c.r64(0).wrapping_add(c.r64(2)).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000890894); }
            let r87 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r87);
            blk = 84; continue;
        }
        84 => {
            c.cmp(c.r64(12), c.r64(2), 64);
            if c.cc_e() { blk = 88; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(3, c.r64(11).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t88) = c.ld32(c.r64(8).wrapping_add(c.r64(15)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008908ae); };
            c.w32(3, t88);
            let r89 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r89);
            blk = 83; continue;
        }
        87 => {
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008908b9); };
            c.w64(3, t90);
            blk = 83; continue;
        }
        88 => {
            c.w64(8, c.r64(0));
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008908c5); };
            c.w64(0, t91);
            blk = 92; continue;
        }
        89 => {
            return RegfileExit::fault(0x70008908cb);
        }
        90 => {
            return RegfileExit::fault(0x70008908cc);
        }
        91 => {
            let Some(t92) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008908cd); };
            c.w64(8, t92);
            blk = 92; continue;
        }
        92 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008908d4); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r93 = c.r64(11) & c.r64(7);
            c.w64(11, r93);
            c.logic(r93, 64);
            let Some(t94) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008908e8); };
            c.logic(t94 & 0x4u64, 8);
            if c.cc_ne() { blk = 242; continue; }
            blk = 94; continue;
        }
        94 => {
            blk = 96; continue;
        }
        95 => {
            return RegfileExit::fault(0x70008908f5);
        }
        96 => {
            let Some(t95) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008908f6); };
            c.w32(8, t95);
            let r96 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r96);
            let Some(t97) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008908fd); };
            c.w32(11, t97);
            let r98 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r98);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 98; continue;
        }
        97 => {
            let r99 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r99);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r100 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r100);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000890918); }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ge() { blk = 103; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t101) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000890922); };
            c.w32(12, t101);
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_ae() { blk = 104; continue; }
            blk = 100; continue;
        }
        100 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t102) = c.ld32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089092e); };
            c.w32(3, t102);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_ae() { blk = 105; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(12, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t103) = c.ld32(c.r64(8).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089093b); };
            c.w32(2, t103);
            if !c.st32(c.r64(8).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x7000890940); }
            let Some(t104) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000890945); };
            c.w32(15, t104);
            c.cmp(c.r32(11), c.r32(15), 32);
            if c.cc_b() { blk = 97; continue; }
            blk = 102; continue;
        }
        102 => {
            blk = 191; continue;
        }
        103 => {
            c.w64(8, (c.r32(0) as u32 as i32 as i64 as u64));
            let r105 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r105);
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000890960); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000890961); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000890967); }
            return c.ret(0x0, 0x7000890969);
        }
        104 => {
            return RegfileExit::fault(0x700089096a);
        }
        105 => {
            return RegfileExit::fault(0x700089096b);
        }
        106 => {
            blk = 108; continue;
        }
        107 => {
            c.w32(11, 0x1u64);
            blk = 108; continue;
        }
        108 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t106) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000890981); };
            c.w32(8, t106);
            let r107 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r107);
            let Some(t108) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000890988); };
            c.cmp(t108, 0xbau64, 16);
            if c.cc_b() { blk = 192; continue; }
            blk = 111; continue;
        }
        111 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(7)) { return RegfileExit::fault(0x70008909a1); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t109) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008909ae); };
            c.w32(11, t109);
            let r110 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r110);
            let Some(t111) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008909b5); };
            c.cmp(t111, 0x834u64, 16);
            if c.cc_ne() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t112) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008909be); };
            c.w32(8, t112);
            let r113 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r113);
            blk = 134; continue;
        }
        114 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t114) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008909d3); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008909da); };
            c.w32(11, t116);
            let r117 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r117);
            let Some(t118) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008909e1); };
            c.w32(12, t118);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t119) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008909e8); };
            c.w32(15, t119);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0xb0u64, 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            return RegfileExit::fault(0x70008909f8);
        }
        118 => {
            let Some(t120) = c.ld32(c.r64(11).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008909f9); };
            c.cmp(t120, c.r32(15), 32);
            if c.cc_e() { blk = 194; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 196; continue; }
            blk = 120; continue;
        }
        120 => {
            let Some(t121) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000890a10); };
            c.w32(8, t121);
            let r122 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r122);
            let Some(t123) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000890a17); };
            c.w32(8, t123);
            let r124 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r124);
            let Some(t125) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000890a1e); };
            c.w32(11, t125);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t126) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000890a25); };
            c.w32(12, t126);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x44u64, 64);
            if c.cc_a() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            return RegfileExit::fault(0x7000890a32);
        }
        122 => {
            let Some(t127) = c.ld32(c.r64(8).wrapping_add(0x117)) else { return RegfileExit::fault(0x7000890a33); };
            c.cmp(t127, c.r32(12), 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r64(11), 0xaeu64, 64);
            if c.cc_a() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            return RegfileExit::fault(0x7000890a45);
        }
        125 => {
            let Some(t128) = c.ld32(c.r64(8).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x7000890a46); };
            c.cmp(t128, c.r32(12), 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r64(11), 0xafu64, 64);
            if c.cc_a() { blk = 195; continue; }
            blk = 127; continue;
        }
        127 => {
            return RegfileExit::fault(0x7000890a5c);
        }
        128 => {
            let Some(t129) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000890a5d); };
            c.w32(8, t129);
            let r130 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r130);
            blk = 129; continue;
        }
        129 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(0, c.r64(8));
            return c.call(0x70008a17f4, 0x7000890a72, 0x7000890a6d);
        }
        131 => {
            c.w64(8, c.r64(0));
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890a75); };
            c.w64(7, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890a7c); };
            c.w64(0, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890a80); };
            c.w64(6, t133);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 134; continue;
        }
        132 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ge() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            c.w64(8, c.r64(9));
            blk = 134; continue;
        }
        134 => {
            c.w32(11, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 135; continue;
        }
        135 => {
            let r134 = c.sub(c.r32(11), 0x2u64, 32);
            c.w32(11, r134);
            if c.cc_o() { blk = 198; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 138; continue;
        }
        137 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890aba); };
            c.w64(6, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890abe); };
            c.w64(7, t136);
            blk = 138; continue;
        }
        138 => {
            c.w32(11, c.r32(8));
            c.w32(12, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000890ae1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000890ae5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000890ae9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000890aed); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 142; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890af7); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890b02); }
            blk = 144; continue;
        }
        141 => {
            c.w64(2, c.r64(8));
            let r137 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r137);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 144; continue;
        }
        142 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 146; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(2, c.r64(9));
            let r138 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r138);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890b23); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 144; continue;
        }
        144 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 147; continue; }
            blk = 145; continue;
        }
        145 => {
            blk = 177; continue;
        }
        146 => {
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_ge() { blk = 177; continue; }
            blk = 147; continue;
        }
        147 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000890b45); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000890b49); }
            c.w64(0, c.r64(7));
            c.w64(3, c.r64(9));
            return c.call(0x7000820354, 0x7000890b58, 0x7000890b53);
        }
        148 => {
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000890b58); };
            c.cmp(t139, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(0)) { return RegfileExit::fault(0x7000890b67); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000890b6b); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000890b74); };
            c.w64(8, t140);
            blk = 152; continue;
        }
        150 => {
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890b7d); };
            c.w64(0, t141);
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890b84); };
            c.w64(3, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890b88); };
            c.w64(6, t143);
            return c.call(0x7000870914, 0x7000890b91, 0x7000890b8c);
        }
        151 => {
            c.w64(8, c.r64(0));
            blk = 152; continue;
        }
        152 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x7000890b94); }
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890b98); };
            c.w64(0, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890b9f); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890ba3); };
            c.w64(6, t146);
            return c.call(0x7000820354, 0x7000890bac, 0x7000890ba7);
        }
        153 => {
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000890bac); };
            c.cmp(t147, c.r32(0), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x7000890bbb); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(7)) { return RegfileExit::fault(0x7000890bbf); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t148) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000890bc8); };
            c.w64(8, t148);
            blk = 157; continue;
        }
        155 => {
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890bd1); };
            c.w64(0, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890bd8); };
            c.w64(3, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890bdc); };
            c.w64(6, t151);
            return c.call(0x7000870914, 0x7000890be5, 0x7000890be0);
        }
        156 => {
            c.w64(8, c.r64(0));
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000890be8); };
            c.w64(0, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000890bec); };
            c.w64(7, t153);
            blk = 157; continue;
        }
        157 => {
            let Some(t154) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000890bf0); };
            c.cmp(t154, 0x0u64, 32);
            if c.cc_e() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000890bff); };
            c.w64(9, t155);
            let Some(t156) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000890c03); };
            c.cmp(t156, c.r32(9), 32);
            if c.cc_ne() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 166; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t157) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000890c11); };
            c.cmp(t157, 0x0u64, 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w64(7, c.r64(0));
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000890c1e); };
            c.cmp(t158, c.r32(7), 32);
            if c.cc_ne() { blk = 171; continue; }
            blk = 163; continue;
        }
        163 => {
            c.w32(1, 0x2u64);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890c30); };
            c.w64(0, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890c37); };
            c.w64(3, t160);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890c3b); };
            c.w64(6, t161);
            return c.call(0x7000820e54, 0x7000890c44, 0x7000890c3f);
        }
        164 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890c44); };
            c.w64(0, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890c4b); };
            c.w64(3, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000890c4f); };
            c.w64(1, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890c53); };
            c.w64(6, t165);
            return c.call(0x7000871cb4, 0x7000890c5c, 0x7000890c57);
        }
        165 => {
            blk = 171; continue;
        }
        166 => {
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890c5e); };
            c.w64(0, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890c65); };
            c.w64(3, t167);
            c.w64(1, c.r64(8));
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890c6c); };
            c.w64(6, t168);
            return c.call(0x7000871cb4, 0x7000890c75, 0x7000890c70);
        }
        167 => {
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890c75); };
            c.w64(6, t169);
            c.w32(1, 0x2u64);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890c7e); };
            c.w64(0, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890c85); };
            c.w64(3, t171);
            return c.call(0x7000820e54, 0x7000890c8e, 0x7000890c89);
        }
        168 => {
            blk = 171; continue;
        }
        169 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890c90); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890c97); };
            c.w64(3, t173);
            c.w64(1, c.r64(8));
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890c9e); };
            c.w64(6, t174);
            return c.call(0x7000871cb4, 0x7000890ca7, 0x7000890ca2);
        }
        170 => {
            let Some(t175) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890ca7); };
            c.w64(0, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890cae); };
            c.w64(3, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000890cb2); };
            c.w64(1, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890cb6); };
            c.w64(6, t178);
            return c.call(0x7000871cb4, 0x7000890cbf, 0x7000890cba);
        }
        171 => {
            let Some(t179) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000890cbf); };
            c.cmp(t179, 0x0u64, 32);
            if c.cc_e() { blk = 212; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t180) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000890cc9); };
            c.w64(7, t180);
            let r181 = c.add(c.r32(7), 0x2u64, 32);
            c.w32(7, r181);
            if c.cc_o() { blk = 212; continue; }
            blk = 173; continue;
        }
        173 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 174; continue;
        }
        174 => {
            let Some(t182) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000890cd9); };
            c.cmp(t182, 0x0u64, 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t183) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000890ce3); };
            c.w64(7, t183);
            let r184 = c.sub(c.r32(7), 0x2u64, 32);
            c.w32(7, r184);
            if c.cc_o() { blk = 227; continue; }
            blk = 176; continue;
        }
        176 => {
            c.w64(8, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 137; continue;
        }
        177 => {
            let Some(t185) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890cf8); };
            c.w64(8, (t185 as u32 as i32 as i64 as u64));
            let r186 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r186);
            c.w64(0, c.r64(7));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000890d06); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000890d07); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000890d0d); }
            return c.ret(0x0, 0x7000890d0f);
        }
        178 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000890d12); };
            c.w64(3, t187);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000890d21, 0x7000890d1c);
        }
        179 => {
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890d21); };
            c.w64(0, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890d25); };
            c.w64(6, t189);
            blk = 1; continue;
        }
        180 => {
            return RegfileExit::fault(0x7000890d2e);
        }
        181 => {
            blk = 31; continue;
        }
        182 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(7)) { return RegfileExit::fault(0x7000890d34); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000890d3b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000890d3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000890d43); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000890d47); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x7000890d53, 0x7000890d4e);
        }
        183 => {
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890d53); };
            c.w64(12, t190);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890d5a); };
            c.w64(7, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890d61); };
            c.w64(8, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000890d65); };
            c.w64(11, t193);
            c.w32(2, 0x8u64);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000890d6e); };
            c.w64(15, t194);
            blk = 41; continue;
        }
        184 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000890d77); };
            c.w64(3, t195);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x7000890d88, 0x7000890d83);
        }
        185 => {
            return RegfileExit::fault(0x7000890d88);
        }
        186 => {
            blk = 72; continue;
        }
        187 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(7)) { return RegfileExit::fault(0x7000890d8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000890d95); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000890d99); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x7000890d9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000890da1); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x7000890dad, 0x7000890da8);
        }
        188 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890dad); };
            c.w64(12, t196);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890db4); };
            c.w64(7, t197);
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890dbb); };
            c.w64(8, t198);
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000890dbf); };
            c.w64(11, t199);
            c.w32(2, 0x8u64);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000890dc8); };
            c.w64(15, t200);
            blk = 82; continue;
        }
        189 => {
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000890dd1); };
            c.w64(3, t201);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(9));
            return c.call(0x700086ed74, 0x7000890de2, 0x7000890ddd);
        }
        190 => {
            return RegfileExit::fault(0x7000890de2);
        }
        191 => {
            return RegfileExit::fault(0x7000890de3);
        }
        192 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a2054, 0x7000890dec, 0x7000890de7);
        }
        193 => {
            c.w64(7, c.r64(0));
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890def); };
            c.w64(0, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890df3); };
            c.w64(6, t203);
            c.w32(9, 0);
            c.logic(0, 32);
            c.w32(11, 0x1u64);
            blk = 111; continue;
        }
        194 => {
            let Some(t204) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000890e05); };
            c.w32(8, t204);
            let r205 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r205);
            blk = 129; continue;
        }
        195 => {
            let Some(t206) = c.ld32(c.r64(8).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000890e11); };
            c.cmp(t206, c.r32(12), 32);
            if c.cc_e() { blk = 128; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000890e1e); };
            c.w64(3, t207);
            c.w64(0, c.r64(7));
            return c.call(0x7000870914, 0x7000890e2d, 0x7000890e28);
        }
        197 => {
            c.w64(8, c.r64(0));
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890e30); };
            c.w64(7, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890e37); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890e3b); };
            c.w64(6, t210);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 129; continue;
        }
        198 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890e4d); }
            blk = 201; continue;
        }
        200 => {
            let r211 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r211);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 201; continue;
        }
        201 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000890e81); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 207; continue; }
            blk = 202; continue;
        }
        202 => {
            if c.cc_ne() { blk = 207; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 207; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(11, c.r64(8));
            let r212 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r212);
            if c.cc_o() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 138; continue;
        }
        207 => {
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000890eb3); };
            c.w64(8, t213);
            let Some(t214) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000890eba); };
            c.w64(11, t214);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t215) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000890ec1); };
            c.w64(15, t215);
            let Some(t216) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000890ec8); };
            c.cmp(t216, c.r64(12), 64);
            if c.cc_a() { blk = 210; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000890ed7, 0x7000890ed2);
        }
        209 => {
            c.w64(8, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000890eda); };
            c.w64(7, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000890ee1); };
            c.w64(0, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000890ee5); };
            c.w64(6, t219);
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 211; continue;
        }
        210 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x7000890eee); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 211; continue;
        }
        211 => {
            let Some(t220) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000890ef5); };
            c.w64(11, t220);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000890efc); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000890f00); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000890f05); }
            blk = 138; continue;
        }
        212 => {
            let Some(t221) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000890f10); };
            c.cmp(t221, 0x0u64, 32);
            if c.cc_ne() { blk = 214; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890f16); };
            c.w64(9, t222);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890f1a); }
            blk = 215; continue;
        }
        214 => {
            let Some(t223) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000890f22); };
            c.w64(9, t223);
            let r224 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r224);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 215; continue;
        }
        215 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(7, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 1) { return RegfileExit::fault(0x7000890f50); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 221; continue; }
            blk = 216; continue;
        }
        216 => {
            if c.cc_ne() { blk = 221; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(8, c.r64(7));
            let r225 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r225);
            if c.cc_o() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 226; continue;
        }
        221 => {
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000890f81); };
            c.w64(7, t226);
            let Some(t227) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000890f88); };
            c.w64(8, t227);
            c.w64(9, c.r64(8).wrapping_add(0xc));
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000890f8f); };
            c.w64(11, t228);
            let Some(t229) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000890f96); };
            c.cmp(t229, c.r64(9), 64);
            if c.cc_a() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000890fa5, 0x7000890fa0);
        }
        223 => {
            blk = 225; continue;
        }
        224 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x7000890fa7); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 225; continue;
        }
        225 => {
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000890fb1); };
            c.w64(7, t230);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000890fb8); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x7000890fbb); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000890fc0); }
            blk = 226; continue;
        }
        226 => {
            c.w64(9, c.r64(0));
            blk = 174; continue;
        }
        227 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000890fcd); }
            let Some(t231) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000890fd1); };
            c.cmp(t231, 0x0u64, 32);
            if c.cc_ne() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890fd7); };
            c.w64(8, t232);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000890fdb); }
            blk = 230; continue;
        }
        229 => {
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000890fe3); };
            c.w64(8, t233);
            let r234 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r234);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 230; continue;
        }
        230 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(7, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(7) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000891011); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 236; continue; }
            blk = 231; continue;
        }
        231 => {
            if c.cc_ne() { blk = 236; continue; }
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_l() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w64(8, c.r64(7));
            let r235 = c.add(c.r32(8), c.r32(7), 32);
            c.w32(8, r235);
            if c.cc_o() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 241; continue;
        }
        236 => {
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000891042); };
            c.w64(7, t236);
            let Some(t237) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000891049); };
            c.w64(8, t237);
            c.w64(11, c.r64(8).wrapping_add(0xc));
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000891050); };
            c.w64(12, t238);
            let Some(t239) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000891057); };
            c.cmp(t239, c.r64(11), 64);
            if c.cc_a() { blk = 239; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000891067, 0x7000891062);
        }
        238 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000891067); };
            c.w64(9, t240);
            blk = 240; continue;
        }
        239 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700089106d); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 240; continue;
        }
        240 => {
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000891077); };
            c.w64(7, t241);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700089107e); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000891081); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000891086); }
            blk = 241; continue;
        }
        241 => {
            c.w64(8, c.r64(0));
            blk = 137; continue;
        }
        242 => {
            c.w32(8, c.r32(8));
            let r242 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r242);
            c.w64(11, 0xfffffffffffc0000u64);
            let r243 = c.r64(11) & c.r64(8);
            c.w64(11, r243);
            c.logic(r243, 64);
            let Some(t244) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008910a3); };
            c.logic(t244 & 0x2u64, 8);
            if c.cc_e() { blk = 94; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w64(12, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008910b2); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x70008910bc, 0x70008910b7);
        }
        244 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008910bc); }
            blk = 94; continue;
        }
        245 => {
            c.w32(12, c.r32(12));
            let r245 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r245);
            c.w64(1, 0xfffffffffffc0000u64);
            let r246 = c.r64(1) & c.r64(12);
            c.w64(1, r246);
            c.logic(r246, 64);
            let Some(t247) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008910d3); };
            c.logic(t247 & 0x2u64, 8);
            if c.cc_e() { blk = 63; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w64(6, c.r64(8).wrapping_add(c.r64(3)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008910e1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008910e2); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(6));
            return c.call(0x70007f0e54, 0x70008910ef, 0x70008910ea);
        }
        247 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008910ef); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008910f1); }
            blk = 63; continue;
        }
        248 => {
            c.w32(15, c.r32(15));
            let r248 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r248);
            c.w64(12, 0xfffffffffffc0000u64);
            let r249 = c.r64(12) & c.r64(15);
            c.w64(12, r249);
            c.logic(r249, 64);
            let Some(t250) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000891107); };
            c.logic(t250 & 0x2u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 249; continue;
        }
        249 => {
            c.w64(3, c.r64(8).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000891117); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000891118); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000891125, 0x7000891120);
        }
        250 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000891125); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000891127); }
            blk = 58; continue;
        }
        251 => {
            c.w32(8, c.r32(8));
            let r251 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r251);
            c.w64(11, 0xfffffffffffc0000u64);
            let r252 = c.r64(11) & c.r64(8);
            c.w64(11, r252);
            c.logic(r252, 64);
            let Some(t253) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089113d); };
            c.logic(t253 & 0x2u64, 8);
            if c.cc_e() { blk = 53; continue; }
            blk = 252; continue;
        }
        252 => {
            c.w64(12, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700089114c); }
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000891156, 0x7000891151);
        }
        253 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000891156); }
            blk = 53; continue;
        }
        254 => {
            return RegfileExit::jump(0x7000891160);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000891174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000891174); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000891178); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000891179); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089117a); }
            let r0 = c.sub(c.r64(4), 0x68u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700089117f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000891183); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000891187); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 169; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(5));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089119b); };
            c.w64(11, t2);
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008911ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(11)) { return RegfileExit::fault(0x70008911b0); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008911be); };
            c.w32(15, t3);
            let r4 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r4);
            let Some(t5) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008911c5); };
            c.cmp(t5, 0x834u64, 16);
            if c.cc_ne() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0x1u64);
            blk = 5; continue;
        }
        4 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            c.w32(3, 0x1u64);
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008911e5); };
            c.w32(2, t6);
            c.w64(1, c.r64(2));
            let r7 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r7);
            let r8 = c.r32(1) & 0x3fu64;
            c.w32(1, r8);
            c.logic(r8, 32);
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_a() { blk = 104; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008911fc); };
            c.w32(1, t9);
            let r10 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r10);
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000891202); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            let Some(t13) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000891208); };
            c.w32(7, t13);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x4cu64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x7000891217);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000891218); };
            c.w32(1, t14);
            let Some(t15) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089121e); };
            c.cmp(t15, c.r32(1), 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700089122a); };
            c.w64(7, t16);
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000891231); };
            c.cmp(t17, c.r32(1), 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089123a); };
            c.w32(7, t18);
            c.w64(8, c.r64(7));
            let r19 = c.r32(8) & 0x8100000u64;
            c.w32(8, r19);
            c.logic(r19, 32);
            c.cmp(c.r32(8), 0x8000000u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 12; continue;
        }
        12 => {
            c.logic(c.r32(7) & 0x200000u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000891261); };
            c.w32(15, t20);
            let r21 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r21);
            let Some(t22) = c.ld16(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000891268); };
            c.w32(7, t22);
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_a() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x7000891273);
        }
        15 => {
            let Some(t23) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000891274); };
            c.w32(15, t23);
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 105; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t24) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000891282); };
            c.w32(12, t24);
            let r25 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r25);
            let Some(t26) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000891289); };
            c.w32(15, t26);
            let Some(t27) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700089128e); };
            c.cmp(t27, c.r32(15), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t28) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089129b); };
            c.w32(15, t28);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t29) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008912a9); };
            c.w32(7, t29);
            c.cmp(c.r32(15), c.r32(7), 32);
            if c.cc_a() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(15));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_ae() { blk = 23; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008912c2); };
            c.w64(2, t30);
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008912c9); };
            c.w32(2, t31);
            let r32 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r32);
            return c.call(0x7000824234, 0x70008912d7, 0x70008912d2);
        }
        21 => {
            return RegfileExit::fault(0x70008912d7);
        }
        22 => {
            return RegfileExit::fault(0x70008912d8);
        }
        23 => {
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 38; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 173; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(1, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(7));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w32(8, c.r32(8));
            let r33 = c.imul2(c.r32(8), 0x4u64, 32);
            c.w32(8, r33);
            if c.cc_o() { blk = 37; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w32(6, 0x8u64);
            let r34 = c.add(c.r64(8), c.r64(6), 64);
            c.w64(8, r34);
            if c.cc_o() { blk = 36; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089131a); };
            c.w64(9, t35);
            let Some(t36) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000891321); };
            c.w64(9, t36);
            c.w64(3, c.r64(9).wrapping_add(c.r64(8)));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000891328); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700089132f); };
            c.cmp(t38, c.r64(3), 64);
            if c.cc_be() { blk = 171; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(8), 0x20000u64, 64);
            if c.cc_ae() { blk = 171; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000891345); };
            c.w64(0, t39);
            if !c.st64(c.r64(0), c.r64(3)) { return RegfileExit::fault(0x700089134c); }
            let r40 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r40);
            blk = 30; continue;
        }
        30 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000891353); };
            c.w64(0, t41);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700089135a); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x700089135e); }
            c.w64(0, c.r64(7).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(6));
            blk = 32; continue;
        }
        31 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700089136f); }
            let r42 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r42);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r64(0), c.r64(6), 64);
            if c.cc_e() { blk = 39; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(1, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_e() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t43) = c.ld32(c.r64(12).wrapping_add(c.r64(3)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089138a); };
            c.w32(1, t43);
            let r44 = c.add(c.r64(3), 0x4u64, 64);
            c.w64(3, r44);
            blk = 31; continue;
        }
        35 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000891395); };
            c.w64(1, t45);
            blk = 31; continue;
        }
        36 => {
            return RegfileExit::fault(0x700089139e);
        }
        37 => {
            return RegfileExit::fault(0x700089139f);
        }
        38 => {
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008913a0); };
            c.w64(9, t46);
            c.w64(12, c.r64(9));
            blk = 39; continue;
        }
        39 => {
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008913aa); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(12, 0xfffffffffffc0000u64);
            let r47 = c.r64(12) & c.r64(11);
            c.w64(12, r47);
            c.logic(r47, 64);
            let Some(t48) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008913be); };
            c.logic(t48 & 0x4u64, 8);
            if c.cc_ne() { blk = 248; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 43; continue;
        }
        42 => {
            return RegfileExit::fault(0x70008913cc);
        }
        43 => {
            let Some(t49) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008913cd); };
            c.w32(9, t49);
            c.w32(12, c.r32(9));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 44; continue;
        }
        44 => {
            let r50 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r50);
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008913e2); };
            c.w32(12, t51);
            let r52 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r52);
            let Some(t53) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008913e9); };
            c.w32(15, t53);
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)).wrapping_add(0x20));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008913f3); }
            c.cmp(c.r32(0), c.r32(15), 32);
            if c.cc_l() { blk = 101; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0xc8u64, 32);
            if c.cc_g() { blk = 101; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(0, c.r8(2));
            let r54 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r54);
            let r55 = c.r32(0) & 0x3fu64;
            c.w32(0, r55);
            c.logic(r55, 32);
            c.w64(3, c.r64(0));
            let r56 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r56);
            c.cmp(c.r32(3), 0x2u64, 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t57) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000891429); };
            c.w32(2, t57);
            let r58 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r58);
            let Some(t59) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000891431); };
            c.w32(2, t59);
            let r60 = c.sub(c.r32(2), 0x77u64, 32);
            c.w32(2, r60);
            c.cmp(c.r32(2), 0xcu64, 32);
            if c.cc_a() { blk = 52; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 50; continue; }
            blk = 49; continue;
        }
        49 => {
            return RegfileExit::fault(0x7000891449);
        }
        50 => {
            let Some(t61) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089144a); };
            c.w32(12, t61);
            let r62 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r62);
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000891452); };
            c.cmp(t63, c.r32(12), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 61; continue;
        }
        52 => {
            return RegfileExit::fault(0x7000891464);
        }
        53 => {
            let Some(t64) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000891465); };
            c.w32(2, t64);
            let Some(t65) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700089146a); };
            c.cmp(t65, c.r32(2), 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_a() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            return RegfileExit::fault(0x7000891483);
        }
        56 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x7)) { return RegfileExit::fault(0x7000891484); }
            c.w64(15, 0xfff7fffffff7ffffu64);
            let Some(t66) = c.ld64(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000891495); };
            c.cmp(t66, c.r64(15), 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089149c); };
            c.w64(12, t67);
            let Some(t68) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008914a3); };
            c.w64(15, t68);
            c.w64(2, c.r64(15).wrapping_add(0xc));
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008914ab); };
            c.w64(1, t69);
            let Some(t70) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008914b2); };
            c.cmp(t70, c.r64(2), 64);
            if c.cc_be() { blk = 175; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(12), c.r64(2)) { return RegfileExit::fault(0x70008914bb); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 59; continue;
        }
        59 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008914c3); };
            c.w64(15, t71);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008914ca); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008914cf); }
            blk = 61; continue;
        }
        60 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008914d8); };
            c.w64(12, t72);
            blk = 61; continue;
        }
        61 => {
            c.w64(15, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(15)) { return RegfileExit::fault(0x70008914e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008914e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008914ea); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t73) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008914f7); };
            c.w32(3, t73);
            let r74 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r74);
            let Some(t75) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008914fe); };
            c.w32(2, t75);
            let r76 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r76);
            let Some(t77) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000891504); };
            c.w32(2, t77);
            let r78 = c.sub(c.r32(2), 0x77u64, 32);
            c.w32(2, r78);
            c.cmp(c.r32(2), 0xcu64, 32);
            if c.cc_a() { blk = 83; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_le() { blk = 73; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(0, c.r64(3));
            let r79 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r79);
            c.logic(r79, 64);
            let Some(t80) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000891526); };
            c.logic(t80 & 0x4u64, 8);
            if c.cc_e() { blk = 71; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r81 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r81);
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_e() { blk = 82; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(2, 0xbu64);
            blk = 67; continue;
        }
        67 => {
            let Some(t82) = c.ld32(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x700089154d); };
            c.w32(1, t82);
            c.w64(6, c.r64(2).wrapping_sub(0x4));
            if !c.st32(c.r64(3).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x7000891554); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r83 = c.r64(7) & c.r64(3);
            c.w64(7, r83);
            c.logic(r83, 64);
            let Some(t84) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000891566); };
            c.logic(t84 & 0x4u64, 8);
            if c.cc_ne() { blk = 245; continue; }
            blk = 69; continue;
        }
        69 => {
            let r85 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r85);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 67; continue; }
            blk = 70; continue;
        }
        70 => {
            blk = 82; continue;
        }
        71 => {
            c.w64(8, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(3).wrapping_add(0xb));
            c.w64(1, c.r64(3).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r86 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r86);
            let r87 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r87);
            c.logic(r87, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000891595); }
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x700089159a); };
            c.w64(0, t88);
            c.w64(10, 0x70008915a8u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008915a8); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008915ac); }
            return c.call(c.r64(0), 0x70008915b2, 0x70008915b0);
        }
        72 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008915b2); }
            let Some(t89) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008915ba); };
            c.w64(4, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008915bf); };
            c.w64(11, t90);
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008915c3); };
            c.w64(12, t91);
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008915c7); };
            c.w64(15, t92);
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008915cb); };
            c.w64(9, t93);
            blk = 82; continue;
        }
        73 => {
            c.w64(0, c.r64(3));
            let r94 = c.r64(0) & 0xfffffffffffc0000u64;
            c.w64(0, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008915dd); };
            c.logic(t95 & 0x4u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            let r96 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r96);
            c.w64(0, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(0), 0xbu64, 64);
            if c.cc_e() { blk = 82; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(2, 0xbu64);
            blk = 76; continue;
        }
        76 => {
            let Some(t97) = c.ld32(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x7000891604); };
            c.w32(1, t97);
            c.w64(6, c.r64(2).wrapping_sub(0x4));
            if !c.st32(c.r64(3).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x700089160b); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r98 = c.r64(7) & c.r64(3);
            c.w64(7, r98);
            c.logic(r98, 64);
            let Some(t99) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089161d); };
            c.logic(t99 & 0x4u64, 8);
            if c.cc_ne() { blk = 242; continue; }
            blk = 78; continue;
        }
        78 => {
            let r100 = c.add(c.r64(2), 0x4u64, 64);
            c.w64(2, r100);
            c.cmp(c.r64(0), c.r64(2), 64);
            if c.cc_ne() { blk = 76; continue; }
            blk = 79; continue;
        }
        79 => {
            blk = 82; continue;
        }
        80 => {
            c.w64(8, c.r64(15).wrapping_add(c.r64(15)));
            c.w64(2, c.r64(3).wrapping_add(0xb));
            c.w64(1, c.r64(3).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r101 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r101);
            let r102 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r102);
            c.logic(r102, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000891649); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x700089164e); };
            c.w64(0, t103);
            c.w64(10, 0x700089165cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700089165c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000891660); }
            return c.call(c.r64(0), 0x7000891666, 0x7000891664);
        }
        81 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000891666); }
            let Some(t104) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089166e); };
            c.w64(4, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891673); };
            c.w64(11, t105);
            let Some(t106) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891677); };
            c.w64(12, t106);
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089167b); };
            c.w64(15, t107);
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089167f); };
            c.w64(9, t108);
            blk = 82; continue;
        }
        82 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 87; continue;
        }
        83 => {
            return RegfileExit::fault(0x7000891687);
        }
        84 => {
            let Some(t109) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000891688); };
            c.w32(0, t109);
            let r110 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r110);
            let Some(t111) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089168f); };
            c.w32(3, t111);
            let Some(t112) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000891692); };
            c.cmp(t112, c.r32(3), 32);
            if c.cc_ne() { blk = 99; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(8, c.r64(15).wrapping_mul(4));
            c.w64(2, c.r64(0).wrapping_add(0xf));
            c.w64(1, c.r64(0).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r113 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r113);
            let r114 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r114);
            c.logic(r114, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008916ba); }
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x13d0)) else { return RegfileExit::fault(0x70008916bf); };
            c.w64(0, t115);
            c.w64(10, 0x70008916cdu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008916cd); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008916d1); }
            return c.call(c.r64(0), 0x70008916d7, 0x70008916d5);
        }
        86 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008916d7); }
            let Some(t116) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008916df); };
            c.w64(4, t116);
            c.w32(3, 0x1u64);
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008916e9); };
            c.w64(11, t117);
            let Some(t118) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008916ed); };
            c.w64(12, t118);
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008916f1); };
            c.w64(15, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008916f5); };
            c.w64(9, t120);
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 93; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w64(0, c.r64(11));
            let Some(t121) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000891701); };
            c.w32(11, t121);
            let r122 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r122);
            let Some(t123) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000891708); };
            c.w32(0, t123);
            let r124 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r124);
            let Some(t125) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089170f); };
            c.w32(0, t125);
            let r126 = c.sub(c.r32(0), 0x77u64, 32);
            c.w32(0, r126);
            c.cmp(c.r32(0), 0xcu64, 32);
            if c.cc_a() { blk = 92; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t127) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089171b); };
            c.w32(0, t127);
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_b() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            return RegfileExit::fault(0x7000891724);
        }
        91 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000891725); };
            c.w64(9, t128);
            if !c.st32(c.r64(11).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700089172c); }
            blk = 97; continue;
        }
        92 => {
            return RegfileExit::fault(0x7000891733);
        }
        93 => {
            let Some(t129) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000891734); };
            c.w32(11, t129);
            let r130 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r130);
            let Some(t131) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089173b); };
            c.w32(0, t131);
            let Some(t132) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700089173f); };
            c.cmp(t132, c.r32(0), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t133) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000891748); };
            c.w32(0, t133);
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_b() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            return RegfileExit::fault(0x7000891751);
        }
        96 => {
            c.w64(9, c.r64(15).wrapping_mul(4));
            let Some(t134) = c.ld64(0x700089148du64) else { return RegfileExit::fault(0x700089175a); };
            c.w64(15, t134);
            if !c.st64(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x7), c.r64(15)) { return RegfileExit::fault(0x7000891761); }
            blk = 97; continue;
        }
        97 => {
            let Some(t135) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000891766); };
            c.w64(9, (t135 as u32 as i32 as i64 as u64));
            let r136 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r136);
            c.w64(0, c.r64(12));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000891774); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000891775); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x700089177b); }
            return c.ret(0x0, 0x700089177d);
        }
        98 => {
            return RegfileExit::fault(0x700089177e);
        }
        99 => {
            return RegfileExit::fault(0x700089177f);
        }
        100 => {
            return RegfileExit::fault(0x7000891780);
        }
        101 => {
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000891781); };
            c.w64(9, t137);
            let Some(t138) = c.ld64(c.r64(9).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000891785); };
            c.w64(9, t138);
            let Some(t139) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000891789); };
            c.w64(5, t139);
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089178d); };
            c.w64(2, t140);
            c.w64(7, c.r64(9));
            let Some(t141) = c.ld64(c.r64(4).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000891797); };
            c.w64(0, t141);
            let Some(t142) = c.ld64(c.r64(4).wrapping_add(0x58)) else { return RegfileExit::fault(0x700089179c); };
            c.w64(6, t142);
            let r143 = c.add(c.r64(4), 0x88u64, 64);
            c.w64(4, r143);
            return RegfileExit::jump(0x7000827674);
        }
        102 => {
            let Some(t144) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008917ad); };
            c.w64(9, (t144 as u32 as i32 as i64 as u64));
            let r145 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r145);
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008917b5); };
            c.w64(0, t146);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008917bf); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008917c0); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008917c6); }
            return c.ret(0x0, 0x70008917c8);
        }
        103 => {
            c.w32(3, 0x1u64);
            blk = 104; continue;
        }
        104 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 177; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t147) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008917da); };
            c.w32(9, t147);
            let r148 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r148);
            let Some(t149) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008917e1); };
            c.cmp(t149, 0xbau64, 16);
            if c.cc_b() { blk = 177; continue; }
            blk = 107; continue;
        }
        107 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(11)) { return RegfileExit::fault(0x70008917fa); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t150) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000891804); };
            c.w32(12, t150);
            let r151 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r151);
            let Some(t152) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089180b); };
            c.cmp(t152, 0x834u64, 16);
            if c.cc_ne() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t153) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000891815); };
            c.w32(9, t153);
            let r154 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r154);
            blk = 130; continue;
        }
        110 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t155) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000891828); };
            c.w32(12, t155);
            let r156 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r156);
            let Some(t157) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089182f); };
            c.w32(12, t157);
            let r158 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r158);
            let Some(t159) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000891837); };
            c.w32(15, t159);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t160) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089183f); };
            c.w32(3, t160);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xb0u64, 64);
            if c.cc_a() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            return RegfileExit::fault(0x700089184f);
        }
        114 => {
            let Some(t161) = c.ld32(c.r64(12).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x7000891850); };
            c.cmp(t161, c.r32(3), 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 181; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t162) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000891868); };
            c.w32(9, t162);
            let r163 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r163);
            let Some(t164) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089186f); };
            c.w32(9, t164);
            let r165 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r165);
            let Some(t166) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000891876); };
            c.w32(12, t166);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t167) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089187d); };
            c.w32(15, t167);
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), 0x44u64, 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            return RegfileExit::fault(0x700089188a);
        }
        118 => {
            let Some(t168) = c.ld32(c.r64(9).wrapping_add(0x117)) else { return RegfileExit::fault(0x700089188b); };
            c.cmp(t168, c.r32(15), 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r64(12), 0xaeu64, 64);
            if c.cc_a() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            return RegfileExit::fault(0x700089189d);
        }
        121 => {
            let Some(t169) = c.ld32(c.r64(9).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700089189e); };
            c.cmp(t169, c.r32(15), 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r64(12), 0xafu64, 64);
            if c.cc_a() { blk = 180; continue; }
            blk = 123; continue;
        }
        123 => {
            return RegfileExit::fault(0x70008918b4);
        }
        124 => {
            let Some(t170) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008918b5); };
            c.w32(9, t170);
            let r171 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r171);
            blk = 125; continue;
        }
        125 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 128; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(0, c.r64(9));
            return c.call(0x70008a17f4, 0x70008918ca, 0x70008918c5);
        }
        127 => {
            c.w64(9, c.r64(0));
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008918cd); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008918d1); };
            c.w64(11, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008918d5); };
            c.w64(6, t174);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 130; continue;
        }
        128 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ge() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(9, c.r64(1));
            blk = 130; continue;
        }
        130 => {
            c.w32(12, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008918f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008918f9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x70008918fd); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 134; continue; }
            blk = 131; continue;
        }
        131 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000891907); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 135; continue; }
            blk = 132; continue;
        }
        132 => {
            if c.cc_e() { blk = 167; continue; }
            blk = 133; continue;
        }
        133 => {
            blk = 135; continue;
        }
        134 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(3, c.r64(9));
            let r175 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r175);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000891936); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(1));
            return c.call(0x7000870914, 0x7000891946, 0x7000891941);
        }
        136 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000891946); }
            c.w32(1, 0x2u64);
            c.w64(9, c.r64(1));
            blk = 137; continue;
        }
        137 => {
            c.w32(12, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089196f); };
            c.w64(11, t176);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(12)) { return RegfileExit::fault(0x7000891973); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x7000891977); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 141; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000891981); }
            let Some(t177) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000891987); };
            c.cmp(t177, 0x0u64, 32);
            if c.cc_ne() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.movsd_rm(2, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700089198d); }
            blk = 143; continue;
        }
        140 => {
            if !c.movsd_rm(2, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000891995); }
            blk = 143; continue;
        }
        141 => {
            let Some(t178) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089199c); };
            c.cmp(t178, 0x0u64, 32);
            if c.cc_ne() { blk = 145; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(3, c.r64(9));
            let r179 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r179);
            if !c.movsd_rm(2, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008919a7); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 143; continue;
        }
        143 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_a() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 158; continue;
        }
        145 => {
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ge() { blk = 158; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008919d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(3)) { return RegfileExit::fault(0x70008919d7); }
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 183; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(2, c.r64(12));
            let r180 = c.sub(c.r32(2), 0x2u64, 32);
            c.w32(2, r180);
            if c.cc_o() { blk = 183; continue; }
            blk = 148; continue;
        }
        148 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 149; continue;
        }
        149 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x70008919f3); }
            let Some(t181) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008919f7); };
            c.w64(0, t181);
            c.w64(3, c.r64(9));
            let Some(t182) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008919fe); };
            c.w64(6, t182);
            return c.call(0x7000820354, 0x7000891a07, 0x7000891a02);
        }
        150 => {
            let Some(t183) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000891a07); };
            c.cmp(t183, c.r32(0), 32);
            if c.cc_e() { blk = 153; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891a10); };
            c.w64(0, t184);
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000891a14); };
            c.w64(3, t185);
            c.w32(1, 0x2u64);
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891a1d); };
            c.w64(6, t186);
            return c.call(0x7000820e54, 0x7000891a26, 0x7000891a21);
        }
        152 => {
            blk = 155; continue;
        }
        153 => {
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891a28); };
            c.w64(0, t187);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000891a2c); };
            c.w64(3, t188);
            let Some(t189) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891a30); };
            c.w64(6, t189);
            return c.call(0x7000870914, 0x7000891a39, 0x7000891a34);
        }
        154 => {
            c.w64(1, c.r64(0));
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891a3c); };
            c.w64(0, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000891a40); };
            c.w64(3, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891a44); };
            c.w64(6, t192);
            return c.call(0x7000871cb4, 0x7000891a4d, 0x7000891a48);
        }
        155 => {
            let Some(t193) = c.ld32(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000891a4d); };
            c.cmp(t193, 0x0u64, 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000891a57); };
            c.w64(9, t194);
            let r195 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r195);
            if c.cc_o() { blk = 197; continue; }
            blk = 157; continue;
        }
        157 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 137; continue;
        }
        158 => {
            let Some(t196) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000891a6d); };
            c.cmp(t196, 0x0u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x7000891a79); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 212; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000891a87); };
            c.w64(12, t197);
            let r198 = c.sub(c.r32(12), 0x2u64, 32);
            c.w32(12, r198);
            if c.cc_o() { blk = 212; continue; }
            blk = 160; continue;
        }
        160 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 161; continue;
        }
        161 => {
            let Some(t199) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891a98); };
            c.w64(0, t199);
            c.w64(3, c.r64(12));
            c.w32(1, 0x2u64);
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891aa4); };
            c.w64(6, t200);
            return c.call(0x7000820e54, 0x7000891aad, 0x7000891aa8);
        }
        162 => {
            let Some(t201) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000891aad); };
            c.cmp(t201, 0x0u64, 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000891ab7); };
            c.w64(9, t202);
            let r203 = c.sub(c.r32(9), 0x2u64, 32);
            c.w32(9, r203);
            if c.cc_o() { blk = 227; continue; }
            blk = 164; continue;
        }
        164 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 165; continue;
        }
        165 => {
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000891ac8); };
            c.w64(3, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891acf); };
            c.w64(0, t205);
            c.w64(1, c.r64(9));
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891ad6); };
            c.w64(6, t206);
            return c.call(0x7000871cb4, 0x7000891adf, 0x7000891ada);
        }
        166 => {
            let Some(t207) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000891adf); };
            c.w64(9, (t207 as u32 as i32 as i64 as u64));
            let r208 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000891ae7); };
            c.w64(0, t209);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000891aee); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000891aef); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000891af5); }
            return c.ret(0x0, 0x7000891af7);
        }
        167 => {
            let Some(t210) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000891af8); };
            c.w64(3, t210);
            c.w64(0, c.r64(11));
            return c.call(0x7000871cb4, 0x7000891b07, 0x7000891b02);
        }
        168 => {
            let Some(t211) = c.ld32(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000891b07); };
            c.w64(9, (t211 as u32 as i32 as i64 as u64));
            let r212 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r212);
            let Some(t213) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000891b0f); };
            c.w64(0, t213);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000891b19); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000891b1a); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000891b20); }
            return c.ret(0x0, 0x7000891b22);
        }
        169 => {
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000891b25); };
            c.w64(3, t214);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000891b34, 0x7000891b2f);
        }
        170 => {
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000891b34); };
            c.w64(0, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891b38); };
            c.w64(6, t216);
            blk = 1; continue;
        }
        171 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x7000891b41); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(12)) { return RegfileExit::fault(0x7000891b45); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000891b49); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000891b4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x7000891b51); }
            c.w64(2, c.r64(8));
            return c.call(0x70007febb4, 0x7000891b5d, 0x7000891b58);
        }
        172 => {
            c.w64(9, c.r64(0));
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891b60); };
            c.w64(11, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000891b64); };
            c.w64(2, t218);
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000891b68); };
            c.w64(12, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000891b6c); };
            c.w64(15, t220);
            c.w32(6, 0x8u64);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000891b75); };
            c.w64(7, t221);
            let Some(t222) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000891b79); };
            c.w64(1, t222);
            blk = 30; continue;
        }
        173 => {
            let Some(t223) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000891b82); };
            c.w64(3, t223);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000891b93, 0x7000891b8e);
        }
        174 => {
            return RegfileExit::fault(0x7000891b93);
        }
        175 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(0)) { return RegfileExit::fault(0x7000891b99); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x7000891b9d); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x7000891ba1); }
            return c.call(0x70007febf4, 0x7000891bab, 0x7000891ba6);
        }
        176 => {
            c.w64(12, c.r64(0));
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891bae); };
            c.w64(11, t224);
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000891bb2); };
            c.w64(9, t225);
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000891bb6); };
            c.w64(0, t226);
            let Some(t227) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000891bba); };
            c.w64(3, t227);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x7000891bbe); }
            blk = 59; continue;
        }
        177 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x7000891bd0, 0x7000891bcb);
        }
        178 => {
            c.w64(11, c.r64(0));
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000891bd3); };
            c.w64(0, t228);
            let Some(t229) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891bd7); };
            c.w64(6, t229);
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(3, 0x1u64);
            blk = 107; continue;
        }
        179 => {
            let Some(t230) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000891be7); };
            c.w32(9, t230);
            let r231 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r231);
            blk = 125; continue;
        }
        180 => {
            let Some(t232) = c.ld32(c.r64(9).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000891bf3); };
            c.cmp(t232, c.r32(15), 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000891c00); };
            c.w64(3, t233);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x7000891c0f, 0x7000891c0a);
        }
        182 => {
            c.w64(9, c.r64(0));
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000891c12); };
            c.w64(0, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000891c16); };
            c.w64(11, t235);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000891c1a); };
            c.w64(6, t236);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 125; continue;
        }
        183 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            if !c.movsd_rm(1, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000891c2b); }
            blk = 186; continue;
        }
        185 => {
            c.w64(2, c.r64(9));
            let r237 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r237);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 186; continue;
        }
        186 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(2, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(2) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x78), 1) { return RegfileExit::fault(0x7000891c50); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 192; continue; }
            blk = 187; continue;
        }
        187 => {
            if c.cc_ne() { blk = 192; continue; }
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 190; continue; }
            blk = 189; continue;
        }
        189 => {
            c.movq_rx(6, 1);
            if let Some(sr) = c.shr(c.r64(6), 0x20u64, 64) { c.w64(6, sr); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_l() { blk = 192; continue; }
            blk = 190; continue;
        }
        190 => {
            c.w64(6, c.r64(2));
            let r238 = c.add(c.r32(6), c.r32(2), 32);
            c.w32(6, r238);
            if c.cc_o() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 149; continue;
        }
        192 => {
            let Some(t239) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000891c7f); };
            c.w64(2, t239);
            let Some(t240) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x7000891c86); };
            c.w64(6, t240);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            let Some(t241) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000891c8d); };
            c.w64(8, t241);
            let Some(t242) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000891c94); };
            c.cmp(t242, c.r64(7), 64);
            if c.cc_a() { blk = 195; continue; }
            blk = 193; continue;
        }
        193 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000891ca3, 0x7000891c9e);
        }
        194 => {
            c.w64(2, c.r64(0));
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000891ca6); };
            c.w64(11, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000891caa); };
            c.w64(9, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000891cae); };
            c.w64(15, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000891cb2); };
            c.w64(12, t246);
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000891cb6); };
            c.w64(3, t247);
            blk = 196; continue;
        }
        195 => {
            if !c.st64(c.r64(2), c.r64(7)) { return RegfileExit::fault(0x7000891cbc); }
            c.w64(2, c.r64(6).wrapping_add(0x1));
            blk = 196; continue;
        }
        196 => {
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000891cc3); };
            c.w64(6, t248);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000891cca); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x78)) { return RegfileExit::fault(0x7000891ccd); }
            if !c.movsd_mr(c.r64(2).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000891cd2); }
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            blk = 149; continue;
        }
        197 => {
            let Some(t249) = c.ld32(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000891cea); };
            c.cmp(t249, 0x0u64, 32);
            if c.cc_ne() { blk = 199; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000891cf0); };
            c.w64(9, t250);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000891cf4); }
            blk = 200; continue;
        }
        199 => {
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000891cfc); };
            c.w64(9, t251);
            let r252 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r252);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 200; continue;
        }
        200 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000891d2c); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_p() { blk = 206; continue; }
            blk = 201; continue;
        }
        201 => {
            if c.cc_ne() { blk = 206; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 206; continue; }
            blk = 204; continue;
        }
        204 => {
            c.w64(11, c.r64(9));
            let r253 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r253);
            if c.cc_o() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 211; continue;
        }
        206 => {
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000891d5e); };
            c.w64(9, t254);
            let Some(t255) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000891d65); };
            c.w64(11, t255);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000891d6c); };
            c.w64(15, t256);
            let Some(t257) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000891d73); };
            c.cmp(t257, c.r64(12), 64);
            if c.cc_a() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000891d82, 0x7000891d7d);
        }
        208 => {
            blk = 210; continue;
        }
        209 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000891d84); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 210; continue;
        }
        210 => {
            let Some(t258) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000891d8e); };
            c.w64(9, t258);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000891d95); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000891d99); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000891d9e); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            blk = 211; continue;
        }
        211 => {
            c.w64(9, c.r64(0));
            blk = 137; continue;
        }
        212 => {
            let Some(t259) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000891db9); };
            c.cmp(t259, 0x0u64, 32);
            if c.cc_ne() { blk = 214; continue; }
            blk = 213; continue;
        }
        213 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000891dbf); }
            blk = 215; continue;
        }
        214 => {
            if !c.movsd_rm(1, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000891dc7); }
            blk = 215; continue;
        }
        215 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(12, cv);
            c.xorpd_rr(2, 2);
            c.cvtsi2sd(2, (c.r32(12) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 1) { return RegfileExit::fault(0x7000891dde); }
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(1));
            if c.cc_p() { blk = 221; continue; }
            blk = 216; continue;
        }
        216 => {
            if c.cc_ne() { blk = 221; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            c.movq_rx(15, 1);
            if let Some(sr) = c.shr(c.r64(15), 0x20u64, 64) { c.w64(15, sr); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_l() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(15, c.r64(12));
            let r260 = c.add(c.r32(15), c.r32(12), 32);
            c.w32(15, r260);
            if c.cc_o() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            c.w64(12, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(12));
            blk = 226; continue;
        }
        221 => {
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000891e10); };
            c.w64(12, t261);
            let Some(t262) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000891e17); };
            c.w64(15, t262);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000891e1f); };
            c.w64(3, t263);
            let Some(t264) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000891e26); };
            c.cmp(t264, c.r64(0), 64);
            if c.cc_a() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000891e35, 0x7000891e30);
        }
        223 => {
            blk = 225; continue;
        }
        224 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000891e37); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(12));
            blk = 225; continue;
        }
        225 => {
            let Some(t265) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000891e42); };
            c.w64(9, t265);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000891e49); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000891e4d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000891e52); }
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000891e57); };
            c.w64(9, t266);
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000891e69); };
            c.w64(11, t267);
            blk = 226; continue;
        }
        226 => {
            c.w64(12, c.r64(0));
            blk = 161; continue;
        }
        227 => {
            let Some(t268) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000891e75); };
            c.cmp(t268, 0x0u64, 32);
            if c.cc_ne() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000891e7b); };
            c.w64(9, t269);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000891e7f); }
            blk = 230; continue;
        }
        229 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000891e87); }
            blk = 230; continue;
        }
        230 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x38), 0) { return RegfileExit::fault(0x7000891eac); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 236; continue; }
            blk = 231; continue;
        }
        231 => {
            if c.cc_ne() { blk = 236; continue; }
            blk = 232; continue;
        }
        232 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            c.movq_rx(11, 0);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w64(11, c.r64(9));
            let r270 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r270);
            if c.cc_o() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 241; continue;
        }
        236 => {
            let Some(t271) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000891ede); };
            c.w64(9, t271);
            let Some(t272) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000891ee5); };
            c.w64(11, t272);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t273) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000891eec); };
            c.w64(15, t273);
            let Some(t274) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000891ef3); };
            c.cmp(t274, c.r64(12), 64);
            if c.cc_a() { blk = 239; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000891f02, 0x7000891efd);
        }
        238 => {
            blk = 240; continue;
        }
        239 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000891f04); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 240; continue;
        }
        240 => {
            let Some(t275) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000891f0e); };
            c.w64(9, t275);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000891f15); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x38)) { return RegfileExit::fault(0x7000891f19); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000891f1e); }
            blk = 241; continue;
        }
        241 => {
            c.w64(9, c.r64(0));
            blk = 165; continue;
        }
        242 => {
            c.w32(1, c.r32(1));
            let r276 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r276);
            c.w64(7, 0xfffffffffffc0000u64);
            let r277 = c.r64(7) & c.r64(1);
            c.w64(7, r277);
            c.logic(r277, 64);
            let Some(t278) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000891f3a); };
            c.logic(t278 & 0x2u64, 8);
            if c.cc_e() { blk = 78; continue; }
            blk = 243; continue;
        }
        243 => {
            c.w64(8, c.r64(3).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000891f48); }
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x7000891f51, 0x7000891f4c);
        }
        244 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000891f51); }
            blk = 78; continue;
        }
        245 => {
            c.w32(1, c.r32(1));
            let r279 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r279);
            c.w64(7, 0xfffffffffffc0000u64);
            let r280 = c.r64(7) & c.r64(1);
            c.w64(7, r280);
            c.logic(r280, 64);
            let Some(t281) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000891f66); };
            c.logic(t281 & 0x2u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w64(8, c.r64(3).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000891f74); }
            c.w64(7, c.r64(3));
            return c.call(0x70007f0e54, 0x7000891f7d, 0x7000891f78);
        }
        247 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000891f7d); }
            blk = 69; continue;
        }
        248 => {
            c.w32(9, c.r32(9));
            let r282 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r282);
            c.w64(12, 0xfffffffffffc0000u64);
            let r283 = c.r64(12) & c.r64(9);
            c.w64(12, r283);
            c.logic(r283, 64);
            let Some(t284) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000891f93); };
            c.logic(t284 & 0x2u64, 8);
            if c.cc_e() { blk = 41; continue; }
            blk = 249; continue;
        }
        249 => {
            c.w64(15, c.r64(11).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000891fa3); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000891fa4); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000891fb1, 0x7000891fac);
        }
        250 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000891fb1); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000891fb3); }
            blk = 41; continue;
        }
        251 => {
            return RegfileExit::jump(0x7000891fbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000891fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000891fd4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000891fd8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000891fd9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000891fda); }
            let r0 = c.sub(c.r64(4), 0x58u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000891fdf); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x7000891fe3); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 301; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(9, c.r64(5));
            c.w64(11, c.r64(9).wrapping_add(0x18));
            c.w64(12, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000891ffb); };
            c.w64(12, t2);
            c.w64(15, (c.r32(0) as u32 as i32 as i64 as u64));
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700089200d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000892011); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x7000892015); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            blk = 18; continue;
        }
        3 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(3, 0);
            c.logic(0, 32);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xfc0)) else { return RegfileExit::fault(0x7000892036); };
            c.w64(2, t3);
            let Some(t4) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089203d); };
            c.cmp(t4, c.r32(3), 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892046); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089204e); };
            c.cmp(t7, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(7, 0x1u64);
            blk = 8; continue;
        }
        7 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        8 => {
            c.w32(1, 0x1u64);
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t8) = c.ld8(c.r64(2).wrapping_add(0xa)) else { return RegfileExit::fault(0x700089206d); };
            c.w32(7, t8);
            let r9 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r9);
            let r10 = c.r32(7) & 0x3fu64;
            c.w32(7, r10);
            c.logic(r10, 32);
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 18; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t11) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089207c); };
            c.w32(7, t11);
            let r12 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r12);
            let Some(t13) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000892082); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892088); };
            c.w32(8, t15);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x7000892098);
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x7000892099); };
            c.w32(7, t16);
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700089209f); };
            c.cmp(t17, c.r32(7), 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008920a4); };
            c.w64(2, t18);
            let Some(t19) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008920ab); };
            c.cmp(t19, c.r32(3), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008276b4, 0x70008920b8, 0x70008920b3);
        }
        15 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008920b8); };
            c.w64(15, t20);
            c.w64(9, c.r64(15).wrapping_add(0x1));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008920c3); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008920c4); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008920ca); }
            return c.ret(0x0, 0x70008920cc);
        }
        16 => {
            c.w32(1, 0x1u64);
            blk = 18; continue;
        }
        17 => {
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t21) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008920e4); };
            c.w32(0, t21);
            let r22 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r22);
            let Some(t23) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008920ec); };
            c.cmp(t23, 0xbau64, 16);
            if c.cc_b() { blk = 303; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000892102); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000892106); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t24) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089210f); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892117); };
            c.cmp(t26, 0x834u64, 16);
            if c.cc_ne() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089211f); };
            c.w32(2, t27);
            let r28 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r28);
            blk = 44; continue;
        }
        23 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 25; continue;
        }
        24 => {
            c.w64(2, c.r64(1));
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ne() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892138); };
            c.w32(2, t29);
            let r30 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r30);
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089213e); };
            c.w32(2, t31);
            let r32 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r32);
            let Some(t33) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892144); };
            c.w32(7, t33);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t34) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089214a); };
            c.w32(8, t34);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xb0u64, 64);
            if c.cc_a() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            return RegfileExit::fault(0x700089215b);
        }
        28 => {
            let Some(t35) = c.ld32(c.r64(2).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700089215c); };
            c.cmp(t35, c.r32(8), 32);
            if c.cc_e() { blk = 305; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 307; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t36) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892172); };
            c.w32(2, t36);
            let r37 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r37);
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000892178); };
            c.w32(2, t38);
            let r39 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r39);
            let Some(t40) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089217e); };
            c.w32(7, t40);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t41) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892184); };
            c.w32(8, t41);
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x44u64, 64);
            if c.cc_a() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            return RegfileExit::fault(0x7000892192);
        }
        32 => {
            let Some(t42) = c.ld32(c.r64(2).wrapping_add(0x117)) else { return RegfileExit::fault(0x7000892193); };
            c.cmp(t42, c.r32(8), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r64(7), 0xaeu64, 64);
            if c.cc_a() { blk = 35; continue; }
            blk = 34; continue;
        }
        34 => {
            return RegfileExit::fault(0x70008921a5);
        }
        35 => {
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008921a6); };
            c.cmp(t43, c.r32(8), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r64(7), 0xafu64, 64);
            if c.cc_a() { blk = 306; continue; }
            blk = 37; continue;
        }
        37 => {
            return RegfileExit::fault(0x70008921bc);
        }
        38 => {
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008921bd); };
            c.w32(2, t44);
            let r45 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r45);
            blk = 39; continue;
        }
        39 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w64(0, c.r64(2));
            return c.call(0x70008a17f4, 0x70008921d2, 0x70008921cd);
        }
        41 => {
            c.w64(2, c.r64(0));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008921d5); };
            c.w64(15, t46);
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008921d9); };
            c.w64(6, t47);
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008921dd); };
            c.w64(12, t48);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008921e8); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008921ec); };
            c.w64(11, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008921f0); };
            c.w64(9, t51);
            blk = 44; continue;
        }
        42 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ge() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(2, c.r64(3));
            blk = 44; continue;
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x70008921fe); }
            c.cmp(c.r64(15), 0x0u64, 64);
            if c.cc_be() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t52) = c.ld64(c.r64(9).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000892208); };
            c.w64(9, t52);
            c.w64(7, c.r64(9));
            blk = 47; continue;
        }
        46 => {
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000892211); };
            c.w64(7, t53);
            blk = 47; continue;
        }
        47 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000892218); };
            c.w64(9, t54);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(0, c.r64(7));
            return c.call(0x7000899a14, 0x700089222d, 0x7000892228);
        }
        49 => {
            c.w64(7, c.r64(0));
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000892230); };
            c.w64(15, t55);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089223b); };
            c.w64(9, t56);
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000892242); };
            c.w64(2, t57);
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000892246); };
            c.w64(11, t58);
            blk = 50; continue;
        }
        50 => {
            c.w32(8, c.r32(2));
            c.w32(12, c.r32(7));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.logic(c.r8(7) & 0x1u64, 8);
            c.w8(6, (c.cc_e() as u64));
            c.w32(6, c.r8(6));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000892265); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000892269); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892272); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 76; continue; }
            blk = 52; continue;
        }
        52 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 55; continue;
        }
        53 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 76; continue; }
            blk = 54; continue;
        }
        54 => {
            c.w64(12, c.r64(1));
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 59; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700089229c); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008922a6); }
            blk = 61; continue;
        }
        58 => {
            c.w64(12, c.r64(2));
            let r59 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r59);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 61; continue;
        }
        59 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 63; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(12, c.r64(7));
            let r60 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r60);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008922c9); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 61; continue;
        }
        61 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 75; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 64; continue;
        }
        63 => {
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_le() { blk = 75; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008922ec); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008922f6); }
            blk = 70; continue;
        }
        67 => {
            c.w64(12, c.r64(7));
            let r61 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r61);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 70; continue;
        }
        68 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 72; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(12, c.r64(2));
            let r62 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r62);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892319); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 70; continue;
        }
        70 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 74; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 73; continue;
        }
        72 => {
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_le() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t63) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000892333); };
            c.w64(12, t63);
            blk = 91; continue;
        }
        74 => {
            c.w64(12, c.r64(7));
            blk = 91; continue;
        }
        75 => {
            c.w64(12, c.r64(2));
            blk = 91; continue;
        }
        76 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 309; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_e() { blk = 309; continue; }
            blk = 78; continue;
        }
        78 => {
            let r64 = c.add(c.r32(12), c.r32(8), 32);
            c.w32(12, r64);
            if c.cc_o() { blk = 309; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            blk = 80; continue;
        }
        80 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892373); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 91; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 85; continue;
        }
        83 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ge() { blk = 91; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(6, c.r64(1));
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892396); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 90; continue; }
            blk = 87; continue;
        }
        87 => {
            blk = 89; continue;
        }
        88 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_le() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t65) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008923af); };
            c.w64(12, t65);
            blk = 91; continue;
        }
        90 => {
            c.w64(12, c.r64(3));
            blk = 91; continue;
        }
        91 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x70008923bb); }
            c.cmp(c.r64(15), 0x1u64, 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t66) = c.ld64(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008923c5); };
            c.w64(11, t66);
            blk = 94; continue;
        }
        93 => {
            c.w64(11, c.r64(9));
            blk = 94; continue;
        }
        94 => {
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008923ce); };
            c.cmp(t67, c.r32(11), 32);
            if c.cc_e() { blk = 98; continue; }
            blk = 95; continue;
        }
        95 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(0, c.r64(11));
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008923e0); };
            c.w64(6, t68);
            return c.call(0x7000899a14, 0x70008923e9, 0x70008923e4);
        }
        97 => {
            c.w64(11, c.r64(0));
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008923ec); };
            c.w64(15, t69);
            let Some(t70) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008923f0); };
            c.w64(12, t70);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008923fb); };
            c.w64(9, t71);
            let Some(t72) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000892402); };
            c.w64(2, t72);
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892406); };
            c.w64(0, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089240a); };
            c.w64(8, t74);
            blk = 99; continue;
        }
        98 => {
            c.w64(11, c.r64(2));
            blk = 99; continue;
        }
        99 => {
            c.w32(6, c.r32(11));
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 102; continue; }
            blk = 100; continue;
        }
        100 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892427); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 124; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 104; continue;
        }
        102 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 124; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(6, c.r64(1));
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 105; continue;
        }
        105 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892451); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700089245c); }
            blk = 110; continue;
        }
        107 => {
            c.w64(6, c.r64(2));
            let r75 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r75);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(6) as u32 as i32 as i64));
            blk = 110; continue;
        }
        108 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 112; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(6, c.r64(11));
            let r76 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r76);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700089247c); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(6) as u32 as i32 as i64));
            blk = 110; continue;
        }
        110 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 139; continue; }
            blk = 111; continue;
        }
        111 => {
            blk = 113; continue;
        }
        112 => {
            c.cmp(c.r32(2), c.r32(11), 32);
            if c.cc_le() { blk = 139; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 117; continue; }
            blk = 114; continue;
        }
        114 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008924a3); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008924ad); }
            blk = 119; continue;
        }
        116 => {
            c.w64(0, c.r64(11));
            let r77 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r77);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            blk = 119; continue;
        }
        117 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 121; continue; }
            blk = 118; continue;
        }
        118 => {
            let r78 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r78);
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008924cb); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 119; continue;
        }
        119 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 123; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 122; continue;
        }
        121 => {
            c.cmp(c.r32(11), c.r32(2), 32);
            if c.cc_le() { blk = 123; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008924e6); };
            c.w64(2, t79);
            blk = 139; continue;
        }
        123 => {
            c.w64(2, c.r64(11));
            blk = 139; continue;
        }
        124 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 126; continue;
        }
        126 => {
            let r80 = c.add(c.r32(6), c.r32(8), 32);
            c.w32(6, r80);
            if c.cc_o() { blk = 326; continue; }
            blk = 127; continue;
        }
        127 => {
            c.w64(2, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 128; continue;
        }
        128 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 131; continue; }
            blk = 129; continue;
        }
        129 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700089251d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 139; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 133; continue;
        }
        131 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_ge() { blk = 139; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(11, c.r64(1));
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 134; continue;
        }
        134 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x700089253f); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 138; continue; }
            blk = 135; continue;
        }
        135 => {
            blk = 137; continue;
        }
        136 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_le() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x7000892555); };
            c.w64(2, t81);
            blk = 139; continue;
        }
        138 => {
            c.w64(2, c.r64(3));
            blk = 139; continue;
        }
        139 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w32(0, c.r32(2));
            c.w32(6, c.r32(12));
            c.logic(c.r8(2) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            c.cmp(c.r32(11), 0x0u64, 32);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(2)) { return RegfileExit::fault(0x7000892589); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700089258d); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_e() { blk = 344; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 344; continue; }
            blk = 141; continue;
        }
        141 => {
            let r82 = c.sub(c.r32(0), c.r32(6), 32);
            c.w32(0, r82);
            if c.cc_o() { blk = 344; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            blk = 143; continue;
        }
        143 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008925b5); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 154; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 148; continue;
        }
        146 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ge() { blk = 154; continue; }
            blk = 147; continue;
        }
        147 => {
            c.w64(0, c.r64(1));
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 151; continue; }
            blk = 149; continue;
        }
        149 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008925d7); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 153; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 152; continue;
        }
        151 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_le() { blk = 153; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x448)) else { return RegfileExit::fault(0x70008925ef); };
            c.w64(11, t83);
            blk = 154; continue;
        }
        153 => {
            c.w64(11, c.r64(3));
            blk = 154; continue;
        }
        154 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x70008925fb); }
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 276; continue; }
            blk = 155; continue;
        }
        155 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 275; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t84) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000892613); };
            c.cmp(t84, 0x0u64, 32);
            if c.cc_ne() { blk = 173; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t85) = c.ld64(c.r64(13).wrapping_add(0xfc0)) else { return RegfileExit::fault(0x700089261d); };
            c.w64(0, t85);
            let Some(t86) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000892624); };
            c.cmp(t86, c.r32(3), 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 158; continue;
        }
        158 => {
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089262d); };
            c.w64(0, t87);
            let Some(t88) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892631); };
            c.w32(6, t88);
            let r89 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r89);
            let Some(t90) = c.ld16(c.r64(6).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892637); };
            c.cmp(t90, 0x834u64, 16);
            if c.cc_ne() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w64(8, c.r64(1));
            blk = 161; continue;
        }
        160 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t91) = c.ld8(c.r64(6).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000892651); };
            c.w32(8, t91);
            let r92 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r92);
            let r93 = c.r32(8) & 0x3fu64;
            c.w32(8, r93);
            c.logic(r93, 32);
            c.cmp(c.r32(8), 0x5u64, 32);
            if c.cc_a() { blk = 172; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892668); };
            c.w64(8, t94);
            let Some(t95) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089266c); };
            c.w32(9, t95);
            let r96 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r96);
            let Some(t97) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000892673); };
            c.w32(9, t97);
            let r98 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r98);
            let Some(t99) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089267a); };
            c.w32(2, t99);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x4cu64, 64);
            if c.cc_a() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            return RegfileExit::fault(0x700089268a);
        }
        165 => {
            let Some(t100) = c.ld32(c.r64(9).wrapping_add(0x137)) else { return RegfileExit::fault(0x700089268b); };
            c.w32(9, t100);
            let Some(t101) = c.ld32(c.r64(6).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000892692); };
            c.cmp(t101, c.r32(9), 32);
            if c.cc_ne() { blk = 172; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x7000892698); };
            c.w64(9, t102);
            let Some(t103) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089269f); };
            c.cmp(t103, c.r32(3), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t104) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008926a5); };
            c.w32(9, t104);
            c.w32(2, c.r64(12).wrapping_add(c.r64(11)));
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_b() { blk = 170; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(6, c.r64(8));
            c.w64(3, c.r64(12));
            c.w64(1, c.r64(11));
            return c.call(0x70008283b4, 0x70008926c0, 0x70008926bb);
        }
        169 => {
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008926c0); };
            c.w64(9, t105);
            let r106 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r106);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008926cb); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008926cc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008926d2); }
            return c.ret(0x0, 0x70008926d4);
        }
        170 => {
            c.w64(6, c.r64(8));
            blk = 277; continue;
        }
        171 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008926dd); };
            c.w64(0, t107);
            blk = 172; continue;
        }
        172 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 174; continue;
        }
        173 => {
            c.w64(9, c.r64(1));
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008926e9); };
            c.w64(0, t108);
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008926f3); };
            c.w64(6, t109);
            let Some(t110) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008926f7); };
            c.w32(9, t110);
            let r111 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r111);
            let Some(t112) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008926fe); };
            c.w32(9, t112);
            let r113 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r113);
            let Some(t114) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892705); };
            c.w32(2, t114);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t115) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089270c); };
            c.w32(8, t115);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0xb0u64, 64);
            if c.cc_a() { blk = 177; continue; }
            blk = 176; continue;
        }
        176 => {
            return RegfileExit::fault(0x700089271c);
        }
        177 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x700089271d); };
            c.cmp(t116, c.r32(8), 32);
            if c.cc_ne() { blk = 180; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 195; continue;
        }
        179 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089272b); };
            c.w64(6, t117);
            blk = 180; continue;
        }
        180 => {
            let Some(t118) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089272f); };
            c.cmp(t118, 0x0u64, 32);
            if c.cc_ne() { blk = 277; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t119) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892739); };
            c.w32(9, t119);
            let r120 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r120);
            let Some(t121) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000892740); };
            c.w32(9, t121);
            let r122 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r122);
            let Some(t123) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892747); };
            c.w32(2, t123);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t124) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089274e); };
            c.w32(8, t124);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x44u64, 64);
            if c.cc_a() { blk = 183; continue; }
            blk = 182; continue;
        }
        182 => {
            return RegfileExit::fault(0x700089275b);
        }
        183 => {
            let Some(t125) = c.ld32(c.r64(9).wrapping_add(0x117)) else { return RegfileExit::fault(0x700089275c); };
            c.cmp(t125, c.r32(8), 32);
            c.w8(3, (c.cc_e() as u64));
            c.w32(3, c.r8(3));
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 190; continue; }
            blk = 184; continue;
        }
        184 => {
            c.cmp(c.r64(2), 0xaeu64, 64);
            if c.cc_a() { blk = 186; continue; }
            blk = 185; continue;
        }
        185 => {
            return RegfileExit::fault(0x7000892777);
        }
        186 => {
            let Some(t126) = c.ld32(c.r64(9).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x7000892778); };
            c.cmp(t126, c.r32(8), 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 187; continue;
        }
        187 => {
            c.cmp(c.r64(2), 0xafu64, 64);
            if c.cc_a() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            return RegfileExit::fault(0x700089278a);
        }
        189 => {
            let Some(t127) = c.ld32(c.r64(9).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x700089278b); };
            c.cmp(t127, c.r32(8), 32);
            if c.cc_ne() { blk = 277; continue; }
            blk = 190; continue;
        }
        190 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 223; continue; }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r64(2), 0xaeu64, 64);
            if c.cc_a() { blk = 193; continue; }
            blk = 192; continue;
        }
        192 => {
            return RegfileExit::fault(0x70008927aa);
        }
        193 => {
            let Some(t128) = c.ld32(c.r64(9).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008927ab); };
            c.cmp(t128, c.r32(8), 32);
            if c.cc_ne() { blk = 277; continue; }
            blk = 194; continue;
        }
        194 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 195; continue;
        }
        195 => {
            c.cmp(c.r32(11), 0xfffcu64, 32);
            if c.cc_ge() { blk = 277; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t129) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008927c7); };
            c.w32(9, t129);
            let r130 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r130);
            let Some(t131) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008927ce); };
            c.w32(2, t131);
            let r132 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r132);
            let Some(t133) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008927d5); };
            c.w32(2, t133);
            let r134 = c.sub(c.r32(2), 0x77u64, 32);
            c.w32(2, r134);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008927dc); }
            c.cmp(c.r32(2), 0xcu64, 32);
            if c.cc_a() { blk = 277; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t135) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008927ed); };
            c.w32(8, t135);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_b() { blk = 277; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t136) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008927fa); };
            c.w32(2, t136);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), 0x6cu64, 64);
            if c.cc_a() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            return RegfileExit::fault(0x7000892809);
        }
        200 => {
            c.w64(2, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t137) = c.ld32(c.r64(6).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700089280d); };
            c.w32(8, t137);
            c.w64(3, c.r64(2));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 204; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(2, c.r64(2).wrapping_mul(2).wrapping_add(0x18));
            let r138 = c.shl(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r138);
            let Some(t139) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089282a); };
            c.w64(6, t139);
            let Some(t140) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000892831); };
            c.w64(0, t140);
            c.w64(15, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000892838); };
            c.w64(9, t141);
            let Some(t142) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700089283f); };
            c.cmp(t142, c.r64(15), 64);
            if c.cc_be() { blk = 362; continue; }
            blk = 202; continue;
        }
        202 => {
            if !c.st64(c.r64(6), c.r64(15)) { return RegfileExit::fault(0x7000892848); }
            c.w64(9, c.r64(0).wrapping_add(0x1));
            blk = 203; continue;
        }
        203 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700089284f); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000892853); }
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000892857); };
            c.w64(15, t143);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700089285e); }
            c.w64(15, c.r64(9).wrapping_add(0x10));
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000892866); }
            let Some(t144) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700089286a); };
            c.w64(0, t144);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000892871); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000892875); }
            c.w64(2, c.r64(15));
            c.w64(3, c.r64(9));
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 207; continue;
        }
        204 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000892884); };
            c.w64(3, t145);
            let Some(t146) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700089288b); };
            c.w64(2, t146);
            c.w64(6, c.r64(2).wrapping_add(0x10));
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000892892); };
            c.w64(7, t147);
            let Some(t148) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000892899); };
            c.cmp(t148, c.r64(6), 64);
            if c.cc_be() { blk = 364; continue; }
            blk = 205; continue;
        }
        205 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008928a2); }
            c.w64(3, c.r64(2).wrapping_add(0x1));
            blk = 206; continue;
        }
        206 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008928a9); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x70008928ad); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008928b1); };
            c.w64(2, t149);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008928b8); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008928bb); }
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008928be); };
            c.w64(7, t150);
            c.w64(9, c.r64(1));
            blk = 207; continue;
        }
        207 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008928c5); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 211; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 211; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008928e0); };
            c.w64(15, t151);
            blk = 210; continue;
        }
        210 => {
            let r152 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r152);
            if !c.st32(c.r64(9).wrapping_add(c.r64(2)), c.r32(15)) { return RegfileExit::fault(0x70008928eb); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 210; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t153) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008928f5); };
            c.w32(9, t153);
            let r154 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r154);
            let Some(t155) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008928fc); };
            c.w32(15, t155);
            let r156 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r156);
            let Some(t157) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892903); };
            c.w32(15, t157);
            let r158 = c.sub(c.r32(15), 0x77u64, 32);
            c.w32(15, r158);
            c.cmp(c.r32(15), 0xcu64, 32);
            if c.cc_a() { blk = 222; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(9));
            let r159 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r159);
            c.logic(r159, 64);
            c.w64(0, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            let Some(t160) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700089292e); };
            c.logic(t160 & 0x4u64, 8);
            if c.cc_e() { blk = 219; continue; }
            blk = 213; continue;
        }
        213 => {
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r161 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r161);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_e() { blk = 221; continue; }
            blk = 214; continue;
        }
        214 => {
            let r162 = c.neg(c.r64(12), 64);
            c.w64(12, r162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000892952); };
            c.w64(15, t163);
            blk = 215; continue;
        }
        215 => {
            let Some(t164) = c.ld32(c.r64(15).wrapping_add(c.r64(0))) else { return RegfileExit::fault(0x7000892956); };
            c.w32(2, t164);
            c.w64(1, c.r64(0).wrapping_add(c.r64(12).wrapping_mul(4)));
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(2)) { return RegfileExit::fault(0x700089295e); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r165 = c.r64(6) & c.r64(9);
            c.w64(6, r165);
            c.logic(r165, 64);
            let Some(t166) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000892971); };
            c.logic(t166 & 0x4u64, 8);
            if c.cc_ne() { blk = 404; continue; }
            blk = 217; continue;
        }
        217 => {
            let r167 = c.add(c.r64(0), 0x4u64, 64);
            c.w64(0, r167);
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ne() { blk = 215; continue; }
            blk = 218; continue;
        }
        218 => {
            blk = 221; continue;
        }
        219 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089298a); };
            c.w64(12, t168);
            c.w64(2, c.r64(0).wrapping_add(c.r64(12)));
            c.w64(1, c.r64(9).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r169 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r169);
            let r170 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r170);
            c.logic(r170, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008929a1); }
            let Some(t171) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x70008929a6); };
            c.w64(0, t171);
            c.w64(10, 0x70008929b4u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008929b4); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008929b8); }
            return c.call(c.r64(0), 0x70008929be, 0x70008929bc);
        }
        220 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008929be); }
            let Some(t172) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008929c6); };
            c.w64(4, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008929cb); };
            c.w64(3, t173);
            blk = 221; continue;
        }
        221 => {
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008929cf); };
            c.w64(9, t174);
            let r175 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r175);
            c.w64(0, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008929dd); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008929de); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008929e4); }
            return c.ret(0x0, 0x70008929e6);
        }
        222 => {
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008929e7); };
            c.w64(15, t176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008929eb); };
            c.w64(6, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008929ef); };
            c.w64(0, t178);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 277; continue;
        }
        223 => {
            c.cmp(c.r32(11), 0xfffcu64, 32);
            if c.cc_ge() { blk = 273; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(9, c.r64(0));
            let Some(t179) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892a0a); };
            c.w32(0, t179);
            let r180 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r180);
            let Some(t181) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892a11); };
            c.w32(3, t181);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(0)) { return RegfileExit::fault(0x7000892a14); }
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x7000892a18); };
            c.cmp(t182, c.r32(3), 32);
            if c.cc_ne() { blk = 274; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t183) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000892a25); };
            c.w32(3, t183);
            let r184 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r184);
            let Some(t185) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000892a2b); };
            c.w32(2, t185);
            let r186 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r186);
            let Some(t187) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892a31); };
            c.w32(8, t187);
            let Some(t188) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892a35); };
            c.w32(2, t188);
            let r189 = c.sub(c.r32(2), 0x77u64, 32);
            c.w32(2, r189);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x7000892a3c); }
            c.cmp(c.r32(2), 0xcu64, 32);
            if c.cc_a() { blk = 274; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w32(2, c.r64(12).wrapping_add(c.r64(11)));
            let Some(t190) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892a4d); };
            c.w32(9, t190);
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_b() { blk = 272; continue; }
            blk = 227; continue;
        }
        227 => {
            let Some(t191) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892a5a); };
            c.w32(9, t191);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t192) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892a61); };
            c.w32(2, t192);
            let r193 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r193);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x7000892a6a); }
            c.cmp(c.r64(9), 0x6cu64, 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            return RegfileExit::fault(0x7000892a74);
        }
        229 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t194) = c.ld32(c.r64(6).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x7000892a78); };
            c.w32(6, t194);
            c.w64(7, c.r64(9));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 233; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000892a8a); };
            c.w64(1, t195);
            let Some(t196) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000892a91); };
            c.w64(15, t196);
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x18));
            c.w64(2, c.r64(9).wrapping_add(c.r64(15)));
            let r197 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r197);
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000892aa2); };
            c.w64(3, t198);
            let Some(t199) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000892aa9); };
            c.cmp(t199, c.r64(2), 64);
            if c.cc_be() { blk = 366; continue; }
            blk = 231; continue;
        }
        231 => {
            if !c.st64(c.r64(1), c.r64(2)) { return RegfileExit::fault(0x7000892ab2); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            blk = 232; continue;
        }
        232 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000892ab9); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000892abd); }
            let Some(t200) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000892ac1); };
            c.w64(15, t200);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000892ac8); }
            c.w64(15, c.r64(9).wrapping_add(0x10));
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000892ad0); }
            let Some(t201) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000892ad4); };
            c.w64(3, t201);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000892adb); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000892adf); }
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 236; continue;
        }
        233 => {
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000892ae7); };
            c.w64(9, t202);
            let Some(t203) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000892aee); };
            c.w64(7, t203);
            c.w64(0, c.r64(7).wrapping_add(0x10));
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000892af5); };
            c.w64(15, t204);
            let Some(t205) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000892afc); };
            c.cmp(t205, c.r64(0), 64);
            if c.cc_be() { blk = 368; continue; }
            blk = 234; continue;
        }
        234 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x7000892b05); }
            c.w64(9, c.r64(7).wrapping_add(0x1));
            blk = 235; continue;
        }
        235 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000892b0c); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(11)) { return RegfileExit::fault(0x7000892b10); }
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000892b14); };
            c.w64(15, t206);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x7000892b1b); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000892b1f); }
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000892b23); };
            c.w64(0, t207);
            blk = 236; continue;
        }
        236 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000892b27); }
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 240; continue; }
            blk = 237; continue;
        }
        237 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(3).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000892b41); };
            c.w64(2, t208);
            blk = 239; continue;
        }
        239 => {
            let r209 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r209);
            if !c.st32(c.r64(3).wrapping_add(c.r64(15)), c.r32(2)) { return RegfileExit::fault(0x7000892b4c); }
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_ne() { blk = 239; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w32(15, c.r64(12).wrapping_add(c.r64(11)));
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_l() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            blk = 243; continue;
        }
        242 => {
            c.w64(15, c.r64(8));
            blk = 243; continue;
        }
        243 => {
            let Some(t210) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892b67); };
            c.w32(3, t210);
            let r211 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r211);
            c.w64(1, c.r64(12));
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000892b73); };
            c.w64(6, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000892b77); };
            c.w64(7, t213);
            blk = 245; continue;
        }
        244 => {
            c.w64(11, (c.r32(2) as u32 as i32 as i64 as u64));
            let r214 = c.add(c.r32(1), 0x2u64, 32);
            c.w32(1, r214);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let r215 = c.add(c.r32(2), 0x2u64, 32);
            c.w32(2, r215);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000892b8c); }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r32(1), c.r32(15), 32);
            if c.cc_ge() { blk = 254; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t216) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892b96); };
            c.w32(9, t216);
            c.cmp(c.r32(1), c.r32(9), 32);
            if c.cc_ae() { blk = 269; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.shl(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let Some(t217) = c.ld32(c.r64(0).wrapping_add(c.r64(9)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000892ba9); };
            c.w32(11, t217);
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000892bae); };
            c.cmp(t218, c.r32(11), 32);
            if c.cc_ne() { blk = 250; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t219) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892bb7); };
            c.w32(11, t219);
            c.cmp(c.r32(1), c.r32(11), 32);
            if c.cc_ae() { blk = 270; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t220) = c.ld32(c.r64(6).wrapping_add(c.r64(9)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892bc4); };
            c.w32(9, t220);
            blk = 252; continue;
        }
        250 => {
            let Some(t221) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892bcb); };
            c.w32(9, t221);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_ae() { blk = 271; continue; }
            blk = 251; continue;
        }
        251 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t222) = c.ld32(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000892bdb); };
            c.w32(9, t222);
            blk = 252; continue;
        }
        252 => {
            let Some(t223) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000892be0); };
            c.w32(11, t223);
            c.cmp(c.r32(2), c.r32(11), 32);
            if c.cc_b() { blk = 244; continue; }
            blk = 253; continue;
        }
        253 => {
            blk = 370; continue;
        }
        254 => {
            c.cmp(c.r32(8), c.r32(12), 32);
            if c.cc_l() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w64(9, c.r64(8));
            blk = 257; continue;
        }
        256 => {
            c.w64(9, c.r64(12));
            blk = 257; continue;
        }
        257 => {
            let Some(t224) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892bfb); };
            c.w64(11, t224);
            let r225 = c.add(c.r32(11), c.r32(12), 32);
            c.w32(11, r225);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_l() { blk = 259; continue; }
            blk = 258; continue;
        }
        258 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(8));
            blk = 259; continue;
        }
        259 => {
            c.w32(12, c.r32(9));
            let r226 = c.sub(c.r32(11), c.r32(12), 32);
            c.w32(11, r226);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(3));
            let r227 = c.r64(12) & 0xfffffffffffc0000u64;
            c.w64(12, r227);
            c.logic(r227, 64);
            c.w64(15, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            let Some(t228) = c.ld8(c.r64(12).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000892c2b); };
            c.logic(t228 & 0x4u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 260; continue;
        }
        260 => {
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r229 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r229);
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 268; continue; }
            blk = 261; continue;
        }
        261 => {
            c.w64(12, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r230 = c.sub(c.r64(12), c.r64(9), 64);
            c.w64(12, r230);
            blk = 262; continue;
        }
        262 => {
            let Some(t231) = c.ld32(c.r64(6).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000892c56); };
            c.w32(9, t231);
            c.w64(0, c.r64(15).wrapping_add(c.r64(12).wrapping_mul(4)));
            if !c.st32(c.r64(3).wrapping_add(c.r64(0)), c.r32(9)) { return RegfileExit::fault(0x7000892c5e); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            c.w64(2, 0xfffffffffffc0000u64);
            let r232 = c.r64(2) & c.r64(3);
            c.w64(2, r232);
            c.logic(r232, 64);
            let Some(t233) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000892c72); };
            c.logic(t233 & 0x4u64, 8);
            if c.cc_ne() { blk = 401; continue; }
            blk = 264; continue;
        }
        264 => {
            let r234 = c.add(c.r64(15), 0x4u64, 64);
            c.w64(15, r234);
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ne() { blk = 262; continue; }
            blk = 265; continue;
        }
        265 => {
            blk = 268; continue;
        }
        266 => {
            c.w64(9, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(2, c.r64(15).wrapping_add(c.r64(6)));
            c.w64(1, c.r64(3).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r235 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r235);
            let r236 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r236);
            c.logic(r236, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000892ca2); }
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0x13c8)) else { return RegfileExit::fault(0x7000892ca7); };
            c.w64(0, t237);
            c.w64(10, 0x7000892cb5u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000892cb5); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000892cb9); }
            return c.call(c.r64(0), 0x7000892cbf, 0x7000892cbd);
        }
        267 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000892cbf); }
            let Some(t238) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000892cc7); };
            c.w64(4, t238);
            blk = 268; continue;
        }
        268 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000892ccc); };
            c.w64(9, t239);
            let r240 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000892cd4); };
            c.w64(0, t241);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000892cdb); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000892cdc); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000892ce2); }
            return c.ret(0x0, 0x7000892ce4);
        }
        269 => {
            return RegfileExit::fault(0x7000892ce5);
        }
        270 => {
            return RegfileExit::fault(0x7000892ce6);
        }
        271 => {
            return RegfileExit::fault(0x7000892ce7);
        }
        272 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000892ce8); };
            c.w64(9, t242);
            blk = 274; continue;
        }
        273 => {
            c.w64(9, c.r64(0));
            blk = 274; continue;
        }
        274 => {
            c.w64(0, c.r64(9));
            blk = 277; continue;
        }
        275 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892cf6); };
            c.w64(6, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000892cfa); };
            c.w64(0, t244);
            blk = 277; continue;
        }
        276 => {
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892d00); };
            c.w64(6, t245);
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000892d04); };
            c.w64(0, t246);
            blk = 277; continue;
        }
        277 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000892d08); }
            c.w64(0, c.r64(1));
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x1c80)) else { return RegfileExit::fault(0x7000892d0c); };
            c.w64(3, t247);
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x7000892d1b, 0x7000892d16);
        }
        278 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892d1b); };
            if !c.push64(t248) { return RegfileExit::fault(0x7000892d1b); }
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000892d1e); };
            c.w64(9, t249);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000892d25); }
            c.w64(7, c.r64(0));
            c.w64(2, c.r64(0));
            c.w32(0, 0x1u64);
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892d32); };
            c.w64(6, t250);
            return c.call(0x70007f7214, 0x7000892d3b, 0x7000892d36);
        }
        279 => {
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000892d3d); }
            let Some(t251) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000892d41); };
            c.w64(9, t251);
            blk = 280; continue;
        }
        280 => {
            c.w32(11, c.r32(9));
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000892d54); };
            c.w64(2, t252);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000892d58); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000892d5c); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 284; continue; }
            blk = 281; continue;
        }
        281 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892d66); }
            let Some(t253) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000892d6c); };
            c.cmp(t253, 0x0u64, 32);
            if c.cc_ne() { blk = 283; continue; }
            blk = 282; continue;
        }
        282 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892d72); }
            blk = 286; continue;
        }
        283 => {
            c.w64(15, c.r64(2));
            let r254 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r254);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(15) as u32 as i32 as i64));
            blk = 286; continue;
        }
        284 => {
            let Some(t255) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000892d8a); };
            c.cmp(t255, 0x0u64, 32);
            if c.cc_ne() { blk = 288; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(15, c.r64(9));
            let r256 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r256);
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892d96); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            blk = 286; continue;
        }
        286 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 289; continue; }
            blk = 287; continue;
        }
        287 => {
            blk = 299; continue;
        }
        288 => {
            c.cmp(c.r32(9), c.r32(2), 32);
            if c.cc_ge() { blk = 299; continue; }
            blk = 289; continue;
        }
        289 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x7000892db8); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000892dbc); }
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000892dc0); };
            c.w64(0, t257);
            c.w64(3, c.r64(9));
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892dc7); };
            c.w64(6, t258);
            return c.call(0x7000820354, 0x7000892dd0, 0x7000892dcb);
        }
        290 => {
            let Some(t259) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000892dd0); };
            c.cmp(t259, c.r32(0), 32);
            if c.cc_ne() { blk = 293; continue; }
            blk = 291; continue;
        }
        291 => {
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000892dd9); };
            c.w64(0, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000892ddd); };
            c.w64(3, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892de1); };
            c.w64(6, t262);
            return c.call(0x7000870914, 0x7000892dea, 0x7000892de5);
        }
        292 => {
            c.w64(1, c.r64(0));
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892ded); };
            c.w64(0, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000892df1); };
            c.w64(3, t264);
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892df5); };
            c.w64(6, t265);
            return c.call(0x7000899bb4, 0x7000892dfe, 0x7000892df9);
        }
        293 => {
            let Some(t266) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000892dfe); };
            c.cmp(t266, 0x0u64, 32);
            if c.cc_e() { blk = 371; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t267) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000892e08); };
            c.w64(9, t267);
            let r268 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r268);
            if c.cc_o() { blk = 371; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 296; continue;
        }
        296 => {
            let Some(t269) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000892e19); };
            c.w32(11, t269);
            let Some(t270) = c.ld8(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000892e1d); };
            c.logic(t270 & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 386; continue; }
            blk = 297; continue;
        }
        297 => {
            let r271 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r271);
            if c.cc_o() { blk = 386; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(3, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 280; continue;
        }
        299 => {
            let Some(t272) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892e45); };
            c.w64(6, t272);
            let Some(t273) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892e49); };
            c.w64(0, t273);
            c.w64(1, c.r64(3));
            let Some(t274) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000892e50); };
            c.w64(3, t274);
            c.w64(9, c.r64(3));
            return c.call(0x7000871cb4, 0x7000892e5f, 0x7000892e5a);
        }
        300 => {
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000892e5f); };
            c.w64(9, t275);
            let r276 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r276);
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892e67); };
            c.w64(0, t277);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000892e6e); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000892e6f); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(9).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000892e75); }
            return c.ret(0x0, 0x7000892e77);
        }
        301 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x7000892e78); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000892e7e); };
            c.w64(3, t278);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000892e8d, 0x7000892e88);
        }
        302 => {
            let Some(t279) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892e8d); };
            c.w64(6, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000892e91); };
            c.w64(0, t280);
            blk = 1; continue;
        }
        303 => {
            c.w64(0, c.r64(12));
            return c.call(0x70008a2054, 0x7000892ea2, 0x7000892e9d);
        }
        304 => {
            c.w64(12, c.r64(0));
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000892ea5); };
            c.w64(15, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892ea9); };
            c.w64(6, t282);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000892eb4); };
            c.w64(11, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892eb8); };
            c.w64(9, t284);
            blk = 20; continue;
        }
        305 => {
            let Some(t285) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000892ec1); };
            c.w32(2, t285);
            let r286 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r286);
            blk = 39; continue;
        }
        306 => {
            let Some(t287) = c.ld32(c.r64(2).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000892ece); };
            c.cmp(t287, c.r32(8), 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000892edb); };
            c.w64(2, t288);
            c.w64(0, c.r64(12));
            c.w64(3, c.r64(2));
            return c.call(0x7000870914, 0x7000892eed, 0x7000892ee8);
        }
        308 => {
            c.w64(2, c.r64(0));
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000892ef0); };
            c.w64(15, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000892ef4); };
            c.w64(6, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000892ef8); };
            c.w64(12, t291);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000892f03); };
            c.w64(0, t292);
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000892f07); };
            c.w64(11, t293);
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892f0b); };
            c.w64(9, t294);
            blk = 39; continue;
        }
        309 => {
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_ne() { blk = 311; continue; }
            blk = 310; continue;
        }
        310 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892f19); }
            blk = 312; continue;
        }
        311 => {
            let r295 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r295);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 312; continue;
        }
        312 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 314; continue; }
            blk = 313; continue;
        }
        313 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000892f2f); }
            blk = 315; continue;
        }
        314 => {
            c.w64(12, c.r64(2));
            let r296 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r296);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            blk = 315; continue;
        }
        315 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(12, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000892f57); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 321; continue; }
            blk = 316; continue;
        }
        316 => {
            if c.cc_ne() { blk = 321; continue; }
            blk = 317; continue;
        }
        317 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            c.movq_rx(6, 0);
            if let Some(sr) = c.shr(c.r64(6), 0x20u64, 64) { c.w64(6, sr); }
            c.cmp(c.r32(6), 0x0u64, 32);
            if c.cc_l() { blk = 321; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w64(6, c.r64(12));
            let r297 = c.add(c.r32(6), c.r32(12), 32);
            c.w32(6, r297);
            if c.cc_o() { blk = 321; continue; }
            blk = 320; continue;
        }
        320 => {
            c.w64(12, (c.r32(6) as u32 as i32 as i64 as u64));
            blk = 80; continue;
        }
        321 => {
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000892f88); };
            c.w64(12, t298);
            let Some(t299) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000892f8f); };
            c.w64(6, t299);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(6)) { return RegfileExit::fault(0x7000892f97); }
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000892f9b); };
            c.w64(6, t300);
            let Some(t301) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000892fa2); };
            c.cmp(t301, c.r64(7), 64);
            if c.cc_a() { blk = 324; continue; }
            blk = 322; continue;
        }
        322 => {
            c.w32(2, 0xcu64);
            c.w64(12, c.r64(2));
            return c.call(0x70007febf4, 0x7000892fb4, 0x7000892faf);
        }
        323 => {
            c.w64(12, c.r64(0));
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000892fb7); };
            c.w64(15, t302);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t303) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000892fc2); };
            c.w64(9, t303);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000892fc9); };
            c.w64(2, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000892fcd); };
            c.w64(0, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000892fd1); };
            c.w64(8, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000892fd5); };
            c.w64(11, t307);
            blk = 325; continue;
        }
        324 => {
            if !c.st64(c.r64(12), c.r64(7)) { return RegfileExit::fault(0x7000892fdb); }
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000892fdf); };
            c.w64(12, t308);
            let r309 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r309);
            blk = 325; continue;
        }
        325 => {
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000892fe7); };
            c.w64(6, t310);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000892fee); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000892ff3); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000892ff8); }
            blk = 80; continue;
        }
        326 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 328; continue; }
            blk = 327; continue;
        }
        327 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893009); }
            blk = 329; continue;
        }
        328 => {
            let r311 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r311);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 329; continue;
        }
        329 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 331; continue; }
            blk = 330; continue;
        }
        330 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893022); }
            blk = 332; continue;
        }
        331 => {
            let r312 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r312);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(2) as u32 as i32 as i64));
            blk = 332; continue;
        }
        332 => {
            c.arith_sd(0, 0, c.xmm_u64(1));
            let cv = c.cvttsd2si32(c.xmm_u64(0)); c.w32(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x7000893045); }
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 338; continue; }
            blk = 333; continue;
        }
        333 => {
            if c.cc_ne() { blk = 338; continue; }
            blk = 334; continue;
        }
        334 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 336; continue; }
            blk = 335; continue;
        }
        335 => {
            c.movq_rx(0, 0);
            if let Some(sr) = c.shr(c.r64(0), 0x20u64, 64) { c.w64(0, sr); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 338; continue; }
            blk = 336; continue;
        }
        336 => {
            c.w64(0, c.r64(11));
            let r313 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r313);
            if c.cc_o() { blk = 338; continue; }
            blk = 337; continue;
        }
        337 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(2));
            blk = 343; continue;
        }
        338 => {
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000893076); };
            c.w64(11, t314);
            let Some(t315) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700089307d); };
            c.w64(0, t315);
            c.w64(2, c.r64(0).wrapping_add(0xc));
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000893084); };
            c.w64(6, t316);
            let Some(t317) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700089308b); };
            c.cmp(t317, c.r64(2), 64);
            if c.cc_a() { blk = 341; continue; }
            blk = 339; continue;
        }
        339 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089309a, 0x7000893095);
        }
        340 => {
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089309a); };
            c.w64(9, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008930a1); };
            c.w64(15, t319);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008930a5); };
            c.w64(12, t320);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            blk = 342; continue;
        }
        341 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008930b2); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 342; continue;
        }
        342 => {
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008930bc); };
            c.w64(11, t321);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008930c3); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008930c7); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008930cc); }
            blk = 343; continue;
        }
        343 => {
            c.w64(2, c.r64(0));
            blk = 128; continue;
        }
        344 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008930d9); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 346; continue; }
            blk = 345; continue;
        }
        345 => {
            c.w64(11, c.r64(12));
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008930e6); }
            c.w64(12, c.r64(11));
            blk = 347; continue;
        }
        346 => {
            c.w64(11, c.r64(12));
            let r322 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r322);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 347; continue;
        }
        347 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 349; continue; }
            blk = 348; continue;
        }
        348 => {
            if !c.movsd_rm(1, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893105); }
            blk = 350; continue;
        }
        349 => {
            c.w64(11, c.r64(2));
            let r323 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r323);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(11) as u32 as i32 as i64));
            blk = 350; continue;
        }
        350 => {
            c.arith_sd(1, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(11, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 1) { return RegfileExit::fault(0x700089312d); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 356; continue; }
            blk = 351; continue;
        }
        351 => {
            if c.cc_ne() { blk = 356; continue; }
            blk = 352; continue;
        }
        352 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 354; continue; }
            blk = 353; continue;
        }
        353 => {
            c.movq_rx(0, 1);
            if let Some(sr) = c.shr(c.r64(0), 0x20u64, 64) { c.w64(0, sr); }
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_l() { blk = 356; continue; }
            blk = 354; continue;
        }
        354 => {
            c.w64(0, c.r64(11));
            let r324 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r324);
            if c.cc_o() { blk = 356; continue; }
            blk = 355; continue;
        }
        355 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 361; continue;
        }
        356 => {
            let Some(t325) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089315e); };
            c.w64(11, t325);
            let Some(t326) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000893165); };
            c.w64(0, t326);
            c.w64(6, c.r64(0).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x700089316c); }
            let Some(t327) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000893170); };
            c.w64(0, t327);
            let Some(t328) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000893177); };
            c.cmp(t328, c.r64(6), 64);
            if c.cc_a() { blk = 359; continue; }
            blk = 357; continue;
        }
        357 => {
            c.w32(2, 0xcu64);
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000893189, 0x7000893184);
        }
        358 => {
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000893189); };
            c.w64(8, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089318d); };
            c.w64(15, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000893191); };
            c.w64(2, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000893195); };
            c.w64(7, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000893199); };
            c.w64(12, t333);
            c.w32(3, 0);
            c.logic(0, 32);
            c.w32(1, 0x1u64);
            let Some(t334) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008931a4); };
            c.w64(9, t334);
            blk = 360; continue;
        }
        359 => {
            if !c.st64(c.r64(11), c.r64(6)) { return RegfileExit::fault(0x70008931ad); }
            let Some(t335) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008931b0); };
            c.w64(11, t335);
            let r336 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r336);
            c.w64(0, c.r64(11));
            blk = 360; continue;
        }
        360 => {
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008931bb); };
            c.w64(11, t337);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008931c2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008931c6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008931cb); }
            blk = 361; continue;
        }
        361 => {
            c.w64(11, c.r64(0));
            blk = 143; continue;
        }
        362 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x70008931d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(3)) { return RegfileExit::fault(0x70008931dc); }
            return c.call(0x70007febf4, 0x70008931e5, 0x70008931e0);
        }
        363 => {
            c.w64(9, c.r64(0));
            let Some(t338) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008931e8); };
            c.w64(7, t338);
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008931ec); };
            c.w64(12, t339);
            c.w32(1, 0x1u64);
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008931f5); };
            c.w64(11, t340);
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008931f9); };
            c.w64(8, t341);
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008931fd); };
            c.w64(3, t342);
            blk = 203; continue;
        }
        364 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700089320b); }
            return c.call(0x70007febf4, 0x7000893214, 0x700089320f);
        }
        365 => {
            c.w64(3, c.r64(0));
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000893217); };
            c.w64(15, t343);
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089321b); };
            c.w64(0, t344);
            let Some(t345) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089321f); };
            c.w64(12, t345);
            c.w32(1, 0x1u64);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000893228); };
            c.w64(11, t346);
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089322c); };
            c.w64(9, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000893230); };
            c.w64(8, t348);
            blk = 206; continue;
        }
        366 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000893239); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x700089323d); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000893241); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febf4, 0x700089324d, 0x7000893248);
        }
        367 => {
            c.w64(9, c.r64(0));
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000893250); };
            c.w64(12, t349);
            let Some(t350) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000893254); };
            c.w64(11, t350);
            let Some(t351) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000893258); };
            c.w64(8, t351);
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089325c); };
            c.w64(0, t352);
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000893260); };
            c.w64(6, t353);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000893264); };
            c.w64(7, t354);
            blk = 232; continue;
        }
        368 => {
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000893272); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(6)) { return RegfileExit::fault(0x7000893276); }
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x7000893282, 0x700089327d);
        }
        369 => {
            c.w64(9, c.r64(0));
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000893285); };
            c.w64(12, t355);
            c.w32(1, 0x1u64);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089328e); };
            c.w64(11, t356);
            let Some(t357) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000893292); };
            c.w64(3, t357);
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000893296); };
            c.w64(8, t358);
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089329a); };
            c.w64(2, t359);
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089329e); };
            c.w64(6, t360);
            blk = 235; continue;
        }
        370 => {
            return RegfileExit::fault(0x70008932a7);
        }
        371 => {
            let Some(t361) = c.ld32(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008932a8); };
            c.cmp(t361, 0x0u64, 32);
            if c.cc_ne() { blk = 373; continue; }
            blk = 372; continue;
        }
        372 => {
            let Some(t362) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008932ae); };
            c.w64(9, t362);
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008932b2); }
            blk = 374; continue;
        }
        373 => {
            let Some(t363) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008932ba); };
            c.w64(9, t363);
            let r364 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r364);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 374; continue;
        }
        374 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(9, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 1) { return RegfileExit::fault(0x70008932ea); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 380; continue; }
            blk = 375; continue;
        }
        375 => {
            if c.cc_ne() { blk = 380; continue; }
            blk = 376; continue;
        }
        376 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 378; continue; }
            blk = 377; continue;
        }
        377 => {
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 380; continue; }
            blk = 378; continue;
        }
        378 => {
            c.w64(11, c.r64(9));
            let r365 = c.add(c.r32(11), c.r32(9), 32);
            c.w32(11, r365);
            if c.cc_o() { blk = 380; continue; }
            blk = 379; continue;
        }
        379 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(9));
            blk = 385; continue;
        }
        380 => {
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089331c); };
            c.w64(9, t366);
            let Some(t367) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000893323); };
            c.w64(11, t367);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700089332a); };
            c.w64(15, t368);
            let Some(t369) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000893331); };
            c.cmp(t369, c.r64(12), 64);
            if c.cc_a() { blk = 383; continue; }
            blk = 381; continue;
        }
        381 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000893340, 0x700089333b);
        }
        382 => {
            blk = 384; continue;
        }
        383 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x7000893342); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 384; continue;
        }
        384 => {
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089334c); };
            c.w64(9, t370);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000893353); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x7000893357); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089335c); }
            blk = 385; continue;
        }
        385 => {
            c.w64(9, c.r64(0));
            blk = 296; continue;
        }
        386 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000893369); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 388; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000893373); };
            c.w64(1, t371);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893377); }
            blk = 389; continue;
        }
        388 => {
            let Some(t372) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089337e); };
            c.w64(1, t372);
            let r373 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r373);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(1) as u32 as i32 as i64));
            blk = 389; continue;
        }
        389 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(11, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 1) { return RegfileExit::fault(0x70008933ac); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 395; continue; }
            blk = 390; continue;
        }
        390 => {
            if c.cc_ne() { blk = 395; continue; }
            blk = 391; continue;
        }
        391 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 393; continue; }
            blk = 392; continue;
        }
        392 => {
            c.movq_rx(12, 1);
            if let Some(sr) = c.shr(c.r64(12), 0x20u64, 64) { c.w64(12, sr); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_l() { blk = 395; continue; }
            blk = 393; continue;
        }
        393 => {
            c.w64(12, c.r64(11));
            let r374 = c.add(c.r32(12), c.r32(11), 32);
            c.w32(12, r374);
            if c.cc_o() { blk = 395; continue; }
            blk = 394; continue;
        }
        394 => {
            c.w64(3, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(3));
            blk = 400; continue;
        }
        395 => {
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008933de); };
            c.w64(11, t375);
            let Some(t376) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008933e5); };
            c.w64(12, t376);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t377) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008933ed); };
            c.w64(0, t377);
            let Some(t378) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008933f4); };
            c.cmp(t378, c.r64(15), 64);
            if c.cc_a() { blk = 398; continue; }
            blk = 396; continue;
        }
        396 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000893403, 0x70008933fe);
        }
        397 => {
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000893403); };
            c.w64(9, t379);
            blk = 399; continue;
        }
        398 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000893409); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 399; continue;
        }
        399 => {
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000893414); };
            c.w64(11, t380);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700089341b); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x700089341f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000893424); }
            blk = 400; continue;
        }
        400 => {
            c.w64(3, c.r64(0));
            blk = 280; continue;
        }
        401 => {
            c.w32(9, c.r32(9));
            let r381 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r381);
            c.w64(2, 0xfffffffffffc0000u64);
            let r382 = c.r64(2) & c.r64(9);
            c.w64(2, r382);
            c.logic(r382, 64);
            let Some(t383) = c.ld8(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000893441); };
            c.logic(t383 & 0x2u64, 8);
            if c.cc_e() { blk = 264; continue; }
            blk = 402; continue;
        }
        402 => {
            c.w64(1, c.r64(3).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700089344f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000893450); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x700089345d, 0x7000893458);
        }
        403 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700089345d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700089345f); }
            blk = 264; continue;
        }
        404 => {
            c.w32(2, c.r32(2));
            let r384 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r384);
            c.w64(6, 0xfffffffffffc0000u64);
            let r385 = c.r64(6) & c.r64(2);
            c.w64(6, r385);
            c.logic(r385, 64);
            let Some(t386) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000893474); };
            c.logic(t386 & 0x2u64, 8);
            if c.cc_e() { blk = 217; continue; }
            blk = 405; continue;
        }
        405 => {
            c.w64(7, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000893482); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000893483); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000893490, 0x700089348b);
        }
        406 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000893490); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000893492); }
            blk = 217; continue;
        }
        407 => {
            return RegfileExit::jump(0x700089349c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008934b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008934b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008934b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008934b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008934ba); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008934bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008934c9); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008934d7); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008934de); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008934f7); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893504); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700089350b); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 15; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000893512); };
            c.w64(8, t11);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089351c); };
            c.w32(9, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000893520); };
            c.cmp(t13, c.r32(9), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x7000893529); };
            c.w64(9, t14);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893533); };
            c.w32(11, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000893537); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000893540); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000893542); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000893544); };
            c.w64(2, t17);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700089354b); }
            c.w64(0, c.r64(7));
            let Some(t18) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x700089354f); };
            c.w64(1, t18);
            return c.call(0x7000893814, 0x7000893558, 0x7000893553);
        }
        10 => {
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000893558); };
            c.w64(1, t19);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700089355f); }
            c.cmp(c.r64(1), 0x4u64, 64);
            if c.cc_g() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return c.ret(0x28, 0x7000893566);
        }
        12 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000893569); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000893570); }
            return c.ret(0x0, 0x7000893572);
        }
        13 => {
            return RegfileExit::fault(0x7000893573);
        }
        14 => {
            return RegfileExit::fault(0x7000893574);
        }
        15 => {
            return RegfileExit::fault(0x7000893575);
        }
        16 => {
            return RegfileExit::fault(0x7000893576);
        }
        17 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000893577); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x700089357b); };
            c.w64(3, t20);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000893589, 0x7000893584);
        }
        18 => {
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000893589); };
            c.w64(6, t21);
            blk = 1; continue;
        }
        19 => {
            return RegfileExit::jump(0x7000893594);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008935b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008935b4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008935b8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008935b9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008935ba); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008935bf); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 30; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008935c9); };
            c.w64(7, t2);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008935d7); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008935de); };
            c.w32(8, t5);
            let r6 = c.sub(c.r32(8), 0xbau64, 32);
            c.w32(8, r6);
            c.cmp(c.r32(8), 0x78eu64, 32);
            if c.cc_a() { blk = 29; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008935f7); };
            c.w64(3, t7);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893604); };
            c.w32(8, t8);
            let r9 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r9);
            let Some(t10) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700089360b); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t11) = c.ld64(c.r64(5).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000893616); };
            c.w64(8, t11);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.w32(11, c.r32(8));
            c.cmp(c.r32(9), 0x0u64, 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089363b); };
            c.w32(15, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089363f); };
            c.cmp(t13, c.r32(15), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t14) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x700089364c); };
            c.w64(15, t14);
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t15) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893656); };
            c.w32(0, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089365a); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_add(0x38)) else { return RegfileExit::fault(0x7000893667); };
            c.w64(0, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089366b); };
            c.cmp(t18, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 16; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t19) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700089367a); };
            c.cmp(t19, c.r32(0), 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893683); };
            c.w32(2, t20);
            let r21 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r21);
            let Some(t22) = c.ld8(c.r64(2).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000893689); };
            c.logic(t22 & 0x10u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t23) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089368f); };
            c.cmp(t23, c.r32(2), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t24) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089369c); };
            c.cmp(t24, 0x41u64, 16);
            if c.cc_e() { blk = 32; continue; }
            blk = 15; continue;
        }
        15 => {
            blk = 17; continue;
        }
        16 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008936ae); };
            c.w64(0, t25);
            blk = 18; continue;
        }
        18 => {
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008936b5); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008936bc); }
            c.cmp(c.r64(1), 0x5u64, 64);
            if c.cc_g() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            return c.ret(0x30, 0x70008936c3);
        }
        20 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008936c6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008936cd); }
            return c.ret(0x0, 0x70008936cf);
        }
        21 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 36; continue; }
            blk = 22; continue;
        }
        22 => {
            let r27 = c.add(c.r32(11), 0x2u64, 32);
            c.w32(11, r27);
            if c.cc_o() { blk = 36; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            blk = 24; continue;
        }
        24 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008936df); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008936e1); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008936e3); };
            c.w64(2, t28);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008936ea); }
            c.w64(0, c.r64(7));
            let Some(t29) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008936ee); };
            c.w64(1, t29);
            return c.call(0x7000893814, 0x70008936f7, 0x70008936f2);
        }
        25 => {
            blk = 18; continue;
        }
        26 => {
            return RegfileExit::fault(0x70008936f9);
        }
        27 => {
            return RegfileExit::fault(0x70008936fa);
        }
        28 => {
            return RegfileExit::fault(0x70008936fb);
        }
        29 => {
            return RegfileExit::fault(0x70008936fc);
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008936fd); }
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000893701); };
            c.w64(3, t30);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700089370f, 0x700089370a);
        }
        31 => {
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089370f); };
            c.w64(6, t31);
            blk = 1; continue;
        }
        32 => {
            let Some(t32) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893718); };
            c.logic(t32 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 21; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 17; continue;
        }
        34 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893723); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 17; continue; }
            blk = 35; continue;
        }
        35 => {
            blk = 21; continue;
        }
        36 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000893738); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893742); }
            blk = 39; continue;
        }
        38 => {
            let r33 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r33);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 39; continue;
        }
        39 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 1) { return RegfileExit::fault(0x7000893776); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 45; continue; }
            blk = 40; continue;
        }
        40 => {
            if c.cc_ne() { blk = 45; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.w64(9, c.r64(8));
            let r34 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r34);
            if c.cc_o() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(8, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 50; continue;
        }
        45 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008937a8); };
            c.w64(8, t35);
            let Some(t36) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008937af); };
            c.w64(9, t36);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008937b6); };
            c.w64(12, t37);
            let Some(t38) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008937bd); };
            c.cmp(t38, c.r64(11), 64);
            if c.cc_a() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008937cd, 0x70008937c8);
        }
        47 => {
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008937cd); };
            c.w64(6, t39);
            let Some(t40) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008937d1); };
            c.w64(7, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008937d5); };
            c.w64(3, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_add(0x30)) else { return RegfileExit::fault(0x70008937d9); };
            c.w64(15, t42);
            blk = 49; continue;
        }
        48 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008937df); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 49; continue;
        }
        49 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008937e9); };
            c.w64(8, t43);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008937f0); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x70008937f4); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008937f9); }
            blk = 50; continue;
        }
        50 => {
            c.w64(8, c.r64(0));
            blk = 24; continue;
        }
        51 => {
            return RegfileExit::jump(0x7000893808);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000893814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000893814); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000893818); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700089381e); };
            c.w64(8, t1);
            c.logic(c.r8(8) & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x700089382e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000893832); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000893836); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700089383a); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x700089383e); }
            let Some(t2) = c.ld64(c.r64(5).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000893842); };
            c.w64(0, t2);
            blk = 2; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000893848); };
            c.w64(8, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089384c); };
            c.w64(7, t4);
            blk = 2; continue;
        }
        2 => {
            c.w32(2, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x700089385c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x7000893860); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700089386a); }
            let Some(t5) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089386f); };
            c.cmp(t5, 0x0u64, 32);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893875); }
            blk = 8; continue;
        }
        5 => {
            c.w64(3, c.r64(8));
            let r6 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r6);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        6 => {
            let Some(t7) = c.ld32(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700089388c); };
            c.cmp(t7, 0x0u64, 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(3, c.r64(0));
            let r8 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r8);
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893897); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 256; continue;
        }
        10 => {
            c.cmp(c.r32(0), c.r32(8), 32);
            if c.cc_ge() { blk = 256; continue; }
            blk = 11; continue;
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008938b9); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 282; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008938c7); };
            c.w32(3, t9);
            let r10 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r10);
            let Some(t11) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008938cd); };
            c.w32(8, t11);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 282; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 275; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008938f4); };
            c.w32(9, t12);
            let r13 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r13);
            let Some(t14) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008938fb); };
            c.w32(9, t14);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000893907); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(2) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(1, 0x1fffffffffffffu64);
            let r15 = c.add(c.r64(1), c.r64(2), 64);
            c.w64(1, r15);
            c.w64(6, 0x3ffffffffffffeu64);
            c.cmp(c.r64(1), c.r64(6), 64);
            if c.cc_be() { blk = 106; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000893942); };
            c.w64(1, t16);
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000893946); };
            c.w64(6, t17);
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 29; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 261; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t18) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089395c); };
            c.w32(2, t18);
            c.logic(c.r32(2) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 22; continue;
        }
        22 => {
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 282; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 282; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(9, c.r64(0));
            blk = 30; continue;
        }
        27 => {
            let Some(t19) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089398d); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            blk = 30; continue;
        }
        28 => {
            if let Some(sr) = c.shr(c.r64(2), 0x2u64, 64) { c.w64(2, sr); }
            let r21 = c.r64(2) & 0xffffffu64;
            c.w64(2, r21);
            c.logic(r21, 64);
            blk = 107; continue;
        }
        29 => {
            c.w64(9, c.r64(0));
            blk = 30; continue;
        }
        30 => {
            c.w64(2, c.r64(7));
            blk = 32; continue;
        }
        31 => {
            let Some(t22) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008939ae); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008939b4); };
            c.w32(8, t24);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 59; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t25) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008939c6); };
            c.w32(1, t25);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008939d5); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            c.logic(c.r32(1) & 0xffc00u64, 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 35; continue;
        }
        35 => {
            let r28 = c.shr(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r28);
            let r29 = c.r32(1) & 0x3ffu64;
            c.w32(1, r29);
            c.logic(r29, 32);
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_be() { blk = 48; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t30) = c.ld16(c.r64(2).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008939f9); };
            c.w32(6, (t30 as u16 as i16 as i64 as u64));
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008939fd); };
            c.w32(7, t31);
            let r32 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r32);
            let r33 = c.sub(c.r32(6), 0x1u64, 32);
            c.w32(6, r33);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x7000893a07); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 38; continue;
        }
        37 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x7000893a10); }
            blk = 38; continue;
        }
        38 => {
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000893a14); };
            c.w64(0, t34);
            let r35 = c.sub(c.r32(0), c.r32(15), 32);
            c.w32(0, r35);
            let r36 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r36);
            let r37 = c.add(c.r32(0), c.r32(15), 32);
            c.w32(0, r37);
            c.w32(12, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t38) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000893a27); };
            c.w32(12, t38);
            let r39 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r39);
            let r40 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.r32(12) & 0x3ffu64;
            c.w32(12, r41);
            c.logic(r41, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t42) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893a41); };
            c.w32(12, t42);
            let r43 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r43);
            let Some(t44) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893a49); };
            c.w32(12, t44);
            let r45 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r45);
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_be() { blk = 40; continue; }
            blk = 39; continue;
        }
        39 => {
            c.w32(15, c.r64(0).wrapping_add(0x1));
            let Some(t46) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000893a5b); };
            c.w64(0, t46);
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r32(15), c.r32(0), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 41; continue;
        }
        41 => {
            blk = 43; continue;
        }
        42 => {
            let r47 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r47);
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_l() { blk = 70; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w32(12, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000893a7a); };
            c.w32(12, t48);
            let r49 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r49);
            let r50 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r50);
            let r51 = c.r32(12) & 0x3ffu64;
            c.w32(12, r51);
            c.logic(r51, 32);
            c.w32(0, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t52) = c.ld32(c.r64(2).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893a94); };
            c.w32(0, t52);
            let r53 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r53);
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893a9b); };
            c.w32(11, t54);
            let r55 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r55);
            c.cmp(c.r32(7), c.r32(11), 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r32(0), c.r32(9), 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_be() { blk = 70; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 240; continue;
        }
        48 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let r56 = c.imul2(c.r64(11), 0x3u64, 64);
            c.w64(11, r56);
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 70; continue; }
            blk = 49; continue;
        }
        49 => {
            let r57 = c.sub(c.r64(11), 0x3u64, 64);
            c.w64(11, r57);
            let Some(t58) = c.ld32(c.r64(2).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893ad4); };
            c.cmp(t58, c.r32(9), 32);
            if c.cc_e() { blk = 240; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_ne() { blk = 49; continue; }
            blk = 51; continue;
        }
        51 => {
            blk = 70; continue;
        }
        52 => {
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893aea); };
            c.w32(11, t59);
            let r60 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r60);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000893af7); };
            c.w64(11, t61);
            blk = 54; continue;
        }
        54 => {
            let Some(t62) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893afe); };
            c.w32(12, t62);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r63 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r63);
            let Some(t64) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893b0c); };
            c.w32(15, t64);
            let r65 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r65);
            let r66 = c.r64(15) & c.r64(12);
            c.w64(15, r66);
            c.logic(r66, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r67 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r67);
            let r68 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r68);
            c.w64(2, c.r64(12));
            let r69 = c.r64(2) & c.r64(15);
            c.w64(2, r69);
            c.logic(r69, 64);
            c.w64(15, c.r64(2));
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893b30); };
            c.w32(2, t70);
            let Some(t71) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000893b35); };
            c.cmp(t71, c.r32(2), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_e() { blk = 240; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 55; continue;
        }
        59 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 269; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t72) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000893b5a); };
            c.logic(t72 & 0x24u64, 8);
            if c.cc_ne() { blk = 274; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t73) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893b64); };
            c.w32(11, t73);
            let r74 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r74);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000893b71); };
            c.w64(11, t75);
            blk = 63; continue;
        }
        63 => {
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893b78); };
            c.w32(12, t76);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r77 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r77);
            let Some(t78) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893b86); };
            c.w32(15, t78);
            let r79 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r79);
            let r80 = c.r64(15) & c.r64(12);
            c.w64(15, r80);
            c.logic(r80, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 65; continue;
        }
        64 => {
            let r81 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r81);
            let r82 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r82);
            c.w64(2, c.r64(12));
            let r83 = c.r64(2) & c.r64(15);
            c.w64(2, r83);
            c.logic(r83, 64);
            c.w64(15, c.r64(2));
            blk = 65; continue;
        }
        65 => {
            c.w64(2, c.r64(15).wrapping_add(0x5));
            let Some(t84) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893ba9); };
            c.w32(1, t84);
            let r85 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r85);
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000893bb1); };
            c.cmp(t86, c.r32(1), 32);
            if c.cc_e() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t87) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000893bba); };
            c.cmp(t87, c.r32(1), 32);
            if c.cc_e() { blk = 64; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t88) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893bc3); };
            c.cmp(t88, c.r32(9), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t89) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893bc9); };
            c.w32(11, t89);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 302; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t90) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000893bdc); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000893be0); };
            c.cmp(t91, c.r32(11), 32);
            if c.cc_ne() { blk = 240; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 101; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t92) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893bfa); };
            c.w32(8, t92);
            let r93 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r93);
            let Some(t94) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893c01); };
            c.w32(8, t94);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 101; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893c11); };
            c.w32(11, t95);
            let r96 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r96);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 101; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 101; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 76; continue;
        }
        76 => {
            c.w64(15, c.r64(11));
            let r97 = c.r32(15) & 0x7u64;
            c.w32(15, r97);
            c.logic(r97, 32);
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_l() { blk = 81; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r32(15), 0x2u64, 32);
            if c.cc_e() { blk = 89; continue; }
            blk = 78; continue;
        }
        78 => {
            c.cmp(c.r32(15), 0x3u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 79; continue;
        }
        79 => {
            c.cmp(c.r32(15), 0x5u64, 32);
            if c.cc_e() { blk = 87; continue; }
            blk = 80; continue;
        }
        80 => {
            blk = 263; continue;
        }
        81 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(15), 0x1u64, 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            blk = 263; continue;
        }
        84 => {
            let Some(t98) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893c7e); };
            c.w32(11, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000893c83); };
            c.cmp(t99, c.r32(11), 32);
            if c.cc_ne() { blk = 264; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t100) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000893c90); };
            c.w32(12, t100);
            let r101 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r101);
            let Some(t102) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893c98); };
            c.w32(11, t102);
            let r103 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r103);
            let Some(t104) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893ca0); };
            c.w32(11, t104);
            blk = 76; continue;
        }
        86 => {
            let Some(t105) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893ca7); };
            c.w32(11, t105);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r106 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r106);
            let Some(t107) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000893cb5); };
            c.w32(12, t107);
            let r108 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r108);
            let Some(t109) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893cbd); };
            c.w32(11, t109);
            let r110 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r110);
            let Some(t111) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893cc5); };
            c.w32(11, t111);
            blk = 76; continue;
        }
        87 => {
            let Some(t112) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000893ccf); };
            c.w32(12, t112);
            let r113 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r113);
            let Some(t114) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893cd7); };
            c.w32(11, t114);
            let r115 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r115);
            let Some(t116) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893cdf); };
            c.w32(11, t116);
            blk = 76; continue;
        }
        88 => {
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 91; continue;
        }
        89 => {
            c.w32(15, 0x1u64);
            blk = 91; continue;
        }
        90 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 265; continue; }
            blk = 92; continue;
        }
        92 => {
            let r117 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r117);
            blk = 93; continue;
        }
        93 => {
            let r118 = c.r32(11) & 0x8u64;
            c.w32(11, r118);
            c.logic(r118, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t119) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x7000893d19); };
            c.w32(8, t119);
            blk = 96; continue;
        }
        95 => {
            let Some(t120) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000893d20); };
            c.w32(8, t120);
            blk = 96; continue;
        }
        96 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 273; continue; }
            blk = 97; continue;
        }
        97 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 272; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 271; continue; }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 101; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 270; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000893d53); };
            c.w64(0, t121);
            let Some(t122) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893d57); };
            c.w32(8, t122);
            let Some(t123) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000893d5b); };
            c.cmp(t123, c.r32(8), 32);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t124) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893d64); };
            c.logic(t124 & 0x1u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t125) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000893d6e); };
            c.w32(2, t125);
            let r126 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r126);
            let Some(t127) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000893d74); };
            c.cmp(t127, c.r32(2), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 104; continue;
        }
        104 => {
            blk = 31; continue;
        }
        105 => {
            c.w64(9, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(2, c.r64(9));
            blk = 106; continue;
        }
        106 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000893d8f); };
            c.w64(6, t128);
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000893d93); };
            c.w64(1, t129);
            blk = 107; continue;
        }
        107 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x7000893d97); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            blk = 109; continue;
        }
        108 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000893da3); };
            c.w32(3, t130);
            let r131 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r131);
            let Some(t132) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893daa); };
            c.w32(9, t132);
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 282; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t133) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000893dbc); };
            c.w32(9, t133);
            let r134 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r134);
            let r135 = c.r32(9) & 0x3fu64;
            c.w32(9, r135);
            c.logic(r135, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 145; continue; }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 129; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 121; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 118; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 115; continue;
        }
        115 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 116; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x28u64, 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 117; continue;
        }
        117 => {
            blk = 282; continue;
        }
        118 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 282; continue;
        }
        121 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_l() { blk = 126; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 125; continue;
        }
        125 => {
            blk = 282; continue;
        }
        126 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 127; continue;
        }
        127 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 128; continue;
        }
        128 => {
            blk = 282; continue;
        }
        129 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 138; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 135; continue; }
            blk = 131; continue;
        }
        131 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 134; continue;
        }
        134 => {
            blk = 282; continue;
        }
        135 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 137; continue;
        }
        137 => {
            blk = 282; continue;
        }
        138 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 282; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 282; continue;
        }
        145 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_l() { blk = 163; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_l() { blk = 155; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_l() { blk = 152; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 150; continue;
        }
        150 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 151; continue;
        }
        151 => {
            blk = 282; continue;
        }
        152 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 189; continue; }
            blk = 153; continue;
        }
        153 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 154; continue;
        }
        154 => {
            blk = 282; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_l() { blk = 160; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 190; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r32(9), 0xfu64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 159; continue;
        }
        159 => {
            blk = 282; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 162; continue;
        }
        162 => {
            blk = 282; continue;
        }
        163 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 172; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 169; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 282; continue;
        }
        169 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 170; continue;
        }
        170 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 171; continue;
        }
        171 => {
            blk = 282; continue;
        }
        172 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 176; continue; }
            blk = 173; continue;
        }
        173 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 174; continue;
        }
        174 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 175; continue;
        }
        175 => {
            blk = 282; continue;
        }
        176 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 184; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 282; continue;
        }
        179 => {
            blk = 184; continue;
        }
        180 => {
            let Some(t136) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000893fe5); };
            c.w32(8, t136);
            let r137 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r137);
            let Some(t138) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000893fec); };
            c.w32(9, t138);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_ae() { blk = 185; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t139) = c.ld64(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894005); };
            c.cmp(t139, c.r64(9), 64);
            if c.cc_e() { blk = 203; continue; }
            blk = 182; continue;
        }
        182 => {
            blk = 240; continue;
        }
        183 => {
            let Some(t140) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894015); };
            c.w32(9, t140);
            let r141 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r141);
            let Some(t142) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089401c); };
            c.w32(9, t142);
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 240; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t143) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894029); };
            c.w32(8, t143);
            let r144 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r144);
            let Some(t145) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000894030); };
            c.w32(9, t145);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 187; continue; }
            blk = 185; continue;
        }
        185 => {
            c.w32(8, 0xfffffffeu64);
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 282; continue; }
            blk = 186; continue;
        }
        186 => {
            blk = 203; continue;
        }
        187 => {
            let Some(t146) = c.ld32(c.r64(8).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894053); };
            c.w32(8, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000894058); };
            c.cmp(t147, c.r32(8), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 188; continue;
        }
        188 => {
            blk = 240; continue;
        }
        189 => {
            let Some(t148) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700089406a); };
            c.w32(9, t148);
            let r149 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r149);
            let Some(t150) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894071); };
            c.w32(9, t150);
            c.cmp(c.r64(2), c.r64(9), 64);
            if c.cc_b() { blk = 240; continue; }
            blk = 190; continue;
        }
        190 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700089407e); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 282; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t151) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894091); };
            c.w32(8, t151);
            let r152 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r152);
            let Some(t153) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000894098); };
            c.w32(9, t153);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r154 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r154);
            c.w32(12, c.r32(2));
            c.w64(10, c.r64(4));
            let r155 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r155);
            let r156 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r156);
            c.logic(r156, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008940b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008940b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008940bd); }
            c.w64(2, c.r64(12));
            c.w64(1, c.r64(13));
            c.w64(15, c.r64(1));
            let Some(t157) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x70008940ca); };
            c.w64(0, t157);
            c.w64(10, 0x70008940d8u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008940d8); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008940dc); }
            return c.call(c.r64(0), 0x70008940e2, 0x70008940e0);
        }
        192 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008940e2); }
            let Some(t158) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008940ea); };
            c.w64(4, t158);
            c.w32(8, c.r32(0));
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008940f2); };
            let r160 = c.r64(8) & t159;
            c.w64(8, r160);
            c.logic(r160, 64);
            c.w32(9, 0);
            c.logic(0, 32);
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008940f9); };
            c.w64(11, t161);
            blk = 194; continue;
        }
        193 => {
            let r162 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r162);
            let r163 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000894106); };
            c.w64(12, t164);
            let r165 = c.r64(12) & c.r64(8);
            c.w64(12, r165);
            c.logic(r165, 64);
            c.w64(8, c.r64(12));
            blk = 194; continue;
        }
        194 => {
            c.w64(12, c.r64(8).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t166) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894115); };
            c.w32(12, t166);
            let r167 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r167);
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700089411d); };
            c.cmp(t168, c.r32(12), 32);
            if c.cc_e() { blk = 202; continue; }
            blk = 195; continue;
        }
        195 => {
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700089412c); };
            c.cmp(t169, c.r32(12), 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 197; continue;
        }
        197 => {
            c.xorpd_rr(0, 0);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000894139); };
            c.cvtsi2sd(0, (t170 as i64));
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700089413f); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 193; continue; }
            blk = 198; continue;
        }
        198 => {
            if c.cc_e() { blk = 240; continue; }
            blk = 199; continue;
        }
        199 => {
            blk = 193; continue;
        }
        200 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089415a); };
            c.cmp(c.r64(12), t171, 64);
            if c.cc_e() { blk = 240; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 193; continue;
        }
        202 => {
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000894166); };
            c.w64(0, t172);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089416a); };
            c.w64(3, t173);
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089416e); };
            c.w64(2, t174);
            blk = 203; continue;
        }
        203 => {
            let Some(t175) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000894172); };
            c.w32(8, t175);
            let r176 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r176);
            let Some(t177) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894179); };
            c.w64(1, t177);
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700089417d); };
            c.w64(7, t178);
            let Some(t179) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000894181); };
            c.w64(6, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000894185); };
            c.cmp(t180, c.r32(8), 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 204; continue;
        }
        204 => {
            blk = 108; continue;
        }
        205 => {
            let Some(t181) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894197); };
            c.w32(9, t181);
            let r182 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r182);
            let Some(t183) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700089419e); };
            c.logic(t183 & 0x4u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 206; continue;
        }
        206 => {
            c.w64(9, c.r64(2));
            let Some(t184) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008941ac); };
            c.cmp(t184, c.r64(9), 64);
            if c.cc_a() { blk = 240; continue; }
            blk = 207; continue;
        }
        207 => {
            blk = 241; continue;
        }
        208 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008941bb); };
            c.w32(11, t185);
            let r186 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r186);
            let Some(t187) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008941c2); };
            c.w32(12, t187);
            c.w64(9, c.r64(2));
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 237; continue; }
            blk = 209; continue;
        }
        209 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 241; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t188) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008941dd); };
            c.w64(11, t188);
            let Some(t189) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008941e1); };
            c.w64(12, t189);
            let Some(t190) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008941e5); };
            c.logic(t190 & 0x1u64, 8);
            if c.cc_ne() { blk = 213; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t191) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008941ec); };
            c.w64(15, t191);
            let r192 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r192);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 241; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t193) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008941fc); };
            c.w64(8, t193);
            blk = 239; continue;
        }
        213 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 241; continue; }
            blk = 214; continue;
        }
        214 => {
            let Some(t194) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089420e); };
            c.w32(8, t194);
            let r195 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r195);
            let Some(t196) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000894215); };
            c.w32(8, t196);
            let r197 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r197);
            let r198 = c.r32(8) & 0x3fu64;
            c.w32(8, r198);
            c.logic(r198, 32);
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_l() { blk = 224; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_l() { blk = 220; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(8), 0x24u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(8), 0x25u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 218; continue;
        }
        218 => {
            c.cmp(c.r32(8), 0x26u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 219; continue;
        }
        219 => {
            blk = 258; continue;
        }
        220 => {
            c.cmp(c.r32(8), 0x21u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 221; continue;
        }
        221 => {
            c.cmp(c.r32(8), 0x22u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 222; continue;
        }
        222 => {
            c.cmp(c.r32(8), 0x23u64, 32);
            if c.cc_e() { blk = 235; continue; }
            blk = 223; continue;
        }
        223 => {
            blk = 258; continue;
        }
        224 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_l() { blk = 229; continue; }
            blk = 225; continue;
        }
        225 => {
            c.cmp(c.r32(8), 0x1eu64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r32(8), 0x1fu64, 32);
            if c.cc_e() { blk = 233; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_e() { blk = 234; continue; }
            blk = 228; continue;
        }
        228 => {
            blk = 258; continue;
        }
        229 => {
            c.cmp(c.r32(8), 0x1cu64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            c.cmp(c.r32(8), 0x1du64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            blk = 258; continue;
        }
        232 => {
            c.w32(8, 0x1u64);
            blk = 236; continue;
        }
        233 => {
            c.w32(8, 0x2u64);
            blk = 236; continue;
        }
        234 => {
            c.w32(8, 0x4u64);
            blk = 236; continue;
        }
        235 => {
            c.w32(8, 0x8u64);
            blk = 236; continue;
        }
        236 => {
            let r199 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r199);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(8), 64);
            c.w64(8, c.r64(0));
            c.w64(0, c.r64(11));
            blk = 239; continue;
        }
        237 => {
            c.w64(10, c.r64(4));
            let r200 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r200);
            let r201 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r201);
            c.logic(r201, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008942d8); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t202) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x70008942e3); };
            c.w64(0, t202);
            c.w64(10, 0x70008942f1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008942f1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008942f5); }
            return c.call(c.r64(0), 0x70008942fb, 0x70008942f9);
        }
        238 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008942fb); }
            let Some(t203) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000894303); };
            c.w64(4, t203);
            c.w64(8, c.r64(0));
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089430b); };
            c.w64(0, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089430f); };
            c.w64(9, t205);
            blk = 239; continue;
        }
        239 => {
            c.w64(2, c.r64(9));
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ae() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089431b); };
            c.w64(8, t206);
            blk = 242; continue;
        }
        241 => {
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000894324); };
            c.w64(8, t207);
            blk = 242; continue;
        }
        242 => {
            let Some(t208) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700089432b); };
            c.cmp(t208, c.r32(8), 32);
            if c.cc_ne() { blk = 253; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894338); };
            c.w64(0, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089433c); };
            c.w64(3, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000894340); };
            c.w64(6, t211);
            return c.call(0x7000870914, 0x7000894349, 0x7000894344);
        }
        244 => {
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894349); };
            if !c.push64(t212) { return RegfileExit::fault(0x7000894349); }
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089434c); };
            if !c.push64(t213) { return RegfileExit::fault(0x700089434c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700089434f); }
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894350); };
            if !c.push64(t214) { return RegfileExit::fault(0x7000894350); }
            c.w32(0, 0x3u64);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894358); };
            c.w64(7, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089435c); };
            c.w64(6, t216);
            return c.call(0x70007f2274, 0x7000894365, 0x7000894360);
        }
        245 => {
            let Some(t217) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000894365); };
            c.cmp(t217, c.r32(0), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 252; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t218) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x7000894375); };
            c.cmp(t218, c.r32(0), 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t219) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089437e); };
            c.w32(8, t219);
            let r220 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r220);
            let Some(t221) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000894385); };
            c.logic(t221 & 0x10u64, 8);
            if c.cc_ne() { blk = 253; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t222) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700089438c); };
            c.cmp(t222, c.r32(8), 32);
            if c.cc_e() { blk = 286; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t223) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894399); };
            c.cmp(t223, 0x41u64, 16);
            if c.cc_e() { blk = 284; continue; }
            blk = 251; continue;
        }
        251 => {
            blk = 259; continue;
        }
        252 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 259; continue; }
            blk = 253; continue;
        }
        253 => {
            let Some(t224) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008943ad); };
            c.cmp(t224, 0x0u64, 32);
            if c.cc_e() { blk = 288; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008943b7); };
            c.w64(8, t225);
            let r226 = c.add(c.r32(8), 0x2u64, 32);
            c.w32(8, r226);
            if c.cc_o() { blk = 288; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w64(0, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        256 => {
            let Some(t227) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008943cd); };
            c.w64(0, t227);
            blk = 257; continue;
        }
        257 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008943d7); }
            return c.ret(0x18, 0x70008943d8);
        }
        258 => {
            return RegfileExit::fault(0x70008943db);
        }
        259 => {
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008943dc); };
            c.w64(0, t228);
            blk = 257; continue;
        }
        260 => {
            blk = 282; continue;
        }
        261 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 282; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t229) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008943f5); };
            c.w32(9, t229);
            let r230 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r230);
            blk = 30; continue;
        }
        263 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000894401); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000894405); }
            blk = 267; continue;
        }
        264 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700089440b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700089440f); }
            blk = 267; continue;
        }
        265 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000894415); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000894419); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 267; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t231) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000894423); };
            c.w64(12, t231);
            blk = 93; continue;
        }
        267 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700089442d); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700089442f); }
            let Some(t232) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x7000894430); };
            c.w64(3, t232);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x7000894444, 0x700089443f);
        }
        268 => {
            let r233 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r233);
            c.w64(8, c.r64(0));
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000894449); };
            c.w64(3, t234);
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700089444d); };
            c.w64(9, t235);
            blk = 96; continue;
        }
        269 => {
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000894456); };
            c.w64(0, t236);
            blk = 282; continue;
        }
        270 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089445c); };
            c.w64(0, t237);
            blk = 282; continue;
        }
        271 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000894462); };
            c.w64(0, t238);
            blk = 282; continue;
        }
        272 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000894468); };
            c.w64(0, t239);
            blk = 282; continue;
        }
        273 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089446e); };
            c.w64(0, t240);
            blk = 282; continue;
        }
        274 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000894474); };
            c.w64(0, t241);
            blk = 282; continue;
        }
        275 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700089447a); };
            c.w64(6, t242);
            return c.call(0x70008a1af4, 0x7000894483, 0x700089447e);
        }
        276 => {
            let Some(t243) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894483); };
            c.w32(8, t243);
            let Some(t244) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000894487); };
            c.cmp(t244, c.r32(8), 32);
            if c.cc_ne() { blk = 279; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t245) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894490); };
            c.logic(t245 & 0x1u64, 8);
            if c.cc_e() { blk = 280; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000894496); };
            c.w64(0, t246);
            blk = 241; continue;
        }
        279 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089449f); };
            c.w64(3, t247);
            blk = 280; continue;
        }
        280 => {
            c.w64(3, c.r64(0));
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008944a6); };
            c.w64(0, t248);
            let Some(t249) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008944aa); };
            c.w64(6, t249);
            return c.call(0x70008bced4, 0x70008944b3, 0x70008944ae);
        }
        281 => {
            c.w64(8, c.r64(0));
            blk = 242; continue;
        }
        282 => {
            let Some(t250) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008944bb); };
            if !c.push64(t250) { return RegfileExit::fault(0x70008944bb); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008944be); }
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x70008944bf); };
            c.w64(3, t251);
            let Some(t252) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008944c6); };
            c.w64(6, t252);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x70008944d7, 0x70008944d2);
        }
        283 => {
            c.w64(8, c.r64(0));
            blk = 242; continue;
        }
        284 => {
            let Some(t253) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008944df); };
            c.logic(t253 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 285; continue;
        }
        285 => {
            blk = 259; continue;
        }
        286 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008944f1); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 259; continue; }
            blk = 287; continue;
        }
        287 => {
            blk = 253; continue;
        }
        288 => {
            let Some(t254) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000894509); };
            c.cmp(t254, 0x0u64, 32);
            if c.cc_ne() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089450f); };
            c.w64(0, t255);
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894513); }
            blk = 291; continue;
        }
        290 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700089451a); };
            c.w64(0, t256);
            let r257 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r257);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 291; continue;
        }
        291 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(0));
            let cv = c.cvttsd2si32(c.xmm_u64(1)); c.w32(8, cv);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 1) { return RegfileExit::fault(0x7000894548); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 297; continue; }
            blk = 292; continue;
        }
        292 => {
            if c.cc_ne() { blk = 297; continue; }
            blk = 293; continue;
        }
        293 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 295; continue; }
            blk = 294; continue;
        }
        294 => {
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_l() { blk = 297; continue; }
            blk = 295; continue;
        }
        295 => {
            c.w64(9, c.r64(8));
            let r258 = c.add(c.r32(9), c.r32(8), 32);
            c.w32(9, r258);
            if c.cc_o() { blk = 297; continue; }
            blk = 296; continue;
        }
        296 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 1; continue;
        }
        297 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700089457a); };
            c.w64(8, t259);
            let Some(t260) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000894581); };
            c.w64(9, t260);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t261) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000894588); };
            c.w64(12, t261);
            let Some(t262) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700089458f); };
            c.cmp(t262, c.r64(11), 64);
            if c.cc_a() { blk = 300; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700089459f, 0x700089459a);
        }
        299 => {
            blk = 301; continue;
        }
        300 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x70008945a1); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 301; continue;
        }
        301 => {
            let Some(t263) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008945ab); };
            c.w64(8, t263);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008945b2); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008945b6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008945bb); }
            blk = 1; continue;
        }
        302 => {
            return RegfileExit::fault(0x70008945c5);
        }
        303 => {
            blk = 259; continue;
        }
        304 => {
            return RegfileExit::jump(0x70008945cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008945d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008945d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008945d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008945d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008945da); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008945df); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008945e3); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008945e7); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 88; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(8, c.r64(5));
            c.w64(9, c.r64(8).wrapping_add(0x18));
            c.w64(11, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008945ff); };
            c.w64(11, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000894604); };
            c.cmp(t3, c.r32(11), 32);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            let Some(t4) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000894613); };
            c.cmp(t4, c.r32(11), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x7000894622); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000894626); }
            let r5 = c.r32(12) | c.r32(15);
            c.w32(12, r5);
            c.logic(r5, 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089463d); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894644); };
            c.cmp(t8, 0xbau64, 16);
            if c.cc_b() { blk = 90; continue; }
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            c.w8(12, (c.cc_e() as u64));
            c.w32(12, c.r8(12));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x700089465e); }
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t9) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894668); };
            c.w32(15, t9);
            let r10 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r10);
            let Some(t11) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089466f); };
            c.cmp(t11, 0x834u64, 16);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t12) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894678); };
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
            let Some(t14) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000894698); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let Some(t16) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700089469f); };
            c.w32(15, t16);
            let r17 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008946a6); };
            c.w32(3, t18);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008946ad); };
            c.w32(2, t19);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0xb0u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008946bd);
        }
        12 => {
            let Some(t20) = c.ld32(c.r64(15).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008946be); };
            c.cmp(t20, c.r32(2), 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t21) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008946d5); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            let Some(t23) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008946dc); };
            c.w32(12, t23);
            let r24 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008946e4); };
            c.w32(15, t25);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t26) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008946ec); };
            c.w32(3, t26);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x44u64, 64);
            if c.cc_a() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            return RegfileExit::fault(0x70008946f9);
        }
        16 => {
            let Some(t27) = c.ld32(c.r64(12).wrapping_add(0x117)) else { return RegfileExit::fault(0x70008946fa); };
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
            return RegfileExit::fault(0x700089470d);
        }
        19 => {
            let Some(t28) = c.ld32(c.r64(12).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x700089470e); };
            c.cmp(t28, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(15), 0xafu64, 64);
            if c.cc_a() { blk = 93; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x7000894725);
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894726); };
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
            return c.call(0x70008a17f4, 0x700089473b, 0x7000894736);
        }
        25 => {
            c.w64(15, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700089473e); };
            c.w64(6, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894742); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894746); };
            c.w64(11, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700089474a); };
            c.w64(0, t34);
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700089474e); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(15)) { return RegfileExit::fault(0x7000894772); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000894776); }
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 102; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t36) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000894789); };
            c.w64(2, t36);
            c.w64(1, c.r64(2));
            blk = 33; continue;
        }
        32 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000894792); };
            c.w64(1, t37);
            blk = 33; continue;
        }
        33 => {
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000894799); };
            c.w64(2, t38);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008947a0); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t39) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008947ad); };
            c.w32(7, t39);
            let r40 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r40);
            let Some(t41) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008947b3); };
            c.logic(t41 & 0x2u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            c.w64(8, c.r64(9));
            let Some(t42) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008947c6); };
            c.w64(8, t42);
            blk = 38; continue;
        }
        37 => {
            c.w64(8, c.r64(2));
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008947cf); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_ne() { blk = 84; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t43) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008947dd); };
            c.w32(9, t43);
            let r44 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r44);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008947e4); }
            let Some(t45) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008947e8); };
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
            let Some(t46) = c.ld8(c.r64(9).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000894803); };
            c.w32(3, t46);
            c.w64(7, c.r64(3));
            let r47 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r47);
            let r48 = c.r32(7) & 0x3fu64;
            c.w32(7, r48);
            c.logic(r48, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x7000894811); }
            c.cmp(c.r32(7), 0x5u64, 32);
            if c.cc_a() { blk = 84; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t49) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089481e); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            let Some(t51) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000894824); };
            c.w32(7, t51);
            let r52 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089482a); };
            c.w32(8, t53);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x4cu64, 64);
            if c.cc_a() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            return RegfileExit::fault(0x700089483a);
        }
        46 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_add(0x137)) else { return RegfileExit::fault(0x700089483b); };
            c.w32(8, t54);
            let Some(t55) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000894842); };
            c.cmp(t55, c.r32(8), 32);
            if c.cc_ne() { blk = 84; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t56) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700089484c); };
            c.w64(8, t56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894853); };
            c.cmp(t57, c.r32(12), 32);
            if c.cc_e() { blk = 84; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w64(7, c.r64(12));
            blk = 50; continue;
        }
        49 => {
            c.w32(8, c.r64(7).wrapping_add(0x2));
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t58) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000894869); };
            c.w64(9, t58);
            let Some(t59) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x700089486d); };
            c.w64(8, t59);
            blk = 50; continue;
        }
        50 => {
            let Some(t60) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000894874); };
            c.cmp(c.r32(7), t60, 32);
            if c.cc_ge() { blk = 76; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t61) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089487d); };
            c.cmp(t61, c.r32(9), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t62) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894887); };
            c.cmp(t62, c.r32(12), 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894891); };
            c.w32(8, t63);
            c.cmp(c.r32(8), c.r32(7), 32);
            if c.cc_le() { blk = 82; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t64) = c.ld8(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700089489e); };
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
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008948b4); };
            c.w32(8, t68);
            let r69 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008948bb); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r71);
            let Some(t72) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008948c2); };
            c.w32(9, t72);
            let r73 = c.sub(c.r32(9), 0x77u64, 32);
            c.w32(9, r73);
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_a() { blk = 78; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t74) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008948d5); };
            c.w32(9, t74);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 77; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            let Some(t75) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008948e5); };
            c.w32(8, t75);
            let r76 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r76);
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008948ed); };
            c.cmp(t77, c.r32(8), 32);
            if c.cc_e() { blk = 49; continue; }
            blk = 58; continue;
        }
        58 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x70008948fa); }
            blk = 65; continue;
        }
        59 => {
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000894903); };
            c.w32(8, t78);
            let r79 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r79);
            let Some(t80) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700089490a); };
            c.w32(9, t80);
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700089490e); };
            c.cmp(t81, c.r32(9), 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t82) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700089491b); };
            c.w32(9, t82);
            c.cmp(c.r32(7), c.r32(9), 32);
            if c.cc_ae() { blk = 79; continue; }
            blk = 61; continue;
        }
        61 => {
            c.w64(9, (c.r32(7) as u32 as i32 as i64 as u64));
            if !c.movsd_rm(0, c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) { return RegfileExit::fault(0x700089492b); }
            c.w64(11, 0xfff7fffffff7ffffu64);
            let Some(t83) = c.ld64(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700089493c); };
            c.cmp(t83, c.r64(11), 64);
            if c.cc_e() { blk = 75; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000894947); };
            c.w64(8, t84);
            let Some(t85) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700089494e); };
            c.w64(9, t85);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000894955); }
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000894959); };
            c.w64(12, t86);
            let Some(t87) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000894960); };
            c.cmp(t87, c.r64(11), 64);
            if c.cc_be() { blk = 96; continue; }
            blk = 63; continue;
        }
        63 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700089496a); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 64; continue;
        }
        64 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000894971); };
            c.w64(9, t88);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000894978); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700089497c); }
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894982); };
            c.w64(11, t89);
            blk = 65; continue;
        }
        65 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000894986); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000894988); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000894989); }
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700089498b); };
            if !c.push64(t90) { return RegfileExit::fault(0x700089498b); }
            c.w32(0, 0x3u64);
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f2274, 0x700089499e, 0x7000894999);
        }
        66 => {
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700089499e); };
            c.cmp(t91, c.r32(0), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w32(8, c.r32(0));
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t92) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008949ae); };
            c.cmp(t92, c.r32(0), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t93) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008949b7); };
            c.w32(8, t93);
            let r94 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r94);
            let Some(t95) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008949be); };
            c.logic(t95 & 0x10u64, 8);
            if c.cc_ne() { blk = 74; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t96) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008949c5); };
            c.cmp(t96, c.r32(8), 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t97) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008949d2); };
            c.cmp(t97, 0x41u64, 16);
            if c.cc_e() { blk = 98; continue; }
            blk = 72; continue;
        }
        72 => {
            blk = 80; continue;
        }
        73 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 80; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008949e6); };
            c.w64(6, t98);
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008949ea); };
            c.w64(2, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008949f1); };
            c.w64(0, t100);
            let Some(t101) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008949f5); };
            c.w64(11, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008949f9); };
            c.w64(1, t102);
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008949fd); };
            c.w64(7, t103);
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 49; continue;
        }
        75 => {
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894a09); };
            c.w64(11, t104);
            blk = 49; continue;
        }
        76 => {
            c.w64(8, c.r64(0));
            let r105 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r105);
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000894a19); };
            c.w64(0, t106);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000894a23); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000894a24); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000894a2a); }
            return c.ret(0x0, 0x7000894a2c);
        }
        77 => {
            return RegfileExit::fault(0x7000894a2d);
        }
        78 => {
            return RegfileExit::fault(0x7000894a2e);
        }
        79 => {
            return RegfileExit::fault(0x7000894a2f);
        }
        80 => {
            let Some(t107) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894a30); };
            c.w64(8, t107);
            let r108 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r108);
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000894a38); };
            c.w64(0, t109);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000894a42); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000894a43); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000894a49); }
            return c.ret(0x0, 0x7000894a4b);
        }
        81 => {
            return RegfileExit::fault(0x7000894a4c);
        }
        82 => {
            c.w64(12, c.r64(7));
            blk = 84; continue;
        }
        83 => {
            c.w64(12, c.r64(7));
            blk = 84; continue;
        }
        84 => {
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000894a55); }
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000894a57); };
            if !c.push64(t110) { return RegfileExit::fault(0x7000894a57); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000894a5a); }
            c.w64(0, c.r64(11));
            c.w64(3, c.r64(1));
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894a61); };
            c.w64(1, t111);
            c.w64(7, c.r64(11));
            return c.call(0x7000893814, 0x7000894a6d, 0x7000894a68);
        }
        85 => {
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894a6d); };
            c.w64(8, t112);
            let r113 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r113);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000894a78); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x7000894a79); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(8).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x7000894a7f); }
            return c.ret(0x0, 0x7000894a81);
        }
        86 => {
            c.w32(8, 0x36u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000894a88); }
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x7000894a8a); };
            c.w64(8, t114);
            let Some(t115) = c.ld32(c.r64(8).wrapping_add(0x157)) else { return RegfileExit::fault(0x7000894a91); };
            c.w32(8, t115);
            let r116 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r116);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000894a9b); }
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000894a9d); };
            c.w64(3, t117);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000894aae, 0x7000894aa9);
        }
        87 => {
            return RegfileExit::fault(0x7000894aae);
        }
        88 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000894aaf); };
            c.w64(3, t118);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            return c.call(0x700086ed74, 0x7000894ac0, 0x7000894abb);
        }
        89 => {
            let Some(t119) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000894ac0); };
            c.w64(6, t119);
            let Some(t120) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894ac4); };
            c.w64(0, t120);
            blk = 1; continue;
        }
        90 => {
            c.w64(0, c.r64(11));
            return c.call(0x70008a2054, 0x7000894ad5, 0x7000894ad0);
        }
        91 => {
            c.w64(11, c.r64(0));
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000894ad8); };
            c.w64(6, t121);
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894adc); };
            c.w64(8, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000894ae0); };
            c.w64(9, t123);
            let Some(t124) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894ae4); };
            c.w64(0, t124);
            blk = 4; continue;
        }
        92 => {
            let Some(t125) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000894aed); };
            c.w32(12, t125);
            let r126 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r126);
            blk = 23; continue;
        }
        93 => {
            let Some(t127) = c.ld32(c.r64(12).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x7000894af9); };
            c.cmp(t127, c.r32(3), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000894b07); };
            c.w64(3, t128);
            c.w64(0, c.r64(11));
            return c.call(0x7000870914, 0x7000894b16, 0x7000894b11);
        }
        95 => {
            c.w64(12, c.r64(0));
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000894b19); };
            c.w64(6, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000894b1d); };
            c.w64(8, t130);
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000894b21); };
            c.w64(11, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000894b25); };
            c.w64(9, t132);
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894b29); };
            c.w64(0, t133);
            blk = 23; continue;
        }
        96 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x58), 0) { return RegfileExit::fault(0x7000894b37); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000894b44, 0x7000894b3f);
        }
        97 => {
            c.w64(8, c.r64(0));
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000894b47); };
            c.w64(6, t134);
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000894b4b); };
            c.w64(2, t135);
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000894b52); };
            c.w64(0, t136);
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000894b56); };
            c.w64(1, t137);
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000894b5a); };
            c.w64(7, t138);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x58)) { return RegfileExit::fault(0x7000894b5e); }
            blk = 64; continue;
        }
        98 => {
            let Some(t139) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000894b68); };
            c.logic(t139 & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 99; continue;
        }
        99 => {
            blk = 80; continue;
        }
        100 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000894b7a); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ne() { blk = 80; continue; }
            blk = 101; continue;
        }
        101 => {
            blk = 74; continue;
        }
        102 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000894b92); };
            c.w64(2, t140);
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t141) = c.ld64(c.r64(8).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000894b9e); };
            c.w64(8, t141);
            c.w64(2, c.r64(8));
            blk = 105; continue;
        }
        105 => {
            c.w32(8, 0x32u64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000894bab); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000894bad); }
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000894bae); };
            c.w64(3, t142);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000894bbf, 0x7000894bba);
        }
        106 => {
            return RegfileExit::fault(0x7000894bbf);
        }
        107 => {
            return RegfileExit::jump(0x7000894bc4);
        }
            _ => unreachable!(),
        }
    }
}
