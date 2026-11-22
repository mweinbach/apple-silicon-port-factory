#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008081b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008081b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008081b8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008081be); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008081c5); };
            c.w32(9, t3);
            let r4 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r4);
            let r5 = c.r32(9) & 0x3fu64;
            c.w32(9, r5);
            c.logic(r5, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008081d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008081d6); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008081da); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008081de); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008081e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008081e6); }
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 954; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 1002; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 1053; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 833; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 878; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 1053; continue;
        }
        10 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 742; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 788; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 1053; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 650; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 696; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 1053; continue;
        }
        17 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 25; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_l() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 558; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 604; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 1053; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 491; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 512; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 1053; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 421; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 470; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 1053; continue;
        }
        29 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 385; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 403; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 1053; continue;
        }
        32 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 347; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 366; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 1053; continue;
        }
        38 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 309; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 328; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 1053; continue;
        }
        41 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 269; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 290; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 1053; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 143; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 205; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 1053; continue;
        }
        48 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 56; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 123; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 1053; continue;
        }
        53 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 1053; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 103; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 1053; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 1053; continue;
        }
        63 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008083dc); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008083e3); };
            c.w32(11, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008083e7); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 64; continue;
        }
        64 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 72; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008083f9); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808400); };
            c.cmp(t12, 0x42u64, 16);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        67 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808410); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            if c.cc_e() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        70 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r13 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r13);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 73; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        72 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 73; continue;
        }
        73 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        75 => {
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808469); };
            c.cmp(t14, 0x834u64, 16);
            if c.cc_e() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808472); };
            c.w32(8, t15);
            blk = 78; continue;
        }
        77 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808478); };
            c.w32(8, t16);
            blk = 78; continue;
        }
        78 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 80; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        80 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080848f); };
            c.w32(7, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1054; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x70008084a2); }
            blk = 1050; continue;
        }
        82 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        83 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008084b4); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008084bb); };
            c.w32(11, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008084bf); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 84; continue;
        }
        84 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t22) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008084d1); };
            c.w32(11, t22);
            let r23 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r23);
            let Some(t24) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008084d8); };
            c.cmp(t24, 0x42u64, 16);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        87 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008084e8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        90 => {
            let Some(t25) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080850b); };
            c.w64(12, t25);
            let r26 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r26);
            let Some(t27) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808515); };
            c.w64(15, t27);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 93; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        92 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        95 => {
            let Some(t28) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080853b); };
            c.cmp(t28, 0x834u64, 16);
            if c.cc_e() { blk = 97; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808544); };
            c.w32(8, t29);
            blk = 98; continue;
        }
        97 => {
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080854a); };
            c.w32(8, t30);
            blk = 98; continue;
        }
        98 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        100 => {
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808561); };
            c.w32(7, t31);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1055; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000808574); }
            blk = 1050; continue;
        }
        102 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        103 => {
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808586); };
            c.w32(9, t32);
            let r33 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r33);
            let Some(t34) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080858d); };
            c.w32(11, t34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000808591); };
            c.cmp(t35, c.r32(11), 32);
            if c.cc_e() { blk = 122; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 112; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t36) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008085a3); };
            c.w32(11, t36);
            let r37 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r37);
            let Some(t38) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008085aa); };
            c.cmp(t38, 0x42u64, 16);
            if c.cc_e() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        107 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008085ba); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            if c.cc_e() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        110 => {
            let Some(t39) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008085dd); };
            c.w64(12, t39);
            let r40 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r40);
            let Some(t41) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x70008085e7); };
            c.w64(15, t41);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 113; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        112 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 113; continue;
        }
        113 => {
            let Some(t42) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808601); };
            c.cmp(t42, 0x834u64, 16);
            if c.cc_e() { blk = 115; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080860a); };
            c.w32(8, t43);
            blk = 116; continue;
        }
        115 => {
            let Some(t44) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808610); };
            c.w32(8, t44);
            blk = 116; continue;
        }
        116 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        118 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808627); };
            c.w32(7, t45);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1056; continue; }
            blk = 119; continue;
        }
        119 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x7000808642); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r46 = c.r64(8) & c.r64(9);
            c.w64(8, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000808654); };
            c.logic(t47 & 0x4u64, 8);
            if c.cc_ne() { blk = 1185; continue; }
            blk = 121; continue;
        }
        121 => {
            blk = 1050; continue;
        }
        122 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        123 => {
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080866c); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            let Some(t50) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808673); };
            c.w32(11, t50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000808677); };
            c.cmp(t51, c.r32(11), 32);
            if c.cc_e() { blk = 142; continue; }
            blk = 124; continue;
        }
        124 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808689); };
            c.w32(11, t52);
            let r53 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r53);
            let Some(t54) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808690); };
            c.cmp(t54, 0x42u64, 16);
            if c.cc_e() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        127 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008086a0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 129; continue; }
            blk = 128; continue;
        }
        128 => {
            if c.cc_e() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        130 => {
            let Some(t55) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008086c3); };
            c.w64(12, t55);
            let r56 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r56);
            let Some(t57) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x70008086cd); };
            c.w64(15, t57);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 131; continue;
        }
        131 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        132 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 133; continue;
        }
        133 => {
            let Some(t58) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008086e7); };
            c.cmp(t58, 0x834u64, 16);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008086f0); };
            c.w32(8, t59);
            blk = 136; continue;
        }
        135 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008086f6); };
            c.w32(8, t60);
            blk = 136; continue;
        }
        136 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        138 => {
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080870d); };
            c.w32(7, t61);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1057; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x7000808728); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 141; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r62 = c.r64(8) & c.r64(9);
            c.w64(8, r62);
            c.logic(r62, 64);
            let Some(t63) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080873a); };
            c.logic(t63 & 0x4u64, 8);
            if c.cc_ne() { blk = 1182; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 1050; continue;
        }
        142 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        143 => {
            let Some(t64) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808752); };
            c.w32(9, t64);
            let r65 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r65);
            let Some(t66) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808759); };
            c.w32(11, t66);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080875d); };
            c.cmp(t67, c.r32(11), 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 144; continue;
        }
        144 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t68) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080876f); };
            c.w32(11, t68);
            let r69 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r69);
            let Some(t70) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808776); };
            c.cmp(t70, 0x42u64, 16);
            if c.cc_e() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        147 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808786); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 149; continue; }
            blk = 148; continue;
        }
        148 => {
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        150 => {
            let Some(t71) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008087a9); };
            c.w64(12, t71);
            let r72 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r72);
            let Some(t73) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x70008087b3); };
            c.w64(15, t73);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 153; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        152 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 153; continue;
        }
        153 => {
            let Some(t74) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008087cd); };
            c.cmp(t74, 0x834u64, 16);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008087d6); };
            c.w32(8, t75);
            blk = 156; continue;
        }
        155 => {
            let Some(t76) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008087dc); };
            c.w32(8, t76);
            blk = 156; continue;
        }
        156 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        158 => {
            let Some(t77) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008087f3); };
            c.w32(7, t77);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1058; continue; }
            blk = 159; continue;
        }
        159 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700080880e); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r78 = c.r64(8) & c.r64(9);
            c.w64(8, r78);
            c.logic(r78, 64);
            let Some(t79) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000808820); };
            c.logic(t79 & 0x4u64, 8);
            if c.cc_ne() { blk = 1179; continue; }
            blk = 161; continue;
        }
        161 => {
            blk = 1050; continue;
        }
        162 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        163 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808838); };
            c.w32(9, t80);
            let r81 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r81);
            let Some(t82) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080883f); };
            c.w32(11, t82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000808843); };
            c.cmp(t83, c.r32(11), 32);
            if c.cc_e() { blk = 182; continue; }
            blk = 164; continue;
        }
        164 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 172; continue; }
            blk = 165; continue;
        }
        165 => {
            let Some(t84) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808855); };
            c.w32(11, t84);
            let r85 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r85);
            let Some(t86) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080885c); };
            c.cmp(t86, 0x42u64, 16);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        167 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080886c); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 169; continue; }
            blk = 168; continue;
        }
        168 => {
            if c.cc_e() { blk = 170; continue; }
            blk = 169; continue;
        }
        169 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        170 => {
            let Some(t87) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080888f); };
            c.w64(12, t87);
            let r88 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r88);
            let Some(t89) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808899); };
            c.w64(15, t89);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        172 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 173; continue;
        }
        173 => {
            let Some(t90) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008088b3); };
            c.cmp(t90, 0x834u64, 16);
            if c.cc_e() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t91) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008088bc); };
            c.w32(8, t91);
            blk = 176; continue;
        }
        175 => {
            let Some(t92) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008088c2); };
            c.w32(8, t92);
            blk = 176; continue;
        }
        176 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 178; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        178 => {
            let Some(t93) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008088d9); };
            c.w32(7, t93);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1059; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x70008088f4); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r94 = c.r64(8) & c.r64(9);
            c.w64(8, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000808906); };
            c.logic(t95 & 0x4u64, 8);
            if c.cc_ne() { blk = 1176; continue; }
            blk = 181; continue;
        }
        181 => {
            blk = 1050; continue;
        }
        182 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        183 => {
            let Some(t96) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080891e); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808925); };
            c.w32(11, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000808929); };
            c.cmp(t99, c.r32(11), 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 184; continue;
        }
        184 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 192; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t100) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080893b); };
            c.w32(11, t100);
            let r101 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r101);
            let Some(t102) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808942); };
            c.cmp(t102, 0x42u64, 16);
            if c.cc_e() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        187 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808952); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            if c.cc_e() { blk = 190; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        190 => {
            let Some(t103) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000808975); };
            c.w64(12, t103);
            let r104 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r104);
            let Some(t105) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080897f); };
            c.w64(15, t105);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 193; continue; }
            blk = 191; continue;
        }
        191 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        192 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 193; continue;
        }
        193 => {
            let Some(t106) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808999); };
            c.cmp(t106, 0x834u64, 16);
            if c.cc_e() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t107) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008089a2); };
            c.w32(8, t107);
            blk = 196; continue;
        }
        195 => {
            let Some(t108) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008089a8); };
            c.w32(8, t108);
            blk = 196; continue;
        }
        196 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        198 => {
            let Some(t109) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008089bf); };
            c.w32(8, t109);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 1060; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t110) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008089da); };
            c.w32(11, t110);
            let Some(t111) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008089df); };
            c.cmp(t111, c.r32(11), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 200; continue;
        }
        200 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008089e8); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r112 = c.r64(7) & c.r64(9);
            c.w64(7, r112);
            c.logic(r112, 64);
            let Some(t113) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008089fa); };
            c.logic(t113 & 0x4u64, 8);
            if c.cc_ne() { blk = 1173; continue; }
            blk = 202; continue;
        }
        202 => {
            blk = 1050; continue;
        }
        203 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        204 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        205 => {
            let Some(t114) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808a19); };
            c.w32(9, t114);
            let r115 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r115);
            let Some(t116) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808a20); };
            c.w32(11, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000808a24); };
            c.cmp(t117, c.r32(11), 32);
            if c.cc_e() { blk = 226; continue; }
            blk = 206; continue;
        }
        206 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 214; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t118) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808a36); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808a3d); };
            c.cmp(t120, 0x42u64, 16);
            if c.cc_e() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        209 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808a4d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            if c.cc_e() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        212 => {
            let Some(t121) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000808a70); };
            c.w64(12, t121);
            let r122 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r122);
            let Some(t123) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808a7a); };
            c.w64(15, t123);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        214 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 215; continue;
        }
        215 => {
            let Some(t124) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808a94); };
            c.cmp(t124, 0x834u64, 16);
            if c.cc_e() { blk = 217; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t125) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808a9d); };
            c.w32(8, t125);
            blk = 218; continue;
        }
        217 => {
            let Some(t126) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808aa3); };
            c.w32(8, t126);
            blk = 218; continue;
        }
        218 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        220 => {
            let Some(t127) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808aba); };
            c.w32(8, t127);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 1061; continue; }
            blk = 221; continue;
        }
        221 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t128) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808ad5); };
            c.w32(11, t128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000808ada); };
            c.cmp(t129, c.r32(11), 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 222; continue;
        }
        222 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x7000808ae3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r130 = c.r64(7) & c.r64(9);
            c.w64(7, r130);
            c.logic(r130, 64);
            let Some(t131) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000808af5); };
            c.logic(t131 & 0x4u64, 8);
            if c.cc_ne() { blk = 1170; continue; }
            blk = 224; continue;
        }
        224 => {
            blk = 1050; continue;
        }
        225 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        226 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        227 => {
            let Some(t132) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808b14); };
            c.w32(9, t132);
            let r133 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r133);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 235; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t134) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808b20); };
            c.w32(11, t134);
            let r135 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r135);
            let Some(t136) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808b27); };
            c.cmp(t136, 0x42u64, 16);
            if c.cc_e() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        230 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808b37); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 232; continue; }
            blk = 231; continue;
        }
        231 => {
            if c.cc_e() { blk = 233; continue; }
            blk = 232; continue;
        }
        232 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        233 => {
            let Some(t137) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000808b5a); };
            c.w64(12, t137);
            let r138 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r138);
            let Some(t139) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808b64); };
            c.w64(15, t139);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        235 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 236; continue;
        }
        236 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 240; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t140) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808b82); };
            c.w32(12, t140);
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000808b86); };
            c.cmp(t141, c.r32(12), 32);
            if c.cc_ne() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808b8f); }
            blk = 241; continue;
        }
        239 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        240 => {
            c.w64(12, c.r64(0));
            let r142 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r142);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 241; continue;
        }
        241 => {
            let Some(t143) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808bad); };
            c.cmp(t143, 0x834u64, 16);
            if c.cc_e() { blk = 243; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t144) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808bb6); };
            c.w32(8, t144);
            blk = 244; continue;
        }
        243 => {
            let Some(t145) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808bbc); };
            c.w32(8, t145);
            blk = 244; continue;
        }
        244 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 246; continue; }
            blk = 245; continue;
        }
        245 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        246 => {
            let Some(t146) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808bd3); };
            c.w32(7, t146);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1062; continue; }
            blk = 247; continue;
        }
        247 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000808bf0); }
            blk = 1050; continue;
        }
        248 => {
            let Some(t147) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808bfc); };
            c.w32(9, t147);
            let r148 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r148);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t149) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808c08); };
            c.w32(11, t149);
            let r150 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r150);
            let Some(t151) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808c0f); };
            c.cmp(t151, 0x42u64, 16);
            if c.cc_e() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        251 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808c1f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            if c.cc_e() { blk = 254; continue; }
            blk = 253; continue;
        }
        253 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        254 => {
            let Some(t152) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000808c42); };
            c.w64(12, t152);
            let r153 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r153);
            let Some(t154) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808c4c); };
            c.w64(15, t154);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 257; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        256 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 257; continue;
        }
        257 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 261; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t155) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808c6a); };
            c.w32(12, t155);
            let Some(t156) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000808c6e); };
            c.cmp(t156, c.r32(12), 32);
            if c.cc_ne() { blk = 260; continue; }
            blk = 259; continue;
        }
        259 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808c77); }
            blk = 262; continue;
        }
        260 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        261 => {
            c.w64(12, c.r64(0));
            let r157 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r157);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 262; continue;
        }
        262 => {
            let Some(t158) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808c95); };
            c.cmp(t158, 0x834u64, 16);
            if c.cc_e() { blk = 264; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t159) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808c9e); };
            c.w32(8, t159);
            blk = 265; continue;
        }
        264 => {
            let Some(t160) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808ca4); };
            c.w32(8, t160);
            blk = 265; continue;
        }
        265 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 267; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        267 => {
            let Some(t161) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000808cbb); };
            c.w32(7, t161);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1063; continue; }
            blk = 268; continue;
        }
        268 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000808cd8); }
            blk = 1050; continue;
        }
        269 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 277; continue; }
            blk = 270; continue;
        }
        270 => {
            let Some(t162) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808ce9); };
            c.w32(8, t162);
            let r163 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r163);
            let Some(t164) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808cf0); };
            c.cmp(t164, 0x42u64, 16);
            if c.cc_e() { blk = 272; continue; }
            blk = 271; continue;
        }
        271 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        272 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808d00); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 274; continue; }
            blk = 273; continue;
        }
        273 => {
            if c.cc_e() { blk = 275; continue; }
            blk = 274; continue;
        }
        274 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        275 => {
            let Some(t165) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000808d23); };
            c.w64(9, t165);
            let r166 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r166);
            let Some(t167) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808d2d); };
            c.w64(11, t167);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 278; continue; }
            blk = 276; continue;
        }
        276 => {
            c.w64(8, c.r64(1));
            blk = 1044; continue;
        }
        277 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 278; continue;
        }
        278 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000808d47); }
            c.w64(9, c.r64(0));
            blk = 281; continue;
        }
        279 => {
            return c.call(0x70008a1014, 0x7000808d55, 0x7000808d50);
        }
        280 => {
            c.w64(9, c.r64(0));
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000808d58); };
            c.w64(0, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000808d5c); };
            c.w64(6, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000808d60); };
            c.w64(2, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000808d64); };
            c.w64(1, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000808d68); };
            c.w64(8, t172);
            blk = 281; continue;
        }
        281 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t173) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808d72); };
            c.w32(11, t173);
            let r174 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000808d79); };
            c.cmp(t175, c.r32(11), 32);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let Some(t176) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808d82); };
            c.cmp(t176, 0x43u64, 16);
            if c.cc_ne() { blk = 279; continue; }
            blk = 284; continue;
        }
        284 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808d8a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1168; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w32(7, c.r32(7));
            blk = 287; continue;
        }
        286 => {
            let r177 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r177);
            c.w64(7, c.r64(9));
            blk = 287; continue;
        }
        287 => {
            let Some(t178) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808da9); };
            c.w32(9, t178);
            let r179 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r179);
            let Some(t180) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000808db0); };
            c.logic(t180 & 0x4u64, 8);
            if c.cc_ne() { blk = 1064; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t181) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000808dbb); };
            c.cmp(t181, c.r64(8), 64);
            if c.cc_be() { blk = 1064; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t182) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000808dc5); };
            c.w64(9, t182);
            let Some(t183) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000808dc9); };
            c.w32(11, t183);
            let r184 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r184);
            if !c.st8(c.r64(8).wrapping_add(c.r64(9)), c.r8(7)) { return RegfileExit::fault(0x7000808dd0); }
            blk = 1050; continue;
        }
        290 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 296; continue; }
            blk = 291; continue;
        }
        291 => {
            c.w64(8, c.r64(1));
            let Some(t185) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808de1); };
            c.w32(9, t185);
            let r186 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r186);
            let Some(t187) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808de8); };
            c.cmp(t187, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 292; continue;
        }
        292 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808df4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 293; continue;
        }
        293 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t188) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000808e18); };
            c.w64(11, t188);
            let r189 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r189);
            let Some(t190) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808e22); };
            c.w64(12, t190);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 297; continue; }
            blk = 295; continue;
        }
        295 => {
            blk = 1044; continue;
        }
        296 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 297; continue;
        }
        297 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000808e3f); }
            c.w64(11, c.r64(0));
            blk = 300; continue;
        }
        298 => {
            return c.call(0x70008a1014, 0x7000808e4d, 0x7000808e48);
        }
        299 => {
            c.w64(11, c.r64(0));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000808e50); };
            c.w64(0, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000808e54); };
            c.w64(6, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000808e58); };
            c.w64(2, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000808e5c); };
            c.w64(8, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000808e60); };
            c.w64(9, t195);
            blk = 300; continue;
        }
        300 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 305; continue; }
            blk = 301; continue;
        }
        301 => {
            let Some(t196) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808e6a); };
            c.w32(12, t196);
            let r197 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r197);
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000808e71); };
            c.cmp(t198, c.r32(12), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t199) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808e7a); };
            c.cmp(t199, 0x43u64, 16);
            if c.cc_ne() { blk = 298; continue; }
            blk = 303; continue;
        }
        303 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808e83); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1166; continue; }
            blk = 304; continue;
        }
        304 => {
            c.w32(7, c.r32(7));
            blk = 306; continue;
        }
        305 => {
            let r200 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r200);
            c.w64(7, c.r64(11));
            blk = 306; continue;
        }
        306 => {
            let Some(t201) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808ea2); };
            c.w32(11, t201);
            let r202 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r202);
            let Some(t203) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000808ea9); };
            c.logic(t203 & 0x4u64, 8);
            if c.cc_ne() { blk = 1065; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t204) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000808eb4); };
            c.cmp(t204, c.r64(9), 64);
            if c.cc_be() { blk = 1065; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t205) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000808ebe); };
            c.w64(8, t205);
            let Some(t206) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000808ec2); };
            c.w32(11, t206);
            let r207 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r207);
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), c.r8(7)) { return RegfileExit::fault(0x7000808ec9); }
            blk = 1050; continue;
        }
        309 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 315; continue; }
            blk = 310; continue;
        }
        310 => {
            c.w64(8, c.r64(1));
            let Some(t208) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808eda); };
            c.w32(9, t208);
            let r209 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r209);
            let Some(t210) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808ee1); };
            c.cmp(t210, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 311; continue;
        }
        311 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808eed); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 312; continue;
        }
        312 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t211) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000808f11); };
            c.w64(11, t211);
            let r212 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r212);
            let Some(t213) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000808f1b); };
            c.w64(12, t213);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 316; continue; }
            blk = 314; continue;
        }
        314 => {
            blk = 1044; continue;
        }
        315 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 316; continue;
        }
        316 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000808f38); }
            c.w64(11, c.r64(0));
            blk = 319; continue;
        }
        317 => {
            return c.call(0x70008a1014, 0x7000808f46, 0x7000808f41);
        }
        318 => {
            c.w64(11, c.r64(0));
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000808f49); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000808f4d); };
            c.w64(6, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000808f51); };
            c.w64(2, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000808f55); };
            c.w64(8, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000808f59); };
            c.w64(9, t218);
            blk = 319; continue;
        }
        319 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 324; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t219) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808f63); };
            c.w32(12, t219);
            let r220 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r220);
            let Some(t221) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000808f6a); };
            c.cmp(t221, c.r32(12), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t222) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808f73); };
            c.cmp(t222, 0x43u64, 16);
            if c.cc_ne() { blk = 317; continue; }
            blk = 322; continue;
        }
        322 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808f7c); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1164; continue; }
            blk = 323; continue;
        }
        323 => {
            c.w32(7, c.r32(7));
            blk = 325; continue;
        }
        324 => {
            let r223 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r223);
            c.w64(7, c.r64(11));
            blk = 325; continue;
        }
        325 => {
            let Some(t224) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000808f9b); };
            c.w32(11, t224);
            let r225 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r225);
            let Some(t226) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000808fa2); };
            c.logic(t226 & 0x4u64, 8);
            if c.cc_ne() { blk = 1066; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t227) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000808fad); };
            c.cmp(t227, c.r64(9), 64);
            if c.cc_be() { blk = 1066; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t228) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000808fb7); };
            c.w64(8, t228);
            let Some(t229) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000808fbb); };
            c.w32(11, t229);
            let r230 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r230);
            if !c.st16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x7000808fc2); }
            blk = 1050; continue;
        }
        328 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 334; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w64(8, c.r64(1));
            let Some(t231) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000808fd4); };
            c.w32(9, t231);
            let r232 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r232);
            let Some(t233) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000808fdb); };
            c.cmp(t233, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 330; continue;
        }
        330 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000808fe7); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 331; continue;
        }
        331 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 332; continue;
        }
        332 => {
            let Some(t234) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080900b); };
            c.w64(11, t234);
            let r235 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r235);
            let Some(t236) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000809015); };
            c.w64(12, t236);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 335; continue; }
            blk = 333; continue;
        }
        333 => {
            blk = 1044; continue;
        }
        334 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 335; continue;
        }
        335 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000809032); }
            c.w64(11, c.r64(0));
            blk = 338; continue;
        }
        336 => {
            return c.call(0x70008a1014, 0x7000809040, 0x700080903b);
        }
        337 => {
            c.w64(11, c.r64(0));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809043); };
            c.w64(0, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809047); };
            c.w64(6, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080904b); };
            c.w64(2, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080904f); };
            c.w64(8, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000809053); };
            c.w64(9, t241);
            blk = 338; continue;
        }
        338 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 343; continue; }
            blk = 339; continue;
        }
        339 => {
            let Some(t242) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080905d); };
            c.w32(12, t242);
            let r243 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r243);
            let Some(t244) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809064); };
            c.cmp(t244, c.r32(12), 32);
            if c.cc_e() { blk = 341; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t245) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080906d); };
            c.cmp(t245, 0x43u64, 16);
            if c.cc_ne() { blk = 336; continue; }
            blk = 341; continue;
        }
        341 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809076); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1162; continue; }
            blk = 342; continue;
        }
        342 => {
            c.w32(7, c.r32(7));
            blk = 344; continue;
        }
        343 => {
            let r246 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r246);
            c.w64(7, c.r64(11));
            blk = 344; continue;
        }
        344 => {
            let Some(t247) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809095); };
            c.w32(11, t247);
            let r248 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r248);
            let Some(t249) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080909c); };
            c.logic(t249 & 0x4u64, 8);
            if c.cc_ne() { blk = 1067; continue; }
            blk = 345; continue;
        }
        345 => {
            let Some(t250) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008090a7); };
            c.cmp(t250, c.r64(9), 64);
            if c.cc_be() { blk = 1067; continue; }
            blk = 346; continue;
        }
        346 => {
            let Some(t251) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008090b1); };
            c.w64(8, t251);
            let Some(t252) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008090b5); };
            c.w32(11, t252);
            let r253 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r253);
            if !c.st16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x70008090bc); }
            blk = 1050; continue;
        }
        347 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 353; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w64(8, c.r64(1));
            let Some(t254) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008090ce); };
            c.w32(9, t254);
            let r255 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r255);
            let Some(t256) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008090d5); };
            c.cmp(t256, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 349; continue;
        }
        349 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008090e1); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 350; continue;
        }
        350 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 351; continue;
        }
        351 => {
            let Some(t257) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000809105); };
            c.w64(11, t257);
            let r258 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r258);
            let Some(t259) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080910f); };
            c.w64(12, t259);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 354; continue; }
            blk = 352; continue;
        }
        352 => {
            blk = 1044; continue;
        }
        353 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 354; continue;
        }
        354 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080912c); }
            c.w64(11, c.r64(0));
            blk = 357; continue;
        }
        355 => {
            return c.call(0x70008a1014, 0x700080913a, 0x7000809135);
        }
        356 => {
            c.w64(11, c.r64(0));
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080913d); };
            c.w64(0, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809141); };
            c.w64(6, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809145); };
            c.w64(2, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809149); };
            c.w64(8, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080914d); };
            c.w64(9, t264);
            blk = 357; continue;
        }
        357 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 362; continue; }
            blk = 358; continue;
        }
        358 => {
            let Some(t265) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809157); };
            c.w32(12, t265);
            let r266 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r266);
            let Some(t267) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080915e); };
            c.cmp(t267, c.r32(12), 32);
            if c.cc_e() { blk = 360; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t268) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809167); };
            c.cmp(t268, 0x43u64, 16);
            if c.cc_ne() { blk = 355; continue; }
            blk = 360; continue;
        }
        360 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809170); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1160; continue; }
            blk = 361; continue;
        }
        361 => {
            c.w32(7, c.r32(7));
            blk = 363; continue;
        }
        362 => {
            let r269 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r269);
            c.w64(7, c.r64(11));
            blk = 363; continue;
        }
        363 => {
            let Some(t270) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080918f); };
            c.w32(11, t270);
            let r271 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r271);
            let Some(t272) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000809196); };
            c.logic(t272 & 0x4u64, 8);
            if c.cc_ne() { blk = 1068; continue; }
            blk = 364; continue;
        }
        364 => {
            let Some(t273) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008091a1); };
            c.cmp(t273, c.r64(9), 64);
            if c.cc_be() { blk = 1068; continue; }
            blk = 365; continue;
        }
        365 => {
            let Some(t274) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008091ab); };
            c.w64(8, t274);
            let Some(t275) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008091af); };
            c.w32(11, t275);
            let r276 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r276);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x70008091b6); }
            blk = 1050; continue;
        }
        366 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 372; continue; }
            blk = 367; continue;
        }
        367 => {
            c.w64(8, c.r64(1));
            let Some(t277) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008091c7); };
            c.w32(9, t277);
            let r278 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r278);
            let Some(t279) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008091ce); };
            c.cmp(t279, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 368; continue;
        }
        368 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008091da); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 369; continue;
        }
        369 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 370; continue;
        }
        370 => {
            let Some(t280) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008091fe); };
            c.w64(11, t280);
            let r281 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r281);
            let Some(t282) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000809208); };
            c.w64(12, t282);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 373; continue; }
            blk = 371; continue;
        }
        371 => {
            blk = 1044; continue;
        }
        372 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 373; continue;
        }
        373 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000809225); }
            c.w64(11, c.r64(0));
            blk = 376; continue;
        }
        374 => {
            return c.call(0x70008a1014, 0x7000809233, 0x700080922e);
        }
        375 => {
            c.w64(11, c.r64(0));
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809236); };
            c.w64(0, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080923a); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080923e); };
            c.w64(2, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809242); };
            c.w64(8, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000809246); };
            c.w64(9, t287);
            blk = 376; continue;
        }
        376 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 381; continue; }
            blk = 377; continue;
        }
        377 => {
            let Some(t288) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809250); };
            c.w32(12, t288);
            let r289 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r289);
            let Some(t290) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809257); };
            c.cmp(t290, c.r32(12), 32);
            if c.cc_e() { blk = 379; continue; }
            blk = 378; continue;
        }
        378 => {
            let Some(t291) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809260); };
            c.cmp(t291, 0x43u64, 16);
            if c.cc_ne() { blk = 374; continue; }
            blk = 379; continue;
        }
        379 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809269); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1158; continue; }
            blk = 380; continue;
        }
        380 => {
            c.w32(7, c.r32(7));
            blk = 382; continue;
        }
        381 => {
            let r292 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r292);
            c.w64(7, c.r64(11));
            blk = 382; continue;
        }
        382 => {
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809288); };
            c.w32(11, t293);
            let r294 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r294);
            let Some(t295) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080928f); };
            c.logic(t295 & 0x4u64, 8);
            if c.cc_ne() { blk = 1076; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t296) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080929a); };
            c.cmp(t296, c.r64(9), 64);
            if c.cc_be() { blk = 1076; continue; }
            blk = 384; continue;
        }
        384 => {
            let Some(t297) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008092a4); };
            c.w64(8, t297);
            let Some(t298) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008092a8); };
            c.w32(11, t298);
            let r299 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r299);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x70008092af); }
            blk = 1050; continue;
        }
        385 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 391; continue; }
            blk = 386; continue;
        }
        386 => {
            c.w64(8, c.r64(1));
            let Some(t300) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008092c0); };
            c.w32(9, t300);
            let r301 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r301);
            let Some(t302) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008092c7); };
            c.cmp(t302, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 387; continue;
        }
        387 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008092d3); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 388; continue;
        }
        388 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 389; continue;
        }
        389 => {
            let Some(t303) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008092f7); };
            c.w64(11, t303);
            let r304 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r304);
            let Some(t305) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000809301); };
            c.w64(12, t305);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 392; continue; }
            blk = 390; continue;
        }
        390 => {
            blk = 1044; continue;
        }
        391 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 392; continue;
        }
        392 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080931e); }
            c.w64(11, c.r64(0));
            blk = 395; continue;
        }
        393 => {
            return c.call(0x70008a1014, 0x700080932c, 0x7000809327);
        }
        394 => {
            c.w64(11, c.r64(0));
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080932f); };
            c.w64(0, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809333); };
            c.w64(6, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809337); };
            c.w64(2, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080933b); };
            c.w64(8, t309);
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080933f); };
            c.w64(9, t310);
            blk = 395; continue;
        }
        395 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 399; continue; }
            blk = 396; continue;
        }
        396 => {
            let Some(t311) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809349); };
            c.w32(12, t311);
            let r312 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r312);
            let Some(t313) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809350); };
            c.cmp(t313, c.r32(12), 32);
            if c.cc_e() { blk = 398; continue; }
            blk = 397; continue;
        }
        397 => {
            let Some(t314) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809359); };
            c.cmp(t314, 0x43u64, 16);
            if c.cc_ne() { blk = 393; continue; }
            blk = 398; continue;
        }
        398 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809362); }
            c.cvtsd2ss(0, c.xmm_u64(0));
            blk = 400; continue;
        }
        399 => {
            let r315 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r315);
            c.xorpd_rr(0, 0);
            c.cvtsi2ss(0, (c.r32(11) as u32 as i32 as i64));
            blk = 400; continue;
        }
        400 => {
            let Some(t316) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809379); };
            c.w32(7, t316);
            let r317 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r317);
            let Some(t318) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080937f); };
            c.logic(t318 & 0x4u64, 8);
            if c.cc_ne() { blk = 1084; continue; }
            blk = 401; continue;
        }
        401 => {
            let Some(t319) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000809389); };
            c.cmp(t319, c.r64(9), 64);
            if c.cc_be() { blk = 1084; continue; }
            blk = 402; continue;
        }
        402 => {
            let Some(t320) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000809393); };
            c.w64(7, t320);
            let Some(t321) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000809397); };
            c.w32(8, t321);
            let r322 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r322);
            if !c.movss_mr(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)), 0) { return RegfileExit::fault(0x700080939e); }
            blk = 1050; continue;
        }
        403 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 409; continue; }
            blk = 404; continue;
        }
        404 => {
            c.w64(8, c.r64(1));
            let Some(t323) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008093b1); };
            c.w32(9, t323);
            let r324 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r324);
            let Some(t325) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008093b8); };
            c.cmp(t325, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 405; continue;
        }
        405 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008093c4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 406; continue;
        }
        406 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t326) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008093e8); };
            c.w64(11, t326);
            let r327 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r327);
            let Some(t328) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x70008093f2); };
            c.w64(12, t328);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 410; continue; }
            blk = 408; continue;
        }
        408 => {
            blk = 1044; continue;
        }
        409 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 410; continue;
        }
        410 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080940f); }
            c.w64(11, c.r64(0));
            blk = 413; continue;
        }
        411 => {
            return c.call(0x70008a1014, 0x700080941d, 0x7000809418);
        }
        412 => {
            c.w64(11, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809420); };
            c.w64(0, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809424); };
            c.w64(6, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809428); };
            c.w64(2, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080942c); };
            c.w64(8, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000809430); };
            c.w64(9, t333);
            blk = 413; continue;
        }
        413 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 417; continue; }
            blk = 414; continue;
        }
        414 => {
            let Some(t334) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080943a); };
            c.w32(12, t334);
            let r335 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r335);
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809441); };
            c.cmp(t336, c.r32(12), 32);
            if c.cc_e() { blk = 416; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t337) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080944a); };
            c.cmp(t337, 0x43u64, 16);
            if c.cc_ne() { blk = 411; continue; }
            blk = 416; continue;
        }
        416 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809453); }
            blk = 418; continue;
        }
        417 => {
            let r338 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r338);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 418; continue;
        }
        418 => {
            let Some(t339) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809467); };
            c.w32(7, t339);
            let r340 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r340);
            let Some(t341) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080946d); };
            c.logic(t341 & 0x4u64, 8);
            if c.cc_ne() { blk = 1092; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t342) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000809477); };
            c.cmp(t342, c.r64(9), 64);
            if c.cc_be() { blk = 1092; continue; }
            blk = 420; continue;
        }
        420 => {
            let Some(t343) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000809481); };
            c.w64(7, t343);
            let Some(t344) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000809485); };
            c.w32(8, t344);
            let r345 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r345);
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), 0) { return RegfileExit::fault(0x700080948c); }
            blk = 1050; continue;
        }
        421 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 427; continue; }
            blk = 422; continue;
        }
        422 => {
            c.w64(8, c.r64(1));
            let Some(t346) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080949f); };
            c.w32(9, t346);
            let r347 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r347);
            let Some(t348) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008094a6); };
            c.cmp(t348, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 423; continue;
        }
        423 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008094b2); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 424; continue;
        }
        424 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 425; continue;
        }
        425 => {
            let Some(t349) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008094d6); };
            c.w64(11, t349);
            let r350 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r350);
            let Some(t351) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x70008094e0); };
            c.w64(12, t351);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 428; continue; }
            blk = 426; continue;
        }
        426 => {
            blk = 1044; continue;
        }
        427 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 428; continue;
        }
        428 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008094fd); }
            c.w64(11, c.r64(0));
            blk = 431; continue;
        }
        429 => {
            return c.call(0x70008a1014, 0x700080950b, 0x7000809506);
        }
        430 => {
            c.w64(11, c.r64(0));
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080950e); };
            c.w64(0, t352);
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809512); };
            c.w64(6, t353);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809516); };
            c.w64(2, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080951a); };
            c.w64(8, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080951e); };
            c.w64(9, t356);
            blk = 431; continue;
        }
        431 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 463; continue; }
            blk = 432; continue;
        }
        432 => {
            let Some(t357) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080952c); };
            c.w32(12, t357);
            let r358 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r358);
            let Some(t359) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809533); };
            c.cmp(t359, c.r32(12), 32);
            if c.cc_e() { blk = 434; continue; }
            blk = 433; continue;
        }
        433 => {
            let Some(t360) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080953c); };
            c.cmp(t360, 0x43u64, 16);
            if c.cc_ne() { blk = 429; continue; }
            blk = 434; continue;
        }
        434 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809545); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 462; continue; }
            blk = 435; continue;
        }
        435 => {
            c.w64(10, 0x406fe00000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 461; continue; }
            blk = 436; continue;
        }
        436 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 444; continue; }
            blk = 437; continue;
        }
        437 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 443; continue; }
            blk = 438; continue;
        }
        438 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 440; continue; }
            blk = 439; continue;
        }
        439 => {
            c.mov128_rr(1, 0);
            blk = 449; continue;
        }
        440 => {
            c.mov128_rr(1, 0);
            let Some(t361) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008095aa); };
            c.w64(10, t361);
            if !c.xorpd_rm(1, c.r64(10)) { return RegfileExit::fault(0x70008095b1); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(2, 10);
            c.mov128_rr(3, 1);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(3));
            if c.cc_be() { blk = 442; continue; }
            blk = 441; continue;
        }
        441 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(3));
            c.mov128_rr(3, 1);
            blk = 442; continue;
        }
        442 => {
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008095ea); };
            c.w64(10, t362);
            if !c.xorpd_rm(3, c.r64(10)) { return RegfileExit::fault(0x70008095f1); }
            c.mov128_rr(1, 3);
            blk = 449; continue;
        }
        443 => {
            c.mov128_rr(1, 0);
            blk = 449; continue;
        }
        444 => {
            let Some(t363) = c.ld64(0x70008095b7u64) else { return RegfileExit::fault(0x70008095ff); };
            c.w64(10, t363);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 448; continue; }
            blk = 445; continue;
        }
        445 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 447; continue; }
            blk = 446; continue;
        }
        446 => {
            c.mov128_rr(1, 2);
            blk = 449; continue;
        }
        447 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.mov128_rr(1, 2);
            blk = 449; continue;
        }
        448 => {
            c.mov128_rr(1, 0);
            blk = 449; continue;
        }
        449 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x37);
            c.psrlq(2, 0x2);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 457; continue; }
            blk = 450; continue;
        }
        450 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 456; continue; }
            blk = 451; continue;
        }
        451 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x3f);
            c.psrlq(0, 0x1);
            c.mov128_rr(2, 1);
            c.w64(7, c.r64(0));
            let r364 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r364);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000809677); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700080967c); }
            if !c.movsd_mr(c.r64(4), 2) { return RegfileExit::fault(0x700080967f); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000809684); }
            blk = 452; continue;
        }
        452 => {
            c.fprem();
            c.fnstsw_ax();
            let r365 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r365);
            let r366 = c.r32(0) & 0xffu64;
            c.w32(0, r366);
            c.logic(r366, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000809693); }
            if !c.popfq() { return RegfileExit::fault(0x7000809694); }
            if c.cc_p() { blk = 452; continue; }
            blk = 453; continue;
        }
        453 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x7000809699); }
            if !c.movsd_rm(2, c.r64(4)) { return RegfileExit::fault(0x700080969c); }
            let r367 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r367);
            c.xorpd_rr(0, 0);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_p() { blk = 458; continue; }
            blk = 454; continue;
        }
        454 => {
            if c.cc_e() { blk = 459; continue; }
            blk = 455; continue;
        }
        455 => {
            blk = 458; continue;
        }
        456 => {
            c.w64(7, c.r64(0));
            blk = 459; continue;
        }
        457 => {
            c.w64(7, c.r64(0));
            blk = 458; continue;
        }
        458 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.mov128_rr(1, 0);
            blk = 459; continue;
        }
        459 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 1156; continue; }
            blk = 460; continue;
        }
        460 => {
            c.w32(11, c.r32(11));
            c.w64(0, c.r64(7));
            blk = 467; continue;
        }
        461 => {
            c.w32(11, 0xffu64);
            blk = 467; continue;
        }
        462 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 467; continue;
        }
        463 => {
            let r368 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r368);
            c.cmp(c.r32(11), 0xffu64, 32);
            if c.cc_be() { blk = 467; continue; }
            blk = 464; continue;
        }
        464 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 466; continue; }
            blk = 465; continue;
        }
        465 => {
            c.w32(11, 0xffu64);
            blk = 467; continue;
        }
        466 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(7));
            blk = 467; continue;
        }
        467 => {
            let Some(t369) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809713); };
            c.w32(7, t369);
            let r370 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r370);
            let Some(t371) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000809719); };
            c.logic(t371 & 0x4u64, 8);
            if c.cc_ne() { blk = 1100; continue; }
            blk = 468; continue;
        }
        468 => {
            let Some(t372) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000809723); };
            c.cmp(t372, c.r64(9), 64);
            if c.cc_be() { blk = 1100; continue; }
            blk = 469; continue;
        }
        469 => {
            let Some(t373) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080972d); };
            c.w64(7, t373);
            let Some(t374) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000809731); };
            c.w32(8, t374);
            let r375 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r375);
            if !c.st8(c.r64(9).wrapping_add(c.r64(7)), c.r8(11)) { return RegfileExit::fault(0x7000809738); }
            blk = 1050; continue;
        }
        470 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 476; continue; }
            blk = 471; continue;
        }
        471 => {
            c.w64(8, c.r64(1));
            let Some(t376) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809749); };
            c.w32(9, t376);
            let r377 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r377);
            let Some(t378) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809750); };
            c.cmp(t378, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 472; continue;
        }
        472 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080975c); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 473; continue;
        }
        473 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 474; continue;
        }
        474 => {
            let Some(t379) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000809780); };
            c.w64(11, t379);
            let r380 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080978a); };
            c.w64(12, t381);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 477; continue; }
            blk = 475; continue;
        }
        475 => {
            blk = 1044; continue;
        }
        476 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 477; continue;
        }
        477 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008097a7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 489; continue; }
            blk = 478; continue;
        }
        478 => {
            let Some(t382) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008097b3); };
            c.w32(11, t382);
            let r383 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r383);
            let Some(t384) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008097ba); };
            c.cmp(t384, 0x41u64, 16);
            if c.cc_e() { blk = 481; continue; }
            blk = 479; continue;
        }
        479 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008097c2); }
            c.w32(0, 0x1u64);
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008097c8); };
            c.w64(3, t385);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x70008097da, 0x70008097d5);
        }
        480 => {
            c.w64(11, c.r64(0));
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008097dd); };
            c.w64(0, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008097e1); };
            c.w64(6, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008097e5); };
            c.w64(2, t388);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008097e9); };
            c.w64(7, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008097ed); };
            c.w64(3, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008097f1); };
            c.w64(8, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008097f5); };
            c.w64(9, t392);
            blk = 482; continue;
        }
        481 => {
            c.w64(11, c.r64(0));
            blk = 482; continue;
        }
        482 => {
            let Some(t393) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008097fe); };
            c.w32(12, t393);
            let r394 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r394);
            let Some(t395) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000809805); };
            c.logic(t395 & 0x4u64, 8);
            if c.cc_ne() { blk = 1101; continue; }
            blk = 483; continue;
        }
        483 => {
            let Some(t396) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000809811); };
            c.cmp(t396, c.r64(9), 64);
            if c.cc_be() { blk = 1101; continue; }
            blk = 484; continue;
        }
        484 => {
            let Some(t397) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080981b); };
            c.w64(7, t397);
            let Some(t398) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080981f); };
            c.w32(8, t398);
            let Some(t399) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000809823); };
            c.w32(12, t399);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 487; continue; }
            blk = 485; continue;
        }
        485 => {
            let Some(t400) = c.ld64(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809830); };
            c.w64(11, t400);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 488; continue; }
            blk = 486; continue;
        }
        486 => {
            let r401 = c.neg(c.r64(11), 64);
            c.w64(11, r401);
            blk = 488; continue;
        }
        487 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 488; continue;
        }
        488 => {
            let r402 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r402);
            if !c.st64(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(11)) { return RegfileExit::fault(0x7000809845); }
            blk = 1050; continue;
        }
        489 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000809853); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000809854); }
            c.w32(0, 0x2u64);
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700080985a); };
            c.w64(3, t403);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x7000809869, 0x7000809864);
        }
        490 => {
            return RegfileExit::fault(0x7000809869);
        }
        491 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 497; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(8, c.r64(1));
            let Some(t404) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809872); };
            c.w32(9, t404);
            let r405 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r405);
            let Some(t406) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809879); };
            c.cmp(t406, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 493; continue;
        }
        493 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809885); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 494; continue;
        }
        494 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 495; continue;
        }
        495 => {
            let Some(t407) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008098a9); };
            c.w64(11, t407);
            let r408 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r408);
            let Some(t409) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x70008098b3); };
            c.w64(12, t409);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 498; continue; }
            blk = 496; continue;
        }
        496 => {
            blk = 1044; continue;
        }
        497 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 498; continue;
        }
        498 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008098d0); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 510; continue; }
            blk = 499; continue;
        }
        499 => {
            let Some(t410) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008098dc); };
            c.w32(11, t410);
            let r411 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r411);
            let Some(t412) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008098e3); };
            c.cmp(t412, 0x41u64, 16);
            if c.cc_e() { blk = 502; continue; }
            blk = 500; continue;
        }
        500 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008098eb); }
            c.w32(0, 0x1u64);
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008098f1); };
            c.w64(3, t413);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x7000809903, 0x70008098fe);
        }
        501 => {
            c.w64(11, c.r64(0));
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809906); };
            c.w64(0, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080990a); };
            c.w64(6, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080990e); };
            c.w64(2, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000809912); };
            c.w64(7, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000809916); };
            c.w64(3, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080991a); };
            c.w64(8, t419);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080991e); };
            c.w64(9, t420);
            blk = 503; continue;
        }
        502 => {
            c.w64(11, c.r64(0));
            blk = 503; continue;
        }
        503 => {
            let Some(t421) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809927); };
            c.w32(12, t421);
            let r422 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r422);
            let Some(t423) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080992e); };
            c.logic(t423 & 0x4u64, 8);
            if c.cc_ne() { blk = 1102; continue; }
            blk = 504; continue;
        }
        504 => {
            let Some(t424) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080993a); };
            c.cmp(t424, c.r64(9), 64);
            if c.cc_be() { blk = 1102; continue; }
            blk = 505; continue;
        }
        505 => {
            let Some(t425) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000809944); };
            c.w64(7, t425);
            let Some(t426) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000809948); };
            c.w32(8, t426);
            let Some(t427) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080994c); };
            c.w32(12, t427);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 508; continue; }
            blk = 506; continue;
        }
        506 => {
            let Some(t428) = c.ld64(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809959); };
            c.w64(11, t428);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 509; continue; }
            blk = 507; continue;
        }
        507 => {
            let r429 = c.neg(c.r64(11), 64);
            c.w64(11, r429);
            blk = 509; continue;
        }
        508 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 509; continue;
        }
        509 => {
            let r430 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r430);
            if !c.st64(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(11)) { return RegfileExit::fault(0x700080996e); }
            blk = 1050; continue;
        }
        510 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080997c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080997d); }
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700080997e); };
            c.w64(3, t431);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700080998f, 0x700080998a);
        }
        511 => {
            return RegfileExit::fault(0x700080998f);
        }
        512 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 518; continue; }
            blk = 513; continue;
        }
        513 => {
            c.w64(8, c.r64(1));
            let Some(t432) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809998); };
            c.w32(9, t432);
            let r433 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r433);
            let Some(t434) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080999f); };
            c.cmp(t434, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 514; continue;
        }
        514 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008099ab); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 515; continue;
        }
        515 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 516; continue;
        }
        516 => {
            let Some(t435) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x70008099cf); };
            c.w64(11, t435);
            let r436 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r436);
            let Some(t437) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x70008099d9); };
            c.w64(12, t437);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 519; continue; }
            blk = 517; continue;
        }
        517 => {
            blk = 1044; continue;
        }
        518 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 519; continue;
        }
        519 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008099f6); }
            c.w64(11, c.r64(0));
            blk = 522; continue;
        }
        520 => {
            return c.call(0x70008a1014, 0x7000809a04, 0x70008099ff);
        }
        521 => {
            c.w64(11, c.r64(0));
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809a07); };
            c.w64(0, t438);
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809a0b); };
            c.w64(6, t439);
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809a0f); };
            c.w64(2, t440);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809a13); };
            c.w64(8, t441);
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000809a17); };
            c.w64(9, t442);
            blk = 522; continue;
        }
        522 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 527; continue; }
            blk = 523; continue;
        }
        523 => {
            let Some(t443) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809a21); };
            c.w32(12, t443);
            let r444 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r444);
            let Some(t445) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809a28); };
            c.cmp(t445, c.r32(12), 32);
            if c.cc_e() { blk = 525; continue; }
            blk = 524; continue;
        }
        524 => {
            let Some(t446) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809a31); };
            c.cmp(t446, 0x43u64, 16);
            if c.cc_ne() { blk = 520; continue; }
            blk = 525; continue;
        }
        525 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809a3a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1154; continue; }
            blk = 526; continue;
        }
        526 => {
            c.w32(7, c.r32(7));
            blk = 528; continue;
        }
        527 => {
            let r447 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r447);
            c.w64(7, c.r64(11));
            blk = 528; continue;
        }
        528 => {
            c.w64(11, c.r64(2));
            let Some(t448) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809a5c); };
            c.w32(12, t448);
            let r449 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r449);
            let Some(t450) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000809a63); };
            c.w32(15, t450);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000809a68); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 554; continue; }
            blk = 529; continue;
        }
        529 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 1103; continue; }
            blk = 530; continue;
        }
        530 => {
            let Some(t451) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809a80); };
            c.w64(12, t451);
            let Some(t452) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000809a85); };
            c.w64(15, t452);
            let Some(t453) = c.ld8(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000809a89); };
            c.logic(t453 & 0x1u64, 8);
            if c.cc_ne() { blk = 533; continue; }
            blk = 531; continue;
        }
        531 => {
            let Some(t454) = c.ld64(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000809a90); };
            c.w64(3, t454);
            let r455 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r455);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1103; continue; }
            blk = 532; continue;
        }
        532 => {
            let Some(t456) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000809aa0); };
            c.w64(12, t456);
            blk = 556; continue;
        }
        533 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1103; continue; }
            blk = 534; continue;
        }
        534 => {
            let Some(t457) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809ab2); };
            c.w32(3, t457);
            let r458 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r458);
            let Some(t459) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000809ab9); };
            c.w32(3, t459);
            let r460 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r460);
            let r461 = c.r32(3) & 0x3fu64;
            c.w32(3, r461);
            c.logic(r461, 32);
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 542; continue; }
            blk = 535; continue;
        }
        535 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 539; continue; }
            blk = 536; continue;
        }
        536 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 549; continue; }
            blk = 537; continue;
        }
        537 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 552; continue; }
            blk = 538; continue;
        }
        538 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 552; continue; }
            blk = 539; continue;
        }
        539 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 551; continue; }
            blk = 540; continue;
        }
        540 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 551; continue; }
            blk = 541; continue;
        }
        541 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 552; continue; }
            blk = 542; continue;
        }
        542 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 546; continue; }
            blk = 543; continue;
        }
        543 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 550; continue; }
            blk = 544; continue;
        }
        544 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 550; continue; }
            blk = 545; continue;
        }
        545 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 551; continue; }
            blk = 546; continue;
        }
        546 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 549; continue; }
            blk = 547; continue;
        }
        547 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 549; continue; }
            blk = 548; continue;
        }
        548 => {
            return RegfileExit::fault(0x7000809b09);
        }
        549 => {
            c.w32(3, 0x1u64);
            blk = 553; continue;
        }
        550 => {
            c.w32(3, 0x2u64);
            blk = 553; continue;
        }
        551 => {
            c.w32(3, 0x4u64);
            blk = 553; continue;
        }
        552 => {
            c.w32(3, 0x8u64);
            blk = 553; continue;
        }
        553 => {
            let r462 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r462);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.cqo();
            c.idiv(c.r64(3), 64);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 556; continue;
        }
        554 => {
            c.w64(10, c.r64(4));
            let r463 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r463);
            let r464 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r464);
            c.logic(r464, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000809b4b); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t465) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000809b56); };
            c.w64(0, t465);
            c.w64(10, 0x7000809b64u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000809b64); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000809b68); }
            return c.call(c.r64(0), 0x7000809b6e, 0x7000809b6c);
        }
        555 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000809b6e); }
            let Some(t466) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000809b76); };
            c.w64(4, t466);
            c.w64(12, c.r64(0));
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809b7e); };
            c.w64(0, t467);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809b82); };
            c.w64(6, t468);
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809b86); };
            c.w64(11, t469);
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809b8a); };
            c.w64(8, t470);
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000809b8e); };
            c.w64(9, t471);
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000809b92); };
            c.w64(7, t472);
            blk = 556; continue;
        }
        556 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 1103; continue; }
            blk = 557; continue;
        }
        557 => {
            c.w64(2, c.r64(11));
            let Some(t473) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000809ba2); };
            c.w64(8, t473);
            let Some(t474) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000809ba6); };
            c.w32(11, t474);
            let r475 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r475);
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), c.r8(7)) { return RegfileExit::fault(0x7000809bad); }
            blk = 1050; continue;
        }
        558 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 564; continue; }
            blk = 559; continue;
        }
        559 => {
            c.w64(8, c.r64(1));
            let Some(t476) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809bbe); };
            c.w32(9, t476);
            let r477 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r477);
            let Some(t478) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809bc5); };
            c.cmp(t478, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 560; continue;
        }
        560 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809bd1); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 561; continue;
        }
        561 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 562; continue;
        }
        562 => {
            let Some(t479) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000809bf5); };
            c.w64(11, t479);
            let r480 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r480);
            let Some(t481) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000809bff); };
            c.w64(12, t481);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 565; continue; }
            blk = 563; continue;
        }
        563 => {
            blk = 1044; continue;
        }
        564 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 565; continue;
        }
        565 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000809c1c); }
            c.w64(11, c.r64(0));
            blk = 568; continue;
        }
        566 => {
            return c.call(0x70008a1014, 0x7000809c2a, 0x7000809c25);
        }
        567 => {
            c.w64(11, c.r64(0));
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809c2d); };
            c.w64(0, t482);
            let Some(t483) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809c31); };
            c.w64(6, t483);
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809c35); };
            c.w64(2, t484);
            let Some(t485) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809c39); };
            c.w64(8, t485);
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000809c3d); };
            c.w64(9, t486);
            blk = 568; continue;
        }
        568 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 573; continue; }
            blk = 569; continue;
        }
        569 => {
            let Some(t487) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809c47); };
            c.w32(12, t487);
            let r488 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r488);
            let Some(t489) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809c4e); };
            c.cmp(t489, c.r32(12), 32);
            if c.cc_e() { blk = 571; continue; }
            blk = 570; continue;
        }
        570 => {
            let Some(t490) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809c57); };
            c.cmp(t490, 0x43u64, 16);
            if c.cc_ne() { blk = 566; continue; }
            blk = 571; continue;
        }
        571 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809c60); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1152; continue; }
            blk = 572; continue;
        }
        572 => {
            c.w32(7, c.r32(7));
            blk = 574; continue;
        }
        573 => {
            let r491 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r491);
            c.w64(7, c.r64(11));
            blk = 574; continue;
        }
        574 => {
            c.w64(11, c.r64(2));
            let Some(t492) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809c82); };
            c.w32(12, t492);
            let r493 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r493);
            let Some(t494) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000809c89); };
            c.w32(15, t494);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000809c8e); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 600; continue; }
            blk = 575; continue;
        }
        575 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 1104; continue; }
            blk = 576; continue;
        }
        576 => {
            let Some(t495) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809ca6); };
            c.w64(12, t495);
            let Some(t496) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000809cab); };
            c.w64(15, t496);
            let Some(t497) = c.ld8(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000809caf); };
            c.logic(t497 & 0x1u64, 8);
            if c.cc_ne() { blk = 579; continue; }
            blk = 577; continue;
        }
        577 => {
            let Some(t498) = c.ld64(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000809cb6); };
            c.w64(3, t498);
            let r499 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r499);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1104; continue; }
            blk = 578; continue;
        }
        578 => {
            let Some(t500) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000809cc6); };
            c.w64(12, t500);
            blk = 602; continue;
        }
        579 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1104; continue; }
            blk = 580; continue;
        }
        580 => {
            let Some(t501) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809cd8); };
            c.w32(3, t501);
            let r502 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r502);
            let Some(t503) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000809cdf); };
            c.w32(3, t503);
            let r504 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r504);
            let r505 = c.r32(3) & 0x3fu64;
            c.w32(3, r505);
            c.logic(r505, 32);
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 588; continue; }
            blk = 581; continue;
        }
        581 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 585; continue; }
            blk = 582; continue;
        }
        582 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 595; continue; }
            blk = 583; continue;
        }
        583 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 598; continue; }
            blk = 584; continue;
        }
        584 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 598; continue; }
            blk = 585; continue;
        }
        585 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 597; continue; }
            blk = 586; continue;
        }
        586 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 597; continue; }
            blk = 587; continue;
        }
        587 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 598; continue; }
            blk = 588; continue;
        }
        588 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 592; continue; }
            blk = 589; continue;
        }
        589 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 596; continue; }
            blk = 590; continue;
        }
        590 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 596; continue; }
            blk = 591; continue;
        }
        591 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 597; continue; }
            blk = 592; continue;
        }
        592 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 595; continue; }
            blk = 593; continue;
        }
        593 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 595; continue; }
            blk = 594; continue;
        }
        594 => {
            return RegfileExit::fault(0x7000809d2f);
        }
        595 => {
            c.w32(3, 0x1u64);
            blk = 599; continue;
        }
        596 => {
            c.w32(3, 0x2u64);
            blk = 599; continue;
        }
        597 => {
            c.w32(3, 0x4u64);
            blk = 599; continue;
        }
        598 => {
            c.w32(3, 0x8u64);
            blk = 599; continue;
        }
        599 => {
            let r506 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r506);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.cqo();
            c.idiv(c.r64(3), 64);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 602; continue;
        }
        600 => {
            c.w64(10, c.r64(4));
            let r507 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r507);
            let r508 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r508);
            c.logic(r508, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000809d71); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t509) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000809d7c); };
            c.w64(0, t509);
            c.w64(10, 0x7000809d8au64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000809d8a); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000809d8e); }
            return c.call(c.r64(0), 0x7000809d94, 0x7000809d92);
        }
        601 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000809d94); }
            let Some(t510) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000809d9c); };
            c.w64(4, t510);
            c.w64(12, c.r64(0));
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809da4); };
            c.w64(0, t511);
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809da8); };
            c.w64(6, t512);
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809dac); };
            c.w64(11, t513);
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809db0); };
            c.w64(8, t514);
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000809db4); };
            c.w64(9, t515);
            let Some(t516) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000809db8); };
            c.w64(7, t516);
            blk = 602; continue;
        }
        602 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 1104; continue; }
            blk = 603; continue;
        }
        603 => {
            c.w64(2, c.r64(11));
            let Some(t517) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000809dc8); };
            c.w64(8, t517);
            let Some(t518) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000809dcc); };
            c.w32(11, t518);
            let r519 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r519);
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), c.r8(7)) { return RegfileExit::fault(0x7000809dd3); }
            blk = 1050; continue;
        }
        604 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 610; continue; }
            blk = 605; continue;
        }
        605 => {
            c.w64(8, c.r64(1));
            let Some(t520) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809de4); };
            c.w32(9, t520);
            let r521 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r521);
            let Some(t522) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809deb); };
            c.cmp(t522, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 606; continue;
        }
        606 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809df7); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 607; continue;
        }
        607 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 608; continue;
        }
        608 => {
            let Some(t523) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x7000809e1b); };
            c.w64(11, t523);
            let r524 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r524);
            let Some(t525) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x7000809e25); };
            c.w64(12, t525);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 611; continue; }
            blk = 609; continue;
        }
        609 => {
            blk = 1044; continue;
        }
        610 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 611; continue;
        }
        611 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000809e42); }
            c.w64(11, c.r64(0));
            blk = 614; continue;
        }
        612 => {
            return c.call(0x70008a1014, 0x7000809e50, 0x7000809e4b);
        }
        613 => {
            c.w64(11, c.r64(0));
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809e53); };
            c.w64(0, t526);
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809e57); };
            c.w64(6, t527);
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809e5b); };
            c.w64(2, t528);
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809e5f); };
            c.w64(8, t529);
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000809e63); };
            c.w64(9, t530);
            blk = 614; continue;
        }
        614 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 619; continue; }
            blk = 615; continue;
        }
        615 => {
            let Some(t531) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809e6d); };
            c.w32(12, t531);
            let r532 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r532);
            let Some(t533) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000809e74); };
            c.cmp(t533, c.r32(12), 32);
            if c.cc_e() { blk = 617; continue; }
            blk = 616; continue;
        }
        616 => {
            let Some(t534) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000809e7d); };
            c.cmp(t534, 0x43u64, 16);
            if c.cc_ne() { blk = 612; continue; }
            blk = 617; continue;
        }
        617 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000809e86); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1150; continue; }
            blk = 618; continue;
        }
        618 => {
            c.w32(7, c.r32(7));
            blk = 620; continue;
        }
        619 => {
            let r535 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r535);
            c.w64(7, c.r64(11));
            blk = 620; continue;
        }
        620 => {
            c.w64(11, c.r64(2));
            let Some(t536) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809ea8); };
            c.w32(12, t536);
            let r537 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r537);
            let Some(t538) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000809eaf); };
            c.w32(15, t538);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x7000809eb4); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 646; continue; }
            blk = 621; continue;
        }
        621 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 1105; continue; }
            blk = 622; continue;
        }
        622 => {
            let Some(t539) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000809ecc); };
            c.w64(12, t539);
            let Some(t540) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000809ed1); };
            c.w64(15, t540);
            let Some(t541) = c.ld8(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000809ed5); };
            c.logic(t541 & 0x1u64, 8);
            if c.cc_ne() { blk = 625; continue; }
            blk = 623; continue;
        }
        623 => {
            let Some(t542) = c.ld64(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000809edc); };
            c.w64(3, t542);
            let r543 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r543);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1105; continue; }
            blk = 624; continue;
        }
        624 => {
            let Some(t544) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000809eec); };
            c.w64(12, t544);
            blk = 648; continue;
        }
        625 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1105; continue; }
            blk = 626; continue;
        }
        626 => {
            let Some(t545) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000809efe); };
            c.w32(3, t545);
            let r546 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r546);
            let Some(t547) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000809f05); };
            c.w32(3, t547);
            let r548 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r548);
            let r549 = c.r32(3) & 0x3fu64;
            c.w32(3, r549);
            c.logic(r549, 32);
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 634; continue; }
            blk = 627; continue;
        }
        627 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 631; continue; }
            blk = 628; continue;
        }
        628 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 641; continue; }
            blk = 629; continue;
        }
        629 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 644; continue; }
            blk = 630; continue;
        }
        630 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 644; continue; }
            blk = 631; continue;
        }
        631 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 643; continue; }
            blk = 632; continue;
        }
        632 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 643; continue; }
            blk = 633; continue;
        }
        633 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 644; continue; }
            blk = 634; continue;
        }
        634 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 638; continue; }
            blk = 635; continue;
        }
        635 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 642; continue; }
            blk = 636; continue;
        }
        636 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 642; continue; }
            blk = 637; continue;
        }
        637 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 643; continue; }
            blk = 638; continue;
        }
        638 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 641; continue; }
            blk = 639; continue;
        }
        639 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 641; continue; }
            blk = 640; continue;
        }
        640 => {
            return RegfileExit::fault(0x7000809f55);
        }
        641 => {
            c.w32(3, 0x1u64);
            blk = 645; continue;
        }
        642 => {
            c.w32(3, 0x2u64);
            blk = 645; continue;
        }
        643 => {
            c.w32(3, 0x4u64);
            blk = 645; continue;
        }
        644 => {
            c.w32(3, 0x8u64);
            blk = 645; continue;
        }
        645 => {
            let r550 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r550);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.cqo();
            c.idiv(c.r64(3), 64);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 648; continue;
        }
        646 => {
            c.w64(10, c.r64(4));
            let r551 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r551);
            let r552 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r552);
            c.logic(r552, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000809f97); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t553) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000809fa2); };
            c.w64(0, t553);
            c.w64(10, 0x7000809fb0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000809fb0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000809fb4); }
            return c.call(c.r64(0), 0x7000809fba, 0x7000809fb8);
        }
        647 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000809fba); }
            let Some(t554) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000809fc2); };
            c.w64(4, t554);
            c.w64(12, c.r64(0));
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000809fca); };
            c.w64(0, t555);
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000809fce); };
            c.w64(6, t556);
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000809fd2); };
            c.w64(11, t557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000809fd6); };
            c.w64(8, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000809fda); };
            c.w64(9, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000809fde); };
            c.w64(7, t560);
            blk = 648; continue;
        }
        648 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 1105; continue; }
            blk = 649; continue;
        }
        649 => {
            c.w64(2, c.r64(11));
            let Some(t561) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000809fee); };
            c.w64(8, t561);
            let Some(t562) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000809ff2); };
            c.w32(11, t562);
            let r563 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r563);
            if !c.st16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x7000809ff9); }
            blk = 1050; continue;
        }
        650 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 656; continue; }
            blk = 651; continue;
        }
        651 => {
            c.w64(8, c.r64(1));
            let Some(t564) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a00b); };
            c.w32(9, t564);
            let r565 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r565);
            let Some(t566) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a012); };
            c.cmp(t566, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 652; continue;
        }
        652 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a01e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 653; continue;
        }
        653 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 654; continue;
        }
        654 => {
            let Some(t567) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080a042); };
            c.w64(11, t567);
            let r568 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r568);
            let Some(t569) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080a04c); };
            c.w64(12, t569);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 657; continue; }
            blk = 655; continue;
        }
        655 => {
            blk = 1044; continue;
        }
        656 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 657; continue;
        }
        657 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080a069); }
            c.w64(11, c.r64(0));
            blk = 660; continue;
        }
        658 => {
            return c.call(0x70008a1014, 0x700080a077, 0x700080a072);
        }
        659 => {
            c.w64(11, c.r64(0));
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a07a); };
            c.w64(0, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a07e); };
            c.w64(6, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a082); };
            c.w64(2, t572);
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a086); };
            c.w64(8, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a08a); };
            c.w64(9, t574);
            blk = 660; continue;
        }
        660 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 665; continue; }
            blk = 661; continue;
        }
        661 => {
            let Some(t575) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a094); };
            c.w32(12, t575);
            let r576 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r576);
            let Some(t577) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080a09b); };
            c.cmp(t577, c.r32(12), 32);
            if c.cc_e() { blk = 663; continue; }
            blk = 662; continue;
        }
        662 => {
            let Some(t578) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a0a4); };
            c.cmp(t578, 0x43u64, 16);
            if c.cc_ne() { blk = 658; continue; }
            blk = 663; continue;
        }
        663 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a0ad); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1148; continue; }
            blk = 664; continue;
        }
        664 => {
            c.w32(7, c.r32(7));
            blk = 666; continue;
        }
        665 => {
            let r579 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r579);
            c.w64(7, c.r64(11));
            blk = 666; continue;
        }
        666 => {
            c.w64(11, c.r64(2));
            let Some(t580) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a0cf); };
            c.w32(12, t580);
            let r581 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r581);
            let Some(t582) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080a0d6); };
            c.w32(15, t582);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700080a0db); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 692; continue; }
            blk = 667; continue;
        }
        667 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 1106; continue; }
            blk = 668; continue;
        }
        668 => {
            let Some(t583) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a0f3); };
            c.w64(12, t583);
            let Some(t584) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080a0f8); };
            c.w64(15, t584);
            let Some(t585) = c.ld8(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080a0fc); };
            c.logic(t585 & 0x1u64, 8);
            if c.cc_ne() { blk = 671; continue; }
            blk = 669; continue;
        }
        669 => {
            let Some(t586) = c.ld64(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080a103); };
            c.w64(3, t586);
            let r587 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r587);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1106; continue; }
            blk = 670; continue;
        }
        670 => {
            let Some(t588) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080a113); };
            c.w64(12, t588);
            blk = 694; continue;
        }
        671 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1106; continue; }
            blk = 672; continue;
        }
        672 => {
            let Some(t589) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a125); };
            c.w32(3, t589);
            let r590 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r590);
            let Some(t591) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080a12c); };
            c.w32(3, t591);
            let r592 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r592);
            let r593 = c.r32(3) & 0x3fu64;
            c.w32(3, r593);
            c.logic(r593, 32);
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 680; continue; }
            blk = 673; continue;
        }
        673 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 677; continue; }
            blk = 674; continue;
        }
        674 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 687; continue; }
            blk = 675; continue;
        }
        675 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 690; continue; }
            blk = 676; continue;
        }
        676 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 690; continue; }
            blk = 677; continue;
        }
        677 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 689; continue; }
            blk = 678; continue;
        }
        678 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 689; continue; }
            blk = 679; continue;
        }
        679 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 690; continue; }
            blk = 680; continue;
        }
        680 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 684; continue; }
            blk = 681; continue;
        }
        681 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 688; continue; }
            blk = 682; continue;
        }
        682 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 688; continue; }
            blk = 683; continue;
        }
        683 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 689; continue; }
            blk = 684; continue;
        }
        684 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 687; continue; }
            blk = 685; continue;
        }
        685 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 687; continue; }
            blk = 686; continue;
        }
        686 => {
            return RegfileExit::fault(0x700080a17c);
        }
        687 => {
            c.w32(3, 0x1u64);
            blk = 691; continue;
        }
        688 => {
            c.w32(3, 0x2u64);
            blk = 691; continue;
        }
        689 => {
            c.w32(3, 0x4u64);
            blk = 691; continue;
        }
        690 => {
            c.w32(3, 0x8u64);
            blk = 691; continue;
        }
        691 => {
            let r594 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r594);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.cqo();
            c.idiv(c.r64(3), 64);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 694; continue;
        }
        692 => {
            c.w64(10, c.r64(4));
            let r595 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r595);
            let r596 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r596);
            c.logic(r596, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080a1be); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t597) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080a1c9); };
            c.w64(0, t597);
            c.w64(10, 0x700080a1d7u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080a1d7); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080a1db); }
            return c.call(c.r64(0), 0x700080a1e1, 0x700080a1df);
        }
        693 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080a1e1); }
            let Some(t598) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080a1e9); };
            c.w64(4, t598);
            c.w64(12, c.r64(0));
            let Some(t599) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a1f1); };
            c.w64(0, t599);
            let Some(t600) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a1f5); };
            c.w64(6, t600);
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a1f9); };
            c.w64(11, t601);
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a1fd); };
            c.w64(8, t602);
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a201); };
            c.w64(9, t603);
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080a205); };
            c.w64(7, t604);
            blk = 694; continue;
        }
        694 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 1106; continue; }
            blk = 695; continue;
        }
        695 => {
            c.w64(2, c.r64(11));
            let Some(t605) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080a215); };
            c.w64(8, t605);
            let Some(t606) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080a219); };
            c.w32(11, t606);
            let r607 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r607);
            if !c.st16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x700080a220); }
            blk = 1050; continue;
        }
        696 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 702; continue; }
            blk = 697; continue;
        }
        697 => {
            c.w64(8, c.r64(1));
            let Some(t608) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a232); };
            c.w32(9, t608);
            let r609 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r609);
            let Some(t610) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a239); };
            c.cmp(t610, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 698; continue;
        }
        698 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a245); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 699; continue;
        }
        699 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 700; continue;
        }
        700 => {
            let Some(t611) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080a269); };
            c.w64(11, t611);
            let r612 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r612);
            let Some(t613) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080a273); };
            c.w64(12, t613);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 703; continue; }
            blk = 701; continue;
        }
        701 => {
            blk = 1044; continue;
        }
        702 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 703; continue;
        }
        703 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080a290); }
            c.w64(11, c.r64(0));
            blk = 706; continue;
        }
        704 => {
            return c.call(0x70008a1014, 0x700080a29e, 0x700080a299);
        }
        705 => {
            c.w64(11, c.r64(0));
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a2a1); };
            c.w64(0, t614);
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a2a5); };
            c.w64(6, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a2a9); };
            c.w64(2, t616);
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a2ad); };
            c.w64(8, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a2b1); };
            c.w64(9, t618);
            blk = 706; continue;
        }
        706 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 711; continue; }
            blk = 707; continue;
        }
        707 => {
            let Some(t619) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a2bb); };
            c.w32(12, t619);
            let r620 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r620);
            let Some(t621) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080a2c2); };
            c.cmp(t621, c.r32(12), 32);
            if c.cc_e() { blk = 709; continue; }
            blk = 708; continue;
        }
        708 => {
            let Some(t622) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a2cb); };
            c.cmp(t622, 0x43u64, 16);
            if c.cc_ne() { blk = 704; continue; }
            blk = 709; continue;
        }
        709 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a2d4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1146; continue; }
            blk = 710; continue;
        }
        710 => {
            c.w32(7, c.r32(7));
            blk = 712; continue;
        }
        711 => {
            let r623 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r623);
            c.w64(7, c.r64(11));
            blk = 712; continue;
        }
        712 => {
            c.w64(11, c.r64(2));
            let Some(t624) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a2f6); };
            c.w32(12, t624);
            let r625 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r625);
            let Some(t626) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080a2fd); };
            c.w32(15, t626);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700080a302); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 738; continue; }
            blk = 713; continue;
        }
        713 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 1107; continue; }
            blk = 714; continue;
        }
        714 => {
            let Some(t627) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a31a); };
            c.w64(12, t627);
            let Some(t628) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080a31f); };
            c.w64(15, t628);
            let Some(t629) = c.ld8(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080a323); };
            c.logic(t629 & 0x1u64, 8);
            if c.cc_ne() { blk = 717; continue; }
            blk = 715; continue;
        }
        715 => {
            let Some(t630) = c.ld64(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080a32a); };
            c.w64(3, t630);
            let r631 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r631);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1107; continue; }
            blk = 716; continue;
        }
        716 => {
            let Some(t632) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080a33a); };
            c.w64(12, t632);
            blk = 740; continue;
        }
        717 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1107; continue; }
            blk = 718; continue;
        }
        718 => {
            let Some(t633) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a34c); };
            c.w32(3, t633);
            let r634 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r634);
            let Some(t635) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080a353); };
            c.w32(3, t635);
            let r636 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r636);
            let r637 = c.r32(3) & 0x3fu64;
            c.w32(3, r637);
            c.logic(r637, 32);
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 726; continue; }
            blk = 719; continue;
        }
        719 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 723; continue; }
            blk = 720; continue;
        }
        720 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 733; continue; }
            blk = 721; continue;
        }
        721 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 736; continue; }
            blk = 722; continue;
        }
        722 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 736; continue; }
            blk = 723; continue;
        }
        723 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 735; continue; }
            blk = 724; continue;
        }
        724 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 735; continue; }
            blk = 725; continue;
        }
        725 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 736; continue; }
            blk = 726; continue;
        }
        726 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 730; continue; }
            blk = 727; continue;
        }
        727 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 734; continue; }
            blk = 728; continue;
        }
        728 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 734; continue; }
            blk = 729; continue;
        }
        729 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 735; continue; }
            blk = 730; continue;
        }
        730 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 733; continue; }
            blk = 731; continue;
        }
        731 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 733; continue; }
            blk = 732; continue;
        }
        732 => {
            return RegfileExit::fault(0x700080a3a3);
        }
        733 => {
            c.w32(3, 0x1u64);
            blk = 737; continue;
        }
        734 => {
            c.w32(3, 0x2u64);
            blk = 737; continue;
        }
        735 => {
            c.w32(3, 0x4u64);
            blk = 737; continue;
        }
        736 => {
            c.w32(3, 0x8u64);
            blk = 737; continue;
        }
        737 => {
            let r638 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r638);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.cqo();
            c.idiv(c.r64(3), 64);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 740; continue;
        }
        738 => {
            c.w64(10, c.r64(4));
            let r639 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r639);
            let r640 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r640);
            c.logic(r640, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080a3e5); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t641) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080a3f0); };
            c.w64(0, t641);
            c.w64(10, 0x700080a3feu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080a3fe); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080a402); }
            return c.call(c.r64(0), 0x700080a408, 0x700080a406);
        }
        739 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080a408); }
            let Some(t642) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080a410); };
            c.w64(4, t642);
            c.w64(12, c.r64(0));
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a418); };
            c.w64(0, t643);
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a41c); };
            c.w64(6, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a420); };
            c.w64(11, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a424); };
            c.w64(8, t646);
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a428); };
            c.w64(9, t647);
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080a42c); };
            c.w64(7, t648);
            blk = 740; continue;
        }
        740 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 1107; continue; }
            blk = 741; continue;
        }
        741 => {
            c.w64(2, c.r64(11));
            let Some(t649) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080a43c); };
            c.w64(8, t649);
            let Some(t650) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080a440); };
            c.w32(11, t650);
            let r651 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r651);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x700080a447); }
            blk = 1050; continue;
        }
        742 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 748; continue; }
            blk = 743; continue;
        }
        743 => {
            c.w64(8, c.r64(1));
            let Some(t652) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a458); };
            c.w32(9, t652);
            let r653 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r653);
            let Some(t654) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a45f); };
            c.cmp(t654, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 744; continue;
        }
        744 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a46b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 745; continue;
        }
        745 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 746; continue;
        }
        746 => {
            let Some(t655) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080a48f); };
            c.w64(11, t655);
            let r656 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r656);
            let Some(t657) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080a499); };
            c.w64(12, t657);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 749; continue; }
            blk = 747; continue;
        }
        747 => {
            blk = 1044; continue;
        }
        748 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 749; continue;
        }
        749 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080a4b6); }
            c.w64(11, c.r64(0));
            blk = 752; continue;
        }
        750 => {
            return c.call(0x70008a1014, 0x700080a4c4, 0x700080a4bf);
        }
        751 => {
            c.w64(11, c.r64(0));
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a4c7); };
            c.w64(0, t658);
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a4cb); };
            c.w64(6, t659);
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a4cf); };
            c.w64(2, t660);
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a4d3); };
            c.w64(8, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a4d7); };
            c.w64(9, t662);
            blk = 752; continue;
        }
        752 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 757; continue; }
            blk = 753; continue;
        }
        753 => {
            let Some(t663) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a4e1); };
            c.w32(12, t663);
            let r664 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r664);
            let Some(t665) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080a4e8); };
            c.cmp(t665, c.r32(12), 32);
            if c.cc_e() { blk = 755; continue; }
            blk = 754; continue;
        }
        754 => {
            let Some(t666) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a4f1); };
            c.cmp(t666, 0x43u64, 16);
            if c.cc_ne() { blk = 750; continue; }
            blk = 755; continue;
        }
        755 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a4fa); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1144; continue; }
            blk = 756; continue;
        }
        756 => {
            c.w32(7, c.r32(7));
            blk = 758; continue;
        }
        757 => {
            let r667 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r667);
            c.w64(7, c.r64(11));
            blk = 758; continue;
        }
        758 => {
            c.w64(11, c.r64(2));
            let Some(t668) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a51c); };
            c.w32(12, t668);
            let r669 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r669);
            let Some(t670) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080a523); };
            c.w32(15, t670);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(7)) { return RegfileExit::fault(0x700080a528); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 784; continue; }
            blk = 759; continue;
        }
        759 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 1115; continue; }
            blk = 760; continue;
        }
        760 => {
            let Some(t671) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a540); };
            c.w64(12, t671);
            let Some(t672) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080a545); };
            c.w64(15, t672);
            let Some(t673) = c.ld8(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080a549); };
            c.logic(t673 & 0x1u64, 8);
            if c.cc_ne() { blk = 763; continue; }
            blk = 761; continue;
        }
        761 => {
            let Some(t674) = c.ld64(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080a550); };
            c.w64(3, t674);
            let r675 = c.add(c.r64(15), c.r64(3), 64);
            c.w64(15, r675);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1115; continue; }
            blk = 762; continue;
        }
        762 => {
            let Some(t676) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080a560); };
            c.w64(12, t676);
            blk = 786; continue;
        }
        763 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 1115; continue; }
            blk = 764; continue;
        }
        764 => {
            let Some(t677) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a572); };
            c.w32(3, t677);
            let r678 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r678);
            let Some(t679) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080a579); };
            c.w32(3, t679);
            let r680 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r680);
            let r681 = c.r32(3) & 0x3fu64;
            c.w32(3, r681);
            c.logic(r681, 32);
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 772; continue; }
            blk = 765; continue;
        }
        765 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 769; continue; }
            blk = 766; continue;
        }
        766 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 779; continue; }
            blk = 767; continue;
        }
        767 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 782; continue; }
            blk = 768; continue;
        }
        768 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 782; continue; }
            blk = 769; continue;
        }
        769 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 781; continue; }
            blk = 770; continue;
        }
        770 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 781; continue; }
            blk = 771; continue;
        }
        771 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 782; continue; }
            blk = 772; continue;
        }
        772 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 776; continue; }
            blk = 773; continue;
        }
        773 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 780; continue; }
            blk = 774; continue;
        }
        774 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 780; continue; }
            blk = 775; continue;
        }
        775 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 781; continue; }
            blk = 776; continue;
        }
        776 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 779; continue; }
            blk = 777; continue;
        }
        777 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 779; continue; }
            blk = 778; continue;
        }
        778 => {
            return RegfileExit::fault(0x700080a5c9);
        }
        779 => {
            c.w32(3, 0x1u64);
            blk = 783; continue;
        }
        780 => {
            c.w32(3, 0x2u64);
            blk = 783; continue;
        }
        781 => {
            c.w32(3, 0x4u64);
            blk = 783; continue;
        }
        782 => {
            c.w32(3, 0x8u64);
            blk = 783; continue;
        }
        783 => {
            let r682 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r682);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            c.cqo();
            c.idiv(c.r64(3), 64);
            c.w64(10, c.r64(12));
            c.w64(12, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 786; continue;
        }
        784 => {
            c.w64(10, c.r64(4));
            let r683 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r683);
            let r684 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r684);
            c.logic(r684, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080a60b); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t685) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080a616); };
            c.w64(0, t685);
            c.w64(10, 0x700080a624u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080a624); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080a628); }
            return c.call(c.r64(0), 0x700080a62e, 0x700080a62c);
        }
        785 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080a62e); }
            let Some(t686) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080a636); };
            c.w64(4, t686);
            c.w64(12, c.r64(0));
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a63e); };
            c.w64(0, t687);
            let Some(t688) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a642); };
            c.w64(6, t688);
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a646); };
            c.w64(11, t689);
            let Some(t690) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a64a); };
            c.w64(8, t690);
            let Some(t691) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a64e); };
            c.w64(9, t691);
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080a652); };
            c.w64(7, t692);
            blk = 786; continue;
        }
        786 => {
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 1115; continue; }
            blk = 787; continue;
        }
        787 => {
            c.w64(2, c.r64(11));
            let Some(t693) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080a662); };
            c.w64(8, t693);
            let Some(t694) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080a666); };
            c.w32(11, t694);
            let r695 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r695);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x700080a66d); }
            blk = 1050; continue;
        }
        788 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 794; continue; }
            blk = 789; continue;
        }
        789 => {
            c.w64(8, c.r64(1));
            let Some(t696) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a67e); };
            c.w32(9, t696);
            let r697 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r697);
            let Some(t698) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a685); };
            c.cmp(t698, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 790; continue;
        }
        790 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a691); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 791; continue;
        }
        791 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 792; continue;
        }
        792 => {
            let Some(t699) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080a6b5); };
            c.w64(11, t699);
            let r700 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r700);
            let Some(t701) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080a6bf); };
            c.w64(12, t701);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 795; continue; }
            blk = 793; continue;
        }
        793 => {
            blk = 1044; continue;
        }
        794 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 795; continue;
        }
        795 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080a6dc); }
            c.w64(11, c.r64(0));
            blk = 798; continue;
        }
        796 => {
            return c.call(0x70008a1014, 0x700080a6ea, 0x700080a6e5);
        }
        797 => {
            c.w64(11, c.r64(0));
            let Some(t702) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a6ed); };
            c.w64(0, t702);
            let Some(t703) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a6f1); };
            c.w64(6, t703);
            let Some(t704) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a6f5); };
            c.w64(2, t704);
            let Some(t705) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a6f9); };
            c.w64(8, t705);
            let Some(t706) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a6fd); };
            c.w64(9, t706);
            blk = 798; continue;
        }
        798 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 802; continue; }
            blk = 799; continue;
        }
        799 => {
            let Some(t707) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a707); };
            c.w32(12, t707);
            let r708 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r708);
            let Some(t709) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080a70e); };
            c.cmp(t709, c.r32(12), 32);
            if c.cc_e() { blk = 801; continue; }
            blk = 800; continue;
        }
        800 => {
            let Some(t710) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a717); };
            c.cmp(t710, 0x43u64, 16);
            if c.cc_ne() { blk = 796; continue; }
            blk = 801; continue;
        }
        801 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a720); }
            c.cvtsd2ss(0, c.xmm_u64(0));
            blk = 803; continue;
        }
        802 => {
            let r711 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r711);
            c.xorpd_rr(0, 0);
            c.cvtsi2ss(0, (c.r32(11) as u32 as i32 as i64));
            blk = 803; continue;
        }
        803 => {
            c.w64(7, c.r64(2));
            let Some(t712) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a73a); };
            c.w32(11, t712);
            let r713 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r713);
            let Some(t714) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080a741); };
            c.w32(12, t714);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700080a745); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 829; continue; }
            blk = 804; continue;
        }
        804 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1123; continue; }
            blk = 805; continue;
        }
        805 => {
            let Some(t715) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a75e); };
            c.w64(11, t715);
            let Some(t716) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080a762); };
            c.w64(12, t716);
            let Some(t717) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080a766); };
            c.logic(t717 & 0x1u64, 8);
            if c.cc_ne() { blk = 808; continue; }
            blk = 806; continue;
        }
        806 => {
            let Some(t718) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080a76c); };
            c.w64(15, t718);
            let r719 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r719);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1123; continue; }
            blk = 807; continue;
        }
        807 => {
            let Some(t720) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080a77c); };
            c.w64(11, t720);
            blk = 831; continue;
        }
        808 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1123; continue; }
            blk = 809; continue;
        }
        809 => {
            let Some(t721) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a78e); };
            c.w32(15, t721);
            let r722 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r722);
            let Some(t723) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080a795); };
            c.w32(15, t723);
            let r724 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r724);
            let r725 = c.r32(15) & 0x3fu64;
            c.w32(15, r725);
            c.logic(r725, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 817; continue; }
            blk = 810; continue;
        }
        810 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 814; continue; }
            blk = 811; continue;
        }
        811 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 824; continue; }
            blk = 812; continue;
        }
        812 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 827; continue; }
            blk = 813; continue;
        }
        813 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 827; continue; }
            blk = 814; continue;
        }
        814 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 826; continue; }
            blk = 815; continue;
        }
        815 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 826; continue; }
            blk = 816; continue;
        }
        816 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 827; continue; }
            blk = 817; continue;
        }
        817 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 821; continue; }
            blk = 818; continue;
        }
        818 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 825; continue; }
            blk = 819; continue;
        }
        819 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 825; continue; }
            blk = 820; continue;
        }
        820 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 826; continue; }
            blk = 821; continue;
        }
        821 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 824; continue; }
            blk = 822; continue;
        }
        822 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 824; continue; }
            blk = 823; continue;
        }
        823 => {
            return RegfileExit::fault(0x700080a7fa);
        }
        824 => {
            c.w32(15, 0x1u64);
            blk = 828; continue;
        }
        825 => {
            c.w32(15, 0x2u64);
            blk = 828; continue;
        }
        826 => {
            c.w32(15, 0x4u64);
            blk = 828; continue;
        }
        827 => {
            c.w32(15, 0x8u64);
            blk = 828; continue;
        }
        828 => {
            let r726 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r726);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 831; continue;
        }
        829 => {
            c.w64(10, c.r64(4));
            let r727 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r727);
            let r728 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r728);
            c.logic(r728, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080a840); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t729) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080a84b); };
            c.w64(0, t729);
            c.w64(10, 0x700080a859u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080a859); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080a85d); }
            return c.call(c.r64(0), 0x700080a863, 0x700080a861);
        }
        830 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080a863); }
            let Some(t730) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080a86b); };
            c.w64(4, t730);
            c.w64(11, c.r64(0));
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a873); };
            c.w64(0, t731);
            let Some(t732) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a877); };
            c.w64(6, t732);
            let Some(t733) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a87b); };
            c.w64(7, t733);
            let Some(t734) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a87f); };
            c.w64(8, t734);
            let Some(t735) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a883); };
            c.w64(9, t735);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700080a887); }
            blk = 831; continue;
        }
        831 => {
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ae() { blk = 1123; continue; }
            blk = 832; continue;
        }
        832 => {
            c.w64(2, c.r64(7));
            let Some(t736) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080a898); };
            c.w64(7, t736);
            let Some(t737) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080a89c); };
            c.w32(8, t737);
            let r738 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r738);
            if !c.movss_mr(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)), 0) { return RegfileExit::fault(0x700080a8a3); }
            blk = 1050; continue;
        }
        833 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 839; continue; }
            blk = 834; continue;
        }
        834 => {
            c.w64(8, c.r64(1));
            let Some(t739) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a8b6); };
            c.w32(9, t739);
            let r740 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r740);
            let Some(t741) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a8bd); };
            c.cmp(t741, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 835; continue;
        }
        835 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a8c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 836; continue;
        }
        836 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 837; continue;
        }
        837 => {
            let Some(t742) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080a8ed); };
            c.w64(11, t742);
            let r743 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r743);
            let Some(t744) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080a8f7); };
            c.w64(12, t744);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 840; continue; }
            blk = 838; continue;
        }
        838 => {
            blk = 1044; continue;
        }
        839 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 840; continue;
        }
        840 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080a914); }
            c.w64(11, c.r64(0));
            blk = 843; continue;
        }
        841 => {
            return c.call(0x70008a1014, 0x700080a922, 0x700080a91d);
        }
        842 => {
            c.w64(11, c.r64(0));
            let Some(t745) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080a925); };
            c.w64(0, t745);
            let Some(t746) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080a929); };
            c.w64(6, t746);
            let Some(t747) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080a92d); };
            c.w64(2, t747);
            let Some(t748) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080a931); };
            c.w64(8, t748);
            let Some(t749) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080a935); };
            c.w64(9, t749);
            blk = 843; continue;
        }
        843 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 847; continue; }
            blk = 844; continue;
        }
        844 => {
            let Some(t750) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a93f); };
            c.w32(12, t750);
            let r751 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r751);
            let Some(t752) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080a946); };
            c.cmp(t752, c.r32(12), 32);
            if c.cc_e() { blk = 846; continue; }
            blk = 845; continue;
        }
        845 => {
            let Some(t753) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080a94f); };
            c.cmp(t753, 0x43u64, 16);
            if c.cc_ne() { blk = 841; continue; }
            blk = 846; continue;
        }
        846 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080a958); }
            blk = 848; continue;
        }
        847 => {
            let r754 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r754);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 848; continue;
        }
        848 => {
            c.w64(7, c.r64(2));
            let Some(t755) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a96f); };
            c.w32(11, t755);
            let r756 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r756);
            let Some(t757) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080a976); };
            c.w32(12, t757);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x28), 0) { return RegfileExit::fault(0x700080a97a); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 874; continue; }
            blk = 849; continue;
        }
        849 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1131; continue; }
            blk = 850; continue;
        }
        850 => {
            let Some(t758) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080a993); };
            c.w64(11, t758);
            let Some(t759) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080a997); };
            c.w64(12, t759);
            let Some(t760) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080a99b); };
            c.logic(t760 & 0x1u64, 8);
            if c.cc_ne() { blk = 853; continue; }
            blk = 851; continue;
        }
        851 => {
            let Some(t761) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080a9a1); };
            c.w64(15, t761);
            let r762 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r762);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1131; continue; }
            blk = 852; continue;
        }
        852 => {
            let Some(t763) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080a9b1); };
            c.w64(11, t763);
            blk = 876; continue;
        }
        853 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1131; continue; }
            blk = 854; continue;
        }
        854 => {
            let Some(t764) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080a9c3); };
            c.w32(15, t764);
            let r765 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r765);
            let Some(t766) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080a9ca); };
            c.w32(15, t766);
            let r767 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r767);
            let r768 = c.r32(15) & 0x3fu64;
            c.w32(15, r768);
            c.logic(r768, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 862; continue; }
            blk = 855; continue;
        }
        855 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 859; continue; }
            blk = 856; continue;
        }
        856 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 869; continue; }
            blk = 857; continue;
        }
        857 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 872; continue; }
            blk = 858; continue;
        }
        858 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 872; continue; }
            blk = 859; continue;
        }
        859 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 871; continue; }
            blk = 860; continue;
        }
        860 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 871; continue; }
            blk = 861; continue;
        }
        861 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 872; continue; }
            blk = 862; continue;
        }
        862 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 866; continue; }
            blk = 863; continue;
        }
        863 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 870; continue; }
            blk = 864; continue;
        }
        864 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 870; continue; }
            blk = 865; continue;
        }
        865 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 871; continue; }
            blk = 866; continue;
        }
        866 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 869; continue; }
            blk = 867; continue;
        }
        867 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 869; continue; }
            blk = 868; continue;
        }
        868 => {
            return RegfileExit::fault(0x700080aa2f);
        }
        869 => {
            c.w32(15, 0x1u64);
            blk = 873; continue;
        }
        870 => {
            c.w32(15, 0x2u64);
            blk = 873; continue;
        }
        871 => {
            c.w32(15, 0x4u64);
            blk = 873; continue;
        }
        872 => {
            c.w32(15, 0x8u64);
            blk = 873; continue;
        }
        873 => {
            let r769 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r769);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 876; continue;
        }
        874 => {
            c.w64(10, c.r64(4));
            let r770 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r770);
            let r771 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r771);
            c.logic(r771, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080aa75); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t772) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080aa80); };
            c.w64(0, t772);
            c.w64(10, 0x700080aa8eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080aa8e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080aa92); }
            return c.call(c.r64(0), 0x700080aa98, 0x700080aa96);
        }
        875 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080aa98); }
            let Some(t773) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080aaa0); };
            c.w64(4, t773);
            c.w64(11, c.r64(0));
            let Some(t774) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080aaa8); };
            c.w64(0, t774);
            let Some(t775) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080aaac); };
            c.w64(6, t775);
            let Some(t776) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080aab0); };
            c.w64(7, t776);
            let Some(t777) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080aab4); };
            c.w64(8, t777);
            let Some(t778) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080aab8); };
            c.w64(9, t778);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700080aabc); }
            blk = 876; continue;
        }
        876 => {
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ae() { blk = 1131; continue; }
            blk = 877; continue;
        }
        877 => {
            c.w64(2, c.r64(7));
            let Some(t779) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080aacd); };
            c.w64(7, t779);
            let Some(t780) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080aad1); };
            c.w32(8, t780);
            let r781 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r781);
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), 0) { return RegfileExit::fault(0x700080aad8); }
            blk = 1050; continue;
        }
        878 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 884; continue; }
            blk = 879; continue;
        }
        879 => {
            c.w64(8, c.r64(1));
            let Some(t782) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080aaeb); };
            c.w32(9, t782);
            let r783 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r783);
            let Some(t784) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080aaf2); };
            c.cmp(t784, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 880; continue;
        }
        880 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080aafe); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 881; continue;
        }
        881 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 882; continue;
        }
        882 => {
            let Some(t785) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080ab22); };
            c.w64(11, t785);
            let r786 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r786);
            let Some(t787) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080ab2c); };
            c.w64(12, t787);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 885; continue; }
            blk = 883; continue;
        }
        883 => {
            blk = 1044; continue;
        }
        884 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 885; continue;
        }
        885 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080ab49); }
            c.w64(11, c.r64(0));
            blk = 888; continue;
        }
        886 => {
            return c.call(0x70008a1014, 0x700080ab57, 0x700080ab52);
        }
        887 => {
            c.w64(11, c.r64(0));
            let Some(t788) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080ab5a); };
            c.w64(0, t788);
            let Some(t789) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080ab5e); };
            c.w64(6, t789);
            let Some(t790) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080ab62); };
            c.w64(2, t790);
            let Some(t791) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080ab66); };
            c.w64(8, t791);
            let Some(t792) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080ab6a); };
            c.w64(9, t792);
            blk = 888; continue;
        }
        888 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 920; continue; }
            blk = 889; continue;
        }
        889 => {
            let Some(t793) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ab78); };
            c.w32(12, t793);
            let r794 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r794);
            let Some(t795) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080ab7f); };
            c.cmp(t795, c.r32(12), 32);
            if c.cc_e() { blk = 891; continue; }
            blk = 890; continue;
        }
        890 => {
            let Some(t796) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ab88); };
            c.cmp(t796, 0x43u64, 16);
            if c.cc_ne() { blk = 886; continue; }
            blk = 891; continue;
        }
        891 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ab91); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 919; continue; }
            blk = 892; continue;
        }
        892 => {
            let Some(t797) = c.ld64(0x700080955bu64) else { return RegfileExit::fault(0x700080aba5); };
            c.w64(10, t797);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 918; continue; }
            blk = 893; continue;
        }
        893 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 901; continue; }
            blk = 894; continue;
        }
        894 => {
            let Some(t798) = c.ld64(0x7000809582u64) else { return RegfileExit::fault(0x700080abc9); };
            c.w64(10, t798);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 900; continue; }
            blk = 895; continue;
        }
        895 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 897; continue; }
            blk = 896; continue;
        }
        896 => {
            c.mov128_rr(1, 0);
            blk = 906; continue;
        }
        897 => {
            c.mov128_rr(1, 0);
            let Some(t799) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x700080abf0); };
            c.w64(10, t799);
            if !c.xorpd_rm(1, c.r64(10)) { return RegfileExit::fault(0x700080abf7); }
            let Some(t800) = c.ld64(0x70008095b7u64) else { return RegfileExit::fault(0x700080abfb); };
            c.w64(10, t800);
            c.movq_xr(2, 10);
            c.mov128_rr(3, 1);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(3));
            if c.cc_be() { blk = 899; continue; }
            blk = 898; continue;
        }
        898 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(3));
            c.mov128_rr(3, 1);
            blk = 899; continue;
        }
        899 => {
            let Some(t801) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x700080ac2d); };
            c.w64(10, t801);
            if !c.xorpd_rm(3, c.r64(10)) { return RegfileExit::fault(0x700080ac34); }
            c.mov128_rr(1, 3);
            blk = 906; continue;
        }
        900 => {
            c.mov128_rr(1, 0);
            blk = 906; continue;
        }
        901 => {
            let Some(t802) = c.ld64(0x70008095b7u64) else { return RegfileExit::fault(0x700080ac42); };
            c.w64(10, t802);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 905; continue; }
            blk = 902; continue;
        }
        902 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 904; continue; }
            blk = 903; continue;
        }
        903 => {
            c.mov128_rr(1, 2);
            blk = 906; continue;
        }
        904 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.mov128_rr(1, 2);
            blk = 906; continue;
        }
        905 => {
            c.mov128_rr(1, 0);
            blk = 906; continue;
        }
        906 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x37);
            c.psrlq(2, 0x2);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 914; continue; }
            blk = 907; continue;
        }
        907 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 913; continue; }
            blk = 908; continue;
        }
        908 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x3f);
            c.psrlq(0, 0x1);
            c.mov128_rr(2, 1);
            c.w64(7, c.r64(0));
            let r803 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r803);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080acba); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700080acbf); }
            if !c.movsd_mr(c.r64(4), 2) { return RegfileExit::fault(0x700080acc2); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700080acc7); }
            blk = 909; continue;
        }
        909 => {
            c.fprem();
            c.fnstsw_ax();
            let r804 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r804);
            let r805 = c.r32(0) & 0xffu64;
            c.w32(0, r805);
            c.logic(r805, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080acd6); }
            if !c.popfq() { return RegfileExit::fault(0x700080acd7); }
            if c.cc_p() { blk = 909; continue; }
            blk = 910; continue;
        }
        910 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x700080acdc); }
            if !c.movsd_rm(2, c.r64(4)) { return RegfileExit::fault(0x700080acdf); }
            let r806 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r806);
            c.xorpd_rr(0, 0);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_p() { blk = 915; continue; }
            blk = 911; continue;
        }
        911 => {
            if c.cc_e() { blk = 916; continue; }
            blk = 912; continue;
        }
        912 => {
            blk = 915; continue;
        }
        913 => {
            c.w64(7, c.r64(0));
            blk = 916; continue;
        }
        914 => {
            c.w64(7, c.r64(0));
            blk = 915; continue;
        }
        915 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.mov128_rr(1, 0);
            blk = 916; continue;
        }
        916 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 1142; continue; }
            blk = 917; continue;
        }
        917 => {
            c.w32(11, c.r32(11));
            blk = 924; continue;
        }
        918 => {
            c.w64(7, c.r64(0));
            c.w32(11, 0xffu64);
            blk = 924; continue;
        }
        919 => {
            c.w64(7, c.r64(0));
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 924; continue;
        }
        920 => {
            let r807 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r807);
            c.w64(7, c.r64(0));
            c.cmp(c.r32(11), 0xffu64, 32);
            if c.cc_be() { blk = 924; continue; }
            blk = 921; continue;
        }
        921 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 923; continue; }
            blk = 922; continue;
        }
        922 => {
            c.w32(11, 0xffu64);
            blk = 924; continue;
        }
        923 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 924; continue;
        }
        924 => {
            c.w64(12, c.r64(2));
            let Some(t808) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080ad5d); };
            c.w32(15, t808);
            let r809 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r809);
            let Some(t810) = c.ld32(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080ad65); };
            c.w32(0, t810);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700080ad69); }
            c.logic(c.r8(0) & 0x10u64, 8);
            if c.cc_ne() { blk = 950; continue; }
            blk = 925; continue;
        }
        925 => {
            c.logic(c.r8(0) & 0x4u64, 8);
            if c.cc_ne() { blk = 1139; continue; }
            blk = 926; continue;
        }
        926 => {
            let Some(t811) = c.ld64(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080ad7d); };
            c.w64(15, t811);
            let Some(t812) = c.ld64(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080ad81); };
            c.w64(0, t812);
            let Some(t813) = c.ld8(c.r64(12).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080ad86); };
            c.logic(t813 & 0x1u64, 8);
            if c.cc_ne() { blk = 929; continue; }
            blk = 927; continue;
        }
        927 => {
            let Some(t814) = c.ld64(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080ad8e); };
            c.w64(3, t814);
            let r815 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r815);
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_a() { blk = 1139; continue; }
            blk = 928; continue;
        }
        928 => {
            let Some(t816) = c.ld64(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080ad9f); };
            c.w64(15, t816);
            c.w64(0, c.r64(15));
            blk = 952; continue;
        }
        929 => {
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_a() { blk = 1139; continue; }
            blk = 930; continue;
        }
        930 => {
            let Some(t817) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080adb5); };
            c.w32(3, t817);
            let r818 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r818);
            let Some(t819) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080adbd); };
            c.w32(3, t819);
            let r820 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r820);
            let r821 = c.r32(3) & 0x3fu64;
            c.w32(3, r821);
            c.logic(r821, 32);
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_l() { blk = 938; continue; }
            blk = 931; continue;
        }
        931 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_l() { blk = 935; continue; }
            blk = 932; continue;
        }
        932 => {
            c.cmp(c.r32(3), 0x24u64, 32);
            if c.cc_e() { blk = 945; continue; }
            blk = 933; continue;
        }
        933 => {
            c.cmp(c.r32(3), 0x25u64, 32);
            if c.cc_e() { blk = 948; continue; }
            blk = 934; continue;
        }
        934 => {
            c.cmp(c.r32(3), 0x26u64, 32);
            if c.cc_e() { blk = 948; continue; }
            blk = 935; continue;
        }
        935 => {
            c.cmp(c.r32(3), 0x21u64, 32);
            if c.cc_e() { blk = 947; continue; }
            blk = 936; continue;
        }
        936 => {
            c.cmp(c.r32(3), 0x22u64, 32);
            if c.cc_e() { blk = 947; continue; }
            blk = 937; continue;
        }
        937 => {
            c.cmp(c.r32(3), 0x23u64, 32);
            if c.cc_e() { blk = 948; continue; }
            blk = 938; continue;
        }
        938 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_l() { blk = 942; continue; }
            blk = 939; continue;
        }
        939 => {
            c.cmp(c.r32(3), 0x1eu64, 32);
            if c.cc_e() { blk = 946; continue; }
            blk = 940; continue;
        }
        940 => {
            c.cmp(c.r32(3), 0x1fu64, 32);
            if c.cc_e() { blk = 946; continue; }
            blk = 941; continue;
        }
        941 => {
            c.cmp(c.r32(3), 0x20u64, 32);
            if c.cc_e() { blk = 947; continue; }
            blk = 942; continue;
        }
        942 => {
            c.cmp(c.r32(3), 0x1cu64, 32);
            if c.cc_e() { blk = 945; continue; }
            blk = 943; continue;
        }
        943 => {
            c.cmp(c.r32(3), 0x1du64, 32);
            if c.cc_e() { blk = 945; continue; }
            blk = 944; continue;
        }
        944 => {
            return RegfileExit::fault(0x700080ae0d);
        }
        945 => {
            c.w32(3, 0x1u64);
            blk = 949; continue;
        }
        946 => {
            c.w32(3, 0x2u64);
            blk = 949; continue;
        }
        947 => {
            c.w32(3, 0x4u64);
            blk = 949; continue;
        }
        948 => {
            c.w32(3, 0x8u64);
            blk = 949; continue;
        }
        949 => {
            let r822 = c.sub(c.r64(15), c.r64(0), 64);
            c.w64(15, r822);
            c.w64(0, c.r64(15));
            c.cqo();
            c.idiv(c.r64(3), 64);
            blk = 952; continue;
        }
        950 => {
            c.w64(10, c.r64(4));
            let r823 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r823);
            let r824 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r824);
            c.logic(r824, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080ae40); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(12));
            let Some(t825) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080ae4b); };
            c.w64(0, t825);
            c.w64(10, 0x700080ae59u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080ae59); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080ae5d); }
            return c.call(c.r64(0), 0x700080ae63, 0x700080ae61);
        }
        951 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080ae63); }
            let Some(t826) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080ae6b); };
            c.w64(4, t826);
            let Some(t827) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080ae70); };
            c.w64(11, t827);
            let Some(t828) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080ae74); };
            c.w64(7, t828);
            let Some(t829) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080ae78); };
            c.w64(6, t829);
            let Some(t830) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080ae7c); };
            c.w64(12, t830);
            let Some(t831) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080ae80); };
            c.w64(8, t831);
            let Some(t832) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080ae84); };
            c.w64(9, t832);
            blk = 952; continue;
        }
        952 => {
            c.cmp(c.r64(9), c.r64(0), 64);
            if c.cc_ae() { blk = 1139; continue; }
            blk = 953; continue;
        }
        953 => {
            c.w64(2, c.r64(12));
            let Some(t833) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080ae94); };
            c.w64(8, t833);
            let Some(t834) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080ae98); };
            c.w32(12, t834);
            let r835 = c.add(c.r64(8), c.r64(12), 64);
            c.w64(8, r835);
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), c.r8(11)) { return RegfileExit::fault(0x700080ae9f); }
            c.w64(0, c.r64(7));
            blk = 1050; continue;
        }
        954 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 960; continue; }
            blk = 955; continue;
        }
        955 => {
            c.w64(8, c.r64(1));
            let Some(t836) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080aeb3); };
            c.w32(9, t836);
            let r837 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r837);
            let Some(t838) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080aeba); };
            c.cmp(t838, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 956; continue;
        }
        956 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080aec6); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 957; continue;
        }
        957 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 958; continue;
        }
        958 => {
            let Some(t839) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080aeea); };
            c.w64(11, t839);
            let r840 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r840);
            let Some(t841) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080aef4); };
            c.w64(12, t841);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 961; continue; }
            blk = 959; continue;
        }
        959 => {
            blk = 1044; continue;
        }
        960 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 961; continue;
        }
        961 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080af11); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1000; continue; }
            blk = 962; continue;
        }
        962 => {
            let Some(t842) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080af1d); };
            c.w32(11, t842);
            let r843 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r843);
            let Some(t844) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080af24); };
            c.cmp(t844, 0x41u64, 16);
            if c.cc_e() { blk = 965; continue; }
            blk = 963; continue;
        }
        963 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080af2c); }
            c.w32(0, 0x1u64);
            let Some(t845) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700080af32); };
            c.w64(3, t845);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700080af44, 0x700080af3f);
        }
        964 => {
            c.w64(11, c.r64(0));
            let Some(t846) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080af47); };
            c.w64(0, t846);
            let Some(t847) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080af4b); };
            c.w64(6, t847);
            let Some(t848) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080af4f); };
            c.w64(2, t848);
            let Some(t849) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080af53); };
            c.w64(7, t849);
            let Some(t850) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080af57); };
            c.w64(3, t850);
            let Some(t851) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080af5b); };
            c.w64(8, t851);
            let Some(t852) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080af5f); };
            c.w64(9, t852);
            blk = 966; continue;
        }
        965 => {
            c.w64(11, c.r64(0));
            blk = 966; continue;
        }
        966 => {
            c.w64(12, c.r64(2));
            let Some(t853) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080af6b); };
            c.w32(15, t853);
            let r854 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r854);
            let Some(t855) = c.ld32(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080af73); };
            c.w32(2, t855);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700080af77); }
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 992; continue; }
            blk = 967; continue;
        }
        967 => {
            c.logic(c.r8(2) & 0x4u64, 8);
            if c.cc_ne() { blk = 1140; continue; }
            blk = 968; continue;
        }
        968 => {
            let Some(t856) = c.ld64(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080af8d); };
            c.w64(15, t856);
            let Some(t857) = c.ld64(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080af91); };
            c.w64(2, t857);
            let Some(t858) = c.ld8(c.r64(12).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080af96); };
            c.logic(t858 & 0x1u64, 8);
            if c.cc_ne() { blk = 971; continue; }
            blk = 969; continue;
        }
        969 => {
            let Some(t859) = c.ld64(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080af9e); };
            c.w64(1, t859);
            let r860 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r860);
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_a() { blk = 1140; continue; }
            blk = 970; continue;
        }
        970 => {
            let Some(t861) = c.ld64(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080afaf); };
            c.w64(15, t861);
            blk = 994; continue;
        }
        971 => {
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_a() { blk = 1140; continue; }
            blk = 972; continue;
        }
        972 => {
            let Some(t862) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080afc2); };
            c.w32(1, t862);
            let r863 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r863);
            let Some(t864) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080afca); };
            c.w32(1, t864);
            let r865 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r865);
            let r866 = c.r32(1) & 0x3fu64;
            c.w32(1, r866);
            c.logic(r866, 32);
            c.cmp(c.r32(1), 0x21u64, 32);
            if c.cc_l() { blk = 980; continue; }
            blk = 973; continue;
        }
        973 => {
            c.cmp(c.r32(1), 0x24u64, 32);
            if c.cc_l() { blk = 977; continue; }
            blk = 974; continue;
        }
        974 => {
            c.cmp(c.r32(1), 0x24u64, 32);
            if c.cc_e() { blk = 987; continue; }
            blk = 975; continue;
        }
        975 => {
            c.cmp(c.r32(1), 0x25u64, 32);
            if c.cc_e() { blk = 990; continue; }
            blk = 976; continue;
        }
        976 => {
            c.cmp(c.r32(1), 0x26u64, 32);
            if c.cc_e() { blk = 990; continue; }
            blk = 977; continue;
        }
        977 => {
            c.cmp(c.r32(1), 0x21u64, 32);
            if c.cc_e() { blk = 989; continue; }
            blk = 978; continue;
        }
        978 => {
            c.cmp(c.r32(1), 0x22u64, 32);
            if c.cc_e() { blk = 989; continue; }
            blk = 979; continue;
        }
        979 => {
            c.cmp(c.r32(1), 0x23u64, 32);
            if c.cc_e() { blk = 990; continue; }
            blk = 980; continue;
        }
        980 => {
            c.cmp(c.r32(1), 0x1eu64, 32);
            if c.cc_l() { blk = 984; continue; }
            blk = 981; continue;
        }
        981 => {
            c.cmp(c.r32(1), 0x1eu64, 32);
            if c.cc_e() { blk = 988; continue; }
            blk = 982; continue;
        }
        982 => {
            c.cmp(c.r32(1), 0x1fu64, 32);
            if c.cc_e() { blk = 988; continue; }
            blk = 983; continue;
        }
        983 => {
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_e() { blk = 989; continue; }
            blk = 984; continue;
        }
        984 => {
            c.cmp(c.r32(1), 0x1cu64, 32);
            if c.cc_e() { blk = 987; continue; }
            blk = 985; continue;
        }
        985 => {
            c.cmp(c.r32(1), 0x1du64, 32);
            if c.cc_e() { blk = 987; continue; }
            blk = 986; continue;
        }
        986 => {
            return RegfileExit::fault(0x700080b01a);
        }
        987 => {
            c.w32(1, 0x1u64);
            blk = 991; continue;
        }
        988 => {
            c.w32(1, 0x2u64);
            blk = 991; continue;
        }
        989 => {
            c.w32(1, 0x4u64);
            blk = 991; continue;
        }
        990 => {
            c.w32(1, 0x8u64);
            blk = 991; continue;
        }
        991 => {
            let r867 = c.sub(c.r64(15), c.r64(2), 64);
            c.w64(15, r867);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(10));
            c.cqo();
            c.idiv(c.r64(1), 64);
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 994; continue;
        }
        992 => {
            c.w64(10, c.r64(4));
            let r868 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r868);
            let r869 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r869);
            c.logic(r869, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080b05c); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(12));
            let Some(t870) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080b067); };
            c.w64(0, t870);
            c.w64(10, 0x700080b075u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080b075); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080b079); }
            return c.call(c.r64(0), 0x700080b07f, 0x700080b07d);
        }
        993 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080b07f); }
            let Some(t871) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080b087); };
            c.w64(4, t871);
            c.w64(15, c.r64(0));
            let Some(t872) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080b08f); };
            c.w64(0, t872);
            let Some(t873) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b093); };
            c.w64(6, t873);
            let Some(t874) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b097); };
            c.w64(12, t874);
            let Some(t875) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b09b); };
            c.w64(7, t875);
            let Some(t876) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b09f); };
            c.w64(3, t876);
            let Some(t877) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b0a3); };
            c.w64(8, t877);
            let Some(t878) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080b0a7); };
            c.w64(11, t878);
            let Some(t879) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080b0ab); };
            c.w64(9, t879);
            blk = 994; continue;
        }
        994 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ae() { blk = 1140; continue; }
            blk = 995; continue;
        }
        995 => {
            c.w64(2, c.r64(12));
            let Some(t880) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080b0bb); };
            c.w64(7, t880);
            let Some(t881) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080b0bf); };
            c.w32(8, t881);
            let Some(t882) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080b0c3); };
            c.w32(12, t882);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 998; continue; }
            blk = 996; continue;
        }
        996 => {
            let Some(t883) = c.ld64(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080b0d0); };
            c.w64(11, t883);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 999; continue; }
            blk = 997; continue;
        }
        997 => {
            let r884 = c.neg(c.r64(11), 64);
            c.w64(11, r884);
            blk = 999; continue;
        }
        998 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 999; continue;
        }
        999 => {
            let r885 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r885);
            if !c.st64(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(11)) { return RegfileExit::fault(0x700080b0e5); }
            blk = 1050; continue;
        }
        1000 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080b0f3); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080b0f4); }
            let Some(t886) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700080b0f5); };
            c.w64(3, t886);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700080b106, 0x700080b101);
        }
        1001 => {
            return RegfileExit::fault(0x700080b106);
        }
        1002 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 1008; continue; }
            blk = 1003; continue;
        }
        1003 => {
            c.w64(8, c.r64(1));
            let Some(t887) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080b10f); };
            c.w32(9, t887);
            let r888 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r888);
            let Some(t889) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080b116); };
            c.cmp(t889, 0x42u64, 16);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 1004; continue;
        }
        1004 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080b122); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 1044; continue; }
            blk = 1005; continue;
        }
        1005 => {
            if c.cc_ne() { blk = 1044; continue; }
            blk = 1006; continue;
        }
        1006 => {
            let Some(t890) = c.ld64(0x7000808435u64) else { return RegfileExit::fault(0x700080b146); };
            c.w64(11, t890);
            let r891 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r891);
            let Some(t892) = c.ld64(0x7000808442u64) else { return RegfileExit::fault(0x700080b150); };
            c.w64(12, t892);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 1009; continue; }
            blk = 1007; continue;
        }
        1007 => {
            blk = 1044; continue;
        }
        1008 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 1009; continue;
        }
        1009 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080b16d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1051; continue; }
            blk = 1010; continue;
        }
        1010 => {
            let Some(t893) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080b179); };
            c.w32(11, t893);
            let r894 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r894);
            let Some(t895) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080b180); };
            c.cmp(t895, 0x41u64, 16);
            if c.cc_e() { blk = 1013; continue; }
            blk = 1011; continue;
        }
        1011 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080b188); }
            c.w32(0, 0x1u64);
            let Some(t896) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700080b18e); };
            c.w64(3, t896);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700080b1a0, 0x700080b19b);
        }
        1012 => {
            c.w64(11, c.r64(0));
            let Some(t897) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080b1a3); };
            c.w64(0, t897);
            let Some(t898) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b1a7); };
            c.w64(6, t898);
            let Some(t899) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080b1ab); };
            c.w64(9, t899);
            let Some(t900) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b1af); };
            c.w64(2, t900);
            let Some(t901) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b1b3); };
            c.w64(7, t901);
            let Some(t902) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b1b7); };
            c.w64(3, t902);
            let Some(t903) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b1bb); };
            c.w64(8, t903);
            blk = 1014; continue;
        }
        1013 => {
            c.w64(11, c.r64(0));
            blk = 1014; continue;
        }
        1014 => {
            c.w64(12, c.r64(2));
            let Some(t904) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080b1c7); };
            c.w32(15, t904);
            let r905 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r905);
            let Some(t906) = c.ld32(c.r64(15).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080b1cf); };
            c.w32(2, t906);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700080b1d3); }
            c.logic(c.r8(2) & 0x10u64, 8);
            if c.cc_ne() { blk = 1040; continue; }
            blk = 1015; continue;
        }
        1015 => {
            c.logic(c.r8(2) & 0x4u64, 8);
            if c.cc_ne() { blk = 1141; continue; }
            blk = 1016; continue;
        }
        1016 => {
            let Some(t907) = c.ld64(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080b1e9); };
            c.w64(15, t907);
            let Some(t908) = c.ld64(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080b1ed); };
            c.w64(2, t908);
            let Some(t909) = c.ld8(c.r64(12).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080b1f2); };
            c.logic(t909 & 0x1u64, 8);
            if c.cc_ne() { blk = 1019; continue; }
            blk = 1017; continue;
        }
        1017 => {
            let Some(t910) = c.ld64(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080b1fa); };
            c.w64(1, t910);
            let r911 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r911);
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_a() { blk = 1141; continue; }
            blk = 1018; continue;
        }
        1018 => {
            let Some(t912) = c.ld64(c.r64(12).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080b20b); };
            c.w64(15, t912);
            blk = 1042; continue;
        }
        1019 => {
            c.cmp(c.r64(2), c.r64(15), 64);
            if c.cc_a() { blk = 1141; continue; }
            blk = 1020; continue;
        }
        1020 => {
            let Some(t913) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080b21e); };
            c.w32(1, t913);
            let r914 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r914);
            let Some(t915) = c.ld8(c.r64(1).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080b226); };
            c.w32(1, t915);
            let r916 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r916);
            let r917 = c.r32(1) & 0x3fu64;
            c.w32(1, r917);
            c.logic(r917, 32);
            c.cmp(c.r32(1), 0x21u64, 32);
            if c.cc_l() { blk = 1028; continue; }
            blk = 1021; continue;
        }
        1021 => {
            c.cmp(c.r32(1), 0x24u64, 32);
            if c.cc_l() { blk = 1025; continue; }
            blk = 1022; continue;
        }
        1022 => {
            c.cmp(c.r32(1), 0x24u64, 32);
            if c.cc_e() { blk = 1035; continue; }
            blk = 1023; continue;
        }
        1023 => {
            c.cmp(c.r32(1), 0x25u64, 32);
            if c.cc_e() { blk = 1038; continue; }
            blk = 1024; continue;
        }
        1024 => {
            c.cmp(c.r32(1), 0x26u64, 32);
            if c.cc_e() { blk = 1038; continue; }
            blk = 1025; continue;
        }
        1025 => {
            c.cmp(c.r32(1), 0x21u64, 32);
            if c.cc_e() { blk = 1037; continue; }
            blk = 1026; continue;
        }
        1026 => {
            c.cmp(c.r32(1), 0x22u64, 32);
            if c.cc_e() { blk = 1037; continue; }
            blk = 1027; continue;
        }
        1027 => {
            c.cmp(c.r32(1), 0x23u64, 32);
            if c.cc_e() { blk = 1038; continue; }
            blk = 1028; continue;
        }
        1028 => {
            c.cmp(c.r32(1), 0x1eu64, 32);
            if c.cc_l() { blk = 1032; continue; }
            blk = 1029; continue;
        }
        1029 => {
            c.cmp(c.r32(1), 0x1eu64, 32);
            if c.cc_e() { blk = 1036; continue; }
            blk = 1030; continue;
        }
        1030 => {
            c.cmp(c.r32(1), 0x1fu64, 32);
            if c.cc_e() { blk = 1036; continue; }
            blk = 1031; continue;
        }
        1031 => {
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_e() { blk = 1037; continue; }
            blk = 1032; continue;
        }
        1032 => {
            c.cmp(c.r32(1), 0x1cu64, 32);
            if c.cc_e() { blk = 1035; continue; }
            blk = 1033; continue;
        }
        1033 => {
            c.cmp(c.r32(1), 0x1du64, 32);
            if c.cc_e() { blk = 1035; continue; }
            blk = 1034; continue;
        }
        1034 => {
            return RegfileExit::fault(0x700080b276);
        }
        1035 => {
            c.w32(1, 0x1u64);
            blk = 1039; continue;
        }
        1036 => {
            c.w32(1, 0x2u64);
            blk = 1039; continue;
        }
        1037 => {
            c.w32(1, 0x4u64);
            blk = 1039; continue;
        }
        1038 => {
            c.w32(1, 0x8u64);
            blk = 1039; continue;
        }
        1039 => {
            let r918 = c.sub(c.r64(15), c.r64(2), 64);
            c.w64(15, r918);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(15));
            c.w64(15, c.r64(10));
            c.cqo();
            c.idiv(c.r64(1), 64);
            c.w64(10, c.r64(15));
            c.w64(15, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 1042; continue;
        }
        1040 => {
            c.w64(10, c.r64(4));
            let r919 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r919);
            let r920 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r920);
            c.logic(r920, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080b2b8); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(12));
            let Some(t921) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080b2c3); };
            c.w64(0, t921);
            c.w64(10, 0x700080b2d1u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080b2d1); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080b2d5); }
            return c.call(c.r64(0), 0x700080b2db, 0x700080b2d9);
        }
        1041 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080b2db); }
            let Some(t922) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080b2e3); };
            c.w64(4, t922);
            c.w64(15, c.r64(0));
            let Some(t923) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080b2eb); };
            c.w64(0, t923);
            let Some(t924) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b2ef); };
            c.w64(6, t924);
            let Some(t925) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080b2f3); };
            c.w64(9, t925);
            let Some(t926) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080b2f7); };
            c.w64(11, t926);
            let Some(t927) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b2fb); };
            c.w64(12, t927);
            let Some(t928) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b2ff); };
            c.w64(7, t928);
            let Some(t929) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b303); };
            c.w64(3, t929);
            let Some(t930) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b307); };
            c.w64(8, t930);
            blk = 1042; continue;
        }
        1042 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 1045; continue; }
            blk = 1043; continue;
        }
        1043 => {
            blk = 1141; continue;
        }
        1044 => {
            let Some(t931) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700080b315); };
            c.w64(5, t931);
            let r932 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r932);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080b31d); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700080b31e); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080b31f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080b320); }
            let Some(t933) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080b322); };
            if !c.push64(t933) { return RegfileExit::fault(0x700080b322); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700080b326); }
            c.w32(0, 0x5u64);
            let Some(t934) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x700080b330); };
            c.w64(3, t934);
            return RegfileExit::jump(0x700086ed74);
        }
        1045 => {
            c.w64(7, c.r64(12));
            let Some(t935) = c.ld64(c.r64(7).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080b33f); };
            c.w64(8, t935);
            let Some(t936) = c.ld32(c.r64(7).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080b343); };
            c.w32(7, t936);
            let Some(t937) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080b346); };
            c.w32(12, t937);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 1048; continue; }
            blk = 1046; continue;
        }
        1046 => {
            let Some(t938) = c.ld64(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080b353); };
            c.w64(11, t938);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 1049; continue; }
            blk = 1047; continue;
        }
        1047 => {
            let r939 = c.neg(c.r64(11), 64);
            c.w64(11, r939);
            blk = 1049; continue;
        }
        1048 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 1049; continue;
        }
        1049 => {
            let r940 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r940);
            if !c.st64(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(11)) { return RegfileExit::fault(0x700080b368); }
            blk = 1050; continue;
        }
        1050 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080b36f); }
            return c.ret(0x0, 0x700080b370);
        }
        1051 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080b376); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080b377); }
            let Some(t941) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700080b378); };
            c.w64(3, t941);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700080b389, 0x700080b384);
        }
        1052 => {
            return RegfileExit::fault(0x700080b389);
        }
        1053 => {
            return RegfileExit::fault(0x700080b38a);
        }
        1054 => {
            return RegfileExit::fault(0x700080b38b);
        }
        1055 => {
            return RegfileExit::fault(0x700080b38c);
        }
        1056 => {
            return RegfileExit::fault(0x700080b38d);
        }
        1057 => {
            return RegfileExit::fault(0x700080b38e);
        }
        1058 => {
            return RegfileExit::fault(0x700080b38f);
        }
        1059 => {
            return RegfileExit::fault(0x700080b390);
        }
        1060 => {
            return RegfileExit::fault(0x700080b391);
        }
        1061 => {
            return RegfileExit::fault(0x700080b392);
        }
        1062 => {
            return RegfileExit::fault(0x700080b393);
        }
        1063 => {
            return RegfileExit::fault(0x700080b394);
        }
        1064 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(8, c.r64(1));
            let Some(t942) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b39b); };
            c.w64(7, t942);
            let Some(t943) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b39f); };
            c.w64(3, t943);
            blk = 1044; continue;
        }
        1065 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t944) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b3ab); };
            c.w64(3, t944);
            let Some(t945) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b3af); };
            c.w64(7, t945);
            blk = 1044; continue;
        }
        1066 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t946) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b3bb); };
            c.w64(3, t946);
            let Some(t947) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b3bf); };
            c.w64(7, t947);
            blk = 1044; continue;
        }
        1067 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t948) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b3cb); };
            c.w64(3, t948);
            let Some(t949) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b3cf); };
            c.w64(7, t949);
            blk = 1044; continue;
        }
        1068 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700080b3d8); }
            c.cmp(c.r32(7), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 1070; continue; }
            blk = 1069; continue;
        }
        1069 => {
            c.w32(7, c.r32(7));
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            blk = 1075; continue;
        }
        1070 => {
            let Some(t950) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b3eb); };
            c.w64(9, t950);
            let Some(t951) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700080b3f2); };
            c.w64(11, t951);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t952) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b3f9); };
            c.w64(15, t952);
            let Some(t953) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080b400); };
            c.cmp(t953, c.r64(12), 64);
            if c.cc_a() { blk = 1073; continue; }
            blk = 1071; continue;
        }
        1071 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700080b412, 0x700080b40d);
        }
        1072 => {
            let Some(t954) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080b412); };
            c.w64(7, t954);
            let Some(t955) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b416); };
            c.w64(6, t955);
            let Some(t956) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b41a); };
            c.w64(2, t956);
            let Some(t957) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b41e); };
            c.w64(8, t957);
            blk = 1074; continue;
        }
        1073 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700080b424); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1074; continue;
        }
        1074 => {
            c.w32(10, c.r32(7));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t958) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b43a); };
            c.w64(7, t958);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700080b441); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b444); }
            blk = 1075; continue;
        }
        1075 => {
            let Some(t959) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b449); };
            c.w64(3, t959);
            let Some(t960) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b44d); };
            c.w64(7, t960);
            blk = 1044; continue;
        }
        1076 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700080b456); }
            c.w64(9, c.r64(7));
            let r961 = c.add(c.r32(9), c.r32(7), 32);
            c.w32(9, r961);
            if c.cc_o() { blk = 1078; continue; }
            blk = 1077; continue;
        }
        1077 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 1083; continue;
        }
        1078 => {
            let Some(t962) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b467); };
            c.w64(9, t962);
            let Some(t963) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700080b46e); };
            c.w64(11, t963);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t964) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b475); };
            c.w64(15, t964);
            let Some(t965) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080b47c); };
            c.cmp(t965, c.r64(12), 64);
            if c.cc_a() { blk = 1081; continue; }
            blk = 1079; continue;
        }
        1079 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700080b48e, 0x700080b489);
        }
        1080 => {
            let Some(t966) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080b48e); };
            c.w64(7, t966);
            let Some(t967) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b492); };
            c.w64(6, t967);
            let Some(t968) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b496); };
            c.w64(2, t968);
            let Some(t969) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b49a); };
            c.w64(8, t969);
            blk = 1082; continue;
        }
        1081 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700080b4a0); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1082; continue;
        }
        1082 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            let Some(t970) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b4b2); };
            c.w64(7, t970);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700080b4b9); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b4bc); }
            blk = 1083; continue;
        }
        1083 => {
            let Some(t971) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b4c1); };
            c.w64(3, t971);
            let Some(t972) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b4c5); };
            c.w64(7, t972);
            blk = 1044; continue;
        }
        1084 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700080b4ce); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1091; continue; }
            blk = 1085; continue;
        }
        1085 => {
            let Some(t973) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080b4d7); };
            c.w32(7, t973);
            let Some(t974) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b4da); };
            c.cmp(t974, c.r32(7), 32);
            if c.cc_e() { blk = 1091; continue; }
            blk = 1086; continue;
        }
        1086 => {
            let Some(t975) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b4e3); };
            c.w64(7, t975);
            let Some(t976) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080b4ea); };
            c.w64(9, t976);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t977) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b4f1); };
            c.w64(12, t977);
            let Some(t978) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700080b4f8); };
            c.cmp(t978, c.r64(11), 64);
            if c.cc_a() { blk = 1089; continue; }
            blk = 1087; continue;
        }
        1087 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700080b50b, 0x700080b506);
        }
        1088 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700080b50b); }
            let Some(t979) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b510); };
            c.w64(6, t979);
            let Some(t980) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b514); };
            c.w64(2, t980);
            let Some(t981) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b518); };
            c.w64(8, t981);
            blk = 1090; continue;
        }
        1089 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700080b51e); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 1090; continue;
        }
        1090 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t982) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b52c); };
            c.w64(7, t982);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700080b533); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b536); }
            blk = 1091; continue;
        }
        1091 => {
            let Some(t983) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b53b); };
            c.w64(3, t983);
            let Some(t984) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b53f); };
            c.w64(7, t984);
            blk = 1044; continue;
        }
        1092 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700080b548); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1099; continue; }
            blk = 1093; continue;
        }
        1093 => {
            let Some(t985) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080b551); };
            c.w32(7, t985);
            let Some(t986) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b554); };
            c.cmp(t986, c.r32(7), 32);
            if c.cc_e() { blk = 1099; continue; }
            blk = 1094; continue;
        }
        1094 => {
            let Some(t987) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b55d); };
            c.w64(7, t987);
            let Some(t988) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080b564); };
            c.w64(9, t988);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t989) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b56b); };
            c.w64(12, t989);
            let Some(t990) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700080b572); };
            c.cmp(t990, c.r64(11), 64);
            if c.cc_a() { blk = 1097; continue; }
            blk = 1095; continue;
        }
        1095 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700080b585, 0x700080b580);
        }
        1096 => {
            let Some(t991) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b585); };
            c.w64(8, t991);
            let Some(t992) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b589); };
            c.w64(6, t992);
            let Some(t993) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b58d); };
            c.w64(2, t993);
            blk = 1098; continue;
        }
        1097 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700080b593); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 1098; continue;
        }
        1098 => {
            let Some(t994) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b59d); };
            c.w64(7, t994);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700080b5a4); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700080b5a7); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b5ac); }
            blk = 1099; continue;
        }
        1099 => {
            let Some(t995) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b5b1); };
            c.w64(3, t995);
            let Some(t996) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b5b5); };
            c.w64(7, t996);
            blk = 1044; continue;
        }
        1100 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t997) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b5c2); };
            c.w64(3, t997);
            let Some(t998) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b5c6); };
            c.w64(7, t998);
            blk = 1044; continue;
        }
        1101 => {
            c.w64(0, c.r64(11));
            blk = 1044; continue;
        }
        1102 => {
            c.w64(0, c.r64(11));
            blk = 1044; continue;
        }
        1103 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t999) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b5e2); };
            c.w64(3, t999);
            c.w64(2, c.r64(11));
            let Some(t1000) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b5e9); };
            c.w64(7, t1000);
            blk = 1044; continue;
        }
        1104 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t1001) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b5f5); };
            c.w64(3, t1001);
            c.w64(2, c.r64(11));
            let Some(t1002) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b5fc); };
            c.w64(7, t1002);
            blk = 1044; continue;
        }
        1105 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t1003) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b608); };
            c.w64(3, t1003);
            c.w64(2, c.r64(11));
            let Some(t1004) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b60f); };
            c.w64(7, t1004);
            blk = 1044; continue;
        }
        1106 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t1005) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b61b); };
            c.w64(3, t1005);
            c.w64(2, c.r64(11));
            let Some(t1006) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b622); };
            c.w64(7, t1006);
            blk = 1044; continue;
        }
        1107 => {
            c.cmp(c.r32(7), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 1109; continue; }
            blk = 1108; continue;
        }
        1108 => {
            c.w32(7, c.r32(7));
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            blk = 1114; continue;
        }
        1109 => {
            let Some(t1007) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b63a); };
            c.w64(9, t1007);
            let Some(t1008) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700080b641); };
            c.w64(12, t1008);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t1009) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b649); };
            c.w64(0, t1009);
            let Some(t1010) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700080b650); };
            c.cmp(t1010, c.r64(15), 64);
            if c.cc_a() { blk = 1112; continue; }
            blk = 1110; continue;
        }
        1110 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700080b65f, 0x700080b65a);
        }
        1111 => {
            let Some(t1011) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080b65f); };
            c.w64(7, t1011);
            let Some(t1012) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b663); };
            c.w64(6, t1012);
            let Some(t1013) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b667); };
            c.w64(11, t1013);
            let Some(t1014) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b66b); };
            c.w64(8, t1014);
            blk = 1113; continue;
        }
        1112 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x700080b671); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1113; continue;
        }
        1113 => {
            c.w32(10, c.r32(7));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t1015) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b688); };
            c.w64(7, t1015);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700080b68f); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b692); }
            blk = 1114; continue;
        }
        1114 => {
            let Some(t1016) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b697); };
            c.w64(3, t1016);
            c.w64(2, c.r64(11));
            let Some(t1017) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b69e); };
            c.w64(7, t1017);
            blk = 1044; continue;
        }
        1115 => {
            c.w64(9, c.r64(7));
            let r1018 = c.add(c.r32(7), c.r32(9), 32);
            c.w32(7, r1018);
            if c.cc_o() { blk = 1117; continue; }
            blk = 1116; continue;
        }
        1116 => {
            c.w64(0, (c.r32(7) as u32 as i32 as i64 as u64));
            blk = 1122; continue;
        }
        1117 => {
            let Some(t1019) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b6b4); };
            c.w64(7, t1019);
            let Some(t1020) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080b6bb); };
            c.w64(12, t1020);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t1021) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b6c3); };
            c.w64(0, t1021);
            let Some(t1022) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700080b6ca); };
            c.cmp(t1022, c.r64(15), 64);
            if c.cc_a() { blk = 1120; continue; }
            blk = 1118; continue;
        }
        1118 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700080b6d9, 0x700080b6d4);
        }
        1119 => {
            let Some(t1023) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080b6d9); };
            c.w64(9, t1023);
            let Some(t1024) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b6dd); };
            c.w64(6, t1024);
            let Some(t1025) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b6e1); };
            c.w64(11, t1025);
            let Some(t1026) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b6e5); };
            c.w64(8, t1026);
            blk = 1121; continue;
        }
        1120 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x700080b6eb); }
            c.w64(7, c.r64(12).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 1121; continue;
        }
        1121 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            let Some(t1027) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b6ff); };
            c.w64(7, t1027);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700080b706); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b709); }
            blk = 1122; continue;
        }
        1122 => {
            let Some(t1028) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b70e); };
            c.w64(3, t1028);
            c.w64(2, c.r64(11));
            let Some(t1029) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b715); };
            c.w64(7, t1029);
            blk = 1044; continue;
        }
        1123 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1130; continue; }
            blk = 1124; continue;
        }
        1124 => {
            let Some(t1030) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080b722); };
            c.w32(9, t1030);
            let Some(t1031) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b726); };
            c.cmp(t1031, c.r32(9), 32);
            if c.cc_e() { blk = 1130; continue; }
            blk = 1125; continue;
        }
        1125 => {
            let Some(t1032) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b72f); };
            c.w64(9, t1032);
            let Some(t1033) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700080b736); };
            c.w64(11, t1033);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t1034) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b73d); };
            c.w64(15, t1034);
            let Some(t1035) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080b744); };
            c.cmp(t1035, c.r64(12), 64);
            if c.cc_a() { blk = 1128; continue; }
            blk = 1126; continue;
        }
        1126 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700080b753, 0x700080b74e);
        }
        1127 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700080b753); }
            let Some(t1036) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b758); };
            c.w64(6, t1036);
            let Some(t1037) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b75c); };
            c.w64(7, t1037);
            let Some(t1038) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b760); };
            c.w64(8, t1038);
            blk = 1129; continue;
        }
        1128 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700080b766); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1129; continue;
        }
        1129 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t1039) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b774); };
            c.w64(9, t1039);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080b77b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b77f); }
            blk = 1130; continue;
        }
        1130 => {
            let Some(t1040) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b784); };
            c.w64(3, t1040);
            c.w64(2, c.r64(7));
            let Some(t1041) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b78b); };
            c.w64(7, t1041);
            blk = 1044; continue;
        }
        1131 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1138; continue; }
            blk = 1132; continue;
        }
        1132 => {
            let Some(t1042) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080b798); };
            c.w32(9, t1042);
            let Some(t1043) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b79c); };
            c.cmp(t1043, c.r32(9), 32);
            if c.cc_e() { blk = 1138; continue; }
            blk = 1133; continue;
        }
        1133 => {
            let Some(t1044) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080b7a5); };
            c.w64(9, t1044);
            let Some(t1045) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700080b7ac); };
            c.w64(11, t1045);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t1046) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080b7b3); };
            c.w64(15, t1046);
            let Some(t1047) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080b7ba); };
            c.cmp(t1047, c.r64(12), 64);
            if c.cc_a() { blk = 1136; continue; }
            blk = 1134; continue;
        }
        1134 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700080b7c9, 0x700080b7c4);
        }
        1135 => {
            let Some(t1048) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080b7c9); };
            c.w64(8, t1048);
            let Some(t1049) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080b7cd); };
            c.w64(6, t1049);
            let Some(t1050) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080b7d1); };
            c.w64(7, t1050);
            blk = 1137; continue;
        }
        1136 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700080b7d7); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1137; continue;
        }
        1137 => {
            let Some(t1051) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080b7e1); };
            c.w64(9, t1051);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080b7e8); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x28)) { return RegfileExit::fault(0x700080b7ec); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080b7f1); }
            blk = 1138; continue;
        }
        1138 => {
            let Some(t1052) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b7f6); };
            c.w64(3, t1052);
            c.w64(2, c.r64(7));
            let Some(t1053) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b7fd); };
            c.w64(7, t1053);
            blk = 1044; continue;
        }
        1139 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t1054) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080b80a); };
            c.w64(3, t1054);
            c.w64(2, c.r64(12));
            let Some(t1055) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080b811); };
            c.w64(7, t1055);
            blk = 1044; continue;
        }
        1140 => {
            c.w64(0, c.r64(11));
            c.w64(2, c.r64(12));
            blk = 1044; continue;
        }
        1141 => {
            c.w64(0, c.r64(11));
            c.w64(2, c.r64(12));
            blk = 1044; continue;
        }
        1142 => {
            let r1056 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1056);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x700080b834); }
            return c.call(0x70008708b4, 0x700080b83e, 0x700080b839);
        }
        1143 => {
            let Some(t1057) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b83e); };
            c.w32(11, t1057);
            let r1058 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1058);
            blk = 917; continue;
        }
        1144 => {
            let r1059 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1059);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b84f); }
            return c.call(0x70008708b4, 0x700080b859, 0x700080b854);
        }
        1145 => {
            let Some(t1060) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b859); };
            c.w32(7, t1060);
            let r1061 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1061);
            blk = 756; continue;
        }
        1146 => {
            let r1062 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1062);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b869); }
            return c.call(0x70008708b4, 0x700080b873, 0x700080b86e);
        }
        1147 => {
            let Some(t1063) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b873); };
            c.w32(7, t1063);
            let r1064 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1064);
            blk = 710; continue;
        }
        1148 => {
            let r1065 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1065);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b883); }
            return c.call(0x70008708b4, 0x700080b88d, 0x700080b888);
        }
        1149 => {
            let Some(t1066) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b88d); };
            c.w32(7, t1066);
            let r1067 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1067);
            blk = 664; continue;
        }
        1150 => {
            let r1068 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1068);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b89d); }
            return c.call(0x70008708b4, 0x700080b8a7, 0x700080b8a2);
        }
        1151 => {
            let Some(t1069) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b8a7); };
            c.w32(7, t1069);
            let r1070 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1070);
            blk = 618; continue;
        }
        1152 => {
            let r1071 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1071);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b8b7); }
            return c.call(0x70008708b4, 0x700080b8c1, 0x700080b8bc);
        }
        1153 => {
            let Some(t1072) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b8c1); };
            c.w32(7, t1072);
            let r1073 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1073);
            blk = 572; continue;
        }
        1154 => {
            let r1074 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1074);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b8d1); }
            return c.call(0x70008708b4, 0x700080b8db, 0x700080b8d6);
        }
        1155 => {
            let Some(t1075) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b8db); };
            c.w32(7, t1075);
            let r1076 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1076);
            blk = 526; continue;
        }
        1156 => {
            let r1077 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1077);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x700080b8eb); }
            return c.call(0x70008708b4, 0x700080b8f5, 0x700080b8f0);
        }
        1157 => {
            let Some(t1078) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b8f5); };
            c.w32(11, t1078);
            let r1079 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1079);
            blk = 460; continue;
        }
        1158 => {
            let r1080 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1080);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b906); }
            return c.call(0x70008708b4, 0x700080b910, 0x700080b90b);
        }
        1159 => {
            let Some(t1081) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b910); };
            c.w32(7, t1081);
            let r1082 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1082);
            blk = 380; continue;
        }
        1160 => {
            let r1083 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1083);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b920); }
            return c.call(0x70008708b4, 0x700080b92a, 0x700080b925);
        }
        1161 => {
            let Some(t1084) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b92a); };
            c.w32(7, t1084);
            let r1085 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1085);
            blk = 361; continue;
        }
        1162 => {
            let r1086 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1086);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b93a); }
            return c.call(0x70008708b4, 0x700080b944, 0x700080b93f);
        }
        1163 => {
            let Some(t1087) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b944); };
            c.w32(7, t1087);
            let r1088 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1088);
            blk = 342; continue;
        }
        1164 => {
            let r1089 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1089);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b954); }
            return c.call(0x70008708b4, 0x700080b95e, 0x700080b959);
        }
        1165 => {
            let Some(t1090) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b95e); };
            c.w32(7, t1090);
            let r1091 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1091);
            blk = 323; continue;
        }
        1166 => {
            let r1092 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1092);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b96e); }
            return c.call(0x70008708b4, 0x700080b978, 0x700080b973);
        }
        1167 => {
            let Some(t1093) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b978); };
            c.w32(7, t1093);
            let r1094 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1094);
            blk = 304; continue;
        }
        1168 => {
            let r1095 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1095);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080b988); }
            return c.call(0x70008708b4, 0x700080b992, 0x700080b98d);
        }
        1169 => {
            let Some(t1096) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700080b992); };
            c.w32(7, t1096);
            let r1097 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r1097);
            blk = 285; continue;
        }
        1170 => {
            c.w32(0, c.r32(0));
            let r1098 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1098);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1099 = c.r64(7) & c.r64(0);
            c.w64(7, r1099);
            c.logic(r1099, 64);
            let Some(t1100) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080b9ad); };
            c.logic(t1100 & 0x2u64, 8);
            if c.cc_e() { blk = 224; continue; }
            blk = 1171; continue;
        }
        1171 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080b9bb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080b9bc); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700080b9c9, 0x700080b9c4);
        }
        1172 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080b9c9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080b9cb); }
            blk = 224; continue;
        }
        1173 => {
            c.w32(0, c.r32(0));
            let r1101 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1101);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1102 = c.r64(7) & c.r64(0);
            c.w64(7, r1102);
            c.logic(r1102, 64);
            let Some(t1103) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080b9e0); };
            c.logic(t1103 & 0x2u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 1174; continue;
        }
        1174 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080b9ee); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080b9ef); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700080b9fc, 0x700080b9f7);
        }
        1175 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080b9fc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080b9fe); }
            blk = 202; continue;
        }
        1176 => {
            c.w32(0, c.r32(0));
            let r1104 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1104);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1105 = c.r64(8) & c.r64(0);
            c.w64(8, r1105);
            c.logic(r1105, 64);
            let Some(t1106) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080ba13); };
            c.logic(t1106 & 0x2u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 1177; continue;
        }
        1177 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080ba22); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080ba23); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700080ba30, 0x700080ba2b);
        }
        1178 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080ba30); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080ba32); }
            blk = 181; continue;
        }
        1179 => {
            c.w32(0, c.r32(0));
            let r1107 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1107);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1108 = c.r64(8) & c.r64(0);
            c.w64(8, r1108);
            c.logic(r1108, 64);
            let Some(t1109) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080ba47); };
            c.logic(t1109 & 0x2u64, 8);
            if c.cc_e() { blk = 161; continue; }
            blk = 1180; continue;
        }
        1180 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080ba56); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080ba57); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700080ba64, 0x700080ba5f);
        }
        1181 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080ba64); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080ba66); }
            blk = 161; continue;
        }
        1182 => {
            c.w32(0, c.r32(0));
            let r1110 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1110);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1111 = c.r64(8) & c.r64(0);
            c.w64(8, r1111);
            c.logic(r1111, 64);
            let Some(t1112) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080ba7b); };
            c.logic(t1112 & 0x2u64, 8);
            if c.cc_e() { blk = 141; continue; }
            blk = 1183; continue;
        }
        1183 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080ba8a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080ba8b); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700080ba98, 0x700080ba93);
        }
        1184 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080ba98); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080ba9a); }
            blk = 141; continue;
        }
        1185 => {
            c.w32(0, c.r32(0));
            let r1113 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1113);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1114 = c.r64(8) & c.r64(0);
            c.w64(8, r1114);
            c.logic(r1114, 64);
            let Some(t1115) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080baaf); };
            c.logic(t1115 & 0x2u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 1186; continue;
        }
        1186 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080babe); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080babf); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700080bacc, 0x700080bac7);
        }
        1187 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080bacc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080bace); }
            blk = 121; continue;
        }
        1188 => {
            return RegfileExit::jump(0x700080bad8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700080baf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700080baf4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700080baf8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080bafe); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080bb05); };
            c.w32(9, t3);
            let r4 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r4);
            let r5 = c.r32(9) & 0x3fu64;
            c.w32(9, r5);
            c.logic(r5, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700080bb12); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x700080bb16); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x700080bb1a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700080bb1e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(6)) { return RegfileExit::fault(0x700080bb22); }
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 32; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_l() { blk = 17; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 472; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 473; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 474; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 470; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 471; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 474; continue;
        }
        10 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 468; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 469; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 474; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 466; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 467; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 474; continue;
        }
        17 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 25; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_l() { blk = 22; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 464; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 465; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 474; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 462; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 463; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 474; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 460; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 461; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 474; continue;
        }
        29 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 458; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 459; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 474; continue;
        }
        32 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_l() { blk = 48; continue; }
            blk = 33; continue;
        }
        33 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_l() { blk = 41; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 456; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 457; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 474; continue;
        }
        38 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 454; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 455; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 474; continue;
        }
        41 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 452; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 453; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 474; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 340; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 474; continue;
        }
        48 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 56; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 53; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 204; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 474; continue;
        }
        53 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 372; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 411; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 474; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 474; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 474; continue;
        }
        63 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080bd18); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080bd24); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080bd2b); };
            c.cmp(t10, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080bd37); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 66; continue;
        }
        66 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r11 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r11);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 70; continue; }
            blk = 68; continue;
        }
        68 => {
            blk = 446; continue;
        }
        69 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 70; continue;
        }
        70 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 446; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t12) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080bd89); };
            c.cmp(t12, 0x834u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t13) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080bd92); };
            c.w32(8, t13);
            blk = 74; continue;
        }
        73 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080bd98); };
            c.w32(8, t14);
            blk = 74; continue;
        }
        74 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 75; continue;
        }
        75 => {
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 93; continue; }
            blk = 76; continue;
        }
        76 => {
            blk = 446; continue;
        }
        77 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080bdb8); };
            c.w32(12, t15);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 91; continue; }
            blk = 78; continue;
        }
        78 => {
            let r16 = c.add(c.r64(15), 0x400u64, 64);
            c.w64(15, r16);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 479; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(15, c.r64(11).wrapping_add(0x1));
            c.w64(7, c.r64(15));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 479; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 477; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080be07); };
            c.w64(15, t17);
            let Some(t18) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080be0e); };
            c.w64(3, t18);
            c.w64(6, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080be1d); };
            c.w64(0, t19);
            let Some(t20) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700080be24); };
            c.cmp(t20, c.r64(1), 64);
            if c.cc_be() { blk = 475; continue; }
            blk = 82; continue;
        }
        82 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x700080be2d); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 83; continue;
        }
        83 => {
            c.w32(15, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080be38); };
            c.w64(0, t21);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700080be3f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(15)) { return RegfileExit::fault(0x700080be42); }
            c.w64(12, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_e() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080be5b); };
            c.w64(15, t22);
            blk = 85; continue;
        }
        85 => {
            let r23 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r23);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(15)) { return RegfileExit::fault(0x700080be66); }
            c.cmp(c.r64(7), c.r64(12), 64);
            if c.cc_ne() { blk = 85; continue; }
            blk = 86; continue;
        }
        86 => {
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_e() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            let r24 = c.sub(c.r64(12), 0x4u64, 64);
            c.w64(12, r24);
            let Some(t25) = c.ld32(c.r64(12).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080be79); };
            c.w32(7, t25);
            if !c.st32(c.r64(12).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x700080be7d); }
            c.cmp(c.r64(12), 0x7u64, 64);
            if c.cc_ne() { blk = 87; continue; }
            blk = 88; continue;
        }
        88 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080be87); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r26 = c.r64(7) & c.r64(2);
            c.w64(7, r26);
            c.logic(r26, 64);
            let Some(t27) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080be99); };
            c.logic(t27 & 0x4u64, 8);
            if c.cc_ne() { blk = 704; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(9, c.r64(1));
            blk = 91; continue;
        }
        91 => {
            let Some(t28) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080bea6); };
            c.w32(1, t28);
            let r29 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r29);
            let Some(t30) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080beac); };
            c.cmp(t30, 0x834u64, 16);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r31 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r31);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700080beba); }
            blk = 93; continue;
        }
        93 => {
            let Some(t32) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080bebd); };
            c.w32(1, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080bec1); };
            c.cmp(t33, c.r32(1), 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t34) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080bece); };
            c.w32(1, t34);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 108; continue; }
            blk = 95; continue;
        }
        95 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 499; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080bef5); };
            c.w64(12, t35);
            let Some(t36) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700080befc); };
            c.w64(15, t36);
            c.w64(0, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(0).wrapping_add(c.r64(15)));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080bf0c); };
            c.w64(6, t37);
            let Some(t38) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700080bf13); };
            c.cmp(t38, c.r64(3), 64);
            if c.cc_be() { blk = 496; continue; }
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(12), c.r64(3)) { return RegfileExit::fault(0x700080bf1c); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 98; continue;
        }
        98 => {
            let r39 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r39);
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080bf26); };
            c.w64(15, t40);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700080bf2d); }
            if !c.st32(c.r64(12).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080bf32); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 101; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t41) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080bf4c); };
            c.w64(8, t41);
            blk = 100; continue;
        }
        100 => {
            let r42 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r42);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(8)) { return RegfileExit::fault(0x700080bf57); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 100; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let r43 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r43);
            let Some(t44) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080bf6a); };
            c.w32(1, t44);
            if !c.st32(c.r64(7).wrapping_add(c.r64(12)), c.r32(1)) { return RegfileExit::fault(0x700080bf6e); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 102; continue; }
            blk = 103; continue;
        }
        103 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700080bf78); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 104; continue;
        }
        104 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r45 = c.r64(1) & c.r64(2);
            c.w64(1, r45);
            c.logic(r45, 64);
            let Some(t46) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080bf8c); };
            c.logic(t46 & 0x4u64, 8);
            if c.cc_ne() { blk = 701; continue; }
            blk = 105; continue;
        }
        105 => {
            c.w64(9, c.r64(12));
            blk = 106; continue;
        }
        106 => {
            let Some(t47) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080bf99); };
            c.w32(1, t47);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 498; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080bfac); };
            c.w64(0, t48);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700080bfb0); }
            blk = 451; continue;
        }
        108 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080bfba); };
            c.w64(0, t49);
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080bfbe); };
            c.w64(7, t50);
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080bfc2); };
            c.w64(3, t51);
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080bfc6); };
            c.w64(1, t52);
            let Some(t53) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080bfca); };
            c.w64(6, t53);
            blk = 446; continue;
        }
        109 => {
            let Some(t54) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080bfd3); };
            c.w32(9, t54);
            let r55 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r55);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t56) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080bfdf); };
            c.w32(11, t56);
            let r57 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r57);
            let Some(t58) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080bfe6); };
            c.cmp(t58, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 111; continue;
        }
        111 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080bff2); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 112; continue;
        }
        112 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t59) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080c015); };
            c.w64(12, t59);
            let r60 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r60);
            let Some(t61) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080c01f); };
            c.w64(15, t61);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 116; continue; }
            blk = 114; continue;
        }
        114 => {
            blk = 446; continue;
        }
        115 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 116; continue;
        }
        116 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080c036); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 446; continue; }
            blk = 117; continue;
        }
        117 => {
            let Some(t62) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c042); };
            c.cmp(t62, 0x834u64, 16);
            if c.cc_e() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t63) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c04b); };
            c.w32(8, t63);
            blk = 120; continue;
        }
        119 => {
            let Some(t64) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080c051); };
            c.w32(8, t64);
            blk = 120; continue;
        }
        120 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700080c05e); }
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 123; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 140; continue; }
            blk = 122; continue;
        }
        122 => {
            blk = 446; continue;
        }
        123 => {
            let Some(t65) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c075); };
            c.w32(12, t65);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 137; continue; }
            blk = 124; continue;
        }
        124 => {
            let r66 = c.add(c.r64(15), 0x400u64, 64);
            c.w64(15, r66);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 505; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w64(15, c.r64(11).wrapping_add(0x1));
            c.w64(7, c.r64(15));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 505; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 503; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080c0c4); };
            c.w64(15, t67);
            let Some(t68) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080c0cb); };
            c.w64(1, t68);
            c.w64(0, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(0).wrapping_add(c.r64(1)));
            let Some(t69) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080c0da); };
            c.w64(11, t69);
            let Some(t70) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700080c0e1); };
            c.cmp(t70, c.r64(8), 64);
            if c.cc_be() { blk = 501; continue; }
            blk = 128; continue;
        }
        128 => {
            if !c.st64(c.r64(15), c.r64(8)) { return RegfileExit::fault(0x700080c0ea); }
            let r71 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r71);
            blk = 129; continue;
        }
        129 => {
            c.w32(8, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080c0f5); };
            c.w64(11, t72);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700080c0fc); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700080c100); }
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 132; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080c119); };
            c.w64(11, t73);
            blk = 131; continue;
        }
        131 => {
            let r74 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r74);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700080c124); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_ne() { blk = 131; continue; }
            blk = 132; continue;
        }
        132 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let r75 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r75);
            let Some(t76) = c.ld32(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080c137); };
            c.w32(7, t76);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x700080c13b); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 133; continue; }
            blk = 134; continue;
        }
        134 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080c145); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r77 = c.r64(7) & c.r64(2);
            c.w64(7, r77);
            c.logic(r77, 64);
            let Some(t78) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c157); };
            c.logic(t78 & 0x4u64, 8);
            if c.cc_ne() { blk = 698; continue; }
            blk = 136; continue;
        }
        136 => {
            c.w64(9, c.r64(1));
            blk = 137; continue;
        }
        137 => {
            let Some(t79) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c164); };
            c.w32(1, t79);
            let r80 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r80);
            let Some(t81) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c16a); };
            c.cmp(t81, 0x834u64, 16);
            if c.cc_e() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080c172); };
            c.w64(11, t82);
            blk = 140; continue;
        }
        139 => {
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080c178); };
            c.w64(11, t83);
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r84 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r84);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700080c182); }
            blk = 140; continue;
        }
        140 => {
            let Some(t85) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c185); };
            c.w32(1, t85);
            let Some(t86) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080c189); };
            c.cmp(t86, c.r32(1), 32);
            if c.cc_ne() { blk = 153; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t87) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c196); };
            c.w32(1, t87);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 155; continue; }
            blk = 142; continue;
        }
        142 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 525; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080c1bd); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700080c1c4); };
            c.w64(12, t89);
            c.w64(15, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t90) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080c1d3); };
            c.w64(3, t90);
            let Some(t91) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700080c1da); };
            c.cmp(t91, c.r64(0), 64);
            if c.cc_be() { blk = 522; continue; }
            blk = 144; continue;
        }
        144 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700080c1e3); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 145; continue;
        }
        145 => {
            let r92 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r92);
            let Some(t93) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080c1ed); };
            c.w64(12, t93);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700080c1f4); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080c1f8); }
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080c1fc); };
            c.w64(7, t94);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 148; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t95) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080c215); };
            c.w64(12, t95);
            blk = 147; continue;
        }
        147 => {
            let r96 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r96);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x700080c220); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 147; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let r97 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r97);
            let Some(t98) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080c233); };
            c.w32(1, t98);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700080c237); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 149; continue; }
            blk = 150; continue;
        }
        150 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700080c241); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 151; continue;
        }
        151 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r99 = c.r64(1) & c.r64(2);
            c.w64(1, r99);
            c.logic(r99, 64);
            let Some(t100) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c255); };
            c.logic(t100 & 0x4u64, 8);
            if c.cc_ne() { blk = 695; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w64(9, c.r64(8));
            blk = 153; continue;
        }
        153 => {
            let Some(t101) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c262); };
            c.w32(1, t101);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 524; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080c275); };
            c.w64(0, t102);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700080c279); }
            blk = 451; continue;
        }
        155 => {
            let Some(t103) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080c283); };
            c.w64(0, t103);
            let Some(t104) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080c287); };
            c.w64(7, t104);
            let Some(t105) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080c28b); };
            c.w64(1, t105);
            blk = 446; continue;
        }
        156 => {
            let Some(t106) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c294); };
            c.w32(9, t106);
            let r107 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r107);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 162; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t108) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c2a0); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c2a7); };
            c.cmp(t110, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 158; continue;
        }
        158 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080c2b3); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 159; continue;
        }
        159 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t111) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080c2d6); };
            c.w64(12, t111);
            let r112 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r112);
            let Some(t113) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080c2e0); };
            c.w64(15, t113);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 163; continue; }
            blk = 161; continue;
        }
        161 => {
            blk = 446; continue;
        }
        162 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 163; continue;
        }
        163 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080c2f7); }
            let Some(t114) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c2fb); };
            c.cmp(t114, 0x834u64, 16);
            if c.cc_e() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t115) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c304); };
            c.w32(8, t115);
            blk = 166; continue;
        }
        165 => {
            let Some(t116) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080c30a); };
            c.w32(8, t116);
            blk = 166; continue;
        }
        166 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700080c317); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_e() { blk = 169; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 186; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 446; continue;
        }
        169 => {
            let Some(t117) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c32e); };
            c.w32(12, t117);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 183; continue; }
            blk = 170; continue;
        }
        170 => {
            let r118 = c.add(c.r64(15), 0x400u64, 64);
            c.w64(15, r118);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 531; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(15, c.r64(11).wrapping_add(0x1));
            c.w64(7, c.r64(15));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 531; continue; }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 529; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080c37d); };
            c.w64(15, t119);
            let Some(t120) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080c384); };
            c.w64(1, t120);
            c.w64(0, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(0).wrapping_add(c.r64(1)));
            let Some(t121) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080c393); };
            c.w64(11, t121);
            let Some(t122) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700080c39a); };
            c.cmp(t122, c.r64(8), 64);
            if c.cc_be() { blk = 527; continue; }
            blk = 174; continue;
        }
        174 => {
            if !c.st64(c.r64(15), c.r64(8)) { return RegfileExit::fault(0x700080c3a3); }
            let r123 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r123);
            blk = 175; continue;
        }
        175 => {
            c.w32(8, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080c3ae); };
            c.w64(11, t124);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700080c3b5); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700080c3b9); }
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 178; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080c3d2); };
            c.w64(11, t125);
            blk = 177; continue;
        }
        177 => {
            let r126 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r126);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700080c3dd); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_ne() { blk = 177; continue; }
            blk = 178; continue;
        }
        178 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            let r127 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r127);
            let Some(t128) = c.ld32(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080c3f0); };
            c.w32(7, t128);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x700080c3f4); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 179; continue; }
            blk = 180; continue;
        }
        180 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080c3fe); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r129 = c.r64(7) & c.r64(2);
            c.w64(7, r129);
            c.logic(r129, 64);
            let Some(t130) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c410); };
            c.logic(t130 & 0x4u64, 8);
            if c.cc_ne() { blk = 692; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(9, c.r64(1));
            blk = 183; continue;
        }
        183 => {
            let Some(t131) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c41d); };
            c.w32(1, t131);
            let r132 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r132);
            let Some(t133) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c423); };
            c.cmp(t133, 0x834u64, 16);
            if c.cc_e() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080c42b); };
            c.w64(11, t134);
            blk = 186; continue;
        }
        185 => {
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080c431); };
            c.w64(11, t135);
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r136 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r136);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700080c43b); }
            blk = 186; continue;
        }
        186 => {
            let Some(t137) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c43e); };
            c.w32(1, t137);
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080c442); };
            c.cmp(t138, c.r32(1), 32);
            if c.cc_ne() { blk = 199; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t139) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c44f); };
            c.w32(1, t139);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 203; continue; }
            blk = 188; continue;
        }
        188 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 551; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080c476); };
            c.w64(8, t140);
            let Some(t141) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700080c47d); };
            c.w64(12, t141);
            c.w64(15, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t142) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080c48c); };
            c.w64(3, t142);
            let Some(t143) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700080c493); };
            c.cmp(t143, c.r64(0), 64);
            if c.cc_be() { blk = 548; continue; }
            blk = 190; continue;
        }
        190 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700080c49c); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 191; continue;
        }
        191 => {
            let r144 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r144);
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080c4a6); };
            c.w64(12, t145);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700080c4ad); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080c4b1); }
            let Some(t146) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080c4b5); };
            c.w64(7, t146);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 194; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080c4ce); };
            c.w64(12, t147);
            blk = 193; continue;
        }
        193 => {
            let r148 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r148);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x700080c4d9); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 193; continue; }
            blk = 194; continue;
        }
        194 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            let r149 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r149);
            let Some(t150) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080c4ec); };
            c.w32(1, t150);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700080c4f0); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 195; continue; }
            blk = 196; continue;
        }
        196 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700080c4fa); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r151 = c.r64(1) & c.r64(2);
            c.w64(1, r151);
            c.logic(r151, 64);
            let Some(t152) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c50e); };
            c.logic(t152 & 0x4u64, 8);
            if c.cc_ne() { blk = 689; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(9, c.r64(8));
            blk = 199; continue;
        }
        199 => {
            let Some(t153) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c51b); };
            c.w32(1, t153);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 550; continue; }
            blk = 200; continue;
        }
        200 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080c536); };
            c.w64(0, t154);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700080c53a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r155 = c.r64(7) & c.r64(9);
            c.w64(7, r155);
            c.logic(r155, 64);
            let Some(t156) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c54c); };
            c.logic(t156 & 0x4u64, 8);
            if c.cc_ne() { blk = 686; continue; }
            blk = 202; continue;
        }
        202 => {
            blk = 451; continue;
        }
        203 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080c55b); };
            c.w64(0, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080c55f); };
            c.w64(7, t158);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080c563); };
            c.w64(1, t159);
            blk = 446; continue;
        }
        204 => {
            let Some(t160) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c56c); };
            c.w32(9, t160);
            let r161 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r161);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 210; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t162) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c578); };
            c.w32(11, t162);
            let r163 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r163);
            let Some(t164) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c57f); };
            c.cmp(t164, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 206; continue;
        }
        206 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080c58b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 207; continue;
        }
        207 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t165) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080c5ae); };
            c.w64(12, t165);
            let r166 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r166);
            let Some(t167) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080c5b8); };
            c.w64(15, t167);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 211; continue; }
            blk = 209; continue;
        }
        209 => {
            blk = 446; continue;
        }
        210 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 211; continue;
        }
        211 => {
            let Some(t168) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c5cf); };
            c.cmp(t168, 0x834u64, 16);
            if c.cc_e() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t169) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c5d8); };
            c.w32(8, t169);
            blk = 214; continue;
        }
        213 => {
            let Some(t170) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080c5de); };
            c.w32(8, t170);
            blk = 214; continue;
        }
        214 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 446; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t171) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c5f4); };
            c.w32(12, t171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080c5f8); };
            c.cmp(t172, c.r32(12), 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t173) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c605); };
            c.w32(12, t173);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 446; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 556; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t174) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080c62c); };
            c.w64(1, t174);
            let Some(t175) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700080c633); };
            c.w64(7, t175);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t176) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080c642); };
            c.w64(0, t176);
            let Some(t177) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700080c649); };
            c.cmp(t177, c.r64(6), 64);
            if c.cc_be() { blk = 553; continue; }
            blk = 219; continue;
        }
        219 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700080c652); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 220; continue;
        }
        220 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080c65d); };
            c.w64(15, t178);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700080c664); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080c668); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 223; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t179) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080c680); };
            c.w64(12, t179);
            blk = 222; continue;
        }
        222 => {
            let r180 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r180);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700080c68b); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 222; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 225; continue; }
            blk = 224; continue;
        }
        224 => {
            let r181 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r181);
            let Some(t182) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080c69e); };
            c.w32(8, t182);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x700080c6a2); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 224; continue; }
            blk = 225; continue;
        }
        225 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080c6ac); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r183 = c.r64(7) & c.r64(2);
            c.w64(7, r183);
            c.logic(r183, 64);
            let Some(t184) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c6be); };
            c.logic(t184 & 0x4u64, 8);
            if c.cc_ne() { blk = 683; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(9, c.r64(1));
            blk = 228; continue;
        }
        228 => {
            let Some(t185) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c6cb); };
            c.w32(1, t185);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 555; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t186) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080c6e6); };
            c.w64(0, t186);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700080c6ea); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 230; continue;
        }
        230 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r187 = c.r64(7) & c.r64(9);
            c.w64(7, r187);
            c.logic(r187, 64);
            let Some(t188) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c6fc); };
            c.logic(t188 & 0x4u64, 8);
            if c.cc_ne() { blk = 680; continue; }
            blk = 231; continue;
        }
        231 => {
            blk = 451; continue;
        }
        232 => {
            let Some(t189) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c70b); };
            c.w32(9, t189);
            let r190 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r190);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 238; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t191) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c717); };
            c.w32(11, t191);
            let r192 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r192);
            let Some(t193) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c71e); };
            c.cmp(t193, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 234; continue;
        }
        234 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080c72a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 235; continue;
        }
        235 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t194) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080c74d); };
            c.w64(12, t194);
            let r195 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r195);
            let Some(t196) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080c757); };
            c.w64(15, t196);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 239; continue; }
            blk = 237; continue;
        }
        237 => {
            blk = 446; continue;
        }
        238 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 239; continue;
        }
        239 => {
            let Some(t197) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c76e); };
            c.cmp(t197, 0x834u64, 16);
            if c.cc_e() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t198) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c777); };
            c.w32(8, t198);
            blk = 242; continue;
        }
        241 => {
            let Some(t199) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080c77d); };
            c.w32(8, t199);
            blk = 242; continue;
        }
        242 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 446; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t200) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c793); };
            c.w32(12, t200);
            let Some(t201) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080c797); };
            c.cmp(t201, c.r32(12), 32);
            if c.cc_ne() { blk = 256; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t202) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c7a4); };
            c.w32(12, t202);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 446; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 561; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080c7cb); };
            c.w64(1, t203);
            let Some(t204) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700080c7d2); };
            c.w64(7, t204);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080c7e1); };
            c.w64(0, t205);
            let Some(t206) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700080c7e8); };
            c.cmp(t206, c.r64(6), 64);
            if c.cc_be() { blk = 558; continue; }
            blk = 247; continue;
        }
        247 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700080c7f1); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 248; continue;
        }
        248 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080c7fc); };
            c.w64(15, t207);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700080c803); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080c807); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 251; continue; }
            blk = 249; continue;
        }
        249 => {
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080c81f); };
            c.w64(12, t208);
            blk = 250; continue;
        }
        250 => {
            let r209 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r209);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700080c82a); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 250; continue; }
            blk = 251; continue;
        }
        251 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            let r210 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r210);
            let Some(t211) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080c83d); };
            c.w32(8, t211);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x700080c841); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 252; continue; }
            blk = 253; continue;
        }
        253 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080c84b); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r212 = c.r64(7) & c.r64(2);
            c.w64(7, r212);
            c.logic(r212, 64);
            let Some(t213) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c85d); };
            c.logic(t213 & 0x4u64, 8);
            if c.cc_ne() { blk = 677; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w64(9, c.r64(1));
            blk = 256; continue;
        }
        256 => {
            let Some(t214) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c86a); };
            c.w32(1, t214);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 560; continue; }
            blk = 257; continue;
        }
        257 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080c885); };
            c.w64(0, t215);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700080c889); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 258; continue;
        }
        258 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r216 = c.r64(7) & c.r64(9);
            c.w64(7, r216);
            c.logic(r216, 64);
            let Some(t217) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080c89b); };
            c.logic(t217 & 0x4u64, 8);
            if c.cc_ne() { blk = 674; continue; }
            blk = 259; continue;
        }
        259 => {
            blk = 451; continue;
        }
        260 => {
            let Some(t218) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c8aa); };
            c.w32(9, t218);
            let r219 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r219);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t220) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080c8b6); };
            c.w32(11, t220);
            let r221 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r221);
            let Some(t222) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c8bd); };
            c.cmp(t222, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 262; continue;
        }
        262 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080c8c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 263; continue;
        }
        263 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t223) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080c8ec); };
            c.w64(12, t223);
            let r224 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r224);
            let Some(t225) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080c8f6); };
            c.w64(15, t225);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 267; continue; }
            blk = 265; continue;
        }
        265 => {
            blk = 446; continue;
        }
        266 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 267; continue;
        }
        267 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080c90d); }
            let Some(t226) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080c911); };
            c.cmp(t226, 0x834u64, 16);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t227) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c91a); };
            c.w32(8, t227);
            blk = 270; continue;
        }
        269 => {
            let Some(t228) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080c920); };
            c.w32(8, t228);
            blk = 270; continue;
        }
        270 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700080c92d); }
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 273; continue; }
            blk = 271; continue;
        }
        271 => {
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 290; continue; }
            blk = 272; continue;
        }
        272 => {
            blk = 446; continue;
        }
        273 => {
            let Some(t229) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080c944); };
            c.w32(12, t229);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 287; continue; }
            blk = 274; continue;
        }
        274 => {
            let r230 = c.add(c.r64(15), 0x400u64, 64);
            c.w64(15, r230);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 567; continue; }
            blk = 275; continue;
        }
        275 => {
            c.w64(15, c.r64(11).wrapping_add(0x1));
            c.w64(7, c.r64(15));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 567; continue; }
            blk = 276; continue;
        }
        276 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 565; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080c993); };
            c.w64(15, t231);
            let Some(t232) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080c99a); };
            c.w64(1, t232);
            c.w64(0, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(0).wrapping_add(c.r64(1)));
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080c9a9); };
            c.w64(11, t233);
            let Some(t234) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700080c9b0); };
            c.cmp(t234, c.r64(8), 64);
            if c.cc_be() { blk = 563; continue; }
            blk = 278; continue;
        }
        278 => {
            if !c.st64(c.r64(15), c.r64(8)) { return RegfileExit::fault(0x700080c9b9); }
            let r235 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r235);
            blk = 279; continue;
        }
        279 => {
            c.w32(8, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080c9c4); };
            c.w64(11, t236);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700080c9cb); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x700080c9cf); }
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_e() { blk = 282; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t237) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080c9e8); };
            c.w64(11, t237);
            blk = 281; continue;
        }
        281 => {
            let r238 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r238);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700080c9f3); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_ne() { blk = 281; continue; }
            blk = 282; continue;
        }
        282 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let r239 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r239);
            let Some(t240) = c.ld32(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080ca06); };
            c.w32(7, t240);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x700080ca0a); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 283; continue; }
            blk = 284; continue;
        }
        284 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080ca14); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r241 = c.r64(7) & c.r64(2);
            c.w64(7, r241);
            c.logic(r241, 64);
            let Some(t242) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080ca26); };
            c.logic(t242 & 0x4u64, 8);
            if c.cc_ne() { blk = 671; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w64(9, c.r64(1));
            blk = 287; continue;
        }
        287 => {
            let Some(t243) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ca33); };
            c.w32(1, t243);
            let r244 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r244);
            let Some(t245) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ca39); };
            c.cmp(t245, 0x834u64, 16);
            if c.cc_e() { blk = 289; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t246) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080ca41); };
            c.w64(11, t246);
            blk = 290; continue;
        }
        289 => {
            let Some(t247) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080ca47); };
            c.w64(11, t247);
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r248 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r248);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700080ca51); }
            blk = 290; continue;
        }
        290 => {
            let Some(t249) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ca54); };
            c.w32(1, t249);
            let Some(t250) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080ca58); };
            c.cmp(t250, c.r32(1), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 291; continue;
        }
        291 => {
            let Some(t251) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080ca65); };
            c.w32(1, t251);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 307; continue; }
            blk = 292; continue;
        }
        292 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 587; continue; }
            blk = 293; continue;
        }
        293 => {
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080ca8c); };
            c.w64(8, t252);
            let Some(t253) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700080ca93); };
            c.w64(12, t253);
            c.w64(15, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080caa2); };
            c.w64(3, t254);
            let Some(t255) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700080caa9); };
            c.cmp(t255, c.r64(0), 64);
            if c.cc_be() { blk = 584; continue; }
            blk = 294; continue;
        }
        294 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x700080cab2); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 295; continue;
        }
        295 => {
            let r256 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r256);
            let Some(t257) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080cabc); };
            c.w64(12, t257);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700080cac3); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080cac7); }
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080cacb); };
            c.w64(7, t258);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 298; continue; }
            blk = 296; continue;
        }
        296 => {
            let Some(t259) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080cae4); };
            c.w64(12, t259);
            blk = 297; continue;
        }
        297 => {
            let r260 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r260);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x700080caef); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 297; continue; }
            blk = 298; continue;
        }
        298 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 300; continue; }
            blk = 299; continue;
        }
        299 => {
            let r261 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r261);
            let Some(t262) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080cb02); };
            c.w32(1, t262);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x700080cb06); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 299; continue; }
            blk = 300; continue;
        }
        300 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x700080cb10); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r263 = c.r64(1) & c.r64(2);
            c.w64(1, r263);
            c.logic(r263, 64);
            let Some(t264) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080cb24); };
            c.logic(t264 & 0x4u64, 8);
            if c.cc_ne() { blk = 668; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(9, c.r64(8));
            blk = 303; continue;
        }
        303 => {
            let Some(t265) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cb31); };
            c.w32(1, t265);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 586; continue; }
            blk = 304; continue;
        }
        304 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080cb4c); };
            c.w64(0, t266);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700080cb50); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 306; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r267 = c.r64(7) & c.r64(9);
            c.w64(7, r267);
            c.logic(r267, 64);
            let Some(t268) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080cb62); };
            c.logic(t268 & 0x4u64, 8);
            if c.cc_ne() { blk = 665; continue; }
            blk = 306; continue;
        }
        306 => {
            blk = 451; continue;
        }
        307 => {
            let Some(t269) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080cb71); };
            c.w64(0, t269);
            let Some(t270) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080cb75); };
            c.w64(7, t270);
            let Some(t271) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080cb79); };
            c.w64(1, t271);
            blk = 446; continue;
        }
        308 => {
            let Some(t272) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cb82); };
            c.w32(9, t272);
            let r273 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r273);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t274) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080cb8e); };
            c.w32(11, t274);
            let r275 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r275);
            let Some(t276) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cb95); };
            c.cmp(t276, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 310; continue;
        }
        310 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080cba1); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 311; continue;
        }
        311 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t277) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080cbc4); };
            c.w64(12, t277);
            let r278 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r278);
            let Some(t279) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080cbce); };
            c.w64(15, t279);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 315; continue; }
            blk = 313; continue;
        }
        313 => {
            blk = 446; continue;
        }
        314 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 315; continue;
        }
        315 => {
            let Some(t280) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cbe5); };
            c.cmp(t280, 0x834u64, 16);
            if c.cc_e() { blk = 317; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t281) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cbee); };
            c.w32(8, t281);
            blk = 318; continue;
        }
        317 => {
            let Some(t282) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080cbf4); };
            c.w32(8, t282);
            blk = 318; continue;
        }
        318 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 446; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t283) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cc0a); };
            c.w32(12, t283);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ae() { blk = 589; continue; }
            blk = 320; continue;
        }
        320 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t284) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cc28); };
            c.w32(3, t284);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700080cc2d); }
            let Some(t285) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080cc31); };
            c.cmp(t285, c.r32(3), 32);
            if c.cc_e() { blk = 339; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t286) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080cc3e); };
            c.w32(3, t286);
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080cc42); };
            c.cmp(t287, c.r32(3), 32);
            if c.cc_ne() { blk = 334; continue; }
            blk = 322; continue;
        }
        322 => {
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 338; continue; }
            blk = 323; continue;
        }
        323 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 593; continue; }
            blk = 324; continue;
        }
        324 => {
            let Some(t288) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080cc69); };
            c.w64(1, t288);
            let Some(t289) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700080cc70); };
            c.w64(3, t289);
            c.w64(6, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t290) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080cc7f); };
            c.w64(0, t290);
            let Some(t291) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700080cc86); };
            c.cmp(t291, c.r64(7), 64);
            if c.cc_be() { blk = 590; continue; }
            blk = 325; continue;
        }
        325 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700080cc8f); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 326; continue;
        }
        326 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080cc9a); };
            c.w64(15, t292);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700080cca1); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080cca5); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 329; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080ccbd); };
            c.w64(12, t293);
            blk = 328; continue;
        }
        328 => {
            let r294 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r294);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700080ccc8); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 328; continue; }
            blk = 329; continue;
        }
        329 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 331; continue; }
            blk = 330; continue;
        }
        330 => {
            let r295 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r295);
            let Some(t296) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080ccdb); };
            c.w32(8, t296);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x700080ccdf); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 330; continue; }
            blk = 331; continue;
        }
        331 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080cce9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r297 = c.r64(7) & c.r64(2);
            c.w64(7, r297);
            c.logic(r297, 64);
            let Some(t298) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080ccfb); };
            c.logic(t298 & 0x4u64, 8);
            if c.cc_ne() { blk = 662; continue; }
            blk = 333; continue;
        }
        333 => {
            c.w64(9, c.r64(1));
            blk = 334; continue;
        }
        334 => {
            let Some(t299) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cd08); };
            c.w32(1, t299);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 592; continue; }
            blk = 335; continue;
        }
        335 => {
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080cd1b); };
            c.w64(0, t300);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080cd1f); };
            c.w64(1, t301);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700080cd23); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 337; continue; }
            blk = 336; continue;
        }
        336 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r302 = c.r64(7) & c.r64(9);
            c.w64(7, r302);
            c.logic(r302, 64);
            let Some(t303) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080cd35); };
            c.logic(t303 & 0x4u64, 8);
            if c.cc_ne() { blk = 659; continue; }
            blk = 337; continue;
        }
        337 => {
            blk = 451; continue;
        }
        338 => {
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080cd44); };
            c.w64(7, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080cd48); };
            c.w64(3, t305);
            blk = 446; continue;
        }
        339 => {
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080cd51); };
            c.w64(7, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080cd55); };
            c.w64(3, t307);
            blk = 446; continue;
        }
        340 => {
            let Some(t308) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cd5e); };
            c.w32(9, t308);
            let r309 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r309);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 346; continue; }
            blk = 341; continue;
        }
        341 => {
            let Some(t310) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080cd6a); };
            c.w32(11, t310);
            let r311 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r311);
            let Some(t312) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cd71); };
            c.cmp(t312, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 342; continue;
        }
        342 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080cd7d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 343; continue;
        }
        343 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 344; continue;
        }
        344 => {
            let Some(t313) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080cda0); };
            c.w64(12, t313);
            let r314 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r314);
            let Some(t315) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080cdaa); };
            c.w64(15, t315);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 347; continue; }
            blk = 345; continue;
        }
        345 => {
            blk = 446; continue;
        }
        346 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 347; continue;
        }
        347 => {
            let Some(t316) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cdc1); };
            c.cmp(t316, 0x834u64, 16);
            if c.cc_e() { blk = 349; continue; }
            blk = 348; continue;
        }
        348 => {
            let Some(t317) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cdca); };
            c.w32(8, t317);
            blk = 350; continue;
        }
        349 => {
            let Some(t318) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080cdd0); };
            c.w32(8, t318);
            blk = 350; continue;
        }
        350 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 446; continue; }
            blk = 351; continue;
        }
        351 => {
            let Some(t319) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cde6); };
            c.w32(12, t319);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ae() { blk = 595; continue; }
            blk = 352; continue;
        }
        352 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t320) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ce04); };
            c.w32(3, t320);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700080ce09); }
            let Some(t321) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080ce0d); };
            c.cmp(t321, c.r32(3), 32);
            if c.cc_e() { blk = 371; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t322) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ce1a); };
            c.w32(3, t322);
            let Some(t323) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080ce1e); };
            c.cmp(t323, c.r32(3), 32);
            if c.cc_ne() { blk = 366; continue; }
            blk = 354; continue;
        }
        354 => {
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 370; continue; }
            blk = 355; continue;
        }
        355 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 599; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t324) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080ce45); };
            c.w64(1, t324);
            let Some(t325) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700080ce4c); };
            c.w64(3, t325);
            c.w64(6, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t326) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080ce5b); };
            c.w64(0, t326);
            let Some(t327) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700080ce62); };
            c.cmp(t327, c.r64(7), 64);
            if c.cc_be() { blk = 596; continue; }
            blk = 357; continue;
        }
        357 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700080ce6b); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 358; continue;
        }
        358 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700080ce76); };
            c.w64(15, t328);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700080ce7d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080ce81); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 361; continue; }
            blk = 359; continue;
        }
        359 => {
            let Some(t329) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080ce99); };
            c.w64(12, t329);
            blk = 360; continue;
        }
        360 => {
            let r330 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r330);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700080cea4); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 360; continue; }
            blk = 361; continue;
        }
        361 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 363; continue; }
            blk = 362; continue;
        }
        362 => {
            let r331 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r331);
            let Some(t332) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080ceb7); };
            c.w32(8, t332);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x700080cebb); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 362; continue; }
            blk = 363; continue;
        }
        363 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080cec5); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 365; continue; }
            blk = 364; continue;
        }
        364 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r333 = c.r64(7) & c.r64(2);
            c.w64(7, r333);
            c.logic(r333, 64);
            let Some(t334) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080ced7); };
            c.logic(t334 & 0x4u64, 8);
            if c.cc_ne() { blk = 656; continue; }
            blk = 365; continue;
        }
        365 => {
            c.w64(9, c.r64(1));
            blk = 366; continue;
        }
        366 => {
            let Some(t335) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cee4); };
            c.w32(1, t335);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 598; continue; }
            blk = 367; continue;
        }
        367 => {
            let Some(t336) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080cef7); };
            c.w64(0, t336);
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080cefb); };
            c.w64(1, t337);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700080ceff); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 369; continue; }
            blk = 368; continue;
        }
        368 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r338 = c.r64(7) & c.r64(9);
            c.w64(7, r338);
            c.logic(r338, 64);
            let Some(t339) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080cf11); };
            c.logic(t339 & 0x4u64, 8);
            if c.cc_ne() { blk = 653; continue; }
            blk = 369; continue;
        }
        369 => {
            blk = 451; continue;
        }
        370 => {
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080cf20); };
            c.w64(7, t340);
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080cf24); };
            c.w64(3, t341);
            blk = 446; continue;
        }
        371 => {
            let Some(t342) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080cf2d); };
            c.w64(7, t342);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080cf31); };
            c.w64(3, t343);
            blk = 446; continue;
        }
        372 => {
            let Some(t344) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cf3a); };
            c.w32(9, t344);
            let r345 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r345);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 378; continue; }
            blk = 373; continue;
        }
        373 => {
            let Some(t346) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080cf46); };
            c.w32(11, t346);
            let r347 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r347);
            let Some(t348) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cf4d); };
            c.cmp(t348, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 374; continue;
        }
        374 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080cf59); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 375; continue;
        }
        375 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 376; continue;
        }
        376 => {
            let Some(t349) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080cf7c); };
            c.w64(12, t349);
            let r350 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r350);
            let Some(t351) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080cf86); };
            c.w64(15, t351);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 379; continue; }
            blk = 377; continue;
        }
        377 => {
            blk = 446; continue;
        }
        378 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 379; continue;
        }
        379 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 382; continue; }
            blk = 380; continue;
        }
        380 => {
            let Some(t352) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080cfa1); };
            c.w32(12, t352);
            let Some(t353) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080cfa5); };
            c.cmp(t353, c.r32(12), 32);
            if c.cc_ne() { blk = 446; continue; }
            blk = 381; continue;
        }
        381 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080cfb2); }
            blk = 383; continue;
        }
        382 => {
            c.w64(12, c.r64(0));
            let r354 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r354);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 383; continue;
        }
        383 => {
            let Some(t355) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080cfc8); };
            c.cmp(t355, 0x834u64, 16);
            if c.cc_e() { blk = 385; continue; }
            blk = 384; continue;
        }
        384 => {
            let Some(t356) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cfd1); };
            c.w32(8, t356);
            blk = 386; continue;
        }
        385 => {
            let Some(t357) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080cfd7); };
            c.w32(8, t357);
            blk = 386; continue;
        }
        386 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_e() { blk = 389; continue; }
            blk = 387; continue;
        }
        387 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 409; continue; }
            blk = 388; continue;
        }
        388 => {
            blk = 446; continue;
        }
        389 => {
            let Some(t358) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080cff4); };
            c.w32(8, t358);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 407; continue; }
            blk = 390; continue;
        }
        390 => {
            let r359 = c.add(c.r64(12), 0x400u64, 64);
            c.w64(12, r359);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 607; continue; }
            blk = 391; continue;
        }
        391 => {
            c.w64(12, c.r64(11).wrapping_add(0x1));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(12, c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x10));
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 607; continue; }
            blk = 392; continue;
        }
        392 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 605; continue; }
            blk = 393; continue;
        }
        393 => {
            c.w64(1, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 601; continue; }
            blk = 394; continue;
        }
        394 => {
            let Some(t360) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080d058); };
            c.w64(7, t360);
            let Some(t361) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080d05f); };
            c.w64(15, t361);
            c.w64(3, c.r64(1).wrapping_add(c.r64(15)));
            let Some(t362) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080d066); };
            c.cmp(t362, c.r64(3), 64);
            if c.cc_be() { blk = 603; continue; }
            blk = 395; continue;
        }
        395 => {
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x700080d070); }
            c.w64(1, c.r64(15).wrapping_add(0x1));
            blk = 396; continue;
        }
        396 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t363) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700080d07b); };
            c.w64(15, t363);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700080d082); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080d086); }
            let Some(t364) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080d089); };
            c.w64(7, t364);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700080d090); }
            c.w64(7, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 398; continue; }
            blk = 397; continue;
        }
        397 => {
            let r365 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r365);
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x700080d0ae); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 397; continue; }
            blk = 398; continue;
        }
        398 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 404; continue; }
            blk = 399; continue;
        }
        399 => {
            c.w64(8, 0xfff7fffffff7ffffu64);
            blk = 400; continue;
        }
        400 => {
            let r366 = c.sub(c.r64(7), 0x8u64, 64);
            c.w64(7, r366);
            let Some(t367) = c.ld64(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080d0cd); };
            c.cmp(t367, c.r64(8), 64);
            if c.cc_e() { blk = 402; continue; }
            blk = 401; continue;
        }
        401 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(c.r64(9))) { return RegfileExit::fault(0x700080d0d3); }
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x700080d0d9); }
            blk = 403; continue;
        }
        402 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(1)), c.r64(8)) { return RegfileExit::fault(0x700080d0e0); }
            blk = 403; continue;
        }
        403 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 400; continue; }
            blk = 404; continue;
        }
        404 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080d0ea); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 406; continue; }
            blk = 405; continue;
        }
        405 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r368 = c.r64(7) & c.r64(2);
            c.w64(7, r368);
            c.logic(r368, 64);
            let Some(t369) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080d0fc); };
            c.logic(t369 & 0x4u64, 8);
            if c.cc_ne() { blk = 650; continue; }
            blk = 406; continue;
        }
        406 => {
            c.w64(9, c.r64(1));
            blk = 407; continue;
        }
        407 => {
            let Some(t370) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080d109); };
            c.w32(1, t370);
            let r371 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r371);
            let Some(t372) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080d10f); };
            c.cmp(t372, 0x834u64, 16);
            if c.cc_ne() { blk = 409; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r373 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r373);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700080d11d); }
            blk = 409; continue;
        }
        409 => {
            let Some(t374) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080d120); };
            c.w32(1, t374);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 623; continue; }
            blk = 410; continue;
        }
        410 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700080d13d); }
            blk = 451; continue;
        }
        411 => {
            let Some(t375) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080d149); };
            c.w32(9, t375);
            let r376 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r376);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 417; continue; }
            blk = 412; continue;
        }
        412 => {
            let Some(t377) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080d155); };
            c.w32(11, t377);
            let r378 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r378);
            let Some(t379) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080d15c); };
            c.cmp(t379, 0x42u64, 16);
            if c.cc_ne() { blk = 446; continue; }
            blk = 413; continue;
        }
        413 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080d168); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 446; continue; }
            blk = 414; continue;
        }
        414 => {
            if c.cc_ne() { blk = 446; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t380) = c.ld64(0x700080bd5cu64) else { return RegfileExit::fault(0x700080d18b); };
            c.w64(12, t380);
            let r381 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r381);
            let Some(t382) = c.ld64(0x700080bd69u64) else { return RegfileExit::fault(0x700080d195); };
            c.w64(15, t382);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 418; continue; }
            blk = 416; continue;
        }
        416 => {
            blk = 446; continue;
        }
        417 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 418; continue;
        }
        418 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 421; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t383) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080d1b0); };
            c.w32(12, t383);
            let Some(t384) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080d1b4); };
            c.cmp(t384, c.r32(12), 32);
            if c.cc_ne() { blk = 446; continue; }
            blk = 420; continue;
        }
        420 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080d1c1); }
            blk = 422; continue;
        }
        421 => {
            c.w64(12, c.r64(0));
            let r385 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r385);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 422; continue;
        }
        422 => {
            let Some(t386) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080d1d7); };
            c.cmp(t386, 0x834u64, 16);
            if c.cc_e() { blk = 424; continue; }
            blk = 423; continue;
        }
        423 => {
            let Some(t387) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080d1e0); };
            c.w32(8, t387);
            blk = 425; continue;
        }
        424 => {
            let Some(t388) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080d1e6); };
            c.w32(8, t388);
            blk = 425; continue;
        }
        425 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_be() { blk = 428; continue; }
            blk = 426; continue;
        }
        426 => {
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 449; continue; }
            blk = 427; continue;
        }
        427 => {
            blk = 446; continue;
        }
        428 => {
            let Some(t389) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080d203); };
            c.w32(8, t389);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 447; continue; }
            blk = 429; continue;
        }
        429 => {
            let r390 = c.add(c.r64(12), 0x400u64, 64);
            c.w64(12, r390);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 630; continue; }
            blk = 430; continue;
        }
        430 => {
            c.w64(12, c.r64(11).wrapping_add(0x1));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(12, c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x10));
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 630; continue; }
            blk = 431; continue;
        }
        431 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 628; continue; }
            blk = 432; continue;
        }
        432 => {
            c.w64(1, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 624; continue; }
            blk = 433; continue;
        }
        433 => {
            let Some(t391) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080d267); };
            c.w64(7, t391);
            let Some(t392) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080d26e); };
            c.w64(15, t392);
            c.w64(3, c.r64(1).wrapping_add(c.r64(15)));
            let Some(t393) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080d275); };
            c.cmp(t393, c.r64(3), 64);
            if c.cc_be() { blk = 626; continue; }
            blk = 434; continue;
        }
        434 => {
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x700080d27f); }
            c.w64(1, c.r64(15).wrapping_add(0x1));
            blk = 435; continue;
        }
        435 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t394) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700080d28a); };
            c.w64(15, t394);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700080d291); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700080d295); }
            let Some(t395) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080d298); };
            c.w64(7, t395);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700080d29f); }
            c.w64(7, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 437; continue; }
            blk = 436; continue;
        }
        436 => {
            let r396 = c.sub(c.r64(8), 0x8u64, 64);
            c.w64(8, r396);
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x700080d2bd); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 436; continue; }
            blk = 437; continue;
        }
        437 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 443; continue; }
            blk = 438; continue;
        }
        438 => {
            let Some(t397) = c.ld64(0x700080d0c1u64) else { return RegfileExit::fault(0x700080d2ce); };
            c.w64(8, t397);
            blk = 439; continue;
        }
        439 => {
            let r398 = c.sub(c.r64(7), 0x8u64, 64);
            c.w64(7, r398);
            let Some(t399) = c.ld64(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x700080d2d9); };
            c.cmp(t399, c.r64(8), 64);
            if c.cc_e() { blk = 441; continue; }
            blk = 440; continue;
        }
        440 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(c.r64(9))) { return RegfileExit::fault(0x700080d2df); }
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x700080d2e5); }
            blk = 442; continue;
        }
        441 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(1)), c.r64(8)) { return RegfileExit::fault(0x700080d2ec); }
            blk = 442; continue;
        }
        442 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 439; continue; }
            blk = 443; continue;
        }
        443 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700080d2f6); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 445; continue; }
            blk = 444; continue;
        }
        444 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r400 = c.r64(7) & c.r64(2);
            c.w64(7, r400);
            c.logic(r400, 64);
            let Some(t401) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080d308); };
            c.logic(t401 & 0x4u64, 8);
            if c.cc_ne() { blk = 647; continue; }
            blk = 445; continue;
        }
        445 => {
            c.w64(9, c.r64(1));
            blk = 447; continue;
        }
        446 => {
            let Some(t402) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700080d317); };
            c.w64(5, t402);
            let r403 = c.add(c.r64(4), 0x70u64, 64);
            c.w64(4, r403);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080d31f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700080d320); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080d321); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700080d322); }
            let Some(t404) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080d323); };
            if !c.push64(t404) { return RegfileExit::fault(0x700080d323); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700080d327); }
            c.w32(0, 0x5u64);
            let Some(t405) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x700080d331); };
            c.w64(3, t405);
            return RegfileExit::jump(0x700086ed74);
        }
        447 => {
            let Some(t406) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080d33d); };
            c.w32(1, t406);
            let r407 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r407);
            let Some(t408) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080d343); };
            c.cmp(t408, 0x834u64, 16);
            if c.cc_ne() { blk = 449; continue; }
            blk = 448; continue;
        }
        448 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r409 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r409);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x700080d351); }
            blk = 449; continue;
        }
        449 => {
            let Some(t410) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080d354); };
            c.w32(1, t410);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 646; continue; }
            blk = 450; continue;
        }
        450 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700080d371); }
            blk = 451; continue;
        }
        451 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700080d37b); }
            return c.ret(0x0, 0x700080d37c);
        }
        452 => {
            return RegfileExit::fault(0x700080d37d);
        }
        453 => {
            return RegfileExit::fault(0x700080d37e);
        }
        454 => {
            return RegfileExit::fault(0x700080d37f);
        }
        455 => {
            return RegfileExit::fault(0x700080d380);
        }
        456 => {
            return RegfileExit::fault(0x700080d381);
        }
        457 => {
            return RegfileExit::fault(0x700080d382);
        }
        458 => {
            return RegfileExit::fault(0x700080d383);
        }
        459 => {
            return RegfileExit::fault(0x700080d384);
        }
        460 => {
            return RegfileExit::fault(0x700080d385);
        }
        461 => {
            return RegfileExit::fault(0x700080d386);
        }
        462 => {
            return RegfileExit::fault(0x700080d387);
        }
        463 => {
            return RegfileExit::fault(0x700080d388);
        }
        464 => {
            return RegfileExit::fault(0x700080d389);
        }
        465 => {
            return RegfileExit::fault(0x700080d38a);
        }
        466 => {
            return RegfileExit::fault(0x700080d38b);
        }
        467 => {
            return RegfileExit::fault(0x700080d38c);
        }
        468 => {
            return RegfileExit::fault(0x700080d38d);
        }
        469 => {
            return RegfileExit::fault(0x700080d38e);
        }
        470 => {
            return RegfileExit::fault(0x700080d38f);
        }
        471 => {
            return RegfileExit::fault(0x700080d390);
        }
        472 => {
            return RegfileExit::fault(0x700080d391);
        }
        473 => {
            return RegfileExit::fault(0x700080d392);
        }
        474 => {
            return RegfileExit::fault(0x700080d393);
        }
        475 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d394); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080d398); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700080d39c); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080d3a0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700080d3a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080d3a8); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700080d3b4, 0x700080d3af);
        }
        476 => {
            c.w64(1, c.r64(0));
            let Some(t411) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d3b7); };
            c.w64(2, t411);
            let Some(t412) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d3bb); };
            c.w64(11, t412);
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d3bf); };
            c.w64(8, t413);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d3c3); };
            c.w64(9, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d3c7); };
            c.w64(7, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d3cb); };
            c.w64(12, t416);
            blk = 83; continue;
        }
        477 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t417) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d3d6); };
            c.w64(3, t417);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080d3e4, 0x700080d3df);
        }
        478 => {
            return RegfileExit::fault(0x700080d3e4);
        }
        479 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d3e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080d3e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700080d3ed); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 495; continue; }
            blk = 480; continue;
        }
        480 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 482; continue; }
            blk = 481; continue;
        }
        481 => {
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            blk = 491; continue;
        }
        482 => {
            let Some(t418) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080d40d); };
            c.w64(7, t418);
            let Some(t419) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080d414); };
            c.w64(9, t419);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t420) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080d41b); };
            c.w64(15, t420);
            let Some(t421) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080d422); };
            c.cmp(t421, c.r64(12), 64);
            if c.cc_a() { blk = 485; continue; }
            blk = 483; continue;
        }
        483 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700080d434, 0x700080d42f);
        }
        484 => {
            c.w64(7, c.r64(0));
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080d437); };
            c.w64(0, t422);
            let Some(t423) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d43b); };
            c.w64(2, t423);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d43f); };
            c.w64(3, t424);
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080d443); };
            c.w64(1, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d447); };
            c.w64(6, t426);
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d44b); };
            c.w64(11, t427);
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d44f); };
            c.w64(8, t428);
            blk = 486; continue;
        }
        485 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x700080d455); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 486; continue;
        }
        486 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 490; continue; }
            blk = 487; continue;
        }
        487 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 489; continue; }
            blk = 488; continue;
        }
        488 => {
            let r429 = c.r64(10) | 0x1u64;
            c.w64(10, r429);
            c.logic(r429, 64);
            blk = 489; continue;
        }
        489 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 490; continue;
        }
        490 => {
            let Some(t430) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080d483); };
            c.w64(9, t430);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080d48a); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080d48e); }
            blk = 491; continue;
        }
        491 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080d493); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080d494); }
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x2u64);
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700080d49c); };
            c.w64(3, t431);
            c.w64(7, c.r64(0));
            c.w64(9, c.r64(3));
            return c.call(0x700086ed74, 0x700080d4ae, 0x700080d4a9);
        }
        492 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 494; continue; }
            blk = 493; continue;
        }
        493 => {
            c.w64(9, c.r64(0));
            let Some(t432) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d4b5); };
            c.w64(2, t432);
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d4b9); };
            c.w64(11, t433);
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d4bd); };
            c.w64(8, t434);
            blk = 91; continue;
        }
        494 => {
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080d4c6); };
            c.w64(0, t435);
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d4ca); };
            c.w64(2, t436);
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080d4ce); };
            c.w64(7, t437);
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d4d2); };
            c.w64(3, t438);
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080d4d6); };
            c.w64(1, t439);
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d4da); };
            c.w64(6, t440);
            blk = 446; continue;
        }
        495 => {
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080d4e3); };
            c.w64(7, t441);
            blk = 446; continue;
        }
        496 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d4ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080d4f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080d4f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700080d4f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x700080d4fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700080d500); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700080d50c, 0x700080d507);
        }
        497 => {
            c.w64(12, c.r64(0));
            let Some(t442) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d50f); };
            c.w64(2, t442);
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d513); };
            c.w64(11, t443);
            let Some(t444) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d517); };
            c.w64(9, t444);
            let Some(t445) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d51b); };
            c.w64(1, t445);
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d51f); };
            c.w64(8, t446);
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d523); };
            c.w64(7, t447);
            blk = 98; continue;
        }
        498 => {
            return RegfileExit::fault(0x700080d52c);
        }
        499 => {
            let Some(t448) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d52d); };
            c.w64(3, t448);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080d53d, 0x700080d538);
        }
        500 => {
            return RegfileExit::fault(0x700080d53d);
        }
        501 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d53e); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080d542); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700080d546); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080d54a); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700080d556, 0x700080d551);
        }
        502 => {
            c.w64(1, c.r64(0));
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d559); };
            c.w64(2, t449);
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d55d); };
            c.w64(3, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d561); };
            c.w64(6, t451);
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d565); };
            c.w64(9, t452);
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d569); };
            c.w64(7, t453);
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d56d); };
            c.w64(12, t454);
            blk = 129; continue;
        }
        503 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t455) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d578); };
            c.w64(3, t455);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700080d589, 0x700080d584);
        }
        504 => {
            return RegfileExit::fault(0x700080d589);
        }
        505 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d58a); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 521; continue; }
            blk = 506; continue;
        }
        506 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 508; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            blk = 517; continue;
        }
        508 => {
            let Some(t456) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080d5aa); };
            c.w64(7, t456);
            let Some(t457) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080d5b1); };
            c.w64(9, t457);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t458) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080d5b8); };
            c.w64(15, t458);
            let Some(t459) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080d5bf); };
            c.cmp(t459, c.r64(12), 64);
            if c.cc_a() { blk = 511; continue; }
            blk = 509; continue;
        }
        509 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700080d5d1, 0x700080d5cc);
        }
        510 => {
            c.w64(7, c.r64(0));
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080d5d4); };
            c.w64(0, t460);
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d5d8); };
            c.w64(2, t461);
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d5dc); };
            c.w64(3, t462);
            let Some(t463) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080d5e0); };
            c.w64(1, t463);
            let Some(t464) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d5e4); };
            c.w64(6, t464);
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d5e8); };
            c.w64(11, t465);
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d5ec); };
            c.w64(8, t466);
            blk = 512; continue;
        }
        511 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x700080d5f2); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 512; continue;
        }
        512 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 516; continue; }
            blk = 513; continue;
        }
        513 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 515; continue; }
            blk = 514; continue;
        }
        514 => {
            let r467 = c.r64(10) | 0x1u64;
            c.w64(10, r467);
            c.logic(r467, 64);
            blk = 515; continue;
        }
        515 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 516; continue;
        }
        516 => {
            let Some(t468) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080d620); };
            c.w64(9, t468);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080d627); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080d62b); }
            blk = 517; continue;
        }
        517 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080d630); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080d631); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t469) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700080d634); };
            c.w64(3, t469);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700080d648, 0x700080d643);
        }
        518 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 520; continue; }
            blk = 519; continue;
        }
        519 => {
            c.w64(9, c.r64(0));
            let Some(t470) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d64f); };
            c.w64(2, t470);
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d653); };
            c.w64(3, t471);
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d657); };
            c.w64(6, t472);
            blk = 137; continue;
        }
        520 => {
            let Some(t473) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080d660); };
            c.w64(0, t473);
            let Some(t474) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d664); };
            c.w64(2, t474);
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080d668); };
            c.w64(7, t475);
            let Some(t476) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d66c); };
            c.w64(3, t476);
            let Some(t477) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080d670); };
            c.w64(1, t477);
            let Some(t478) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d674); };
            c.w64(6, t478);
            blk = 446; continue;
        }
        521 => {
            let Some(t479) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080d67d); };
            c.w64(7, t479);
            blk = 446; continue;
        }
        522 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d686); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080d68a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700080d68e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700080d692); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x700080d69e, 0x700080d699);
        }
        523 => {
            c.w64(8, c.r64(0));
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d6a1); };
            c.w64(2, t480);
            let Some(t481) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d6a5); };
            c.w64(11, t481);
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d6a9); };
            c.w64(9, t482);
            let Some(t483) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d6ad); };
            c.w64(1, t483);
            let Some(t484) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d6b1); };
            c.w64(7, t484);
            blk = 145; continue;
        }
        524 => {
            return RegfileExit::fault(0x700080d6ba);
        }
        525 => {
            let Some(t485) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d6bb); };
            c.w64(3, t485);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080d6cb, 0x700080d6c6);
        }
        526 => {
            return RegfileExit::fault(0x700080d6cb);
        }
        527 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d6cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080d6d0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700080d6d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080d6d8); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700080d6e4, 0x700080d6df);
        }
        528 => {
            c.w64(1, c.r64(0));
            let Some(t486) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d6e7); };
            c.w64(2, t486);
            let Some(t487) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d6eb); };
            c.w64(3, t487);
            let Some(t488) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d6ef); };
            c.w64(6, t488);
            let Some(t489) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d6f3); };
            c.w64(9, t489);
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d6f7); };
            c.w64(7, t490);
            let Some(t491) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d6fb); };
            c.w64(12, t491);
            blk = 175; continue;
        }
        529 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t492) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d706); };
            c.w64(3, t492);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700080d717, 0x700080d712);
        }
        530 => {
            return RegfileExit::fault(0x700080d717);
        }
        531 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d718); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 547; continue; }
            blk = 532; continue;
        }
        532 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 534; continue; }
            blk = 533; continue;
        }
        533 => {
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            blk = 543; continue;
        }
        534 => {
            let Some(t493) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080d738); };
            c.w64(7, t493);
            let Some(t494) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080d73f); };
            c.w64(9, t494);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t495) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080d746); };
            c.w64(15, t495);
            let Some(t496) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080d74d); };
            c.cmp(t496, c.r64(12), 64);
            if c.cc_a() { blk = 537; continue; }
            blk = 535; continue;
        }
        535 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700080d75f, 0x700080d75a);
        }
        536 => {
            c.w64(7, c.r64(0));
            let Some(t497) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080d762); };
            c.w64(0, t497);
            let Some(t498) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d766); };
            c.w64(2, t498);
            let Some(t499) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d76a); };
            c.w64(3, t499);
            let Some(t500) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080d76e); };
            c.w64(1, t500);
            let Some(t501) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d772); };
            c.w64(6, t501);
            let Some(t502) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d776); };
            c.w64(11, t502);
            let Some(t503) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d77a); };
            c.w64(8, t503);
            blk = 538; continue;
        }
        537 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x700080d780); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 538; continue;
        }
        538 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 542; continue; }
            blk = 539; continue;
        }
        539 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 541; continue; }
            blk = 540; continue;
        }
        540 => {
            let r504 = c.r64(10) | 0x1u64;
            c.w64(10, r504);
            c.logic(r504, 64);
            blk = 541; continue;
        }
        541 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 542; continue;
        }
        542 => {
            let Some(t505) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080d7ae); };
            c.w64(9, t505);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080d7b5); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080d7b9); }
            blk = 543; continue;
        }
        543 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080d7be); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080d7bf); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t506) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700080d7c2); };
            c.w64(3, t506);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700080d7d6, 0x700080d7d1);
        }
        544 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 546; continue; }
            blk = 545; continue;
        }
        545 => {
            c.w64(9, c.r64(0));
            let Some(t507) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d7dd); };
            c.w64(2, t507);
            let Some(t508) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d7e1); };
            c.w64(3, t508);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d7e5); };
            c.w64(6, t509);
            blk = 183; continue;
        }
        546 => {
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080d7ee); };
            c.w64(0, t510);
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d7f2); };
            c.w64(2, t511);
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080d7f6); };
            c.w64(7, t512);
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d7fa); };
            c.w64(3, t513);
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080d7fe); };
            c.w64(1, t514);
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d802); };
            c.w64(6, t515);
            blk = 446; continue;
        }
        547 => {
            let Some(t516) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080d80b); };
            c.w64(7, t516);
            blk = 446; continue;
        }
        548 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d814); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080d818); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700080d81c); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700080d820); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x700080d82c, 0x700080d827);
        }
        549 => {
            c.w64(8, c.r64(0));
            let Some(t517) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d82f); };
            c.w64(2, t517);
            let Some(t518) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d833); };
            c.w64(11, t518);
            let Some(t519) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d837); };
            c.w64(9, t519);
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d83b); };
            c.w64(1, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d83f); };
            c.w64(7, t521);
            blk = 191; continue;
        }
        550 => {
            return RegfileExit::fault(0x700080d848);
        }
        551 => {
            let Some(t522) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d849); };
            c.w64(3, t522);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080d859, 0x700080d854);
        }
        552 => {
            return RegfileExit::fault(0x700080d859);
        }
        553 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d85a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080d85e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080d862); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700080d866); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700080d86a); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700080d86e); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700080d87a, 0x700080d875);
        }
        554 => {
            c.w64(1, c.r64(0));
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d87d); };
            c.w64(2, t523);
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d881); };
            c.w64(11, t524);
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d885); };
            c.w64(9, t525);
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d889); };
            c.w64(12, t526);
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d88d); };
            c.w64(8, t527);
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d891); };
            c.w64(15, t528);
            blk = 220; continue;
        }
        555 => {
            return RegfileExit::fault(0x700080d89a);
        }
        556 => {
            let Some(t529) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d89b); };
            c.w64(3, t529);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080d8ab, 0x700080d8a6);
        }
        557 => {
            return RegfileExit::fault(0x700080d8ab);
        }
        558 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d8ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080d8b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080d8b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x700080d8b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x700080d8bc); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700080d8c0); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700080d8cc, 0x700080d8c7);
        }
        559 => {
            c.w64(1, c.r64(0));
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d8cf); };
            c.w64(2, t530);
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d8d3); };
            c.w64(11, t531);
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d8d7); };
            c.w64(9, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d8db); };
            c.w64(12, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d8df); };
            c.w64(8, t534);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d8e3); };
            c.w64(15, t535);
            blk = 248; continue;
        }
        560 => {
            return RegfileExit::fault(0x700080d8ec);
        }
        561 => {
            let Some(t536) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d8ed); };
            c.w64(3, t536);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080d8fd, 0x700080d8f8);
        }
        562 => {
            return RegfileExit::fault(0x700080d8fd);
        }
        563 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d8fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080d902); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700080d906); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080d90a); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x700080d916, 0x700080d911);
        }
        564 => {
            c.w64(1, c.r64(0));
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d919); };
            c.w64(2, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d91d); };
            c.w64(3, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d921); };
            c.w64(6, t539);
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080d925); };
            c.w64(9, t540);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080d929); };
            c.w64(7, t541);
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080d92d); };
            c.w64(12, t542);
            blk = 279; continue;
        }
        565 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t543) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080d938); };
            c.w64(3, t543);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700080d949, 0x700080d944);
        }
        566 => {
            return RegfileExit::fault(0x700080d949);
        }
        567 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080d94a); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 583; continue; }
            blk = 568; continue;
        }
        568 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 570; continue; }
            blk = 569; continue;
        }
        569 => {
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            blk = 579; continue;
        }
        570 => {
            let Some(t544) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080d96a); };
            c.w64(7, t544);
            let Some(t545) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700080d971); };
            c.w64(9, t545);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t546) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080d978); };
            c.w64(15, t546);
            let Some(t547) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080d97f); };
            c.cmp(t547, c.r64(12), 64);
            if c.cc_a() { blk = 573; continue; }
            blk = 571; continue;
        }
        571 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700080d991, 0x700080d98c);
        }
        572 => {
            c.w64(7, c.r64(0));
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080d994); };
            c.w64(0, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080d998); };
            c.w64(2, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080d99c); };
            c.w64(3, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080d9a0); };
            c.w64(1, t551);
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080d9a4); };
            c.w64(6, t552);
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080d9a8); };
            c.w64(11, t553);
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080d9ac); };
            c.w64(8, t554);
            blk = 574; continue;
        }
        573 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x700080d9b2); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 574; continue;
        }
        574 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 578; continue; }
            blk = 575; continue;
        }
        575 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 577; continue; }
            blk = 576; continue;
        }
        576 => {
            let r555 = c.r64(10) | 0x1u64;
            c.w64(10, r555);
            c.logic(r555, 64);
            blk = 577; continue;
        }
        577 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 578; continue;
        }
        578 => {
            let Some(t556) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080d9e0); };
            c.w64(9, t556);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080d9e7); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080d9eb); }
            blk = 579; continue;
        }
        579 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080d9f0); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080d9f1); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t557) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700080d9f4); };
            c.w64(3, t557);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(3));
            return c.call(0x700086ed74, 0x700080da08, 0x700080da03);
        }
        580 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 582; continue; }
            blk = 581; continue;
        }
        581 => {
            c.w64(9, c.r64(0));
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080da0f); };
            c.w64(2, t558);
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080da13); };
            c.w64(3, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080da17); };
            c.w64(6, t560);
            blk = 287; continue;
        }
        582 => {
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080da20); };
            c.w64(0, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080da24); };
            c.w64(2, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080da28); };
            c.w64(7, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080da2c); };
            c.w64(3, t564);
            let Some(t565) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080da30); };
            c.w64(1, t565);
            let Some(t566) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080da34); };
            c.w64(6, t566);
            blk = 446; continue;
        }
        583 => {
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080da3d); };
            c.w64(7, t567);
            blk = 446; continue;
        }
        584 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080da46); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700080da4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x700080da4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x700080da52); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x700080da5e, 0x700080da59);
        }
        585 => {
            c.w64(8, c.r64(0));
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080da61); };
            c.w64(2, t568);
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080da65); };
            c.w64(11, t569);
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080da69); };
            c.w64(9, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080da6d); };
            c.w64(1, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080da71); };
            c.w64(7, t572);
            blk = 295; continue;
        }
        586 => {
            return RegfileExit::fault(0x700080da7a);
        }
        587 => {
            let Some(t573) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080da7b); };
            c.w64(3, t573);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080da8b, 0x700080da86);
        }
        588 => {
            return RegfileExit::fault(0x700080da8b);
        }
        589 => {
            return RegfileExit::fault(0x700080da8c);
        }
        590 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080da8d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700080da91); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080da95); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080da99); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700080da9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x700080daa1); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700080daad, 0x700080daa8);
        }
        591 => {
            c.w64(1, c.r64(0));
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080dab0); };
            c.w64(2, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080dab4); };
            c.w64(11, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080dab8); };
            c.w64(9, t576);
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080dabc); };
            c.w64(12, t577);
            let Some(t578) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080dac0); };
            c.w64(8, t578);
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700080dac4); };
            c.w64(15, t579);
            blk = 326; continue;
        }
        592 => {
            return RegfileExit::fault(0x700080dacd);
        }
        593 => {
            let Some(t580) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080dace); };
            c.w64(3, t580);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080dade, 0x700080dad9);
        }
        594 => {
            return RegfileExit::fault(0x700080dade);
        }
        595 => {
            return RegfileExit::fault(0x700080dadf);
        }
        596 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080dae0); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(11)) { return RegfileExit::fault(0x700080dae4); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080dae8); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080daec); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700080daf0); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(15)) { return RegfileExit::fault(0x700080daf4); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700080db00, 0x700080dafb);
        }
        597 => {
            c.w64(1, c.r64(0));
            let Some(t581) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080db03); };
            c.w64(2, t581);
            let Some(t582) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080db07); };
            c.w64(11, t582);
            let Some(t583) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080db0b); };
            c.w64(9, t583);
            let Some(t584) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080db0f); };
            c.w64(12, t584);
            let Some(t585) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080db13); };
            c.w64(8, t585);
            let Some(t586) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700080db17); };
            c.w64(15, t586);
            blk = 358; continue;
        }
        598 => {
            return RegfileExit::fault(0x700080db20);
        }
        599 => {
            let Some(t587) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080db21); };
            c.w64(3, t587);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080db31, 0x700080db2c);
        }
        600 => {
            return RegfileExit::fault(0x700080db31);
        }
        601 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t588) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700080db34); };
            c.w64(3, t588);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(6));
            return c.call(0x700086ed74, 0x700080db45, 0x700080db40);
        }
        602 => {
            return RegfileExit::fault(0x700080db45);
        }
        603 => {
            let r589 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r589);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700080db48); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700080db4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080db4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080db52); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700080db56); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080db5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080db5f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700080db63); }
            let Some(t590) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700080db67); };
            c.w64(3, t590);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x700080db7b, 0x700080db76);
        }
        604 => {
            c.w64(1, c.r64(0));
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080db7e); };
            c.w64(0, t591);
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080db82); };
            c.w64(2, t592);
            let Some(t593) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080db86); };
            c.w64(11, t593);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700080db8a); }
            let Some(t594) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080db8f); };
            c.w64(9, t594);
            let Some(t595) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080db93); };
            c.w64(12, t595);
            let Some(t596) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080db97); };
            c.w64(8, t596);
            blk = 396; continue;
        }
        605 => {
            let Some(t597) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080dba0); };
            c.w64(3, t597);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080dbb0, 0x700080dbab);
        }
        606 => {
            return RegfileExit::fault(0x700080dbb0);
        }
        607 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080dbb1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080dbb5); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700080dbb9); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 446; continue; }
            blk = 608; continue;
        }
        608 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 610; continue; }
            blk = 609; continue;
        }
        609 => {
            c.w32(8, c.r64(11).wrapping_add(c.r64(11)));
            blk = 619; continue;
        }
        610 => {
            let Some(t598) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080dbda); };
            c.w64(8, t598);
            let Some(t599) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700080dbe1); };
            c.w64(9, t599);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t600) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080dbe8); };
            c.w64(15, t600);
            let Some(t601) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080dbef); };
            c.cmp(t601, c.r64(12), 64);
            if c.cc_a() { blk = 613; continue; }
            blk = 611; continue;
        }
        611 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700080dc01, 0x700080dbfc);
        }
        612 => {
            c.w64(8, c.r64(0));
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080dc04); };
            c.w64(0, t602);
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080dc08); };
            c.w64(2, t603);
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080dc0c); };
            c.w64(7, t604);
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080dc10); };
            c.w64(3, t605);
            let Some(t606) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080dc14); };
            c.w64(1, t606);
            let Some(t607) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080dc18); };
            c.w64(6, t607);
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080dc1c); };
            c.w64(11, t608);
            blk = 614; continue;
        }
        613 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700080dc22); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 614; continue;
        }
        614 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 618; continue; }
            blk = 615; continue;
        }
        615 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 617; continue; }
            blk = 616; continue;
        }
        616 => {
            let r609 = c.r64(10) | 0x1u64;
            c.w64(10, r609);
            c.logic(r609, 64);
            blk = 617; continue;
        }
        617 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 618; continue;
        }
        618 => {
            let Some(t610) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080dc50); };
            c.w64(9, t610);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080dc57); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080dc5b); }
            blk = 619; continue;
        }
        619 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080dc61); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080dc62); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t611) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700080dc66); };
            c.w64(3, t611);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700080dc7a, 0x700080dc75);
        }
        620 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 622; continue; }
            blk = 621; continue;
        }
        621 => {
            c.w64(9, c.r64(0));
            let Some(t612) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080dc81); };
            c.w64(0, t612);
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080dc85); };
            c.w64(2, t613);
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080dc89); };
            c.w64(11, t614);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700080dc8d); }
            blk = 407; continue;
        }
        622 => {
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080dc97); };
            c.w64(0, t615);
            let Some(t616) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080dc9b); };
            c.w64(2, t616);
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080dc9f); };
            c.w64(7, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080dca3); };
            c.w64(3, t618);
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080dca7); };
            c.w64(1, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080dcab); };
            c.w64(6, t620);
            blk = 446; continue;
        }
        623 => {
            return RegfileExit::fault(0x700080dcb4);
        }
        624 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t621) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700080dcb7); };
            c.w64(3, t621);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(6));
            return c.call(0x700086ed74, 0x700080dcc8, 0x700080dcc3);
        }
        625 => {
            return RegfileExit::fault(0x700080dcc8);
        }
        626 => {
            let r622 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r622);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700080dccb); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700080dccf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080dcd1); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700080dcd5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x700080dcda); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080dcde); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700080dce2); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700080dce6); }
            let Some(t623) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700080dcea); };
            c.w64(3, t623);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x700080dcfe, 0x700080dcf9);
        }
        627 => {
            c.w64(1, c.r64(0));
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080dd01); };
            c.w64(0, t624);
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080dd05); };
            c.w64(11, t625);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700080dd09); }
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080dd0e); };
            c.w64(9, t626);
            let Some(t627) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080dd12); };
            c.w64(2, t627);
            let Some(t628) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700080dd16); };
            c.w64(12, t628);
            let Some(t629) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700080dd1a); };
            c.w64(8, t629);
            blk = 435; continue;
        }
        628 => {
            let Some(t630) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700080dd23); };
            c.w64(3, t630);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700080dd33, 0x700080dd2e);
        }
        629 => {
            return RegfileExit::fault(0x700080dd33);
        }
        630 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x700080dd34); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x700080dd38); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700080dd3d); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 446; continue; }
            blk = 631; continue;
        }
        631 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 633; continue; }
            blk = 632; continue;
        }
        632 => {
            c.w32(8, c.r64(11).wrapping_add(c.r64(11)));
            blk = 642; continue;
        }
        633 => {
            let Some(t631) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700080dd5d); };
            c.w64(8, t631);
            let Some(t632) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700080dd64); };
            c.w64(9, t632);
            c.w64(12, c.r64(9).wrapping_add(0xc));
            let Some(t633) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700080dd6b); };
            c.w64(15, t633);
            let Some(t634) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700080dd72); };
            c.cmp(t634, c.r64(12), 64);
            if c.cc_a() { blk = 636; continue; }
            blk = 634; continue;
        }
        634 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700080dd84, 0x700080dd7f);
        }
        635 => {
            c.w64(8, c.r64(0));
            let Some(t635) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080dd87); };
            c.w64(0, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080dd8b); };
            c.w64(11, t636);
            let Some(t637) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080dd8f); };
            c.w64(2, t637);
            let Some(t638) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080dd93); };
            c.w64(7, t638);
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080dd97); };
            c.w64(3, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080dd9b); };
            c.w64(1, t640);
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080dd9f); };
            c.w64(6, t641);
            blk = 637; continue;
        }
        636 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700080dda5); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 637; continue;
        }
        637 => {
            let Some(t642) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080ddac); };
            c.w64(9, t642);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700080ddb3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 641; continue; }
            blk = 638; continue;
        }
        638 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 640; continue; }
            blk = 639; continue;
        }
        639 => {
            let r643 = c.r64(10) | 0x1u64;
            c.w64(10, r643);
            c.logic(r643, 64);
            blk = 640; continue;
        }
        640 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 641; continue;
        }
        641 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700080ddde); }
            blk = 642; continue;
        }
        642 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700080dde4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080dde5); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t644) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700080dde9); };
            c.w64(3, t644);
            c.w32(0, 0x2u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x700080ddfd, 0x700080ddf8);
        }
        643 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 645; continue; }
            blk = 644; continue;
        }
        644 => {
            c.w64(9, c.r64(0));
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080de04); };
            c.w64(0, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080de08); };
            c.w64(11, t646);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x700080de0c); }
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080de11); };
            c.w64(2, t647);
            blk = 447; continue;
        }
        645 => {
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080de1a); };
            c.w64(0, t648);
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080de1e); };
            c.w64(2, t649);
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080de22); };
            c.w64(7, t650);
            let Some(t651) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080de26); };
            c.w64(3, t651);
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080de2a); };
            c.w64(1, t652);
            let Some(t653) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700080de2e); };
            c.w64(6, t653);
            blk = 446; continue;
        }
        646 => {
            return RegfileExit::fault(0x700080de37);
        }
        647 => {
            c.w32(1, c.r32(1));
            let r654 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r654);
            c.w64(7, 0xfffffffffffc0000u64);
            let r655 = c.r64(7) & c.r64(1);
            c.w64(7, r655);
            c.logic(r655, 64);
            let Some(t656) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080de47); };
            c.logic(t656 & 0x2u64, 8);
            if c.cc_e() { blk = 445; continue; }
            blk = 648; continue;
        }
        648 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080de55); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080de5e, 0x700080de59);
        }
        649 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080de5e); }
            blk = 445; continue;
        }
        650 => {
            c.w32(1, c.r32(1));
            let r657 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r657);
            c.w64(7, 0xfffffffffffc0000u64);
            let r658 = c.r64(7) & c.r64(1);
            c.w64(7, r658);
            c.logic(r658, 64);
            let Some(t659) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080de73); };
            c.logic(t659 & 0x2u64, 8);
            if c.cc_e() { blk = 406; continue; }
            blk = 651; continue;
        }
        651 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080de81); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080de8a, 0x700080de85);
        }
        652 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080de8a); }
            blk = 406; continue;
        }
        653 => {
            c.w32(0, c.r32(0));
            let r660 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r660);
            c.w64(7, 0xfffffffffffc0000u64);
            let r661 = c.r64(7) & c.r64(0);
            c.w64(7, r661);
            c.logic(r661, 64);
            let Some(t662) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080de9f); };
            c.logic(t662 & 0x2u64, 8);
            if c.cc_e() { blk = 369; continue; }
            blk = 654; continue;
        }
        654 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080dead); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x700080deb6, 0x700080deb1);
        }
        655 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080deb6); }
            blk = 369; continue;
        }
        656 => {
            c.w32(1, c.r32(1));
            let r663 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r663);
            c.w64(7, 0xfffffffffffc0000u64);
            let r664 = c.r64(7) & c.r64(1);
            c.w64(7, r664);
            c.logic(r664, 64);
            let Some(t665) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080decb); };
            c.logic(t665 & 0x2u64, 8);
            if c.cc_e() { blk = 365; continue; }
            blk = 657; continue;
        }
        657 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080ded9); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080dee2, 0x700080dedd);
        }
        658 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080dee2); }
            blk = 365; continue;
        }
        659 => {
            c.w32(0, c.r32(0));
            let r666 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r666);
            c.w64(7, 0xfffffffffffc0000u64);
            let r667 = c.r64(7) & c.r64(0);
            c.w64(7, r667);
            c.logic(r667, 64);
            let Some(t668) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080def7); };
            c.logic(t668 & 0x2u64, 8);
            if c.cc_e() { blk = 337; continue; }
            blk = 660; continue;
        }
        660 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080df05); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x700080df0e, 0x700080df09);
        }
        661 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080df0e); }
            blk = 337; continue;
        }
        662 => {
            c.w32(1, c.r32(1));
            let r669 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r669);
            c.w64(7, 0xfffffffffffc0000u64);
            let r670 = c.r64(7) & c.r64(1);
            c.w64(7, r670);
            c.logic(r670, 64);
            let Some(t671) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080df23); };
            c.logic(t671 & 0x2u64, 8);
            if c.cc_e() { blk = 333; continue; }
            blk = 663; continue;
        }
        663 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080df31); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080df3a, 0x700080df35);
        }
        664 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080df3a); }
            blk = 333; continue;
        }
        665 => {
            c.w32(0, c.r32(0));
            let r672 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r672);
            c.w64(7, 0xfffffffffffc0000u64);
            let r673 = c.r64(7) & c.r64(0);
            c.w64(7, r673);
            c.logic(r673, 64);
            let Some(t674) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080df4f); };
            c.logic(t674 & 0x2u64, 8);
            if c.cc_e() { blk = 306; continue; }
            blk = 666; continue;
        }
        666 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080df5d); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x700080df66, 0x700080df61);
        }
        667 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080df66); }
            blk = 306; continue;
        }
        668 => {
            c.w32(8, c.r32(8));
            let r675 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r675);
            c.w64(1, 0xfffffffffffc0000u64);
            let r676 = c.r64(1) & c.r64(8);
            c.w64(1, r676);
            c.logic(r676, 64);
            let Some(t677) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080df7c); };
            c.logic(t677 & 0x2u64, 8);
            if c.cc_e() { blk = 302; continue; }
            blk = 669; continue;
        }
        669 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080df8a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080df8b); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080df98, 0x700080df93);
        }
        670 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080df98); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080df9a); }
            blk = 302; continue;
        }
        671 => {
            c.w32(1, c.r32(1));
            let r678 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r678);
            c.w64(7, 0xfffffffffffc0000u64);
            let r679 = c.r64(7) & c.r64(1);
            c.w64(7, r679);
            c.logic(r679, 64);
            let Some(t680) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080dfaf); };
            c.logic(t680 & 0x2u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 672; continue;
        }
        672 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080dfbd); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080dfc6, 0x700080dfc1);
        }
        673 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080dfc6); }
            blk = 286; continue;
        }
        674 => {
            c.w32(0, c.r32(0));
            let r681 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r681);
            c.w64(7, 0xfffffffffffc0000u64);
            let r682 = c.r64(7) & c.r64(0);
            c.w64(7, r682);
            c.logic(r682, 64);
            let Some(t683) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080dfdb); };
            c.logic(t683 & 0x2u64, 8);
            if c.cc_e() { blk = 259; continue; }
            blk = 675; continue;
        }
        675 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080dfe9); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x700080dff2, 0x700080dfed);
        }
        676 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080dff2); }
            blk = 259; continue;
        }
        677 => {
            c.w32(1, c.r32(1));
            let r684 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r684);
            c.w64(7, 0xfffffffffffc0000u64);
            let r685 = c.r64(7) & c.r64(1);
            c.w64(7, r685);
            c.logic(r685, 64);
            let Some(t686) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e007); };
            c.logic(t686 & 0x2u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 678; continue;
        }
        678 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e015); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080e01e, 0x700080e019);
        }
        679 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e01e); }
            blk = 255; continue;
        }
        680 => {
            c.w32(0, c.r32(0));
            let r687 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r687);
            c.w64(7, 0xfffffffffffc0000u64);
            let r688 = c.r64(7) & c.r64(0);
            c.w64(7, r688);
            c.logic(r688, 64);
            let Some(t689) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e033); };
            c.logic(t689 & 0x2u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 681; continue;
        }
        681 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e041); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x700080e04a, 0x700080e045);
        }
        682 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e04a); }
            blk = 231; continue;
        }
        683 => {
            c.w32(1, c.r32(1));
            let r690 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r690);
            c.w64(7, 0xfffffffffffc0000u64);
            let r691 = c.r64(7) & c.r64(1);
            c.w64(7, r691);
            c.logic(r691, 64);
            let Some(t692) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e05f); };
            c.logic(t692 & 0x2u64, 8);
            if c.cc_e() { blk = 227; continue; }
            blk = 684; continue;
        }
        684 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e06d); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080e076, 0x700080e071);
        }
        685 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e076); }
            blk = 227; continue;
        }
        686 => {
            c.w32(0, c.r32(0));
            let r693 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r693);
            c.w64(7, 0xfffffffffffc0000u64);
            let r694 = c.r64(7) & c.r64(0);
            c.w64(7, r694);
            c.logic(r694, 64);
            let Some(t695) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e08b); };
            c.logic(t695 & 0x2u64, 8);
            if c.cc_e() { blk = 202; continue; }
            blk = 687; continue;
        }
        687 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e099); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x700080e0a2, 0x700080e09d);
        }
        688 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e0a2); }
            blk = 202; continue;
        }
        689 => {
            c.w32(8, c.r32(8));
            let r696 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r696);
            c.w64(1, 0xfffffffffffc0000u64);
            let r697 = c.r64(1) & c.r64(8);
            c.w64(1, r697);
            c.logic(r697, 64);
            let Some(t698) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e0b8); };
            c.logic(t698 & 0x2u64, 8);
            if c.cc_e() { blk = 198; continue; }
            blk = 690; continue;
        }
        690 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e0c6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080e0c7); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080e0d4, 0x700080e0cf);
        }
        691 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080e0d4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e0d6); }
            blk = 198; continue;
        }
        692 => {
            c.w32(1, c.r32(1));
            let r699 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r699);
            c.w64(7, 0xfffffffffffc0000u64);
            let r700 = c.r64(7) & c.r64(1);
            c.w64(7, r700);
            c.logic(r700, 64);
            let Some(t701) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e0eb); };
            c.logic(t701 & 0x2u64, 8);
            if c.cc_e() { blk = 182; continue; }
            blk = 693; continue;
        }
        693 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e0f9); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080e102, 0x700080e0fd);
        }
        694 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e102); }
            blk = 182; continue;
        }
        695 => {
            c.w32(8, c.r32(8));
            let r702 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r702);
            c.w64(1, 0xfffffffffffc0000u64);
            let r703 = c.r64(1) & c.r64(8);
            c.w64(1, r703);
            c.logic(r703, 64);
            let Some(t704) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e118); };
            c.logic(t704 & 0x2u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 696; continue;
        }
        696 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e126); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080e127); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080e134, 0x700080e12f);
        }
        697 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080e134); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e136); }
            blk = 152; continue;
        }
        698 => {
            c.w32(1, c.r32(1));
            let r705 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r705);
            c.w64(7, 0xfffffffffffc0000u64);
            let r706 = c.r64(7) & c.r64(1);
            c.w64(7, r706);
            c.logic(r706, 64);
            let Some(t707) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e14b); };
            c.logic(t707 & 0x2u64, 8);
            if c.cc_e() { blk = 136; continue; }
            blk = 699; continue;
        }
        699 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e159); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080e162, 0x700080e15d);
        }
        700 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e162); }
            blk = 136; continue;
        }
        701 => {
            c.w32(12, c.r32(12));
            let r708 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r708);
            c.w64(1, 0xfffffffffffc0000u64);
            let r709 = c.r64(1) & c.r64(12);
            c.w64(1, r709);
            c.logic(r709, 64);
            let Some(t710) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e178); };
            c.logic(t710 & 0x2u64, 8);
            if c.cc_e() { blk = 105; continue; }
            blk = 702; continue;
        }
        702 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e186); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080e187); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700080e194, 0x700080e18f);
        }
        703 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080e194); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e196); }
            blk = 105; continue;
        }
        704 => {
            c.w32(1, c.r32(1));
            let r711 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r711);
            c.w64(7, 0xfffffffffffc0000u64);
            let r712 = c.r64(7) & c.r64(1);
            c.w64(7, r712);
            c.logic(r712, 64);
            let Some(t713) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e1ab); };
            c.logic(t713 & 0x2u64, 8);
            if c.cc_e() { blk = 90; continue; }
            blk = 705; continue;
        }
        705 => {
            c.w64(9, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080e1b9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700080e1ba); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700080e1c7, 0x700080e1c2);
        }
        706 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700080e1c7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700080e1c9); }
            blk = 90; continue;
        }
        707 => {
            return RegfileExit::jump(0x700080e1d0);
        }
            _ => unreachable!(),
        }
    }
}
