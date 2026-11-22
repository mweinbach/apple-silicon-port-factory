#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700080e1d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700080e1d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700080e1d8); }
            let r0 = c.sub(c.r64(4), 0x40u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e1de); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080e1e5); };
            c.w32(9, t3);
            let r4 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r4);
            let r5 = c.r32(9) & 0x3fu64;
            c.w32(9, r5);
            c.logic(r5, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x700080e1f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x700080e1f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700080e1fa); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(1)) { return RegfileExit::fault(0x700080e1fe); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x700080e202); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700080e206); }
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
            if c.cc_e() { blk = 931; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 979; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 1029; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 809; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 854; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 1029; continue;
        }
        10 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 718; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 764; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 1029; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 626; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 672; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 1029; continue;
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
            if c.cc_e() { blk = 534; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 580; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 1029; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 463; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 486; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 1029; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 389; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 440; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 1029; continue;
        }
        29 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 349; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 369; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 1029; continue;
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
            if c.cc_e() { blk = 307; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 328; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 1029; continue;
        }
        38 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 265; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 286; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 1029; continue;
        }
        41 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 225; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 1029; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 125; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 1029; continue;
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
            if c.cc_e() { blk = 109; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 157; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 1029; continue;
        }
        53 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 191; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 208; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 1029; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 1029; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 1029; continue;
        }
        63 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e3fc); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e403); };
            c.w32(11, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e407); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 64; continue;
        }
        64 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 70; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t10) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e419); };
            c.w32(11, t10);
            let r11 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r11);
            let Some(t12) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e420); };
            c.cmp(t12, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 66; continue;
        }
        66 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e42c); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 67; continue;
        }
        67 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 68; continue;
        }
        68 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r13 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r13);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 71; continue; }
            blk = 69; continue;
        }
        69 => {
            blk = 984; continue;
        }
        70 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 71; continue;
        }
        71 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 984; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t14) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e47e); };
            c.cmp(t14, 0x834u64, 16);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e487); };
            c.w32(8, t15);
            blk = 75; continue;
        }
        74 => {
            let Some(t16) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080e48d); };
            c.w32(8, t16);
            blk = 75; continue;
        }
        75 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e4a0); };
            c.w32(7, t17);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1030; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700080e4b3); }
            blk = 1026; continue;
        }
        78 => {
            let Some(t18) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e4bd); };
            c.w32(9, t18);
            let r19 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r19);
            let Some(t20) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e4c4); };
            c.w32(11, t20);
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e4c8); };
            c.cmp(t21, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 79; continue;
        }
        79 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t22) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e4da); };
            c.w32(11, t22);
            let r23 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r23);
            let Some(t24) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e4e1); };
            c.cmp(t24, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 81; continue;
        }
        81 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e4ed); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 82; continue;
        }
        82 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t25) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080e510); };
            c.w64(12, t25);
            let r26 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r26);
            let Some(t27) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080e51a); };
            c.w64(15, t27);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            blk = 984; continue;
        }
        85 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 86; continue;
        }
        86 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 984; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t28) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e539); };
            c.cmp(t28, 0x834u64, 16);
            if c.cc_e() { blk = 89; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e542); };
            c.w32(8, t29);
            blk = 90; continue;
        }
        89 => {
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080e548); };
            c.w32(8, t30);
            blk = 90; continue;
        }
        90 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t31) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e55b); };
            c.w32(7, t31);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1031; continue; }
            blk = 92; continue;
        }
        92 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700080e56e); }
            blk = 1026; continue;
        }
        93 => {
            let Some(t32) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e578); };
            c.w32(9, t32);
            let r33 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r33);
            let Some(t34) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e57f); };
            c.w32(11, t34);
            let Some(t35) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e583); };
            c.cmp(t35, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 94; continue;
        }
        94 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t36) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e595); };
            c.w32(11, t36);
            let r37 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r37);
            let Some(t38) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e59c); };
            c.cmp(t38, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 96; continue;
        }
        96 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e5a8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 97; continue;
        }
        97 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t39) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080e5cb); };
            c.w64(12, t39);
            let r40 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r40);
            let Some(t41) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080e5d5); };
            c.w64(15, t41);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 101; continue; }
            blk = 99; continue;
        }
        99 => {
            blk = 984; continue;
        }
        100 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 101; continue;
        }
        101 => {
            let Some(t42) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e5ec); };
            c.cmp(t42, 0x834u64, 16);
            if c.cc_e() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t43) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e5f5); };
            c.w32(8, t43);
            blk = 104; continue;
        }
        103 => {
            let Some(t44) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080e5fb); };
            c.w32(8, t44);
            blk = 104; continue;
        }
        104 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t45) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e60e); };
            c.w32(7, t45);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1032; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700080e629); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r46 = c.r64(8) & c.r64(9);
            c.w64(8, r46);
            c.logic(r46, 64);
            let Some(t47) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e63b); };
            c.logic(t47 & 0x4u64, 8);
            if c.cc_ne() { blk = 1122; continue; }
            blk = 108; continue;
        }
        108 => {
            blk = 1026; continue;
        }
        109 => {
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e64b); };
            c.w32(9, t48);
            let r49 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r49);
            let Some(t50) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e652); };
            c.w32(11, t50);
            let Some(t51) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e656); };
            c.cmp(t51, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 110; continue;
        }
        110 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e668); };
            c.w32(11, t52);
            let r53 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r53);
            let Some(t54) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e66f); };
            c.cmp(t54, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 112; continue;
        }
        112 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e67b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 113; continue;
        }
        113 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t55) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080e69e); };
            c.w64(12, t55);
            let r56 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r56);
            let Some(t57) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080e6a8); };
            c.w64(15, t57);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 117; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 984; continue;
        }
        116 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 117; continue;
        }
        117 => {
            let Some(t58) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e6bf); };
            c.cmp(t58, 0x834u64, 16);
            if c.cc_e() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e6c8); };
            c.w32(8, t59);
            blk = 120; continue;
        }
        119 => {
            let Some(t60) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080e6ce); };
            c.w32(8, t60);
            blk = 120; continue;
        }
        120 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t61) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e6e1); };
            c.w32(7, t61);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1033; continue; }
            blk = 122; continue;
        }
        122 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700080e6fc); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r62 = c.r64(8) & c.r64(9);
            c.w64(8, r62);
            c.logic(r62, 64);
            let Some(t63) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e70e); };
            c.logic(t63 & 0x4u64, 8);
            if c.cc_ne() { blk = 1119; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 1026; continue;
        }
        125 => {
            let Some(t64) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e71e); };
            c.w32(9, t64);
            let r65 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r65);
            let Some(t66) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e725); };
            c.w32(11, t66);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e729); };
            c.cmp(t67, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 126; continue;
        }
        126 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 132; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t68) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e73b); };
            c.w32(11, t68);
            let r69 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r69);
            let Some(t70) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e742); };
            c.cmp(t70, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 128; continue;
        }
        128 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e74e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 129; continue;
        }
        129 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t71) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080e771); };
            c.w64(12, t71);
            let r72 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r72);
            let Some(t73) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080e77b); };
            c.w64(15, t73);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 133; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 984; continue;
        }
        132 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 133; continue;
        }
        133 => {
            let Some(t74) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e792); };
            c.cmp(t74, 0x834u64, 16);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t75) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e79b); };
            c.w32(8, t75);
            blk = 136; continue;
        }
        135 => {
            let Some(t76) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080e7a1); };
            c.w32(8, t76);
            blk = 136; continue;
        }
        136 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t77) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e7b4); };
            c.w32(7, t77);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1034; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700080e7cf); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r78 = c.r64(8) & c.r64(9);
            c.w64(8, r78);
            c.logic(r78, 64);
            let Some(t79) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e7e1); };
            c.logic(t79 & 0x4u64, 8);
            if c.cc_ne() { blk = 1116; continue; }
            blk = 140; continue;
        }
        140 => {
            blk = 1026; continue;
        }
        141 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e7f1); };
            c.w32(9, t80);
            let r81 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r81);
            let Some(t82) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e7f8); };
            c.w32(11, t82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e7fc); };
            c.cmp(t83, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 142; continue;
        }
        142 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t84) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e80e); };
            c.w32(11, t84);
            let r85 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r85);
            let Some(t86) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e815); };
            c.cmp(t86, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 144; continue;
        }
        144 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e821); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 145; continue;
        }
        145 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t87) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080e844); };
            c.w64(12, t87);
            let r88 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r88);
            let Some(t89) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080e84e); };
            c.w64(15, t89);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 149; continue; }
            blk = 147; continue;
        }
        147 => {
            blk = 984; continue;
        }
        148 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 149; continue;
        }
        149 => {
            let Some(t90) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e865); };
            c.cmp(t90, 0x834u64, 16);
            if c.cc_e() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t91) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e86e); };
            c.w32(8, t91);
            blk = 152; continue;
        }
        151 => {
            let Some(t92) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080e874); };
            c.w32(8, t92);
            blk = 152; continue;
        }
        152 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t93) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e887); };
            c.w32(7, t93);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1035; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(9).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x700080e8a2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r94 = c.r64(8) & c.r64(9);
            c.w64(8, r94);
            c.logic(r94, 64);
            let Some(t95) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e8b4); };
            c.logic(t95 & 0x4u64, 8);
            if c.cc_ne() { blk = 1113; continue; }
            blk = 156; continue;
        }
        156 => {
            blk = 1026; continue;
        }
        157 => {
            let Some(t96) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e8c4); };
            c.w32(9, t96);
            let r97 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r97);
            let Some(t98) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e8cb); };
            c.w32(11, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e8cf); };
            c.cmp(t99, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 158; continue;
        }
        158 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t100) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e8e1); };
            c.w32(11, t100);
            let r101 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r101);
            let Some(t102) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e8e8); };
            c.cmp(t102, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 160; continue;
        }
        160 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e8f4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 161; continue;
        }
        161 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t103) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080e917); };
            c.w64(12, t103);
            let r104 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r104);
            let Some(t105) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080e921); };
            c.w64(15, t105);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 165; continue; }
            blk = 163; continue;
        }
        163 => {
            blk = 984; continue;
        }
        164 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 165; continue;
        }
        165 => {
            let Some(t106) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e938); };
            c.cmp(t106, 0x834u64, 16);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t107) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e941); };
            c.w32(8, t107);
            blk = 168; continue;
        }
        167 => {
            let Some(t108) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080e947); };
            c.w32(8, t108);
            blk = 168; continue;
        }
        168 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t109) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080e95a); };
            c.w32(8, t109);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 1036; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t110) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e975); };
            c.w32(11, t110);
            let Some(t111) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080e97a); };
            c.cmp(t111, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 171; continue;
        }
        171 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700080e987); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r112 = c.r64(7) & c.r64(9);
            c.w64(7, r112);
            c.logic(r112, 64);
            let Some(t113) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080e999); };
            c.logic(t113 & 0x4u64, 8);
            if c.cc_ne() { blk = 1110; continue; }
            blk = 173; continue;
        }
        173 => {
            blk = 1026; continue;
        }
        174 => {
            let Some(t114) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e9a8); };
            c.w32(9, t114);
            let r115 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r115);
            let Some(t116) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e9af); };
            c.w32(11, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700080e9b3); };
            c.cmp(t117, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 175; continue;
        }
        175 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t118) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080e9c5); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080e9cc); };
            c.cmp(t120, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 177; continue;
        }
        177 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080e9d8); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 178; continue;
        }
        178 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t121) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080e9fb); };
            c.w64(12, t121);
            let r122 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r122);
            let Some(t123) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ea05); };
            c.w64(15, t123);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 182; continue; }
            blk = 180; continue;
        }
        180 => {
            blk = 984; continue;
        }
        181 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 182; continue;
        }
        182 => {
            let Some(t124) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ea1c); };
            c.cmp(t124, 0x834u64, 16);
            if c.cc_e() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t125) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080ea25); };
            c.w32(8, t125);
            blk = 185; continue;
        }
        184 => {
            let Some(t126) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080ea2b); };
            c.w32(8, t126);
            blk = 185; continue;
        }
        185 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t127) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080ea3e); };
            c.w32(8, t127);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 1037; continue; }
            blk = 187; continue;
        }
        187 => {
            c.w64(8, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t128) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ea59); };
            c.w32(11, t128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700080ea5e); };
            c.cmp(t129, c.r32(11), 32);
            if c.cc_e() { blk = 984; continue; }
            blk = 188; continue;
        }
        188 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x700080ea6b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r130 = c.r64(7) & c.r64(9);
            c.w64(7, r130);
            c.logic(r130, 64);
            let Some(t131) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700080ea7d); };
            c.logic(t131 & 0x4u64, 8);
            if c.cc_ne() { blk = 1107; continue; }
            blk = 190; continue;
        }
        190 => {
            blk = 1026; continue;
        }
        191 => {
            let Some(t132) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ea8c); };
            c.w32(9, t132);
            let r133 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r133);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 197; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t134) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ea98); };
            c.w32(11, t134);
            let r135 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r135);
            let Some(t136) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ea9f); };
            c.cmp(t136, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 193; continue;
        }
        193 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080eaab); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 194; continue;
        }
        194 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 195; continue;
        }
        195 => {
            let Some(t137) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080eace); };
            c.w64(12, t137);
            let r138 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r138);
            let Some(t139) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ead8); };
            c.w64(15, t139);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 198; continue; }
            blk = 196; continue;
        }
        196 => {
            blk = 984; continue;
        }
        197 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 198; continue;
        }
        198 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 201; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t140) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080eaf3); };
            c.w32(12, t140);
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080eaf7); };
            c.cmp(t141, c.r32(12), 32);
            if c.cc_ne() { blk = 984; continue; }
            blk = 200; continue;
        }
        200 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080eb04); }
            blk = 202; continue;
        }
        201 => {
            c.w64(12, c.r64(0));
            let r142 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r142);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 202; continue;
        }
        202 => {
            let Some(t143) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080eb1a); };
            c.cmp(t143, 0x834u64, 16);
            if c.cc_e() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t144) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080eb23); };
            c.w32(8, t144);
            blk = 205; continue;
        }
        204 => {
            let Some(t145) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080eb29); };
            c.w32(8, t145);
            blk = 205; continue;
        }
        205 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 206; continue;
        }
        206 => {
            let Some(t146) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080eb3c); };
            c.w32(7, t146);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1038; continue; }
            blk = 207; continue;
        }
        207 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700080eb59); }
            blk = 1026; continue;
        }
        208 => {
            let Some(t147) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080eb65); };
            c.w32(9, t147);
            let r148 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r148);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 214; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t149) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080eb71); };
            c.w32(11, t149);
            let r150 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r150);
            let Some(t151) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080eb78); };
            c.cmp(t151, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 210; continue;
        }
        210 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080eb84); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 211; continue;
        }
        211 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t152) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080eba7); };
            c.w64(12, t152);
            let r153 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r153);
            let Some(t154) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ebb1); };
            c.w64(15, t154);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 215; continue; }
            blk = 213; continue;
        }
        213 => {
            blk = 984; continue;
        }
        214 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 215; continue;
        }
        215 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 218; continue; }
            blk = 216; continue;
        }
        216 => {
            let Some(t155) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ebcc); };
            c.w32(12, t155);
            let Some(t156) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080ebd0); };
            c.cmp(t156, c.r32(12), 32);
            if c.cc_ne() { blk = 984; continue; }
            blk = 217; continue;
        }
        217 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ebdd); }
            blk = 219; continue;
        }
        218 => {
            c.w64(12, c.r64(0));
            let r157 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r157);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 219; continue;
        }
        219 => {
            let Some(t158) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ebf3); };
            c.cmp(t158, 0x834u64, 16);
            if c.cc_e() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            let Some(t159) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080ebfc); };
            c.w32(8, t159);
            blk = 222; continue;
        }
        221 => {
            let Some(t160) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080ec02); };
            c.w32(8, t160);
            blk = 222; continue;
        }
        222 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 984; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t161) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080ec15); };
            c.w32(7, t161);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ae() { blk = 1039; continue; }
            blk = 224; continue;
        }
        224 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700080ec32); }
            blk = 1026; continue;
        }
        225 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 226; continue;
        }
        226 => {
            let Some(t162) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ec43); };
            c.w32(8, t162);
            let r163 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r163);
            let Some(t164) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ec4a); };
            c.cmp(t164, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 227; continue;
        }
        227 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ec56); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 228; continue;
        }
        228 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t165) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080ec79); };
            c.w64(9, t165);
            let r166 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r166);
            let Some(t167) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ec83); };
            c.w64(11, t167);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            blk = 984; continue;
        }
        231 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 232; continue;
        }
        232 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700080ec9a); }
            c.w64(9, c.r64(0));
            blk = 235; continue;
        }
        233 => {
            return c.call(0x70008a1014, 0x700080eca8, 0x700080eca3);
        }
        234 => {
            c.w64(9, c.r64(0));
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080ecab); };
            c.w64(0, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080ecaf); };
            c.w64(6, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080ecb3); };
            c.w64(2, t170);
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080ecb7); };
            c.w64(1, t171);
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080ecbb); };
            c.w64(8, t172);
            blk = 235; continue;
        }
        235 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 240; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t173) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ecc5); };
            c.w32(11, t173);
            let r174 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080eccc); };
            c.cmp(t175, c.r32(11), 32);
            if c.cc_e() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t176) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ecd5); };
            c.cmp(t176, 0x43u64, 16);
            if c.cc_ne() { blk = 233; continue; }
            blk = 238; continue;
        }
        238 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ecdd); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1105; continue; }
            blk = 239; continue;
        }
        239 => {
            c.w32(7, c.r32(7));
            blk = 241; continue;
        }
        240 => {
            let r177 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r177);
            c.w64(7, c.r64(9));
            blk = 241; continue;
        }
        241 => {
            let Some(t178) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080ecfc); };
            c.w32(9, t178);
            let r179 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r179);
            let Some(t180) = c.ld8(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080ed03); };
            c.logic(t180 & 0x4u64, 8);
            if c.cc_ne() { blk = 1040; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t181) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080ed0e); };
            c.cmp(t181, c.r64(8), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t182) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080ed18); };
            c.w64(9, t182);
            let Some(t183) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080ed1c); };
            c.w32(11, t183);
            let r184 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r184);
            if !c.st8(c.r64(8).wrapping_add(c.r64(9)), c.r8(7)) { return RegfileExit::fault(0x700080ed23); }
            blk = 1026; continue;
        }
        244 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 252; continue; }
            blk = 245; continue;
        }
        245 => {
            c.w64(8, c.r64(1));
            let Some(t185) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ed34); };
            c.w32(9, t185);
            let r186 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r186);
            let Some(t187) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ed3b); };
            c.cmp(t187, 0x42u64, 16);
            if c.cc_e() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        247 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ed4b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            if c.cc_e() { blk = 250; continue; }
            blk = 249; continue;
        }
        249 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        250 => {
            let Some(t188) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080ed6f); };
            c.w64(11, t188);
            let r189 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r189);
            let Some(t190) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ed79); };
            c.w64(12, t190);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 253; continue; }
            blk = 251; continue;
        }
        251 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        252 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 253; continue;
        }
        253 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700080ed99); }
            c.w64(11, c.r64(0));
            blk = 256; continue;
        }
        254 => {
            return c.call(0x70008a1014, 0x700080eda7, 0x700080eda2);
        }
        255 => {
            c.w64(11, c.r64(0));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080edaa); };
            c.w64(0, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080edae); };
            c.w64(6, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080edb2); };
            c.w64(2, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080edb6); };
            c.w64(8, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080edba); };
            c.w64(9, t195);
            blk = 256; continue;
        }
        256 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 261; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t196) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080edc4); };
            c.w32(12, t196);
            let r197 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r197);
            let Some(t198) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080edcb); };
            c.cmp(t198, c.r32(12), 32);
            if c.cc_e() { blk = 259; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t199) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080edd4); };
            c.cmp(t199, 0x43u64, 16);
            if c.cc_ne() { blk = 254; continue; }
            blk = 259; continue;
        }
        259 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080eddd); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1103; continue; }
            blk = 260; continue;
        }
        260 => {
            c.w32(7, c.r32(7));
            blk = 262; continue;
        }
        261 => {
            let r200 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r200);
            c.w64(7, c.r64(11));
            blk = 262; continue;
        }
        262 => {
            let Some(t201) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080edfc); };
            c.w32(11, t201);
            let r202 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r202);
            let Some(t203) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080ee03); };
            c.logic(t203 & 0x4u64, 8);
            if c.cc_ne() { blk = 1041; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t204) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080ee0e); };
            c.cmp(t204, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t205) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080ee18); };
            c.w64(8, t205);
            let Some(t206) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080ee1c); };
            c.w32(11, t206);
            let r207 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r207);
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), c.r8(7)) { return RegfileExit::fault(0x700080ee23); }
            blk = 1026; continue;
        }
        265 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 273; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w64(8, c.r64(1));
            let Some(t208) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ee34); };
            c.w32(9, t208);
            let r209 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r209);
            let Some(t210) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ee3b); };
            c.cmp(t210, 0x42u64, 16);
            if c.cc_e() { blk = 268; continue; }
            blk = 267; continue;
        }
        267 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        268 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ee4b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 270; continue; }
            blk = 269; continue;
        }
        269 => {
            if c.cc_e() { blk = 271; continue; }
            blk = 270; continue;
        }
        270 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        271 => {
            let Some(t211) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080ee6f); };
            c.w64(11, t211);
            let r212 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r212);
            let Some(t213) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ee79); };
            c.w64(12, t213);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 274; continue; }
            blk = 272; continue;
        }
        272 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        273 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 274; continue;
        }
        274 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700080ee99); }
            c.w64(11, c.r64(0));
            blk = 277; continue;
        }
        275 => {
            return c.call(0x70008a1014, 0x700080eea7, 0x700080eea2);
        }
        276 => {
            c.w64(11, c.r64(0));
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080eeaa); };
            c.w64(0, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080eeae); };
            c.w64(6, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080eeb2); };
            c.w64(2, t216);
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080eeb6); };
            c.w64(8, t217);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080eeba); };
            c.w64(9, t218);
            blk = 277; continue;
        }
        277 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 282; continue; }
            blk = 278; continue;
        }
        278 => {
            let Some(t219) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080eec4); };
            c.w32(12, t219);
            let r220 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r220);
            let Some(t221) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080eecb); };
            c.cmp(t221, c.r32(12), 32);
            if c.cc_e() { blk = 280; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t222) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080eed4); };
            c.cmp(t222, 0x43u64, 16);
            if c.cc_ne() { blk = 275; continue; }
            blk = 280; continue;
        }
        280 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080eedd); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1101; continue; }
            blk = 281; continue;
        }
        281 => {
            c.w32(7, c.r32(7));
            blk = 283; continue;
        }
        282 => {
            let r223 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r223);
            c.w64(7, c.r64(11));
            blk = 283; continue;
        }
        283 => {
            let Some(t224) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080eefc); };
            c.w32(11, t224);
            let r225 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r225);
            let Some(t226) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080ef03); };
            c.logic(t226 & 0x4u64, 8);
            if c.cc_ne() { blk = 1042; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t227) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080ef0e); };
            c.cmp(t227, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t228) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080ef18); };
            c.w64(8, t228);
            let Some(t229) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080ef1c); };
            c.w32(11, t229);
            let r230 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r230);
            if !c.st16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x700080ef23); }
            blk = 1026; continue;
        }
        286 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 294; continue; }
            blk = 287; continue;
        }
        287 => {
            c.w64(8, c.r64(1));
            let Some(t231) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ef35); };
            c.w32(9, t231);
            let r232 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r232);
            let Some(t233) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ef3c); };
            c.cmp(t233, 0x42u64, 16);
            if c.cc_e() { blk = 289; continue; }
            blk = 288; continue;
        }
        288 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        289 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ef4c); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            if c.cc_e() { blk = 292; continue; }
            blk = 291; continue;
        }
        291 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        292 => {
            let Some(t234) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080ef70); };
            c.w64(11, t234);
            let r235 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r235);
            let Some(t236) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ef7a); };
            c.w64(12, t236);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 295; continue; }
            blk = 293; continue;
        }
        293 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        294 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 295; continue;
        }
        295 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700080ef9a); }
            c.w64(11, c.r64(0));
            blk = 298; continue;
        }
        296 => {
            return c.call(0x70008a1014, 0x700080efa8, 0x700080efa3);
        }
        297 => {
            c.w64(11, c.r64(0));
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080efab); };
            c.w64(0, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080efaf); };
            c.w64(6, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080efb3); };
            c.w64(2, t239);
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080efb7); };
            c.w64(8, t240);
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080efbb); };
            c.w64(9, t241);
            blk = 298; continue;
        }
        298 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t242) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080efc5); };
            c.w32(12, t242);
            let r243 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r243);
            let Some(t244) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080efcc); };
            c.cmp(t244, c.r32(12), 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t245) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080efd5); };
            c.cmp(t245, 0x43u64, 16);
            if c.cc_ne() { blk = 296; continue; }
            blk = 301; continue;
        }
        301 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080efde); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1099; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w32(7, c.r32(7));
            blk = 304; continue;
        }
        303 => {
            let r246 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r246);
            c.w64(7, c.r64(11));
            blk = 304; continue;
        }
        304 => {
            let Some(t247) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080effd); };
            c.w32(11, t247);
            let r248 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r248);
            let Some(t249) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f004); };
            c.logic(t249 & 0x4u64, 8);
            if c.cc_ne() { blk = 1043; continue; }
            blk = 305; continue;
        }
        305 => {
            let Some(t250) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f00f); };
            c.cmp(t250, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 306; continue;
        }
        306 => {
            let Some(t251) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f019); };
            c.w64(8, t251);
            let Some(t252) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f01d); };
            c.w32(11, t252);
            let r253 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r253);
            if !c.st16(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x700080f024); }
            blk = 1026; continue;
        }
        307 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 315; continue; }
            blk = 308; continue;
        }
        308 => {
            c.w64(8, c.r64(1));
            let Some(t254) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f036); };
            c.w32(9, t254);
            let r255 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r255);
            let Some(t256) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f03d); };
            c.cmp(t256, 0x42u64, 16);
            if c.cc_e() { blk = 310; continue; }
            blk = 309; continue;
        }
        309 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        310 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f04d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 312; continue; }
            blk = 311; continue;
        }
        311 => {
            if c.cc_e() { blk = 313; continue; }
            blk = 312; continue;
        }
        312 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        313 => {
            let Some(t257) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f071); };
            c.w64(11, t257);
            let r258 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r258);
            let Some(t259) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f07b); };
            c.w64(12, t259);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 316; continue; }
            blk = 314; continue;
        }
        314 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        315 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 316; continue;
        }
        316 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700080f09b); }
            c.w64(11, c.r64(0));
            blk = 319; continue;
        }
        317 => {
            return c.call(0x70008a1014, 0x700080f0a9, 0x700080f0a4);
        }
        318 => {
            c.w64(11, c.r64(0));
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f0ac); };
            c.w64(0, t260);
            let Some(t261) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f0b0); };
            c.w64(6, t261);
            let Some(t262) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f0b4); };
            c.w64(2, t262);
            let Some(t263) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080f0b8); };
            c.w64(8, t263);
            let Some(t264) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080f0bc); };
            c.w64(9, t264);
            blk = 319; continue;
        }
        319 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 324; continue; }
            blk = 320; continue;
        }
        320 => {
            let Some(t265) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f0c6); };
            c.w32(12, t265);
            let r266 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r266);
            let Some(t267) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080f0cd); };
            c.cmp(t267, c.r32(12), 32);
            if c.cc_e() { blk = 322; continue; }
            blk = 321; continue;
        }
        321 => {
            let Some(t268) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f0d6); };
            c.cmp(t268, 0x43u64, 16);
            if c.cc_ne() { blk = 317; continue; }
            blk = 322; continue;
        }
        322 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f0df); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1097; continue; }
            blk = 323; continue;
        }
        323 => {
            c.w32(7, c.r32(7));
            blk = 325; continue;
        }
        324 => {
            let r269 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r269);
            c.w64(7, c.r64(11));
            blk = 325; continue;
        }
        325 => {
            let Some(t270) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f0fe); };
            c.w32(11, t270);
            let r271 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r271);
            let Some(t272) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f105); };
            c.logic(t272 & 0x4u64, 8);
            if c.cc_ne() { blk = 1044; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t273) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f110); };
            c.cmp(t273, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t274) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f11a); };
            c.w64(8, t274);
            let Some(t275) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f11e); };
            c.w32(11, t275);
            let r276 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r276);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x700080f125); }
            blk = 1026; continue;
        }
        328 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 336; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w64(8, c.r64(1));
            let Some(t277) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f136); };
            c.w32(9, t277);
            let r278 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r278);
            let Some(t279) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f13d); };
            c.cmp(t279, 0x42u64, 16);
            if c.cc_e() { blk = 331; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        331 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f14d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 333; continue; }
            blk = 332; continue;
        }
        332 => {
            if c.cc_e() { blk = 334; continue; }
            blk = 333; continue;
        }
        333 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        334 => {
            let Some(t280) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f171); };
            c.w64(11, t280);
            let r281 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r281);
            let Some(t282) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f17b); };
            c.w64(12, t282);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 337; continue; }
            blk = 335; continue;
        }
        335 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        336 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 337; continue;
        }
        337 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700080f19b); }
            c.w64(11, c.r64(0));
            blk = 340; continue;
        }
        338 => {
            return c.call(0x70008a1014, 0x700080f1a9, 0x700080f1a4);
        }
        339 => {
            c.w64(11, c.r64(0));
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f1ac); };
            c.w64(0, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f1b0); };
            c.w64(6, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f1b4); };
            c.w64(2, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080f1b8); };
            c.w64(8, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080f1bc); };
            c.w64(9, t287);
            blk = 340; continue;
        }
        340 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 345; continue; }
            blk = 341; continue;
        }
        341 => {
            let Some(t288) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f1c6); };
            c.w32(12, t288);
            let r289 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r289);
            let Some(t290) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080f1cd); };
            c.cmp(t290, c.r32(12), 32);
            if c.cc_e() { blk = 343; continue; }
            blk = 342; continue;
        }
        342 => {
            let Some(t291) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f1d6); };
            c.cmp(t291, 0x43u64, 16);
            if c.cc_ne() { blk = 338; continue; }
            blk = 343; continue;
        }
        343 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f1df); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1095; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w32(7, c.r32(7));
            blk = 346; continue;
        }
        345 => {
            let r292 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r292);
            c.w64(7, c.r64(11));
            blk = 346; continue;
        }
        346 => {
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f1fe); };
            c.w32(11, t293);
            let r294 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r294);
            let Some(t295) = c.ld8(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f205); };
            c.logic(t295 & 0x4u64, 8);
            if c.cc_ne() { blk = 1052; continue; }
            blk = 347; continue;
        }
        347 => {
            let Some(t296) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f210); };
            c.cmp(t296, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 348; continue;
        }
        348 => {
            let Some(t297) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f21a); };
            c.w64(8, t297);
            let Some(t298) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f21e); };
            c.w32(11, t298);
            let r299 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r299);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x700080f225); }
            blk = 1026; continue;
        }
        349 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 357; continue; }
            blk = 350; continue;
        }
        350 => {
            c.w64(8, c.r64(1));
            let Some(t300) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f236); };
            c.w32(9, t300);
            let r301 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r301);
            let Some(t302) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f23d); };
            c.cmp(t302, 0x42u64, 16);
            if c.cc_e() { blk = 352; continue; }
            blk = 351; continue;
        }
        351 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        352 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f24d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 354; continue; }
            blk = 353; continue;
        }
        353 => {
            if c.cc_e() { blk = 355; continue; }
            blk = 354; continue;
        }
        354 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        355 => {
            let Some(t303) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f271); };
            c.w64(11, t303);
            let r304 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r304);
            let Some(t305) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f27b); };
            c.w64(12, t305);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 358; continue; }
            blk = 356; continue;
        }
        356 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        357 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 358; continue;
        }
        358 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700080f29b); }
            c.w64(11, c.r64(0));
            blk = 361; continue;
        }
        359 => {
            return c.call(0x70008a1014, 0x700080f2a9, 0x700080f2a4);
        }
        360 => {
            c.w64(11, c.r64(0));
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f2ac); };
            c.w64(0, t306);
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f2b0); };
            c.w64(6, t307);
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f2b4); };
            c.w64(2, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080f2b8); };
            c.w64(8, t309);
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080f2bc); };
            c.w64(9, t310);
            blk = 361; continue;
        }
        361 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 365; continue; }
            blk = 362; continue;
        }
        362 => {
            let Some(t311) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f2c6); };
            c.w32(12, t311);
            let r312 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r312);
            let Some(t313) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080f2cd); };
            c.cmp(t313, c.r32(12), 32);
            if c.cc_e() { blk = 364; continue; }
            blk = 363; continue;
        }
        363 => {
            let Some(t314) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f2d6); };
            c.cmp(t314, 0x43u64, 16);
            if c.cc_ne() { blk = 359; continue; }
            blk = 364; continue;
        }
        364 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f2df); }
            c.cvtsd2ss(0, c.xmm_u64(0));
            blk = 366; continue;
        }
        365 => {
            let r315 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r315);
            c.xorpd_rr(0, 0);
            c.cvtsi2ss(0, (c.r32(11) as u32 as i32 as i64));
            blk = 366; continue;
        }
        366 => {
            let Some(t316) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f2f6); };
            c.w32(7, t316);
            let r317 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r317);
            let Some(t318) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f2fc); };
            c.logic(t318 & 0x4u64, 8);
            if c.cc_ne() { blk = 1060; continue; }
            blk = 367; continue;
        }
        367 => {
            let Some(t319) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f306); };
            c.cmp(t319, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 368; continue;
        }
        368 => {
            let Some(t320) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f310); };
            c.w64(7, t320);
            let Some(t321) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f314); };
            c.w32(8, t321);
            let r322 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r322);
            if !c.movss_mr(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)), 0) { return RegfileExit::fault(0x700080f31b); }
            blk = 1026; continue;
        }
        369 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 377; continue; }
            blk = 370; continue;
        }
        370 => {
            c.w64(8, c.r64(1));
            let Some(t323) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f32e); };
            c.w32(9, t323);
            let r324 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r324);
            let Some(t325) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f335); };
            c.cmp(t325, 0x42u64, 16);
            if c.cc_e() { blk = 372; continue; }
            blk = 371; continue;
        }
        371 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        372 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f345); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 374; continue; }
            blk = 373; continue;
        }
        373 => {
            if c.cc_e() { blk = 375; continue; }
            blk = 374; continue;
        }
        374 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        375 => {
            let Some(t326) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f369); };
            c.w64(11, t326);
            let r327 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r327);
            let Some(t328) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f373); };
            c.w64(12, t328);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 378; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        377 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 378; continue;
        }
        378 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x700080f393); }
            c.w64(11, c.r64(0));
            blk = 381; continue;
        }
        379 => {
            return c.call(0x70008a1014, 0x700080f3a1, 0x700080f39c);
        }
        380 => {
            c.w64(11, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f3a4); };
            c.w64(0, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f3a8); };
            c.w64(6, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f3ac); };
            c.w64(2, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080f3b0); };
            c.w64(8, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700080f3b4); };
            c.w64(9, t333);
            blk = 381; continue;
        }
        381 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 385; continue; }
            blk = 382; continue;
        }
        382 => {
            let Some(t334) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f3be); };
            c.w32(12, t334);
            let r335 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r335);
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080f3c5); };
            c.cmp(t336, c.r32(12), 32);
            if c.cc_e() { blk = 384; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t337) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f3ce); };
            c.cmp(t337, 0x43u64, 16);
            if c.cc_ne() { blk = 379; continue; }
            blk = 384; continue;
        }
        384 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f3d7); }
            blk = 386; continue;
        }
        385 => {
            let r338 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r338);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 386; continue;
        }
        386 => {
            let Some(t339) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f3eb); };
            c.w32(7, t339);
            let r340 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r340);
            let Some(t341) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f3f1); };
            c.logic(t341 & 0x4u64, 8);
            if c.cc_ne() { blk = 1068; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t342) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f3fb); };
            c.cmp(t342, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 388; continue;
        }
        388 => {
            let Some(t343) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f405); };
            c.w64(7, t343);
            let Some(t344) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f409); };
            c.w32(8, t344);
            let r345 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r345);
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), 0) { return RegfileExit::fault(0x700080f410); }
            blk = 1026; continue;
        }
        389 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 397; continue; }
            blk = 390; continue;
        }
        390 => {
            c.w64(8, c.r64(1));
            let Some(t346) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f423); };
            c.w32(9, t346);
            let r347 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r347);
            let Some(t348) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f42a); };
            c.cmp(t348, 0x42u64, 16);
            if c.cc_e() { blk = 392; continue; }
            blk = 391; continue;
        }
        391 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        392 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f43a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 394; continue; }
            blk = 393; continue;
        }
        393 => {
            if c.cc_e() { blk = 395; continue; }
            blk = 394; continue;
        }
        394 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        395 => {
            let Some(t349) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f45e); };
            c.w64(11, t349);
            let r350 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r350);
            let Some(t351) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f468); };
            c.w64(12, t351);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 398; continue; }
            blk = 396; continue;
        }
        396 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        397 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 398; continue;
        }
        398 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700080f488); }
            c.w64(11, c.r64(0));
            blk = 401; continue;
        }
        399 => {
            return c.call(0x70008a1014, 0x700080f496, 0x700080f491);
        }
        400 => {
            c.w64(11, c.r64(0));
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f499); };
            c.w64(0, t352);
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f49d); };
            c.w64(6, t353);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f4a1); };
            c.w64(2, t354);
            let Some(t355) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080f4a5); };
            c.w64(8, t355);
            let Some(t356) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080f4a9); };
            c.w64(9, t356);
            blk = 401; continue;
        }
        401 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 433; continue; }
            blk = 402; continue;
        }
        402 => {
            let Some(t357) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f4b7); };
            c.w32(12, t357);
            let r358 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r358);
            let Some(t359) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080f4be); };
            c.cmp(t359, c.r32(12), 32);
            if c.cc_e() { blk = 404; continue; }
            blk = 403; continue;
        }
        403 => {
            let Some(t360) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f4c7); };
            c.cmp(t360, 0x43u64, 16);
            if c.cc_ne() { blk = 399; continue; }
            blk = 404; continue;
        }
        404 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f4d0); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 432; continue; }
            blk = 405; continue;
        }
        405 => {
            c.w64(10, 0x406fe00000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 431; continue; }
            blk = 406; continue;
        }
        406 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 414; continue; }
            blk = 407; continue;
        }
        407 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 413; continue; }
            blk = 408; continue;
        }
        408 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 410; continue; }
            blk = 409; continue;
        }
        409 => {
            c.mov128_rr(1, 0);
            blk = 419; continue;
        }
        410 => {
            c.mov128_rr(1, 0);
            let Some(t361) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x700080f535); };
            c.w64(10, t361);
            if !c.xorpd_rm(1, c.r64(10)) { return RegfileExit::fault(0x700080f53c); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(2, 10);
            c.mov128_rr(3, 1);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(3));
            if c.cc_be() { blk = 412; continue; }
            blk = 411; continue;
        }
        411 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(3));
            c.mov128_rr(3, 1);
            blk = 412; continue;
        }
        412 => {
            let Some(t362) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x700080f575); };
            c.w64(10, t362);
            if !c.xorpd_rm(3, c.r64(10)) { return RegfileExit::fault(0x700080f57c); }
            c.mov128_rr(1, 3);
            blk = 419; continue;
        }
        413 => {
            c.mov128_rr(1, 0);
            blk = 419; continue;
        }
        414 => {
            let Some(t363) = c.ld64(0x700080f542u64) else { return RegfileExit::fault(0x700080f58a); };
            c.w64(10, t363);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 418; continue; }
            blk = 415; continue;
        }
        415 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 417; continue; }
            blk = 416; continue;
        }
        416 => {
            c.mov128_rr(1, 2);
            blk = 419; continue;
        }
        417 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.mov128_rr(1, 2);
            blk = 419; continue;
        }
        418 => {
            c.mov128_rr(1, 0);
            blk = 419; continue;
        }
        419 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x37);
            c.psrlq(2, 0x2);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 427; continue; }
            blk = 420; continue;
        }
        420 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 426; continue; }
            blk = 421; continue;
        }
        421 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x3f);
            c.psrlq(0, 0x1);
            c.mov128_rr(2, 1);
            c.w64(7, c.r64(0));
            let r364 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r364);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700080f602); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700080f607); }
            if !c.movsd_mr(c.r64(4), 2) { return RegfileExit::fault(0x700080f60a); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x700080f60f); }
            blk = 422; continue;
        }
        422 => {
            c.fprem();
            c.fnstsw_ax();
            let r365 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r365);
            let r366 = c.r32(0) & 0xffu64;
            c.w32(0, r366);
            c.logic(r366, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080f61e); }
            if !c.popfq() { return RegfileExit::fault(0x700080f61f); }
            if c.cc_p() { blk = 422; continue; }
            blk = 423; continue;
        }
        423 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x700080f624); }
            if !c.movsd_rm(2, c.r64(4)) { return RegfileExit::fault(0x700080f627); }
            let r367 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r367);
            c.xorpd_rr(0, 0);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_p() { blk = 428; continue; }
            blk = 424; continue;
        }
        424 => {
            if c.cc_e() { blk = 429; continue; }
            blk = 425; continue;
        }
        425 => {
            blk = 428; continue;
        }
        426 => {
            c.w64(7, c.r64(0));
            blk = 429; continue;
        }
        427 => {
            c.w64(7, c.r64(0));
            blk = 428; continue;
        }
        428 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.mov128_rr(1, 0);
            blk = 429; continue;
        }
        429 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(11, cv);
            c.cmp(c.r64(11), 0x1u64, 64);
            if c.cc_o() { blk = 1093; continue; }
            blk = 430; continue;
        }
        430 => {
            c.w32(11, c.r32(11));
            c.w64(0, c.r64(7));
            blk = 437; continue;
        }
        431 => {
            c.w32(11, 0xffu64);
            blk = 437; continue;
        }
        432 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 437; continue;
        }
        433 => {
            let r368 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r368);
            c.cmp(c.r32(11), 0xffu64, 32);
            if c.cc_be() { blk = 437; continue; }
            blk = 434; continue;
        }
        434 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_l() { blk = 436; continue; }
            blk = 435; continue;
        }
        435 => {
            c.w32(11, 0xffu64);
            blk = 437; continue;
        }
        436 => {
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(7));
            blk = 437; continue;
        }
        437 => {
            let Some(t369) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f69e); };
            c.w32(7, t369);
            let r370 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r370);
            let Some(t371) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f6a4); };
            c.logic(t371 & 0x4u64, 8);
            if c.cc_ne() { blk = 1076; continue; }
            blk = 438; continue;
        }
        438 => {
            let Some(t372) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f6ae); };
            c.cmp(t372, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 439; continue;
        }
        439 => {
            let Some(t373) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f6b8); };
            c.w64(7, t373);
            let Some(t374) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f6bc); };
            c.w32(8, t374);
            let r375 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r375);
            if !c.st8(c.r64(9).wrapping_add(c.r64(7)), c.r8(11)) { return RegfileExit::fault(0x700080f6c3); }
            blk = 1026; continue;
        }
        440 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 448; continue; }
            blk = 441; continue;
        }
        441 => {
            c.w64(8, c.r64(1));
            let Some(t376) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f6d4); };
            c.w32(9, t376);
            let r377 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r377);
            let Some(t378) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f6db); };
            c.cmp(t378, 0x42u64, 16);
            if c.cc_e() { blk = 443; continue; }
            blk = 442; continue;
        }
        442 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        443 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f6eb); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 445; continue; }
            blk = 444; continue;
        }
        444 => {
            if c.cc_e() { blk = 446; continue; }
            blk = 445; continue;
        }
        445 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        446 => {
            let Some(t379) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f70f); };
            c.w64(11, t379);
            let r380 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f719); };
            c.w64(12, t381);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 449; continue; }
            blk = 447; continue;
        }
        447 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        448 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 449; continue;
        }
        449 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700080f739); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 461; continue; }
            blk = 450; continue;
        }
        450 => {
            let Some(t382) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f745); };
            c.w32(11, t382);
            let r383 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r383);
            let Some(t384) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f74c); };
            c.cmp(t384, 0x41u64, 16);
            if c.cc_e() { blk = 453; continue; }
            blk = 451; continue;
        }
        451 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080f754); }
            c.w32(0, 0x1u64);
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700080f75a); };
            c.w64(3, t385);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700080f76c, 0x700080f767);
        }
        452 => {
            c.w64(11, c.r64(0));
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f76f); };
            c.w64(0, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f773); };
            c.w64(6, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f777); };
            c.w64(2, t388);
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080f77b); };
            c.w64(7, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080f77f); };
            c.w64(3, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080f783); };
            c.w64(8, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080f787); };
            c.w64(9, t392);
            blk = 454; continue;
        }
        453 => {
            c.w64(11, c.r64(0));
            blk = 454; continue;
        }
        454 => {
            let Some(t393) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f790); };
            c.w32(12, t393);
            let r394 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r394);
            let Some(t395) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f797); };
            c.logic(t395 & 0x4u64, 8);
            if c.cc_ne() { blk = 1077; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t396) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f7a3); };
            c.cmp(t396, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 456; continue;
        }
        456 => {
            let Some(t397) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f7ad); };
            c.w64(7, t397);
            let Some(t398) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f7b1); };
            c.w32(8, t398);
            let Some(t399) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080f7b5); };
            c.w32(12, t399);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 459; continue; }
            blk = 457; continue;
        }
        457 => {
            let Some(t400) = c.ld64(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f7c2); };
            c.w64(11, t400);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 460; continue; }
            blk = 458; continue;
        }
        458 => {
            let r401 = c.neg(c.r64(11), 64);
            c.w64(11, r401);
            blk = 460; continue;
        }
        459 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 460; continue;
        }
        460 => {
            let r402 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r402);
            if !c.st64(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(11)) { return RegfileExit::fault(0x700080f7d7); }
            blk = 1026; continue;
        }
        461 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080f7e5); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080f7e6); }
            c.w32(0, 0x2u64);
            let Some(t403) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700080f7ec); };
            c.w64(3, t403);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x700080f7fb, 0x700080f7f6);
        }
        462 => {
            return RegfileExit::fault(0x700080f7fb);
        }
        463 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 471; continue; }
            blk = 464; continue;
        }
        464 => {
            c.w64(8, c.r64(1));
            let Some(t404) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f804); };
            c.w32(9, t404);
            let r405 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r405);
            let Some(t406) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f80b); };
            c.cmp(t406, 0x42u64, 16);
            if c.cc_e() { blk = 466; continue; }
            blk = 465; continue;
        }
        465 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        466 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f81b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 468; continue; }
            blk = 467; continue;
        }
        467 => {
            if c.cc_e() { blk = 469; continue; }
            blk = 468; continue;
        }
        468 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        469 => {
            let Some(t407) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f83f); };
            c.w64(11, t407);
            let r408 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r408);
            let Some(t409) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f849); };
            c.w64(12, t409);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 472; continue; }
            blk = 470; continue;
        }
        470 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        471 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(1));
            blk = 472; continue;
        }
        472 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700080f869); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 484; continue; }
            blk = 473; continue;
        }
        473 => {
            let Some(t410) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f875); };
            c.w32(11, t410);
            let r411 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r411);
            let Some(t412) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f87c); };
            c.cmp(t412, 0x41u64, 16);
            if c.cc_e() { blk = 476; continue; }
            blk = 474; continue;
        }
        474 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080f884); }
            c.w32(0, 0x1u64);
            let Some(t413) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x700080f88a); };
            c.w64(3, t413);
            c.w64(11, c.r64(3));
            c.w64(12, c.r64(0));
            return c.call(0x700086ed74, 0x700080f89c, 0x700080f897);
        }
        475 => {
            c.w64(11, c.r64(0));
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f89f); };
            c.w64(0, t414);
            let Some(t415) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f8a3); };
            c.w64(6, t415);
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f8a7); };
            c.w64(2, t416);
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080f8ab); };
            c.w64(7, t417);
            let Some(t418) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700080f8af); };
            c.w64(3, t418);
            let Some(t419) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700080f8b3); };
            c.w64(8, t419);
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700080f8b7); };
            c.w64(9, t420);
            blk = 477; continue;
        }
        476 => {
            c.w64(11, c.r64(0));
            blk = 477; continue;
        }
        477 => {
            let Some(t421) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f8c0); };
            c.w32(12, t421);
            let r422 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r422);
            let Some(t423) = c.ld8(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f8c7); };
            c.logic(t423 & 0x4u64, 8);
            if c.cc_ne() { blk = 1078; continue; }
            blk = 478; continue;
        }
        478 => {
            let Some(t424) = c.ld64(c.r64(2).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080f8d3); };
            c.cmp(t424, c.r64(9), 64);
            if c.cc_be() { blk = 1026; continue; }
            blk = 479; continue;
        }
        479 => {
            let Some(t425) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080f8dd); };
            c.w64(7, t425);
            let Some(t426) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080f8e1); };
            c.w32(8, t426);
            let Some(t427) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700080f8e5); };
            c.w32(12, t427);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 482; continue; }
            blk = 480; continue;
        }
        480 => {
            let Some(t428) = c.ld64(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f8f2); };
            c.w64(11, t428);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 483; continue; }
            blk = 481; continue;
        }
        481 => {
            let r429 = c.neg(c.r64(11), 64);
            c.w64(11, r429);
            blk = 483; continue;
        }
        482 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 483; continue;
        }
        483 => {
            let r430 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r430);
            if !c.st64(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(8)), c.r64(11)) { return RegfileExit::fault(0x700080f907); }
            blk = 1026; continue;
        }
        484 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700080f915); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700080f916); }
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700080f917); };
            c.w64(3, t431);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700080f928, 0x700080f923);
        }
        485 => {
            return RegfileExit::fault(0x700080f928);
        }
        486 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 494; continue; }
            blk = 487; continue;
        }
        487 => {
            c.w64(8, c.r64(1));
            let Some(t432) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f931); };
            c.w32(9, t432);
            let r433 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r433);
            let Some(t434) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f938); };
            c.cmp(t434, 0x42u64, 16);
            if c.cc_e() { blk = 489; continue; }
            blk = 488; continue;
        }
        488 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        489 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f948); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 491; continue; }
            blk = 490; continue;
        }
        490 => {
            if c.cc_e() { blk = 492; continue; }
            blk = 491; continue;
        }
        491 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        492 => {
            let Some(t435) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080f96c); };
            c.w64(11, t435);
            let r436 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r436);
            let Some(t437) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080f976); };
            c.w64(12, t437);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 495; continue; }
            blk = 493; continue;
        }
        493 => {
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        494 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(9, c.r64(7));
            blk = 495; continue;
        }
        495 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x700080f993); }
            c.w64(7, c.r64(0));
            blk = 498; continue;
        }
        496 => {
            return c.call(0x70008a1014, 0x700080f9a1, 0x700080f99c);
        }
        497 => {
            c.w64(7, c.r64(0));
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080f9a4); };
            c.w64(0, t438);
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080f9a8); };
            c.w64(6, t439);
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080f9ac); };
            c.w64(2, t440);
            let Some(t441) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080f9b0); };
            c.w64(9, t441);
            blk = 498; continue;
        }
        498 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 503; continue; }
            blk = 499; continue;
        }
        499 => {
            let Some(t442) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080f9ba); };
            c.w32(8, t442);
            let r443 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r443);
            let Some(t444) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080f9c1); };
            c.cmp(t444, c.r32(8), 32);
            if c.cc_e() { blk = 501; continue; }
            blk = 500; continue;
        }
        500 => {
            let Some(t445) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080f9ca); };
            c.cmp(t445, 0x43u64, 16);
            if c.cc_ne() { blk = 496; continue; }
            blk = 501; continue;
        }
        501 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700080f9d2); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1091; continue; }
            blk = 502; continue;
        }
        502 => {
            c.w32(7, c.r32(7));
            blk = 504; continue;
        }
        503 => {
            let r446 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r446);
            blk = 504; continue;
        }
        504 => {
            c.w64(8, c.r64(2));
            let Some(t447) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080f9ef); };
            c.w32(11, t447);
            let r448 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r448);
            let Some(t449) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080f9f6); };
            c.w32(12, t449);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700080f9fa); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 530; continue; }
            blk = 505; continue;
        }
        505 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 506; continue;
        }
        506 => {
            let Some(t450) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080fa12); };
            c.w64(11, t450);
            let Some(t451) = c.ld64(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080fa16); };
            c.w64(12, t451);
            let Some(t452) = c.ld8(c.r64(8).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080fa1a); };
            c.logic(t452 & 0x1u64, 8);
            if c.cc_ne() { blk = 509; continue; }
            blk = 507; continue;
        }
        507 => {
            let Some(t453) = c.ld64(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080fa21); };
            c.w64(15, t453);
            let r454 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r454);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 508; continue;
        }
        508 => {
            let Some(t455) = c.ld64(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080fa31); };
            c.w64(11, t455);
            blk = 532; continue;
        }
        509 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 510; continue;
        }
        510 => {
            let Some(t456) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080fa43); };
            c.w32(15, t456);
            let r457 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r457);
            let Some(t458) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080fa4a); };
            c.w32(15, t458);
            let r459 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r459);
            let r460 = c.r32(15) & 0x3fu64;
            c.w32(15, r460);
            c.logic(r460, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 518; continue; }
            blk = 511; continue;
        }
        511 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 515; continue; }
            blk = 512; continue;
        }
        512 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 525; continue; }
            blk = 513; continue;
        }
        513 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 528; continue; }
            blk = 514; continue;
        }
        514 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 528; continue; }
            blk = 515; continue;
        }
        515 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 527; continue; }
            blk = 516; continue;
        }
        516 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 527; continue; }
            blk = 517; continue;
        }
        517 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 528; continue; }
            blk = 518; continue;
        }
        518 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 522; continue; }
            blk = 519; continue;
        }
        519 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 526; continue; }
            blk = 520; continue;
        }
        520 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 526; continue; }
            blk = 521; continue;
        }
        521 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 527; continue; }
            blk = 522; continue;
        }
        522 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 525; continue; }
            blk = 523; continue;
        }
        523 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 525; continue; }
            blk = 524; continue;
        }
        524 => {
            return RegfileExit::fault(0x700080faaf);
        }
        525 => {
            c.w32(15, 0x1u64);
            blk = 529; continue;
        }
        526 => {
            c.w32(15, 0x2u64);
            blk = 529; continue;
        }
        527 => {
            c.w32(15, 0x4u64);
            blk = 529; continue;
        }
        528 => {
            c.w32(15, 0x8u64);
            blk = 529; continue;
        }
        529 => {
            let r461 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r461);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 532; continue;
        }
        530 => {
            c.w64(10, c.r64(4));
            let r462 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r462);
            let r463 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r463);
            c.logic(r463, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080faf5); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(8));
            let Some(t464) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080fb00); };
            c.w64(0, t464);
            c.w64(10, 0x700080fb0eu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080fb0e); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080fb12); }
            return c.call(c.r64(0), 0x700080fb18, 0x700080fb16);
        }
        531 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080fb18); }
            let Some(t465) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080fb20); };
            c.w64(4, t465);
            c.w64(11, c.r64(0));
            let Some(t466) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080fb28); };
            c.w64(0, t466);
            let Some(t467) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080fb2c); };
            c.w64(8, t467);
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080fb30); };
            c.w64(9, t468);
            let Some(t469) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080fb34); };
            c.w64(7, t469);
            blk = 532; continue;
        }
        532 => {
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 533; continue;
        }
        533 => {
            c.w64(2, c.r64(8));
            let Some(t470) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080fb44); };
            c.w64(8, t470);
            let Some(t471) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080fb48); };
            c.w32(11, t471);
            let r472 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r472);
            if !c.st8(c.r64(9).wrapping_add(c.r64(8)), c.r8(7)) { return RegfileExit::fault(0x700080fb4f); }
            blk = 1026; continue;
        }
        534 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 540; continue; }
            blk = 535; continue;
        }
        535 => {
            let Some(t473) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080fb5d); };
            c.w32(8, t473);
            let r474 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r474);
            let Some(t475) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080fb64); };
            c.cmp(t475, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 536; continue;
        }
        536 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080fb70); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 537; continue;
        }
        537 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 538; continue;
        }
        538 => {
            let Some(t476) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080fb93); };
            c.w64(9, t476);
            let r477 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r477);
            let Some(t478) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080fb9d); };
            c.w64(11, t478);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 541; continue; }
            blk = 539; continue;
        }
        539 => {
            blk = 984; continue;
        }
        540 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 541; continue;
        }
        541 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700080fbb7); }
            c.w64(7, c.r64(0));
            blk = 544; continue;
        }
        542 => {
            return c.call(0x70008a1014, 0x700080fbc5, 0x700080fbc0);
        }
        543 => {
            c.w64(7, c.r64(0));
            let Some(t479) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080fbc8); };
            c.w64(0, t479);
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080fbcc); };
            c.w64(6, t480);
            let Some(t481) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080fbd0); };
            c.w64(2, t481);
            let Some(t482) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080fbd4); };
            c.w64(8, t482);
            blk = 544; continue;
        }
        544 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 549; continue; }
            blk = 545; continue;
        }
        545 => {
            let Some(t483) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080fbde); };
            c.w32(9, t483);
            let r484 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r484);
            let Some(t485) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080fbe5); };
            c.cmp(t485, c.r32(9), 32);
            if c.cc_e() { blk = 547; continue; }
            blk = 546; continue;
        }
        546 => {
            let Some(t486) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080fbee); };
            c.cmp(t486, 0x43u64, 16);
            if c.cc_ne() { blk = 542; continue; }
            blk = 547; continue;
        }
        547 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700080fbf6); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1089; continue; }
            blk = 548; continue;
        }
        548 => {
            c.w32(7, c.r32(7));
            blk = 550; continue;
        }
        549 => {
            let r487 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r487);
            blk = 550; continue;
        }
        550 => {
            c.w64(9, c.r64(2));
            let Some(t488) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080fc13); };
            c.w32(11, t488);
            let r489 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r489);
            let Some(t490) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080fc1a); };
            c.w32(12, t490);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700080fc1e); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 576; continue; }
            blk = 551; continue;
        }
        551 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 552; continue;
        }
        552 => {
            let Some(t491) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080fc36); };
            c.w64(11, t491);
            let Some(t492) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080fc3a); };
            c.w64(12, t492);
            let Some(t493) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080fc3e); };
            c.logic(t493 & 0x1u64, 8);
            if c.cc_ne() { blk = 555; continue; }
            blk = 553; continue;
        }
        553 => {
            let Some(t494) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080fc45); };
            c.w64(15, t494);
            let r495 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r495);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 554; continue;
        }
        554 => {
            let Some(t496) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080fc55); };
            c.w64(11, t496);
            blk = 578; continue;
        }
        555 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 556; continue;
        }
        556 => {
            let Some(t497) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080fc67); };
            c.w32(15, t497);
            let r498 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r498);
            let Some(t499) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080fc6e); };
            c.w32(15, t499);
            let r500 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r500);
            let r501 = c.r32(15) & 0x3fu64;
            c.w32(15, r501);
            c.logic(r501, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 564; continue; }
            blk = 557; continue;
        }
        557 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 561; continue; }
            blk = 558; continue;
        }
        558 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 571; continue; }
            blk = 559; continue;
        }
        559 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 574; continue; }
            blk = 560; continue;
        }
        560 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 574; continue; }
            blk = 561; continue;
        }
        561 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 573; continue; }
            blk = 562; continue;
        }
        562 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 573; continue; }
            blk = 563; continue;
        }
        563 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 574; continue; }
            blk = 564; continue;
        }
        564 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 568; continue; }
            blk = 565; continue;
        }
        565 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 572; continue; }
            blk = 566; continue;
        }
        566 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 572; continue; }
            blk = 567; continue;
        }
        567 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 573; continue; }
            blk = 568; continue;
        }
        568 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 571; continue; }
            blk = 569; continue;
        }
        569 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 571; continue; }
            blk = 570; continue;
        }
        570 => {
            return RegfileExit::fault(0x700080fcd3);
        }
        571 => {
            c.w32(15, 0x1u64);
            blk = 575; continue;
        }
        572 => {
            c.w32(15, 0x2u64);
            blk = 575; continue;
        }
        573 => {
            c.w32(15, 0x4u64);
            blk = 575; continue;
        }
        574 => {
            c.w32(15, 0x8u64);
            blk = 575; continue;
        }
        575 => {
            let r502 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r502);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 578; continue;
        }
        576 => {
            c.w64(10, c.r64(4));
            let r503 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r503);
            let r504 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r504);
            c.logic(r504, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080fd19); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(9));
            let Some(t505) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080fd24); };
            c.w64(0, t505);
            c.w64(10, 0x700080fd32u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080fd32); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080fd36); }
            return c.call(c.r64(0), 0x700080fd3c, 0x700080fd3a);
        }
        577 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080fd3c); }
            let Some(t506) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080fd44); };
            c.w64(4, t506);
            c.w64(11, c.r64(0));
            let Some(t507) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080fd4c); };
            c.w64(0, t507);
            let Some(t508) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080fd50); };
            c.w64(9, t508);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080fd54); };
            c.w64(8, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080fd58); };
            c.w64(7, t510);
            blk = 578; continue;
        }
        578 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 579; continue;
        }
        579 => {
            c.w64(2, c.r64(9));
            let Some(t511) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080fd68); };
            c.w64(9, t511);
            let Some(t512) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080fd6c); };
            c.w32(11, t512);
            let r513 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r513);
            if !c.st8(c.r64(8).wrapping_add(c.r64(9)), c.r8(7)) { return RegfileExit::fault(0x700080fd73); }
            blk = 1026; continue;
        }
        580 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 586; continue; }
            blk = 581; continue;
        }
        581 => {
            let Some(t514) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080fd81); };
            c.w32(8, t514);
            let r515 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r515);
            let Some(t516) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080fd88); };
            c.cmp(t516, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 582; continue;
        }
        582 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080fd94); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 583; continue;
        }
        583 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 584; continue;
        }
        584 => {
            let Some(t517) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080fdb7); };
            c.w64(9, t517);
            let r518 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r518);
            let Some(t519) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080fdc1); };
            c.w64(11, t519);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 587; continue; }
            blk = 585; continue;
        }
        585 => {
            blk = 984; continue;
        }
        586 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 587; continue;
        }
        587 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700080fddb); }
            c.w64(7, c.r64(0));
            blk = 590; continue;
        }
        588 => {
            return c.call(0x70008a1014, 0x700080fde9, 0x700080fde4);
        }
        589 => {
            c.w64(7, c.r64(0));
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080fdec); };
            c.w64(0, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080fdf0); };
            c.w64(6, t521);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080fdf4); };
            c.w64(2, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080fdf8); };
            c.w64(8, t523);
            blk = 590; continue;
        }
        590 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 595; continue; }
            blk = 591; continue;
        }
        591 => {
            let Some(t524) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080fe02); };
            c.w32(9, t524);
            let r525 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r525);
            let Some(t526) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700080fe09); };
            c.cmp(t526, c.r32(9), 32);
            if c.cc_e() { blk = 593; continue; }
            blk = 592; continue;
        }
        592 => {
            let Some(t527) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080fe12); };
            c.cmp(t527, 0x43u64, 16);
            if c.cc_ne() { blk = 588; continue; }
            blk = 593; continue;
        }
        593 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700080fe1a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1087; continue; }
            blk = 594; continue;
        }
        594 => {
            c.w32(7, c.r32(7));
            blk = 596; continue;
        }
        595 => {
            let r528 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r528);
            blk = 596; continue;
        }
        596 => {
            c.w64(9, c.r64(2));
            let Some(t529) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080fe37); };
            c.w32(11, t529);
            let r530 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r530);
            let Some(t531) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700080fe3e); };
            c.w32(12, t531);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700080fe42); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 622; continue; }
            blk = 597; continue;
        }
        597 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 598; continue;
        }
        598 => {
            let Some(t532) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700080fe5a); };
            c.w64(11, t532);
            let Some(t533) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700080fe5e); };
            c.w64(12, t533);
            let Some(t534) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x700080fe62); };
            c.logic(t534 & 0x1u64, 8);
            if c.cc_ne() { blk = 601; continue; }
            blk = 599; continue;
        }
        599 => {
            let Some(t535) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700080fe69); };
            c.w64(15, t535);
            let r536 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r536);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 600; continue;
        }
        600 => {
            let Some(t537) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700080fe79); };
            c.w64(11, t537);
            blk = 624; continue;
        }
        601 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 602; continue;
        }
        602 => {
            let Some(t538) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080fe8b); };
            c.w32(15, t538);
            let r539 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r539);
            let Some(t540) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x700080fe92); };
            c.w32(15, t540);
            let r541 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r541);
            let r542 = c.r32(15) & 0x3fu64;
            c.w32(15, r542);
            c.logic(r542, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 610; continue; }
            blk = 603; continue;
        }
        603 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 607; continue; }
            blk = 604; continue;
        }
        604 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 617; continue; }
            blk = 605; continue;
        }
        605 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 620; continue; }
            blk = 606; continue;
        }
        606 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 620; continue; }
            blk = 607; continue;
        }
        607 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 619; continue; }
            blk = 608; continue;
        }
        608 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 619; continue; }
            blk = 609; continue;
        }
        609 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 620; continue; }
            blk = 610; continue;
        }
        610 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 614; continue; }
            blk = 611; continue;
        }
        611 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 618; continue; }
            blk = 612; continue;
        }
        612 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 618; continue; }
            blk = 613; continue;
        }
        613 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 619; continue; }
            blk = 614; continue;
        }
        614 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 617; continue; }
            blk = 615; continue;
        }
        615 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 617; continue; }
            blk = 616; continue;
        }
        616 => {
            return RegfileExit::fault(0x700080fef7);
        }
        617 => {
            c.w32(15, 0x1u64);
            blk = 621; continue;
        }
        618 => {
            c.w32(15, 0x2u64);
            blk = 621; continue;
        }
        619 => {
            c.w32(15, 0x4u64);
            blk = 621; continue;
        }
        620 => {
            c.w32(15, 0x8u64);
            blk = 621; continue;
        }
        621 => {
            let r543 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r543);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 624; continue;
        }
        622 => {
            c.w64(10, c.r64(4));
            let r544 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r544);
            let r545 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r545);
            c.logic(r545, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700080ff3d); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(9));
            let Some(t546) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700080ff48); };
            c.w64(0, t546);
            c.w64(10, 0x700080ff56u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700080ff56); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700080ff5a); }
            return c.call(c.r64(0), 0x700080ff60, 0x700080ff5e);
        }
        623 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700080ff60); }
            let Some(t547) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700080ff68); };
            c.w64(4, t547);
            c.w64(11, c.r64(0));
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700080ff70); };
            c.w64(0, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700080ff74); };
            c.w64(9, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700080ff78); };
            c.w64(8, t550);
            let Some(t551) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700080ff7c); };
            c.w64(7, t551);
            blk = 624; continue;
        }
        624 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 625; continue;
        }
        625 => {
            c.w64(2, c.r64(9));
            let Some(t552) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700080ff8c); };
            c.w64(9, t552);
            let Some(t553) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700080ff90); };
            c.w32(11, t553);
            let r554 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r554);
            if !c.st16(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x700080ff97); }
            blk = 1026; continue;
        }
        626 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 632; continue; }
            blk = 627; continue;
        }
        627 => {
            let Some(t555) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700080ffa6); };
            c.w32(8, t555);
            let r556 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r556);
            let Some(t557) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700080ffad); };
            c.cmp(t557, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 628; continue;
        }
        628 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700080ffb9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 629; continue;
        }
        629 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 630; continue;
        }
        630 => {
            let Some(t558) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700080ffdc); };
            c.w64(9, t558);
            let r559 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r559);
            let Some(t560) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700080ffe6); };
            c.w64(11, t560);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 633; continue; }
            blk = 631; continue;
        }
        631 => {
            blk = 984; continue;
        }
        632 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 633; continue;
        }
        633 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000810000); }
            c.w64(7, c.r64(0));
            blk = 636; continue;
        }
        634 => {
            return c.call(0x70008a1014, 0x700081000e, 0x7000810009);
        }
        635 => {
            c.w64(7, c.r64(0));
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810011); };
            c.w64(0, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000810015); };
            c.w64(6, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810019); };
            c.w64(2, t563);
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081001d); };
            c.w64(8, t564);
            blk = 636; continue;
        }
        636 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 641; continue; }
            blk = 637; continue;
        }
        637 => {
            let Some(t565) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810027); };
            c.w32(9, t565);
            let r566 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r566);
            let Some(t567) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081002e); };
            c.cmp(t567, c.r32(9), 32);
            if c.cc_e() { blk = 639; continue; }
            blk = 638; continue;
        }
        638 => {
            let Some(t568) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000810037); };
            c.cmp(t568, 0x43u64, 16);
            if c.cc_ne() { blk = 634; continue; }
            blk = 639; continue;
        }
        639 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700081003f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1085; continue; }
            blk = 640; continue;
        }
        640 => {
            c.w32(7, c.r32(7));
            blk = 642; continue;
        }
        641 => {
            let r569 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r569);
            blk = 642; continue;
        }
        642 => {
            c.w64(9, c.r64(2));
            let Some(t570) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081005c); };
            c.w32(11, t570);
            let r571 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r571);
            let Some(t572) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000810063); };
            c.w32(12, t572);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000810067); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 668; continue; }
            blk = 643; continue;
        }
        643 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 644; continue;
        }
        644 => {
            let Some(t573) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081007f); };
            c.w64(11, t573);
            let Some(t574) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000810083); };
            c.w64(12, t574);
            let Some(t575) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000810087); };
            c.logic(t575 & 0x1u64, 8);
            if c.cc_ne() { blk = 647; continue; }
            blk = 645; continue;
        }
        645 => {
            let Some(t576) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x700081008e); };
            c.w64(15, t576);
            let r577 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r577);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 646; continue;
        }
        646 => {
            let Some(t578) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700081009e); };
            c.w64(11, t578);
            blk = 670; continue;
        }
        647 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 648; continue;
        }
        648 => {
            let Some(t579) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008100b0); };
            c.w32(15, t579);
            let r580 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r580);
            let Some(t581) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008100b7); };
            c.w32(15, t581);
            let r582 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r582);
            let r583 = c.r32(15) & 0x3fu64;
            c.w32(15, r583);
            c.logic(r583, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 656; continue; }
            blk = 649; continue;
        }
        649 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 653; continue; }
            blk = 650; continue;
        }
        650 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 663; continue; }
            blk = 651; continue;
        }
        651 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 666; continue; }
            blk = 652; continue;
        }
        652 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 666; continue; }
            blk = 653; continue;
        }
        653 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 665; continue; }
            blk = 654; continue;
        }
        654 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 665; continue; }
            blk = 655; continue;
        }
        655 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 666; continue; }
            blk = 656; continue;
        }
        656 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 660; continue; }
            blk = 657; continue;
        }
        657 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 664; continue; }
            blk = 658; continue;
        }
        658 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 664; continue; }
            blk = 659; continue;
        }
        659 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 665; continue; }
            blk = 660; continue;
        }
        660 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 663; continue; }
            blk = 661; continue;
        }
        661 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 663; continue; }
            blk = 662; continue;
        }
        662 => {
            return RegfileExit::fault(0x700081011c);
        }
        663 => {
            c.w32(15, 0x1u64);
            blk = 667; continue;
        }
        664 => {
            c.w32(15, 0x2u64);
            blk = 667; continue;
        }
        665 => {
            c.w32(15, 0x4u64);
            blk = 667; continue;
        }
        666 => {
            c.w32(15, 0x8u64);
            blk = 667; continue;
        }
        667 => {
            let r584 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r584);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 670; continue;
        }
        668 => {
            c.w64(10, c.r64(4));
            let r585 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r585);
            let r586 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r586);
            c.logic(r586, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000810162); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(9));
            let Some(t587) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x700081016d); };
            c.w64(0, t587);
            c.w64(10, 0x700081017bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700081017b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700081017f); }
            return c.call(c.r64(0), 0x7000810185, 0x7000810183);
        }
        669 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000810185); }
            let Some(t588) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081018d); };
            c.w64(4, t588);
            c.w64(11, c.r64(0));
            let Some(t589) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810195); };
            c.w64(0, t589);
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810199); };
            c.w64(9, t590);
            let Some(t591) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081019d); };
            c.w64(8, t591);
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008101a1); };
            c.w64(7, t592);
            blk = 670; continue;
        }
        670 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 671; continue;
        }
        671 => {
            c.w64(2, c.r64(9));
            let Some(t593) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008101b1); };
            c.w64(9, t593);
            let Some(t594) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008101b5); };
            c.w32(11, t594);
            let r595 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r595);
            if !c.st16(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(2)), c.r16(7)) { return RegfileExit::fault(0x70008101bc); }
            blk = 1026; continue;
        }
        672 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 678; continue; }
            blk = 673; continue;
        }
        673 => {
            let Some(t596) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008101cb); };
            c.w32(8, t596);
            let r597 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r597);
            let Some(t598) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008101d2); };
            c.cmp(t598, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 674; continue;
        }
        674 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008101de); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 675; continue;
        }
        675 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 676; continue;
        }
        676 => {
            let Some(t599) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x7000810201); };
            c.w64(9, t599);
            let r600 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r600);
            let Some(t601) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700081020b); };
            c.w64(11, t601);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 679; continue; }
            blk = 677; continue;
        }
        677 => {
            blk = 984; continue;
        }
        678 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 679; continue;
        }
        679 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000810225); }
            c.w64(7, c.r64(0));
            blk = 682; continue;
        }
        680 => {
            return c.call(0x70008a1014, 0x7000810233, 0x700081022e);
        }
        681 => {
            c.w64(7, c.r64(0));
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810236); };
            c.w64(0, t602);
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081023a); };
            c.w64(6, t603);
            let Some(t604) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081023e); };
            c.w64(2, t604);
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810242); };
            c.w64(8, t605);
            blk = 682; continue;
        }
        682 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 687; continue; }
            blk = 683; continue;
        }
        683 => {
            let Some(t606) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081024c); };
            c.w32(9, t606);
            let r607 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r607);
            let Some(t608) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000810253); };
            c.cmp(t608, c.r32(9), 32);
            if c.cc_e() { blk = 685; continue; }
            blk = 684; continue;
        }
        684 => {
            let Some(t609) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081025c); };
            c.cmp(t609, 0x43u64, 16);
            if c.cc_ne() { blk = 680; continue; }
            blk = 685; continue;
        }
        685 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810264); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1083; continue; }
            blk = 686; continue;
        }
        686 => {
            c.w32(7, c.r32(7));
            blk = 688; continue;
        }
        687 => {
            let r610 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r610);
            blk = 688; continue;
        }
        688 => {
            c.w64(9, c.r64(2));
            let Some(t611) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000810281); };
            c.w32(11, t611);
            let r612 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r612);
            let Some(t613) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000810288); };
            c.w32(12, t613);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x700081028c); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 714; continue; }
            blk = 689; continue;
        }
        689 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 690; continue;
        }
        690 => {
            let Some(t614) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008102a4); };
            c.w64(11, t614);
            let Some(t615) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008102a8); };
            c.w64(12, t615);
            let Some(t616) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008102ac); };
            c.logic(t616 & 0x1u64, 8);
            if c.cc_ne() { blk = 693; continue; }
            blk = 691; continue;
        }
        691 => {
            let Some(t617) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008102b3); };
            c.w64(15, t617);
            let r618 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r618);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 692; continue;
        }
        692 => {
            let Some(t619) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008102c3); };
            c.w64(11, t619);
            blk = 716; continue;
        }
        693 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 694; continue;
        }
        694 => {
            let Some(t620) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008102d5); };
            c.w32(15, t620);
            let r621 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r621);
            let Some(t622) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008102dc); };
            c.w32(15, t622);
            let r623 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r623);
            let r624 = c.r32(15) & 0x3fu64;
            c.w32(15, r624);
            c.logic(r624, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 702; continue; }
            blk = 695; continue;
        }
        695 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 699; continue; }
            blk = 696; continue;
        }
        696 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 709; continue; }
            blk = 697; continue;
        }
        697 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 712; continue; }
            blk = 698; continue;
        }
        698 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 712; continue; }
            blk = 699; continue;
        }
        699 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 711; continue; }
            blk = 700; continue;
        }
        700 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 711; continue; }
            blk = 701; continue;
        }
        701 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 712; continue; }
            blk = 702; continue;
        }
        702 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 706; continue; }
            blk = 703; continue;
        }
        703 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 710; continue; }
            blk = 704; continue;
        }
        704 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 710; continue; }
            blk = 705; continue;
        }
        705 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 711; continue; }
            blk = 706; continue;
        }
        706 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 709; continue; }
            blk = 707; continue;
        }
        707 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 709; continue; }
            blk = 708; continue;
        }
        708 => {
            return RegfileExit::fault(0x7000810341);
        }
        709 => {
            c.w32(15, 0x1u64);
            blk = 713; continue;
        }
        710 => {
            c.w32(15, 0x2u64);
            blk = 713; continue;
        }
        711 => {
            c.w32(15, 0x4u64);
            blk = 713; continue;
        }
        712 => {
            c.w32(15, 0x8u64);
            blk = 713; continue;
        }
        713 => {
            let r625 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r625);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 716; continue;
        }
        714 => {
            c.w64(10, c.r64(4));
            let r626 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r626);
            let r627 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r627);
            c.logic(r627, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000810387); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(9));
            let Some(t628) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000810392); };
            c.w64(0, t628);
            c.w64(10, 0x70008103a0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008103a0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008103a4); }
            return c.call(c.r64(0), 0x70008103aa, 0x70008103a8);
        }
        715 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008103aa); }
            let Some(t629) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008103b2); };
            c.w64(4, t629);
            c.w64(11, c.r64(0));
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008103ba); };
            c.w64(0, t630);
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008103be); };
            c.w64(9, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008103c2); };
            c.w64(8, t632);
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008103c6); };
            c.w64(7, t633);
            blk = 716; continue;
        }
        716 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 717; continue;
        }
        717 => {
            c.w64(2, c.r64(9));
            let Some(t634) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008103d6); };
            c.w64(9, t634);
            let Some(t635) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008103da); };
            c.w32(11, t635);
            let r636 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r636);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x70008103e1); }
            blk = 1026; continue;
        }
        718 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 724; continue; }
            blk = 719; continue;
        }
        719 => {
            let Some(t637) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008103ef); };
            c.w32(8, t637);
            let r638 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r638);
            let Some(t639) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008103f6); };
            c.cmp(t639, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 720; continue;
        }
        720 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810402); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 721; continue;
        }
        721 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 722; continue;
        }
        722 => {
            let Some(t640) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x7000810425); };
            c.w64(9, t640);
            let r641 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r641);
            let Some(t642) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x700081042f); };
            c.w64(11, t642);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 725; continue; }
            blk = 723; continue;
        }
        723 => {
            blk = 984; continue;
        }
        724 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 725; continue;
        }
        725 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000810449); }
            c.w64(7, c.r64(0));
            blk = 728; continue;
        }
        726 => {
            return c.call(0x70008a1014, 0x7000810457, 0x7000810452);
        }
        727 => {
            c.w64(7, c.r64(0));
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081045a); };
            c.w64(0, t643);
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081045e); };
            c.w64(6, t644);
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810462); };
            c.w64(2, t645);
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810466); };
            c.w64(8, t646);
            blk = 728; continue;
        }
        728 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 733; continue; }
            blk = 729; continue;
        }
        729 => {
            let Some(t647) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810470); };
            c.w32(9, t647);
            let r648 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r648);
            let Some(t649) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000810477); };
            c.cmp(t649, c.r32(9), 32);
            if c.cc_e() { blk = 731; continue; }
            blk = 730; continue;
        }
        730 => {
            let Some(t650) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000810480); };
            c.cmp(t650, 0x43u64, 16);
            if c.cc_ne() { blk = 726; continue; }
            blk = 731; continue;
        }
        731 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810488); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1081; continue; }
            blk = 732; continue;
        }
        732 => {
            c.w32(7, c.r32(7));
            blk = 734; continue;
        }
        733 => {
            let r651 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r651);
            blk = 734; continue;
        }
        734 => {
            c.w64(9, c.r64(2));
            let Some(t652) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008104a5); };
            c.w32(11, t652);
            let r653 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r653);
            let Some(t654) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008104ac); };
            c.w32(12, t654);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008104b0); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 760; continue; }
            blk = 735; continue;
        }
        735 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 736; continue;
        }
        736 => {
            let Some(t655) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008104c8); };
            c.w64(11, t655);
            let Some(t656) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008104cc); };
            c.w64(12, t656);
            let Some(t657) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008104d0); };
            c.logic(t657 & 0x1u64, 8);
            if c.cc_ne() { blk = 739; continue; }
            blk = 737; continue;
        }
        737 => {
            let Some(t658) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008104d7); };
            c.w64(15, t658);
            let r659 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r659);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 738; continue;
        }
        738 => {
            let Some(t660) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008104e7); };
            c.w64(11, t660);
            blk = 762; continue;
        }
        739 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 740; continue;
        }
        740 => {
            let Some(t661) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008104f9); };
            c.w32(15, t661);
            let r662 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r662);
            let Some(t663) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000810500); };
            c.w32(15, t663);
            let r664 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r664);
            let r665 = c.r32(15) & 0x3fu64;
            c.w32(15, r665);
            c.logic(r665, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 748; continue; }
            blk = 741; continue;
        }
        741 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 745; continue; }
            blk = 742; continue;
        }
        742 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 755; continue; }
            blk = 743; continue;
        }
        743 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 758; continue; }
            blk = 744; continue;
        }
        744 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 758; continue; }
            blk = 745; continue;
        }
        745 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 757; continue; }
            blk = 746; continue;
        }
        746 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 757; continue; }
            blk = 747; continue;
        }
        747 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 758; continue; }
            blk = 748; continue;
        }
        748 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 752; continue; }
            blk = 749; continue;
        }
        749 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 756; continue; }
            blk = 750; continue;
        }
        750 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 756; continue; }
            blk = 751; continue;
        }
        751 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 757; continue; }
            blk = 752; continue;
        }
        752 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 755; continue; }
            blk = 753; continue;
        }
        753 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 755; continue; }
            blk = 754; continue;
        }
        754 => {
            return RegfileExit::fault(0x7000810565);
        }
        755 => {
            c.w32(15, 0x1u64);
            blk = 759; continue;
        }
        756 => {
            c.w32(15, 0x2u64);
            blk = 759; continue;
        }
        757 => {
            c.w32(15, 0x4u64);
            blk = 759; continue;
        }
        758 => {
            c.w32(15, 0x8u64);
            blk = 759; continue;
        }
        759 => {
            let r666 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r666);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 762; continue;
        }
        760 => {
            c.w64(10, c.r64(4));
            let r667 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r667);
            let r668 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r668);
            c.logic(r668, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008105ab); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(9));
            let Some(t669) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x70008105b6); };
            c.w64(0, t669);
            c.w64(10, 0x70008105c4u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008105c4); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008105c8); }
            return c.call(c.r64(0), 0x70008105ce, 0x70008105cc);
        }
        761 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008105ce); }
            let Some(t670) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008105d6); };
            c.w64(4, t670);
            c.w64(11, c.r64(0));
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008105de); };
            c.w64(0, t671);
            let Some(t672) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008105e2); };
            c.w64(9, t672);
            let Some(t673) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008105e6); };
            c.w64(8, t673);
            let Some(t674) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008105ea); };
            c.w64(7, t674);
            blk = 762; continue;
        }
        762 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 763; continue;
        }
        763 => {
            c.w64(2, c.r64(9));
            let Some(t675) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008105fa); };
            c.w64(9, t675);
            let Some(t676) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008105fe); };
            c.w32(11, t676);
            let r677 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r677);
            if !c.st32(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(4)), c.r32(7)) { return RegfileExit::fault(0x7000810605); }
            blk = 1026; continue;
        }
        764 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 770; continue; }
            blk = 765; continue;
        }
        765 => {
            let Some(t678) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810613); };
            c.w32(8, t678);
            let r679 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r679);
            let Some(t680) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081061a); };
            c.cmp(t680, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 766; continue;
        }
        766 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810626); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 767; continue;
        }
        767 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 768; continue;
        }
        768 => {
            let Some(t681) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x7000810649); };
            c.w64(9, t681);
            let r682 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r682);
            let Some(t683) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x7000810653); };
            c.w64(11, t683);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 771; continue; }
            blk = 769; continue;
        }
        769 => {
            blk = 984; continue;
        }
        770 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 771; continue;
        }
        771 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700081066d); }
            c.w64(7, c.r64(0));
            blk = 774; continue;
        }
        772 => {
            return c.call(0x70008a1014, 0x700081067b, 0x7000810676);
        }
        773 => {
            c.w64(7, c.r64(0));
            let Some(t684) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081067e); };
            c.w64(0, t684);
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000810682); };
            c.w64(6, t685);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810686); };
            c.w64(2, t686);
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081068a); };
            c.w64(8, t687);
            blk = 774; continue;
        }
        774 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 778; continue; }
            blk = 775; continue;
        }
        775 => {
            let Some(t688) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810694); };
            c.w32(9, t688);
            let r689 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r689);
            let Some(t690) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081069b); };
            c.cmp(t690, c.r32(9), 32);
            if c.cc_e() { blk = 777; continue; }
            blk = 776; continue;
        }
        776 => {
            let Some(t691) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008106a4); };
            c.cmp(t691, 0x43u64, 16);
            if c.cc_ne() { blk = 772; continue; }
            blk = 777; continue;
        }
        777 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008106ac); }
            c.cvtsd2ss(0, c.xmm_u64(0));
            blk = 779; continue;
        }
        778 => {
            let r692 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r692);
            c.xorpd_rr(0, 0);
            c.cvtsi2ss(0, (c.r32(7) as u32 as i32 as i64));
            blk = 779; continue;
        }
        779 => {
            c.w64(7, c.r64(2));
            let Some(t693) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008106c3); };
            c.w32(9, t693);
            let r694 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r694);
            let Some(t695) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008106ca); };
            c.w32(11, t695);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x70008106ce); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 805; continue; }
            blk = 780; continue;
        }
        780 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 781; continue;
        }
        781 => {
            let Some(t696) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008106e7); };
            c.w64(9, t696);
            let Some(t697) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008106eb); };
            c.w64(11, t697);
            let Some(t698) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008106ef); };
            c.logic(t698 & 0x1u64, 8);
            if c.cc_ne() { blk = 784; continue; }
            blk = 782; continue;
        }
        782 => {
            let Some(t699) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008106f5); };
            c.w64(12, t699);
            let r700 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r700);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 783; continue;
        }
        783 => {
            let Some(t701) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000810705); };
            c.w64(9, t701);
            blk = 807; continue;
        }
        784 => {
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 785; continue;
        }
        785 => {
            let Some(t702) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810717); };
            c.w32(12, t702);
            let r703 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r703);
            let Some(t704) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x700081071e); };
            c.w32(12, t704);
            let r705 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r705);
            let r706 = c.r32(12) & 0x3fu64;
            c.w32(12, r706);
            c.logic(r706, 32);
            c.cmp(c.r32(12), 0x21u64, 32);
            if c.cc_l() { blk = 793; continue; }
            blk = 786; continue;
        }
        786 => {
            c.cmp(c.r32(12), 0x24u64, 32);
            if c.cc_l() { blk = 790; continue; }
            blk = 787; continue;
        }
        787 => {
            c.cmp(c.r32(12), 0x24u64, 32);
            if c.cc_e() { blk = 800; continue; }
            blk = 788; continue;
        }
        788 => {
            c.cmp(c.r32(12), 0x25u64, 32);
            if c.cc_e() { blk = 803; continue; }
            blk = 789; continue;
        }
        789 => {
            c.cmp(c.r32(12), 0x26u64, 32);
            if c.cc_e() { blk = 803; continue; }
            blk = 790; continue;
        }
        790 => {
            c.cmp(c.r32(12), 0x21u64, 32);
            if c.cc_e() { blk = 802; continue; }
            blk = 791; continue;
        }
        791 => {
            c.cmp(c.r32(12), 0x22u64, 32);
            if c.cc_e() { blk = 802; continue; }
            blk = 792; continue;
        }
        792 => {
            c.cmp(c.r32(12), 0x23u64, 32);
            if c.cc_e() { blk = 803; continue; }
            blk = 793; continue;
        }
        793 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_l() { blk = 797; continue; }
            blk = 794; continue;
        }
        794 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_e() { blk = 801; continue; }
            blk = 795; continue;
        }
        795 => {
            c.cmp(c.r32(12), 0x1fu64, 32);
            if c.cc_e() { blk = 801; continue; }
            blk = 796; continue;
        }
        796 => {
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_e() { blk = 802; continue; }
            blk = 797; continue;
        }
        797 => {
            c.cmp(c.r32(12), 0x1cu64, 32);
            if c.cc_e() { blk = 800; continue; }
            blk = 798; continue;
        }
        798 => {
            c.cmp(c.r32(12), 0x1du64, 32);
            if c.cc_e() { blk = 800; continue; }
            blk = 799; continue;
        }
        799 => {
            return RegfileExit::fault(0x7000810784);
        }
        800 => {
            c.w32(12, 0x1u64);
            blk = 804; continue;
        }
        801 => {
            c.w32(12, 0x2u64);
            blk = 804; continue;
        }
        802 => {
            c.w32(12, 0x4u64);
            blk = 804; continue;
        }
        803 => {
            c.w32(12, 0x8u64);
            blk = 804; continue;
        }
        804 => {
            let r707 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r707);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(10));
            c.cqo();
            c.idiv(c.r64(12), 64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 807; continue;
        }
        805 => {
            c.w64(10, c.r64(4));
            let r708 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r708);
            let r709 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r709);
            c.logic(r709, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008107ca); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t710) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x70008107d5); };
            c.w64(0, t710);
            c.w64(10, 0x70008107e3u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008107e3); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008107e7); }
            return c.call(c.r64(0), 0x70008107ed, 0x70008107eb);
        }
        806 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008107ed); }
            let Some(t711) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008107f5); };
            c.w64(4, t711);
            c.w64(9, c.r64(0));
            let Some(t712) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008107fd); };
            c.w64(0, t712);
            let Some(t713) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810801); };
            c.w64(7, t713);
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810805); };
            c.w64(8, t714);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x7000810809); }
            blk = 807; continue;
        }
        807 => {
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 808; continue;
        }
        808 => {
            c.w64(2, c.r64(7));
            let Some(t715) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x700081081a); };
            c.w64(7, t715);
            let Some(t716) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x700081081e); };
            c.w32(9, t716);
            let r717 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r717);
            if !c.movss_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)), 0) { return RegfileExit::fault(0x7000810825); }
            blk = 1026; continue;
        }
        809 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 815; continue; }
            blk = 810; continue;
        }
        810 => {
            let Some(t718) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810835); };
            c.w32(8, t718);
            let r719 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r719);
            let Some(t720) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081083c); };
            c.cmp(t720, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 811; continue;
        }
        811 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810848); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 812; continue;
        }
        812 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 813; continue;
        }
        813 => {
            let Some(t721) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x700081086b); };
            c.w64(9, t721);
            let r722 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r722);
            let Some(t723) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x7000810875); };
            c.w64(11, t723);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 816; continue; }
            blk = 814; continue;
        }
        814 => {
            blk = 984; continue;
        }
        815 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 816; continue;
        }
        816 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x700081088f); }
            c.w64(7, c.r64(0));
            blk = 819; continue;
        }
        817 => {
            return c.call(0x70008a1014, 0x700081089d, 0x7000810898);
        }
        818 => {
            c.w64(7, c.r64(0));
            let Some(t724) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008108a0); };
            c.w64(0, t724);
            let Some(t725) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008108a4); };
            c.w64(6, t725);
            let Some(t726) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008108a8); };
            c.w64(2, t726);
            let Some(t727) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008108ac); };
            c.w64(8, t727);
            blk = 819; continue;
        }
        819 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 823; continue; }
            blk = 820; continue;
        }
        820 => {
            let Some(t728) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008108b6); };
            c.w32(9, t728);
            let r729 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r729);
            let Some(t730) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008108bd); };
            c.cmp(t730, c.r32(9), 32);
            if c.cc_e() { blk = 822; continue; }
            blk = 821; continue;
        }
        821 => {
            let Some(t731) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008108c6); };
            c.cmp(t731, 0x43u64, 16);
            if c.cc_ne() { blk = 817; continue; }
            blk = 822; continue;
        }
        822 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x70008108ce); }
            blk = 824; continue;
        }
        823 => {
            let r732 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r732);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            blk = 824; continue;
        }
        824 => {
            c.w64(7, c.r64(2));
            let Some(t733) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008108e2); };
            c.w32(9, t733);
            let r734 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r734);
            let Some(t735) = c.ld32(c.r64(9).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008108e9); };
            c.w32(11, t735);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x70008108ed); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 850; continue; }
            blk = 825; continue;
        }
        825 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 826; continue;
        }
        826 => {
            let Some(t736) = c.ld64(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000810906); };
            c.w64(9, t736);
            let Some(t737) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081090a); };
            c.w64(11, t737);
            let Some(t738) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x700081090e); };
            c.logic(t738 & 0x1u64, 8);
            if c.cc_ne() { blk = 829; continue; }
            blk = 827; continue;
        }
        827 => {
            let Some(t739) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000810914); };
            c.w64(12, t739);
            let r740 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r740);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 828; continue;
        }
        828 => {
            let Some(t741) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000810924); };
            c.w64(9, t741);
            blk = 852; continue;
        }
        829 => {
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 830; continue;
        }
        830 => {
            let Some(t742) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810936); };
            c.w32(12, t742);
            let r743 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r743);
            let Some(t744) = c.ld8(c.r64(12).wrapping_add(0xa)) else { return RegfileExit::fault(0x700081093d); };
            c.w32(12, t744);
            let r745 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r745);
            let r746 = c.r32(12) & 0x3fu64;
            c.w32(12, r746);
            c.logic(r746, 32);
            c.cmp(c.r32(12), 0x21u64, 32);
            if c.cc_l() { blk = 838; continue; }
            blk = 831; continue;
        }
        831 => {
            c.cmp(c.r32(12), 0x24u64, 32);
            if c.cc_l() { blk = 835; continue; }
            blk = 832; continue;
        }
        832 => {
            c.cmp(c.r32(12), 0x24u64, 32);
            if c.cc_e() { blk = 845; continue; }
            blk = 833; continue;
        }
        833 => {
            c.cmp(c.r32(12), 0x25u64, 32);
            if c.cc_e() { blk = 848; continue; }
            blk = 834; continue;
        }
        834 => {
            c.cmp(c.r32(12), 0x26u64, 32);
            if c.cc_e() { blk = 848; continue; }
            blk = 835; continue;
        }
        835 => {
            c.cmp(c.r32(12), 0x21u64, 32);
            if c.cc_e() { blk = 847; continue; }
            blk = 836; continue;
        }
        836 => {
            c.cmp(c.r32(12), 0x22u64, 32);
            if c.cc_e() { blk = 847; continue; }
            blk = 837; continue;
        }
        837 => {
            c.cmp(c.r32(12), 0x23u64, 32);
            if c.cc_e() { blk = 848; continue; }
            blk = 838; continue;
        }
        838 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_l() { blk = 842; continue; }
            blk = 839; continue;
        }
        839 => {
            c.cmp(c.r32(12), 0x1eu64, 32);
            if c.cc_e() { blk = 846; continue; }
            blk = 840; continue;
        }
        840 => {
            c.cmp(c.r32(12), 0x1fu64, 32);
            if c.cc_e() { blk = 846; continue; }
            blk = 841; continue;
        }
        841 => {
            c.cmp(c.r32(12), 0x20u64, 32);
            if c.cc_e() { blk = 847; continue; }
            blk = 842; continue;
        }
        842 => {
            c.cmp(c.r32(12), 0x1cu64, 32);
            if c.cc_e() { blk = 845; continue; }
            blk = 843; continue;
        }
        843 => {
            c.cmp(c.r32(12), 0x1du64, 32);
            if c.cc_e() { blk = 845; continue; }
            blk = 844; continue;
        }
        844 => {
            return RegfileExit::fault(0x70008109a3);
        }
        845 => {
            c.w32(12, 0x1u64);
            blk = 849; continue;
        }
        846 => {
            c.w32(12, 0x2u64);
            blk = 849; continue;
        }
        847 => {
            c.w32(12, 0x4u64);
            blk = 849; continue;
        }
        848 => {
            c.w32(12, 0x8u64);
            blk = 849; continue;
        }
        849 => {
            let r747 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r747);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(10));
            c.cqo();
            c.idiv(c.r64(12), 64);
            c.w64(10, c.r64(9));
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 852; continue;
        }
        850 => {
            c.w64(10, c.r64(4));
            let r748 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r748);
            let r749 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r749);
            c.logic(r749, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008109e9); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t750) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x70008109f4); };
            c.w64(0, t750);
            c.w64(10, 0x7000810a02u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000810a02); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000810a06); }
            return c.call(c.r64(0), 0x7000810a0c, 0x7000810a0a);
        }
        851 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000810a0c); }
            let Some(t751) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000810a14); };
            c.w64(4, t751);
            c.w64(9, c.r64(0));
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810a1c); };
            c.w64(0, t752);
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810a20); };
            c.w64(7, t753);
            let Some(t754) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810a24); };
            c.w64(8, t754);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x7000810a28); }
            blk = 852; continue;
        }
        852 => {
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 853; continue;
        }
        853 => {
            c.w64(2, c.r64(7));
            let Some(t755) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000810a39); };
            c.w64(7, t755);
            let Some(t756) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000810a3d); };
            c.w32(9, t756);
            let r757 = c.add(c.r64(7), c.r64(9), 64);
            c.w64(7, r757);
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)), 0) { return RegfileExit::fault(0x7000810a44); }
            blk = 1026; continue;
        }
        854 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 860; continue; }
            blk = 855; continue;
        }
        855 => {
            let Some(t758) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810a54); };
            c.w32(8, t758);
            let r759 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r759);
            let Some(t760) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000810a5b); };
            c.cmp(t760, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 856; continue;
        }
        856 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810a67); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 857; continue;
        }
        857 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 858; continue;
        }
        858 => {
            let Some(t761) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x7000810a8a); };
            c.w64(9, t761);
            let r762 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r762);
            let Some(t763) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x7000810a94); };
            c.w64(11, t763);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 861; continue; }
            blk = 859; continue;
        }
        859 => {
            blk = 984; continue;
        }
        860 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 861; continue;
        }
        861 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000810aae); }
            c.w64(7, c.r64(0));
            blk = 864; continue;
        }
        862 => {
            return c.call(0x70008a1014, 0x7000810abc, 0x7000810ab7);
        }
        863 => {
            c.w64(7, c.r64(0));
            let Some(t764) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810abf); };
            c.w64(0, t764);
            let Some(t765) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000810ac3); };
            c.w64(6, t765);
            let Some(t766) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810ac7); };
            c.w64(2, t766);
            let Some(t767) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810acb); };
            c.w64(8, t767);
            blk = 864; continue;
        }
        864 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 896; continue; }
            blk = 865; continue;
        }
        865 => {
            let Some(t768) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810ad9); };
            c.w32(9, t768);
            let r769 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r769);
            let Some(t770) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000810ae0); };
            c.cmp(t770, c.r32(9), 32);
            if c.cc_e() { blk = 867; continue; }
            blk = 866; continue;
        }
        866 => {
            let Some(t771) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000810ae9); };
            c.cmp(t771, 0x43u64, 16);
            if c.cc_ne() { blk = 862; continue; }
            blk = 867; continue;
        }
        867 => {
            if !c.movsd_rm(0, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810af1); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 895; continue; }
            blk = 868; continue;
        }
        868 => {
            let Some(t772) = c.ld64(0x700080f4e6u64) else { return RegfileExit::fault(0x7000810b04); };
            c.w64(10, t772);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 894; continue; }
            blk = 869; continue;
        }
        869 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 877; continue; }
            blk = 870; continue;
        }
        870 => {
            let Some(t773) = c.ld64(0x700080f50du64) else { return RegfileExit::fault(0x7000810b28); };
            c.w64(10, t773);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 876; continue; }
            blk = 871; continue;
        }
        871 => {
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 873; continue; }
            blk = 872; continue;
        }
        872 => {
            c.mov128_rr(1, 0);
            blk = 882; continue;
        }
        873 => {
            c.mov128_rr(1, 0);
            let Some(t774) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000810b4f); };
            c.w64(10, t774);
            if !c.xorpd_rm(1, c.r64(10)) { return RegfileExit::fault(0x7000810b56); }
            let Some(t775) = c.ld64(0x700080f542u64) else { return RegfileExit::fault(0x7000810b5a); };
            c.w64(10, t775);
            c.movq_xr(2, 10);
            c.mov128_rr(3, 1);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(3));
            if c.cc_be() { blk = 875; continue; }
            blk = 874; continue;
        }
        874 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(0, 1, c.xmm_u64(3));
            c.mov128_rr(3, 1);
            blk = 875; continue;
        }
        875 => {
            let Some(t776) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x7000810b8c); };
            c.w64(10, t776);
            if !c.xorpd_rm(3, c.r64(10)) { return RegfileExit::fault(0x7000810b93); }
            c.mov128_rr(1, 3);
            blk = 882; continue;
        }
        876 => {
            c.mov128_rr(1, 0);
            blk = 882; continue;
        }
        877 => {
            let Some(t777) = c.ld64(0x700080f542u64) else { return RegfileExit::fault(0x7000810ba1); };
            c.w64(10, t777);
            c.movq_xr(1, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_ae() { blk = 881; continue; }
            blk = 878; continue;
        }
        878 => {
            c.mov128_rr(2, 0);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 880; continue; }
            blk = 879; continue;
        }
        879 => {
            c.mov128_rr(1, 2);
            blk = 882; continue;
        }
        880 => {
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x36);
            c.psrlq(1, 0x2);
            c.arith_sd(1, 2, c.xmm_u64(1));
            c.mov128_rr(1, 2);
            blk = 882; continue;
        }
        881 => {
            c.mov128_rr(1, 0);
            blk = 882; continue;
        }
        882 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x37);
            c.psrlq(2, 0x2);
            c.arith_sd(0, 2, c.xmm_u64(1));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_a() { blk = 890; continue; }
            blk = 883; continue;
        }
        883 => {
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_a() { blk = 889; continue; }
            blk = 884; continue;
        }
        884 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x3f);
            c.psrlq(0, 0x1);
            c.mov128_rr(2, 1);
            c.w64(9, c.r64(0));
            let r778 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r778);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000810c19); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000810c1e); }
            if !c.movsd_mr(c.r64(4), 2) { return RegfileExit::fault(0x7000810c21); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x7000810c26); }
            blk = 885; continue;
        }
        885 => {
            c.fprem();
            c.fnstsw_ax();
            let r779 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r779);
            let r780 = c.r32(0) & 0xffu64;
            c.w32(0, r780);
            c.logic(r780, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000810c35); }
            if !c.popfq() { return RegfileExit::fault(0x7000810c36); }
            if c.cc_p() { blk = 885; continue; }
            blk = 886; continue;
        }
        886 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x7000810c3b); }
            if !c.movsd_rm(2, c.r64(4)) { return RegfileExit::fault(0x7000810c3e); }
            let r781 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r781);
            c.xorpd_rr(0, 0);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_p() { blk = 891; continue; }
            blk = 887; continue;
        }
        887 => {
            if c.cc_e() { blk = 892; continue; }
            blk = 888; continue;
        }
        888 => {
            blk = 891; continue;
        }
        889 => {
            c.w64(9, c.r64(0));
            blk = 892; continue;
        }
        890 => {
            c.w64(9, c.r64(0));
            blk = 891; continue;
        }
        891 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(1));
            c.mov128_rr(1, 0);
            blk = 892; continue;
        }
        892 => {
            let cv = c.cvttsd2si64(c.xmm_u64(1)); c.w64(7, cv);
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_o() { blk = 1079; continue; }
            blk = 893; continue;
        }
        893 => {
            c.w32(7, c.r32(7));
            blk = 900; continue;
        }
        894 => {
            c.w64(9, c.r64(0));
            c.w32(7, 0xffu64);
            blk = 900; continue;
        }
        895 => {
            c.w64(9, c.r64(0));
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 900; continue;
        }
        896 => {
            let r782 = c.sar(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r782);
            c.w64(9, c.r64(0));
            c.cmp(c.r32(7), 0xffu64, 32);
            if c.cc_be() { blk = 900; continue; }
            blk = 897; continue;
        }
        897 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 899; continue; }
            blk = 898; continue;
        }
        898 => {
            c.w32(7, 0xffu64);
            blk = 900; continue;
        }
        899 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 900; continue;
        }
        900 => {
            c.w64(11, c.r64(2));
            let Some(t783) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000810cb4); };
            c.w32(12, t783);
            let r784 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r784);
            let Some(t785) = c.ld32(c.r64(12).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000810cbb); };
            c.w32(15, t785);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000810cc0); }
            c.logic(c.r8(15) & 0x10u64, 8);
            if c.cc_ne() { blk = 926; continue; }
            blk = 901; continue;
        }
        901 => {
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_ne() { blk = 930; continue; }
            blk = 902; continue;
        }
        902 => {
            let Some(t786) = c.ld64(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000810cd8); };
            c.w64(12, t786);
            let Some(t787) = c.ld64(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000810cdd); };
            c.w64(15, t787);
            let Some(t788) = c.ld8(c.r64(11).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000810ce1); };
            c.logic(t788 & 0x1u64, 8);
            if c.cc_ne() { blk = 905; continue; }
            blk = 903; continue;
        }
        903 => {
            let Some(t789) = c.ld64(c.r64(11).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000810ce8); };
            c.w64(0, t789);
            let r790 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r790);
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 930; continue; }
            blk = 904; continue;
        }
        904 => {
            let Some(t791) = c.ld64(c.r64(11).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000810cf8); };
            c.w64(12, t791);
            c.w64(0, c.r64(12));
            blk = 928; continue;
        }
        905 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_a() { blk = 930; continue; }
            blk = 906; continue;
        }
        906 => {
            let Some(t792) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810d0d); };
            c.w32(0, t792);
            let r793 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r793);
            let Some(t794) = c.ld8(c.r64(0).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000810d14); };
            c.w32(0, t794);
            let r795 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r795);
            let r796 = c.r32(0) & 0x3fu64;
            c.w32(0, r796);
            c.logic(r796, 32);
            c.cmp(c.r32(0), 0x21u64, 32);
            if c.cc_l() { blk = 914; continue; }
            blk = 907; continue;
        }
        907 => {
            c.cmp(c.r32(0), 0x24u64, 32);
            if c.cc_l() { blk = 911; continue; }
            blk = 908; continue;
        }
        908 => {
            c.cmp(c.r32(0), 0x24u64, 32);
            if c.cc_e() { blk = 921; continue; }
            blk = 909; continue;
        }
        909 => {
            c.cmp(c.r32(0), 0x25u64, 32);
            if c.cc_e() { blk = 924; continue; }
            blk = 910; continue;
        }
        910 => {
            c.cmp(c.r32(0), 0x26u64, 32);
            if c.cc_e() { blk = 924; continue; }
            blk = 911; continue;
        }
        911 => {
            c.cmp(c.r32(0), 0x21u64, 32);
            if c.cc_e() { blk = 923; continue; }
            blk = 912; continue;
        }
        912 => {
            c.cmp(c.r32(0), 0x22u64, 32);
            if c.cc_e() { blk = 923; continue; }
            blk = 913; continue;
        }
        913 => {
            c.cmp(c.r32(0), 0x23u64, 32);
            if c.cc_e() { blk = 924; continue; }
            blk = 914; continue;
        }
        914 => {
            c.cmp(c.r32(0), 0x1eu64, 32);
            if c.cc_l() { blk = 918; continue; }
            blk = 915; continue;
        }
        915 => {
            c.cmp(c.r32(0), 0x1eu64, 32);
            if c.cc_e() { blk = 922; continue; }
            blk = 916; continue;
        }
        916 => {
            c.cmp(c.r32(0), 0x1fu64, 32);
            if c.cc_e() { blk = 922; continue; }
            blk = 917; continue;
        }
        917 => {
            c.cmp(c.r32(0), 0x20u64, 32);
            if c.cc_e() { blk = 923; continue; }
            blk = 918; continue;
        }
        918 => {
            c.cmp(c.r32(0), 0x1cu64, 32);
            if c.cc_e() { blk = 921; continue; }
            blk = 919; continue;
        }
        919 => {
            c.cmp(c.r32(0), 0x1du64, 32);
            if c.cc_e() { blk = 921; continue; }
            blk = 920; continue;
        }
        920 => {
            return RegfileExit::fault(0x7000810d68);
        }
        921 => {
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(0));
            blk = 925; continue;
        }
        922 => {
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(0));
            blk = 925; continue;
        }
        923 => {
            c.w32(0, 0x4u64);
            c.w64(3, c.r64(0));
            blk = 925; continue;
        }
        924 => {
            c.w32(0, 0x8u64);
            c.w64(3, c.r64(0));
            blk = 925; continue;
        }
        925 => {
            let r797 = c.sub(c.r64(12), c.r64(15), 64);
            c.w64(12, r797);
            c.w64(0, c.r64(12));
            c.cqo();
            c.idiv(c.r64(3), 64);
            blk = 928; continue;
        }
        926 => {
            c.w64(10, c.r64(4));
            let r798 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r798);
            let r799 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r799);
            c.logic(r799, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000810da7); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(11));
            let Some(t800) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000810db2); };
            c.w64(0, t800);
            c.w64(10, 0x7000810dc0u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000810dc0); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000810dc4); }
            return c.call(c.r64(0), 0x7000810dca, 0x7000810dc8);
        }
        927 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000810dca); }
            let Some(t801) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000810dd2); };
            c.w64(4, t801);
            let Some(t802) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000810dd7); };
            c.w64(7, t802);
            let Some(t803) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810ddb); };
            c.w64(9, t803);
            let Some(t804) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810ddf); };
            c.w64(11, t804);
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810de3); };
            c.w64(8, t805);
            blk = 928; continue;
        }
        928 => {
            c.cmp(c.r64(8), c.r64(0), 64);
            if c.cc_ae() { blk = 930; continue; }
            blk = 929; continue;
        }
        929 => {
            c.w64(2, c.r64(11));
            let Some(t806) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000810def); };
            c.w64(11, t806);
            let Some(t807) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000810df3); };
            c.w32(12, t807);
            let r808 = c.add(c.r64(11), c.r64(12), 64);
            c.w64(11, r808);
            if !c.st8(c.r64(8).wrapping_add(c.r64(11)), c.r8(7)) { return RegfileExit::fault(0x7000810dfa); }
            blk = 930; continue;
        }
        930 => {
            c.w64(0, c.r64(9));
            blk = 1026; continue;
        }
        931 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 937; continue; }
            blk = 932; continue;
        }
        932 => {
            let Some(t809) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810e0b); };
            c.w32(8, t809);
            let r810 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r810);
            let Some(t811) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000810e12); };
            c.cmp(t811, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 933; continue;
        }
        933 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000810e1e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 934; continue;
        }
        934 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 935; continue;
        }
        935 => {
            let Some(t812) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x7000810e41); };
            c.w64(9, t812);
            let r813 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r813);
            let Some(t814) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x7000810e4b); };
            c.w64(11, t814);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 938; continue; }
            blk = 936; continue;
        }
        936 => {
            blk = 984; continue;
        }
        937 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 938; continue;
        }
        938 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x7000810e65); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 977; continue; }
            blk = 939; continue;
        }
        939 => {
            let Some(t815) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810e71); };
            c.w32(7, t815);
            let r816 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r816);
            let Some(t817) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000810e77); };
            c.cmp(t817, 0x41u64, 16);
            if c.cc_e() { blk = 942; continue; }
            blk = 940; continue;
        }
        940 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000810e7e); }
            let Some(t818) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x7000810e7f); };
            c.w64(3, t818);
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x7000810e93, 0x7000810e8e);
        }
        941 => {
            c.w64(7, c.r64(0));
            let Some(t819) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810e96); };
            c.w64(0, t819);
            let Some(t820) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810e9a); };
            c.w64(2, t820);
            let Some(t821) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810e9e); };
            c.w64(8, t821);
            blk = 943; continue;
        }
        942 => {
            c.w64(7, c.r64(0));
            blk = 943; continue;
        }
        943 => {
            c.w64(9, c.r64(2));
            let Some(t822) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000810eaa); };
            c.w32(11, t822);
            let r823 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r823);
            let Some(t824) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000810eb1); };
            c.w32(12, t824);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000810eb5); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 969; continue; }
            blk = 944; continue;
        }
        944 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 945; continue;
        }
        945 => {
            let Some(t825) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000810ecd); };
            c.w64(11, t825);
            let Some(t826) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000810ed1); };
            c.w64(12, t826);
            let Some(t827) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000810ed5); };
            c.logic(t827 & 0x1u64, 8);
            if c.cc_ne() { blk = 948; continue; }
            blk = 946; continue;
        }
        946 => {
            let Some(t828) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000810edc); };
            c.w64(15, t828);
            let r829 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r829);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 947; continue;
        }
        947 => {
            let Some(t830) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000810eec); };
            c.w64(11, t830);
            blk = 971; continue;
        }
        948 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 949; continue;
        }
        949 => {
            let Some(t831) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000810efe); };
            c.w32(15, t831);
            let r832 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r832);
            let Some(t833) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000810f05); };
            c.w32(15, t833);
            let r834 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r834);
            let r835 = c.r32(15) & 0x3fu64;
            c.w32(15, r835);
            c.logic(r835, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 957; continue; }
            blk = 950; continue;
        }
        950 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 954; continue; }
            blk = 951; continue;
        }
        951 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 964; continue; }
            blk = 952; continue;
        }
        952 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 967; continue; }
            blk = 953; continue;
        }
        953 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 967; continue; }
            blk = 954; continue;
        }
        954 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 966; continue; }
            blk = 955; continue;
        }
        955 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 966; continue; }
            blk = 956; continue;
        }
        956 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 967; continue; }
            blk = 957; continue;
        }
        957 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 961; continue; }
            blk = 958; continue;
        }
        958 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 965; continue; }
            blk = 959; continue;
        }
        959 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 965; continue; }
            blk = 960; continue;
        }
        960 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 966; continue; }
            blk = 961; continue;
        }
        961 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 964; continue; }
            blk = 962; continue;
        }
        962 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 964; continue; }
            blk = 963; continue;
        }
        963 => {
            return RegfileExit::fault(0x7000810f6a);
        }
        964 => {
            c.w32(15, 0x1u64);
            blk = 968; continue;
        }
        965 => {
            c.w32(15, 0x2u64);
            blk = 968; continue;
        }
        966 => {
            c.w32(15, 0x4u64);
            blk = 968; continue;
        }
        967 => {
            c.w32(15, 0x8u64);
            blk = 968; continue;
        }
        968 => {
            let r836 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r836);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 971; continue;
        }
        969 => {
            c.w64(10, c.r64(4));
            let r837 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r837);
            let r838 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r838);
            c.logic(r838, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000810fb0); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(9));
            let Some(t839) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000810fbb); };
            c.w64(0, t839);
            c.w64(10, 0x7000810fc9u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000810fc9); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000810fcd); }
            return c.call(c.r64(0), 0x7000810fd3, 0x7000810fd1);
        }
        970 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000810fd3); }
            let Some(t840) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000810fdb); };
            c.w64(4, t840);
            c.w64(11, c.r64(0));
            let Some(t841) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000810fe3); };
            c.w64(0, t841);
            let Some(t842) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000810fe7); };
            c.w64(9, t842);
            let Some(t843) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000810feb); };
            c.w64(8, t843);
            let Some(t844) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000810fef); };
            c.w64(7, t844);
            blk = 971; continue;
        }
        971 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 972; continue;
        }
        972 => {
            c.w64(2, c.r64(9));
            let Some(t845) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000810fff); };
            c.w64(9, t845);
            let Some(t846) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000811003); };
            c.w32(11, t846);
            let Some(t847) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811007); };
            c.w32(12, t847);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 975; continue; }
            blk = 973; continue;
        }
        973 => {
            let Some(t848) = c.ld64(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811014); };
            c.w64(7, t848);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 976; continue; }
            blk = 974; continue;
        }
        974 => {
            let r849 = c.neg(c.r64(7), 64);
            c.w64(7, r849);
            blk = 976; continue;
        }
        975 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 976; continue;
        }
        976 => {
            let r850 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r850);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(7)) { return RegfileExit::fault(0x7000811028); }
            blk = 1026; continue;
        }
        977 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000811036); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000811037); }
            let Some(t851) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000811038); };
            c.w64(3, t851);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000811049, 0x7000811044);
        }
        978 => {
            return RegfileExit::fault(0x7000811049);
        }
        979 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 986; continue; }
            blk = 980; continue;
        }
        980 => {
            let Some(t852) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081104f); };
            c.w32(8, t852);
            let r853 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r853);
            let Some(t854) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811056); };
            c.cmp(t854, 0x42u64, 16);
            if c.cc_ne() { blk = 984; continue; }
            blk = 981; continue;
        }
        981 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081105e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 984; continue; }
            blk = 982; continue;
        }
        982 => {
            if c.cc_ne() { blk = 984; continue; }
            blk = 983; continue;
        }
        983 => {
            let Some(t855) = c.ld64(0x700080e451u64) else { return RegfileExit::fault(0x7000811079); };
            c.w64(9, t855);
            let r856 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r856);
            let Some(t857) = c.ld64(0x700080e45eu64) else { return RegfileExit::fault(0x7000811083); };
            c.w64(11, t857);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_be() { blk = 987; continue; }
            blk = 984; continue;
        }
        984 => {
            let Some(t858) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700081108f); };
            c.w64(5, t858);
            let r859 = c.add(c.r64(4), 0x50u64, 64);
            c.w64(4, r859);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000811097); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000811098); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000811099); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081109a); }
            let Some(t860) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081109b); };
            if !c.push64(t860) { return RegfileExit::fault(0x700081109b); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x700081109f); }
            c.w32(0, 0x5u64);
            let Some(t861) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x70008110a9); };
            c.w64(3, t861);
            return RegfileExit::jump(0x700086ed74);
        }
        985 => {
            blk = 987; continue;
        }
        986 => {
            c.w64(7, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(7));
            blk = 987; continue;
        }
        987 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008110c0); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1027; continue; }
            blk = 988; continue;
        }
        988 => {
            let Some(t862) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008110cc); };
            c.w32(7, t862);
            let r863 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r863);
            let Some(t864) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008110d2); };
            c.cmp(t864, 0x41u64, 16);
            if c.cc_e() { blk = 991; continue; }
            blk = 989; continue;
        }
        989 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008110d9); }
            let Some(t865) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008110da); };
            c.w64(3, t865);
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008110ee, 0x70008110e9);
        }
        990 => {
            c.w64(7, c.r64(0));
            let Some(t866) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008110f1); };
            c.w64(0, t866);
            let Some(t867) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008110f5); };
            c.w64(8, t867);
            let Some(t868) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008110f9); };
            c.w64(2, t868);
            blk = 992; continue;
        }
        991 => {
            c.w64(7, c.r64(0));
            blk = 992; continue;
        }
        992 => {
            c.w64(9, c.r64(2));
            let Some(t869) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000811105); };
            c.w32(11, t869);
            let r870 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r870);
            let Some(t871) = c.ld32(c.r64(11).wrapping_add(0x2b)) else { return RegfileExit::fault(0x700081110c); };
            c.w32(12, t871);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000811110); }
            c.logic(c.r8(12) & 0x10u64, 8);
            if c.cc_ne() { blk = 1018; continue; }
            blk = 993; continue;
        }
        993 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_ne() { blk = 1026; continue; }
            blk = 994; continue;
        }
        994 => {
            let Some(t872) = c.ld64(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000811128); };
            c.w64(11, t872);
            let Some(t873) = c.ld64(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081112c); };
            c.w64(12, t873);
            let Some(t874) = c.ld8(c.r64(9).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000811130); };
            c.logic(t874 & 0x1u64, 8);
            if c.cc_ne() { blk = 997; continue; }
            blk = 995; continue;
        }
        995 => {
            let Some(t875) = c.ld64(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000811137); };
            c.w64(15, t875);
            let r876 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r876);
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 996; continue;
        }
        996 => {
            let Some(t877) = c.ld64(c.r64(9).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000811147); };
            c.w64(11, t877);
            blk = 1020; continue;
        }
        997 => {
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_a() { blk = 1026; continue; }
            blk = 998; continue;
        }
        998 => {
            let Some(t878) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811159); };
            c.w32(15, t878);
            let r879 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r879);
            let Some(t880) = c.ld8(c.r64(15).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000811160); };
            c.w32(15, t880);
            let r881 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r881);
            let r882 = c.r32(15) & 0x3fu64;
            c.w32(15, r882);
            c.logic(r882, 32);
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_l() { blk = 1006; continue; }
            blk = 999; continue;
        }
        999 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_l() { blk = 1003; continue; }
            blk = 1000; continue;
        }
        1000 => {
            c.cmp(c.r32(15), 0x24u64, 32);
            if c.cc_e() { blk = 1013; continue; }
            blk = 1001; continue;
        }
        1001 => {
            c.cmp(c.r32(15), 0x25u64, 32);
            if c.cc_e() { blk = 1016; continue; }
            blk = 1002; continue;
        }
        1002 => {
            c.cmp(c.r32(15), 0x26u64, 32);
            if c.cc_e() { blk = 1016; continue; }
            blk = 1003; continue;
        }
        1003 => {
            c.cmp(c.r32(15), 0x21u64, 32);
            if c.cc_e() { blk = 1015; continue; }
            blk = 1004; continue;
        }
        1004 => {
            c.cmp(c.r32(15), 0x22u64, 32);
            if c.cc_e() { blk = 1015; continue; }
            blk = 1005; continue;
        }
        1005 => {
            c.cmp(c.r32(15), 0x23u64, 32);
            if c.cc_e() { blk = 1016; continue; }
            blk = 1006; continue;
        }
        1006 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_l() { blk = 1010; continue; }
            blk = 1007; continue;
        }
        1007 => {
            c.cmp(c.r32(15), 0x1eu64, 32);
            if c.cc_e() { blk = 1014; continue; }
            blk = 1008; continue;
        }
        1008 => {
            c.cmp(c.r32(15), 0x1fu64, 32);
            if c.cc_e() { blk = 1014; continue; }
            blk = 1009; continue;
        }
        1009 => {
            c.cmp(c.r32(15), 0x20u64, 32);
            if c.cc_e() { blk = 1015; continue; }
            blk = 1010; continue;
        }
        1010 => {
            c.cmp(c.r32(15), 0x1cu64, 32);
            if c.cc_e() { blk = 1013; continue; }
            blk = 1011; continue;
        }
        1011 => {
            c.cmp(c.r32(15), 0x1du64, 32);
            if c.cc_e() { blk = 1013; continue; }
            blk = 1012; continue;
        }
        1012 => {
            return RegfileExit::fault(0x70008111c5);
        }
        1013 => {
            c.w32(15, 0x1u64);
            blk = 1017; continue;
        }
        1014 => {
            c.w32(15, 0x2u64);
            blk = 1017; continue;
        }
        1015 => {
            c.w32(15, 0x4u64);
            blk = 1017; continue;
        }
        1016 => {
            c.w32(15, 0x8u64);
            blk = 1017; continue;
        }
        1017 => {
            let r883 = c.sub(c.r64(11), c.r64(12), 64);
            c.w64(11, r883);
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(11));
            c.w64(11, c.r64(10));
            c.cqo();
            c.idiv(c.r64(15), 64);
            c.w64(10, c.r64(11));
            c.w64(11, c.r64(0));
            c.w64(0, c.r64(10));
            blk = 1020; continue;
        }
        1018 => {
            c.w64(10, c.r64(4));
            let r884 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r884);
            let r885 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r885);
            c.logic(r885, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700081120b); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(9));
            let Some(t886) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000811216); };
            c.w64(0, t886);
            c.w64(10, 0x7000811224u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000811224); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000811228); }
            return c.call(c.r64(0), 0x700081122e, 0x700081122c);
        }
        1019 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700081122e); }
            let Some(t887) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000811236); };
            c.w64(4, t887);
            c.w64(11, c.r64(0));
            let Some(t888) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081123e); };
            c.w64(0, t888);
            let Some(t889) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000811242); };
            c.w64(8, t889);
            let Some(t890) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000811246); };
            c.w64(7, t890);
            let Some(t891) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081124a); };
            c.w64(9, t891);
            blk = 1020; continue;
        }
        1020 => {
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_ae() { blk = 1026; continue; }
            blk = 1021; continue;
        }
        1021 => {
            let Some(t892) = c.ld64(c.r64(9).wrapping_add(0x27)) else { return RegfileExit::fault(0x7000811253); };
            c.w64(11, t892);
            let Some(t893) = c.ld32(c.r64(9).wrapping_add(0x2f)) else { return RegfileExit::fault(0x7000811257); };
            c.w32(9, t893);
            let Some(t894) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081125b); };
            c.w32(12, t894);
            c.logic(c.r32(12) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 1024; continue; }
            blk = 1022; continue;
        }
        1022 => {
            let Some(t895) = c.ld64(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811268); };
            c.w64(7, t895);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 1025; continue; }
            blk = 1023; continue;
        }
        1023 => {
            let r896 = c.neg(c.r64(7), 64);
            c.w64(7, r896);
            blk = 1025; continue;
        }
        1024 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 1025; continue;
        }
        1025 => {
            let r897 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r897);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(7)) { return RegfileExit::fault(0x700081127c); }
            blk = 1026; continue;
        }
        1026 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000811283); }
            return c.ret(0x0, 0x7000811284);
        }
        1027 => {
            c.w32(7, 0x28u64);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081128a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081128b); }
            let Some(t898) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x700081128c); };
            c.w64(3, t898);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700081129d, 0x7000811298);
        }
        1028 => {
            return RegfileExit::fault(0x700081129d);
        }
        1029 => {
            return RegfileExit::fault(0x700081129e);
        }
        1030 => {
            return RegfileExit::fault(0x700081129f);
        }
        1031 => {
            return RegfileExit::fault(0x70008112a0);
        }
        1032 => {
            return RegfileExit::fault(0x70008112a1);
        }
        1033 => {
            return RegfileExit::fault(0x70008112a2);
        }
        1034 => {
            return RegfileExit::fault(0x70008112a3);
        }
        1035 => {
            return RegfileExit::fault(0x70008112a4);
        }
        1036 => {
            return RegfileExit::fault(0x70008112a5);
        }
        1037 => {
            return RegfileExit::fault(0x70008112a6);
        }
        1038 => {
            return RegfileExit::fault(0x70008112a7);
        }
        1039 => {
            return RegfileExit::fault(0x70008112a8);
        }
        1040 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t899) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008112ac); };
            c.w64(3, t899);
            let Some(t900) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008112b0); };
            c.w64(7, t900);
            blk = 984; continue;
        }
        1041 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(8));
            let Some(t901) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008112bf); };
            c.w64(7, t901);
            let Some(t902) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008112c3); };
            c.w64(3, t902);
            blk = 984; continue;
        }
        1042 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(8));
            let Some(t903) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008112d2); };
            c.w64(7, t903);
            let Some(t904) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008112d6); };
            c.w64(3, t904);
            blk = 984; continue;
        }
        1043 => {
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            c.w64(1, c.r64(8));
            let Some(t905) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008112e5); };
            c.w64(7, t905);
            let Some(t906) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008112e9); };
            c.w64(3, t906);
            blk = 984; continue;
        }
        1044 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008112f2); }
            c.cmp(c.r32(7), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 1046; continue; }
            blk = 1045; continue;
        }
        1045 => {
            c.w32(7, c.r32(7));
            c.w32(0, c.r64(7).wrapping_add(c.r64(7)));
            blk = 1051; continue;
        }
        1046 => {
            let Some(t907) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811305); };
            c.w64(9, t907);
            let Some(t908) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700081130c); };
            c.w64(11, t908);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t909) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000811313); };
            c.w64(15, t909);
            let Some(t910) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081131a); };
            c.cmp(t910, c.r64(12), 64);
            if c.cc_a() { blk = 1049; continue; }
            blk = 1047; continue;
        }
        1047 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x700081132c, 0x7000811327);
        }
        1048 => {
            let Some(t911) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081132c); };
            c.w64(7, t911);
            let Some(t912) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000811330); };
            c.w64(6, t912);
            let Some(t913) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000811334); };
            c.w64(2, t913);
            let Some(t914) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000811338); };
            c.w64(8, t914);
            blk = 1050; continue;
        }
        1049 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x700081133e); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1050; continue;
        }
        1050 => {
            c.w32(10, c.r32(7));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t915) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000811354); };
            c.w64(7, t915);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700081135b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081135e); }
            blk = 1051; continue;
        }
        1051 => {
            c.w64(1, c.r64(8));
            let Some(t916) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000811366); };
            c.w64(7, t916);
            let Some(t917) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081136a); };
            c.w64(3, t917);
            blk = 984; continue;
        }
        1052 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000811373); }
            c.w64(9, c.r64(7));
            let r918 = c.add(c.r32(9), c.r32(7), 32);
            c.w32(9, r918);
            if c.cc_o() { blk = 1054; continue; }
            blk = 1053; continue;
        }
        1053 => {
            c.w64(0, (c.r32(9) as u32 as i32 as i64 as u64));
            blk = 1059; continue;
        }
        1054 => {
            let Some(t919) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811384); };
            c.w64(9, t919);
            let Some(t920) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700081138b); };
            c.w64(11, t920);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t921) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000811392); };
            c.w64(15, t921);
            let Some(t922) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000811399); };
            c.cmp(t922, c.r64(12), 64);
            if c.cc_a() { blk = 1057; continue; }
            blk = 1055; continue;
        }
        1055 => {
            c.w32(2, 0xcu64);
            c.w64(9, c.r64(2));
            return c.call(0x70007febf4, 0x70008113ab, 0x70008113a6);
        }
        1056 => {
            let Some(t923) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008113ab); };
            c.w64(7, t923);
            let Some(t924) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008113af); };
            c.w64(6, t924);
            let Some(t925) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008113b3); };
            c.w64(2, t925);
            let Some(t926) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008113b7); };
            c.w64(8, t926);
            blk = 1058; continue;
        }
        1057 => {
            if !c.st64(c.r64(9), c.r64(12)) { return RegfileExit::fault(0x70008113bd); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1058; continue;
        }
        1058 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(7) as u32 as i32 as i64));
            let Some(t927) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008113cf); };
            c.w64(7, t927);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008113d6); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008113d9); }
            blk = 1059; continue;
        }
        1059 => {
            c.w64(1, c.r64(8));
            let Some(t928) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008113e1); };
            c.w64(7, t928);
            let Some(t929) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008113e5); };
            c.w64(3, t929);
            blk = 984; continue;
        }
        1060 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x70008113ee); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1067; continue; }
            blk = 1061; continue;
        }
        1061 => {
            let Some(t930) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008113f7); };
            c.w32(7, t930);
            let Some(t931) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008113fa); };
            c.cmp(t931, c.r32(7), 32);
            if c.cc_e() { blk = 1067; continue; }
            blk = 1062; continue;
        }
        1062 => {
            let Some(t932) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811403); };
            c.w64(7, t932);
            let Some(t933) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x700081140a); };
            c.w64(9, t933);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t934) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000811411); };
            c.w64(12, t934);
            let Some(t935) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000811418); };
            c.cmp(t935, c.r64(11), 64);
            if c.cc_a() { blk = 1065; continue; }
            blk = 1063; continue;
        }
        1063 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x700081142b, 0x7000811426);
        }
        1064 => {
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x700081142b); }
            let Some(t936) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000811430); };
            c.w64(6, t936);
            let Some(t937) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000811434); };
            c.w64(2, t937);
            let Some(t938) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000811438); };
            c.w64(8, t938);
            blk = 1066; continue;
        }
        1065 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x700081143e); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 1066; continue;
        }
        1066 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t939) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081144c); };
            c.w64(7, t939);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000811453); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000811456); }
            blk = 1067; continue;
        }
        1067 => {
            c.w64(1, c.r64(8));
            let Some(t940) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081145e); };
            c.w64(7, t940);
            let Some(t941) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000811462); };
            c.w64(3, t941);
            blk = 984; continue;
        }
        1068 => {
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x40), 0) { return RegfileExit::fault(0x700081146b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1075; continue; }
            blk = 1069; continue;
        }
        1069 => {
            let Some(t942) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811474); };
            c.w32(7, t942);
            let Some(t943) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000811477); };
            c.cmp(t943, c.r32(7), 32);
            if c.cc_e() { blk = 1075; continue; }
            blk = 1070; continue;
        }
        1070 => {
            let Some(t944) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811480); };
            c.w64(7, t944);
            let Some(t945) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000811487); };
            c.w64(9, t945);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t946) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081148e); };
            c.w64(12, t946);
            let Some(t947) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000811495); };
            c.cmp(t947, c.r64(11), 64);
            if c.cc_a() { blk = 1073; continue; }
            blk = 1071; continue;
        }
        1071 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x70008114a8, 0x70008114a3);
        }
        1072 => {
            let Some(t948) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008114a8); };
            c.w64(8, t948);
            let Some(t949) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008114ac); };
            c.w64(6, t949);
            let Some(t950) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008114b0); };
            c.w64(2, t950);
            blk = 1074; continue;
        }
        1073 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008114b6); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            c.w64(0, c.r64(7));
            blk = 1074; continue;
        }
        1074 => {
            let Some(t951) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008114c0); };
            c.w64(7, t951);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008114c7); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x40)) { return RegfileExit::fault(0x70008114ca); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008114cf); }
            blk = 1075; continue;
        }
        1075 => {
            c.w64(1, c.r64(8));
            let Some(t952) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008114d7); };
            c.w64(7, t952);
            let Some(t953) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008114db); };
            c.w64(3, t953);
            blk = 984; continue;
        }
        1076 => {
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            c.w64(1, c.r64(8));
            let Some(t954) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008114eb); };
            c.w64(7, t954);
            let Some(t955) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008114ef); };
            c.w64(3, t955);
            blk = 984; continue;
        }
        1077 => {
            c.w64(0, c.r64(11));
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        1078 => {
            c.w64(0, c.r64(11));
            c.w64(1, c.r64(8));
            blk = 984; continue;
        }
        1079 => {
            let r956 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r956);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x7000811512); }
            return c.call(0x70008708b4, 0x700081151c, 0x7000811517);
        }
        1080 => {
            let Some(t957) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700081151c); };
            c.w32(7, t957);
            let r958 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r958);
            blk = 893; continue;
        }
        1081 => {
            let r959 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r959);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700081152c); }
            return c.call(0x70008708b4, 0x7000811536, 0x7000811531);
        }
        1082 => {
            let Some(t960) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000811536); };
            c.w32(7, t960);
            let r961 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r961);
            blk = 732; continue;
        }
        1083 => {
            let r962 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r962);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000811546); }
            return c.call(0x70008708b4, 0x7000811550, 0x700081154b);
        }
        1084 => {
            let Some(t963) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000811550); };
            c.w32(7, t963);
            let r964 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r964);
            blk = 686; continue;
        }
        1085 => {
            let r965 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r965);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000811560); }
            return c.call(0x70008708b4, 0x700081156a, 0x7000811565);
        }
        1086 => {
            let Some(t966) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700081156a); };
            c.w32(7, t966);
            let r967 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r967);
            blk = 640; continue;
        }
        1087 => {
            let r968 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r968);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700081157a); }
            return c.call(0x70008708b4, 0x7000811584, 0x700081157f);
        }
        1088 => {
            let Some(t969) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000811584); };
            c.w32(7, t969);
            let r970 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r970);
            blk = 594; continue;
        }
        1089 => {
            let r971 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r971);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000811594); }
            return c.call(0x70008708b4, 0x700081159e, 0x7000811599);
        }
        1090 => {
            let Some(t972) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700081159e); };
            c.w32(7, t972);
            let r973 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r973);
            blk = 548; continue;
        }
        1091 => {
            let r974 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r974);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008115ae); }
            return c.call(0x70008708b4, 0x70008115b8, 0x70008115b3);
        }
        1092 => {
            let Some(t975) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008115b8); };
            c.w32(7, t975);
            let r976 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r976);
            blk = 502; continue;
        }
        1093 => {
            let r977 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r977);
            if !c.movsd_mr(c.r64(4), 1) { return RegfileExit::fault(0x70008115c8); }
            return c.call(0x70008708b4, 0x70008115d2, 0x70008115cd);
        }
        1094 => {
            let Some(t978) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008115d2); };
            c.w32(11, t978);
            let r979 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r979);
            blk = 430; continue;
        }
        1095 => {
            let r980 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r980);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008115e3); }
            return c.call(0x70008708b4, 0x70008115ed, 0x70008115e8);
        }
        1096 => {
            let Some(t981) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008115ed); };
            c.w32(7, t981);
            let r982 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r982);
            blk = 344; continue;
        }
        1097 => {
            let r983 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r983);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008115fd); }
            return c.call(0x70008708b4, 0x7000811607, 0x7000811602);
        }
        1098 => {
            let Some(t984) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000811607); };
            c.w32(7, t984);
            let r985 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r985);
            blk = 323; continue;
        }
        1099 => {
            let r986 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r986);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000811617); }
            return c.call(0x70008708b4, 0x7000811621, 0x700081161c);
        }
        1100 => {
            let Some(t987) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000811621); };
            c.w32(7, t987);
            let r988 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r988);
            blk = 302; continue;
        }
        1101 => {
            let r989 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r989);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000811631); }
            return c.call(0x70008708b4, 0x700081163b, 0x7000811636);
        }
        1102 => {
            let Some(t990) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700081163b); };
            c.w32(7, t990);
            let r991 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r991);
            blk = 281; continue;
        }
        1103 => {
            let r992 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r992);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x700081164b); }
            return c.call(0x70008708b4, 0x7000811655, 0x7000811650);
        }
        1104 => {
            let Some(t993) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x7000811655); };
            c.w32(7, t993);
            let r994 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r994);
            blk = 260; continue;
        }
        1105 => {
            let r995 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r995);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x7000811665); }
            return c.call(0x70008708b4, 0x700081166f, 0x700081166a);
        }
        1106 => {
            let Some(t996) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x700081166f); };
            c.w32(7, t996);
            let r997 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r997);
            blk = 239; continue;
        }
        1107 => {
            c.w32(0, c.r32(0));
            let r998 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r998);
            c.w64(7, 0xfffffffffffc0000u64);
            let r999 = c.r64(7) & c.r64(0);
            c.w64(7, r999);
            c.logic(r999, 64);
            let Some(t1000) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081168a); };
            c.logic(t1000 & 0x2u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 1108; continue;
        }
        1108 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000811698); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000811699); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008116a6, 0x70008116a1);
        }
        1109 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008116a6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008116a8); }
            blk = 190; continue;
        }
        1110 => {
            c.w32(0, c.r32(0));
            let r1001 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1001);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1002 = c.r64(7) & c.r64(0);
            c.w64(7, r1002);
            c.logic(r1002, 64);
            let Some(t1003) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008116bd); };
            c.logic(t1003 & 0x2u64, 8);
            if c.cc_e() { blk = 173; continue; }
            blk = 1111; continue;
        }
        1111 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008116cb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008116cc); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008116d9, 0x70008116d4);
        }
        1112 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008116d9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008116db); }
            blk = 173; continue;
        }
        1113 => {
            c.w32(0, c.r32(0));
            let r1004 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1004);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1005 = c.r64(8) & c.r64(0);
            c.w64(8, r1005);
            c.logic(r1005, 64);
            let Some(t1006) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008116f0); };
            c.logic(t1006 & 0x2u64, 8);
            if c.cc_e() { blk = 156; continue; }
            blk = 1114; continue;
        }
        1114 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008116ff); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000811700); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700081170d, 0x7000811708);
        }
        1115 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081170d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081170f); }
            blk = 156; continue;
        }
        1116 => {
            c.w32(0, c.r32(0));
            let r1007 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1007);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1008 = c.r64(8) & c.r64(0);
            c.w64(8, r1008);
            c.logic(r1008, 64);
            let Some(t1009) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000811724); };
            c.logic(t1009 & 0x2u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 1117; continue;
        }
        1117 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000811733); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000811734); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000811741, 0x700081173c);
        }
        1118 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000811741); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000811743); }
            blk = 140; continue;
        }
        1119 => {
            c.w32(0, c.r32(0));
            let r1010 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1010);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1011 = c.r64(8) & c.r64(0);
            c.w64(8, r1011);
            c.logic(r1011, 64);
            let Some(t1012) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000811758); };
            c.logic(t1012 & 0x2u64, 8);
            if c.cc_e() { blk = 124; continue; }
            blk = 1120; continue;
        }
        1120 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000811767); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000811768); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000811775, 0x7000811770);
        }
        1121 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000811775); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000811777); }
            blk = 124; continue;
        }
        1122 => {
            c.w32(0, c.r32(0));
            let r1013 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1013);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1014 = c.r64(8) & c.r64(0);
            c.w64(8, r1014);
            c.logic(r1014, 64);
            let Some(t1015) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081178c); };
            c.logic(t1015 & 0x2u64, 8);
            if c.cc_e() { blk = 108; continue; }
            blk = 1123; continue;
        }
        1123 => {
            c.w64(11, c.r64(9).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081179b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081179c); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x70008117a9, 0x70008117a4);
        }
        1124 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008117a9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008117ab); }
            blk = 108; continue;
        }
        1125 => {
            return RegfileExit::jump(0x70008117b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008117d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008117d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008117d8); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008117de); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008117e5); };
            c.w32(9, t3);
            let r4 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r4);
            let r5 = c.r32(9) & 0x3fu64;
            c.w32(9, r5);
            c.logic(r5, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008117f2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x70008117f6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008117fa); }
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
            if c.cc_e() { blk = 349; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 350; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 351; continue;
        }
        7 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 347; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 348; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 351; continue;
        }
        10 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_l() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 345; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 346; continue; }
            blk = 13; continue;
        }
        13 => {
            blk = 351; continue;
        }
        14 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 343; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 344; continue; }
            blk = 16; continue;
        }
        16 => {
            blk = 351; continue;
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
            if c.cc_e() { blk = 341; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 342; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 351; continue;
        }
        22 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 339; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 340; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 351; continue;
        }
        25 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 29; continue; }
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 337; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 338; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 351; continue;
        }
        29 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 335; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 336; continue; }
            blk = 31; continue;
        }
        31 => {
            blk = 351; continue;
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
            if c.cc_e() { blk = 333; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 334; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 351; continue;
        }
        38 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 331; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 332; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 351; continue;
        }
        41 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_l() { blk = 45; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 329; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 330; continue; }
            blk = 44; continue;
        }
        44 => {
            blk = 351; continue;
        }
        45 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 261; continue; }
            blk = 47; continue;
        }
        47 => {
            blk = 351; continue;
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
            if c.cc_e() { blk = 145; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 52; continue;
        }
        52 => {
            blk = 351; continue;
        }
        53 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 293; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 310; continue; }
            blk = 55; continue;
        }
        55 => {
            blk = 351; continue;
        }
        56 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 117; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 59; continue;
        }
        59 => {
            blk = 351; continue;
        }
        60 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 90; continue; }
            blk = 62; continue;
        }
        62 => {
            blk = 351; continue;
        }
        63 => {
            let Some(t6) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008119f0); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t8) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008119fc); };
            c.w32(11, t8);
            let r9 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r9);
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811a03); };
            c.cmp(t10, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000811a0f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 66; continue;
        }
        66 => {
            if c.cc_ne() { blk = 325; continue; }
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
            blk = 325; continue;
        }
        69 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 70; continue;
        }
        70 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 325; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t12) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811a61); };
            c.cmp(t12, 0x834u64, 16);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t13) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811a6a); };
            c.w32(8, t13);
            blk = 74; continue;
        }
        73 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000811a70); };
            c.w32(8, t14);
            blk = 74; continue;
        }
        74 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t15) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811a86); };
            c.w32(12, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000811a8a); };
            c.cmp(t16, c.r32(12), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t17) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811a97); };
            c.w32(12, t17);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 77; continue;
        }
        77 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 355; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811abe); };
            c.w64(1, t18);
            let Some(t19) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000811ac5); };
            c.w64(7, t19);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000811ad4); };
            c.w64(0, t20);
            let Some(t21) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000811adb); };
            c.cmp(t21, c.r64(6), 64);
            if c.cc_be() { blk = 352; continue; }
            blk = 79; continue;
        }
        79 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x7000811ae4); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 80; continue;
        }
        80 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000811aef); };
            c.w64(15, t22);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000811af6); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000811afa); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t23) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000811b12); };
            c.w64(12, t23);
            blk = 82; continue;
        }
        82 => {
            let r24 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r24);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000811b1d); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 82; continue; }
            blk = 83; continue;
        }
        83 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            let r25 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r25);
            let Some(t26) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000811b30); };
            c.w32(8, t26);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x7000811b34); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 84; continue; }
            blk = 85; continue;
        }
        85 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000811b3e); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 86; continue;
        }
        86 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r27 = c.r64(7) & c.r64(2);
            c.w64(7, r27);
            c.logic(r27, 64);
            let Some(t28) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000811b50); };
            c.logic(t28 & 0x4u64, 8);
            if c.cc_ne() { blk = 435; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(9, c.r64(1));
            blk = 88; continue;
        }
        88 => {
            let Some(t29) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811b5d); };
            c.w32(1, t29);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 354; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000811b70); };
            c.w64(0, t30);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000811b74); }
            blk = 328; continue;
        }
        90 => {
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811b7e); };
            c.w32(9, t31);
            let r32 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r32);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t33) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811b8a); };
            c.w32(11, t33);
            let r34 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r34);
            let Some(t35) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811b91); };
            c.cmp(t35, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 92; continue;
        }
        92 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000811b9d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 93; continue;
        }
        93 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t36) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x7000811bc0); };
            c.w64(12, t36);
            let r37 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r37);
            let Some(t38) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x7000811bca); };
            c.w64(15, t38);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 95; continue;
        }
        95 => {
            blk = 325; continue;
        }
        96 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 97; continue;
        }
        97 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 325; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t39) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811be9); };
            c.cmp(t39, 0x834u64, 16);
            if c.cc_e() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            let Some(t40) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811bf2); };
            c.w32(8, t40);
            blk = 101; continue;
        }
        100 => {
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000811bf8); };
            c.w32(8, t41);
            blk = 101; continue;
        }
        101 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t42) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811c0e); };
            c.w32(12, t42);
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000811c12); };
            c.cmp(t43, c.r32(12), 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t44) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811c1f); };
            c.w32(12, t44);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 360; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t45) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811c46); };
            c.w64(1, t45);
            let Some(t46) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000811c4d); };
            c.w64(7, t46);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000811c5c); };
            c.w64(0, t47);
            let Some(t48) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000811c63); };
            c.cmp(t48, c.r64(6), 64);
            if c.cc_be() { blk = 357; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x7000811c6c); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 107; continue;
        }
        107 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000811c77); };
            c.w64(15, t49);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000811c7e); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000811c82); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 110; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000811c9a); };
            c.w64(12, t50);
            blk = 109; continue;
        }
        109 => {
            let r51 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r51);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000811ca5); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 109; continue; }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            let r52 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r52);
            let Some(t53) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000811cb8); };
            c.w32(8, t53);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x7000811cbc); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 111; continue; }
            blk = 112; continue;
        }
        112 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000811cc6); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r54 = c.r64(7) & c.r64(2);
            c.w64(7, r54);
            c.logic(r54, 64);
            let Some(t55) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000811cd8); };
            c.logic(t55 & 0x4u64, 8);
            if c.cc_ne() { blk = 432; continue; }
            blk = 114; continue;
        }
        114 => {
            c.w64(9, c.r64(1));
            blk = 115; continue;
        }
        115 => {
            let Some(t56) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811ce5); };
            c.w32(1, t56);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 359; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t57) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000811cf8); };
            c.w64(0, t57);
            if !c.st32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000811cfc); }
            blk = 328; continue;
        }
        117 => {
            let Some(t58) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811d06); };
            c.w32(9, t58);
            let r59 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r59);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 118; continue;
        }
        118 => {
            let Some(t60) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811d12); };
            c.w32(11, t60);
            let r61 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r61);
            let Some(t62) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811d19); };
            c.cmp(t62, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 119; continue;
        }
        119 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000811d25); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 120; continue;
        }
        120 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t63) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x7000811d48); };
            c.w64(12, t63);
            let r64 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r64);
            let Some(t65) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x7000811d52); };
            c.w64(15, t65);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 124; continue; }
            blk = 122; continue;
        }
        122 => {
            blk = 325; continue;
        }
        123 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 124; continue;
        }
        124 => {
            let Some(t66) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811d69); };
            c.cmp(t66, 0x834u64, 16);
            if c.cc_e() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t67) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811d72); };
            c.w32(8, t67);
            blk = 127; continue;
        }
        126 => {
            let Some(t68) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000811d78); };
            c.w32(8, t68);
            blk = 127; continue;
        }
        127 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t69) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811d8e); };
            c.w32(12, t69);
            let Some(t70) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000811d92); };
            c.cmp(t70, c.r32(12), 32);
            if c.cc_ne() { blk = 141; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t71) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811d9f); };
            c.w32(12, t71);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 365; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811dc6); };
            c.w64(1, t72);
            let Some(t73) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000811dcd); };
            c.w64(7, t73);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000811ddc); };
            c.w64(0, t74);
            let Some(t75) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000811de3); };
            c.cmp(t75, c.r64(6), 64);
            if c.cc_be() { blk = 362; continue; }
            blk = 132; continue;
        }
        132 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x7000811dec); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 133; continue;
        }
        133 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000811df7); };
            c.w64(15, t76);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000811dfe); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000811e02); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 136; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t77) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000811e1a); };
            c.w64(12, t77);
            blk = 135; continue;
        }
        135 => {
            let r78 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r78);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000811e25); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 135; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 138; continue; }
            blk = 137; continue;
        }
        137 => {
            let r79 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r79);
            let Some(t80) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000811e38); };
            c.w32(8, t80);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x7000811e3c); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 137; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000811e46); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r81 = c.r64(7) & c.r64(2);
            c.w64(7, r81);
            c.logic(r81, 64);
            let Some(t82) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000811e58); };
            c.logic(t82 & 0x4u64, 8);
            if c.cc_ne() { blk = 429; continue; }
            blk = 140; continue;
        }
        140 => {
            c.w64(9, c.r64(1));
            blk = 141; continue;
        }
        141 => {
            let Some(t83) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811e65); };
            c.w32(1, t83);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 364; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000811e80); };
            c.w64(0, t84);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000811e84); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r85 = c.r64(7) & c.r64(9);
            c.w64(7, r85);
            c.logic(r85, 64);
            let Some(t86) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000811e96); };
            c.logic(t86 & 0x4u64, 8);
            if c.cc_ne() { blk = 426; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 328; continue;
        }
        145 => {
            let Some(t87) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811ea5); };
            c.w32(9, t87);
            let r88 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r88);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t89) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811eb1); };
            c.w32(11, t89);
            let r90 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r90);
            let Some(t91) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811eb8); };
            c.cmp(t91, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 147; continue;
        }
        147 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000811ec4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 148; continue;
        }
        148 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t92) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x7000811ee7); };
            c.w64(12, t92);
            let r93 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r93);
            let Some(t94) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x7000811ef1); };
            c.w64(15, t94);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 152; continue; }
            blk = 150; continue;
        }
        150 => {
            blk = 325; continue;
        }
        151 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 152; continue;
        }
        152 => {
            let Some(t95) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000811f08); };
            c.cmp(t95, 0x834u64, 16);
            if c.cc_e() { blk = 154; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t96) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811f11); };
            c.w32(8, t96);
            blk = 155; continue;
        }
        154 => {
            let Some(t97) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000811f17); };
            c.w32(8, t97);
            blk = 155; continue;
        }
        155 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t98) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000811f2d); };
            c.w32(12, t98);
            let Some(t99) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000811f31); };
            c.cmp(t99, c.r32(12), 32);
            if c.cc_ne() { blk = 169; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t100) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000811f3e); };
            c.w32(12, t100);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 158; continue;
        }
        158 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 370; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000811f65); };
            c.w64(1, t101);
            let Some(t102) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000811f6c); };
            c.w64(7, t102);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000811f7b); };
            c.w64(0, t103);
            let Some(t104) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000811f82); };
            c.cmp(t104, c.r64(6), 64);
            if c.cc_be() { blk = 367; continue; }
            blk = 160; continue;
        }
        160 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x7000811f8b); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 161; continue;
        }
        161 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t105) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000811f96); };
            c.w64(15, t105);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000811f9d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000811fa1); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 164; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t106) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000811fb9); };
            c.w64(12, t106);
            blk = 163; continue;
        }
        163 => {
            let r107 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r107);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000811fc4); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 163; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 166; continue; }
            blk = 165; continue;
        }
        165 => {
            let r108 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r108);
            let Some(t109) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000811fd7); };
            c.w32(8, t109);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x7000811fdb); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 165; continue; }
            blk = 166; continue;
        }
        166 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000811fe5); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r110 = c.r64(7) & c.r64(2);
            c.w64(7, r110);
            c.logic(r110, 64);
            let Some(t111) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000811ff7); };
            c.logic(t111 & 0x4u64, 8);
            if c.cc_ne() { blk = 423; continue; }
            blk = 168; continue;
        }
        168 => {
            c.w64(9, c.r64(1));
            blk = 169; continue;
        }
        169 => {
            let Some(t112) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000812004); };
            c.w32(1, t112);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 369; continue; }
            blk = 170; continue;
        }
        170 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081201f); };
            c.w64(0, t113);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000812023); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 172; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r114 = c.r64(7) & c.r64(9);
            c.w64(7, r114);
            c.logic(r114, 64);
            let Some(t115) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812035); };
            c.logic(t115 & 0x4u64, 8);
            if c.cc_ne() { blk = 420; continue; }
            blk = 172; continue;
        }
        172 => {
            blk = 328; continue;
        }
        173 => {
            let Some(t116) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812044); };
            c.w32(9, t116);
            let r117 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r117);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t118) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000812050); };
            c.w32(11, t118);
            let r119 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r119);
            let Some(t120) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812057); };
            c.cmp(t120, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 175; continue;
        }
        175 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000812063); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 176; continue;
        }
        176 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 177; continue;
        }
        177 => {
            let Some(t121) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x7000812086); };
            c.w64(12, t121);
            let r122 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r122);
            let Some(t123) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x7000812090); };
            c.w64(15, t123);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 180; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 325; continue;
        }
        179 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 180; continue;
        }
        180 => {
            let Some(t124) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008120a7); };
            c.cmp(t124, 0x834u64, 16);
            if c.cc_e() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t125) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008120b0); };
            c.w32(8, t125);
            blk = 183; continue;
        }
        182 => {
            let Some(t126) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008120b6); };
            c.w32(8, t126);
            blk = 183; continue;
        }
        183 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t127) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008120cc); };
            c.w32(12, t127);
            let Some(t128) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008120d0); };
            c.cmp(t128, c.r32(12), 32);
            if c.cc_ne() { blk = 197; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t129) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008120dd); };
            c.w32(12, t129);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 186; continue;
        }
        186 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 375; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t130) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000812104); };
            c.w64(1, t130);
            let Some(t131) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081210b); };
            c.w64(7, t131);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t132) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081211a); };
            c.w64(0, t132);
            let Some(t133) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000812121); };
            c.cmp(t133, c.r64(6), 64);
            if c.cc_be() { blk = 372; continue; }
            blk = 188; continue;
        }
        188 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081212a); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 189; continue;
        }
        189 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t134) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000812135); };
            c.w64(15, t134);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081213c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000812140); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 192; continue; }
            blk = 190; continue;
        }
        190 => {
            let Some(t135) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000812158); };
            c.w64(12, t135);
            blk = 191; continue;
        }
        191 => {
            let r136 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r136);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000812163); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 191; continue; }
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 194; continue; }
            blk = 193; continue;
        }
        193 => {
            let r137 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r137);
            let Some(t138) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000812176); };
            c.w32(8, t138);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x700081217a); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 193; continue; }
            blk = 194; continue;
        }
        194 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000812184); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 196; continue; }
            blk = 195; continue;
        }
        195 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r139 = c.r64(7) & c.r64(2);
            c.w64(7, r139);
            c.logic(r139, 64);
            let Some(t140) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812196); };
            c.logic(t140 & 0x4u64, 8);
            if c.cc_ne() { blk = 417; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(9, c.r64(1));
            blk = 197; continue;
        }
        197 => {
            let Some(t141) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008121a3); };
            c.w32(1, t141);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 374; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008121be); };
            c.w64(0, t142);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008121c2); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r143 = c.r64(7) & c.r64(9);
            c.w64(7, r143);
            c.logic(r143, 64);
            let Some(t144) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008121d4); };
            c.logic(t144 & 0x4u64, 8);
            if c.cc_ne() { blk = 414; continue; }
            blk = 200; continue;
        }
        200 => {
            blk = 328; continue;
        }
        201 => {
            let Some(t145) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008121e3); };
            c.w32(9, t145);
            let r146 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r146);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 207; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t147) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008121ef); };
            c.w32(11, t147);
            let r148 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r148);
            let Some(t149) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008121f6); };
            c.cmp(t149, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 203; continue;
        }
        203 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000812202); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 204; continue;
        }
        204 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t150) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x7000812225); };
            c.w64(12, t150);
            let r151 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r151);
            let Some(t152) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x700081222f); };
            c.w64(15, t152);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 208; continue; }
            blk = 206; continue;
        }
        206 => {
            blk = 325; continue;
        }
        207 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 208; continue;
        }
        208 => {
            let Some(t153) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812246); };
            c.cmp(t153, 0x834u64, 16);
            if c.cc_e() { blk = 210; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t154) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081224f); };
            c.w32(8, t154);
            blk = 211; continue;
        }
        210 => {
            let Some(t155) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000812255); };
            c.w32(8, t155);
            blk = 211; continue;
        }
        211 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 212; continue;
        }
        212 => {
            let Some(t156) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081226b); };
            c.w32(12, t156);
            let Some(t157) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081226f); };
            c.cmp(t157, c.r32(12), 32);
            if c.cc_ne() { blk = 225; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t158) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081227c); };
            c.w32(12, t158);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 380; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t159) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008122a3); };
            c.w64(1, t159);
            let Some(t160) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008122aa); };
            c.w64(7, t160);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(7)));
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008122b9); };
            c.w64(0, t161);
            let Some(t162) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008122c0); };
            c.cmp(t162, c.r64(6), 64);
            if c.cc_be() { blk = 377; continue; }
            blk = 216; continue;
        }
        216 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x70008122c9); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 217; continue;
        }
        217 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t163) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008122d4); };
            c.w64(15, t163);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008122db); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008122df); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 220; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008122f7); };
            c.w64(12, t164);
            blk = 219; continue;
        }
        219 => {
            let r165 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r165);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000812302); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 219; continue; }
            blk = 220; continue;
        }
        220 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 222; continue; }
            blk = 221; continue;
        }
        221 => {
            let r166 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r166);
            let Some(t167) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000812315); };
            c.w32(8, t167);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x7000812319); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 221; continue; }
            blk = 222; continue;
        }
        222 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000812323); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 224; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r168 = c.r64(7) & c.r64(2);
            c.w64(7, r168);
            c.logic(r168, 64);
            let Some(t169) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812335); };
            c.logic(t169 & 0x4u64, 8);
            if c.cc_ne() { blk = 411; continue; }
            blk = 224; continue;
        }
        224 => {
            c.w64(9, c.r64(1));
            blk = 225; continue;
        }
        225 => {
            let Some(t170) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000812342); };
            c.w32(1, t170);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 379; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t171) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081235d); };
            c.w64(0, t171);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000812361); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r172 = c.r64(7) & c.r64(9);
            c.w64(7, r172);
            c.logic(r172, 64);
            let Some(t173) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812373); };
            c.logic(t173 & 0x4u64, 8);
            if c.cc_ne() { blk = 408; continue; }
            blk = 228; continue;
        }
        228 => {
            blk = 328; continue;
        }
        229 => {
            let Some(t174) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812382); };
            c.w32(9, t174);
            let r175 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r175);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 235; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t176) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081238e); };
            c.w32(11, t176);
            let r177 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r177);
            let Some(t178) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812395); };
            c.cmp(t178, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 231; continue;
        }
        231 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008123a1); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 232; continue;
        }
        232 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 233; continue;
        }
        233 => {
            let Some(t179) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x70008123c4); };
            c.w64(12, t179);
            let r180 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r180);
            let Some(t181) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x70008123ce); };
            c.w64(15, t181);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 236; continue; }
            blk = 234; continue;
        }
        234 => {
            blk = 325; continue;
        }
        235 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 236; continue;
        }
        236 => {
            let Some(t182) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008123e5); };
            c.cmp(t182, 0x834u64, 16);
            if c.cc_e() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t183) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008123ee); };
            c.w32(8, t183);
            blk = 239; continue;
        }
        238 => {
            let Some(t184) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008123f4); };
            c.w32(8, t184);
            blk = 239; continue;
        }
        239 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 240; continue;
        }
        240 => {
            let Some(t185) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081240a); };
            c.w32(12, t185);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ae() { blk = 382; continue; }
            blk = 241; continue;
        }
        241 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t186) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812428); };
            c.w32(3, t186);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700081242d); }
            let Some(t187) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000812431); };
            c.cmp(t187, c.r32(3), 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 242; continue;
        }
        242 => {
            let Some(t188) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081243e); };
            c.w32(3, t188);
            let Some(t189) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000812442); };
            c.cmp(t189, c.r32(3), 32);
            if c.cc_ne() { blk = 255; continue; }
            blk = 243; continue;
        }
        243 => {
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 259; continue; }
            blk = 244; continue;
        }
        244 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 386; continue; }
            blk = 245; continue;
        }
        245 => {
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000812469); };
            c.w64(1, t190);
            let Some(t191) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000812470); };
            c.w64(3, t191);
            c.w64(6, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081247f); };
            c.w64(0, t192);
            let Some(t193) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000812486); };
            c.cmp(t193, c.r64(7), 64);
            if c.cc_be() { blk = 383; continue; }
            blk = 246; continue;
        }
        246 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700081248f); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 247; continue;
        }
        247 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081249a); };
            c.w64(15, t194);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008124a1); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008124a5); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 250; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008124bd); };
            c.w64(12, t195);
            blk = 249; continue;
        }
        249 => {
            let r196 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r196);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70008124c8); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 249; continue; }
            blk = 250; continue;
        }
        250 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            let r197 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r197);
            let Some(t198) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70008124db); };
            c.w32(8, t198);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x70008124df); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 251; continue; }
            blk = 252; continue;
        }
        252 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008124e9); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 254; continue; }
            blk = 253; continue;
        }
        253 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r199 = c.r64(7) & c.r64(2);
            c.w64(7, r199);
            c.logic(r199, 64);
            let Some(t200) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008124fb); };
            c.logic(t200 & 0x4u64, 8);
            if c.cc_ne() { blk = 405; continue; }
            blk = 254; continue;
        }
        254 => {
            c.w64(9, c.r64(1));
            blk = 255; continue;
        }
        255 => {
            let Some(t201) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000812508); };
            c.w32(1, t201);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 385; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081251b); };
            c.w64(0, t202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081251f); };
            c.w64(1, t203);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000812523); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 258; continue; }
            blk = 257; continue;
        }
        257 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r204 = c.r64(7) & c.r64(9);
            c.w64(7, r204);
            c.logic(r204, 64);
            let Some(t205) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812535); };
            c.logic(t205 & 0x4u64, 8);
            if c.cc_ne() { blk = 402; continue; }
            blk = 258; continue;
        }
        258 => {
            blk = 328; continue;
        }
        259 => {
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000812544); };
            c.w64(7, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000812548); };
            c.w64(3, t207);
            blk = 325; continue;
        }
        260 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000812551); };
            c.w64(7, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000812555); };
            c.w64(3, t209);
            blk = 325; continue;
        }
        261 => {
            let Some(t210) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081255e); };
            c.w32(9, t210);
            let r211 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r211);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 267; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t212) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081256a); };
            c.w32(11, t212);
            let r213 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r213);
            let Some(t214) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812571); };
            c.cmp(t214, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 263; continue;
        }
        263 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081257d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 264; continue;
        }
        264 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t215) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x70008125a0); };
            c.w64(12, t215);
            let r216 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r216);
            let Some(t217) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x70008125aa); };
            c.w64(15, t217);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 268; continue; }
            blk = 266; continue;
        }
        266 => {
            blk = 325; continue;
        }
        267 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 268; continue;
        }
        268 => {
            let Some(t218) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008125c1); };
            c.cmp(t218, 0x834u64, 16);
            if c.cc_e() { blk = 270; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t219) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008125ca); };
            c.w32(8, t219);
            blk = 271; continue;
        }
        270 => {
            let Some(t220) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008125d0); };
            c.w32(8, t220);
            blk = 271; continue;
        }
        271 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(8));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t221) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008125e6); };
            c.w32(12, t221);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_ae() { blk = 388; continue; }
            blk = 273; continue;
        }
        273 => {
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t222) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812604); };
            c.w32(3, t222);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000812609); }
            let Some(t223) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081260d); };
            c.cmp(t223, c.r32(3), 32);
            if c.cc_e() { blk = 292; continue; }
            blk = 274; continue;
        }
        274 => {
            let Some(t224) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081261a); };
            c.w32(3, t224);
            let Some(t225) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081261e); };
            c.cmp(t225, c.r32(3), 32);
            if c.cc_ne() { blk = 287; continue; }
            blk = 275; continue;
        }
        275 => {
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 291; continue; }
            blk = 276; continue;
        }
        276 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 392; continue; }
            blk = 277; continue;
        }
        277 => {
            let Some(t226) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000812645); };
            c.w64(1, t226);
            let Some(t227) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081264c); };
            c.w64(3, t227);
            c.w64(6, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(7, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081265b); };
            c.w64(0, t228);
            let Some(t229) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000812662); };
            c.cmp(t229, c.r64(7), 64);
            if c.cc_be() { blk = 389; continue; }
            blk = 278; continue;
        }
        278 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700081266b); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 279; continue;
        }
        279 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000812676); };
            c.w64(15, t230);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081267d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000812681); }
            c.w64(7, c.r64(8).wrapping_mul(2).wrapping_add(0x7));
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_e() { blk = 282; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000812699); };
            c.w64(12, t231);
            blk = 281; continue;
        }
        281 => {
            let r232 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r232);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70008126a4); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ne() { blk = 281; continue; }
            blk = 282; continue;
        }
        282 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 284; continue; }
            blk = 283; continue;
        }
        283 => {
            let r233 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r233);
            let Some(t234) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x70008126b7); };
            c.w32(8, t234);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(8)) { return RegfileExit::fault(0x70008126bb); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 283; continue; }
            blk = 284; continue;
        }
        284 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008126c5); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r235 = c.r64(7) & c.r64(2);
            c.w64(7, r235);
            c.logic(r235, 64);
            let Some(t236) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008126d7); };
            c.logic(t236 & 0x4u64, 8);
            if c.cc_ne() { blk = 399; continue; }
            blk = 286; continue;
        }
        286 => {
            c.w64(9, c.r64(1));
            blk = 287; continue;
        }
        287 => {
            let Some(t237) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008126e4); };
            c.w32(1, t237);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 391; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008126f7); };
            c.w64(0, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008126fb); };
            c.w64(1, t239);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008126ff); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 290; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r240 = c.r64(7) & c.r64(9);
            c.w64(7, r240);
            c.logic(r240, 64);
            let Some(t241) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812711); };
            c.logic(t241 & 0x4u64, 8);
            if c.cc_ne() { blk = 396; continue; }
            blk = 290; continue;
        }
        290 => {
            blk = 328; continue;
        }
        291 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000812720); };
            c.w64(7, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000812724); };
            c.w64(3, t243);
            blk = 325; continue;
        }
        292 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081272d); };
            c.w64(7, t244);
            let Some(t245) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000812731); };
            c.w64(3, t245);
            blk = 325; continue;
        }
        293 => {
            let Some(t246) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081273a); };
            c.w32(9, t246);
            let r247 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r247);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 299; continue; }
            blk = 294; continue;
        }
        294 => {
            let Some(t248) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000812746); };
            c.w32(11, t248);
            let r249 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r249);
            let Some(t250) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081274d); };
            c.cmp(t250, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 295; continue;
        }
        295 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000812759); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 296; continue;
        }
        296 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 297; continue;
        }
        297 => {
            let Some(t251) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x700081277c); };
            c.w64(12, t251);
            let r252 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r252);
            let Some(t253) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x7000812786); };
            c.w64(15, t253);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 300; continue; }
            blk = 298; continue;
        }
        298 => {
            blk = 325; continue;
        }
        299 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 300; continue;
        }
        300 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 301; continue;
        }
        301 => {
            let Some(t254) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008127a1); };
            c.w32(12, t254);
            let Some(t255) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008127a5); };
            c.cmp(t255, c.r32(12), 32);
            if c.cc_ne() { blk = 325; continue; }
            blk = 302; continue;
        }
        302 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008127b2); }
            blk = 304; continue;
        }
        303 => {
            c.w64(12, c.r64(0));
            let r256 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r256);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 304; continue;
        }
        304 => {
            let Some(t257) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008127c8); };
            c.cmp(t257, 0x834u64, 16);
            if c.cc_e() { blk = 306; continue; }
            blk = 305; continue;
        }
        305 => {
            let Some(t258) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008127d1); };
            c.w32(8, t258);
            blk = 307; continue;
        }
        306 => {
            let Some(t259) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008127d7); };
            c.w32(8, t259);
            blk = 307; continue;
        }
        307 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 325; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t260) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008127ea); };
            c.w32(1, t260);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 394; continue; }
            blk = 309; continue;
        }
        309 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000812807); }
            blk = 328; continue;
        }
        310 => {
            let Some(t261) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812813); };
            c.w32(9, t261);
            let r262 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r262);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 316; continue; }
            blk = 311; continue;
        }
        311 => {
            let Some(t263) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081281f); };
            c.w32(11, t263);
            let r264 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r264);
            let Some(t265) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812826); };
            c.cmp(t265, 0x42u64, 16);
            if c.cc_ne() { blk = 325; continue; }
            blk = 312; continue;
        }
        312 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000812832); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 325; continue; }
            blk = 313; continue;
        }
        313 => {
            if c.cc_ne() { blk = 325; continue; }
            blk = 314; continue;
        }
        314 => {
            let Some(t266) = c.ld64(0x7000811a34u64) else { return RegfileExit::fault(0x7000812855); };
            c.w64(12, t266);
            let r267 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r267);
            let Some(t268) = c.ld64(0x7000811a41u64) else { return RegfileExit::fault(0x700081285f); };
            c.w64(15, t268);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 317; continue; }
            blk = 315; continue;
        }
        315 => {
            blk = 325; continue;
        }
        316 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 317; continue;
        }
        317 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 320; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t269) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000812877); };
            c.w32(12, t269);
            let Some(t270) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081287b); };
            c.cmp(t270, c.r32(12), 32);
            if c.cc_ne() { blk = 325; continue; }
            blk = 319; continue;
        }
        319 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000812884); }
            blk = 321; continue;
        }
        320 => {
            c.w64(12, c.r64(0));
            let r271 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r271);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 321; continue;
        }
        321 => {
            let Some(t272) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081289a); };
            c.cmp(t272, 0x834u64, 16);
            if c.cc_e() { blk = 323; continue; }
            blk = 322; continue;
        }
        322 => {
            let Some(t273) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008128a3); };
            c.w32(8, t273);
            blk = 324; continue;
        }
        323 => {
            let Some(t274) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008128a9); };
            c.w32(8, t274);
            blk = 324; continue;
        }
        324 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_b() { blk = 326; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t275) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008128b8); };
            c.w64(5, t275);
            let r276 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r276);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008128c0); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008128c1); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008128c2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008128c3); }
            let Some(t277) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008128c4); };
            if !c.push64(t277) { return RegfileExit::fault(0x70008128c4); }
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(0)) { return RegfileExit::fault(0x70008128c8); }
            c.w32(0, 0x5u64);
            let Some(t278) = c.ld64(c.r64(13).wrapping_add(0x2020)) else { return RegfileExit::fault(0x70008128d2); };
            c.w64(3, t278);
            return RegfileExit::jump(0x700086ed74);
        }
        326 => {
            let Some(t279) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008128de); };
            c.w32(1, t279);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 395; continue; }
            blk = 327; continue;
        }
        327 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x70008128fb); }
            blk = 328; continue;
        }
        328 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000812905); }
            return c.ret(0x0, 0x7000812906);
        }
        329 => {
            return RegfileExit::fault(0x7000812907);
        }
        330 => {
            return RegfileExit::fault(0x7000812908);
        }
        331 => {
            return RegfileExit::fault(0x7000812909);
        }
        332 => {
            return RegfileExit::fault(0x700081290a);
        }
        333 => {
            return RegfileExit::fault(0x700081290b);
        }
        334 => {
            return RegfileExit::fault(0x700081290c);
        }
        335 => {
            return RegfileExit::fault(0x700081290d);
        }
        336 => {
            return RegfileExit::fault(0x700081290e);
        }
        337 => {
            return RegfileExit::fault(0x700081290f);
        }
        338 => {
            return RegfileExit::fault(0x7000812910);
        }
        339 => {
            return RegfileExit::fault(0x7000812911);
        }
        340 => {
            return RegfileExit::fault(0x7000812912);
        }
        341 => {
            return RegfileExit::fault(0x7000812913);
        }
        342 => {
            return RegfileExit::fault(0x7000812914);
        }
        343 => {
            return RegfileExit::fault(0x7000812915);
        }
        344 => {
            return RegfileExit::fault(0x7000812916);
        }
        345 => {
            return RegfileExit::fault(0x7000812917);
        }
        346 => {
            return RegfileExit::fault(0x7000812918);
        }
        347 => {
            return RegfileExit::fault(0x7000812919);
        }
        348 => {
            return RegfileExit::fault(0x700081291a);
        }
        349 => {
            return RegfileExit::fault(0x700081291b);
        }
        350 => {
            return RegfileExit::fault(0x700081291c);
        }
        351 => {
            return RegfileExit::fault(0x700081291d);
        }
        352 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700081291e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000812922); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000812926); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700081292a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081292e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000812932); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700081293e, 0x7000812939);
        }
        353 => {
            c.w64(1, c.r64(0));
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000812941); };
            c.w64(2, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000812945); };
            c.w64(11, t281);
            let Some(t282) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000812949); };
            c.w64(9, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081294d); };
            c.w64(12, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000812951); };
            c.w64(8, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000812955); };
            c.w64(15, t285);
            blk = 80; continue;
        }
        354 => {
            return RegfileExit::fault(0x700081295e);
        }
        355 => {
            let Some(t286) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081295f); };
            c.w64(3, t286);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081296f, 0x700081296a);
        }
        356 => {
            return RegfileExit::fault(0x700081296f);
        }
        357 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000812970); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000812974); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000812978); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x700081297c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000812980); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000812984); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x7000812990, 0x700081298b);
        }
        358 => {
            c.w64(1, c.r64(0));
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000812993); };
            c.w64(2, t287);
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000812997); };
            c.w64(11, t288);
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081299b); };
            c.w64(9, t289);
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081299f); };
            c.w64(12, t290);
            let Some(t291) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008129a3); };
            c.w64(8, t291);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008129a7); };
            c.w64(15, t292);
            blk = 107; continue;
        }
        359 => {
            return RegfileExit::fault(0x70008129b0);
        }
        360 => {
            let Some(t293) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008129b1); };
            c.w64(3, t293);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008129c1, 0x70008129bc);
        }
        361 => {
            return RegfileExit::fault(0x70008129c1);
        }
        362 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008129c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008129c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008129ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x70008129ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008129d2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x70008129d6); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x70008129e2, 0x70008129dd);
        }
        363 => {
            c.w64(1, c.r64(0));
            let Some(t294) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008129e5); };
            c.w64(2, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008129e9); };
            c.w64(11, t295);
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008129ed); };
            c.w64(9, t296);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008129f1); };
            c.w64(12, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008129f5); };
            c.w64(8, t298);
            let Some(t299) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008129f9); };
            c.w64(15, t299);
            blk = 133; continue;
        }
        364 => {
            return RegfileExit::fault(0x7000812a02);
        }
        365 => {
            let Some(t300) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000812a03); };
            c.w64(3, t300);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000812a13, 0x7000812a0e);
        }
        366 => {
            return RegfileExit::fault(0x7000812a13);
        }
        367 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000812a14); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000812a18); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000812a1c); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000812a20); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000812a24); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000812a28); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x7000812a34, 0x7000812a2f);
        }
        368 => {
            c.w64(1, c.r64(0));
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000812a37); };
            c.w64(2, t301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000812a3b); };
            c.w64(11, t302);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000812a3f); };
            c.w64(9, t303);
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000812a43); };
            c.w64(12, t304);
            let Some(t305) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000812a47); };
            c.w64(8, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000812a4b); };
            c.w64(15, t306);
            blk = 161; continue;
        }
        369 => {
            return RegfileExit::fault(0x7000812a54);
        }
        370 => {
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000812a55); };
            c.w64(3, t307);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000812a65, 0x7000812a60);
        }
        371 => {
            return RegfileExit::fault(0x7000812a65);
        }
        372 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000812a66); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000812a6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000812a6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000812a72); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000812a76); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000812a7a); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x7000812a86, 0x7000812a81);
        }
        373 => {
            c.w64(1, c.r64(0));
            let Some(t308) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000812a89); };
            c.w64(2, t308);
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000812a8d); };
            c.w64(11, t309);
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000812a91); };
            c.w64(9, t310);
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000812a95); };
            c.w64(12, t311);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000812a99); };
            c.w64(8, t312);
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000812a9d); };
            c.w64(15, t313);
            blk = 189; continue;
        }
        374 => {
            return RegfileExit::fault(0x7000812aa6);
        }
        375 => {
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000812aa7); };
            c.w64(3, t314);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000812ab7, 0x7000812ab2);
        }
        376 => {
            return RegfileExit::fault(0x7000812ab7);
        }
        377 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000812ab8); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x7000812abc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x7000812ac0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(12)) { return RegfileExit::fault(0x7000812ac4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000812ac8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000812acc); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x7000812ad8, 0x7000812ad3);
        }
        378 => {
            c.w64(1, c.r64(0));
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000812adb); };
            c.w64(2, t315);
            let Some(t316) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000812adf); };
            c.w64(11, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000812ae3); };
            c.w64(9, t317);
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000812ae7); };
            c.w64(12, t318);
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000812aeb); };
            c.w64(8, t319);
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000812aef); };
            c.w64(15, t320);
            blk = 217; continue;
        }
        379 => {
            return RegfileExit::fault(0x7000812af8);
        }
        380 => {
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000812af9); };
            c.w64(3, t321);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000812b09, 0x7000812b04);
        }
        381 => {
            return RegfileExit::fault(0x7000812b09);
        }
        382 => {
            return RegfileExit::fault(0x7000812b0a);
        }
        383 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000812b0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000812b0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000812b13); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000812b17); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000812b1b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000812b1f); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000812b2b, 0x7000812b26);
        }
        384 => {
            c.w64(1, c.r64(0));
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000812b2e); };
            c.w64(2, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000812b32); };
            c.w64(11, t323);
            let Some(t324) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000812b36); };
            c.w64(9, t324);
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000812b3a); };
            c.w64(12, t325);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000812b3e); };
            c.w64(8, t326);
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000812b42); };
            c.w64(15, t327);
            blk = 247; continue;
        }
        385 => {
            return RegfileExit::fault(0x7000812b4b);
        }
        386 => {
            let Some(t328) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000812b4c); };
            c.w64(3, t328);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000812b5c, 0x7000812b57);
        }
        387 => {
            return RegfileExit::fault(0x7000812b5c);
        }
        388 => {
            return RegfileExit::fault(0x7000812b5d);
        }
        389 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x7000812b5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000812b62); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000812b66); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(12)) { return RegfileExit::fault(0x7000812b6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000812b6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000812b72); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000812b7e, 0x7000812b79);
        }
        390 => {
            c.w64(1, c.r64(0));
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000812b81); };
            c.w64(2, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000812b85); };
            c.w64(11, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000812b89); };
            c.w64(9, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000812b8d); };
            c.w64(12, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000812b91); };
            c.w64(8, t333);
            let Some(t334) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000812b95); };
            c.w64(15, t334);
            blk = 279; continue;
        }
        391 => {
            return RegfileExit::fault(0x7000812b9e);
        }
        392 => {
            let Some(t335) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000812b9f); };
            c.w64(3, t335);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000812baf, 0x7000812baa);
        }
        393 => {
            return RegfileExit::fault(0x7000812baf);
        }
        394 => {
            return RegfileExit::fault(0x7000812bb0);
        }
        395 => {
            return RegfileExit::fault(0x7000812bb1);
        }
        396 => {
            c.w32(0, c.r32(0));
            let r336 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r336);
            c.w64(7, 0xfffffffffffc0000u64);
            let r337 = c.r64(7) & c.r64(0);
            c.w64(7, r337);
            c.logic(r337, 64);
            let Some(t338) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812bc1); };
            c.logic(t338 & 0x2u64, 8);
            if c.cc_e() { blk = 290; continue; }
            blk = 397; continue;
        }
        397 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812bcf); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000812bd8, 0x7000812bd3);
        }
        398 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812bd8); }
            blk = 290; continue;
        }
        399 => {
            c.w32(1, c.r32(1));
            let r339 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r339);
            c.w64(7, 0xfffffffffffc0000u64);
            let r340 = c.r64(7) & c.r64(1);
            c.w64(7, r340);
            c.logic(r340, 64);
            let Some(t341) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812bed); };
            c.logic(t341 & 0x2u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 400; continue;
        }
        400 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812bfb); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812c04, 0x7000812bff);
        }
        401 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812c04); }
            blk = 286; continue;
        }
        402 => {
            c.w32(0, c.r32(0));
            let r342 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r342);
            c.w64(7, 0xfffffffffffc0000u64);
            let r343 = c.r64(7) & c.r64(0);
            c.w64(7, r343);
            c.logic(r343, 64);
            let Some(t344) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812c19); };
            c.logic(t344 & 0x2u64, 8);
            if c.cc_e() { blk = 258; continue; }
            blk = 403; continue;
        }
        403 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812c27); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000812c30, 0x7000812c2b);
        }
        404 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812c30); }
            blk = 258; continue;
        }
        405 => {
            c.w32(1, c.r32(1));
            let r345 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r345);
            c.w64(7, 0xfffffffffffc0000u64);
            let r346 = c.r64(7) & c.r64(1);
            c.w64(7, r346);
            c.logic(r346, 64);
            let Some(t347) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812c45); };
            c.logic(t347 & 0x2u64, 8);
            if c.cc_e() { blk = 254; continue; }
            blk = 406; continue;
        }
        406 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812c53); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812c5c, 0x7000812c57);
        }
        407 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812c5c); }
            blk = 254; continue;
        }
        408 => {
            c.w32(0, c.r32(0));
            let r348 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r348);
            c.w64(7, 0xfffffffffffc0000u64);
            let r349 = c.r64(7) & c.r64(0);
            c.w64(7, r349);
            c.logic(r349, 64);
            let Some(t350) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812c71); };
            c.logic(t350 & 0x2u64, 8);
            if c.cc_e() { blk = 228; continue; }
            blk = 409; continue;
        }
        409 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812c7f); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000812c88, 0x7000812c83);
        }
        410 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812c88); }
            blk = 228; continue;
        }
        411 => {
            c.w32(1, c.r32(1));
            let r351 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r351);
            c.w64(7, 0xfffffffffffc0000u64);
            let r352 = c.r64(7) & c.r64(1);
            c.w64(7, r352);
            c.logic(r352, 64);
            let Some(t353) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812c9d); };
            c.logic(t353 & 0x2u64, 8);
            if c.cc_e() { blk = 224; continue; }
            blk = 412; continue;
        }
        412 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812cab); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812cb4, 0x7000812caf);
        }
        413 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812cb4); }
            blk = 224; continue;
        }
        414 => {
            c.w32(0, c.r32(0));
            let r354 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r354);
            c.w64(7, 0xfffffffffffc0000u64);
            let r355 = c.r64(7) & c.r64(0);
            c.w64(7, r355);
            c.logic(r355, 64);
            let Some(t356) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812cc9); };
            c.logic(t356 & 0x2u64, 8);
            if c.cc_e() { blk = 200; continue; }
            blk = 415; continue;
        }
        415 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812cd7); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000812ce0, 0x7000812cdb);
        }
        416 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812ce0); }
            blk = 200; continue;
        }
        417 => {
            c.w32(1, c.r32(1));
            let r357 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r357);
            c.w64(7, 0xfffffffffffc0000u64);
            let r358 = c.r64(7) & c.r64(1);
            c.w64(7, r358);
            c.logic(r358, 64);
            let Some(t359) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812cf5); };
            c.logic(t359 & 0x2u64, 8);
            if c.cc_e() { blk = 196; continue; }
            blk = 418; continue;
        }
        418 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812d03); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812d0c, 0x7000812d07);
        }
        419 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812d0c); }
            blk = 196; continue;
        }
        420 => {
            c.w32(0, c.r32(0));
            let r360 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r360);
            c.w64(7, 0xfffffffffffc0000u64);
            let r361 = c.r64(7) & c.r64(0);
            c.w64(7, r361);
            c.logic(r361, 64);
            let Some(t362) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812d21); };
            c.logic(t362 & 0x2u64, 8);
            if c.cc_e() { blk = 172; continue; }
            blk = 421; continue;
        }
        421 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812d2f); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000812d38, 0x7000812d33);
        }
        422 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812d38); }
            blk = 172; continue;
        }
        423 => {
            c.w32(1, c.r32(1));
            let r363 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r363);
            c.w64(7, 0xfffffffffffc0000u64);
            let r364 = c.r64(7) & c.r64(1);
            c.w64(7, r364);
            c.logic(r364, 64);
            let Some(t365) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812d4d); };
            c.logic(t365 & 0x2u64, 8);
            if c.cc_e() { blk = 168; continue; }
            blk = 424; continue;
        }
        424 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812d5b); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812d64, 0x7000812d5f);
        }
        425 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812d64); }
            blk = 168; continue;
        }
        426 => {
            c.w32(0, c.r32(0));
            let r366 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r366);
            c.w64(7, 0xfffffffffffc0000u64);
            let r367 = c.r64(7) & c.r64(0);
            c.w64(7, r367);
            c.logic(r367, 64);
            let Some(t368) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812d79); };
            c.logic(t368 & 0x2u64, 8);
            if c.cc_e() { blk = 144; continue; }
            blk = 427; continue;
        }
        427 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812d87); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000812d90, 0x7000812d8b);
        }
        428 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812d90); }
            blk = 144; continue;
        }
        429 => {
            c.w32(1, c.r32(1));
            let r369 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r369);
            c.w64(7, 0xfffffffffffc0000u64);
            let r370 = c.r64(7) & c.r64(1);
            c.w64(7, r370);
            c.logic(r370, 64);
            let Some(t371) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812da5); };
            c.logic(t371 & 0x2u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 430; continue;
        }
        430 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812db3); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812dbc, 0x7000812db7);
        }
        431 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812dbc); }
            blk = 140; continue;
        }
        432 => {
            c.w32(1, c.r32(1));
            let r372 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r372);
            c.w64(7, 0xfffffffffffc0000u64);
            let r373 = c.r64(7) & c.r64(1);
            c.w64(7, r373);
            c.logic(r373, 64);
            let Some(t374) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812dd1); };
            c.logic(t374 & 0x2u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 433; continue;
        }
        433 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812ddf); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812de8, 0x7000812de3);
        }
        434 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812de8); }
            blk = 114; continue;
        }
        435 => {
            c.w32(1, c.r32(1));
            let r375 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r375);
            c.w64(7, 0xfffffffffffc0000u64);
            let r376 = c.r64(7) & c.r64(1);
            c.w64(7, r376);
            c.logic(r376, 64);
            let Some(t377) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812dfd); };
            c.logic(t377 & 0x2u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 436; continue;
        }
        436 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000812e0b); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000812e14, 0x7000812e0f);
        }
        437 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000812e14); }
            blk = 87; continue;
        }
        438 => {
            return RegfileExit::jump(0x7000812e1c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000812e34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000812e34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000812e38); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000812e3e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000812e45); };
            c.w32(9, t3);
            let Some(t4) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000812e4a); };
            c.w32(12, t4);
            let r5 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r5);
            let r6 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r6);
            let r7 = c.r32(9) & 0x3fu64;
            c.w32(9, r7);
            c.logic(r7, 32);
            let r8 = c.r32(12) & 0x3fu64;
            c.w32(12, r8);
            c.logic(r8, 32);
            let r9 = c.shl(c.r32(9), 0x8u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r9);
            let r10 = c.r32(12) | c.r32(9);
            c.w32(12, r10);
            c.logic(r10, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000812e66); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000812e6a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000812e6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000812e72); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000812e76); }
            c.cmp(c.r32(12), 0x105u64, 32);
            if c.cc_l() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(12), 0x403u64, 32);
            if c.cc_l() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(12), 0x403u64, 32);
            if c.cc_e() { blk = 320; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(12), 0x405u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(12), 0x503u64, 32);
            if c.cc_e() { blk = 367; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 439; continue;
        }
        6 => {
            c.cmp(c.r32(12), 0x105u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x203u64, 32);
            if c.cc_e() { blk = 414; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(12), 0x402u64, 32);
            if c.cc_e() { blk = 273; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 439; continue;
        }
        10 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_l() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x103u64, 32);
            if c.cc_e() { blk = 224; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 439; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 156; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 439; continue;
        }
        19 => {
            c.w64(8, c.r64(2));
            let r11 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r11);
            c.logic(r11, 64);
            let Some(t12) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812f39); };
            c.w64(9, t12);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000812f4d); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000812f54); };
            c.w64(12, t14);
            c.w64(15, c.r64(9));
            let r15 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r15);
            c.logic(r15, 64);
            c.w64(3, c.r64(12));
            let r16 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r16);
            c.logic(r16, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 25; continue; }
            blk = 23; continue;
        }
        23 => {
            blk = 26; continue;
        }
        24 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000812f7d); };
            c.w32(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000812f81); };
            c.cmp(t18, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000812f8e); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r19 = c.r64(8) & c.r64(2);
            c.w64(8, r19);
            c.logic(r19, 64);
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000812f9c); };
            c.logic(t20 & 0x4u64, 8);
            if c.cc_ne() { blk = 569; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812fa7); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000812fae); };
            c.w32(9, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000812fb2); };
            c.cmp(t24, c.r32(9), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 28; continue;
        }
        28 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t25) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000812fc4); };
            c.w32(9, t25);
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            let Some(t27) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000812fcb); };
            c.cmp(t27, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000812fd7); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 31; continue;
        }
        31 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r28 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r28);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 35; continue; }
            blk = 33; continue;
        }
        33 => {
            blk = 438; continue;
        }
        34 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 35; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 438; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t29) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813029); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813030); };
            c.cmp(t31, 0x834u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081303a); };
            c.w32(12, t32);
            blk = 39; continue;
        }
        38 => {
            let Some(t33) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000813040); };
            c.w32(12, t33);
            blk = 39; continue;
        }
        39 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813053); };
            c.w32(1, t34);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 440; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000813066); }
            blk = 437; continue;
        }
        42 => {
            c.w64(9, c.r64(2));
            let r35 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r35);
            c.logic(r35, 64);
            let Some(t36) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081307a); };
            c.w64(12, t36);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 49; continue; }
            blk = 43; continue;
        }
        43 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 49; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081308e); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000813095); };
            c.w64(15, t38);
            c.w64(3, c.r64(12));
            let r39 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r39);
            c.logic(r39, 64);
            c.w64(7, c.r64(15));
            let r40 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r40);
            c.logic(r40, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            blk = 49; continue;
        }
        47 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008130bd); };
            c.w32(7, t41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008130c0); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008130cd); };
            c.w32(7, t43);
            let r44 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r44);
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008130d3); };
            c.cmp(t45, c.r32(7), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t46) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008130e0); };
            c.w32(9, t46);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t47) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008130ed); };
            c.cmp(t47, 0x834u64, 16);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(8, c.r64(12));
            blk = 53; continue;
        }
        52 => {
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008130fb); };
            c.w32(8, t48);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 445; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 441; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000813134); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081313b); };
            c.w64(6, t50);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t51) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813142); };
            c.cmp(t51, c.r64(0), 64);
            if c.cc_be() { blk = 443; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081314c); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 58; continue;
        }
        58 => {
            let r52 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000813156); };
            c.w64(0, t53);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081315d); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000813161); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000813165); };
            c.w64(12, t54);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081316c); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let r55 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r55);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081318c); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 59; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 65; continue; }
            blk = 61; continue;
        }
        61 => {
            let r56 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r56);
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008131a9); };
            c.w32(9, t57);
            let r58 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008131b1); };
            c.cmp(t59, c.r32(9), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let r60 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r60);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x70008131c6); }
            blk = 64; continue;
        }
        63 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x70008131d8); }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 61; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008131e2); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r61 = c.r64(7) & c.r64(2);
            c.w64(7, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008131f6); };
            c.logic(t62 & 0x4u64, 8);
            if c.cc_ne() { blk = 566; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000813200); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000813204); };
            c.w64(6, t64);
            blk = 68; continue;
        }
        68 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000813208); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r65 = c.r64(7) & c.r64(2);
            c.w64(7, r65);
            c.logic(r65, 64);
            let Some(t66) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813216); };
            c.logic(t66 & 0x4u64, 8);
            if c.cc_ne() { blk = 563; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t67) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813220); };
            c.w32(7, t67);
            let r68 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r68);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t69) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081322b); };
            c.w32(8, t69);
            let r70 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r70);
            let Some(t71) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813232); };
            c.cmp(t71, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081323e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 72; continue;
        }
        72 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t72) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000813261); };
            c.w64(9, t72);
            let r73 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r73);
            let Some(t74) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x700081326b); };
            c.w64(12, t74);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 76; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 438; continue;
        }
        75 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 76; continue;
        }
        76 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 80; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w64(9, c.r64(0));
            let Some(t75) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813289); };
            c.w32(12, t75);
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081328d); };
            c.cmp(t76, c.r32(12), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813296); }
            blk = 81; continue;
        }
        79 => {
            c.w64(0, c.r64(9));
            blk = 438; continue;
        }
        80 => {
            c.w64(12, c.r64(0));
            let r77 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r77);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            c.w64(9, c.r64(0));
            blk = 81; continue;
        }
        81 => {
            let Some(t78) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008132b8); };
            c.w32(12, t78);
            let r79 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r79);
            let Some(t80) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008132bf); };
            c.cmp(t80, 0x834u64, 16);
            if c.cc_e() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t81) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008132c9); };
            c.w32(12, t81);
            blk = 84; continue;
        }
        83 => {
            let Some(t82) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008132cf); };
            c.w32(12, t82);
            blk = 84; continue;
        }
        84 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(0, c.r64(9));
            blk = 438; continue;
        }
        86 => {
            let Some(t83) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008132e6); };
            c.w32(1, t83);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 447; continue; }
            blk = 87; continue;
        }
        87 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000813302); }
            c.w64(0, c.r64(9));
            blk = 437; continue;
        }
        88 => {
            c.w64(9, c.r64(2));
            let r84 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081331b); };
            c.w64(12, t85);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 95; continue; }
            blk = 89; continue;
        }
        89 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 95; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081332f); };
            c.w64(15, t86);
            let Some(t87) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000813336); };
            c.w64(15, t87);
            c.w64(3, c.r64(12));
            let r88 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r88);
            c.logic(r88, 64);
            c.w64(7, c.r64(15));
            let r89 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r89);
            c.logic(r89, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 93; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 94; continue; }
            blk = 92; continue;
        }
        92 => {
            blk = 95; continue;
        }
        93 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t90) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081335e); };
            c.w32(7, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000813361); };
            c.cmp(t91, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t92) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081336e); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000813374); };
            c.cmp(t94, c.r32(7), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t95) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813381); };
            c.w32(9, t95);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t96) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081338e); };
            c.cmp(t96, 0x834u64, 16);
            if c.cc_e() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(8, c.r64(12));
            blk = 99; continue;
        }
        98 => {
            let Some(t97) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081339c); };
            c.w32(8, t97);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 452; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 448; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008133d5); };
            c.w64(3, t98);
            let Some(t99) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008133dc); };
            c.w64(6, t99);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t100) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008133e3); };
            c.cmp(t100, c.r64(0), 64);
            if c.cc_be() { blk = 450; continue; }
            blk = 103; continue;
        }
        103 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x70008133ed); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 104; continue;
        }
        104 => {
            let r101 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r101);
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008133f7); };
            c.w64(0, t102);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008133fe); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000813402); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000813406); };
            c.w64(12, t103);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081340d); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let r104 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r104);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081342d); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 105; continue; }
            blk = 106; continue;
        }
        106 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 107; continue;
        }
        107 => {
            let r105 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r105);
            let Some(t106) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081344a); };
            c.w32(9, t106);
            let r107 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000813452); };
            c.cmp(t108, c.r32(9), 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let r109 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r109);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x7000813467); }
            blk = 110; continue;
        }
        109 => {
            let Some(t110) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x700081346f); };
            c.w64(9, t110);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x7000813476); }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 107; continue; }
            blk = 111; continue;
        }
        111 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000813480); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r111 = c.r64(7) & c.r64(2);
            c.w64(7, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813494); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 560; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081349e); };
            c.w64(0, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008134a2); };
            c.w64(6, t114);
            blk = 114; continue;
        }
        114 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008134a6); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r115 = c.r64(7) & c.r64(2);
            c.w64(7, r115);
            c.logic(r115, 64);
            let Some(t116) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008134b4); };
            c.logic(t116 & 0x4u64, 8);
            if c.cc_ne() { blk = 557; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t117) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008134be); };
            c.w32(7, t117);
            let r118 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r118);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t119) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008134c9); };
            c.w32(8, t119);
            let r120 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r120);
            let Some(t121) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008134d0); };
            c.cmp(t121, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008134dc); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 118; continue;
        }
        118 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t122) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x70008134ff); };
            c.w64(9, t122);
            let r123 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r123);
            let Some(t124) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000813509); };
            c.w64(12, t124);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 122; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 438; continue;
        }
        121 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 122; continue;
        }
        122 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t125) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813524); };
            c.w32(9, t125);
            let Some(t126) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000813528); };
            c.cmp(t126, c.r32(9), 32);
            if c.cc_ne() { blk = 438; continue; }
            blk = 124; continue;
        }
        124 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813535); }
            blk = 126; continue;
        }
        125 => {
            c.w64(9, c.r64(0));
            let r127 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r127);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 126; continue;
        }
        126 => {
            let Some(t128) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081354b); };
            c.w32(9, t128);
            let r129 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r129);
            let Some(t130) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813552); };
            c.cmp(t130, 0x834u64, 16);
            if c.cc_e() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t131) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081355b); };
            c.w32(9, t131);
            blk = 129; continue;
        }
        128 => {
            let Some(t132) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000813561); };
            c.w32(9, t132);
            blk = 129; continue;
        }
        129 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t133) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813574); };
            c.w32(1, t133);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 454; continue; }
            blk = 131; continue;
        }
        131 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000813590); }
            blk = 437; continue;
        }
        132 => {
            c.w64(8, c.r64(2));
            let r134 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r134);
            c.logic(r134, 64);
            let Some(t135) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008135a6); };
            c.w64(9, t135);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 139; continue; }
            blk = 133; continue;
        }
        133 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 139; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008135ba); };
            c.w64(12, t136);
            let Some(t137) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008135c1); };
            c.w64(12, t137);
            c.w64(15, c.r64(9));
            let r138 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r138);
            c.logic(r138, 64);
            c.w64(3, c.r64(12));
            let r139 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r139);
            c.logic(r139, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 137; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 138; continue; }
            blk = 136; continue;
        }
        136 => {
            blk = 139; continue;
        }
        137 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t140) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008135ea); };
            c.w32(8, t140);
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008135ee); };
            c.cmp(t141, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008135fb); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r142 = c.r64(8) & c.r64(2);
            c.w64(8, r142);
            c.logic(r142, 64);
            let Some(t143) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813609); };
            c.logic(t143 & 0x4u64, 8);
            if c.cc_ne() { blk = 554; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t144) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813614); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            let Some(t146) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081361b); };
            c.w32(9, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081361f); };
            c.cmp(t147, c.r32(9), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 141; continue;
        }
        141 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t148) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813631); };
            c.w32(9, t148);
            let r149 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r149);
            let Some(t150) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813638); };
            c.cmp(t150, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 143; continue;
        }
        143 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813644); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 144; continue;
        }
        144 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 145; continue;
        }
        145 => {
            let Some(t151) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000813667); };
            c.w64(12, t151);
            let r152 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r152);
            let Some(t153) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000813671); };
            c.w64(15, t153);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 148; continue; }
            blk = 146; continue;
        }
        146 => {
            blk = 438; continue;
        }
        147 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 148; continue;
        }
        148 => {
            let Some(t154) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813688); };
            c.w32(12, t154);
            let r155 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r155);
            let Some(t156) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081368f); };
            c.cmp(t156, 0x834u64, 16);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813699); };
            c.w32(12, t157);
            blk = 151; continue;
        }
        150 => {
            let Some(t158) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081369f); };
            c.w32(12, t158);
            blk = 151; continue;
        }
        151 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008136b2); };
            c.w32(1, t159);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 455; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008136cd); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r160 = c.r64(7) & c.r64(8);
            c.w64(7, r160);
            c.logic(r160, 64);
            let Some(t161) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008136df); };
            c.logic(t161 & 0x4u64, 8);
            if c.cc_ne() { blk = 551; continue; }
            blk = 155; continue;
        }
        155 => {
            blk = 437; continue;
        }
        156 => {
            c.w64(8, c.r64(2));
            let r162 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r162);
            c.logic(r162, 64);
            let Some(t163) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008136f8); };
            c.w64(9, t163);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 163; continue; }
            blk = 157; continue;
        }
        157 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 163; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081370c); };
            c.w64(12, t164);
            let Some(t165) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000813713); };
            c.w64(12, t165);
            c.w64(15, c.r64(9));
            let r166 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r166);
            c.logic(r166, 64);
            c.w64(3, c.r64(12));
            let r167 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r167);
            c.logic(r167, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 161; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 162; continue; }
            blk = 160; continue;
        }
        160 => {
            blk = 163; continue;
        }
        161 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t168) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081373c); };
            c.w32(8, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000813740); };
            c.cmp(t169, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 163; continue;
        }
        163 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081374d); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r170 = c.r64(8) & c.r64(2);
            c.w64(8, r170);
            c.logic(r170, 64);
            let Some(t171) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081375b); };
            c.logic(t171 & 0x4u64, 8);
            if c.cc_ne() { blk = 548; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t172) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813766); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            let Some(t174) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081376d); };
            c.w32(9, t174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000813771); };
            c.cmp(t175, c.r32(9), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 165; continue;
        }
        165 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 171; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t176) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813783); };
            c.w32(9, t176);
            let r177 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r177);
            let Some(t178) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081378a); };
            c.cmp(t178, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813796); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 168; continue;
        }
        168 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t179) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x70008137b9); };
            c.w64(12, t179);
            let r180 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r180);
            let Some(t181) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x70008137c3); };
            c.w64(15, t181);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            blk = 438; continue;
        }
        171 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 172; continue;
        }
        172 => {
            let Some(t182) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008137da); };
            c.w32(12, t182);
            let r183 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r183);
            let Some(t184) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008137e1); };
            c.cmp(t184, 0x834u64, 16);
            if c.cc_e() { blk = 174; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008137eb); };
            c.w32(12, t185);
            blk = 175; continue;
        }
        174 => {
            let Some(t186) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008137f1); };
            c.w32(12, t186);
            blk = 175; continue;
        }
        175 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t187) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813804); };
            c.w32(1, t187);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 456; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081381f); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r188 = c.r64(7) & c.r64(8);
            c.w64(7, r188);
            c.logic(r188, 64);
            let Some(t189) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813831); };
            c.logic(t189 & 0x4u64, 8);
            if c.cc_ne() { blk = 545; continue; }
            blk = 179; continue;
        }
        179 => {
            blk = 437; continue;
        }
        180 => {
            c.w64(9, c.r64(2));
            let r190 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r190);
            c.logic(r190, 64);
            let Some(t191) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081384a); };
            c.w64(12, t191);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 187; continue; }
            blk = 181; continue;
        }
        181 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 187; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081385e); };
            c.w64(15, t192);
            let Some(t193) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000813865); };
            c.w64(15, t193);
            c.w64(3, c.r64(12));
            let r194 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r194);
            c.logic(r194, 64);
            c.w64(7, c.r64(15));
            let r195 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r195);
            c.logic(r195, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 185; continue; }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 186; continue; }
            blk = 184; continue;
        }
        184 => {
            blk = 187; continue;
        }
        185 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t196) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081388d); };
            c.w32(7, t196);
            let Some(t197) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000813890); };
            c.cmp(t197, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t198) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081389d); };
            c.w32(7, t198);
            let r199 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r199);
            let Some(t200) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008138a3); };
            c.cmp(t200, c.r32(7), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t201) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008138b0); };
            c.w32(9, t201);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t202) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008138bd); };
            c.cmp(t202, 0x834u64, 16);
            if c.cc_e() { blk = 190; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w64(8, c.r64(12));
            blk = 191; continue;
        }
        190 => {
            let Some(t203) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008138cb); };
            c.w32(8, t203);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 191; continue;
        }
        191 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 192; continue;
        }
        192 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 461; continue; }
            blk = 193; continue;
        }
        193 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 457; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000813904); };
            c.w64(3, t204);
            let Some(t205) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081390b); };
            c.w64(6, t205);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t206) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813912); };
            c.cmp(t206, c.r64(0), 64);
            if c.cc_be() { blk = 459; continue; }
            blk = 195; continue;
        }
        195 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081391c); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 196; continue;
        }
        196 => {
            let r207 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r207);
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000813926); };
            c.w64(0, t208);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081392d); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000813931); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000813935); };
            c.w64(12, t209);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081393c); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            let r210 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r210);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081395c); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 197; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 203; continue; }
            blk = 199; continue;
        }
        199 => {
            let r211 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r211);
            let Some(t212) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000813979); };
            c.w32(9, t212);
            let r213 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r213);
            let Some(t214) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000813981); };
            c.cmp(t214, c.r32(9), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let r215 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r215);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x7000813996); }
            blk = 202; continue;
        }
        201 => {
            let Some(t216) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x700081399e); };
            c.w64(9, t216);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x70008139a5); }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 199; continue; }
            blk = 203; continue;
        }
        203 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008139af); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r217 = c.r64(7) & c.r64(2);
            c.w64(7, r217);
            c.logic(r217, 64);
            let Some(t218) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008139c3); };
            c.logic(t218 & 0x4u64, 8);
            if c.cc_ne() { blk = 542; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008139cd); };
            c.w64(0, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008139d1); };
            c.w64(6, t220);
            blk = 206; continue;
        }
        206 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008139d5); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r221 = c.r64(7) & c.r64(2);
            c.w64(7, r221);
            c.logic(r221, 64);
            let Some(t222) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008139e3); };
            c.logic(t222 & 0x4u64, 8);
            if c.cc_ne() { blk = 539; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t223) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008139ed); };
            c.w32(7, t223);
            let r224 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r224);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t225) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008139f8); };
            c.w32(8, t225);
            let r226 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r226);
            let Some(t227) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008139ff); };
            c.cmp(t227, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 209; continue;
        }
        209 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813a0b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 210; continue;
        }
        210 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 211; continue;
        }
        211 => {
            let Some(t228) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000813a2e); };
            c.w64(9, t228);
            let r229 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r229);
            let Some(t230) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000813a38); };
            c.w64(12, t230);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 214; continue; }
            blk = 212; continue;
        }
        212 => {
            blk = 438; continue;
        }
        213 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 214; continue;
        }
        214 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 217; continue; }
            blk = 215; continue;
        }
        215 => {
            let Some(t231) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813a53); };
            c.w32(9, t231);
            let Some(t232) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000813a57); };
            c.cmp(t232, c.r32(9), 32);
            if c.cc_ne() { blk = 438; continue; }
            blk = 216; continue;
        }
        216 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813a64); }
            blk = 218; continue;
        }
        217 => {
            c.w64(9, c.r64(0));
            let r233 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r233);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 218; continue;
        }
        218 => {
            let Some(t234) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813a7a); };
            c.w32(9, t234);
            let r235 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r235);
            let Some(t236) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813a81); };
            c.cmp(t236, 0x834u64, 16);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t237) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813a8a); };
            c.w32(9, t237);
            blk = 221; continue;
        }
        220 => {
            let Some(t238) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000813a90); };
            c.w32(9, t238);
            blk = 221; continue;
        }
        221 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t239) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813aa3); };
            c.w32(1, t239);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 463; continue; }
            blk = 223; continue;
        }
        223 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000813abf); }
            blk = 437; continue;
        }
        224 => {
            c.w64(8, c.r64(2));
            let r240 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r240);
            c.logic(r240, 64);
            let Some(t241) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813ad5); };
            c.w64(9, t241);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 225; continue;
        }
        225 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 231; continue; }
            blk = 226; continue;
        }
        226 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000813ae9); };
            c.w64(12, t242);
            let Some(t243) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000813af0); };
            c.w64(12, t243);
            c.w64(15, c.r64(9));
            let r244 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r244);
            c.logic(r244, 64);
            c.w64(3, c.r64(12));
            let r245 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r245);
            c.logic(r245, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 230; continue; }
            blk = 228; continue;
        }
        228 => {
            blk = 231; continue;
        }
        229 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 231; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t246) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000813b19); };
            c.w32(8, t246);
            let Some(t247) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000813b1d); };
            c.cmp(t247, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 231; continue;
        }
        231 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000813b2a); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r248 = c.r64(8) & c.r64(2);
            c.w64(8, r248);
            c.logic(r248, 64);
            let Some(t249) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813b38); };
            c.logic(t249 & 0x4u64, 8);
            if c.cc_ne() { blk = 536; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t250) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813b43); };
            c.w32(8, t250);
            let r251 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r251);
            let Some(t252) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813b4a); };
            c.w32(9, t252);
            let Some(t253) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000813b4e); };
            c.cmp(t253, c.r32(9), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 233; continue;
        }
        233 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 239; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t254) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813b60); };
            c.w32(9, t254);
            let r255 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r255);
            let Some(t256) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813b67); };
            c.cmp(t256, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 235; continue;
        }
        235 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813b73); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 236; continue;
        }
        236 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 237; continue;
        }
        237 => {
            let Some(t257) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000813b96); };
            c.w64(12, t257);
            let r258 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r258);
            let Some(t259) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000813ba0); };
            c.w64(15, t259);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 240; continue; }
            blk = 238; continue;
        }
        238 => {
            blk = 438; continue;
        }
        239 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 240; continue;
        }
        240 => {
            let Some(t260) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813bb7); };
            c.w32(12, t260);
            let r261 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r261);
            let Some(t262) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813bbe); };
            c.cmp(t262, 0x834u64, 16);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t263) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813bc8); };
            c.w32(12, t263);
            blk = 243; continue;
        }
        242 => {
            let Some(t264) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000813bce); };
            c.w32(12, t264);
            blk = 243; continue;
        }
        243 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t265) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813be1); };
            c.w32(1, t265);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 464; continue; }
            blk = 245; continue;
        }
        245 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000813bfc); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r266 = c.r64(7) & c.r64(8);
            c.w64(7, r266);
            c.logic(r266, 64);
            let Some(t267) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813c0e); };
            c.logic(t267 & 0x4u64, 8);
            if c.cc_ne() { blk = 533; continue; }
            blk = 247; continue;
        }
        247 => {
            blk = 437; continue;
        }
        248 => {
            c.w64(8, c.r64(2));
            let r268 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r268);
            c.logic(r268, 64);
            let Some(t269) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813c27); };
            c.w64(9, t269);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 255; continue; }
            blk = 249; continue;
        }
        249 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 255; continue; }
            blk = 250; continue;
        }
        250 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t270) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000813c3b); };
            c.w64(12, t270);
            let Some(t271) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000813c42); };
            c.w64(12, t271);
            c.w64(15, c.r64(9));
            let r272 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r272);
            c.logic(r272, 64);
            c.w64(3, c.r64(12));
            let r273 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r273);
            c.logic(r273, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 253; continue; }
            blk = 251; continue;
        }
        251 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 254; continue; }
            blk = 252; continue;
        }
        252 => {
            blk = 255; continue;
        }
        253 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            let Some(t274) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000813c6b); };
            c.w32(8, t274);
            let Some(t275) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000813c6f); };
            c.cmp(t275, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 255; continue;
        }
        255 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000813c7c); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r276 = c.r64(8) & c.r64(2);
            c.w64(8, r276);
            c.logic(r276, 64);
            let Some(t277) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813c8a); };
            c.logic(t277 & 0x4u64, 8);
            if c.cc_ne() { blk = 530; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t278) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813c95); };
            c.w32(8, t278);
            let r279 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r279);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 262; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t280) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813ca1); };
            c.w32(9, t280);
            let r281 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r281);
            let Some(t282) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813ca8); };
            c.cmp(t282, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 258; continue;
        }
        258 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813cb4); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 259; continue;
        }
        259 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 260; continue;
        }
        260 => {
            let Some(t283) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000813cd7); };
            c.w64(12, t283);
            let r284 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r284);
            let Some(t285) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000813ce1); };
            c.w64(15, t285);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 263; continue; }
            blk = 261; continue;
        }
        261 => {
            blk = 438; continue;
        }
        262 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 263; continue;
        }
        263 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 266; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t286) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813cfc); };
            c.w32(12, t286);
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000813d00); };
            c.cmp(t287, c.r32(12), 32);
            if c.cc_ne() { blk = 438; continue; }
            blk = 265; continue;
        }
        265 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813d0d); }
            blk = 267; continue;
        }
        266 => {
            c.w64(12, c.r64(0));
            let r288 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r288);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 267; continue;
        }
        267 => {
            let Some(t289) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813d23); };
            c.w32(12, t289);
            let r290 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r290);
            let Some(t291) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813d2a); };
            c.cmp(t291, 0x834u64, 16);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t292) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813d34); };
            c.w32(12, t292);
            blk = 270; continue;
        }
        269 => {
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000813d3a); };
            c.w32(12, t293);
            blk = 270; continue;
        }
        270 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t294) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813d4d); };
            c.w32(1, t294);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 465; continue; }
            blk = 272; continue;
        }
        272 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000813d6a); }
            blk = 437; continue;
        }
        273 => {
            c.w64(9, c.r64(2));
            let r295 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r295);
            c.logic(r295, 64);
            let Some(t296) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813d80); };
            c.w64(12, t296);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 280; continue; }
            blk = 274; continue;
        }
        274 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 280; continue; }
            blk = 275; continue;
        }
        275 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000813d94); };
            c.w64(15, t297);
            let Some(t298) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000813d9b); };
            c.w64(15, t298);
            c.w64(3, c.r64(12));
            let r299 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r299);
            c.logic(r299, 64);
            c.w64(7, c.r64(15));
            let r300 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r300);
            c.logic(r300, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 278; continue; }
            blk = 276; continue;
        }
        276 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 279; continue; }
            blk = 277; continue;
        }
        277 => {
            blk = 280; continue;
        }
        278 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 280; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t301) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000813dc3); };
            c.w32(7, t301);
            let Some(t302) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000813dc6); };
            c.cmp(t302, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t303) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813dd3); };
            c.w32(7, t303);
            let r304 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r304);
            let Some(t305) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000813dd9); };
            c.cmp(t305, c.r32(7), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 281; continue;
        }
        281 => {
            let Some(t306) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813de6); };
            c.w32(9, t306);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t307) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813df3); };
            c.cmp(t307, 0x834u64, 16);
            if c.cc_e() { blk = 283; continue; }
            blk = 282; continue;
        }
        282 => {
            c.w64(8, c.r64(12));
            blk = 284; continue;
        }
        283 => {
            let Some(t308) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000813e01); };
            c.w32(8, t308);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 284; continue;
        }
        284 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 285; continue;
        }
        285 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 470; continue; }
            blk = 286; continue;
        }
        286 => {
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000813e25); };
            c.w64(15, t309);
            let Some(t310) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000813e2c); };
            c.w64(3, t310);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000813e3b); };
            c.w64(1, t311);
            let Some(t312) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000813e42); };
            c.cmp(t312, c.r64(0), 64);
            if c.cc_be() { blk = 466; continue; }
            blk = 287; continue;
        }
        287 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x7000813e4b); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 288; continue;
        }
        288 => {
            let r313 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r313);
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000813e55); };
            c.w64(15, t314);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000813e5c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000813e60); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 291; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t315) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000813e72); };
            c.w64(12, t315);
            blk = 290; continue;
        }
        290 => {
            let r316 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r316);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000813e7d); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 290; continue; }
            blk = 291; continue;
        }
        291 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 300; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t317) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x7000813ea1); };
            c.w64(12, t317);
            blk = 294; continue;
        }
        293 => {
            let r318 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r318);
            blk = 294; continue;
        }
        294 => {
            let r319 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r319);
            let Some(t320) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000813eb2); };
            c.cmp(t320, c.r64(12), 64);
            if c.cc_e() { blk = 299; continue; }
            blk = 295; continue;
        }
        295 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x7000813eb8); }
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000813ebe); };
            c.w64(15, t321);
            let Some(t322) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000813ec5); };
            c.w64(0, t322);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000813ed0); };
            c.w64(11, t323);
            let Some(t324) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000813ed7); };
            c.cmp(t324, c.r64(3), 64);
            if c.cc_be() { blk = 468; continue; }
            blk = 296; continue;
        }
        296 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x7000813ee0); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 297; continue;
        }
        297 => {
            let Some(t325) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000813ee7); };
            c.w64(15, t325);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000813eee); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000813ef2); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x7000813ef8); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r326 = c.r64(15) & c.r64(1);
            c.w64(15, r326);
            c.logic(r326, 64);
            let Some(t327) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813f0c); };
            c.logic(t327 & 0x4u64, 8);
            if c.cc_ne() { blk = 527; continue; }
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 293; continue; }
            blk = 300; continue;
        }
        300 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000813f1d); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r328 = c.r64(7) & c.r64(2);
            c.w64(7, r328);
            c.logic(r328, 64);
            let Some(t329) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813f2f); };
            c.logic(t329 & 0x4u64, 8);
            if c.cc_ne() { blk = 524; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000813f39); };
            c.w64(1, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000813f3d); };
            c.w64(0, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000813f41); };
            c.w64(11, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000813f45); };
            c.w64(6, t333);
            blk = 303; continue;
        }
        303 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000813f49); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r334 = c.r64(7) & c.r64(2);
            c.w64(7, r334);
            c.logic(r334, 64);
            let Some(t335) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000813f57); };
            c.logic(t335 & 0x4u64, 8);
            if c.cc_ne() { blk = 521; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t336) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813f61); };
            c.w32(7, t336);
            let r337 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r337);
            let Some(t338) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813f67); };
            c.w32(8, t338);
            let Some(t339) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000813f6b); };
            c.cmp(t339, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 305; continue;
        }
        305 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 311; continue; }
            blk = 306; continue;
        }
        306 => {
            let Some(t340) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813f7d); };
            c.w32(8, t340);
            let r341 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r341);
            let Some(t342) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813f84); };
            c.cmp(t342, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 307; continue;
        }
        307 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000813f90); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 308; continue;
        }
        308 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t343) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000813fb3); };
            c.w64(9, t343);
            let r344 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r344);
            let Some(t345) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000813fbd); };
            c.w64(12, t345);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 312; continue; }
            blk = 310; continue;
        }
        310 => {
            blk = 438; continue;
        }
        311 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 312; continue;
        }
        312 => {
            let Some(t346) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000813fd4); };
            c.w32(9, t346);
            let r347 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r347);
            let Some(t348) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000813fdb); };
            c.cmp(t348, 0x834u64, 16);
            if c.cc_e() { blk = 314; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t349) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813fe4); };
            c.w32(9, t349);
            blk = 315; continue;
        }
        314 => {
            let Some(t350) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000813fea); };
            c.w32(9, t350);
            blk = 315; continue;
        }
        315 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t351) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000813ffd); };
            c.w32(1, t351);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 472; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000814017); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r352 = c.r64(8) & c.r64(7);
            c.w64(8, r352);
            c.logic(r352, 64);
            let Some(t353) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814028); };
            c.logic(t353 & 0x4u64, 8);
            if c.cc_ne() { blk = 518; continue; }
            blk = 319; continue;
        }
        319 => {
            blk = 437; continue;
        }
        320 => {
            c.w64(9, c.r64(2));
            let r354 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r354);
            c.logic(r354, 64);
            let Some(t355) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814042); };
            c.w64(12, t355);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 327; continue; }
            blk = 321; continue;
        }
        321 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 327; continue; }
            blk = 322; continue;
        }
        322 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000814056); };
            c.w64(15, t356);
            let Some(t357) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081405d); };
            c.w64(15, t357);
            c.w64(3, c.r64(12));
            let r358 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r358);
            c.logic(r358, 64);
            c.w64(7, c.r64(15));
            let r359 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r359);
            c.logic(r359, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 325; continue; }
            blk = 323; continue;
        }
        323 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 326; continue; }
            blk = 324; continue;
        }
        324 => {
            blk = 327; continue;
        }
        325 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 327; continue; }
            blk = 326; continue;
        }
        326 => {
            let Some(t360) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000814085); };
            c.w32(7, t360);
            let Some(t361) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000814088); };
            c.cmp(t361, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t362) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814095); };
            c.w32(7, t362);
            let r363 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r363);
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081409b); };
            c.cmp(t364, c.r32(7), 32);
            if c.cc_e() { blk = 350; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t365) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008140a8); };
            c.w32(9, t365);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t366) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008140b5); };
            c.cmp(t366, 0x834u64, 16);
            if c.cc_e() { blk = 330; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w64(8, c.r64(12));
            blk = 331; continue;
        }
        330 => {
            let Some(t367) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008140c3); };
            c.w32(8, t367);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 331; continue;
        }
        331 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 332; continue;
        }
        332 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 477; continue; }
            blk = 333; continue;
        }
        333 => {
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008140e7); };
            c.w64(15, t368);
            let Some(t369) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008140ee); };
            c.w64(3, t369);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008140fd); };
            c.w64(1, t370);
            let Some(t371) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000814104); };
            c.cmp(t371, c.r64(0), 64);
            if c.cc_be() { blk = 473; continue; }
            blk = 334; continue;
        }
        334 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x700081410d); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 335; continue;
        }
        335 => {
            let r372 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r372);
            let Some(t373) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000814117); };
            c.w64(15, t373);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081411e); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000814122); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 338; continue; }
            blk = 336; continue;
        }
        336 => {
            let Some(t374) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000814134); };
            c.w64(12, t374);
            blk = 337; continue;
        }
        337 => {
            let r375 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r375);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081413f); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 337; continue; }
            blk = 338; continue;
        }
        338 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 347; continue; }
            blk = 339; continue;
        }
        339 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t376) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x7000814163); };
            c.w64(12, t376);
            blk = 341; continue;
        }
        340 => {
            let r377 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r377);
            blk = 341; continue;
        }
        341 => {
            let r378 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r378);
            let Some(t379) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000814174); };
            c.cmp(t379, c.r64(12), 64);
            if c.cc_e() { blk = 346; continue; }
            blk = 342; continue;
        }
        342 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081417a); }
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000814180); };
            c.w64(15, t380);
            let Some(t381) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000814187); };
            c.w64(0, t381);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t382) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000814192); };
            c.w64(11, t382);
            let Some(t383) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000814199); };
            c.cmp(t383, c.r64(3), 64);
            if c.cc_be() { blk = 475; continue; }
            blk = 343; continue;
        }
        343 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008141a2); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 344; continue;
        }
        344 => {
            let Some(t384) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008141a9); };
            c.w64(15, t384);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008141b0); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008141b4); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x70008141ba); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 346; continue; }
            blk = 345; continue;
        }
        345 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r385 = c.r64(15) & c.r64(1);
            c.w64(15, r385);
            c.logic(r385, 64);
            let Some(t386) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008141ce); };
            c.logic(t386 & 0x4u64, 8);
            if c.cc_ne() { blk = 515; continue; }
            blk = 346; continue;
        }
        346 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 340; continue; }
            blk = 347; continue;
        }
        347 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008141df); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r387 = c.r64(7) & c.r64(2);
            c.w64(7, r387);
            c.logic(r387, 64);
            let Some(t388) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008141f1); };
            c.logic(t388 & 0x4u64, 8);
            if c.cc_ne() { blk = 512; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008141fb); };
            c.w64(1, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008141ff); };
            c.w64(0, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000814203); };
            c.w64(11, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000814207); };
            c.w64(6, t392);
            blk = 350; continue;
        }
        350 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081420b); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r393 = c.r64(7) & c.r64(2);
            c.w64(7, r393);
            c.logic(r393, 64);
            let Some(t394) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814219); };
            c.logic(t394 & 0x4u64, 8);
            if c.cc_ne() { blk = 509; continue; }
            blk = 351; continue;
        }
        351 => {
            let Some(t395) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814223); };
            c.w32(7, t395);
            let r396 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r396);
            let Some(t397) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000814229); };
            c.w32(8, t397);
            let Some(t398) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081422d); };
            c.cmp(t398, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 352; continue;
        }
        352 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 358; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t399) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081423f); };
            c.w32(8, t399);
            let r400 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r400);
            let Some(t401) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814246); };
            c.cmp(t401, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 354; continue;
        }
        354 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000814252); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 355; continue;
        }
        355 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t402) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000814275); };
            c.w64(9, t402);
            let r403 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r403);
            let Some(t404) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x700081427f); };
            c.w64(12, t404);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 359; continue; }
            blk = 357; continue;
        }
        357 => {
            blk = 438; continue;
        }
        358 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 359; continue;
        }
        359 => {
            let Some(t405) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000814296); };
            c.w32(9, t405);
            let r406 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r406);
            let Some(t407) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081429d); };
            c.cmp(t407, 0x834u64, 16);
            if c.cc_e() { blk = 361; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t408) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008142a6); };
            c.w32(9, t408);
            blk = 362; continue;
        }
        361 => {
            let Some(t409) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008142ac); };
            c.w32(9, t409);
            blk = 362; continue;
        }
        362 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 363; continue;
        }
        363 => {
            let Some(t410) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008142bf); };
            c.w32(1, t410);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 479; continue; }
            blk = 364; continue;
        }
        364 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008142d9); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 366; continue; }
            blk = 365; continue;
        }
        365 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r411 = c.r64(8) & c.r64(7);
            c.w64(8, r411);
            c.logic(r411, 64);
            let Some(t412) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008142ea); };
            c.logic(t412 & 0x4u64, 8);
            if c.cc_ne() { blk = 506; continue; }
            blk = 366; continue;
        }
        366 => {
            blk = 437; continue;
        }
        367 => {
            c.w64(9, c.r64(2));
            let r413 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r413);
            c.logic(r413, 64);
            let Some(t414) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814304); };
            c.w64(12, t414);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 374; continue; }
            blk = 368; continue;
        }
        368 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 374; continue; }
            blk = 369; continue;
        }
        369 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000814318); };
            c.w64(15, t415);
            let Some(t416) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081431f); };
            c.w64(15, t416);
            c.w64(3, c.r64(12));
            let r417 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r417);
            c.logic(r417, 64);
            c.w64(7, c.r64(15));
            let r418 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r418);
            c.logic(r418, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 372; continue; }
            blk = 370; continue;
        }
        370 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 373; continue; }
            blk = 371; continue;
        }
        371 => {
            blk = 374; continue;
        }
        372 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 374; continue; }
            blk = 373; continue;
        }
        373 => {
            let Some(t419) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000814347); };
            c.w32(7, t419);
            let Some(t420) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081434a); };
            c.cmp(t420, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t421) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814357); };
            c.w32(7, t421);
            let r422 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r422);
            let Some(t423) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081435d); };
            c.cmp(t423, c.r32(7), 32);
            if c.cc_e() { blk = 397; continue; }
            blk = 375; continue;
        }
        375 => {
            let Some(t424) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081436a); };
            c.w32(9, t424);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t425) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814377); };
            c.cmp(t425, 0x834u64, 16);
            if c.cc_e() { blk = 377; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(8, c.r64(12));
            blk = 378; continue;
        }
        377 => {
            let Some(t426) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000814385); };
            c.w32(8, t426);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 378; continue;
        }
        378 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 379; continue;
        }
        379 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 484; continue; }
            blk = 380; continue;
        }
        380 => {
            let Some(t427) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008143a9); };
            c.w64(15, t427);
            let Some(t428) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008143b0); };
            c.w64(3, t428);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008143bf); };
            c.w64(1, t429);
            let Some(t430) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008143c6); };
            c.cmp(t430, c.r64(0), 64);
            if c.cc_be() { blk = 480; continue; }
            blk = 381; continue;
        }
        381 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x70008143cf); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 382; continue;
        }
        382 => {
            let r431 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r431);
            let Some(t432) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008143d9); };
            c.w64(15, t432);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008143e0); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008143e4); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 385; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008143f6); };
            c.w64(12, t433);
            blk = 384; continue;
        }
        384 => {
            let r434 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r434);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x7000814401); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 384; continue; }
            blk = 385; continue;
        }
        385 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 394; continue; }
            blk = 386; continue;
        }
        386 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t435) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x7000814425); };
            c.w64(12, t435);
            blk = 388; continue;
        }
        387 => {
            let r436 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r436);
            blk = 388; continue;
        }
        388 => {
            let r437 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r437);
            let Some(t438) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000814436); };
            c.cmp(t438, c.r64(12), 64);
            if c.cc_e() { blk = 393; continue; }
            blk = 389; continue;
        }
        389 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081443c); }
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000814442); };
            c.w64(15, t439);
            let Some(t440) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000814449); };
            c.w64(0, t440);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000814454); };
            c.w64(11, t441);
            let Some(t442) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700081445b); };
            c.cmp(t442, c.r64(3), 64);
            if c.cc_be() { blk = 482; continue; }
            blk = 390; continue;
        }
        390 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x7000814464); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 391; continue;
        }
        391 => {
            let Some(t443) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081446b); };
            c.w64(15, t443);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000814472); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000814476); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x700081447c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 393; continue; }
            blk = 392; continue;
        }
        392 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r444 = c.r64(15) & c.r64(1);
            c.w64(15, r444);
            c.logic(r444, 64);
            let Some(t445) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814490); };
            c.logic(t445 & 0x4u64, 8);
            if c.cc_ne() { blk = 503; continue; }
            blk = 393; continue;
        }
        393 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 387; continue; }
            blk = 394; continue;
        }
        394 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008144a1); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 396; continue; }
            blk = 395; continue;
        }
        395 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r446 = c.r64(7) & c.r64(2);
            c.w64(7, r446);
            c.logic(r446, 64);
            let Some(t447) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008144b3); };
            c.logic(t447 & 0x4u64, 8);
            if c.cc_ne() { blk = 500; continue; }
            blk = 396; continue;
        }
        396 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008144bd); };
            c.w64(1, t448);
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008144c1); };
            c.w64(0, t449);
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008144c5); };
            c.w64(11, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008144c9); };
            c.w64(6, t451);
            blk = 397; continue;
        }
        397 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008144cd); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r452 = c.r64(7) & c.r64(2);
            c.w64(7, r452);
            c.logic(r452, 64);
            let Some(t453) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008144db); };
            c.logic(t453 & 0x4u64, 8);
            if c.cc_ne() { blk = 497; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t454) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008144e5); };
            c.w32(7, t454);
            let r455 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r455);
            let Some(t456) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008144eb); };
            c.w32(8, t456);
            let Some(t457) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008144ef); };
            c.cmp(t457, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 399; continue;
        }
        399 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 405; continue; }
            blk = 400; continue;
        }
        400 => {
            let Some(t458) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000814501); };
            c.w32(8, t458);
            let r459 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r459);
            let Some(t460) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814508); };
            c.cmp(t460, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 401; continue;
        }
        401 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000814514); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 402; continue;
        }
        402 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 403; continue;
        }
        403 => {
            let Some(t461) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000814537); };
            c.w64(9, t461);
            let r462 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r462);
            let Some(t463) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000814541); };
            c.w64(12, t463);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 406; continue; }
            blk = 404; continue;
        }
        404 => {
            blk = 438; continue;
        }
        405 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 406; continue;
        }
        406 => {
            let Some(t464) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000814558); };
            c.w32(9, t464);
            let r465 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r465);
            let Some(t466) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081455f); };
            c.cmp(t466, 0x834u64, 16);
            if c.cc_e() { blk = 408; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t467) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000814568); };
            c.w32(9, t467);
            blk = 409; continue;
        }
        408 => {
            let Some(t468) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081456e); };
            c.w32(9, t468);
            blk = 409; continue;
        }
        409 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 410; continue;
        }
        410 => {
            let Some(t469) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000814581); };
            c.w32(1, t469);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 486; continue; }
            blk = 411; continue;
        }
        411 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081459b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 413; continue; }
            blk = 412; continue;
        }
        412 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r470 = c.r64(8) & c.r64(7);
            c.w64(8, r470);
            c.logic(r470, 64);
            let Some(t471) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008145ac); };
            c.logic(t471 & 0x4u64, 8);
            if c.cc_ne() { blk = 494; continue; }
            blk = 413; continue;
        }
        413 => {
            blk = 437; continue;
        }
        414 => {
            c.w64(8, c.r64(2));
            let r472 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r472);
            c.logic(r472, 64);
            let Some(t473) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008145c6); };
            c.w64(9, t473);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 421; continue; }
            blk = 415; continue;
        }
        415 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 421; continue; }
            blk = 416; continue;
        }
        416 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008145da); };
            c.w64(12, t474);
            let Some(t475) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008145e1); };
            c.w64(12, t475);
            c.w64(15, c.r64(9));
            let r476 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r476);
            c.logic(r476, 64);
            c.w64(3, c.r64(12));
            let r477 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r477);
            c.logic(r477, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 419; continue; }
            blk = 417; continue;
        }
        417 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 420; continue; }
            blk = 418; continue;
        }
        418 => {
            blk = 421; continue;
        }
        419 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 421; continue; }
            blk = 420; continue;
        }
        420 => {
            let Some(t478) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081460a); };
            c.w32(8, t478);
            let Some(t479) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081460e); };
            c.cmp(t479, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 421; continue;
        }
        421 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081461b); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r480 = c.r64(8) & c.r64(2);
            c.w64(8, r480);
            c.logic(r480, 64);
            let Some(t481) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814629); };
            c.logic(t481 & 0x4u64, 8);
            if c.cc_ne() { blk = 491; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t482) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814634); };
            c.w32(8, t482);
            let r483 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r483);
            let Some(t484) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081463b); };
            c.w32(9, t484);
            let Some(t485) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081463f); };
            c.cmp(t485, c.r32(9), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 423; continue;
        }
        423 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 429; continue; }
            blk = 424; continue;
        }
        424 => {
            let Some(t486) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000814651); };
            c.w32(9, t486);
            let r487 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r487);
            let Some(t488) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000814658); };
            c.cmp(t488, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 425; continue;
        }
        425 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000814664); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 438; continue; }
            blk = 426; continue;
        }
        426 => {
            if c.cc_ne() { blk = 438; continue; }
            blk = 427; continue;
        }
        427 => {
            let Some(t489) = c.ld64(0x7000812ffcu64) else { return RegfileExit::fault(0x7000814687); };
            c.w64(12, t489);
            let r490 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r490);
            let Some(t491) = c.ld64(0x7000813009u64) else { return RegfileExit::fault(0x7000814691); };
            c.w64(15, t491);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 430; continue; }
            blk = 428; continue;
        }
        428 => {
            blk = 438; continue;
        }
        429 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 430; continue;
        }
        430 => {
            let Some(t492) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008146a5); };
            c.w32(12, t492);
            let r493 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r493);
            let Some(t494) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008146ac); };
            c.cmp(t494, 0x834u64, 16);
            if c.cc_e() { blk = 432; continue; }
            blk = 431; continue;
        }
        431 => {
            let Some(t495) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008146b6); };
            c.w32(12, t495);
            blk = 433; continue;
        }
        432 => {
            let Some(t496) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008146bc); };
            c.w32(12, t496);
            blk = 433; continue;
        }
        433 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 438; continue; }
            blk = 434; continue;
        }
        434 => {
            let Some(t497) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008146cb); };
            c.w32(1, t497);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 487; continue; }
            blk = 435; continue;
        }
        435 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008146e6); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 437; continue; }
            blk = 436; continue;
        }
        436 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r498 = c.r64(7) & c.r64(8);
            c.w64(7, r498);
            c.logic(r498, 64);
            let Some(t499) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008146f8); };
            c.logic(t499 & 0x4u64, 8);
            if c.cc_ne() { blk = 488; continue; }
            blk = 437; continue;
        }
        437 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000814705); }
            return c.ret(0x8, 0x7000814706);
        }
        438 => {
            let Some(t500) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000814709); };
            c.w64(5, t500);
            let Some(t501) = c.ld64(c.r64(13).wrapping_add(0x2010)) else { return RegfileExit::fault(0x700081470d); };
            c.w64(3, t501);
            let Some(t502) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x7000814714); };
            c.w64(10, t502);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x7000814719); }
            let Some(t503) = c.ld64(c.r64(4).wrapping_add(0x78)) else { return RegfileExit::fault(0x700081471e); };
            c.w64(10, t503);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x7000814723); }
            if !c.st64(c.r64(4).wrapping_add(0x78), c.r64(2)) { return RegfileExit::fault(0x7000814728); }
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(1)) { return RegfileExit::fault(0x700081472d); }
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(0)) { return RegfileExit::fault(0x7000814732); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(11)) { return RegfileExit::fault(0x7000814737); }
            let Some(t504) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700081473c); };
            c.w64(10, t504);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000814741); }
            c.w32(0, 0x6u64);
            let r505 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r505);
            return RegfileExit::jump(0x700086ed74);
        }
        439 => {
            return RegfileExit::fault(0x7000814754);
        }
        440 => {
            return RegfileExit::fault(0x7000814755);
        }
        441 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t506) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000814758); };
            c.w64(1, t506);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x7000814769, 0x7000814764);
        }
        442 => {
            return RegfileExit::fault(0x7000814769);
        }
        443 => {
            let r507 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r507);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081476d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000814771); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000814772); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000814776); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081477a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081477e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000814782); }
            c.w32(0, 0x2u64);
            let Some(t508) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081478b); };
            c.w64(3, t508);
            return c.call(0x700086ed74, 0x7000814797, 0x7000814792);
        }
        444 => {
            c.w64(15, c.r64(0));
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081479a); };
            c.w64(2, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081479e); };
            c.w64(1, t510);
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008147a2); };
            c.w64(11, t511);
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008147a6); };
            c.w64(7, t512);
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008147aa); };
            c.w64(8, t513);
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008147ae); };
            c.w64(9, t514);
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008147b2); };
            c.w64(12, t515);
            blk = 58; continue;
        }
        445 => {
            let Some(t516) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008147bb); };
            c.w64(3, t516);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008147cb, 0x70008147c6);
        }
        446 => {
            return RegfileExit::fault(0x70008147cb);
        }
        447 => {
            return RegfileExit::fault(0x70008147cc);
        }
        448 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t517) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008147cf); };
            c.w64(1, t517);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x70008147e0, 0x70008147db);
        }
        449 => {
            return RegfileExit::fault(0x70008147e0);
        }
        450 => {
            let r518 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r518);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008147e4); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008147e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008147e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008147ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008147f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008147f5); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008147f9); }
            let Some(t519) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008147fd); };
            c.w64(3, t519);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700081480e, 0x7000814809);
        }
        451 => {
            c.w64(15, c.r64(0));
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000814811); };
            c.w64(2, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000814815); };
            c.w64(1, t521);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000814819); };
            c.w64(11, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081481d); };
            c.w64(7, t523);
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000814821); };
            c.w64(8, t524);
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000814825); };
            c.w64(9, t525);
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000814829); };
            c.w64(12, t526);
            blk = 104; continue;
        }
        452 => {
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000814832); };
            c.w64(3, t527);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000814842, 0x700081483d);
        }
        453 => {
            return RegfileExit::fault(0x7000814842);
        }
        454 => {
            return RegfileExit::fault(0x7000814843);
        }
        455 => {
            return RegfileExit::fault(0x7000814844);
        }
        456 => {
            return RegfileExit::fault(0x7000814845);
        }
        457 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000814848); };
            c.w64(1, t528);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x7000814859, 0x7000814854);
        }
        458 => {
            return RegfileExit::fault(0x7000814859);
        }
        459 => {
            let r529 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r529);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081485d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000814861); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000814862); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000814866); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081486a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081486e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000814872); }
            let Some(t530) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000814876); };
            c.w64(3, t530);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000814887, 0x7000814882);
        }
        460 => {
            c.w64(15, c.r64(0));
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081488a); };
            c.w64(2, t531);
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081488e); };
            c.w64(1, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000814892); };
            c.w64(11, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000814896); };
            c.w64(7, t534);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081489a); };
            c.w64(8, t535);
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081489e); };
            c.w64(9, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008148a2); };
            c.w64(12, t537);
            blk = 196; continue;
        }
        461 => {
            let Some(t538) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008148ab); };
            c.w64(3, t538);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008148bb, 0x70008148b6);
        }
        462 => {
            return RegfileExit::fault(0x70008148bb);
        }
        463 => {
            return RegfileExit::fault(0x70008148bc);
        }
        464 => {
            return RegfileExit::fault(0x70008148bd);
        }
        465 => {
            return RegfileExit::fault(0x70008148be);
        }
        466 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008148bf); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x70008148c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008148c7); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008148cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008148cf); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x70008148db, 0x70008148d6);
        }
        467 => {
            c.w64(1, c.r64(0));
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008148de); };
            c.w64(2, t539);
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008148e2); };
            c.w64(11, t540);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008148e6); };
            c.w64(7, t541);
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008148ea); };
            c.w64(8, t542);
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008148ee); };
            c.w64(9, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008148f2); };
            c.w64(12, t544);
            blk = 288; continue;
        }
        468 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008148fb); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000814904); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000814908); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081490c); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x7000814910); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000814914); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000814919); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000814925, 0x7000814920);
        }
        469 => {
            c.w64(11, c.r64(0));
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000814928); };
            c.w64(2, t545);
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081492c); };
            c.w64(1, t546);
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000814930); };
            c.w64(8, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000814934); };
            c.w64(9, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000814938); };
            c.w64(6, t549);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700081493c); }
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000814941); };
            c.w64(7, t550);
            let Some(t551) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x7000814945); };
            c.w64(12, t551);
            blk = 297; continue;
        }
        470 => {
            let Some(t552) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000814951); };
            c.w64(3, t552);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000814961, 0x700081495c);
        }
        471 => {
            return RegfileExit::fault(0x7000814961);
        }
        472 => {
            return RegfileExit::fault(0x7000814962);
        }
        473 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000814963); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000814967); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081496b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081496f); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000814973); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700081497f, 0x700081497a);
        }
        474 => {
            c.w64(1, c.r64(0));
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000814982); };
            c.w64(2, t553);
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000814986); };
            c.w64(11, t554);
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081498a); };
            c.w64(7, t555);
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081498e); };
            c.w64(8, t556);
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000814992); };
            c.w64(9, t557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000814996); };
            c.w64(12, t558);
            blk = 335; continue;
        }
        475 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081499f); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008149a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008149ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008149b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x70008149b4); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x70008149b8); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x70008149bd); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x70008149c9, 0x70008149c4);
        }
        476 => {
            c.w64(11, c.r64(0));
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008149cc); };
            c.w64(2, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008149d0); };
            c.w64(1, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008149d4); };
            c.w64(8, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008149d8); };
            c.w64(9, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008149dc); };
            c.w64(6, t563);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x70008149e0); }
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008149e5); };
            c.w64(7, t564);
            let Some(t565) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x70008149e9); };
            c.w64(12, t565);
            blk = 344; continue;
        }
        477 => {
            let Some(t566) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008149f5); };
            c.w64(3, t566);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000814a05, 0x7000814a00);
        }
        478 => {
            return RegfileExit::fault(0x7000814a05);
        }
        479 => {
            return RegfileExit::fault(0x7000814a06);
        }
        480 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000814a07); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000814a0b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000814a0f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000814a13); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000814a17); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000814a23, 0x7000814a1e);
        }
        481 => {
            c.w64(1, c.r64(0));
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000814a26); };
            c.w64(2, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000814a2a); };
            c.w64(11, t568);
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000814a2e); };
            c.w64(7, t569);
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000814a32); };
            c.w64(8, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000814a36); };
            c.w64(9, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000814a3a); };
            c.w64(12, t572);
            blk = 382; continue;
        }
        482 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000814a43); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000814a4c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000814a50); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000814a54); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x7000814a58); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000814a5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000814a61); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x7000814a6d, 0x7000814a68);
        }
        483 => {
            c.w64(11, c.r64(0));
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000814a70); };
            c.w64(2, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000814a74); };
            c.w64(1, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000814a78); };
            c.w64(8, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000814a7c); };
            c.w64(9, t576);
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000814a80); };
            c.w64(6, t577);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000814a84); }
            let Some(t578) = c.ld64(0x70008131d0u64) else { return RegfileExit::fault(0x7000814a89); };
            c.w64(12, t578);
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000814a90); };
            c.w64(7, t579);
            blk = 391; continue;
        }
        484 => {
            let Some(t580) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000814a99); };
            c.w64(3, t580);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000814aa9, 0x7000814aa4);
        }
        485 => {
            return RegfileExit::fault(0x7000814aa9);
        }
        486 => {
            return RegfileExit::fault(0x7000814aaa);
        }
        487 => {
            return RegfileExit::fault(0x7000814aab);
        }
        488 => {
            c.w32(0, c.r32(0));
            let r581 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r581);
            c.w64(7, 0xfffffffffffc0000u64);
            let r582 = c.r64(7) & c.r64(0);
            c.w64(7, r582);
            c.logic(r582, 64);
            let Some(t583) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814abb); };
            c.logic(t583 & 0x2u64, 8);
            if c.cc_e() { blk = 437; continue; }
            blk = 489; continue;
        }
        489 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814ac9); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814aca); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000814ad7, 0x7000814ad2);
        }
        490 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814ad7); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814ad9); }
            blk = 437; continue;
        }
        491 => {
            c.w32(11, c.r32(11));
            let r584 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r584);
            c.w64(8, 0xfffffffffffc0000u64);
            let r585 = c.r64(8) & c.r64(11);
            c.w64(8, r585);
            c.logic(r585, 64);
            let Some(t586) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814aef); };
            c.logic(t586 & 0x2u64, 8);
            if c.cc_e() { blk = 422; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814afe); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814aff); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000814b0c, 0x7000814b07);
        }
        493 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814b0c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814b0e); }
            blk = 422; continue;
        }
        494 => {
            c.w32(0, c.r32(0));
            let r587 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r587);
            c.w64(8, 0xfffffffffffc0000u64);
            let r588 = c.r64(8) & c.r64(0);
            c.w64(8, r588);
            c.logic(r588, 64);
            let Some(t589) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814b23); };
            c.logic(t589 & 0x2u64, 8);
            if c.cc_e() { blk = 413; continue; }
            blk = 495; continue;
        }
        495 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814b32); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000814b3c, 0x7000814b37);
        }
        496 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814b3c); }
            blk = 413; continue;
        }
        497 => {
            c.w32(11, c.r32(11));
            let r590 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r590);
            c.w64(7, 0xfffffffffffc0000u64);
            let r591 = c.r64(7) & c.r64(11);
            c.w64(7, r591);
            c.logic(r591, 64);
            let Some(t592) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814b53); };
            c.logic(t592 & 0x2u64, 8);
            if c.cc_e() { blk = 398; continue; }
            blk = 498; continue;
        }
        498 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814b61); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x7000814b6a, 0x7000814b65);
        }
        499 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814b6a); }
            blk = 398; continue;
        }
        500 => {
            c.w32(1, c.r32(1));
            let r593 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r593);
            c.w64(7, 0xfffffffffffc0000u64);
            let r594 = c.r64(7) & c.r64(1);
            c.w64(7, r594);
            c.logic(r594, 64);
            let Some(t595) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814b7f); };
            c.logic(t595 & 0x2u64, 8);
            if c.cc_e() { blk = 396; continue; }
            blk = 501; continue;
        }
        501 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814b8d); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000814b96, 0x7000814b91);
        }
        502 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814b96); }
            blk = 396; continue;
        }
        503 => {
            c.w32(11, c.r32(11));
            let r596 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r596);
            c.w64(15, 0xfffffffffffc0000u64);
            let r597 = c.r64(15) & c.r64(11);
            c.w64(15, r597);
            c.logic(r597, 64);
            let Some(t598) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814bac); };
            c.logic(t598 & 0x2u64, 8);
            if c.cc_e() { blk = 393; continue; }
            blk = 504; continue;
        }
        504 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814bbb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814bbc); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000814bc9, 0x7000814bc4);
        }
        505 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814bc9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814bcb); }
            blk = 393; continue;
        }
        506 => {
            c.w32(0, c.r32(0));
            let r599 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r599);
            c.w64(8, 0xfffffffffffc0000u64);
            let r600 = c.r64(8) & c.r64(0);
            c.w64(8, r600);
            c.logic(r600, 64);
            let Some(t601) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814be0); };
            c.logic(t601 & 0x2u64, 8);
            if c.cc_e() { blk = 366; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814bef); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000814bf9, 0x7000814bf4);
        }
        508 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814bf9); }
            blk = 366; continue;
        }
        509 => {
            c.w32(11, c.r32(11));
            let r602 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r602);
            c.w64(7, 0xfffffffffffc0000u64);
            let r603 = c.r64(7) & c.r64(11);
            c.w64(7, r603);
            c.logic(r603, 64);
            let Some(t604) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814c10); };
            c.logic(t604 & 0x2u64, 8);
            if c.cc_e() { blk = 351; continue; }
            blk = 510; continue;
        }
        510 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814c1e); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x7000814c27, 0x7000814c22);
        }
        511 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814c27); }
            blk = 351; continue;
        }
        512 => {
            c.w32(1, c.r32(1));
            let r605 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r605);
            c.w64(7, 0xfffffffffffc0000u64);
            let r606 = c.r64(7) & c.r64(1);
            c.w64(7, r606);
            c.logic(r606, 64);
            let Some(t607) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814c3c); };
            c.logic(t607 & 0x2u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 513; continue;
        }
        513 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814c4a); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000814c53, 0x7000814c4e);
        }
        514 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814c53); }
            blk = 349; continue;
        }
        515 => {
            c.w32(11, c.r32(11));
            let r608 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r608);
            c.w64(15, 0xfffffffffffc0000u64);
            let r609 = c.r64(15) & c.r64(11);
            c.w64(15, r609);
            c.logic(r609, 64);
            let Some(t610) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814c69); };
            c.logic(t610 & 0x2u64, 8);
            if c.cc_e() { blk = 346; continue; }
            blk = 516; continue;
        }
        516 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814c78); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814c79); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000814c86, 0x7000814c81);
        }
        517 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814c86); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814c88); }
            blk = 346; continue;
        }
        518 => {
            c.w32(0, c.r32(0));
            let r611 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r611);
            c.w64(8, 0xfffffffffffc0000u64);
            let r612 = c.r64(8) & c.r64(0);
            c.w64(8, r612);
            c.logic(r612, 64);
            let Some(t613) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814c9d); };
            c.logic(t613 & 0x2u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 519; continue;
        }
        519 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814cac); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000814cb6, 0x7000814cb1);
        }
        520 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814cb6); }
            blk = 319; continue;
        }
        521 => {
            c.w32(11, c.r32(11));
            let r614 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r614);
            c.w64(7, 0xfffffffffffc0000u64);
            let r615 = c.r64(7) & c.r64(11);
            c.w64(7, r615);
            c.logic(r615, 64);
            let Some(t616) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814ccd); };
            c.logic(t616 & 0x2u64, 8);
            if c.cc_e() { blk = 304; continue; }
            blk = 522; continue;
        }
        522 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814cdb); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x7000814ce4, 0x7000814cdf);
        }
        523 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814ce4); }
            blk = 304; continue;
        }
        524 => {
            c.w32(1, c.r32(1));
            let r617 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r617);
            c.w64(7, 0xfffffffffffc0000u64);
            let r618 = c.r64(7) & c.r64(1);
            c.w64(7, r618);
            c.logic(r618, 64);
            let Some(t619) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814cf9); };
            c.logic(t619 & 0x2u64, 8);
            if c.cc_e() { blk = 302; continue; }
            blk = 525; continue;
        }
        525 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814d07); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000814d10, 0x7000814d0b);
        }
        526 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814d10); }
            blk = 302; continue;
        }
        527 => {
            c.w32(11, c.r32(11));
            let r620 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r620);
            c.w64(15, 0xfffffffffffc0000u64);
            let r621 = c.r64(15) & c.r64(11);
            c.w64(15, r621);
            c.logic(r621, 64);
            let Some(t622) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814d26); };
            c.logic(t622 & 0x2u64, 8);
            if c.cc_e() { blk = 299; continue; }
            blk = 528; continue;
        }
        528 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814d35); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814d36); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x7000814d43, 0x7000814d3e);
        }
        529 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814d43); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814d45); }
            blk = 299; continue;
        }
        530 => {
            c.w32(11, c.r32(11));
            let r623 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r623);
            c.w64(8, 0xfffffffffffc0000u64);
            let r624 = c.r64(8) & c.r64(11);
            c.w64(8, r624);
            c.logic(r624, 64);
            let Some(t625) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814d5b); };
            c.logic(t625 & 0x2u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 531; continue;
        }
        531 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814d6a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814d6b); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000814d78, 0x7000814d73);
        }
        532 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814d78); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814d7a); }
            blk = 256; continue;
        }
        533 => {
            c.w32(0, c.r32(0));
            let r626 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r626);
            c.w64(7, 0xfffffffffffc0000u64);
            let r627 = c.r64(7) & c.r64(0);
            c.w64(7, r627);
            c.logic(r627, 64);
            let Some(t628) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814d8f); };
            c.logic(t628 & 0x2u64, 8);
            if c.cc_e() { blk = 247; continue; }
            blk = 534; continue;
        }
        534 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814d9d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814d9e); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000814dab, 0x7000814da6);
        }
        535 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814dab); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814dad); }
            blk = 247; continue;
        }
        536 => {
            c.w32(11, c.r32(11));
            let r629 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r629);
            c.w64(8, 0xfffffffffffc0000u64);
            let r630 = c.r64(8) & c.r64(11);
            c.w64(8, r630);
            c.logic(r630, 64);
            let Some(t631) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814dc3); };
            c.logic(t631 & 0x2u64, 8);
            if c.cc_e() { blk = 232; continue; }
            blk = 537; continue;
        }
        537 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814dd2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814dd3); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000814de0, 0x7000814ddb);
        }
        538 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814de0); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814de2); }
            blk = 232; continue;
        }
        539 => {
            c.w32(11, c.r32(11));
            let r632 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r632);
            c.w64(7, 0xfffffffffffc0000u64);
            let r633 = c.r64(7) & c.r64(11);
            c.w64(7, r633);
            c.logic(r633, 64);
            let Some(t634) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814df8); };
            c.logic(t634 & 0x2u64, 8);
            if c.cc_e() { blk = 207; continue; }
            blk = 540; continue;
        }
        540 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814e06); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x7000814e0f, 0x7000814e0a);
        }
        541 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814e0f); }
            blk = 207; continue;
        }
        542 => {
            c.w32(15, c.r32(15));
            let r635 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r635);
            c.w64(7, 0xfffffffffffc0000u64);
            let r636 = c.r64(7) & c.r64(15);
            c.w64(7, r636);
            c.logic(r636, 64);
            let Some(t637) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814e25); };
            c.logic(t637 & 0x2u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 543; continue;
        }
        543 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814e33); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000814e3c, 0x7000814e37);
        }
        544 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814e3c); }
            blk = 205; continue;
        }
        545 => {
            c.w32(0, c.r32(0));
            let r638 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r638);
            c.w64(7, 0xfffffffffffc0000u64);
            let r639 = c.r64(7) & c.r64(0);
            c.w64(7, r639);
            c.logic(r639, 64);
            let Some(t640) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814e51); };
            c.logic(t640 & 0x2u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 546; continue;
        }
        546 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814e5f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814e60); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000814e6d, 0x7000814e68);
        }
        547 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814e6d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814e6f); }
            blk = 179; continue;
        }
        548 => {
            c.w32(11, c.r32(11));
            let r641 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r641);
            c.w64(8, 0xfffffffffffc0000u64);
            let r642 = c.r64(8) & c.r64(11);
            c.w64(8, r642);
            c.logic(r642, 64);
            let Some(t643) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814e85); };
            c.logic(t643 & 0x2u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 549; continue;
        }
        549 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814e94); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814e95); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000814ea2, 0x7000814e9d);
        }
        550 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814ea2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814ea4); }
            blk = 164; continue;
        }
        551 => {
            c.w32(0, c.r32(0));
            let r644 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r644);
            c.w64(7, 0xfffffffffffc0000u64);
            let r645 = c.r64(7) & c.r64(0);
            c.w64(7, r645);
            c.logic(r645, 64);
            let Some(t646) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814eb9); };
            c.logic(t646 & 0x2u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 552; continue;
        }
        552 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814ec7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814ec8); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000814ed5, 0x7000814ed0);
        }
        553 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814ed5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814ed7); }
            blk = 155; continue;
        }
        554 => {
            c.w32(11, c.r32(11));
            let r647 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r647);
            c.w64(8, 0xfffffffffffc0000u64);
            let r648 = c.r64(8) & c.r64(11);
            c.w64(8, r648);
            c.logic(r648, 64);
            let Some(t649) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814eed); };
            c.logic(t649 & 0x2u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 555; continue;
        }
        555 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814efc); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814efd); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000814f0a, 0x7000814f05);
        }
        556 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814f0a); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814f0c); }
            blk = 140; continue;
        }
        557 => {
            c.w32(11, c.r32(11));
            let r650 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r650);
            c.w64(7, 0xfffffffffffc0000u64);
            let r651 = c.r64(7) & c.r64(11);
            c.w64(7, r651);
            c.logic(r651, 64);
            let Some(t652) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814f22); };
            c.logic(t652 & 0x2u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 558; continue;
        }
        558 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814f30); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x7000814f39, 0x7000814f34);
        }
        559 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814f39); }
            blk = 115; continue;
        }
        560 => {
            c.w32(15, c.r32(15));
            let r653 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r653);
            c.w64(7, 0xfffffffffffc0000u64);
            let r654 = c.r64(7) & c.r64(15);
            c.w64(7, r654);
            c.logic(r654, 64);
            let Some(t655) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814f4f); };
            c.logic(t655 & 0x2u64, 8);
            if c.cc_e() { blk = 113; continue; }
            blk = 561; continue;
        }
        561 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814f5d); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000814f66, 0x7000814f61);
        }
        562 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814f66); }
            blk = 113; continue;
        }
        563 => {
            c.w32(11, c.r32(11));
            let r656 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r656);
            c.w64(7, 0xfffffffffffc0000u64);
            let r657 = c.r64(7) & c.r64(11);
            c.w64(7, r657);
            c.logic(r657, 64);
            let Some(t658) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814f7c); };
            c.logic(t658 & 0x2u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 564; continue;
        }
        564 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814f8a); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x7000814f93, 0x7000814f8e);
        }
        565 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814f93); }
            blk = 69; continue;
        }
        566 => {
            c.w32(15, c.r32(15));
            let r659 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r659);
            c.w64(7, 0xfffffffffffc0000u64);
            let r660 = c.r64(7) & c.r64(15);
            c.w64(7, r660);
            c.logic(r660, 64);
            let Some(t661) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814fa9); };
            c.logic(t661 & 0x2u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 567; continue;
        }
        567 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814fb7); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000814fc0, 0x7000814fbb);
        }
        568 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814fc0); }
            blk = 67; continue;
        }
        569 => {
            c.w32(11, c.r32(11));
            let r662 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r662);
            c.w64(8, 0xfffffffffffc0000u64);
            let r663 = c.r64(8) & c.r64(11);
            c.w64(8, r663);
            c.logic(r663, 64);
            let Some(t664) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000814fd6); };
            c.logic(t664 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 570; continue;
        }
        570 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000814fe5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000814fe6); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000814ff3, 0x7000814fee);
        }
        571 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000814ff3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000814ff5); }
            blk = 27; continue;
        }
        572 => {
            return RegfileExit::jump(0x7000814ffc);
        }
            _ => unreachable!(),
        }
    }
}
