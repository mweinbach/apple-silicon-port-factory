#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_7000815014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000815014); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000815018); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081501e); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000815025); };
            c.w32(9, t3);
            let Some(t4) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x700081502a); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x7000815046); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700081504a); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700081504e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000815052); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000815056); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x700081505a); }
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
            if c.cc_e() { blk = 593; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(12), 0x405u64, 32);
            if c.cc_e() { blk = 462; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(12), 0x503u64, 32);
            if c.cc_e() { blk = 677; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 820; continue;
        }
        6 => {
            c.cmp(c.r32(12), 0x105u64, 32);
            if c.cc_e() { blk = 334; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x203u64, 32);
            if c.cc_e() { blk = 761; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(12), 0x402u64, 32);
            if c.cc_e() { blk = 510; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 820; continue;
        }
        10 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_l() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 148; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x103u64, 32);
            if c.cc_e() { blk = 405; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 820; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 278; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 820; continue;
        }
        19 => {
            c.w64(8, c.r64(2));
            let r11 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r11);
            c.logic(r11, 64);
            let Some(t12) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081511d); };
            c.w64(9, t12);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 20; continue;
        }
        20 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 27; continue; }
            blk = 21; continue;
        }
        21 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815131); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000815138); };
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
            blk = 27; continue;
        }
        24 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000815161); };
            c.w32(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000815165); };
            c.cmp(t18, c.r32(8), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        27 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000815176); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r19 = c.r64(8) & c.r64(2);
            c.w64(8, r19);
            c.logic(r19, 64);
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815184); };
            c.logic(t20 & 0x4u64, 8);
            if c.cc_ne() { blk = 1298; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081518f); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 36; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081519b); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            let Some(t25) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008151a2); };
            c.cmp(t25, 0x42u64, 16);
            if c.cc_e() { blk = 31; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        31 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008151b2); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            if c.cc_e() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        34 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r26 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r26);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        36 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        39 => {
            let Some(t27) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081520b); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815212); };
            c.cmp(t29, 0x834u64, 16);
            if c.cc_e() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081521c); };
            c.w32(12, t30);
            blk = 42; continue;
        }
        41 => {
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000815222); };
            c.w32(12, t31);
            blk = 42; continue;
        }
        42 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_be() { blk = 45; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_b() { blk = 61; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        45 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815245); };
            c.w32(15, t32);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(9), c.r64(3), 64);
            if c.cc_b() { blk = 59; continue; }
            blk = 46; continue;
        }
        46 => {
            let r33 = c.add(c.r64(3), 0x400u64, 64);
            c.w64(3, r33);
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_be() { blk = 825; continue; }
            blk = 47; continue;
        }
        47 => {
            c.w64(3, c.r64(9).wrapping_add(0x1));
            c.w64(7, c.r64(3));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 825; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 823; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t34) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815294); };
            c.w64(3, t34);
            let Some(t35) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081529b); };
            c.w64(6, t35);
            c.w64(1, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(11, c.r64(1).wrapping_add(c.r64(6)));
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008152aa); };
            c.w64(0, t36);
            let Some(t37) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008152b1); };
            c.cmp(t37, c.r64(11), 64);
            if c.cc_be() { blk = 821; continue; }
            blk = 50; continue;
        }
        50 => {
            if !c.st64(c.r64(3), c.r64(11)) { return RegfileExit::fault(0x70008152ba); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 51; continue;
        }
        51 => {
            c.w32(11, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t38) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008152c5); };
            c.w64(0, t38);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008152cc); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008152cf); }
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_e() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008152e8); };
            c.w64(15, t39);
            blk = 53; continue;
        }
        53 => {
            let r40 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r40);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(15)) { return RegfileExit::fault(0x70008152f3); }
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_ne() { blk = 53; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 56; continue; }
            blk = 55; continue;
        }
        55 => {
            let r41 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r41);
            let Some(t42) = c.ld32(c.r64(11).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000815306); };
            c.w32(7, t42);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x700081530a); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 55; continue; }
            blk = 56; continue;
        }
        56 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000815314); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r43 = c.r64(7) & c.r64(2);
            c.w64(7, r43);
            c.logic(r43, 64);
            let Some(t44) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815326); };
            c.logic(t44 & 0x4u64, 8);
            if c.cc_ne() { blk = 1295; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(8, c.r64(1));
            blk = 59; continue;
        }
        59 => {
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815333); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815339); };
            c.cmp(t47, 0x834u64, 16);
            if c.cc_ne() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            c.w64(1, c.r64(9).wrapping_add(0x1));
            let r48 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r48);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000815347); }
            blk = 61; continue;
        }
        61 => {
            let Some(t49) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081534a); };
            c.w32(1, t49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081534e); };
            c.cmp(t50, c.r32(1), 32);
            if c.cc_ne() { blk = 74; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081535b); };
            c.w32(1, t51);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 76; continue; }
            blk = 63; continue;
        }
        63 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 845; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815382); };
            c.w64(11, t52);
            let Some(t53) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000815389); };
            c.w64(15, t53);
            c.w64(0, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(0).wrapping_add(c.r64(15)));
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000815398); };
            c.w64(6, t54);
            let Some(t55) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700081539f); };
            c.cmp(t55, c.r64(3), 64);
            if c.cc_be() { blk = 842; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008153a8); }
            c.w64(11, c.r64(15).wrapping_add(0x1));
            blk = 66; continue;
        }
        66 => {
            let r56 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r56);
            let Some(t57) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008153b1); };
            c.w64(15, t57);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008153b8); }
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008153bc); }
            c.w64(7, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 69; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t58) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008153d5); };
            c.w64(12, t58);
            blk = 68; continue;
        }
        68 => {
            let r59 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r59);
            if !c.st32(c.r64(1).wrapping_add(c.r64(11)), c.r32(12)) { return RegfileExit::fault(0x70008153e0); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 70; continue;
        }
        70 => {
            let r60 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r60);
            let Some(t61) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008153f3); };
            c.w32(1, t61);
            if !c.st32(c.r64(7).wrapping_add(c.r64(11)), c.r32(1)) { return RegfileExit::fault(0x70008153f7); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 70; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(11)) { return RegfileExit::fault(0x7000815401); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r62 = c.r64(1) & c.r64(2);
            c.w64(1, r62);
            c.logic(r62, 64);
            let Some(t63) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815415); };
            c.logic(t63 & 0x4u64, 8);
            if c.cc_ne() { blk = 1292; continue; }
            blk = 73; continue;
        }
        73 => {
            c.w64(8, c.r64(11));
            blk = 74; continue;
        }
        74 => {
            let Some(t64) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815422); };
            c.w32(1, t64);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 844; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000815435); };
            c.w64(0, t65);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x7000815439); }
            blk = 817; continue;
        }
        76 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000815443); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000815447); };
            c.w64(0, t67);
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081544b); };
            c.w64(9, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081544f); };
            c.w64(6, t69);
            blk = 819; continue;
        }
        77 => {
            c.w64(9, c.r64(2));
            let r70 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r70);
            c.logic(r70, 64);
            let Some(t71) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815462); };
            c.w64(12, t71);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 85; continue; }
            blk = 78; continue;
        }
        78 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 85; continue; }
            blk = 79; continue;
        }
        79 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815476); };
            c.w64(15, t72);
            let Some(t73) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081547d); };
            c.w64(15, t73);
            c.w64(3, c.r64(12));
            let r74 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r74);
            c.logic(r74, 64);
            c.w64(7, c.r64(15));
            let r75 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r75);
            c.logic(r75, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 82; continue; }
            blk = 80; continue;
        }
        80 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            blk = 85; continue;
        }
        82 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 85; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t76) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008154a5); };
            c.w32(7, t76);
            let Some(t77) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x70008154a8); };
            c.cmp(t77, c.r32(7), 32);
            if c.cc_ne() { blk = 85; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        85 => {
            let Some(t78) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008154b9); };
            c.w32(7, t78);
            let r79 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008154bf); };
            c.cmp(t80, c.r32(7), 32);
            if c.cc_e() { blk = 105; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t81) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008154cc); };
            c.w32(9, t81);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t82) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008154d9); };
            c.cmp(t82, 0x834u64, 16);
            if c.cc_e() { blk = 88; continue; }
            blk = 87; continue;
        }
        87 => {
            c.w64(8, c.r64(12));
            blk = 89; continue;
        }
        88 => {
            let Some(t83) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008154e7); };
            c.w32(8, t83);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 104; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 851; continue; }
            blk = 91; continue;
        }
        91 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 847; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t84) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815520); };
            c.w64(3, t84);
            let Some(t85) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000815527); };
            c.w64(0, t85);
            c.w64(2, c.r64(15).wrapping_add(c.r64(0)));
            let Some(t86) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081552e); };
            c.cmp(t86, c.r64(2), 64);
            if c.cc_be() { blk = 849; continue; }
            blk = 93; continue;
        }
        93 => {
            if !c.st64(c.r64(3), c.r64(2)) { return RegfileExit::fault(0x7000815538); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 94; continue;
        }
        94 => {
            let r87 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r87);
            let Some(t88) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000815542); };
            c.w64(0, t88);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000815549); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081554d); }
            let Some(t89) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000815551); };
            c.w64(12, t89);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815558); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            let r90 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r90);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x7000815578); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 95; continue; }
            blk = 96; continue;
        }
        96 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 101; continue; }
            blk = 97; continue;
        }
        97 => {
            let r91 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r91);
            let Some(t92) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x7000815595); };
            c.w32(9, t92);
            let r93 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r93);
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081559d); };
            c.cmp(t94, c.r32(9), 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let r95 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r95);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x70008155b2); }
            blk = 100; continue;
        }
        99 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x70008155c4); }
            blk = 100; continue;
        }
        100 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 97; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t96) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008155ce); };
            c.w64(7, t96);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x70008155d2); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r97 = c.r64(8) & c.r64(7);
            c.w64(8, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008155e6); };
            c.logic(t98 & 0x4u64, 8);
            if c.cc_ne() { blk = 1289; continue; }
            blk = 103; continue;
        }
        103 => {
            c.w64(2, c.r64(7));
            let Some(t99) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008155f4); };
            c.w64(0, t99);
            blk = 105; continue;
        }
        104 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        105 => {
            c.w64(7, c.r64(11));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x7000815605); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r100 = c.r64(8) & c.r64(2);
            c.w64(8, r100);
            c.logic(r100, 64);
            let Some(t101) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815612); };
            c.logic(t101 & 0x4u64, 8);
            if c.cc_ne() { blk = 1286; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t102) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081561d); };
            c.w32(8, t102);
            let r103 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r103);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t104) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815629); };
            c.w32(9, t104);
            let r105 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r105);
            let Some(t106) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815630); };
            c.cmp(t106, 0x42u64, 16);
            if c.cc_e() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        109 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815640); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 111; continue; }
            blk = 110; continue;
        }
        110 => {
            if c.cc_e() { blk = 112; continue; }
            blk = 111; continue;
        }
        111 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        112 => {
            let Some(t107) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000815663); };
            c.w64(11, t107);
            let r108 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r108);
            let Some(t109) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x700081566d); };
            c.w64(12, t109);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        114 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 115; continue;
        }
        115 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 119; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t110) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081568b); };
            c.w32(11, t110);
            let Some(t111) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081568f); };
            c.cmp(t111, c.r32(11), 32);
            if c.cc_ne() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815698); }
            blk = 120; continue;
        }
        118 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        119 => {
            c.w64(11, c.r64(0));
            let r112 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r112);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 120; continue;
        }
        120 => {
            let Some(t113) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008156b6); };
            c.w32(11, t113);
            let r114 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r114);
            let Some(t115) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008156bd); };
            c.cmp(t115, 0x834u64, 16);
            if c.cc_e() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008156c6); };
            c.w32(11, t116);
            blk = 123; continue;
        }
        122 => {
            let Some(t117) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008156cc); };
            c.w32(11, t117);
            blk = 123; continue;
        }
        123 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_e() { blk = 126; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 146; continue; }
            blk = 125; continue;
        }
        125 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        126 => {
            let Some(t118) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008156ec); };
            c.w32(11, t118);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_b() { blk = 144; continue; }
            blk = 127; continue;
        }
        127 => {
            let r119 = c.add(c.r64(12), 0x400u64, 64);
            c.w64(12, r119);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 859; continue; }
            blk = 128; continue;
        }
        128 => {
            c.w64(12, c.r64(9).wrapping_add(0x1));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(12, c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x10));
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 859; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 857; continue; }
            blk = 130; continue;
        }
        130 => {
            c.w64(1, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 853; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t120) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815750); };
            c.w64(7, t120);
            let Some(t121) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000815757); };
            c.w64(15, t121);
            c.w64(3, c.r64(1).wrapping_add(c.r64(15)));
            let Some(t122) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081575e); };
            c.cmp(t122, c.r64(3), 64);
            if c.cc_be() { blk = 855; continue; }
            blk = 132; continue;
        }
        132 => {
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x7000815768); }
            c.w64(1, c.r64(15).wrapping_add(0x1));
            blk = 133; continue;
        }
        133 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000815773); };
            c.w64(15, t123);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081577a); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700081577e); }
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000815781); };
            c.w64(7, t124);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815788); }
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_e() { blk = 135; continue; }
            blk = 134; continue;
        }
        134 => {
            let r125 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r125);
            if !c.movsd_mr(c.r64(11).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x70008157a6); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ne() { blk = 134; continue; }
            blk = 135; continue;
        }
        135 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 141; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t126) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x70008157b7); };
            c.w64(11, t126);
            blk = 137; continue;
        }
        137 => {
            let r127 = c.sub(c.r64(7), 0x8u64, 64);
            c.w64(7, r127);
            let Some(t128) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008157c2); };
            c.cmp(t128, c.r64(11), 64);
            if c.cc_e() { blk = 139; continue; }
            blk = 138; continue;
        }
        138 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(c.r64(8))) { return RegfileExit::fault(0x70008157c8); }
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x70008157ce); }
            blk = 140; continue;
        }
        139 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(1)), c.r64(11)) { return RegfileExit::fault(0x70008157d5); }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 137; continue; }
            blk = 141; continue;
        }
        141 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008157df); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r129 = c.r64(7) & c.r64(2);
            c.w64(7, r129);
            c.logic(r129, 64);
            let Some(t130) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008157f1); };
            c.logic(t130 & 0x4u64, 8);
            if c.cc_ne() { blk = 1283; continue; }
            blk = 143; continue;
        }
        143 => {
            c.w64(8, c.r64(1));
            blk = 144; continue;
        }
        144 => {
            let Some(t131) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008157fe); };
            c.w32(1, t131);
            let r132 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r132);
            let Some(t133) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815804); };
            c.cmp(t133, 0x834u64, 16);
            if c.cc_ne() { blk = 146; continue; }
            blk = 145; continue;
        }
        145 => {
            c.w64(1, c.r64(9).wrapping_add(0x1));
            let r134 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r134);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000815812); }
            blk = 146; continue;
        }
        146 => {
            let Some(t135) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815815); };
            c.w32(1, t135);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 876; continue; }
            blk = 147; continue;
        }
        147 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000815832); }
            blk = 817; continue;
        }
        148 => {
            c.w64(9, c.r64(2));
            let r136 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r136);
            c.logic(r136, 64);
            let Some(t137) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815848); };
            c.w64(12, t137);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 156; continue; }
            blk = 149; continue;
        }
        149 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 156; continue; }
            blk = 150; continue;
        }
        150 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t138) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081585c); };
            c.w64(15, t138);
            let Some(t139) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000815863); };
            c.w64(15, t139);
            c.w64(3, c.r64(12));
            let r140 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r140);
            c.logic(r140, 64);
            c.w64(7, c.r64(15));
            let r141 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r141);
            c.logic(r141, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 153; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 154; continue; }
            blk = 152; continue;
        }
        152 => {
            blk = 156; continue;
        }
        153 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 156; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t142) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081588b); };
            c.w32(7, t142);
            let Some(t143) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081588e); };
            c.cmp(t143, c.r32(7), 32);
            if c.cc_ne() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        156 => {
            let Some(t144) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081589f); };
            c.w32(7, t144);
            let r145 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r145);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008158a5); }
            let Some(t146) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008158a9); };
            c.cmp(t146, c.r32(7), 32);
            if c.cc_e() { blk = 177; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t147) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008158b6); };
            c.w32(9, t147);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t148) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008158c3); };
            c.cmp(t148, 0x834u64, 16);
            if c.cc_e() { blk = 159; continue; }
            blk = 158; continue;
        }
        158 => {
            c.w64(8, c.r64(12));
            blk = 160; continue;
        }
        159 => {
            let Some(t149) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008158d1); };
            c.w32(8, t149);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 176; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 881; continue; }
            blk = 162; continue;
        }
        162 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 877; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t150) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081590a); };
            c.w64(3, t150);
            let Some(t151) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000815911); };
            c.w64(2, t151);
            c.w64(7, c.r64(15).wrapping_add(c.r64(2)));
            let Some(t152) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815918); };
            c.cmp(t152, c.r64(7), 64);
            if c.cc_be() { blk = 879; continue; }
            blk = 164; continue;
        }
        164 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x7000815922); }
            c.w64(7, c.r64(2).wrapping_add(0x1));
            blk = 165; continue;
        }
        165 => {
            let r153 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r153);
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700081592c); };
            c.w64(15, t154);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000815933); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000815937); }
            let Some(t155) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081593b); };
            c.w64(12, t155);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815942); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 167; continue; }
            blk = 166; continue;
        }
        166 => {
            let r156 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r156);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(7)), 0) { return RegfileExit::fault(0x7000815962); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 166; continue; }
            blk = 167; continue;
        }
        167 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 168; continue;
        }
        168 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081597b); };
            c.w64(9, t157);
            blk = 169; continue;
        }
        169 => {
            let r158 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r158);
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000815983); };
            c.w32(15, t159);
            let r160 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r160);
            let Some(t161) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081598b); };
            c.cmp(t161, c.r32(15), 32);
            if c.cc_e() { blk = 171; continue; }
            blk = 170; continue;
        }
        170 => {
            let r162 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r162);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(15) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(7)), 0) { return RegfileExit::fault(0x70008159a0); }
            blk = 172; continue;
        }
        171 => {
            let Some(t163) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x70008159a8); };
            c.w64(15, t163);
            if !c.st64(c.r64(12).wrapping_add(c.r64(7)), c.r64(15)) { return RegfileExit::fault(0x70008159af); }
            blk = 172; continue;
        }
        172 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 169; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008159b9); };
            c.w64(9, t164);
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x70008159bd); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r165 = c.r64(8) & c.r64(9);
            c.w64(8, r165);
            c.logic(r165, 64);
            let Some(t166) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008159d1); };
            c.logic(t166 & 0x4u64, 8);
            if c.cc_ne() { blk = 1280; continue; }
            blk = 175; continue;
        }
        175 => {
            blk = 178; continue;
        }
        176 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        177 => {
            c.w64(9, c.r64(2));
            blk = 178; continue;
        }
        178 => {
            c.w64(7, c.r64(11));
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008159ec); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r167 = c.r64(8) & c.r64(9);
            c.w64(8, r167);
            c.logic(r167, 64);
            let Some(t168) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008159fa); };
            c.logic(t168 & 0x4u64, 8);
            if c.cc_ne() { blk = 1277; continue; }
            blk = 179; continue;
        }
        179 => {
            let Some(t169) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815a05); };
            c.w32(8, t169);
            let r170 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r170);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 187; continue; }
            blk = 180; continue;
        }
        180 => {
            let Some(t171) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815a11); };
            c.w32(11, t171);
            let r172 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r172);
            let Some(t173) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815a18); };
            c.cmp(t173, 0x42u64, 16);
            if c.cc_e() { blk = 182; continue; }
            blk = 181; continue;
        }
        181 => {
            c.w64(2, c.r64(9));
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        182 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815a2b); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 184; continue; }
            blk = 183; continue;
        }
        183 => {
            if c.cc_e() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            c.w64(2, c.r64(9));
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        185 => {
            let Some(t174) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000815a51); };
            c.w64(12, t174);
            let r175 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r175);
            let Some(t176) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x7000815a5b); };
            c.w64(15, t176);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 188; continue; }
            blk = 186; continue;
        }
        186 => {
            c.w64(2, c.r64(9));
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        187 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 188; continue;
        }
        188 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 192; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t177) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815a7c); };
            c.w32(12, t177);
            let Some(t178) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000815a80); };
            c.cmp(t178, c.r32(12), 32);
            if c.cc_ne() { blk = 191; continue; }
            blk = 190; continue;
        }
        190 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815a89); }
            blk = 193; continue;
        }
        191 => {
            c.w64(2, c.r64(9));
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        192 => {
            c.w64(12, c.r64(0));
            let r179 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r179);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 193; continue;
        }
        193 => {
            let Some(t180) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815aaa); };
            c.w32(12, t180);
            let r181 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r181);
            let Some(t182) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815ab1); };
            c.cmp(t182, 0x834u64, 16);
            if c.cc_e() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t183) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815abb); };
            c.w32(12, t183);
            blk = 196; continue;
        }
        195 => {
            let Some(t184) = c.ld32(c.r64(9).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000815ac1); };
            c.w32(12, t184);
            blk = 196; continue;
        }
        196 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 199; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 219; continue; }
            blk = 198; continue;
        }
        198 => {
            c.w64(2, c.r64(9));
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        199 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815ae4); };
            c.w32(12, t185);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 217; continue; }
            blk = 200; continue;
        }
        200 => {
            let r186 = c.add(c.r64(15), 0x400u64, 64);
            c.w64(15, r186);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 889; continue; }
            blk = 201; continue;
        }
        201 => {
            c.w64(15, c.r64(11).wrapping_add(0x1));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(15, c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(15), 0x3fffu64, 64);
            if c.cc_ae() { blk = 889; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r64(15), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 887; continue; }
            blk = 203; continue;
        }
        203 => {
            c.w64(1, c.r64(15).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 883; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815b48); };
            c.w64(7, t187);
            let Some(t188) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000815b4f); };
            c.w64(3, t188);
            c.w64(2, c.r64(1).wrapping_add(c.r64(3)));
            let Some(t189) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815b56); };
            c.cmp(t189, c.r64(2), 64);
            if c.cc_be() { blk = 885; continue; }
            blk = 205; continue;
        }
        205 => {
            if !c.st64(c.r64(7), c.r64(2)) { return RegfileExit::fault(0x7000815b60); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 206; continue;
        }
        206 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000815b6b); };
            c.w64(3, t190);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000815b72); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000815b75); }
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000815b78); };
            c.w64(7, t191);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815b7f); }
            c.w64(7, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(12, c.r64(15).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_e() { blk = 208; continue; }
            blk = 207; continue;
        }
        207 => {
            let r192 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r192);
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x7000815b9d); }
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_ne() { blk = 207; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 214; continue; }
            blk = 209; continue;
        }
        209 => {
            let Some(t193) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x7000815bae); };
            c.w64(12, t193);
            blk = 210; continue;
        }
        210 => {
            let r194 = c.sub(c.r64(7), 0x8u64, 64);
            c.w64(7, r194);
            let Some(t195) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000815bb9); };
            c.cmp(t195, c.r64(12), 64);
            if c.cc_e() { blk = 212; continue; }
            blk = 211; continue;
        }
        211 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(c.r64(8))) { return RegfileExit::fault(0x7000815bbf); }
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x7000815bc5); }
            blk = 213; continue;
        }
        212 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(1)), c.r64(12)) { return RegfileExit::fault(0x7000815bcc); }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 210; continue; }
            blk = 214; continue;
        }
        214 => {
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000815bd6); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 216; continue; }
            blk = 215; continue;
        }
        215 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r196 = c.r64(7) & c.r64(9);
            c.w64(7, r196);
            c.logic(r196, 64);
            let Some(t197) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815be9); };
            c.logic(t197 & 0x4u64, 8);
            if c.cc_ne() { blk = 1274; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(8, c.r64(1));
            blk = 217; continue;
        }
        217 => {
            let Some(t198) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815bf6); };
            c.w32(1, t198);
            let r199 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r199);
            let Some(t200) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815bfd); };
            c.cmp(t200, 0x834u64, 16);
            if c.cc_ne() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r201 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r201);
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000815c0b); }
            blk = 219; continue;
        }
        219 => {
            let Some(t202) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815c0f); };
            c.w32(1, t202);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 906; continue; }
            blk = 220; continue;
        }
        220 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000815c2c); }
            blk = 817; continue;
        }
        221 => {
            c.w64(8, c.r64(2));
            let r203 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r203);
            c.logic(r203, 64);
            let Some(t204) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815c42); };
            c.w64(9, t204);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 229; continue; }
            blk = 222; continue;
        }
        222 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 223; continue;
        }
        223 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815c56); };
            c.w64(12, t205);
            let Some(t206) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000815c5d); };
            c.w64(12, t206);
            c.w64(15, c.r64(9));
            let r207 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r207);
            c.logic(r207, 64);
            c.w64(3, c.r64(12));
            let r208 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r208);
            c.logic(r208, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 226; continue; }
            blk = 224; continue;
        }
        224 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 227; continue; }
            blk = 225; continue;
        }
        225 => {
            blk = 229; continue;
        }
        226 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            let Some(t209) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000815c86); };
            c.w32(8, t209);
            let Some(t210) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000815c8a); };
            c.cmp(t210, c.r32(8), 32);
            if c.cc_ne() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        229 => {
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000815c9e); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r211 = c.r64(8) & c.r64(2);
            c.w64(8, r211);
            c.logic(r211, 64);
            let Some(t212) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815cac); };
            c.logic(t212 & 0x4u64, 8);
            if c.cc_ne() { blk = 1271; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t213) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815cb7); };
            c.w32(8, t213);
            let r214 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r214);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 236; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t215) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815cc3); };
            c.w32(11, t215);
            let r216 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r216);
            let Some(t217) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815cca); };
            c.cmp(t217, 0x42u64, 16);
            if c.cc_ne() { blk = 819; continue; }
            blk = 232; continue;
        }
        232 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000815cd6); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 819; continue; }
            blk = 233; continue;
        }
        233 => {
            if c.cc_ne() { blk = 819; continue; }
            blk = 234; continue;
        }
        234 => {
            let Some(t218) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000815cf9); };
            c.w64(12, t218);
            let r219 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r219);
            let Some(t220) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x7000815d03); };
            c.w64(15, t220);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 237; continue; }
            blk = 235; continue;
        }
        235 => {
            blk = 819; continue;
        }
        236 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 237; continue;
        }
        237 => {
            let Some(t221) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815d1a); };
            c.w32(12, t221);
            let r222 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r222);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000815d21); }
            let Some(t223) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815d25); };
            c.cmp(t223, 0x834u64, 16);
            if c.cc_e() { blk = 239; continue; }
            blk = 238; continue;
        }
        238 => {
            let Some(t224) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815d2f); };
            c.w32(12, t224);
            blk = 240; continue;
        }
        239 => {
            let Some(t225) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000815d35); };
            c.w32(12, t225);
            blk = 240; continue;
        }
        240 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000815d42); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_e() { blk = 243; continue; }
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 260; continue; }
            blk = 242; continue;
        }
        242 => {
            blk = 819; continue;
        }
        243 => {
            let Some(t226) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815d59); };
            c.w32(15, t226);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_b() { blk = 257; continue; }
            blk = 244; continue;
        }
        244 => {
            let r227 = c.add(c.r64(3), 0x400u64, 64);
            c.w64(3, r227);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_be() { blk = 911; continue; }
            blk = 245; continue;
        }
        245 => {
            c.w64(3, c.r64(11).wrapping_add(0x1));
            c.w64(7, c.r64(3));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 911; continue; }
            blk = 246; continue;
        }
        246 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 909; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t228) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815da8); };
            c.w64(3, t228);
            let Some(t229) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000815daf); };
            c.w64(1, t229);
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(12, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t230) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000815dbe); };
            c.w64(11, t230);
            let Some(t231) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000815dc5); };
            c.cmp(t231, c.r64(12), 64);
            if c.cc_be() { blk = 907; continue; }
            blk = 248; continue;
        }
        248 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x7000815dce); }
            let r232 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r232);
            blk = 249; continue;
        }
        249 => {
            c.w32(9, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t233) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000815dd9); };
            c.w64(11, t233);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000815de0); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000815de4); }
            c.w64(9, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_e() { blk = 252; continue; }
            blk = 250; continue;
        }
        250 => {
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000815dfd); };
            c.w64(11, t234);
            blk = 251; continue;
        }
        251 => {
            let r235 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r235);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x7000815e08); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 251; continue; }
            blk = 252; continue;
        }
        252 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 254; continue; }
            blk = 253; continue;
        }
        253 => {
            let r236 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r236);
            let Some(t237) = c.ld32(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000815e1b); };
            c.w32(7, t237);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x7000815e1f); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 253; continue; }
            blk = 254; continue;
        }
        254 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000815e29); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r238 = c.r64(7) & c.r64(2);
            c.w64(7, r238);
            c.logic(r238, 64);
            let Some(t239) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815e3b); };
            c.logic(t239 & 0x4u64, 8);
            if c.cc_ne() { blk = 1268; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w64(8, c.r64(1));
            blk = 257; continue;
        }
        257 => {
            let Some(t240) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815e48); };
            c.w32(1, t240);
            let r241 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r241);
            let Some(t242) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000815e4e); };
            c.cmp(t242, 0x834u64, 16);
            if c.cc_e() { blk = 259; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000815e56); };
            c.w64(11, t243);
            blk = 260; continue;
        }
        259 => {
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000815e5c); };
            c.w64(11, t244);
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r245 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r245);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000815e66); }
            blk = 260; continue;
        }
        260 => {
            let Some(t246) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000815e69); };
            c.w32(1, t246);
            let Some(t247) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000815e6d); };
            c.cmp(t247, c.r32(1), 32);
            if c.cc_ne() { blk = 273; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t248) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815e7a); };
            c.w32(1, t248);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 277; continue; }
            blk = 262; continue;
        }
        262 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 930; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t249) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815ea1); };
            c.w64(9, t249);
            let Some(t250) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000815ea8); };
            c.w64(12, t250);
            c.w64(15, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000815eb7); };
            c.w64(6, t251);
            let Some(t252) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000815ebe); };
            c.cmp(t252, c.r64(3), 64);
            if c.cc_be() { blk = 927; continue; }
            blk = 264; continue;
        }
        264 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x7000815ec7); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 265; continue;
        }
        265 => {
            let r253 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r253);
            let Some(t254) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000815ed1); };
            c.w64(12, t254);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000815ed8); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000815edc); }
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000815ee0); };
            c.w64(7, t255);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 268; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t256) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000815ef9); };
            c.w64(12, t256);
            blk = 267; continue;
        }
        267 => {
            let r257 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r257);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x7000815f04); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 267; continue; }
            blk = 268; continue;
        }
        268 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 270; continue; }
            blk = 269; continue;
        }
        269 => {
            let r258 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r258);
            let Some(t259) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000815f17); };
            c.w32(1, t259);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x7000815f1b); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 269; continue; }
            blk = 270; continue;
        }
        270 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000815f25); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 272; continue; }
            blk = 271; continue;
        }
        271 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r260 = c.r64(1) & c.r64(2);
            c.w64(1, r260);
            c.logic(r260, 64);
            let Some(t261) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815f39); };
            c.logic(t261 & 0x4u64, 8);
            if c.cc_ne() { blk = 1265; continue; }
            blk = 272; continue;
        }
        272 => {
            c.w64(8, c.r64(9));
            blk = 273; continue;
        }
        273 => {
            let Some(t262) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000815f46); };
            c.w32(1, t262);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 929; continue; }
            blk = 274; continue;
        }
        274 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000815f61); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 276; continue; }
            blk = 275; continue;
        }
        275 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r263 = c.r64(7) & c.r64(8);
            c.w64(7, r263);
            c.logic(r263, 64);
            let Some(t264) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815f73); };
            c.logic(t264 & 0x4u64, 8);
            if c.cc_ne() { blk = 1262; continue; }
            blk = 276; continue;
        }
        276 => {
            blk = 817; continue;
        }
        277 => {
            let Some(t265) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000815f82); };
            c.w64(1, t265);
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000815f86); };
            c.w64(9, t266);
            blk = 819; continue;
        }
        278 => {
            c.w64(8, c.r64(2));
            let r267 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r267);
            c.logic(r267, 64);
            let Some(t268) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000815f99); };
            c.w64(9, t268);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 286; continue; }
            blk = 279; continue;
        }
        279 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 286; continue; }
            blk = 280; continue;
        }
        280 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t269) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000815fad); };
            c.w64(12, t269);
            let Some(t270) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000815fb4); };
            c.w64(12, t270);
            c.w64(15, c.r64(9));
            let r271 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r271);
            c.logic(r271, 64);
            c.w64(3, c.r64(12));
            let r272 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r272);
            c.logic(r272, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 283; continue; }
            blk = 281; continue;
        }
        281 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 284; continue; }
            blk = 282; continue;
        }
        282 => {
            blk = 286; continue;
        }
        283 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 286; continue; }
            blk = 284; continue;
        }
        284 => {
            let Some(t273) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000815fdd); };
            c.w32(8, t273);
            let Some(t274) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000815fe1); };
            c.cmp(t274, c.r32(8), 32);
            if c.cc_ne() { blk = 286; continue; }
            blk = 285; continue;
        }
        285 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        286 => {
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000815ff5); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r275 = c.r64(8) & c.r64(2);
            c.w64(8, r275);
            c.logic(r275, 64);
            let Some(t276) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816003); };
            c.logic(t276 & 0x4u64, 8);
            if c.cc_ne() { blk = 1259; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t277) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081600e); };
            c.w32(8, t277);
            let r278 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r278);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 293; continue; }
            blk = 288; continue;
        }
        288 => {
            let Some(t279) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081601a); };
            c.w32(11, t279);
            let r280 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r280);
            let Some(t281) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816021); };
            c.cmp(t281, 0x42u64, 16);
            if c.cc_ne() { blk = 819; continue; }
            blk = 289; continue;
        }
        289 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081602d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 819; continue; }
            blk = 290; continue;
        }
        290 => {
            if c.cc_ne() { blk = 819; continue; }
            blk = 291; continue;
        }
        291 => {
            let Some(t282) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000816050); };
            c.w64(12, t282);
            let r283 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r283);
            let Some(t284) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x700081605a); };
            c.w64(15, t284);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 294; continue; }
            blk = 292; continue;
        }
        292 => {
            blk = 819; continue;
        }
        293 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 294; continue;
        }
        294 => {
            let Some(t285) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816071); };
            c.w32(12, t285);
            let r286 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r286);
            let Some(t287) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816078); };
            c.cmp(t287, 0x834u64, 16);
            if c.cc_e() { blk = 296; continue; }
            blk = 295; continue;
        }
        295 => {
            let Some(t288) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816082); };
            c.w32(12, t288);
            blk = 297; continue;
        }
        296 => {
            let Some(t289) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000816088); };
            c.w32(12, t289);
            blk = 297; continue;
        }
        297 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000816095); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 300; continue; }
            blk = 298; continue;
        }
        298 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 316; continue; }
            blk = 299; continue;
        }
        299 => {
            blk = 819; continue;
        }
        300 => {
            let Some(t290) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008160ac); };
            c.w32(15, t290);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_b() { blk = 314; continue; }
            blk = 301; continue;
        }
        301 => {
            let r291 = c.add(c.r64(3), 0x400u64, 64);
            c.w64(3, r291);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_be() { blk = 936; continue; }
            blk = 302; continue;
        }
        302 => {
            c.w64(3, c.r64(11).wrapping_add(0x1));
            c.w64(7, c.r64(3));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 936; continue; }
            blk = 303; continue;
        }
        303 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 934; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t292) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008160fb); };
            c.w64(3, t292);
            let Some(t293) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000816102); };
            c.w64(1, t293);
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(0, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t294) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000816111); };
            c.w64(12, t294);
            let Some(t295) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000816118); };
            c.cmp(t295, c.r64(0), 64);
            if c.cc_be() { blk = 932; continue; }
            blk = 305; continue;
        }
        305 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x7000816122); }
            let r296 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r296);
            blk = 306; continue;
        }
        306 => {
            c.w32(9, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081612d); };
            c.w64(12, t297);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000816134); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000816138); }
            c.w64(9, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_e() { blk = 309; continue; }
            blk = 307; continue;
        }
        307 => {
            let Some(t298) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816151); };
            c.w64(12, t298);
            blk = 308; continue;
        }
        308 => {
            let r299 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r299);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081615c); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 308; continue; }
            blk = 309; continue;
        }
        309 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 311; continue; }
            blk = 310; continue;
        }
        310 => {
            let r300 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r300);
            let Some(t301) = c.ld32(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700081616f); };
            c.w32(7, t301);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x7000816173); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 310; continue; }
            blk = 311; continue;
        }
        311 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081617d); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 313; continue; }
            blk = 312; continue;
        }
        312 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r302 = c.r64(7) & c.r64(2);
            c.w64(7, r302);
            c.logic(r302, 64);
            let Some(t303) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081618f); };
            c.logic(t303 & 0x4u64, 8);
            if c.cc_ne() { blk = 1256; continue; }
            blk = 313; continue;
        }
        313 => {
            c.w64(8, c.r64(1));
            blk = 314; continue;
        }
        314 => {
            let Some(t304) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081619c); };
            c.w32(1, t304);
            let r305 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r305);
            let Some(t306) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008161a2); };
            c.cmp(t306, 0x834u64, 16);
            if c.cc_ne() { blk = 316; continue; }
            blk = 315; continue;
        }
        315 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r307 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r307);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008161b0); }
            blk = 316; continue;
        }
        316 => {
            let Some(t308) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008161b3); };
            c.w32(1, t308);
            let Some(t309) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008161b7); };
            c.cmp(t309, c.r32(1), 32);
            if c.cc_ne() { blk = 329; continue; }
            blk = 317; continue;
        }
        317 => {
            let Some(t310) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008161c4); };
            c.w32(1, t310);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 333; continue; }
            blk = 318; continue;
        }
        318 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 955; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008161eb); };
            c.w64(9, t311);
            let Some(t312) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008161f2); };
            c.w64(12, t312);
            c.w64(15, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t313) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000816201); };
            c.w64(3, t313);
            let Some(t314) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000816208); };
            c.cmp(t314, c.r64(0), 64);
            if c.cc_be() { blk = 952; continue; }
            blk = 320; continue;
        }
        320 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x7000816211); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 321; continue;
        }
        321 => {
            let r315 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r315);
            let Some(t316) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081621b); };
            c.w64(12, t316);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000816222); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000816226); }
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081622a); };
            c.w64(7, t317);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 324; continue; }
            blk = 322; continue;
        }
        322 => {
            let Some(t318) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816243); };
            c.w64(12, t318);
            blk = 323; continue;
        }
        323 => {
            let r319 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r319);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x700081624e); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 323; continue; }
            blk = 324; continue;
        }
        324 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 326; continue; }
            blk = 325; continue;
        }
        325 => {
            let r320 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r320);
            let Some(t321) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000816261); };
            c.w32(1, t321);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x7000816265); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 325; continue; }
            blk = 326; continue;
        }
        326 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x700081626f); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 328; continue; }
            blk = 327; continue;
        }
        327 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r322 = c.r64(1) & c.r64(2);
            c.w64(1, r322);
            c.logic(r322, 64);
            let Some(t323) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816283); };
            c.logic(t323 & 0x4u64, 8);
            if c.cc_ne() { blk = 1253; continue; }
            blk = 328; continue;
        }
        328 => {
            c.w64(8, c.r64(9));
            blk = 329; continue;
        }
        329 => {
            let Some(t324) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816290); };
            c.w32(1, t324);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 954; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008162ab); };
            c.w64(0, t325);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008162af); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 332; continue; }
            blk = 331; continue;
        }
        331 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r326 = c.r64(7) & c.r64(8);
            c.w64(7, r326);
            c.logic(r326, 64);
            let Some(t327) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008162c1); };
            c.logic(t327 & 0x4u64, 8);
            if c.cc_ne() { blk = 1250; continue; }
            blk = 332; continue;
        }
        332 => {
            blk = 817; continue;
        }
        333 => {
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008162d0); };
            c.w64(1, t328);
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008162d4); };
            c.w64(0, t329);
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008162d8); };
            c.w64(9, t330);
            blk = 819; continue;
        }
        334 => {
            c.w64(9, c.r64(2));
            let r331 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r331);
            c.logic(r331, 64);
            let Some(t332) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008162eb); };
            c.w64(12, t332);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 342; continue; }
            blk = 335; continue;
        }
        335 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 342; continue; }
            blk = 336; continue;
        }
        336 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t333) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008162ff); };
            c.w64(15, t333);
            let Some(t334) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000816306); };
            c.w64(15, t334);
            c.w64(3, c.r64(12));
            let r335 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r335);
            c.logic(r335, 64);
            c.w64(7, c.r64(15));
            let r336 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r336);
            c.logic(r336, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 339; continue; }
            blk = 337; continue;
        }
        337 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 340; continue; }
            blk = 338; continue;
        }
        338 => {
            blk = 342; continue;
        }
        339 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 342; continue; }
            blk = 340; continue;
        }
        340 => {
            let Some(t337) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081632e); };
            c.w32(7, t337);
            let Some(t338) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000816331); };
            c.cmp(t338, c.r32(7), 32);
            if c.cc_ne() { blk = 342; continue; }
            blk = 341; continue;
        }
        341 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        342 => {
            let Some(t339) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816342); };
            c.w32(7, t339);
            let r340 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r340);
            let Some(t341) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000816348); };
            c.cmp(t341, c.r32(7), 32);
            if c.cc_e() { blk = 362; continue; }
            blk = 343; continue;
        }
        343 => {
            let Some(t342) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816355); };
            c.w32(9, t342);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t343) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816362); };
            c.cmp(t343, 0x834u64, 16);
            if c.cc_e() { blk = 345; continue; }
            blk = 344; continue;
        }
        344 => {
            c.w64(8, c.r64(12));
            blk = 346; continue;
        }
        345 => {
            let Some(t344) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000816370); };
            c.w32(8, t344);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 346; continue;
        }
        346 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 361; continue; }
            blk = 347; continue;
        }
        347 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 961; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 957; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008163a9); };
            c.w64(3, t345);
            let Some(t346) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008163b0); };
            c.w64(0, t346);
            c.w64(2, c.r64(15).wrapping_add(c.r64(0)));
            let Some(t347) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008163b7); };
            c.cmp(t347, c.r64(2), 64);
            if c.cc_be() { blk = 959; continue; }
            blk = 350; continue;
        }
        350 => {
            if !c.st64(c.r64(3), c.r64(2)) { return RegfileExit::fault(0x70008163c1); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 351; continue;
        }
        351 => {
            let r348 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r348);
            let Some(t349) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008163cb); };
            c.w64(0, t349);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008163d2); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008163d6); }
            let Some(t350) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008163da); };
            c.w64(12, t350);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008163e1); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 353; continue; }
            blk = 352; continue;
        }
        352 => {
            let r351 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r351);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x7000816401); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 352; continue; }
            blk = 353; continue;
        }
        353 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 358; continue; }
            blk = 354; continue;
        }
        354 => {
            let r352 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r352);
            let Some(t353) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081641e); };
            c.w32(9, t353);
            let r354 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r354);
            let Some(t355) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816426); };
            c.cmp(t355, c.r32(9), 32);
            if c.cc_e() { blk = 356; continue; }
            blk = 355; continue;
        }
        355 => {
            let r356 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r356);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081643b); }
            blk = 357; continue;
        }
        356 => {
            let Some(t357) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x7000816443); };
            c.w64(9, t357);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x700081644a); }
            blk = 357; continue;
        }
        357 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 354; continue; }
            blk = 358; continue;
        }
        358 => {
            let Some(t358) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000816454); };
            c.w64(7, t358);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000816458); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 360; continue; }
            blk = 359; continue;
        }
        359 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r359 = c.r64(8) & c.r64(7);
            c.w64(8, r359);
            c.logic(r359, 64);
            let Some(t360) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081646c); };
            c.logic(t360 & 0x4u64, 8);
            if c.cc_ne() { blk = 1247; continue; }
            blk = 360; continue;
        }
        360 => {
            c.w64(2, c.r64(7));
            let Some(t361) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081647a); };
            c.w64(0, t361);
            blk = 362; continue;
        }
        361 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        362 => {
            c.w64(7, c.r64(11));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700081648b); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r362 = c.r64(8) & c.r64(2);
            c.w64(8, r362);
            c.logic(r362, 64);
            let Some(t363) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816498); };
            c.logic(t363 & 0x4u64, 8);
            if c.cc_ne() { blk = 1244; continue; }
            blk = 363; continue;
        }
        363 => {
            let Some(t364) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008164a3); };
            c.w32(8, t364);
            let r365 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r365);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 371; continue; }
            blk = 364; continue;
        }
        364 => {
            let Some(t366) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008164af); };
            c.w32(9, t366);
            let r367 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r367);
            let Some(t368) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008164b6); };
            c.cmp(t368, 0x42u64, 16);
            if c.cc_e() { blk = 366; continue; }
            blk = 365; continue;
        }
        365 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        366 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008164c6); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 368; continue; }
            blk = 367; continue;
        }
        367 => {
            if c.cc_e() { blk = 369; continue; }
            blk = 368; continue;
        }
        368 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        369 => {
            let Some(t369) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x70008164e9); };
            c.w64(11, t369);
            let r370 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r370);
            let Some(t371) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x70008164f3); };
            c.w64(12, t371);
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_be() { blk = 372; continue; }
            blk = 370; continue;
        }
        370 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        371 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 372; continue;
        }
        372 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 376; continue; }
            blk = 373; continue;
        }
        373 => {
            let Some(t372) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816511); };
            c.w32(11, t372);
            let Some(t373) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000816515); };
            c.cmp(t373, c.r32(11), 32);
            if c.cc_ne() { blk = 375; continue; }
            blk = 374; continue;
        }
        374 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700081651e); }
            blk = 377; continue;
        }
        375 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        376 => {
            c.w64(11, c.r64(0));
            let r374 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r374);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 377; continue;
        }
        377 => {
            let Some(t375) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081653c); };
            c.w32(11, t375);
            let r376 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r376);
            let Some(t377) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816543); };
            c.cmp(t377, 0x834u64, 16);
            if c.cc_e() { blk = 379; continue; }
            blk = 378; continue;
        }
        378 => {
            let Some(t378) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081654c); };
            c.w32(11, t378);
            blk = 380; continue;
        }
        379 => {
            let Some(t379) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000816552); };
            c.w32(11, t379);
            blk = 380; continue;
        }
        380 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_be() { blk = 383; continue; }
            blk = 381; continue;
        }
        381 => {
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 403; continue; }
            blk = 382; continue;
        }
        382 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        383 => {
            let Some(t380) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816572); };
            c.w32(11, t380);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(11));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_b() { blk = 401; continue; }
            blk = 384; continue;
        }
        384 => {
            let r381 = c.add(c.r64(12), 0x400u64, 64);
            c.w64(12, r381);
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 969; continue; }
            blk = 385; continue;
        }
        385 => {
            c.w64(12, c.r64(9).wrapping_add(0x1));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.shr(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(12, c.r64(15).wrapping_add(c.r64(12)).wrapping_add(0x10));
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 969; continue; }
            blk = 386; continue;
        }
        386 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 967; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(1, c.r64(12).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 963; continue; }
            blk = 388; continue;
        }
        388 => {
            let Some(t382) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008165d6); };
            c.w64(7, t382);
            let Some(t383) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008165dd); };
            c.w64(15, t383);
            c.w64(3, c.r64(1).wrapping_add(c.r64(15)));
            let Some(t384) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008165e4); };
            c.cmp(t384, c.r64(3), 64);
            if c.cc_be() { blk = 965; continue; }
            blk = 389; continue;
        }
        389 => {
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x70008165ee); }
            c.w64(1, c.r64(15).wrapping_add(0x1));
            blk = 390; continue;
        }
        390 => {
            c.w32(7, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x70008165f9); };
            c.w64(15, t385);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000816600); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000816604); }
            let Some(t386) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816607); };
            c.w64(7, t386);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x700081660e); }
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_e() { blk = 392; continue; }
            blk = 391; continue;
        }
        391 => {
            let r387 = c.sub(c.r64(11), 0x8u64, 64);
            c.w64(11, r387);
            if !c.movsd_mr(c.r64(11).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x700081662c); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ne() { blk = 391; continue; }
            blk = 392; continue;
        }
        392 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 398; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t388) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x700081663d); };
            c.w64(11, t388);
            blk = 394; continue;
        }
        394 => {
            let r389 = c.sub(c.r64(7), 0x8u64, 64);
            c.w64(7, r389);
            let Some(t390) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000816648); };
            c.cmp(t390, c.r64(11), 64);
            if c.cc_e() { blk = 396; continue; }
            blk = 395; continue;
        }
        395 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(c.r64(8))) { return RegfileExit::fault(0x700081664e); }
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x7000816654); }
            blk = 397; continue;
        }
        396 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(1)), c.r64(11)) { return RegfileExit::fault(0x700081665b); }
            blk = 397; continue;
        }
        397 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 394; continue; }
            blk = 398; continue;
        }
        398 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000816665); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 400; continue; }
            blk = 399; continue;
        }
        399 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r391 = c.r64(7) & c.r64(2);
            c.w64(7, r391);
            c.logic(r391, 64);
            let Some(t392) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816677); };
            c.logic(t392 & 0x4u64, 8);
            if c.cc_ne() { blk = 1241; continue; }
            blk = 400; continue;
        }
        400 => {
            c.w64(8, c.r64(1));
            blk = 401; continue;
        }
        401 => {
            let Some(t393) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816684); };
            c.w32(1, t393);
            let r394 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r394);
            let Some(t395) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081668a); };
            c.cmp(t395, 0x834u64, 16);
            if c.cc_ne() { blk = 403; continue; }
            blk = 402; continue;
        }
        402 => {
            c.w64(1, c.r64(9).wrapping_add(0x1));
            let r396 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r396);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000816698); }
            blk = 403; continue;
        }
        403 => {
            let Some(t397) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081669b); };
            c.w32(1, t397);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 986; continue; }
            blk = 404; continue;
        }
        404 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x70008166b8); }
            blk = 817; continue;
        }
        405 => {
            c.w64(8, c.r64(2));
            let r398 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r398);
            c.logic(r398, 64);
            let Some(t399) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008166ce); };
            c.w64(9, t399);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 413; continue; }
            blk = 406; continue;
        }
        406 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 413; continue; }
            blk = 407; continue;
        }
        407 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t400) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008166e2); };
            c.w64(12, t400);
            let Some(t401) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008166e9); };
            c.w64(12, t401);
            c.w64(15, c.r64(9));
            let r402 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r402);
            c.logic(r402, 64);
            c.w64(3, c.r64(12));
            let r403 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r403);
            c.logic(r403, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 410; continue; }
            blk = 408; continue;
        }
        408 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 411; continue; }
            blk = 409; continue;
        }
        409 => {
            blk = 413; continue;
        }
        410 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 413; continue; }
            blk = 411; continue;
        }
        411 => {
            let Some(t404) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000816712); };
            c.w32(8, t404);
            let Some(t405) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000816716); };
            c.cmp(t405, c.r32(8), 32);
            if c.cc_ne() { blk = 413; continue; }
            blk = 412; continue;
        }
        412 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        413 => {
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700081672a); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r406 = c.r64(8) & c.r64(2);
            c.w64(8, r406);
            c.logic(r406, 64);
            let Some(t407) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816738); };
            c.logic(t407 & 0x4u64, 8);
            if c.cc_ne() { blk = 1238; continue; }
            blk = 414; continue;
        }
        414 => {
            let Some(t408) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816743); };
            c.w32(8, t408);
            let r409 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r409);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 420; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t410) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081674f); };
            c.w32(11, t410);
            let r411 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r411);
            let Some(t412) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816756); };
            c.cmp(t412, 0x42u64, 16);
            if c.cc_ne() { blk = 819; continue; }
            blk = 416; continue;
        }
        416 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000816762); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 819; continue; }
            blk = 417; continue;
        }
        417 => {
            if c.cc_ne() { blk = 819; continue; }
            blk = 418; continue;
        }
        418 => {
            let Some(t413) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000816785); };
            c.w64(12, t413);
            let r414 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r414);
            let Some(t415) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x700081678f); };
            c.w64(15, t415);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 421; continue; }
            blk = 419; continue;
        }
        419 => {
            blk = 819; continue;
        }
        420 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 421; continue;
        }
        421 => {
            let Some(t416) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008167a6); };
            c.w32(12, t416);
            let r417 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r417);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008167ad); }
            let Some(t418) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008167b1); };
            c.cmp(t418, 0x834u64, 16);
            if c.cc_e() { blk = 423; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t419) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008167bb); };
            c.w32(12, t419);
            blk = 424; continue;
        }
        423 => {
            let Some(t420) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008167c1); };
            c.w32(12, t420);
            blk = 424; continue;
        }
        424 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008167ce); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 427; continue; }
            blk = 425; continue;
        }
        425 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 444; continue; }
            blk = 426; continue;
        }
        426 => {
            blk = 819; continue;
        }
        427 => {
            let Some(t421) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008167e5); };
            c.w32(15, t421);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_b() { blk = 441; continue; }
            blk = 428; continue;
        }
        428 => {
            let r422 = c.add(c.r64(3), 0x400u64, 64);
            c.w64(3, r422);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_be() { blk = 991; continue; }
            blk = 429; continue;
        }
        429 => {
            c.w64(3, c.r64(11).wrapping_add(0x1));
            c.w64(7, c.r64(3));
            if let Some(sr) = c.shr(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(7, c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x10));
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 991; continue; }
            blk = 430; continue;
        }
        430 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 989; continue; }
            blk = 431; continue;
        }
        431 => {
            let Some(t423) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000816834); };
            c.w64(3, t423);
            let Some(t424) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081683b); };
            c.w64(1, t424);
            c.w64(9, c.r64(7).wrapping_mul(4).wrapping_add(0x8));
            c.w64(12, c.r64(9).wrapping_add(c.r64(1)));
            let Some(t425) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081684a); };
            c.w64(11, t425);
            let Some(t426) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000816851); };
            c.cmp(t426, c.r64(12), 64);
            if c.cc_be() { blk = 987; continue; }
            blk = 432; continue;
        }
        432 => {
            if !c.st64(c.r64(3), c.r64(12)) { return RegfileExit::fault(0x700081685a); }
            let r427 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r427);
            blk = 433; continue;
        }
        433 => {
            c.w32(9, c.r64(7).wrapping_add(c.r64(7)));
            let Some(t428) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000816865); };
            c.w64(11, t428);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081686c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000816870); }
            c.w64(9, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_e() { blk = 436; continue; }
            blk = 434; continue;
        }
        434 => {
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816889); };
            c.w64(11, t429);
            blk = 435; continue;
        }
        435 => {
            let r430 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r430);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x7000816894); }
            c.cmp(c.r64(7), c.r64(9), 64);
            if c.cc_ne() { blk = 435; continue; }
            blk = 436; continue;
        }
        436 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 438; continue; }
            blk = 437; continue;
        }
        437 => {
            let r431 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r431);
            let Some(t432) = c.ld32(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008168a7); };
            c.w32(7, t432);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(7)) { return RegfileExit::fault(0x70008168ab); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 437; continue; }
            blk = 438; continue;
        }
        438 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008168b5); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 440; continue; }
            blk = 439; continue;
        }
        439 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r433 = c.r64(7) & c.r64(2);
            c.w64(7, r433);
            c.logic(r433, 64);
            let Some(t434) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008168c7); };
            c.logic(t434 & 0x4u64, 8);
            if c.cc_ne() { blk = 1235; continue; }
            blk = 440; continue;
        }
        440 => {
            c.w64(8, c.r64(1));
            blk = 441; continue;
        }
        441 => {
            let Some(t435) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008168d4); };
            c.w32(1, t435);
            let r436 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r436);
            let Some(t437) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008168da); };
            c.cmp(t437, 0x834u64, 16);
            if c.cc_e() { blk = 443; continue; }
            blk = 442; continue;
        }
        442 => {
            let Some(t438) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008168e2); };
            c.w64(11, t438);
            blk = 444; continue;
        }
        443 => {
            let Some(t439) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008168e8); };
            c.w64(11, t439);
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r440 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r440);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008168f2); }
            blk = 444; continue;
        }
        444 => {
            let Some(t441) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008168f5); };
            c.w32(1, t441);
            let Some(t442) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008168f9); };
            c.cmp(t442, c.r32(1), 32);
            if c.cc_ne() { blk = 457; continue; }
            blk = 445; continue;
        }
        445 => {
            let Some(t443) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816906); };
            c.w32(1, t443);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 461; continue; }
            blk = 446; continue;
        }
        446 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1010; continue; }
            blk = 447; continue;
        }
        447 => {
            let Some(t444) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081692d); };
            c.w64(9, t444);
            let Some(t445) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000816934); };
            c.w64(12, t445);
            c.w64(15, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(15).wrapping_add(c.r64(12)));
            let Some(t446) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000816943); };
            c.w64(6, t446);
            let Some(t447) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x700081694a); };
            c.cmp(t447, c.r64(3), 64);
            if c.cc_be() { blk = 1007; continue; }
            blk = 448; continue;
        }
        448 => {
            if !c.st64(c.r64(9), c.r64(3)) { return RegfileExit::fault(0x7000816953); }
            c.w64(9, c.r64(12).wrapping_add(0x1));
            blk = 449; continue;
        }
        449 => {
            let r448 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r448);
            let Some(t449) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081695d); };
            c.w64(12, t449);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000816964); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000816968); }
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081696c); };
            c.w64(7, t450);
            c.w64(7, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 452; continue; }
            blk = 450; continue;
        }
        450 => {
            let Some(t451) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816985); };
            c.w64(12, t451);
            blk = 451; continue;
        }
        451 => {
            let r452 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r452);
            if !c.st32(c.r64(1).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x7000816990); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 451; continue; }
            blk = 452; continue;
        }
        452 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 454; continue; }
            blk = 453; continue;
        }
        453 => {
            let r453 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r453);
            let Some(t454) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008169a3); };
            c.w32(1, t454);
            if !c.st32(c.r64(7).wrapping_add(c.r64(9)), c.r32(1)) { return RegfileExit::fault(0x70008169a7); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 453; continue; }
            blk = 454; continue;
        }
        454 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008169b1); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 456; continue; }
            blk = 455; continue;
        }
        455 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r455 = c.r64(1) & c.r64(2);
            c.w64(1, r455);
            c.logic(r455, 64);
            let Some(t456) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008169c5); };
            c.logic(t456 & 0x4u64, 8);
            if c.cc_ne() { blk = 1232; continue; }
            blk = 456; continue;
        }
        456 => {
            c.w64(8, c.r64(9));
            blk = 457; continue;
        }
        457 => {
            let Some(t457) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008169d2); };
            c.w32(1, t457);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 1009; continue; }
            blk = 458; continue;
        }
        458 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x70008169ed); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 460; continue; }
            blk = 459; continue;
        }
        459 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r458 = c.r64(7) & c.r64(8);
            c.w64(7, r458);
            c.logic(r458, 64);
            let Some(t459) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008169ff); };
            c.logic(t459 & 0x4u64, 8);
            if c.cc_ne() { blk = 1229; continue; }
            blk = 460; continue;
        }
        460 => {
            blk = 817; continue;
        }
        461 => {
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000816a0e); };
            c.w64(1, t460);
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000816a12); };
            c.w64(9, t461);
            blk = 819; continue;
        }
        462 => {
            c.w64(8, c.r64(2));
            let r462 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r462);
            c.logic(r462, 64);
            let Some(t463) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816a25); };
            c.w64(9, t463);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 470; continue; }
            blk = 463; continue;
        }
        463 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 470; continue; }
            blk = 464; continue;
        }
        464 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t464) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000816a39); };
            c.w64(12, t464);
            let Some(t465) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000816a40); };
            c.w64(12, t465);
            c.w64(15, c.r64(9));
            let r466 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r466);
            c.logic(r466, 64);
            c.w64(3, c.r64(12));
            let r467 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r467);
            c.logic(r467, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 467; continue; }
            blk = 465; continue;
        }
        465 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 468; continue; }
            blk = 466; continue;
        }
        466 => {
            blk = 470; continue;
        }
        467 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 470; continue; }
            blk = 468; continue;
        }
        468 => {
            let Some(t468) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000816a69); };
            c.w32(8, t468);
            let Some(t469) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000816a6d); };
            c.cmp(t469, c.r32(8), 32);
            if c.cc_ne() { blk = 470; continue; }
            blk = 469; continue;
        }
        469 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        470 => {
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000816a81); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r470 = c.r64(8) & c.r64(2);
            c.w64(8, r470);
            c.logic(r470, 64);
            let Some(t471) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816a8f); };
            c.logic(t471 & 0x4u64, 8);
            if c.cc_ne() { blk = 1226; continue; }
            blk = 471; continue;
        }
        471 => {
            let Some(t472) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816a9a); };
            c.w32(8, t472);
            let r473 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r473);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 477; continue; }
            blk = 472; continue;
        }
        472 => {
            let Some(t474) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816aa6); };
            c.w32(11, t474);
            let r475 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r475);
            let Some(t476) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816aad); };
            c.cmp(t476, 0x42u64, 16);
            if c.cc_ne() { blk = 819; continue; }
            blk = 473; continue;
        }
        473 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000816ab9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 819; continue; }
            blk = 474; continue;
        }
        474 => {
            if c.cc_ne() { blk = 819; continue; }
            blk = 475; continue;
        }
        475 => {
            let Some(t477) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000816adc); };
            c.w64(12, t477);
            let r478 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r478);
            let Some(t479) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x7000816ae6); };
            c.w64(15, t479);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 478; continue; }
            blk = 476; continue;
        }
        476 => {
            blk = 819; continue;
        }
        477 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 478; continue;
        }
        478 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 481; continue; }
            blk = 479; continue;
        }
        479 => {
            let Some(t480) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816b01); };
            c.w32(12, t480);
            let Some(t481) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000816b05); };
            c.cmp(t481, c.r32(12), 32);
            if c.cc_ne() { blk = 819; continue; }
            blk = 480; continue;
        }
        480 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x7000816b12); }
            blk = 482; continue;
        }
        481 => {
            c.w64(12, c.r64(0));
            let r482 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r482);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 482; continue;
        }
        482 => {
            let Some(t483) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816b28); };
            c.w32(12, t483);
            let r484 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r484);
            let Some(t485) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816b2f); };
            c.cmp(t485, 0x834u64, 16);
            if c.cc_e() { blk = 484; continue; }
            blk = 483; continue;
        }
        483 => {
            let Some(t486) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816b39); };
            c.w32(12, t486);
            blk = 485; continue;
        }
        484 => {
            let Some(t487) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000816b3f); };
            c.w32(12, t487);
            blk = 485; continue;
        }
        485 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(12), c.r64(11), 64);
            if c.cc_be() { blk = 488; continue; }
            blk = 486; continue;
        }
        486 => {
            c.cmp(c.r64(11), c.r64(12), 64);
            if c.cc_b() { blk = 508; continue; }
            blk = 487; continue;
        }
        487 => {
            blk = 819; continue;
        }
        488 => {
            let Some(t488) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816b5c); };
            c.w32(12, t488);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 506; continue; }
            blk = 489; continue;
        }
        489 => {
            let r489 = c.add(c.r64(15), 0x400u64, 64);
            c.w64(15, r489);
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 1018; continue; }
            blk = 490; continue;
        }
        490 => {
            c.w64(15, c.r64(11).wrapping_add(0x1));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(15, c.r64(3).wrapping_add(c.r64(15)).wrapping_add(0x10));
            c.cmp(c.r64(15), 0x3fffu64, 64);
            if c.cc_ae() { blk = 1018; continue; }
            blk = 491; continue;
        }
        491 => {
            c.cmp(c.r64(15), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 1016; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(1, c.r64(15).wrapping_mul(8).wrapping_add(0x8));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 1012; continue; }
            blk = 493; continue;
        }
        493 => {
            let Some(t490) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000816bc0); };
            c.w64(7, t490);
            let Some(t491) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000816bc7); };
            c.w64(9, t491);
            c.w64(3, c.r64(1).wrapping_add(c.r64(9)));
            let Some(t492) = c.ld64(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816bce); };
            c.cmp(t492, c.r64(3), 64);
            if c.cc_be() { blk = 1014; continue; }
            blk = 494; continue;
        }
        494 => {
            if !c.st64(c.r64(7), c.r64(3)) { return RegfileExit::fault(0x7000816bd8); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 495; continue;
        }
        495 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t493) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000816be3); };
            c.w64(9, t493);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000816bea); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000816bee); }
            let Some(t494) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816bf1); };
            c.w64(7, t494);
            if !c.movsd_rm(1, c.r64(7).wrapping_add(0x3)) { return RegfileExit::fault(0x7000816bf8); }
            c.w64(7, c.r64(12).wrapping_mul(4).wrapping_add(0x7));
            c.w64(9, c.r64(15).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_e() { blk = 497; continue; }
            blk = 496; continue;
        }
        496 => {
            let r495 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r495);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x7000816c16); }
            c.cmp(c.r64(9), c.r64(7), 64);
            if c.cc_ne() { blk = 496; continue; }
            blk = 497; continue;
        }
        497 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 503; continue; }
            blk = 498; continue;
        }
        498 => {
            let Some(t496) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x7000816c27); };
            c.w64(9, t496);
            blk = 499; continue;
        }
        499 => {
            let r497 = c.sub(c.r64(7), 0x8u64, 64);
            c.w64(7, r497);
            let Some(t498) = c.ld64(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000816c32); };
            c.cmp(t498, c.r64(9), 64);
            if c.cc_e() { blk = 501; continue; }
            blk = 500; continue;
        }
        500 => {
            if !c.movsd_rm(1, c.r64(7).wrapping_add(c.r64(8))) { return RegfileExit::fault(0x7000816c38); }
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(1)), 1) { return RegfileExit::fault(0x7000816c3e); }
            blk = 502; continue;
        }
        501 => {
            if !c.st64(c.r64(7).wrapping_add(c.r64(1)), c.r64(9)) { return RegfileExit::fault(0x7000816c45); }
            blk = 502; continue;
        }
        502 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 499; continue; }
            blk = 503; continue;
        }
        503 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000816c4f); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 505; continue; }
            blk = 504; continue;
        }
        504 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r499 = c.r64(7) & c.r64(2);
            c.w64(7, r499);
            c.logic(r499, 64);
            let Some(t500) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816c61); };
            c.logic(t500 & 0x4u64, 8);
            if c.cc_ne() { blk = 1223; continue; }
            blk = 505; continue;
        }
        505 => {
            c.w64(8, c.r64(1));
            blk = 506; continue;
        }
        506 => {
            let Some(t501) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816c6e); };
            c.w32(1, t501);
            let r502 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r502);
            let Some(t503) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816c74); };
            c.cmp(t503, 0x834u64, 16);
            if c.cc_ne() { blk = 508; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r504 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r504);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000816c82); }
            blk = 508; continue;
        }
        508 => {
            let Some(t505) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816c85); };
            c.w32(1, t505);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 1034; continue; }
            blk = 509; continue;
        }
        509 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000816ca2); }
            blk = 817; continue;
        }
        510 => {
            c.w64(9, c.r64(2));
            let r506 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r506);
            c.logic(r506, 64);
            let Some(t507) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816cb8); };
            c.w64(12, t507);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 518; continue; }
            blk = 511; continue;
        }
        511 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 518; continue; }
            blk = 512; continue;
        }
        512 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t508) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000816ccc); };
            c.w64(15, t508);
            let Some(t509) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000816cd3); };
            c.w64(15, t509);
            c.w64(3, c.r64(12));
            let r510 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r510);
            c.logic(r510, 64);
            c.w64(1, c.r64(15));
            let r511 = c.r64(1) & 0xfffffffffffc0000u64;
            c.w64(1, r511);
            c.logic(r511, 64);
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_e() { blk = 515; continue; }
            blk = 513; continue;
        }
        513 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 516; continue; }
            blk = 514; continue;
        }
        514 => {
            blk = 518; continue;
        }
        515 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 518; continue; }
            blk = 516; continue;
        }
        516 => {
            let Some(t512) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000816cfb); };
            c.w32(1, t512);
            let Some(t513) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000816cfe); };
            c.cmp(t513, c.r32(1), 32);
            if c.cc_ne() { blk = 518; continue; }
            blk = 517; continue;
        }
        517 => {
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000816d07); };
            c.w64(1, t514);
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        518 => {
            let Some(t515) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816d13); };
            c.w32(1, t515);
            let r516 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r516);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(1)) { return RegfileExit::fault(0x7000816d19); }
            let Some(t517) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000816d1d); };
            c.cmp(t517, c.r32(1), 32);
            if c.cc_e() { blk = 542; continue; }
            blk = 519; continue;
        }
        519 => {
            let Some(t518) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816d2a); };
            c.w32(9, t518);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t519) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816d37); };
            c.cmp(t519, 0x834u64, 16);
            if c.cc_e() { blk = 521; continue; }
            blk = 520; continue;
        }
        520 => {
            c.w64(8, c.r64(12));
            blk = 522; continue;
        }
        521 => {
            let Some(t520) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000816d45); };
            c.w32(8, t520);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 522; continue;
        }
        522 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000816d4f); }
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 541; continue; }
            blk = 523; continue;
        }
        523 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1039; continue; }
            blk = 524; continue;
        }
        524 => {
            let Some(t521) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000816d6d); };
            c.w64(15, t521);
            let Some(t522) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000816d74); };
            c.w64(3, t522);
            c.w64(2, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(1, c.r64(2).wrapping_add(c.r64(3)));
            let Some(t523) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000816d83); };
            c.w64(8, t523);
            let Some(t524) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000816d8a); };
            c.cmp(t524, c.r64(1), 64);
            if c.cc_be() { blk = 1035; continue; }
            blk = 525; continue;
        }
        525 => {
            if !c.st64(c.r64(15), c.r64(1)) { return RegfileExit::fault(0x7000816d93); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 526; continue;
        }
        526 => {
            c.w32(8, c.r64(12).wrapping_add(c.r64(12)));
            let Some(t525) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000816d9e); };
            c.w64(12, t525);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000816da5); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000816da9); }
            c.w64(8, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 529; continue; }
            blk = 527; continue;
        }
        527 => {
            let Some(t526) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000816dbb); };
            c.w64(9, t526);
            blk = 528; continue;
        }
        528 => {
            let r527 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r527);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(9)) { return RegfileExit::fault(0x7000816dc6); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 528; continue; }
            blk = 529; continue;
        }
        529 => {
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000816dd0); };
            c.w64(8, t528);
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 538; continue; }
            blk = 530; continue;
        }
        530 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t529) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x7000816dee); };
            c.w64(15, t529);
            let Some(t530) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000816df5); };
            c.w64(12, t530);
            blk = 532; continue;
        }
        531 => {
            let r531 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r531);
            blk = 532; continue;
        }
        532 => {
            let r532 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r532);
            let Some(t533) = c.ld64(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x7000816e03); };
            c.cmp(t533, c.r64(15), 64);
            if c.cc_e() { blk = 537; continue; }
            blk = 533; continue;
        }
        533 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(12))) { return RegfileExit::fault(0x7000816e09); }
            let Some(t534) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000816e0f); };
            c.w64(3, t534);
            let Some(t535) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000816e16); };
            c.w64(2, t535);
            c.w64(7, c.r64(2).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t536) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000816e21); };
            c.w64(0, t536);
            let Some(t537) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000816e28); };
            c.cmp(t537, c.r64(7), 64);
            if c.cc_be() { blk = 1037; continue; }
            blk = 534; continue;
        }
        534 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x7000816e31); }
            c.w64(7, c.r64(2).wrapping_add(0x1));
            blk = 535; continue;
        }
        535 => {
            let Some(t538) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000816e38); };
            c.w64(0, t538);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000816e3f); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000816e42); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(7)) { return RegfileExit::fault(0x7000816e47); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 537; continue; }
            blk = 536; continue;
        }
        536 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r539 = c.r64(0) & c.r64(1);
            c.w64(0, r539);
            c.logic(r539, 64);
            let Some(t540) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816e5a); };
            c.logic(t540 & 0x4u64, 8);
            if c.cc_ne() { blk = 1220; continue; }
            blk = 537; continue;
        }
        537 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 531; continue; }
            blk = 538; continue;
        }
        538 => {
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000816e6a); };
            c.w64(7, t541);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000816e6e); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 540; continue; }
            blk = 539; continue;
        }
        539 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r542 = c.r64(8) & c.r64(7);
            c.w64(8, r542);
            c.logic(r542, 64);
            let Some(t543) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816e80); };
            c.logic(t543 & 0x4u64, 8);
            if c.cc_ne() { blk = 1217; continue; }
            blk = 540; continue;
        }
        540 => {
            blk = 543; continue;
        }
        541 => {
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000816e8d); };
            c.w64(1, t544);
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        542 => {
            c.w64(7, c.r64(2));
            blk = 543; continue;
        }
        543 => {
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000816e9f); }
            c.w64(1, 0xfffffffffffc0000u64);
            let r545 = c.r64(1) & c.r64(7);
            c.w64(1, r545);
            c.logic(r545, 64);
            let Some(t546) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000816ead); };
            c.logic(t546 & 0x4u64, 8);
            if c.cc_ne() { blk = 1214; continue; }
            blk = 544; continue;
        }
        544 => {
            let Some(t547) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816eb7); };
            c.w32(1, t547);
            let r548 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r548);
            let Some(t549) = c.ld8(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000816ebd); };
            c.logic(t549 & 0x1u64, 8);
            if c.cc_e() { blk = 552; continue; }
            blk = 545; continue;
        }
        545 => {
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000816ec7); };
            c.w64(11, t550);
            let Some(t551) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816ecb); };
            c.w32(8, t551);
            let r552 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r552);
            let Some(t553) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816ed2); };
            c.cmp(t553, 0x42u64, 16);
            if c.cc_e() { blk = 547; continue; }
            blk = 546; continue;
        }
        546 => {
            c.w64(2, c.r64(7));
            c.w64(1, c.r64(11));
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000816ee0); };
            c.w64(0, t554);
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000816ee4); };
            c.w64(6, t555);
            blk = 819; continue;
        }
        547 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000816eed); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 549; continue; }
            blk = 548; continue;
        }
        548 => {
            if c.cc_e() { blk = 550; continue; }
            blk = 549; continue;
        }
        549 => {
            c.w64(2, c.r64(7));
            c.w64(1, c.r64(11));
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000816f0f); };
            c.w64(0, t556);
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000816f13); };
            c.w64(6, t557);
            blk = 819; continue;
        }
        550 => {
            let Some(t558) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000816f1c); };
            c.w64(12, t558);
            let r559 = c.add(c.r64(12), c.r64(8), 64);
            c.w64(12, r559);
            let Some(t560) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x7000816f26); };
            c.w64(15, t560);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 553; continue; }
            blk = 551; continue;
        }
        551 => {
            c.w64(2, c.r64(7));
            c.w64(1, c.r64(11));
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000816f38); };
            c.w64(0, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000816f3c); };
            c.w64(6, t562);
            blk = 819; continue;
        }
        552 => {
            let Some(t563) = c.ld32(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000816f45); };
            c.w64(8, (t563 as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000816f4c); };
            c.w64(11, t564);
            blk = 553; continue;
        }
        553 => {
            let Some(t565) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000816f50); };
            c.w32(12, t565);
            let r566 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r566);
            let Some(t567) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000816f57); };
            c.cmp(t567, 0x834u64, 16);
            if c.cc_e() { blk = 555; continue; }
            blk = 554; continue;
        }
        554 => {
            let Some(t568) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816f61); };
            c.w32(12, t568);
            blk = 556; continue;
        }
        555 => {
            let Some(t569) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000816f67); };
            c.w32(12, t569);
            blk = 556; continue;
        }
        556 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_e() { blk = 559; continue; }
            blk = 557; continue;
        }
        557 => {
            c.cmp(c.r64(8), c.r64(15), 64);
            if c.cc_b() { blk = 575; continue; }
            blk = 558; continue;
        }
        558 => {
            c.w64(2, c.r64(7));
            c.w64(1, c.r64(11));
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000816f88); };
            c.w64(0, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000816f8c); };
            c.w64(6, t571);
            blk = 819; continue;
        }
        559 => {
            let Some(t572) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000816f95); };
            c.w32(15, t572);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(8), c.r64(0), 64);
            if c.cc_b() { blk = 573; continue; }
            blk = 560; continue;
        }
        560 => {
            let r573 = c.add(c.r64(0), 0x400u64, 64);
            c.w64(0, r573);
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_be() { blk = 1045; continue; }
            blk = 561; continue;
        }
        561 => {
            c.w64(0, c.r64(8).wrapping_add(0x1));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(0, c.r64(3).wrapping_add(c.r64(0)).wrapping_add(0x10));
            c.cmp(c.r64(0), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 1045; continue; }
            blk = 562; continue;
        }
        562 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1043; continue; }
            blk = 563; continue;
        }
        563 => {
            let Some(t574) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000816fe1); };
            c.w64(3, t574);
            let Some(t575) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000816fe8); };
            c.w64(2, t575);
            c.w64(6, c.r64(0).wrapping_mul(4).wrapping_add(0x8));
            c.w64(11, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t576) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000816ff7); };
            c.w64(9, t576);
            let Some(t577) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000816ffe); };
            c.cmp(t577, c.r64(11), 64);
            if c.cc_be() { blk = 1041; continue; }
            blk = 564; continue;
        }
        564 => {
            if !c.st64(c.r64(3), c.r64(11)) { return RegfileExit::fault(0x7000817007); }
            c.w64(9, c.r64(2).wrapping_add(0x1));
            blk = 565; continue;
        }
        565 => {
            c.w32(11, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t578) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000817012); };
            c.w64(3, t578);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000817019); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700081701d); }
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(15, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_e() { blk = 568; continue; }
            blk = 566; continue;
        }
        566 => {
            let Some(t579) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000817036); };
            c.w64(0, t579);
            blk = 567; continue;
        }
        567 => {
            let r580 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r580);
            if !c.st32(c.r64(15).wrapping_add(c.r64(9)), c.r32(0)) { return RegfileExit::fault(0x7000817041); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_ne() { blk = 567; continue; }
            blk = 568; continue;
        }
        568 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 570; continue; }
            blk = 569; continue;
        }
        569 => {
            let r581 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r581);
            let Some(t582) = c.ld32(c.r64(11).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x7000817054); };
            c.w32(15, t582);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x7000817058); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 569; continue; }
            blk = 570; continue;
        }
        570 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000817062); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 572; continue; }
            blk = 571; continue;
        }
        571 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r583 = c.r64(1) & c.r64(7);
            c.w64(1, r583);
            c.logic(r583, 64);
            let Some(t584) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817076); };
            c.logic(t584 & 0x4u64, 8);
            if c.cc_ne() { blk = 1211; continue; }
            blk = 572; continue;
        }
        572 => {
            c.w64(1, c.r64(9));
            blk = 573; continue;
        }
        573 => {
            let Some(t585) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817083); };
            c.w32(9, t585);
            let r586 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r586);
            let Some(t587) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081708a); };
            c.cmp(t587, 0x834u64, 16);
            if c.cc_ne() { blk = 575; continue; }
            blk = 574; continue;
        }
        574 => {
            c.w64(9, c.r64(8).wrapping_add(0x1));
            let r588 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r588);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x700081709a); }
            blk = 575; continue;
        }
        575 => {
            let Some(t589) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081709e); };
            c.w32(9, t589);
            let Some(t590) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008170a2); };
            c.cmp(t590, c.r32(9), 32);
            if c.cc_ne() { blk = 588; continue; }
            blk = 576; continue;
        }
        576 => {
            let Some(t591) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008170af); };
            c.w32(9, t591);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(11, c.r64(9));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 592; continue; }
            blk = 577; continue;
        }
        577 => {
            c.cmp(c.r64(11), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1065; continue; }
            blk = 578; continue;
        }
        578 => {
            let Some(t592) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008170d6); };
            c.w64(15, t592);
            let Some(t593) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008170dd); };
            c.w64(0, t593);
            c.w64(2, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t594) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008170ec); };
            c.w64(6, t594);
            let Some(t595) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008170f3); };
            c.cmp(t595, c.r64(3), 64);
            if c.cc_be() { blk = 1062; continue; }
            blk = 579; continue;
        }
        579 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008170fc); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 580; continue;
        }
        580 => {
            let r596 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r596);
            let Some(t597) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000817106); };
            c.w64(0, t597);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081710d); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000817111); }
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_e() { blk = 583; continue; }
            blk = 581; continue;
        }
        581 => {
            let Some(t598) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081712a); };
            c.w64(12, t598);
            blk = 582; continue;
        }
        582 => {
            let r599 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r599);
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(12)) { return RegfileExit::fault(0x7000817135); }
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_ne() { blk = 582; continue; }
            blk = 583; continue;
        }
        583 => {
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_e() { blk = 585; continue; }
            blk = 584; continue;
        }
        584 => {
            let r600 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r600);
            let Some(t601) = c.ld32(c.r64(11).wrapping_add(c.r64(1))) else { return RegfileExit::fault(0x7000817148); };
            c.w32(9, t601);
            if !c.st32(c.r64(11).wrapping_add(c.r64(15)), c.r32(9)) { return RegfileExit::fault(0x700081714c); }
            c.cmp(c.r64(11), 0x7u64, 64);
            if c.cc_ne() { blk = 584; continue; }
            blk = 585; continue;
        }
        585 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000817156); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 587; continue; }
            blk = 586; continue;
        }
        586 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r602 = c.r64(1) & c.r64(7);
            c.w64(1, r602);
            c.logic(r602, 64);
            let Some(t603) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081716a); };
            c.logic(t603 & 0x4u64, 8);
            if c.cc_ne() { blk = 1208; continue; }
            blk = 587; continue;
        }
        587 => {
            c.w64(1, c.r64(15));
            blk = 588; continue;
        }
        588 => {
            let Some(t604) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817177); };
            c.w32(7, t604);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 1064; continue; }
            blk = 589; continue;
        }
        589 => {
            c.w64(7, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817191); };
            c.w64(0, t605);
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(0)) { return RegfileExit::fault(0x7000817195); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 591; continue; }
            blk = 590; continue;
        }
        590 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r606 = c.r64(8) & c.r64(1);
            c.w64(8, r606);
            c.logic(r606, 64);
            let Some(t607) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008171a6); };
            c.logic(t607 & 0x4u64, 8);
            if c.cc_ne() { blk = 1205; continue; }
            blk = 591; continue;
        }
        591 => {
            blk = 817; continue;
        }
        592 => {
            c.w64(2, c.r64(7));
            let Some(t608) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008171b9); };
            c.w64(1, t608);
            let Some(t609) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008171bd); };
            c.w64(0, t609);
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008171c1); };
            c.w64(9, t610);
            let Some(t611) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008171c5); };
            c.w64(6, t611);
            blk = 819; continue;
        }
        593 => {
            c.w64(9, c.r64(2));
            let r612 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r612);
            c.logic(r612, 64);
            let Some(t613) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008171d8); };
            c.w64(12, t613);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 601; continue; }
            blk = 594; continue;
        }
        594 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 601; continue; }
            blk = 595; continue;
        }
        595 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t614) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008171ec); };
            c.w64(15, t614);
            let Some(t615) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008171f3); };
            c.w64(15, t615);
            c.w64(3, c.r64(12));
            let r616 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r616);
            c.logic(r616, 64);
            c.w64(7, c.r64(15));
            let r617 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r617);
            c.logic(r617, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 598; continue; }
            blk = 596; continue;
        }
        596 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 599; continue; }
            blk = 597; continue;
        }
        597 => {
            blk = 601; continue;
        }
        598 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 601; continue; }
            blk = 599; continue;
        }
        599 => {
            let Some(t618) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081721b); };
            c.w32(7, t618);
            let Some(t619) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081721e); };
            c.cmp(t619, c.r32(7), 32);
            if c.cc_ne() { blk = 601; continue; }
            blk = 600; continue;
        }
        600 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        601 => {
            let Some(t620) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081722f); };
            c.w32(7, t620);
            let r621 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r621);
            let Some(t622) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000817235); };
            c.cmp(t622, c.r32(7), 32);
            if c.cc_e() { blk = 626; continue; }
            blk = 602; continue;
        }
        602 => {
            let Some(t623) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817242); };
            c.w32(9, t623);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t624) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081724f); };
            c.cmp(t624, 0x834u64, 16);
            if c.cc_e() { blk = 604; continue; }
            blk = 603; continue;
        }
        603 => {
            c.w64(8, c.r64(12));
            blk = 605; continue;
        }
        604 => {
            let Some(t625) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081725d); };
            c.w32(8, t625);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 605; continue;
        }
        605 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 625; continue; }
            blk = 606; continue;
        }
        606 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1071; continue; }
            blk = 607; continue;
        }
        607 => {
            let Some(t626) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817281); };
            c.w64(15, t626);
            let Some(t627) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000817288); };
            c.w64(3, t627);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t628) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000817297); };
            c.w64(2, t628);
            let Some(t629) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700081729e); };
            c.cmp(t629, c.r64(0), 64);
            if c.cc_be() { blk = 1067; continue; }
            blk = 608; continue;
        }
        608 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x70008172a7); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 609; continue;
        }
        609 => {
            let r630 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r630);
            let Some(t631) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008172b1); };
            c.w64(0, t631);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008172b8); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008172bc); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 612; continue; }
            blk = 610; continue;
        }
        610 => {
            let Some(t632) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008172ce); };
            c.w64(12, t632);
            blk = 611; continue;
        }
        611 => {
            let r633 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r633);
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(12)) { return RegfileExit::fault(0x70008172d9); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 611; continue; }
            blk = 612; continue;
        }
        612 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 622; continue; }
            blk = 613; continue;
        }
        613 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t634) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x70008172fd); };
            c.w64(12, t634);
            blk = 615; continue;
        }
        614 => {
            let r635 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r635);
            blk = 615; continue;
        }
        615 => {
            let r636 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r636);
            let Some(t637) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081730e); };
            c.cmp(t637, c.r64(12), 64);
            if c.cc_e() { blk = 620; continue; }
            blk = 616; continue;
        }
        616 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x7000817314); }
            let Some(t638) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081731a); };
            c.w64(0, t638);
            let Some(t639) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000817321); };
            c.w64(3, t639);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t640) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081732c); };
            c.w64(1, t640);
            let Some(t641) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000817333); };
            c.cmp(t641, c.r64(2), 64);
            if c.cc_be() { blk = 1069; continue; }
            blk = 617; continue;
        }
        617 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700081733c); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 618; continue;
        }
        618 => {
            let Some(t642) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000817343); };
            c.w64(0, t642);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081734a); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081734d); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x7000817352); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 620; continue; }
            blk = 619; continue;
        }
        619 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r643 = c.r64(0) & c.r64(15);
            c.w64(0, r643);
            c.logic(r643, 64);
            let Some(t644) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817365); };
            c.logic(t644 & 0x4u64, 8);
            if c.cc_ne() { blk = 1202; continue; }
            blk = 620; continue;
        }
        620 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 614; continue; }
            blk = 621; continue;
        }
        621 => {
            let Some(t645) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000817375); };
            c.w64(1, t645);
            blk = 622; continue;
        }
        622 => {
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000817379); };
            c.w64(7, t646);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081737d); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 624; continue; }
            blk = 623; continue;
        }
        623 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r647 = c.r64(8) & c.r64(7);
            c.w64(8, r647);
            c.logic(r647, 64);
            let Some(t648) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817391); };
            c.logic(t648 & 0x4u64, 8);
            if c.cc_ne() { blk = 1199; continue; }
            blk = 624; continue;
        }
        624 => {
            blk = 627; continue;
        }
        625 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        626 => {
            c.w64(7, c.r64(2));
            blk = 627; continue;
        }
        627 => {
            c.w64(9, c.r64(11));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008173ac); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r649 = c.r64(8) & c.r64(7);
            c.w64(8, r649);
            c.logic(r649, 64);
            let Some(t650) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008173ba); };
            c.logic(t650 & 0x4u64, 8);
            if c.cc_ne() { blk = 1196; continue; }
            blk = 628; continue;
        }
        628 => {
            let Some(t651) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008173c5); };
            c.w32(8, t651);
            let r652 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r652);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 636; continue; }
            blk = 629; continue;
        }
        629 => {
            let Some(t653) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008173d1); };
            c.w32(11, t653);
            let r654 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r654);
            let Some(t655) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008173d8); };
            c.cmp(t655, 0x42u64, 16);
            if c.cc_e() { blk = 631; continue; }
            blk = 630; continue;
        }
        630 => {
            c.w64(2, c.r64(7));
            let Some(t656) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008173e3); };
            c.w64(0, t656);
            let Some(t657) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008173e7); };
            c.w64(6, t657);
            blk = 819; continue;
        }
        631 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008173f0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 633; continue; }
            blk = 632; continue;
        }
        632 => {
            if c.cc_e() { blk = 634; continue; }
            blk = 633; continue;
        }
        633 => {
            c.w64(2, c.r64(7));
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081740e); };
            c.w64(0, t658);
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000817412); };
            c.w64(6, t659);
            blk = 819; continue;
        }
        634 => {
            let Some(t660) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x700081741b); };
            c.w64(12, t660);
            let r661 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r661);
            let Some(t662) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x7000817425); };
            c.w64(15, t662);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 637; continue; }
            blk = 635; continue;
        }
        635 => {
            c.w64(2, c.r64(7));
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817434); };
            c.w64(0, t663);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000817438); };
            c.w64(6, t664);
            blk = 819; continue;
        }
        636 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 637; continue;
        }
        637 => {
            let Some(t665) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817447); };
            c.w32(12, t665);
            let r666 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r666);
            let Some(t667) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081744e); };
            c.cmp(t667, 0x834u64, 16);
            if c.cc_e() { blk = 639; continue; }
            blk = 638; continue;
        }
        638 => {
            let Some(t668) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817458); };
            c.w32(12, t668);
            blk = 640; continue;
        }
        639 => {
            let Some(t669) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081745e); };
            c.w32(12, t669);
            blk = 640; continue;
        }
        640 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 643; continue; }
            blk = 641; continue;
        }
        641 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 659; continue; }
            blk = 642; continue;
        }
        642 => {
            c.w64(2, c.r64(7));
            let Some(t670) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081747c); };
            c.w64(0, t670);
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000817480); };
            c.w64(6, t671);
            blk = 819; continue;
        }
        643 => {
            let Some(t672) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817489); };
            c.w32(15, t672);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 657; continue; }
            blk = 644; continue;
        }
        644 => {
            let r673 = c.add(c.r64(0), 0x400u64, 64);
            c.w64(0, r673);
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_be() { blk = 1077; continue; }
            blk = 645; continue;
        }
        645 => {
            c.w64(0, c.r64(11).wrapping_add(0x1));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(0, c.r64(3).wrapping_add(c.r64(0)).wrapping_add(0x10));
            c.cmp(c.r64(0), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 1077; continue; }
            blk = 646; continue;
        }
        646 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1075; continue; }
            blk = 647; continue;
        }
        647 => {
            let Some(t674) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008174d5); };
            c.w64(3, t674);
            let Some(t675) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008174dc); };
            c.w64(2, t675);
            c.w64(6, c.r64(0).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t676) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008174eb); };
            c.w64(9, t676);
            let Some(t677) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008174f2); };
            c.cmp(t677, c.r64(1), 64);
            if c.cc_be() { blk = 1073; continue; }
            blk = 648; continue;
        }
        648 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008174fb); }
            c.w64(1, c.r64(2).wrapping_add(0x1));
            blk = 649; continue;
        }
        649 => {
            c.w32(9, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t678) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000817506); };
            c.w64(3, t678);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x700081750d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x7000817510); }
            c.w64(9, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(15, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_e() { blk = 652; continue; }
            blk = 650; continue;
        }
        650 => {
            let Some(t679) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000817529); };
            c.w64(0, t679);
            blk = 651; continue;
        }
        651 => {
            let r680 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r680);
            if !c.st32(c.r64(15).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000817534); }
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ne() { blk = 651; continue; }
            blk = 652; continue;
        }
        652 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 654; continue; }
            blk = 653; continue;
        }
        653 => {
            let r681 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r681);
            let Some(t682) = c.ld32(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000817547); };
            c.w32(15, t682);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(15)) { return RegfileExit::fault(0x700081754b); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 653; continue; }
            blk = 654; continue;
        }
        654 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000817555); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 656; continue; }
            blk = 655; continue;
        }
        655 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r683 = c.r64(8) & c.r64(7);
            c.w64(8, r683);
            c.logic(r683, 64);
            let Some(t684) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817567); };
            c.logic(t684 & 0x4u64, 8);
            if c.cc_ne() { blk = 1193; continue; }
            blk = 656; continue;
        }
        656 => {
            c.w64(8, c.r64(1));
            blk = 657; continue;
        }
        657 => {
            let Some(t685) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817575); };
            c.w32(1, t685);
            let r686 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r686);
            let Some(t687) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081757b); };
            c.cmp(t687, 0x834u64, 16);
            if c.cc_ne() { blk = 659; continue; }
            blk = 658; continue;
        }
        658 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r688 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r688);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000817589); }
            blk = 659; continue;
        }
        659 => {
            let Some(t689) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081758c); };
            c.w32(1, t689);
            let Some(t690) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000817590); };
            c.cmp(t690, c.r32(1), 32);
            if c.cc_ne() { blk = 672; continue; }
            blk = 660; continue;
        }
        660 => {
            let Some(t691) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081759d); };
            c.w32(1, t691);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(1));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 676; continue; }
            blk = 661; continue;
        }
        661 => {
            c.cmp(c.r64(9), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1097; continue; }
            blk = 662; continue;
        }
        662 => {
            let Some(t692) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008175c4); };
            c.w64(15, t692);
            let Some(t693) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008175cb); };
            c.w64(0, t693);
            c.w64(2, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t694) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008175da); };
            c.w64(6, t694);
            let Some(t695) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008175e1); };
            c.cmp(t695, c.r64(3), 64);
            if c.cc_be() { blk = 1094; continue; }
            blk = 663; continue;
        }
        663 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x70008175ea); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 664; continue;
        }
        664 => {
            let r696 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r696);
            let Some(t697) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008175f4); };
            c.w64(0, t697);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008175fb); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(9)) { return RegfileExit::fault(0x70008175ff); }
            c.w64(9, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_e() { blk = 667; continue; }
            blk = 665; continue;
        }
        665 => {
            let Some(t698) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000817618); };
            c.w64(12, t698);
            blk = 666; continue;
        }
        666 => {
            let r699 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r699);
            if !c.st32(c.r64(1).wrapping_add(c.r64(15)), c.r32(12)) { return RegfileExit::fault(0x7000817623); }
            c.cmp(c.r64(1), c.r64(9), 64);
            if c.cc_ne() { blk = 666; continue; }
            blk = 667; continue;
        }
        667 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 669; continue; }
            blk = 668; continue;
        }
        668 => {
            let r700 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r700);
            let Some(t701) = c.ld32(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x7000817636); };
            c.w32(1, t701);
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(1)) { return RegfileExit::fault(0x700081763a); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 668; continue; }
            blk = 669; continue;
        }
        669 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000817644); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 671; continue; }
            blk = 670; continue;
        }
        670 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r702 = c.r64(1) & c.r64(7);
            c.w64(1, r702);
            c.logic(r702, 64);
            let Some(t703) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817658); };
            c.logic(t703 & 0x4u64, 8);
            if c.cc_ne() { blk = 1190; continue; }
            blk = 671; continue;
        }
        671 => {
            c.w64(8, c.r64(15));
            blk = 672; continue;
        }
        672 => {
            let Some(t704) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817665); };
            c.w32(1, t704);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 1096; continue; }
            blk = 673; continue;
        }
        673 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t705) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817680); };
            c.w64(0, t705);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000817684); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 675; continue; }
            blk = 674; continue;
        }
        674 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r706 = c.r64(7) & c.r64(8);
            c.w64(7, r706);
            c.logic(r706, 64);
            let Some(t707) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817696); };
            c.logic(t707 & 0x4u64, 8);
            if c.cc_ne() { blk = 1187; continue; }
            blk = 675; continue;
        }
        675 => {
            blk = 817; continue;
        }
        676 => {
            c.w64(2, c.r64(7));
            let Some(t708) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008176a8); };
            c.w64(1, t708);
            let Some(t709) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008176ac); };
            c.w64(0, t709);
            let Some(t710) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008176b0); };
            c.w64(9, t710);
            let Some(t711) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008176b4); };
            c.w64(6, t711);
            blk = 819; continue;
        }
        677 => {
            c.w64(9, c.r64(2));
            let r712 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r712);
            c.logic(r712, 64);
            let Some(t713) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008176c7); };
            c.w64(12, t713);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 685; continue; }
            blk = 678; continue;
        }
        678 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 685; continue; }
            blk = 679; continue;
        }
        679 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t714) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008176db); };
            c.w64(15, t714);
            let Some(t715) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008176e2); };
            c.w64(15, t715);
            c.w64(3, c.r64(12));
            let r716 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r716);
            c.logic(r716, 64);
            c.w64(7, c.r64(15));
            let r717 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r717);
            c.logic(r717, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 682; continue; }
            blk = 680; continue;
        }
        680 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 683; continue; }
            blk = 681; continue;
        }
        681 => {
            blk = 685; continue;
        }
        682 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 685; continue; }
            blk = 683; continue;
        }
        683 => {
            let Some(t718) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081770a); };
            c.w32(7, t718);
            let Some(t719) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081770d); };
            c.cmp(t719, c.r32(7), 32);
            if c.cc_ne() { blk = 685; continue; }
            blk = 684; continue;
        }
        684 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        685 => {
            let Some(t720) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081771e); };
            c.w32(7, t720);
            let r721 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r721);
            let Some(t722) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000817724); };
            c.cmp(t722, c.r32(7), 32);
            if c.cc_e() { blk = 710; continue; }
            blk = 686; continue;
        }
        686 => {
            let Some(t723) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817731); };
            c.w32(9, t723);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t724) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081773e); };
            c.cmp(t724, 0x834u64, 16);
            if c.cc_e() { blk = 688; continue; }
            blk = 687; continue;
        }
        687 => {
            c.w64(8, c.r64(12));
            blk = 689; continue;
        }
        688 => {
            let Some(t725) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081774c); };
            c.w32(8, t725);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 689; continue;
        }
        689 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 709; continue; }
            blk = 690; continue;
        }
        690 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1103; continue; }
            blk = 691; continue;
        }
        691 => {
            let Some(t726) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817770); };
            c.w64(15, t726);
            let Some(t727) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000817777); };
            c.w64(3, t727);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t728) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000817786); };
            c.w64(2, t728);
            let Some(t729) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x700081778d); };
            c.cmp(t729, c.r64(0), 64);
            if c.cc_be() { blk = 1099; continue; }
            blk = 692; continue;
        }
        692 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x7000817796); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            blk = 693; continue;
        }
        693 => {
            let r730 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r730);
            let Some(t731) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008177a0); };
            c.w64(0, t731);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008177a7); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008177ab); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 696; continue; }
            blk = 694; continue;
        }
        694 => {
            let Some(t732) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008177bd); };
            c.w64(12, t732);
            blk = 695; continue;
        }
        695 => {
            let r733 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r733);
            if !c.st32(c.r64(9).wrapping_add(c.r64(15)), c.r32(12)) { return RegfileExit::fault(0x70008177c8); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 695; continue; }
            blk = 696; continue;
        }
        696 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 706; continue; }
            blk = 697; continue;
        }
        697 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t734) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x70008177ec); };
            c.w64(12, t734);
            blk = 699; continue;
        }
        698 => {
            let r735 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r735);
            blk = 699; continue;
        }
        699 => {
            let r736 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r736);
            let Some(t737) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x70008177fd); };
            c.cmp(t737, c.r64(12), 64);
            if c.cc_e() { blk = 704; continue; }
            blk = 700; continue;
        }
        700 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x7000817803); }
            let Some(t738) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817809); };
            c.w64(0, t738);
            let Some(t739) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000817810); };
            c.w64(3, t739);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t740) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081781b); };
            c.w64(1, t740);
            let Some(t741) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000817822); };
            c.cmp(t741, c.r64(2), 64);
            if c.cc_be() { blk = 1101; continue; }
            blk = 701; continue;
        }
        701 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x700081782b); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 702; continue;
        }
        702 => {
            let Some(t742) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000817832); };
            c.w64(0, t742);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000817839); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081783c); }
            if !c.st32(c.r64(15).wrapping_add(c.r64(6)), c.r32(1)) { return RegfileExit::fault(0x7000817841); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 704; continue; }
            blk = 703; continue;
        }
        703 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r743 = c.r64(0) & c.r64(15);
            c.w64(0, r743);
            c.logic(r743, 64);
            let Some(t744) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817854); };
            c.logic(t744 & 0x4u64, 8);
            if c.cc_ne() { blk = 1184; continue; }
            blk = 704; continue;
        }
        704 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 698; continue; }
            blk = 705; continue;
        }
        705 => {
            let Some(t745) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000817864); };
            c.w64(1, t745);
            blk = 706; continue;
        }
        706 => {
            let Some(t746) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000817868); };
            c.w64(7, t746);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081786c); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 708; continue; }
            blk = 707; continue;
        }
        707 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r747 = c.r64(8) & c.r64(7);
            c.w64(8, r747);
            c.logic(r747, 64);
            let Some(t748) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817880); };
            c.logic(t748 & 0x4u64, 8);
            if c.cc_ne() { blk = 1181; continue; }
            blk = 708; continue;
        }
        708 => {
            blk = 711; continue;
        }
        709 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        710 => {
            c.w64(7, c.r64(2));
            blk = 711; continue;
        }
        711 => {
            c.w64(8, c.r64(11));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700081789b); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r749 = c.r64(9) & c.r64(7);
            c.w64(9, r749);
            c.logic(r749, 64);
            let Some(t750) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008178a9); };
            c.logic(t750 & 0x4u64, 8);
            if c.cc_ne() { blk = 1178; continue; }
            blk = 712; continue;
        }
        712 => {
            let Some(t751) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008178b4); };
            c.w32(9, t751);
            let r752 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r752);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 720; continue; }
            blk = 713; continue;
        }
        713 => {
            let Some(t753) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008178c4); };
            c.w32(11, t753);
            let r754 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r754);
            let Some(t755) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008178cb); };
            c.cmp(t755, 0x42u64, 16);
            if c.cc_e() { blk = 715; continue; }
            blk = 714; continue;
        }
        714 => {
            c.w64(2, c.r64(7));
            let Some(t756) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008178d6); };
            c.w64(0, t756);
            c.w64(9, c.r64(8));
            let Some(t757) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008178dd); };
            c.w64(6, t757);
            blk = 819; continue;
        }
        715 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008178e6); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 717; continue; }
            blk = 716; continue;
        }
        716 => {
            if c.cc_e() { blk = 718; continue; }
            blk = 717; continue;
        }
        717 => {
            c.w64(2, c.r64(7));
            let Some(t758) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817904); };
            c.w64(0, t758);
            c.w64(9, c.r64(8));
            let Some(t759) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081790b); };
            c.w64(6, t759);
            blk = 819; continue;
        }
        718 => {
            let Some(t760) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000817914); };
            c.w64(12, t760);
            let r761 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r761);
            let Some(t762) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x700081791e); };
            c.w64(15, t762);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 721; continue; }
            blk = 719; continue;
        }
        719 => {
            c.w64(2, c.r64(7));
            let Some(t763) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081792d); };
            c.w64(0, t763);
            c.w64(9, c.r64(8));
            let Some(t764) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000817934); };
            c.w64(6, t764);
            blk = 819; continue;
        }
        720 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 721; continue;
        }
        721 => {
            let Some(t765) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817943); };
            c.w32(12, t765);
            let r766 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r766);
            let Some(t767) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081794a); };
            c.cmp(t767, 0x834u64, 16);
            if c.cc_e() { blk = 723; continue; }
            blk = 722; continue;
        }
        722 => {
            let Some(t768) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817954); };
            c.w32(12, t768);
            blk = 724; continue;
        }
        723 => {
            let Some(t769) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081795a); };
            c.w32(12, t769);
            blk = 724; continue;
        }
        724 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 727; continue; }
            blk = 725; continue;
        }
        725 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 743; continue; }
            blk = 726; continue;
        }
        726 => {
            c.w64(2, c.r64(7));
            let Some(t770) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817978); };
            c.w64(0, t770);
            c.w64(9, c.r64(8));
            let Some(t771) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081797f); };
            c.w64(6, t771);
            blk = 819; continue;
        }
        727 => {
            let Some(t772) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817988); };
            c.w32(15, t772);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(15));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 741; continue; }
            blk = 728; continue;
        }
        728 => {
            let r773 = c.add(c.r64(0), 0x400u64, 64);
            c.w64(0, r773);
            c.cmp(c.r64(0), c.r64(11), 64);
            if c.cc_be() { blk = 1109; continue; }
            blk = 729; continue;
        }
        729 => {
            c.w64(0, c.r64(11).wrapping_add(0x1));
            c.w64(3, c.r64(0));
            if let Some(sr) = c.shr(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(0, c.r64(3).wrapping_add(c.r64(0)).wrapping_add(0x10));
            c.cmp(c.r64(0), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 1109; continue; }
            blk = 730; continue;
        }
        730 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1107; continue; }
            blk = 731; continue;
        }
        731 => {
            let Some(t774) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008179d4); };
            c.w64(3, t774);
            let Some(t775) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008179db); };
            c.w64(2, t775);
            c.w64(6, c.r64(0).wrapping_mul(4).wrapping_add(0x8));
            c.w64(1, c.r64(6).wrapping_add(c.r64(2)));
            let Some(t776) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008179ea); };
            c.w64(8, t776);
            let Some(t777) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008179f1); };
            c.cmp(t777, c.r64(1), 64);
            if c.cc_be() { blk = 1105; continue; }
            blk = 732; continue;
        }
        732 => {
            if !c.st64(c.r64(3), c.r64(1)) { return RegfileExit::fault(0x70008179fa); }
            c.w64(1, c.r64(2).wrapping_add(0x1));
            blk = 733; continue;
        }
        733 => {
            c.w32(8, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t778) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000817a05); };
            c.w64(3, t778);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000817a0c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000817a0f); }
            c.w64(8, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(15, c.r64(0).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 736; continue; }
            blk = 734; continue;
        }
        734 => {
            let Some(t779) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000817a28); };
            c.w64(0, t779);
            blk = 735; continue;
        }
        735 => {
            let r780 = c.sub(c.r64(15), 0x4u64, 64);
            c.w64(15, r780);
            if !c.st32(c.r64(15).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000817a33); }
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_ne() { blk = 735; continue; }
            blk = 736; continue;
        }
        736 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 738; continue; }
            blk = 737; continue;
        }
        737 => {
            let r781 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r781);
            let Some(t782) = c.ld32(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000817a46); };
            c.w32(15, t782);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(15)) { return RegfileExit::fault(0x7000817a4a); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 737; continue; }
            blk = 738; continue;
        }
        738 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000817a54); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 740; continue; }
            blk = 739; continue;
        }
        739 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r783 = c.r64(8) & c.r64(7);
            c.w64(8, r783);
            c.logic(r783, 64);
            let Some(t784) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817a66); };
            c.logic(t784 & 0x4u64, 8);
            if c.cc_ne() { blk = 1175; continue; }
            blk = 740; continue;
        }
        740 => {
            c.w64(9, c.r64(1));
            blk = 741; continue;
        }
        741 => {
            let Some(t785) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817a74); };
            c.w32(1, t785);
            let r786 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r786);
            let Some(t787) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000817a7a); };
            c.cmp(t787, 0x834u64, 16);
            if c.cc_ne() { blk = 743; continue; }
            blk = 742; continue;
        }
        742 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r788 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r788);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000817a88); }
            blk = 743; continue;
        }
        743 => {
            let Some(t789) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817a8b); };
            c.w32(1, t789);
            let Some(t790) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000817a8f); };
            c.cmp(t790, c.r32(1), 32);
            if c.cc_ne() { blk = 756; continue; }
            blk = 744; continue;
        }
        744 => {
            let Some(t791) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817a9c); };
            c.w32(1, t791);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(1));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 760; continue; }
            blk = 745; continue;
        }
        745 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1129; continue; }
            blk = 746; continue;
        }
        746 => {
            let Some(t792) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817ac3); };
            c.w64(15, t792);
            let Some(t793) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000817aca); };
            c.w64(0, t793);
            c.w64(2, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(2).wrapping_add(c.r64(0)));
            let Some(t794) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000817ad9); };
            c.w64(6, t794);
            let Some(t795) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000817ae0); };
            c.cmp(t795, c.r64(3), 64);
            if c.cc_be() { blk = 1126; continue; }
            blk = 747; continue;
        }
        747 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x7000817ae9); }
            c.w64(15, c.r64(0).wrapping_add(0x1));
            blk = 748; continue;
        }
        748 => {
            let r796 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r796);
            let Some(t797) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000817af3); };
            c.w64(0, t797);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000817afa); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000817afe); }
            c.w64(8, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_e() { blk = 751; continue; }
            blk = 749; continue;
        }
        749 => {
            let Some(t798) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000817b17); };
            c.w64(12, t798);
            blk = 750; continue;
        }
        750 => {
            let r799 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r799);
            if !c.st32(c.r64(1).wrapping_add(c.r64(15)), c.r32(12)) { return RegfileExit::fault(0x7000817b22); }
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_ne() { blk = 750; continue; }
            blk = 751; continue;
        }
        751 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 753; continue; }
            blk = 752; continue;
        }
        752 => {
            let r800 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r800);
            let Some(t801) = c.ld32(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000817b35); };
            c.w32(1, t801);
            if !c.st32(c.r64(8).wrapping_add(c.r64(15)), c.r32(1)) { return RegfileExit::fault(0x7000817b39); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 752; continue; }
            blk = 753; continue;
        }
        753 => {
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x7000817b43); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 755; continue; }
            blk = 754; continue;
        }
        754 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r802 = c.r64(1) & c.r64(7);
            c.w64(1, r802);
            c.logic(r802, 64);
            let Some(t803) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817b57); };
            c.logic(t803 & 0x4u64, 8);
            if c.cc_ne() { blk = 1172; continue; }
            blk = 755; continue;
        }
        755 => {
            c.w64(9, c.r64(15));
            blk = 756; continue;
        }
        756 => {
            let Some(t804) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817b64); };
            c.w32(1, t804);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 1128; continue; }
            blk = 757; continue;
        }
        757 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817b7f); };
            c.w64(0, t805);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000817b83); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 759; continue; }
            blk = 758; continue;
        }
        758 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r806 = c.r64(7) & c.r64(9);
            c.w64(7, r806);
            c.logic(r806, 64);
            let Some(t807) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817b95); };
            c.logic(t807 & 0x4u64, 8);
            if c.cc_ne() { blk = 1169; continue; }
            blk = 759; continue;
        }
        759 => {
            blk = 817; continue;
        }
        760 => {
            c.w64(2, c.r64(7));
            let Some(t808) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000817ba7); };
            c.w64(1, t808);
            let Some(t809) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817bab); };
            c.w64(0, t809);
            let Some(t810) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000817baf); };
            c.w64(9, t810);
            let Some(t811) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000817bb3); };
            c.w64(6, t811);
            blk = 819; continue;
        }
        761 => {
            c.w64(8, c.r64(2));
            let r812 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r812);
            c.logic(r812, 64);
            let Some(t813) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817bc6); };
            c.w64(9, t813);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 769; continue; }
            blk = 762; continue;
        }
        762 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 769; continue; }
            blk = 763; continue;
        }
        763 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t814) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817bda); };
            c.w64(12, t814);
            let Some(t815) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000817be1); };
            c.w64(12, t815);
            c.w64(15, c.r64(9));
            let r816 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r816);
            c.logic(r816, 64);
            c.w64(3, c.r64(12));
            let r817 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r817);
            c.logic(r817, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 766; continue; }
            blk = 764; continue;
        }
        764 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 767; continue; }
            blk = 765; continue;
        }
        765 => {
            blk = 769; continue;
        }
        766 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 769; continue; }
            blk = 767; continue;
        }
        767 => {
            let Some(t818) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000817c0a); };
            c.w32(8, t818);
            let Some(t819) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000817c0e); };
            c.cmp(t819, c.r32(8), 32);
            if c.cc_ne() { blk = 769; continue; }
            blk = 768; continue;
        }
        768 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        769 => {
            c.w64(8, c.r64(11));
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000817c22); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r820 = c.r64(9) & c.r64(2);
            c.w64(9, r820);
            c.logic(r820, 64);
            let Some(t821) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817c30); };
            c.logic(t821 & 0x4u64, 8);
            if c.cc_ne() { blk = 1166; continue; }
            blk = 770; continue;
        }
        770 => {
            let Some(t822) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000817c3b); };
            c.w32(9, t822);
            let r823 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r823);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 778; continue; }
            blk = 771; continue;
        }
        771 => {
            let Some(t824) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817c47); };
            c.w32(11, t824);
            let r825 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r825);
            let Some(t826) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000817c4e); };
            c.cmp(t826, 0x42u64, 16);
            if c.cc_e() { blk = 773; continue; }
            blk = 772; continue;
        }
        772 => {
            c.w64(9, c.r64(8));
            blk = 819; continue;
        }
        773 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000817c5e); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 775; continue; }
            blk = 774; continue;
        }
        774 => {
            if c.cc_e() { blk = 776; continue; }
            blk = 775; continue;
        }
        775 => {
            c.w64(9, c.r64(8));
            blk = 819; continue;
        }
        776 => {
            let Some(t827) = c.ld64(0x70008151d7u64) else { return RegfileExit::fault(0x7000817c81); };
            c.w64(12, t827);
            let r828 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r828);
            let Some(t829) = c.ld64(0x70008151e4u64) else { return RegfileExit::fault(0x7000817c8b); };
            c.w64(15, t829);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 779; continue; }
            blk = 777; continue;
        }
        777 => {
            c.w64(9, c.r64(8));
            blk = 819; continue;
        }
        778 => {
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 779; continue;
        }
        779 => {
            let Some(t830) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817ca5); };
            c.w32(12, t830);
            let r831 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r831);
            let Some(t832) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000817cac); };
            c.cmp(t832, 0x834u64, 16);
            if c.cc_e() { blk = 781; continue; }
            blk = 780; continue;
        }
        780 => {
            let Some(t833) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817cb6); };
            c.w32(12, t833);
            blk = 782; continue;
        }
        781 => {
            let Some(t834) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000817cbc); };
            c.w32(12, t834);
            blk = 782; continue;
        }
        782 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), c.r64(11), 64);
            if c.cc_be() { blk = 785; continue; }
            blk = 783; continue;
        }
        783 => {
            c.cmp(c.r64(11), c.r64(15), 64);
            if c.cc_b() { blk = 801; continue; }
            blk = 784; continue;
        }
        784 => {
            c.w64(9, c.r64(8));
            blk = 819; continue;
        }
        785 => {
            let Some(t835) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817cdf); };
            c.w32(15, t835);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(11), c.r64(3), 64);
            if c.cc_b() { blk = 799; continue; }
            blk = 786; continue;
        }
        786 => {
            let r836 = c.add(c.r64(3), 0x400u64, 64);
            c.w64(3, r836);
            c.cmp(c.r64(3), c.r64(11), 64);
            if c.cc_be() { blk = 1135; continue; }
            blk = 787; continue;
        }
        787 => {
            c.w64(3, c.r64(11).wrapping_add(0x1));
            c.w64(1, c.r64(3));
            if let Some(sr) = c.shr(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.w64(1, c.r64(1).wrapping_add(c.r64(3)).wrapping_add(0x10));
            c.cmp(c.r64(1), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 1135; continue; }
            blk = 788; continue;
        }
        788 => {
            c.cmp(c.r64(1), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1133; continue; }
            blk = 789; continue;
        }
        789 => {
            let Some(t837) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817d2e); };
            c.w64(3, t837);
            let Some(t838) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000817d35); };
            c.w64(7, t838);
            c.w64(8, c.r64(1).wrapping_mul(4).wrapping_add(0x8));
            c.w64(6, c.r64(8).wrapping_add(c.r64(7)));
            let Some(t839) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000817d44); };
            c.w64(0, t839);
            let Some(t840) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000817d4b); };
            c.cmp(t840, c.r64(6), 64);
            if c.cc_be() { blk = 1131; continue; }
            blk = 790; continue;
        }
        790 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x7000817d54); }
            let r841 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r841);
            blk = 791; continue;
        }
        791 => {
            c.w32(8, c.r64(1).wrapping_add(c.r64(1)));
            let Some(t842) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000817d5f); };
            c.w64(0, t842);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x7000817d66); }
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000817d69); }
            c.w64(8, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_e() { blk = 794; continue; }
            blk = 792; continue;
        }
        792 => {
            let Some(t843) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000817d82); };
            c.w64(15, t843);
            blk = 793; continue;
        }
        793 => {
            let r844 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r844);
            if !c.st32(c.r64(1).wrapping_add(c.r64(7)), c.r32(15)) { return RegfileExit::fault(0x7000817d8d); }
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_ne() { blk = 793; continue; }
            blk = 794; continue;
        }
        794 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 796; continue; }
            blk = 795; continue;
        }
        795 => {
            let r845 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r845);
            let Some(t846) = c.ld32(c.r64(8).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000817da0); };
            c.w32(1, t846);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x7000817da4); }
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 795; continue; }
            blk = 796; continue;
        }
        796 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(7)) { return RegfileExit::fault(0x7000817dae); }
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 798; continue; }
            blk = 797; continue;
        }
        797 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r847 = c.r64(1) & c.r64(2);
            c.w64(1, r847);
            c.logic(r847, 64);
            let Some(t848) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817dc1); };
            c.logic(t848 & 0x4u64, 8);
            if c.cc_ne() { blk = 1163; continue; }
            blk = 798; continue;
        }
        798 => {
            c.w64(9, c.r64(7));
            blk = 799; continue;
        }
        799 => {
            let Some(t849) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817dce); };
            c.w32(1, t849);
            let r850 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r850);
            let Some(t851) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000817dd4); };
            c.cmp(t851, 0x834u64, 16);
            if c.cc_ne() { blk = 801; continue; }
            blk = 800; continue;
        }
        800 => {
            c.w64(1, c.r64(11).wrapping_add(0x1));
            let r852 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r852);
            if !c.st32(c.r64(2).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000817de2); }
            blk = 801; continue;
        }
        801 => {
            let Some(t853) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000817de5); };
            c.w32(1, t853);
            let Some(t854) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000817de9); };
            c.cmp(t854, c.r32(1), 32);
            if c.cc_ne() { blk = 814; continue; }
            blk = 802; continue;
        }
        802 => {
            let Some(t855) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817df6); };
            c.w32(1, t855);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(1));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 818; continue; }
            blk = 803; continue;
        }
        803 => {
            c.cmp(c.r64(7), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 1155; continue; }
            blk = 804; continue;
        }
        804 => {
            let Some(t856) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817e1d); };
            c.w64(8, t856);
            let Some(t857) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000817e24); };
            c.w64(15, t857);
            c.w64(0, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.w64(3, c.r64(0).wrapping_add(c.r64(15)));
            let Some(t858) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000817e33); };
            c.w64(6, t858);
            let Some(t859) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000817e3a); };
            c.cmp(t859, c.r64(3), 64);
            if c.cc_be() { blk = 1152; continue; }
            blk = 805; continue;
        }
        805 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x7000817e43); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            blk = 806; continue;
        }
        806 => {
            let r860 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r860);
            let Some(t861) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000817e4c); };
            c.w64(15, t861);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000817e53); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x7000817e57); }
            c.w64(7, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(1, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_e() { blk = 809; continue; }
            blk = 807; continue;
        }
        807 => {
            let Some(t862) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000817e70); };
            c.w64(12, t862);
            blk = 808; continue;
        }
        808 => {
            let r863 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r863);
            if !c.st32(c.r64(1).wrapping_add(c.r64(8)), c.r32(12)) { return RegfileExit::fault(0x7000817e7b); }
            c.cmp(c.r64(1), c.r64(7), 64);
            if c.cc_ne() { blk = 808; continue; }
            blk = 809; continue;
        }
        809 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 811; continue; }
            blk = 810; continue;
        }
        810 => {
            let r864 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r864);
            let Some(t865) = c.ld32(c.r64(7).wrapping_add(c.r64(9))) else { return RegfileExit::fault(0x7000817e8e); };
            c.w32(1, t865);
            if !c.st32(c.r64(7).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000817e92); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 810; continue; }
            blk = 811; continue;
        }
        811 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000817e9c); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 813; continue; }
            blk = 812; continue;
        }
        812 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r866 = c.r64(1) & c.r64(2);
            c.w64(1, r866);
            c.logic(r866, 64);
            let Some(t867) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817eb0); };
            c.logic(t867 & 0x4u64, 8);
            if c.cc_ne() { blk = 1160; continue; }
            blk = 813; continue;
        }
        813 => {
            c.w64(9, c.r64(8));
            blk = 814; continue;
        }
        814 => {
            let Some(t868) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000817ebd); };
            c.w32(1, t868);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(11), c.r64(1), 64);
            if c.cc_ae() { blk = 1154; continue; }
            blk = 815; continue;
        }
        815 => {
            c.w64(1, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            let Some(t869) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817ed8); };
            c.w64(0, t869);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x7000817edc); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 817; continue; }
            blk = 816; continue;
        }
        816 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r870 = c.r64(7) & c.r64(9);
            c.w64(7, r870);
            c.logic(r870, 64);
            let Some(t871) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000817eee); };
            c.logic(t871 & 0x4u64, 8);
            if c.cc_ne() { blk = 1157; continue; }
            blk = 817; continue;
        }
        817 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000817efb); }
            return c.ret(0x8, 0x7000817efc);
        }
        818 => {
            let Some(t872) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000817eff); };
            c.w64(1, t872);
            let Some(t873) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817f03); };
            c.w64(0, t873);
            let Some(t874) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000817f07); };
            c.w64(9, t874);
            let Some(t875) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000817f0b); };
            c.w64(6, t875);
            blk = 819; continue;
        }
        819 => {
            let Some(t876) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000817f0f); };
            c.w64(5, t876);
            let Some(t877) = c.ld64(c.r64(13).wrapping_add(0x2010)) else { return RegfileExit::fault(0x7000817f13); };
            c.w64(3, t877);
            let Some(t878) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x7000817f1a); };
            c.w64(10, t878);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x7000817f1f); }
            let Some(t879) = c.ld64(c.r64(4).wrapping_add(0x78)) else { return RegfileExit::fault(0x7000817f24); };
            c.w64(10, t879);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x7000817f29); }
            if !c.st64(c.r64(4).wrapping_add(0x78), c.r64(2)) { return RegfileExit::fault(0x7000817f2e); }
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(1)) { return RegfileExit::fault(0x7000817f33); }
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(0)) { return RegfileExit::fault(0x7000817f38); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(9)) { return RegfileExit::fault(0x7000817f3d); }
            let Some(t880) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x7000817f42); };
            c.w64(10, t880);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x7000817f47); }
            c.w32(0, 0x6u64);
            let r881 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r881);
            return RegfileExit::jump(0x700086ed74);
        }
        820 => {
            return RegfileExit::fault(0x7000817f5a);
        }
        821 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000817f5b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000817f5f); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000817f63); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x7000817f67); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000817f6b); }
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x7000817f77, 0x7000817f72);
        }
        822 => {
            c.w64(1, c.r64(0));
            let Some(t882) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000817f7a); };
            c.w64(2, t882);
            let Some(t883) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000817f7e); };
            c.w64(9, t883);
            let Some(t884) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000817f82); };
            c.w64(12, t884);
            let Some(t885) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000817f86); };
            c.w64(8, t885);
            let Some(t886) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000817f8a); };
            c.w64(7, t886);
            let Some(t887) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000817f8e); };
            c.w64(15, t887);
            blk = 51; continue;
        }
        823 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t888) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000817f99); };
            c.w64(3, t888);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000817fa7, 0x7000817fa2);
        }
        824 => {
            return RegfileExit::fault(0x7000817fa7);
        }
        825 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000817fa8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000817fac); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_l() { blk = 841; continue; }
            blk = 826; continue;
        }
        826 => {
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 828; continue; }
            blk = 827; continue;
        }
        827 => {
            c.w32(7, c.r64(9).wrapping_add(c.r64(9)));
            blk = 837; continue;
        }
        828 => {
            let Some(t889) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000817fcc); };
            c.w64(7, t889);
            let Some(t890) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000817fd3); };
            c.w64(8, t890);
            c.w64(15, c.r64(8).wrapping_add(0xc));
            let Some(t891) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000817fda); };
            c.w64(3, t891);
            let Some(t892) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000817fe1); };
            c.cmp(t892, c.r64(15), 64);
            if c.cc_a() { blk = 831; continue; }
            blk = 829; continue;
        }
        829 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x7000817ff3, 0x7000817fee);
        }
        830 => {
            c.w64(7, c.r64(0));
            let Some(t893) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000817ff6); };
            c.w64(2, t893);
            let Some(t894) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000817ffa); };
            c.w64(1, t894);
            let Some(t895) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000817ffe); };
            c.w64(0, t895);
            let Some(t896) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818002); };
            c.w64(11, t896);
            let Some(t897) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818006); };
            c.w64(6, t897);
            let Some(t898) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081800a); };
            c.w64(9, t898);
            let Some(t899) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081800e); };
            c.w64(12, t899);
            blk = 832; continue;
        }
        831 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x7000818014); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 832; continue;
        }
        832 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 836; continue; }
            blk = 833; continue;
        }
        833 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 835; continue; }
            blk = 834; continue;
        }
        834 => {
            let r900 = c.r64(10) | 0x1u64;
            c.w64(10, r900);
            c.logic(r900, 64);
            blk = 835; continue;
        }
        835 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 836; continue;
        }
        836 => {
            let Some(t901) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000818042); };
            c.w64(8, t901);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000818049); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081804d); }
            blk = 837; continue;
        }
        837 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000818052); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000818053); }
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t902) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000818056); };
            c.w64(3, t902);
            c.w32(0, 0x2u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x700081806a, 0x7000818065);
        }
        838 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 840; continue; }
            blk = 839; continue;
        }
        839 => {
            c.w64(8, c.r64(0));
            let Some(t903) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818071); };
            c.w64(2, t903);
            let Some(t904) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818075); };
            c.w64(9, t904);
            let Some(t905) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818079); };
            c.w64(12, t905);
            blk = 59; continue;
        }
        840 => {
            let Some(t906) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818082); };
            c.w64(2, t906);
            let Some(t907) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818086); };
            c.w64(1, t907);
            let Some(t908) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081808a); };
            c.w64(0, t908);
            let Some(t909) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081808e); };
            c.w64(9, t909);
            let Some(t910) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818092); };
            c.w64(6, t910);
            blk = 819; continue;
        }
        841 => {
            c.w64(9, c.r64(11));
            blk = 819; continue;
        }
        842 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008180a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008180a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008180ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008180af); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x70008180b3); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x70008180bf, 0x70008180ba);
        }
        843 => {
            c.w64(11, c.r64(0));
            let Some(t911) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008180c2); };
            c.w64(2, t911);
            let Some(t912) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008180c6); };
            c.w64(9, t912);
            let Some(t913) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008180ca); };
            c.w64(8, t913);
            let Some(t914) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008180ce); };
            c.w64(1, t914);
            let Some(t915) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008180d2); };
            c.w64(12, t915);
            let Some(t916) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008180d6); };
            c.w64(7, t916);
            blk = 66; continue;
        }
        844 => {
            return RegfileExit::fault(0x70008180df);
        }
        845 => {
            let Some(t917) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008180e0); };
            c.w64(3, t917);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008180f0, 0x70008180eb);
        }
        846 => {
            return RegfileExit::fault(0x70008180f0);
        }
        847 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t918) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008180f3); };
            c.w64(1, t918);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x7000818104, 0x70008180ff);
        }
        848 => {
            return RegfileExit::fault(0x7000818104);
        }
        849 => {
            let r919 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r919);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000818108); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081810d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700081810f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000818113); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000818117); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081811b); }
            let Some(t920) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081811f); };
            c.w64(3, t920);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x7000818133, 0x700081812e);
        }
        850 => {
            c.w64(15, c.r64(0));
            let Some(t921) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818136); };
            c.w64(1, t921);
            let Some(t922) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081813a); };
            c.w64(11, t922);
            let Some(t923) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081813e); };
            c.w64(6, t923);
            let Some(t924) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818142); };
            c.w64(7, t924);
            let Some(t925) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818146); };
            c.w64(8, t925);
            let Some(t926) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081814a); };
            c.w64(9, t926);
            let Some(t927) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081814e); };
            c.w64(12, t927);
            blk = 94; continue;
        }
        851 => {
            let Some(t928) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818157); };
            c.w64(3, t928);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818167, 0x7000818162);
        }
        852 => {
            return RegfileExit::fault(0x7000818167);
        }
        853 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t929) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081816a); };
            c.w64(3, t929);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(6));
            return c.call(0x700086ed74, 0x700081817b, 0x7000818176);
        }
        854 => {
            return RegfileExit::fault(0x700081817b);
        }
        855 => {
            let r930 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r930);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081817e); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000818182); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000818184); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000818188); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700081818d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000818191); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x7000818195); }
            let Some(t931) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000818199); };
            c.w64(3, t931);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x70008181ad, 0x70008181a8);
        }
        856 => {
            c.w64(1, c.r64(0));
            let Some(t932) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008181b0); };
            c.w64(2, t932);
            let Some(t933) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008181b4); };
            c.w64(0, t933);
            let Some(t934) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008181b8); };
            c.w64(9, t934);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008181bc); }
            let Some(t935) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008181c1); };
            c.w64(8, t935);
            let Some(t936) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008181c5); };
            c.w64(12, t936);
            let Some(t937) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008181c9); };
            c.w64(11, t937);
            blk = 133; continue;
        }
        857 => {
            let Some(t938) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008181d2); };
            c.w64(3, t938);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008181e2, 0x70008181dd);
        }
        858 => {
            return RegfileExit::fault(0x70008181e2);
        }
        859 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008181e3); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008181e7); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_l() { blk = 875; continue; }
            blk = 860; continue;
        }
        860 => {
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 862; continue; }
            blk = 861; continue;
        }
        861 => {
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            blk = 871; continue;
        }
        862 => {
            let Some(t939) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000818208); };
            c.w64(8, t939);
            let Some(t940) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700081820f); };
            c.w64(11, t940);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t941) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000818216); };
            c.w64(15, t941);
            let Some(t942) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081821d); };
            c.cmp(t942, c.r64(12), 64);
            if c.cc_a() { blk = 865; continue; }
            blk = 863; continue;
        }
        863 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x700081822f, 0x700081822a);
        }
        864 => {
            c.w64(8, c.r64(0));
            let Some(t943) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818232); };
            c.w64(2, t943);
            let Some(t944) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818236); };
            c.w64(1, t944);
            let Some(t945) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081823a); };
            c.w64(0, t945);
            let Some(t946) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081823e); };
            c.w64(7, t946);
            let Some(t947) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818242); };
            c.w64(6, t947);
            let Some(t948) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818246); };
            c.w64(9, t948);
            blk = 866; continue;
        }
        865 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700081824c); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 866; continue;
        }
        866 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 870; continue; }
            blk = 867; continue;
        }
        867 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 869; continue; }
            blk = 868; continue;
        }
        868 => {
            let r949 = c.r64(10) | 0x1u64;
            c.w64(10, r949);
            c.logic(r949, 64);
            blk = 869; continue;
        }
        869 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 870; continue;
        }
        870 => {
            let Some(t950) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081827a); };
            c.w64(11, t950);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000818281); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000818285); }
            blk = 871; continue;
        }
        871 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700081828b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081828c); }
            let Some(t951) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700081828e); };
            c.w64(3, t951);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008182a1, 0x700081829c);
        }
        872 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 874; continue; }
            blk = 873; continue;
        }
        873 => {
            c.w64(8, c.r64(0));
            let Some(t952) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008182a8); };
            c.w64(2, t952);
            let Some(t953) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008182ac); };
            c.w64(0, t953);
            let Some(t954) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008182b0); };
            c.w64(9, t954);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008182b4); }
            blk = 144; continue;
        }
        874 => {
            let Some(t955) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008182be); };
            c.w64(2, t955);
            let Some(t956) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008182c2); };
            c.w64(1, t956);
            let Some(t957) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008182c6); };
            c.w64(0, t957);
            let Some(t958) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008182ca); };
            c.w64(9, t958);
            let Some(t959) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008182ce); };
            c.w64(6, t959);
            blk = 819; continue;
        }
        875 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        876 => {
            return RegfileExit::fault(0x70008182df);
        }
        877 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t960) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008182e2); };
            c.w64(1, t960);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x70008182f3, 0x70008182ee);
        }
        878 => {
            return RegfileExit::fault(0x70008182f3);
        }
        879 => {
            c.w32(7, c.r64(15).wrapping_add(c.r64(15)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008182f8); }
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008182fb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008182fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000818300); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000818304); }
            let Some(t961) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000818308); };
            c.w64(3, t961);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x700081831c, 0x7000818317);
        }
        880 => {
            c.w64(7, c.r64(0));
            let Some(t962) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081831f); };
            c.w64(1, t962);
            let Some(t963) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818323); };
            c.w64(0, t963);
            let Some(t964) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818327); };
            c.w64(11, t964);
            let Some(t965) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081832b); };
            c.w64(6, t965);
            let Some(t966) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081832f); };
            c.w64(8, t966);
            let Some(t967) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818333); };
            c.w64(9, t967);
            let Some(t968) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818337); };
            c.w64(12, t968);
            blk = 165; continue;
        }
        881 => {
            let Some(t969) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818340); };
            c.w64(3, t969);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818350, 0x700081834b);
        }
        882 => {
            return RegfileExit::fault(0x7000818350);
        }
        883 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t970) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000818353); };
            c.w64(1, t970);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x7000818364, 0x700081835f);
        }
        884 => {
            return RegfileExit::fault(0x7000818364);
        }
        885 => {
            let r971 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r971);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000818367); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700081836a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700081836b); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x700081836f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000818374); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000818378); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x700081837c); }
            let Some(t972) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000818380); };
            c.w64(1, t972);
            c.w64(6, c.r64(3));
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x7000818397, 0x7000818392);
        }
        886 => {
            c.w64(1, c.r64(0));
            let Some(t973) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081839a); };
            c.w64(9, t973);
            let Some(t974) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081839e); };
            c.w64(0, t974);
            let Some(t975) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008183a2); };
            c.w64(11, t975);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x70008183a6); }
            let Some(t976) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008183ab); };
            c.w64(8, t976);
            let Some(t977) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008183af); };
            c.w64(15, t977);
            let Some(t978) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008183b3); };
            c.w64(12, t978);
            blk = 206; continue;
        }
        887 => {
            let Some(t979) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008183bc); };
            c.w64(3, t979);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008183cc, 0x70008183c7);
        }
        888 => {
            return RegfileExit::fault(0x70008183cc);
        }
        889 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008183cd); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008183d1); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 905; continue; }
            blk = 890; continue;
        }
        890 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 892; continue; }
            blk = 891; continue;
        }
        891 => {
            c.w32(8, c.r64(11).wrapping_add(c.r64(11)));
            blk = 901; continue;
        }
        892 => {
            let Some(t980) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008183f2); };
            c.w64(8, t980);
            let Some(t981) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008183f9); };
            c.w64(12, t981);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t982) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000818401); };
            c.w64(3, t982);
            let Some(t983) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000818408); };
            c.cmp(t983, c.r64(15), 64);
            if c.cc_a() { blk = 895; continue; }
            blk = 893; continue;
        }
        893 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000818417, 0x7000818412);
        }
        894 => {
            c.w64(8, c.r64(0));
            let Some(t984) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081841a); };
            c.w64(9, t984);
            let Some(t985) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081841e); };
            c.w64(1, t985);
            let Some(t986) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818422); };
            c.w64(0, t986);
            let Some(t987) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818426); };
            c.w64(7, t987);
            let Some(t988) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081842a); };
            c.w64(6, t988);
            let Some(t989) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081842e); };
            c.w64(11, t989);
            blk = 896; continue;
        }
        895 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x7000818434); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 896; continue;
        }
        896 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 900; continue; }
            blk = 897; continue;
        }
        897 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 899; continue; }
            blk = 898; continue;
        }
        898 => {
            let r990 = c.r64(10) | 0x1u64;
            c.w64(10, r990);
            c.logic(r990, 64);
            blk = 899; continue;
        }
        899 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 900; continue;
        }
        900 => {
            let Some(t991) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000818463); };
            c.w64(12, t991);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x700081846a); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081846e); }
            blk = 901; continue;
        }
        901 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000818474); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000818476); }
            let Some(t992) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000818478); };
            c.w64(3, t992);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081848b, 0x7000818486);
        }
        902 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 904; continue; }
            blk = 903; continue;
        }
        903 => {
            c.w64(8, c.r64(0));
            let Some(t993) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818492); };
            c.w64(9, t993);
            let Some(t994) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818496); };
            c.w64(0, t994);
            let Some(t995) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081849a); };
            c.w64(11, t995);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700081849e); }
            blk = 217; continue;
        }
        904 => {
            let Some(t996) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008184a8); };
            c.w64(2, t996);
            let Some(t997) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008184ac); };
            c.w64(1, t997);
            let Some(t998) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008184b0); };
            c.w64(0, t998);
            let Some(t999) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008184b4); };
            c.w64(9, t999);
            let Some(t1000) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008184b8); };
            c.w64(6, t1000);
            blk = 819; continue;
        }
        905 => {
            c.w64(2, c.r64(9));
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        906 => {
            return RegfileExit::fault(0x70008184cc);
        }
        907 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008184cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008184d1); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008184d5); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febf4, 0x70008184e1, 0x70008184dc);
        }
        908 => {
            c.w64(1, c.r64(0));
            let Some(t1001) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008184e4); };
            c.w64(2, t1001);
            let Some(t1002) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008184e8); };
            c.w64(0, t1002);
            let Some(t1003) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008184ec); };
            c.w64(6, t1003);
            let Some(t1004) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008184f0); };
            c.w64(8, t1004);
            let Some(t1005) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008184f4); };
            c.w64(7, t1005);
            let Some(t1006) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008184f8); };
            c.w64(15, t1006);
            blk = 249; continue;
        }
        909 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t1007) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818503); };
            c.w64(3, t1007);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x7000818514, 0x700081850f);
        }
        910 => {
            return RegfileExit::fault(0x7000818514);
        }
        911 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 819; continue; }
            blk = 912; continue;
        }
        912 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 914; continue; }
            blk = 913; continue;
        }
        913 => {
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            blk = 923; continue;
        }
        914 => {
            let Some(t1008) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000818531); };
            c.w64(7, t1008);
            let Some(t1009) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000818538); };
            c.w64(8, t1009);
            c.w64(15, c.r64(8).wrapping_add(0xc));
            let Some(t1010) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081853f); };
            c.w64(3, t1010);
            let Some(t1011) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000818546); };
            c.cmp(t1011, c.r64(15), 64);
            if c.cc_a() { blk = 917; continue; }
            blk = 915; continue;
        }
        915 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x7000818558, 0x7000818553);
        }
        916 => {
            c.w64(7, c.r64(0));
            let Some(t1012) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081855b); };
            c.w64(2, t1012);
            let Some(t1013) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081855f); };
            c.w64(1, t1013);
            let Some(t1014) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818563); };
            c.w64(0, t1014);
            let Some(t1015) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818567); };
            c.w64(9, t1015);
            let Some(t1016) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081856b); };
            c.w64(6, t1016);
            let Some(t1017) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081856f); };
            c.w64(11, t1017);
            let Some(t1018) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818573); };
            c.w64(12, t1018);
            blk = 918; continue;
        }
        917 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x7000818579); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 918; continue;
        }
        918 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 922; continue; }
            blk = 919; continue;
        }
        919 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 921; continue; }
            blk = 920; continue;
        }
        920 => {
            let r1019 = c.r64(10) | 0x1u64;
            c.w64(10, r1019);
            c.logic(r1019, 64);
            blk = 921; continue;
        }
        921 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 922; continue;
        }
        922 => {
            let Some(t1020) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008185a7); };
            c.w64(8, t1020);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008185ae); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008185b2); }
            blk = 923; continue;
        }
        923 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008185b7); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008185b8); }
            let Some(t1021) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x70008185b9); };
            c.w64(3, t1021);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008185cc, 0x70008185c7);
        }
        924 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 926; continue; }
            blk = 925; continue;
        }
        925 => {
            c.w64(8, c.r64(0));
            let Some(t1022) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008185d3); };
            c.w64(2, t1022);
            let Some(t1023) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008185d7); };
            c.w64(0, t1023);
            let Some(t1024) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008185db); };
            c.w64(6, t1024);
            blk = 257; continue;
        }
        926 => {
            let Some(t1025) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008185e4); };
            c.w64(2, t1025);
            let Some(t1026) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008185e8); };
            c.w64(1, t1026);
            let Some(t1027) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008185ec); };
            c.w64(0, t1027);
            let Some(t1028) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008185f0); };
            c.w64(9, t1028);
            let Some(t1029) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008185f4); };
            c.w64(6, t1029);
            blk = 819; continue;
        }
        927 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008185fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000818601); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000818605); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000818611, 0x700081860c);
        }
        928 => {
            c.w64(9, c.r64(0));
            let Some(t1030) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818614); };
            c.w64(2, t1030);
            let Some(t1031) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818618); };
            c.w64(0, t1031);
            let Some(t1032) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081861c); };
            c.w64(11, t1032);
            let Some(t1033) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000818620); };
            c.w64(8, t1033);
            let Some(t1034) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818624); };
            c.w64(1, t1034);
            let Some(t1035) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818628); };
            c.w64(7, t1035);
            blk = 265; continue;
        }
        929 => {
            return RegfileExit::fault(0x7000818631);
        }
        930 => {
            let Some(t1036) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818632); };
            c.w64(3, t1036);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818642, 0x700081863d);
        }
        931 => {
            return RegfileExit::fault(0x7000818642);
        }
        932 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000818643); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000818647); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700081864b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700081864f); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febf4, 0x700081865b, 0x7000818656);
        }
        933 => {
            c.w64(1, c.r64(0));
            let Some(t1037) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081865e); };
            c.w64(2, t1037);
            let Some(t1038) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818662); };
            c.w64(6, t1038);
            let Some(t1039) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818666); };
            c.w64(11, t1039);
            let Some(t1040) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081866a); };
            c.w64(8, t1040);
            let Some(t1041) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081866e); };
            c.w64(7, t1041);
            let Some(t1042) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818672); };
            c.w64(15, t1042);
            blk = 306; continue;
        }
        934 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t1043) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081867d); };
            c.w64(3, t1043);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x700081868e, 0x7000818689);
        }
        935 => {
            return RegfileExit::fault(0x700081868e);
        }
        936 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700081868f); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 819; continue; }
            blk = 937; continue;
        }
        937 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 939; continue; }
            blk = 938; continue;
        }
        938 => {
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            blk = 948; continue;
        }
        939 => {
            let Some(t1044) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008186af); };
            c.w64(7, t1044);
            let Some(t1045) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008186b6); };
            c.w64(8, t1045);
            c.w64(15, c.r64(8).wrapping_add(0xc));
            let Some(t1046) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008186bd); };
            c.w64(3, t1046);
            let Some(t1047) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008186c4); };
            c.cmp(t1047, c.r64(15), 64);
            if c.cc_a() { blk = 942; continue; }
            blk = 940; continue;
        }
        940 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x70008186d6, 0x70008186d1);
        }
        941 => {
            c.w64(7, c.r64(0));
            let Some(t1048) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008186d9); };
            c.w64(2, t1048);
            let Some(t1049) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008186dd); };
            c.w64(1, t1049);
            let Some(t1050) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008186e1); };
            c.w64(0, t1050);
            let Some(t1051) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008186e5); };
            c.w64(9, t1051);
            let Some(t1052) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008186e9); };
            c.w64(6, t1052);
            let Some(t1053) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008186ed); };
            c.w64(11, t1053);
            let Some(t1054) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008186f1); };
            c.w64(12, t1054);
            blk = 943; continue;
        }
        942 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x70008186f7); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 943; continue;
        }
        943 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 947; continue; }
            blk = 944; continue;
        }
        944 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 946; continue; }
            blk = 945; continue;
        }
        945 => {
            let r1055 = c.r64(10) | 0x1u64;
            c.w64(10, r1055);
            c.logic(r1055, 64);
            blk = 946; continue;
        }
        946 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 947; continue;
        }
        947 => {
            let Some(t1056) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000818725); };
            c.w64(8, t1056);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x700081872c); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000818730); }
            blk = 948; continue;
        }
        948 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000818735); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000818736); }
            let Some(t1057) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000818737); };
            c.w64(3, t1057);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081874a, 0x7000818745);
        }
        949 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 951; continue; }
            blk = 950; continue;
        }
        950 => {
            c.w64(8, c.r64(0));
            let Some(t1058) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818751); };
            c.w64(2, t1058);
            let Some(t1059) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818755); };
            c.w64(6, t1059);
            let Some(t1060) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818759); };
            c.w64(11, t1060);
            blk = 314; continue;
        }
        951 => {
            let Some(t1061) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818762); };
            c.w64(2, t1061);
            let Some(t1062) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818766); };
            c.w64(1, t1062);
            let Some(t1063) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081876a); };
            c.w64(0, t1063);
            let Some(t1064) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081876e); };
            c.w64(9, t1064);
            let Some(t1065) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818772); };
            c.w64(6, t1065);
            blk = 819; continue;
        }
        952 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700081877b); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700081877f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000818783); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000818787); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000818793, 0x700081878e);
        }
        953 => {
            c.w64(9, c.r64(0));
            let Some(t1066) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818796); };
            c.w64(2, t1066);
            let Some(t1067) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081879a); };
            c.w64(11, t1067);
            let Some(t1068) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081879e); };
            c.w64(8, t1068);
            let Some(t1069) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008187a2); };
            c.w64(1, t1069);
            let Some(t1070) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008187a6); };
            c.w64(7, t1070);
            blk = 321; continue;
        }
        954 => {
            return RegfileExit::fault(0x70008187af);
        }
        955 => {
            let Some(t1071) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008187b0); };
            c.w64(3, t1071);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008187c0, 0x70008187bb);
        }
        956 => {
            return RegfileExit::fault(0x70008187c0);
        }
        957 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1072) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x70008187c3); };
            c.w64(1, t1072);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x70008187d4, 0x70008187cf);
        }
        958 => {
            return RegfileExit::fault(0x70008187d4);
        }
        959 => {
            let r1073 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r1073);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008187d8); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008187dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008187df); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008187e3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008187e7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008187eb); }
            let Some(t1074) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x70008187ef); };
            c.w64(3, t1074);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x7000818803, 0x70008187fe);
        }
        960 => {
            c.w64(15, c.r64(0));
            let Some(t1075) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818806); };
            c.w64(1, t1075);
            let Some(t1076) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081880a); };
            c.w64(11, t1076);
            let Some(t1077) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081880e); };
            c.w64(6, t1077);
            let Some(t1078) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818812); };
            c.w64(7, t1078);
            let Some(t1079) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818816); };
            c.w64(8, t1079);
            let Some(t1080) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081881a); };
            c.w64(9, t1080);
            let Some(t1081) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081881e); };
            c.w64(12, t1081);
            blk = 351; continue;
        }
        961 => {
            let Some(t1082) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818827); };
            c.w64(3, t1082);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818837, 0x7000818832);
        }
        962 => {
            return RegfileExit::fault(0x7000818837);
        }
        963 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1083) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081883a); };
            c.w64(3, t1083);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(6));
            return c.call(0x700086ed74, 0x700081884b, 0x7000818846);
        }
        964 => {
            return RegfileExit::fault(0x700081884b);
        }
        965 => {
            let r1084 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r1084);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081884e); }
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000818852); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000818854); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000818858); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700081885d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000818861); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x7000818865); }
            let Some(t1085) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000818869); };
            c.w64(3, t1085);
            c.w32(0, 0x2u64);
            c.w64(6, c.r64(15));
            return c.call(0x700086ed74, 0x700081887d, 0x7000818878);
        }
        966 => {
            c.w64(1, c.r64(0));
            let Some(t1086) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818880); };
            c.w64(2, t1086);
            let Some(t1087) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818884); };
            c.w64(0, t1087);
            let Some(t1088) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818888); };
            c.w64(9, t1088);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x700081888c); }
            let Some(t1089) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818891); };
            c.w64(8, t1089);
            let Some(t1090) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818895); };
            c.w64(12, t1090);
            let Some(t1091) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000818899); };
            c.w64(11, t1091);
            blk = 390; continue;
        }
        967 => {
            let Some(t1092) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008188a2); };
            c.w64(3, t1092);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008188b2, 0x70008188ad);
        }
        968 => {
            return RegfileExit::fault(0x70008188b2);
        }
        969 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x70008188b3); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x70008188b7); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_l() { blk = 985; continue; }
            blk = 970; continue;
        }
        970 => {
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 972; continue; }
            blk = 971; continue;
        }
        971 => {
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            blk = 981; continue;
        }
        972 => {
            let Some(t1093) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008188d8); };
            c.w64(8, t1093);
            let Some(t1094) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008188df); };
            c.w64(11, t1094);
            c.w64(12, c.r64(11).wrapping_add(0xc));
            let Some(t1095) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008188e6); };
            c.w64(15, t1095);
            let Some(t1096) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x70008188ed); };
            c.cmp(t1096, c.r64(12), 64);
            if c.cc_a() { blk = 975; continue; }
            blk = 973; continue;
        }
        973 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x70008188ff, 0x70008188fa);
        }
        974 => {
            c.w64(8, c.r64(0));
            let Some(t1097) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818902); };
            c.w64(2, t1097);
            let Some(t1098) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818906); };
            c.w64(1, t1098);
            let Some(t1099) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081890a); };
            c.w64(0, t1099);
            let Some(t1100) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081890e); };
            c.w64(7, t1100);
            let Some(t1101) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818912); };
            c.w64(6, t1101);
            let Some(t1102) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818916); };
            c.w64(9, t1102);
            blk = 976; continue;
        }
        975 => {
            if !c.st64(c.r64(8), c.r64(12)) { return RegfileExit::fault(0x700081891c); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            blk = 976; continue;
        }
        976 => {
            let Some(t1103) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000818923); };
            c.w64(11, t1103);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081892a); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(9) as i64));
            c.logic(c.r64(9) & c.r64(9), 64);
            if c.cc_ns() { blk = 980; continue; }
            blk = 977; continue;
        }
        977 => {
            c.w64(10, c.r64(9));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 979; continue; }
            blk = 978; continue;
        }
        978 => {
            let r1104 = c.r64(10) | 0x1u64;
            c.w64(10, r1104);
            c.logic(r1104, 64);
            blk = 979; continue;
        }
        979 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 980; continue;
        }
        980 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000818955); }
            blk = 981; continue;
        }
        981 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700081895b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081895c); }
            let Some(t1105) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700081895e); };
            c.w64(3, t1105);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818971, 0x700081896c);
        }
        982 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 984; continue; }
            blk = 983; continue;
        }
        983 => {
            c.w64(8, c.r64(0));
            let Some(t1106) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818978); };
            c.w64(2, t1106);
            let Some(t1107) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081897c); };
            c.w64(0, t1107);
            let Some(t1108) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818980); };
            c.w64(9, t1108);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000818984); }
            blk = 401; continue;
        }
        984 => {
            let Some(t1109) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081898e); };
            c.w64(2, t1109);
            let Some(t1110) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818992); };
            c.w64(1, t1110);
            let Some(t1111) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818996); };
            c.w64(0, t1111);
            let Some(t1112) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081899a); };
            c.w64(9, t1112);
            let Some(t1113) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081899e); };
            c.w64(6, t1113);
            blk = 819; continue;
        }
        985 => {
            c.w64(9, c.r64(7));
            blk = 819; continue;
        }
        986 => {
            return RegfileExit::fault(0x70008189af);
        }
        987 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x70008189b0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x70008189b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008189b8); }
            c.w64(2, c.r64(9));
            return c.call(0x70007febf4, 0x70008189c4, 0x70008189bf);
        }
        988 => {
            c.w64(1, c.r64(0));
            let Some(t1114) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008189c7); };
            c.w64(2, t1114);
            let Some(t1115) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008189cb); };
            c.w64(0, t1115);
            let Some(t1116) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008189cf); };
            c.w64(6, t1116);
            let Some(t1117) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008189d3); };
            c.w64(8, t1117);
            let Some(t1118) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008189d7); };
            c.w64(7, t1118);
            let Some(t1119) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008189db); };
            c.w64(15, t1119);
            blk = 433; continue;
        }
        989 => {
            c.w32(1, 0);
            c.logic(0, 32);
            let Some(t1120) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008189e6); };
            c.w64(3, t1120);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x70008189f7, 0x70008189f2);
        }
        990 => {
            return RegfileExit::fault(0x70008189f7);
        }
        991 => {
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 819; continue; }
            blk = 992; continue;
        }
        992 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 994; continue; }
            blk = 993; continue;
        }
        993 => {
            c.w32(7, c.r64(11).wrapping_add(c.r64(11)));
            blk = 1003; continue;
        }
        994 => {
            let Some(t1121) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000818a14); };
            c.w64(7, t1121);
            let Some(t1122) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x7000818a1b); };
            c.w64(8, t1122);
            c.w64(15, c.r64(8).wrapping_add(0xc));
            let Some(t1123) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000818a22); };
            c.w64(3, t1123);
            let Some(t1124) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000818a29); };
            c.cmp(t1124, c.r64(15), 64);
            if c.cc_a() { blk = 997; continue; }
            blk = 995; continue;
        }
        995 => {
            c.w32(2, 0xcu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febf4, 0x7000818a3b, 0x7000818a36);
        }
        996 => {
            c.w64(7, c.r64(0));
            let Some(t1125) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818a3e); };
            c.w64(2, t1125);
            let Some(t1126) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818a42); };
            c.w64(1, t1126);
            let Some(t1127) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818a46); };
            c.w64(0, t1127);
            let Some(t1128) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818a4a); };
            c.w64(9, t1128);
            let Some(t1129) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818a4e); };
            c.w64(6, t1129);
            let Some(t1130) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818a52); };
            c.w64(11, t1130);
            let Some(t1131) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818a56); };
            c.w64(12, t1131);
            blk = 998; continue;
        }
        997 => {
            if !c.st64(c.r64(7), c.r64(15)) { return RegfileExit::fault(0x7000818a5c); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 998; continue;
        }
        998 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 1002; continue; }
            blk = 999; continue;
        }
        999 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 1001; continue; }
            blk = 1000; continue;
        }
        1000 => {
            let r1132 = c.r64(10) | 0x1u64;
            c.w64(10, r1132);
            c.logic(r1132, 64);
            blk = 1001; continue;
        }
        1001 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 1002; continue;
        }
        1002 => {
            let Some(t1133) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000818a8a); };
            c.w64(8, t1133);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000818a91); }
            if !c.movsd_mr(c.r64(7).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000818a95); }
            blk = 1003; continue;
        }
        1003 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000818a9a); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000818a9b); }
            let Some(t1134) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000818a9c); };
            c.w64(3, t1134);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818aaf, 0x7000818aaa);
        }
        1004 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1006; continue; }
            blk = 1005; continue;
        }
        1005 => {
            c.w64(8, c.r64(0));
            let Some(t1135) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818ab6); };
            c.w64(2, t1135);
            let Some(t1136) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818aba); };
            c.w64(0, t1136);
            let Some(t1137) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818abe); };
            c.w64(6, t1137);
            blk = 441; continue;
        }
        1006 => {
            let Some(t1138) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818ac7); };
            c.w64(2, t1138);
            let Some(t1139) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818acb); };
            c.w64(1, t1139);
            let Some(t1140) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818acf); };
            c.w64(0, t1140);
            let Some(t1141) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818ad3); };
            c.w64(9, t1141);
            let Some(t1142) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818ad7); };
            c.w64(6, t1142);
            blk = 819; continue;
        }
        1007 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000818ae0); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000818ae4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000818ae8); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febf4, 0x7000818af4, 0x7000818aef);
        }
        1008 => {
            c.w64(9, c.r64(0));
            let Some(t1143) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818af7); };
            c.w64(2, t1143);
            let Some(t1144) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818afb); };
            c.w64(0, t1144);
            let Some(t1145) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818aff); };
            c.w64(11, t1145);
            let Some(t1146) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000818b03); };
            c.w64(8, t1146);
            let Some(t1147) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818b07); };
            c.w64(1, t1147);
            let Some(t1148) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818b0b); };
            c.w64(7, t1148);
            blk = 449; continue;
        }
        1009 => {
            return RegfileExit::fault(0x7000818b14);
        }
        1010 => {
            let Some(t1149) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818b15); };
            c.w64(3, t1149);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818b25, 0x7000818b20);
        }
        1011 => {
            return RegfileExit::fault(0x7000818b25);
        }
        1012 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1150) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x7000818b28); };
            c.w64(1, t1150);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x7000818b39, 0x7000818b34);
        }
        1013 => {
            return RegfileExit::fault(0x7000818b39);
        }
        1014 => {
            let r1151 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r1151);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000818b3c); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000818b3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000818b40); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000818b44); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000818b49); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000818b4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(12)) { return RegfileExit::fault(0x7000818b51); }
            let Some(t1152) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x7000818b55); };
            c.w64(1, t1152);
            c.w64(6, c.r64(3));
            c.w32(0, 0x2u64);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x7000818b6c, 0x7000818b67);
        }
        1015 => {
            c.w64(1, c.r64(0));
            let Some(t1153) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818b6f); };
            c.w64(2, t1153);
            let Some(t1154) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818b73); };
            c.w64(0, t1154);
            let Some(t1155) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818b77); };
            c.w64(11, t1155);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000818b7b); }
            let Some(t1156) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818b80); };
            c.w64(8, t1156);
            let Some(t1157) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818b84); };
            c.w64(15, t1157);
            let Some(t1158) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000818b88); };
            c.w64(12, t1158);
            blk = 495; continue;
        }
        1016 => {
            let Some(t1159) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818b91); };
            c.w64(3, t1159);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818ba1, 0x7000818b9c);
        }
        1017 => {
            return RegfileExit::fault(0x7000818ba1);
        }
        1018 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000818ba2); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x48), 0) { return RegfileExit::fault(0x7000818ba6); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 819; continue; }
            blk = 1019; continue;
        }
        1019 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 1021; continue; }
            blk = 1020; continue;
        }
        1020 => {
            c.w32(8, c.r64(11).wrapping_add(c.r64(11)));
            blk = 1030; continue;
        }
        1021 => {
            let Some(t1160) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000818bc7); };
            c.w64(8, t1160);
            let Some(t1161) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000818bce); };
            c.w64(12, t1161);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t1162) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000818bd6); };
            c.w64(3, t1162);
            let Some(t1163) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000818bdd); };
            c.cmp(t1163, c.r64(15), 64);
            if c.cc_a() { blk = 1024; continue; }
            blk = 1022; continue;
        }
        1022 => {
            c.w32(2, 0xcu64);
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000818bef, 0x7000818bea);
        }
        1023 => {
            c.w64(8, c.r64(0));
            let Some(t1164) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818bf2); };
            c.w64(2, t1164);
            let Some(t1165) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818bf6); };
            c.w64(1, t1165);
            let Some(t1166) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818bfa); };
            c.w64(0, t1166);
            let Some(t1167) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818bfe); };
            c.w64(9, t1167);
            let Some(t1168) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000818c02); };
            c.w64(7, t1168);
            let Some(t1169) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818c06); };
            c.w64(6, t1169);
            let Some(t1170) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818c0a); };
            c.w64(11, t1170);
            blk = 1025; continue;
        }
        1024 => {
            if !c.st64(c.r64(8), c.r64(15)) { return RegfileExit::fault(0x7000818c10); }
            c.w64(8, c.r64(12).wrapping_add(0x1));
            blk = 1025; continue;
        }
        1025 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 1029; continue; }
            blk = 1026; continue;
        }
        1026 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 1028; continue; }
            blk = 1027; continue;
        }
        1027 => {
            let r1171 = c.r64(10) | 0x1u64;
            c.w64(10, r1171);
            c.logic(r1171, 64);
            blk = 1028; continue;
        }
        1028 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 1029; continue;
        }
        1029 => {
            let Some(t1172) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000818c3f); };
            c.w64(12, t1172);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000818c46); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000818c4a); }
            blk = 1030; continue;
        }
        1030 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000818c50); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000818c51); }
            let Some(t1173) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000818c53); };
            c.w64(3, t1173);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818c66, 0x7000818c61);
        }
        1031 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1033; continue; }
            blk = 1032; continue;
        }
        1032 => {
            c.w64(8, c.r64(0));
            let Some(t1174) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818c6d); };
            c.w64(2, t1174);
            let Some(t1175) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818c71); };
            c.w64(0, t1175);
            let Some(t1176) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818c75); };
            c.w64(11, t1176);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x48)) { return RegfileExit::fault(0x7000818c79); }
            blk = 506; continue;
        }
        1033 => {
            let Some(t1177) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818c83); };
            c.w64(2, t1177);
            let Some(t1178) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818c87); };
            c.w64(1, t1178);
            let Some(t1179) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818c8b); };
            c.w64(0, t1179);
            let Some(t1180) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818c8f); };
            c.w64(9, t1180);
            let Some(t1181) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818c93); };
            c.w64(6, t1181);
            blk = 819; continue;
        }
        1034 => {
            return RegfileExit::fault(0x7000818c9c);
        }
        1035 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000818c9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000818ca1); }
            return c.call(0x70007febf4, 0x7000818caa, 0x7000818ca5);
        }
        1036 => {
            c.w64(1, c.r64(0));
            let Some(t1182) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818cad); };
            c.w64(0, t1182);
            let Some(t1183) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818cb1); };
            c.w64(11, t1183);
            let Some(t1184) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000818cb5); };
            c.w64(7, t1184);
            let Some(t1185) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818cb9); };
            c.w64(6, t1185);
            let Some(t1186) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818cbd); };
            c.w64(9, t1186);
            let Some(t1187) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818cc1); };
            c.w64(12, t1187);
            blk = 526; continue;
        }
        1037 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000818ccf); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000818cd3); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000818cd7); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x7000818cdb); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000818cdf); }
            return c.call(0x70007febf4, 0x7000818ce9, 0x7000818ce4);
        }
        1038 => {
            c.w64(7, c.r64(0));
            let Some(t1188) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818cec); };
            c.w64(11, t1188);
            let Some(t1189) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818cf0); };
            c.w64(1, t1189);
            let Some(t1190) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818cf4); };
            c.w64(8, t1190);
            let Some(t1191) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818cf8); };
            c.w64(9, t1191);
            let Some(t1192) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818cfc); };
            c.w64(6, t1192);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000818d00); }
            let Some(t1193) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000818d05); };
            c.w64(12, t1193);
            let Some(t1194) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x7000818d09); };
            c.w64(15, t1194);
            blk = 535; continue;
        }
        1039 => {
            let Some(t1195) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818d15); };
            c.w64(3, t1195);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818d25, 0x7000818d20);
        }
        1040 => {
            return RegfileExit::fault(0x7000818d25);
        }
        1041 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000818d26); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000818d2a); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000818d2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000818d32); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000818d36); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000818d42, 0x7000818d3d);
        }
        1042 => {
            c.w64(9, c.r64(0));
            let Some(t1196) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818d45); };
            c.w64(7, t1196);
            let Some(t1197) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818d49); };
            c.w64(8, t1197);
            let Some(t1198) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818d4d); };
            c.w64(12, t1198);
            let Some(t1199) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000818d51); };
            c.w64(1, t1199);
            let Some(t1200) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818d55); };
            c.w64(0, t1200);
            let Some(t1201) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818d59); };
            c.w64(15, t1201);
            blk = 565; continue;
        }
        1043 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1202) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818d64); };
            c.w64(3, t1202);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818d72, 0x7000818d6d);
        }
        1044 => {
            return RegfileExit::fault(0x7000818d72);
        }
        1045 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000818d73); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000818d77); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_l() { blk = 1061; continue; }
            blk = 1046; continue;
        }
        1046 => {
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 1048; continue; }
            blk = 1047; continue;
        }
        1047 => {
            c.w32(1, c.r64(8).wrapping_add(c.r64(8)));
            blk = 1057; continue;
        }
        1048 => {
            let Some(t1203) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000818d97); };
            c.w64(1, t1203);
            let Some(t1204) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000818d9e); };
            c.w64(15, t1204);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t1205) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000818da5); };
            c.w64(3, t1205);
            let Some(t1206) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000818dac); };
            c.cmp(t1206, c.r64(0), 64);
            if c.cc_a() { blk = 1051; continue; }
            blk = 1049; continue;
        }
        1049 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000818dbb, 0x7000818db6);
        }
        1050 => {
            let Some(t1207) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818dbb); };
            c.w64(12, t1207);
            let Some(t1208) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818dbf); };
            c.w64(7, t1208);
            let Some(t1209) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818dc3); };
            c.w64(11, t1209);
            let Some(t1210) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818dc7); };
            c.w64(9, t1210);
            let Some(t1211) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818dcb); };
            c.w64(8, t1211);
            blk = 1052; continue;
        }
        1051 => {
            if !c.st64(c.r64(1), c.r64(0)) { return RegfileExit::fault(0x7000818dd1); }
            c.w64(1, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 1052; continue;
        }
        1052 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(8) as i64));
            c.logic(c.r64(8) & c.r64(8), 64);
            if c.cc_ns() { blk = 1056; continue; }
            blk = 1053; continue;
        }
        1053 => {
            c.w64(10, c.r64(8));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 1055; continue; }
            blk = 1054; continue;
        }
        1054 => {
            let r1212 = c.r64(10) | 0x1u64;
            c.w64(10, r1212);
            c.logic(r1212, 64);
            blk = 1055; continue;
        }
        1055 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 1056; continue;
        }
        1056 => {
            let Some(t1213) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000818e02); };
            c.w64(1, t1213);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000818e09); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000818e0c); }
            c.w64(1, c.r64(0));
            blk = 1057; continue;
        }
        1057 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000818e14); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000818e15); }
            let Some(t1214) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000818e16); };
            c.w64(3, t1214);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818e29, 0x7000818e24);
        }
        1058 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1060; continue; }
            blk = 1059; continue;
        }
        1059 => {
            c.w64(1, c.r64(0));
            let Some(t1215) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818e30); };
            c.w64(7, t1215);
            let Some(t1216) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818e34); };
            c.w64(8, t1216);
            let Some(t1217) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818e38); };
            c.w64(12, t1217);
            blk = 573; continue;
        }
        1060 => {
            let Some(t1218) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818e41); };
            c.w64(2, t1218);
            let Some(t1219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818e45); };
            c.w64(1, t1219);
            let Some(t1220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818e49); };
            c.w64(0, t1220);
            let Some(t1221) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818e4d); };
            c.w64(9, t1221);
            let Some(t1222) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818e51); };
            c.w64(6, t1222);
            blk = 819; continue;
        }
        1061 => {
            c.w64(2, c.r64(7));
            c.w64(1, c.r64(11));
            let Some(t1223) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000818e60); };
            c.w64(0, t1223);
            let Some(t1224) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000818e64); };
            c.w64(6, t1224);
            blk = 819; continue;
        }
        1062 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000818e6d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000818e71); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(9)) { return RegfileExit::fault(0x7000818e75); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000818e79); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x7000818e7d); }
            return c.call(0x70007febf4, 0x7000818e86, 0x7000818e81);
        }
        1063 => {
            c.w64(15, c.r64(0));
            let Some(t1225) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818e89); };
            c.w64(7, t1225);
            let Some(t1226) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818e8d); };
            c.w64(8, t1226);
            let Some(t1227) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000818e91); };
            c.w64(1, t1227);
            let Some(t1228) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818e95); };
            c.w64(9, t1228);
            let Some(t1229) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818e99); };
            c.w64(12, t1229);
            let Some(t1230) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818e9d); };
            c.w64(11, t1230);
            blk = 580; continue;
        }
        1064 => {
            return RegfileExit::fault(0x7000818ea6);
        }
        1065 => {
            let Some(t1231) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818ea7); };
            c.w64(3, t1231);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818eb7, 0x7000818eb2);
        }
        1066 => {
            return RegfileExit::fault(0x7000818eb7);
        }
        1067 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000818eb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000818ebc); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000818ec0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x7000818ec4); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000818ed0, 0x7000818ecb);
        }
        1068 => {
            c.w64(15, c.r64(0));
            let Some(t1232) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818ed3); };
            c.w64(1, t1232);
            let Some(t1233) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818ed7); };
            c.w64(11, t1233);
            let Some(t1234) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000818edb); };
            c.w64(7, t1234);
            let Some(t1235) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818edf); };
            c.w64(8, t1235);
            let Some(t1236) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818ee3); };
            c.w64(9, t1236);
            let Some(t1237) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818ee7); };
            c.w64(12, t1237);
            blk = 609; continue;
        }
        1069 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x7000818ef5); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000818ef9); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000818efd); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x7000818f01); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x7000818f05); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000818f0a); }
            return c.call(0x70007febf4, 0x7000818f13, 0x7000818f0e);
        }
        1070 => {
            c.w64(1, c.r64(0));
            let Some(t1238) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818f16); };
            c.w64(11, t1238);
            let Some(t1239) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818f1a); };
            c.w64(15, t1239);
            let Some(t1240) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818f1e); };
            c.w64(8, t1240);
            let Some(t1241) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818f22); };
            c.w64(9, t1241);
            let Some(t1242) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818f26); };
            c.w64(6, t1242);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000818f2a); }
            let Some(t1243) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000818f2f); };
            c.w64(7, t1243);
            let Some(t1244) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x7000818f33); };
            c.w64(12, t1244);
            blk = 618; continue;
        }
        1071 => {
            let Some(t1245) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818f3f); };
            c.w64(3, t1245);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818f4f, 0x7000818f4a);
        }
        1072 => {
            return RegfileExit::fault(0x7000818f4f);
        }
        1073 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000818f50); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000818f54); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000818f58); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000818f5c); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000818f60); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000818f6c, 0x7000818f67);
        }
        1074 => {
            c.w64(1, c.r64(0));
            let Some(t1246) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818f6f); };
            c.w64(7, t1246);
            let Some(t1247) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818f73); };
            c.w64(11, t1247);
            let Some(t1248) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818f77); };
            c.w64(12, t1248);
            let Some(t1249) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000818f7b); };
            c.w64(8, t1249);
            let Some(t1250) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000818f7f); };
            c.w64(0, t1250);
            let Some(t1251) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000818f83); };
            c.w64(15, t1251);
            blk = 649; continue;
        }
        1075 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1252) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000818f8e); };
            c.w64(3, t1252);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000818f9c, 0x7000818f97);
        }
        1076 => {
            return RegfileExit::fault(0x7000818f9c);
        }
        1077 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000818f9d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000818fa1); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 1093; continue; }
            blk = 1078; continue;
        }
        1078 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 1080; continue; }
            blk = 1079; continue;
        }
        1079 => {
            c.w32(8, c.r64(11).wrapping_add(c.r64(11)));
            blk = 1089; continue;
        }
        1080 => {
            let Some(t1253) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000818fc1); };
            c.w64(8, t1253);
            let Some(t1254) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000818fc8); };
            c.w64(15, t1254);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t1255) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000818fcf); };
            c.w64(3, t1255);
            let Some(t1256) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000818fd6); };
            c.cmp(t1256, c.r64(0), 64);
            if c.cc_a() { blk = 1083; continue; }
            blk = 1081; continue;
        }
        1081 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x7000818fe5, 0x7000818fe0);
        }
        1082 => {
            let Some(t1257) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000818fe5); };
            c.w64(12, t1257);
            let Some(t1258) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000818fe9); };
            c.w64(7, t1258);
            let Some(t1259) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000818fed); };
            c.w64(1, t1259);
            let Some(t1260) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000818ff1); };
            c.w64(9, t1260);
            let Some(t1261) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000818ff5); };
            c.w64(11, t1261);
            blk = 1084; continue;
        }
        1083 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x7000818ffb); }
            c.w64(8, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 1084; continue;
        }
        1084 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 1088; continue; }
            blk = 1085; continue;
        }
        1085 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 1087; continue; }
            blk = 1086; continue;
        }
        1086 => {
            let r1262 = c.r64(10) | 0x1u64;
            c.w64(10, r1262);
            c.logic(r1262, 64);
            blk = 1087; continue;
        }
        1087 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 1088; continue;
        }
        1088 => {
            let Some(t1263) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081902c); };
            c.w64(8, t1263);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000819033); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000819037); }
            c.w64(8, c.r64(0));
            blk = 1089; continue;
        }
        1089 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081903f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819040); }
            let Some(t1264) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000819042); };
            c.w64(3, t1264);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000819055, 0x7000819050);
        }
        1090 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1092; continue; }
            blk = 1091; continue;
        }
        1091 => {
            c.w64(8, c.r64(0));
            let Some(t1265) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081905c); };
            c.w64(7, t1265);
            let Some(t1266) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000819060); };
            c.w64(11, t1266);
            let Some(t1267) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000819064); };
            c.w64(12, t1267);
            blk = 657; continue;
        }
        1092 => {
            let Some(t1268) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081906d); };
            c.w64(2, t1268);
            let Some(t1269) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000819071); };
            c.w64(1, t1269);
            let Some(t1270) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000819075); };
            c.w64(0, t1270);
            let Some(t1271) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000819079); };
            c.w64(9, t1271);
            let Some(t1272) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081907d); };
            c.w64(6, t1272);
            blk = 819; continue;
        }
        1093 => {
            c.w64(2, c.r64(7));
            let Some(t1273) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000819089); };
            c.w64(0, t1273);
            let Some(t1274) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081908d); };
            c.w64(6, t1274);
            blk = 819; continue;
        }
        1094 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000819096); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x700081909a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700081909e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008190a2); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x70008190a6); }
            return c.call(0x70007febf4, 0x70008190af, 0x70008190aa);
        }
        1095 => {
            c.w64(15, c.r64(0));
            let Some(t1275) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008190b2); };
            c.w64(7, t1275);
            let Some(t1276) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008190b6); };
            c.w64(11, t1276);
            let Some(t1277) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008190ba); };
            c.w64(8, t1277);
            let Some(t1278) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008190be); };
            c.w64(1, t1278);
            let Some(t1279) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008190c2); };
            c.w64(12, t1279);
            let Some(t1280) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008190c6); };
            c.w64(9, t1280);
            blk = 664; continue;
        }
        1096 => {
            return RegfileExit::fault(0x70008190cf);
        }
        1097 => {
            let Some(t1281) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008190d0); };
            c.w64(3, t1281);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008190e0, 0x70008190db);
        }
        1098 => {
            return RegfileExit::fault(0x70008190e0);
        }
        1099 => {
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x70008190e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008190e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x70008190e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x70008190ed); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x70008190f9, 0x70008190f4);
        }
        1100 => {
            c.w64(15, c.r64(0));
            let Some(t1282) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008190fc); };
            c.w64(1, t1282);
            let Some(t1283) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000819100); };
            c.w64(11, t1283);
            let Some(t1284) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000819104); };
            c.w64(7, t1284);
            let Some(t1285) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000819108); };
            c.w64(8, t1285);
            let Some(t1286) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081910c); };
            c.w64(9, t1286);
            let Some(t1287) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000819110); };
            c.w64(12, t1287);
            blk = 693; continue;
        }
        1101 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(15)) { return RegfileExit::fault(0x700081911e); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000819122); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x7000819126); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700081912a); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x700081912e); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000819133); }
            return c.call(0x70007febf4, 0x700081913c, 0x7000819137);
        }
        1102 => {
            c.w64(1, c.r64(0));
            let Some(t1288) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081913f); };
            c.w64(11, t1288);
            let Some(t1289) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000819143); };
            c.w64(15, t1289);
            let Some(t1290) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000819147); };
            c.w64(8, t1290);
            let Some(t1291) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081914b); };
            c.w64(9, t1291);
            let Some(t1292) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081914f); };
            c.w64(6, t1292);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x7000819153); }
            let Some(t1293) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000819158); };
            c.w64(7, t1293);
            let Some(t1294) = c.ld64(0x70008155bcu64) else { return RegfileExit::fault(0x700081915c); };
            c.w64(12, t1294);
            blk = 702; continue;
        }
        1103 => {
            let Some(t1295) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000819168); };
            c.w64(3, t1295);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000819178, 0x7000819173);
        }
        1104 => {
            return RegfileExit::fault(0x7000819178);
        }
        1105 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000819179); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700081917d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000819181); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(0)) { return RegfileExit::fault(0x7000819185); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000819189); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x7000819195, 0x7000819190);
        }
        1106 => {
            c.w64(1, c.r64(0));
            let Some(t1296) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000819198); };
            c.w64(7, t1296);
            let Some(t1297) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081919c); };
            c.w64(11, t1297);
            let Some(t1298) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008191a0); };
            c.w64(12, t1298);
            let Some(t1299) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008191a4); };
            c.w64(9, t1299);
            let Some(t1300) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008191a8); };
            c.w64(0, t1300);
            let Some(t1301) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008191ac); };
            c.w64(15, t1301);
            blk = 733; continue;
        }
        1107 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1302) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008191b7); };
            c.w64(3, t1302);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008191c5, 0x70008191c0);
        }
        1108 => {
            return RegfileExit::fault(0x70008191c5);
        }
        1109 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008191c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008191ca); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 1125; continue; }
            blk = 1110; continue;
        }
        1110 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 1112; continue; }
            blk = 1111; continue;
        }
        1111 => {
            c.w32(9, c.r64(11).wrapping_add(c.r64(11)));
            blk = 1121; continue;
        }
        1112 => {
            let Some(t1303) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008191ea); };
            c.w64(9, t1303);
            let Some(t1304) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008191f1); };
            c.w64(15, t1304);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t1305) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008191f8); };
            c.w64(3, t1305);
            let Some(t1306) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008191ff); };
            c.cmp(t1306, c.r64(0), 64);
            if c.cc_a() { blk = 1115; continue; }
            blk = 1113; continue;
        }
        1113 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x700081920e, 0x7000819209);
        }
        1114 => {
            let Some(t1307) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081920e); };
            c.w64(12, t1307);
            let Some(t1308) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000819212); };
            c.w64(7, t1308);
            let Some(t1309) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000819216); };
            c.w64(1, t1309);
            let Some(t1310) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081921a); };
            c.w64(8, t1310);
            let Some(t1311) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081921e); };
            c.w64(11, t1311);
            blk = 1116; continue;
        }
        1115 => {
            if !c.st64(c.r64(9), c.r64(0)) { return RegfileExit::fault(0x7000819224); }
            c.w64(9, c.r64(15).wrapping_add(0x1));
            c.w64(0, c.r64(9));
            blk = 1116; continue;
        }
        1116 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 1120; continue; }
            blk = 1117; continue;
        }
        1117 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 1119; continue; }
            blk = 1118; continue;
        }
        1118 => {
            let r1312 = c.r64(10) | 0x1u64;
            c.w64(10, r1312);
            c.logic(r1312, 64);
            blk = 1119; continue;
        }
        1119 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 1120; continue;
        }
        1120 => {
            let Some(t1313) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000819255); };
            c.w64(9, t1313);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700081925c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000819260); }
            c.w64(9, c.r64(0));
            blk = 1121; continue;
        }
        1121 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819268); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000819269); }
            let Some(t1314) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x700081926b); };
            c.w64(3, t1314);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081927e, 0x7000819279);
        }
        1122 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1124; continue; }
            blk = 1123; continue;
        }
        1123 => {
            c.w64(9, c.r64(0));
            let Some(t1315) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000819285); };
            c.w64(7, t1315);
            let Some(t1316) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000819289); };
            c.w64(11, t1316);
            let Some(t1317) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081928d); };
            c.w64(12, t1317);
            blk = 741; continue;
        }
        1124 => {
            let Some(t1318) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000819296); };
            c.w64(2, t1318);
            let Some(t1319) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081929a); };
            c.w64(1, t1319);
            let Some(t1320) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081929e); };
            c.w64(0, t1320);
            let Some(t1321) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008192a2); };
            c.w64(9, t1321);
            let Some(t1322) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008192a6); };
            c.w64(6, t1322);
            blk = 819; continue;
        }
        1125 => {
            c.w64(2, c.r64(7));
            let Some(t1323) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008192b2); };
            c.w64(0, t1323);
            c.w64(9, c.r64(8));
            let Some(t1324) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008192b9); };
            c.w64(6, t1324);
            blk = 819; continue;
        }
        1126 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x70008192c2); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x70008192c6); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008192ca); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008192ce); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(8)) { return RegfileExit::fault(0x70008192d2); }
            return c.call(0x70007febf4, 0x70008192db, 0x70008192d6);
        }
        1127 => {
            c.w64(15, c.r64(0));
            let Some(t1325) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008192de); };
            c.w64(7, t1325);
            let Some(t1326) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008192e2); };
            c.w64(11, t1326);
            let Some(t1327) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008192e6); };
            c.w64(9, t1327);
            let Some(t1328) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008192ea); };
            c.w64(1, t1328);
            let Some(t1329) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008192ee); };
            c.w64(12, t1329);
            let Some(t1330) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008192f2); };
            c.w64(8, t1330);
            blk = 748; continue;
        }
        1128 => {
            return RegfileExit::fault(0x70008192fb);
        }
        1129 => {
            let Some(t1331) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008192fc); };
            c.w64(3, t1331);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081930c, 0x7000819307);
        }
        1130 => {
            return RegfileExit::fault(0x700081930c);
        }
        1131 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700081930d); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000819311); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x7000819315); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x7000819319); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x700081931d); }
            c.w64(2, c.r64(8));
            return c.call(0x70007febf4, 0x7000819329, 0x7000819324);
        }
        1132 => {
            c.w64(7, c.r64(0));
            let Some(t1332) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081932c); };
            c.w64(2, t1332);
            let Some(t1333) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000819330); };
            c.w64(11, t1333);
            let Some(t1334) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000819334); };
            c.w64(12, t1334);
            let Some(t1335) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000819338); };
            c.w64(9, t1335);
            let Some(t1336) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081933c); };
            c.w64(1, t1336);
            let Some(t1337) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000819340); };
            c.w64(15, t1337);
            blk = 791; continue;
        }
        1133 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t1338) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081934b); };
            c.w64(3, t1338);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000819359, 0x7000819354);
        }
        1134 => {
            return RegfileExit::fault(0x7000819359);
        }
        1135 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x700081935a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700081935e); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_l() { blk = 1151; continue; }
            blk = 1136; continue;
        }
        1136 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 1138; continue; }
            blk = 1137; continue;
        }
        1137 => {
            c.w32(1, c.r64(11).wrapping_add(c.r64(11)));
            blk = 1147; continue;
        }
        1138 => {
            let Some(t1339) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081937e); };
            c.w64(1, t1339);
            let Some(t1340) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000819385); };
            c.w64(9, t1340);
            c.w64(15, c.r64(9).wrapping_add(0xc));
            let Some(t1341) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081938c); };
            c.w64(3, t1341);
            let Some(t1342) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000819393); };
            c.cmp(t1342, c.r64(15), 64);
            if c.cc_a() { blk = 1141; continue; }
            blk = 1139; continue;
        }
        1139 => {
            c.w32(1, 0xcu64);
            c.w64(2, c.r64(1));
            return c.call(0x70007febf4, 0x70008193a5, 0x70008193a0);
        }
        1140 => {
            c.w64(1, c.r64(0));
            let Some(t1343) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008193a8); };
            c.w64(2, t1343);
            let Some(t1344) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008193ac); };
            c.w64(0, t1344);
            let Some(t1345) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008193b0); };
            c.w64(8, t1345);
            let Some(t1346) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008193b4); };
            c.w64(7, t1346);
            let Some(t1347) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008193b8); };
            c.w64(6, t1347);
            let Some(t1348) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008193bc); };
            c.w64(11, t1348);
            let Some(t1349) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008193c0); };
            c.w64(12, t1349);
            blk = 1142; continue;
        }
        1141 => {
            if !c.st64(c.r64(1), c.r64(15)) { return RegfileExit::fault(0x70008193c6); }
            c.w64(1, c.r64(9).wrapping_add(0x1));
            blk = 1142; continue;
        }
        1142 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(11) as i64));
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 1146; continue; }
            blk = 1143; continue;
        }
        1143 => {
            c.w64(10, c.r64(11));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 1145; continue; }
            blk = 1144; continue;
        }
        1144 => {
            let r1350 = c.r64(10) | 0x1u64;
            c.w64(10, r1350);
            c.logic(r1350, 64);
            blk = 1145; continue;
        }
        1145 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 1146; continue;
        }
        1146 => {
            let Some(t1351) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008193f4); };
            c.w64(9, t1351);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008193fb); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008193ff); }
            blk = 1147; continue;
        }
        1147 => {
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000819404); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x7000819405); }
            let Some(t1352) = c.ld64(c.r64(13).wrapping_add(0x1c88)) else { return RegfileExit::fault(0x7000819406); };
            c.w64(3, t1352);
            c.w32(0, 0x2u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000819419, 0x7000819414);
        }
        1148 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 1150; continue; }
            blk = 1149; continue;
        }
        1149 => {
            c.w64(9, c.r64(0));
            let Some(t1353) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000819420); };
            c.w64(2, t1353);
            let Some(t1354) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000819424); };
            c.w64(11, t1354);
            let Some(t1355) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000819428); };
            c.w64(12, t1355);
            blk = 799; continue;
        }
        1150 => {
            let Some(t1356) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000819431); };
            c.w64(2, t1356);
            let Some(t1357) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000819435); };
            c.w64(1, t1357);
            let Some(t1358) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000819439); };
            c.w64(0, t1358);
            let Some(t1359) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081943d); };
            c.w64(9, t1359);
            let Some(t1360) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000819441); };
            c.w64(6, t1360);
            blk = 819; continue;
        }
        1151 => {
            let Some(t1361) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081944a); };
            c.w64(1, t1361);
            c.w64(9, c.r64(8));
            blk = 819; continue;
        }
        1152 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(11)) { return RegfileExit::fault(0x7000819456); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(9)) { return RegfileExit::fault(0x700081945a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700081945e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000819462); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(7)) { return RegfileExit::fault(0x7000819466); }
            c.w64(2, c.r64(0));
            return c.call(0x70007febf4, 0x7000819472, 0x700081946d);
        }
        1153 => {
            c.w64(8, c.r64(0));
            let Some(t1362) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000819475); };
            c.w64(2, t1362);
            let Some(t1363) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000819479); };
            c.w64(11, t1363);
            let Some(t1364) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081947d); };
            c.w64(9, t1364);
            let Some(t1365) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000819481); };
            c.w64(1, t1365);
            let Some(t1366) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000819485); };
            c.w64(12, t1366);
            let Some(t1367) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000819489); };
            c.w64(7, t1367);
            blk = 806; continue;
        }
        1154 => {
            return RegfileExit::fault(0x7000819492);
        }
        1155 => {
            let Some(t1368) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x7000819493); };
            c.w64(3, t1368);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008194a3, 0x700081949e);
        }
        1156 => {
            return RegfileExit::fault(0x70008194a3);
        }
        1157 => {
            c.w32(0, c.r32(0));
            let r1369 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1369);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1370 = c.r64(7) & c.r64(0);
            c.w64(7, r1370);
            c.logic(r1370, 64);
            let Some(t1371) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008194b3); };
            c.logic(t1371 & 0x2u64, 8);
            if c.cc_e() { blk = 817; continue; }
            blk = 1158; continue;
        }
        1158 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008194c1); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x70008194ca, 0x70008194c5);
        }
        1159 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008194ca); }
            blk = 817; continue;
        }
        1160 => {
            c.w32(8, c.r32(8));
            let r1372 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1372);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1373 = c.r64(1) & c.r64(8);
            c.w64(1, r1373);
            c.logic(r1373, 64);
            let Some(t1374) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008194e0); };
            c.logic(t1374 & 0x2u64, 8);
            if c.cc_e() { blk = 813; continue; }
            blk = 1161; continue;
        }
        1161 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008194ee); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008194ef); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x70008194fc, 0x70008194f7);
        }
        1162 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008194fc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008194fe); }
            blk = 813; continue;
        }
        1163 => {
            c.w32(7, c.r32(7));
            let r1375 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1375);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1376 = c.r64(1) & c.r64(7);
            c.w64(1, r1376);
            c.logic(r1376, 64);
            let Some(t1377) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819513); };
            c.logic(t1377 & 0x2u64, 8);
            if c.cc_e() { blk = 798; continue; }
            blk = 1164; continue;
        }
        1164 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819521); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081952a, 0x7000819525);
        }
        1165 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081952a); }
            blk = 798; continue;
        }
        1166 => {
            c.w32(8, c.r32(8));
            let r1378 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1378);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1379 = c.r64(9) & c.r64(8);
            c.w64(9, r1379);
            c.logic(r1379, 64);
            let Some(t1380) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819540); };
            c.logic(t1380 & 0x2u64, 8);
            if c.cc_e() { blk = 770; continue; }
            blk = 1167; continue;
        }
        1167 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081954f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819550); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x700081955d, 0x7000819558);
        }
        1168 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081955d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081955f); }
            blk = 770; continue;
        }
        1169 => {
            c.w32(0, c.r32(0));
            let r1381 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1381);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1382 = c.r64(7) & c.r64(0);
            c.w64(7, r1382);
            c.logic(r1382, 64);
            let Some(t1383) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819574); };
            c.logic(t1383 & 0x2u64, 8);
            if c.cc_e() { blk = 759; continue; }
            blk = 1170; continue;
        }
        1170 => {
            c.w64(8, c.r64(9).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819582); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x700081958b, 0x7000819586);
        }
        1171 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081958b); }
            blk = 759; continue;
        }
        1172 => {
            c.w32(15, c.r32(15));
            let r1384 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1384);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1385 = c.r64(1) & c.r64(15);
            c.w64(1, r1385);
            c.logic(r1385, 64);
            let Some(t1386) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008195a1); };
            c.logic(t1386 & 0x2u64, 8);
            if c.cc_e() { blk = 755; continue; }
            blk = 1173; continue;
        }
        1173 => {
            c.w64(8, c.r64(7).wrapping_add(0x7));
            return c.call(0x70007f0e54, 0x70008195b4, 0x70008195af);
        }
        1174 => {
            blk = 755; continue;
        }
        1175 => {
            c.w32(1, c.r32(1));
            let r1387 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1387);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1388 = c.r64(8) & c.r64(1);
            c.w64(8, r1388);
            c.logic(r1388, 64);
            let Some(t1389) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008195c8); };
            c.logic(t1389 & 0x2u64, 8);
            if c.cc_e() { blk = 740; continue; }
            blk = 1176; continue;
        }
        1176 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008195d7); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008195e1, 0x70008195dc);
        }
        1177 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008195e1); }
            blk = 740; continue;
        }
        1178 => {
            c.w32(8, c.r32(8));
            let r1390 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r1390);
            c.w64(9, 0xfffffffffffc0000u64);
            let r1391 = c.r64(9) & c.r64(8);
            c.w64(9, r1391);
            c.logic(r1391, 64);
            let Some(t1392) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008195f8); };
            c.logic(t1392 & 0x2u64, 8);
            if c.cc_e() { blk = 712; continue; }
            blk = 1179; continue;
        }
        1179 => {
            c.w64(11, c.r64(7).wrapping_sub(0x1));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819607); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000819611, 0x700081960c);
        }
        1180 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819611); }
            blk = 712; continue;
        }
        1181 => {
            c.w32(15, c.r32(15));
            let r1393 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1393);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1394 = c.r64(8) & c.r64(15);
            c.w64(8, r1394);
            c.logic(r1394, 64);
            let Some(t1395) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819628); };
            c.logic(t1395 & 0x2u64, 8);
            if c.cc_e() { blk = 708; continue; }
            blk = 1182; continue;
        }
        1182 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819637); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000819641, 0x700081963c);
        }
        1183 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819641); }
            blk = 708; continue;
        }
        1184 => {
            c.w32(1, c.r32(1));
            let r1396 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1396);
            c.w64(0, 0xfffffffffffc0000u64);
            let r1397 = c.r64(0) & c.r64(1);
            c.w64(0, r1397);
            c.logic(r1397, 64);
            let Some(t1398) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819657); };
            c.logic(t1398 & 0x2u64, 8);
            if c.cc_e() { blk = 704; continue; }
            blk = 1185; continue;
        }
        1185 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819665); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819666); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000819673, 0x700081966e);
        }
        1186 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819673); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819675); }
            blk = 704; continue;
        }
        1187 => {
            c.w32(0, c.r32(0));
            let r1399 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1399);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1400 = c.r64(7) & c.r64(0);
            c.w64(7, r1400);
            c.logic(r1400, 64);
            let Some(t1401) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081968a); };
            c.logic(t1401 & 0x2u64, 8);
            if c.cc_e() { blk = 675; continue; }
            blk = 1188; continue;
        }
        1188 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819698); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819699); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008196a6, 0x70008196a1);
        }
        1189 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008196a6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008196a8); }
            blk = 675; continue;
        }
        1190 => {
            c.w32(15, c.r32(15));
            let r1402 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1402);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1403 = c.r64(1) & c.r64(15);
            c.w64(1, r1403);
            c.logic(r1403, 64);
            let Some(t1404) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008196be); };
            c.logic(t1404 & 0x2u64, 8);
            if c.cc_e() { blk = 671; continue; }
            blk = 1191; continue;
        }
        1191 => {
            c.w64(8, c.r64(7).wrapping_add(0x7));
            return c.call(0x70007f0e54, 0x70008196d1, 0x70008196cc);
        }
        1192 => {
            blk = 671; continue;
        }
        1193 => {
            c.w32(1, c.r32(1));
            let r1405 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1405);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1406 = c.r64(8) & c.r64(1);
            c.w64(8, r1406);
            c.logic(r1406, 64);
            let Some(t1407) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008196e5); };
            c.logic(t1407 & 0x2u64, 8);
            if c.cc_e() { blk = 656; continue; }
            blk = 1194; continue;
        }
        1194 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008196f4); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008196fe, 0x70008196f9);
        }
        1195 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008196fe); }
            blk = 656; continue;
        }
        1196 => {
            c.w32(9, c.r32(9));
            let r1408 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1408);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1409 = c.r64(8) & c.r64(9);
            c.w64(8, r1409);
            c.logic(r1409, 64);
            let Some(t1410) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819715); };
            c.logic(t1410 & 0x2u64, 8);
            if c.cc_e() { blk = 628; continue; }
            blk = 1197; continue;
        }
        1197 => {
            c.w64(11, c.r64(7).wrapping_sub(0x1));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819724); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x700081972e, 0x7000819729);
        }
        1198 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081972e); }
            blk = 628; continue;
        }
        1199 => {
            c.w32(15, c.r32(15));
            let r1411 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1411);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1412 = c.r64(8) & c.r64(15);
            c.w64(8, r1412);
            c.logic(r1412, 64);
            let Some(t1413) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819745); };
            c.logic(t1413 & 0x2u64, 8);
            if c.cc_e() { blk = 624; continue; }
            blk = 1200; continue;
        }
        1200 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819754); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081975e, 0x7000819759);
        }
        1201 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081975e); }
            blk = 624; continue;
        }
        1202 => {
            c.w32(1, c.r32(1));
            let r1414 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1414);
            c.w64(0, 0xfffffffffffc0000u64);
            let r1415 = c.r64(0) & c.r64(1);
            c.w64(0, r1415);
            c.logic(r1415, 64);
            let Some(t1416) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819774); };
            c.logic(t1416 & 0x2u64, 8);
            if c.cc_e() { blk = 620; continue; }
            blk = 1203; continue;
        }
        1203 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819782); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819783); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x7000819790, 0x700081978b);
        }
        1204 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819790); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819792); }
            blk = 620; continue;
        }
        1205 => {
            c.w32(0, c.r32(0));
            let r1417 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1417);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1418 = c.r64(8) & c.r64(0);
            c.w64(8, r1418);
            c.logic(r1418, 64);
            let Some(t1419) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008197a7); };
            c.logic(t1419 & 0x2u64, 8);
            if c.cc_e() { blk = 591; continue; }
            blk = 1206; continue;
        }
        1206 => {
            c.w64(9, c.r64(1).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008197b6); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008197b7); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008197c4, 0x70008197bf);
        }
        1207 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008197c4); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008197c6); }
            blk = 591; continue;
        }
        1208 => {
            c.w32(15, c.r32(15));
            let r1420 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1420);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1421 = c.r64(1) & c.r64(15);
            c.w64(1, r1421);
            c.logic(r1421, 64);
            let Some(t1422) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008197dc); };
            c.logic(t1422 & 0x2u64, 8);
            if c.cc_e() { blk = 587; continue; }
            blk = 1209; continue;
        }
        1209 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008197ea); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008197f4, 0x70008197ef);
        }
        1210 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008197f4); }
            blk = 587; continue;
        }
        1211 => {
            c.w32(9, c.r32(9));
            let r1423 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1423);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1424 = c.r64(1) & c.r64(9);
            c.w64(1, r1424);
            c.logic(r1424, 64);
            let Some(t1425) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081980b); };
            c.logic(t1425 & 0x2u64, 8);
            if c.cc_e() { blk = 572; continue; }
            blk = 1212; continue;
        }
        1212 => {
            c.w64(11, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819819); }
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000819823, 0x700081981e);
        }
        1213 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819823); }
            blk = 572; continue;
        }
        1214 => {
            c.w32(9, c.r32(9));
            let r1426 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1426);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1427 = c.r64(1) & c.r64(9);
            c.w64(1, r1427);
            c.logic(r1427, 64);
            let Some(t1428) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081983a); };
            c.logic(t1428 & 0x2u64, 8);
            if c.cc_e() { blk = 544; continue; }
            blk = 1215; continue;
        }
        1215 => {
            c.w64(8, c.r64(7).wrapping_sub(0x1));
            return c.call(0x70007f13b4, 0x700081984d, 0x7000819848);
        }
        1216 => {
            blk = 544; continue;
        }
        1217 => {
            c.w32(1, c.r32(1));
            let r1429 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1429);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1430 = c.r64(8) & c.r64(1);
            c.w64(8, r1430);
            c.logic(r1430, 64);
            let Some(t1431) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819861); };
            c.logic(t1431 & 0x2u64, 8);
            if c.cc_e() { blk = 540; continue; }
            blk = 1218; continue;
        }
        1218 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819870); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081987a, 0x7000819875);
        }
        1219 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081987a); }
            blk = 540; continue;
        }
        1220 => {
            c.w32(7, c.r32(7));
            let r1432 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1432);
            c.w64(0, 0xfffffffffffc0000u64);
            let r1433 = c.r64(0) & c.r64(7);
            c.w64(0, r1433);
            c.logic(r1433, 64);
            let Some(t1434) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819890); };
            c.logic(t1434 & 0x2u64, 8);
            if c.cc_e() { blk = 537; continue; }
            blk = 1221; continue;
        }
        1221 => {
            c.w64(3, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081989e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081989f); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(3));
            return c.call(0x70007f0e54, 0x70008198ac, 0x70008198a7);
        }
        1222 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008198ac); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008198ae); }
            blk = 537; continue;
        }
        1223 => {
            c.w32(1, c.r32(1));
            let r1435 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1435);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1436 = c.r64(7) & c.r64(1);
            c.w64(7, r1436);
            c.logic(r1436, 64);
            let Some(t1437) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008198c3); };
            c.logic(t1437 & 0x2u64, 8);
            if c.cc_e() { blk = 505; continue; }
            blk = 1224; continue;
        }
        1224 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008198d1); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x70008198da, 0x70008198d5);
        }
        1225 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008198da); }
            blk = 505; continue;
        }
        1226 => {
            c.w32(9, c.r32(9));
            let r1438 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1438);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1439 = c.r64(8) & c.r64(9);
            c.w64(8, r1439);
            c.logic(r1439, 64);
            let Some(t1440) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008198f0); };
            c.logic(t1440 & 0x2u64, 8);
            if c.cc_e() { blk = 471; continue; }
            blk = 1227; continue;
        }
        1227 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008198ff); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819900); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x700081990d, 0x7000819908);
        }
        1228 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081990d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081990f); }
            blk = 471; continue;
        }
        1229 => {
            c.w32(0, c.r32(0));
            let r1441 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1441);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1442 = c.r64(7) & c.r64(0);
            c.w64(7, r1442);
            c.logic(r1442, 64);
            let Some(t1443) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819924); };
            c.logic(t1443 & 0x2u64, 8);
            if c.cc_e() { blk = 460; continue; }
            blk = 1230; continue;
        }
        1230 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819932); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819933); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000819940, 0x700081993b);
        }
        1231 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819940); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819942); }
            blk = 460; continue;
        }
        1232 => {
            c.w32(9, c.r32(9));
            let r1444 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1444);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1445 = c.r64(1) & c.r64(9);
            c.w64(1, r1445);
            c.logic(r1445, 64);
            let Some(t1446) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819958); };
            c.logic(t1446 & 0x2u64, 8);
            if c.cc_e() { blk = 456; continue; }
            blk = 1233; continue;
        }
        1233 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819966); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819967); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819974, 0x700081996f);
        }
        1234 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819974); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819976); }
            blk = 456; continue;
        }
        1235 => {
            c.w32(1, c.r32(1));
            let r1447 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1447);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1448 = c.r64(7) & c.r64(1);
            c.w64(7, r1448);
            c.logic(r1448, 64);
            let Some(t1449) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081998b); };
            c.logic(t1449 & 0x2u64, 8);
            if c.cc_e() { blk = 440; continue; }
            blk = 1236; continue;
        }
        1236 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819999); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x70008199a2, 0x700081999d);
        }
        1237 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008199a2); }
            blk = 440; continue;
        }
        1238 => {
            c.w32(9, c.r32(9));
            let r1450 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1450);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1451 = c.r64(8) & c.r64(9);
            c.w64(8, r1451);
            c.logic(r1451, 64);
            let Some(t1452) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008199b8); };
            c.logic(t1452 & 0x2u64, 8);
            if c.cc_e() { blk = 414; continue; }
            blk = 1239; continue;
        }
        1239 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008199c7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008199c8); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x70008199d5, 0x70008199d0);
        }
        1240 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008199d5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008199d7); }
            blk = 414; continue;
        }
        1241 => {
            c.w32(1, c.r32(1));
            let r1453 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1453);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1454 = c.r64(7) & c.r64(1);
            c.w64(7, r1454);
            c.logic(r1454, 64);
            let Some(t1455) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008199ec); };
            c.logic(t1455 & 0x2u64, 8);
            if c.cc_e() { blk = 400; continue; }
            blk = 1242; continue;
        }
        1242 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008199fa); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819a03, 0x70008199fe);
        }
        1243 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819a03); }
            blk = 400; continue;
        }
        1244 => {
            c.w32(7, c.r32(7));
            let r1456 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1456);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1457 = c.r64(8) & c.r64(7);
            c.w64(8, r1457);
            c.logic(r1457, 64);
            let Some(t1458) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819a18); };
            c.logic(t1458 & 0x2u64, 8);
            if c.cc_e() { blk = 363; continue; }
            blk = 1245; continue;
        }
        1245 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819a27); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819a28); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000819a35, 0x7000819a30);
        }
        1246 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819a35); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819a37); }
            blk = 363; continue;
        }
        1247 => {
            c.w32(15, c.r32(15));
            let r1459 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1459);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1460 = c.r64(8) & c.r64(15);
            c.w64(8, r1460);
            c.logic(r1460, 64);
            let Some(t1461) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819a4d); };
            c.logic(t1461 & 0x2u64, 8);
            if c.cc_e() { blk = 360; continue; }
            blk = 1248; continue;
        }
        1248 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819a5c); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000819a66, 0x7000819a61);
        }
        1249 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819a66); }
            blk = 360; continue;
        }
        1250 => {
            c.w32(0, c.r32(0));
            let r1462 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1462);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1463 = c.r64(7) & c.r64(0);
            c.w64(7, r1463);
            c.logic(r1463, 64);
            let Some(t1464) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819a7c); };
            c.logic(t1464 & 0x2u64, 8);
            if c.cc_e() { blk = 332; continue; }
            blk = 1251; continue;
        }
        1251 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819a8a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819a8b); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000819a98, 0x7000819a93);
        }
        1252 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819a98); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819a9a); }
            blk = 332; continue;
        }
        1253 => {
            c.w32(9, c.r32(9));
            let r1465 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1465);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1466 = c.r64(1) & c.r64(9);
            c.w64(1, r1466);
            c.logic(r1466, 64);
            let Some(t1467) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819ab0); };
            c.logic(t1467 & 0x2u64, 8);
            if c.cc_e() { blk = 328; continue; }
            blk = 1254; continue;
        }
        1254 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819abe); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819abf); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819acc, 0x7000819ac7);
        }
        1255 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819acc); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819ace); }
            blk = 328; continue;
        }
        1256 => {
            c.w32(1, c.r32(1));
            let r1468 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1468);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1469 = c.r64(7) & c.r64(1);
            c.w64(7, r1469);
            c.logic(r1469, 64);
            let Some(t1470) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819ae3); };
            c.logic(t1470 & 0x2u64, 8);
            if c.cc_e() { blk = 313; continue; }
            blk = 1257; continue;
        }
        1257 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819af1); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819afa, 0x7000819af5);
        }
        1258 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819afa); }
            blk = 313; continue;
        }
        1259 => {
            c.w32(9, c.r32(9));
            let r1471 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1471);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1472 = c.r64(8) & c.r64(9);
            c.w64(8, r1472);
            c.logic(r1472, 64);
            let Some(t1473) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819b10); };
            c.logic(t1473 & 0x2u64, 8);
            if c.cc_e() { blk = 287; continue; }
            blk = 1260; continue;
        }
        1260 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819b1f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819b20); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000819b2d, 0x7000819b28);
        }
        1261 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819b2d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819b2f); }
            blk = 287; continue;
        }
        1262 => {
            c.w32(0, c.r32(0));
            let r1474 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1474);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1475 = c.r64(7) & c.r64(0);
            c.w64(7, r1475);
            c.logic(r1475, 64);
            let Some(t1476) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819b44); };
            c.logic(t1476 & 0x2u64, 8);
            if c.cc_e() { blk = 276; continue; }
            blk = 1263; continue;
        }
        1263 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819b52); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819b53); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000819b60, 0x7000819b5b);
        }
        1264 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819b60); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819b62); }
            blk = 276; continue;
        }
        1265 => {
            c.w32(9, c.r32(9));
            let r1477 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1477);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1478 = c.r64(1) & c.r64(9);
            c.w64(1, r1478);
            c.logic(r1478, 64);
            let Some(t1479) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819b78); };
            c.logic(t1479 & 0x2u64, 8);
            if c.cc_e() { blk = 272; continue; }
            blk = 1266; continue;
        }
        1266 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819b86); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819b87); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819b94, 0x7000819b8f);
        }
        1267 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819b94); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819b96); }
            blk = 272; continue;
        }
        1268 => {
            c.w32(1, c.r32(1));
            let r1480 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1480);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1481 = c.r64(7) & c.r64(1);
            c.w64(7, r1481);
            c.logic(r1481, 64);
            let Some(t1482) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819bab); };
            c.logic(t1482 & 0x2u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 1269; continue;
        }
        1269 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819bb9); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819bc2, 0x7000819bbd);
        }
        1270 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819bc2); }
            blk = 256; continue;
        }
        1271 => {
            c.w32(9, c.r32(9));
            let r1483 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r1483);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1484 = c.r64(8) & c.r64(9);
            c.w64(8, r1484);
            c.logic(r1484, 64);
            let Some(t1485) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819bd8); };
            c.logic(t1485 & 0x2u64, 8);
            if c.cc_e() { blk = 230; continue; }
            blk = 1272; continue;
        }
        1272 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819be7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819be8); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000819bf5, 0x7000819bf0);
        }
        1273 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819bf5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819bf7); }
            blk = 230; continue;
        }
        1274 => {
            c.w32(1, c.r32(1));
            let r1486 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1486);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1487 = c.r64(7) & c.r64(1);
            c.w64(7, r1487);
            c.logic(r1487, 64);
            let Some(t1488) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819c0c); };
            c.logic(t1488 & 0x2u64, 8);
            if c.cc_e() { blk = 216; continue; }
            blk = 1275; continue;
        }
        1275 => {
            c.w64(8, c.r64(9).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819c1a); }
            c.w64(7, c.r64(9));
            return c.call(0x70007f0e54, 0x7000819c23, 0x7000819c1e);
        }
        1276 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819c23); }
            blk = 216; continue;
        }
        1277 => {
            c.w32(7, c.r32(7));
            let r1489 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1489);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1490 = c.r64(8) & c.r64(7);
            c.w64(8, r1490);
            c.logic(r1490, 64);
            let Some(t1491) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819c38); };
            c.logic(t1491 & 0x2u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 1278; continue;
        }
        1278 => {
            c.w64(11, c.r64(9).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819c47); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819c48); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000819c55, 0x7000819c50);
        }
        1279 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819c55); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819c57); }
            blk = 179; continue;
        }
        1280 => {
            c.w32(7, c.r32(7));
            let r1492 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1492);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1493 = c.r64(8) & c.r64(7);
            c.w64(8, r1493);
            c.logic(r1493, 64);
            let Some(t1494) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819c6c); };
            c.logic(t1494 & 0x2u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 1281; continue;
        }
        1281 => {
            c.w64(12, c.r64(9).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819c7b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819c7c); }
            c.w64(7, c.r64(9));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000819c89, 0x7000819c84);
        }
        1282 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819c89); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819c8b); }
            blk = 175; continue;
        }
        1283 => {
            c.w32(1, c.r32(1));
            let r1495 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1495);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1496 = c.r64(7) & c.r64(1);
            c.w64(7, r1496);
            c.logic(r1496, 64);
            let Some(t1497) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819ca0); };
            c.logic(t1497 & 0x2u64, 8);
            if c.cc_e() { blk = 143; continue; }
            blk = 1284; continue;
        }
        1284 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819cae); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819cb7, 0x7000819cb2);
        }
        1285 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819cb7); }
            blk = 143; continue;
        }
        1286 => {
            c.w32(7, c.r32(7));
            let r1498 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1498);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1499 = c.r64(8) & c.r64(7);
            c.w64(8, r1499);
            c.logic(r1499, 64);
            let Some(t1500) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819ccc); };
            c.logic(t1500 & 0x2u64, 8);
            if c.cc_e() { blk = 106; continue; }
            blk = 1287; continue;
        }
        1287 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819cdb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819cdc); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000819ce9, 0x7000819ce4);
        }
        1288 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819ce9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819ceb); }
            blk = 106; continue;
        }
        1289 => {
            c.w32(15, c.r32(15));
            let r1501 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r1501);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1502 = c.r64(8) & c.r64(15);
            c.w64(8, r1502);
            c.logic(r1502, 64);
            let Some(t1503) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819d01); };
            c.logic(t1503 & 0x2u64, 8);
            if c.cc_e() { blk = 103; continue; }
            blk = 1290; continue;
        }
        1290 => {
            c.w64(9, c.r64(7).wrapping_add(0x7));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819d10); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x7000819d1a, 0x7000819d15);
        }
        1291 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819d1a); }
            blk = 103; continue;
        }
        1292 => {
            c.w32(11, c.r32(11));
            let r1504 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1504);
            c.w64(1, 0xfffffffffffc0000u64);
            let r1505 = c.r64(1) & c.r64(11);
            c.w64(1, r1505);
            c.logic(r1505, 64);
            let Some(t1506) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819d31); };
            c.logic(t1506 & 0x2u64, 8);
            if c.cc_e() { blk = 73; continue; }
            blk = 1293; continue;
        }
        1293 => {
            c.w64(7, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819d3f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819d40); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819d4d, 0x7000819d48);
        }
        1294 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819d4d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819d4f); }
            blk = 73; continue;
        }
        1295 => {
            c.w32(1, c.r32(1));
            let r1507 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1507);
            c.w64(7, 0xfffffffffffc0000u64);
            let r1508 = c.r64(7) & c.r64(1);
            c.w64(7, r1508);
            c.logic(r1508, 64);
            let Some(t1509) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819d64); };
            c.logic(t1509 & 0x2u64, 8);
            if c.cc_e() { blk = 58; continue; }
            blk = 1296; continue;
        }
        1296 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819d72); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x7000819d7b, 0x7000819d76);
        }
        1297 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819d7b); }
            blk = 58; continue;
        }
        1298 => {
            c.w32(11, c.r32(11));
            let r1510 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r1510);
            c.w64(8, 0xfffffffffffc0000u64);
            let r1511 = c.r64(8) & c.r64(11);
            c.w64(8, r1511);
            c.logic(r1511, 64);
            let Some(t1512) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819d91); };
            c.logic(t1512 & 0x2u64, 8);
            if c.cc_e() { blk = 28; continue; }
            blk = 1299; continue;
        }
        1299 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000819da0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000819da1); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x7000819dae, 0x7000819da9);
        }
        1300 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000819dae); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000819db0); }
            blk = 28; continue;
        }
        1301 => {
            return RegfileExit::jump(0x7000819db8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000819dd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000819dd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000819dd8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000819dde); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000819de5); };
            c.w32(9, t3);
            let Some(t4) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000819dea); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000819e06); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x7000819e0a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000819e0e); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000819e12); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x7000819e16); }
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
            let Some(t12) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819ed9); };
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
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000819eed); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000819ef4); };
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
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000819f1d); };
            c.w32(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000819f21); };
            c.cmp(t18, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x7000819f2e); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r19 = c.r64(8) & c.r64(2);
            c.w64(8, r19);
            c.logic(r19, 64);
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000819f3c); };
            c.logic(t20 & 0x4u64, 8);
            if c.cc_ne() { blk = 569; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000819f47); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000819f4e); };
            c.w32(9, t23);
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x7000819f52); };
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
            let Some(t25) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000819f64); };
            c.w32(9, t25);
            let r26 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r26);
            let Some(t27) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000819f6b); };
            c.cmp(t27, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x7000819f77); }
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
            let Some(t29) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000819fc9); };
            c.w32(12, t29);
            let r30 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r30);
            let Some(t31) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000819fd0); };
            c.cmp(t31, 0x834u64, 16);
            if c.cc_e() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000819fda); };
            c.w32(12, t32);
            blk = 39; continue;
        }
        38 => {
            let Some(t33) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000819fe0); };
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
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000819ff3); };
            c.w32(1, t34);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 440; continue; }
            blk = 41; continue;
        }
        41 => {
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700081a006); }
            blk = 437; continue;
        }
        42 => {
            c.w64(9, c.r64(2));
            let r35 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r35);
            c.logic(r35, 64);
            let Some(t36) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a01a); };
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
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a02e); };
            c.w64(15, t37);
            let Some(t38) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081a035); };
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
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081a05d); };
            c.w32(7, t41);
            let Some(t42) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081a060); };
            c.cmp(t42, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t43) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a06d); };
            c.w32(7, t43);
            let r44 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r44);
            let Some(t45) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081a073); };
            c.cmp(t45, c.r32(7), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t46) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a080); };
            c.w32(9, t46);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t47) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a08d); };
            c.cmp(t47, 0x834u64, 16);
            if c.cc_e() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(8, c.r64(12));
            blk = 53; continue;
        }
        52 => {
            let Some(t48) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081a09b); };
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
            let Some(t49) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a0d4); };
            c.w64(3, t49);
            let Some(t50) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081a0db); };
            c.w64(6, t50);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t51) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a0e2); };
            c.cmp(t51, c.r64(0), 64);
            if c.cc_be() { blk = 443; continue; }
            blk = 57; continue;
        }
        57 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081a0ec); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 58; continue;
        }
        58 => {
            let r52 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r52);
            let Some(t53) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700081a0f6); };
            c.w64(0, t53);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081a0fd); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081a101); }
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081a105); };
            c.w64(12, t54);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a10c); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let r55 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r55);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081a12c); }
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
            let Some(t57) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081a149); };
            c.w32(9, t57);
            let r58 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081a151); };
            c.cmp(t59, c.r32(9), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let r60 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r60);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081a166); }
            blk = 64; continue;
        }
        63 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x700081a178); }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 61; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081a182); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r61 = c.r64(7) & c.r64(2);
            c.w64(7, r61);
            c.logic(r61, 64);
            let Some(t62) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a196); };
            c.logic(t62 & 0x4u64, 8);
            if c.cc_ne() { blk = 566; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081a1a0); };
            c.w64(0, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081a1a4); };
            c.w64(6, t64);
            blk = 68; continue;
        }
        68 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081a1a8); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r65 = c.r64(7) & c.r64(2);
            c.w64(7, r65);
            c.logic(r65, 64);
            let Some(t66) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a1b6); };
            c.logic(t66 & 0x4u64, 8);
            if c.cc_ne() { blk = 563; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t67) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a1c0); };
            c.w32(7, t67);
            let r68 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r68);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 75; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t69) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a1cb); };
            c.w32(8, t69);
            let r70 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r70);
            let Some(t71) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a1d2); };
            c.cmp(t71, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 71; continue;
        }
        71 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a1de); }
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
            let Some(t72) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081a201); };
            c.w64(9, t72);
            let r73 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r73);
            let Some(t74) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081a20b); };
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
            let Some(t75) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a229); };
            c.w32(12, t75);
            let Some(t76) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081a22d); };
            c.cmp(t76, c.r32(12), 32);
            if c.cc_ne() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a236); }
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
            let Some(t78) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a258); };
            c.w32(12, t78);
            let r79 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r79);
            let Some(t80) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a25f); };
            c.cmp(t80, 0x834u64, 16);
            if c.cc_e() { blk = 83; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t81) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a269); };
            c.w32(12, t81);
            blk = 84; continue;
        }
        83 => {
            let Some(t82) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081a26f); };
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
            let Some(t83) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a286); };
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
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081a2a2); }
            c.w64(0, c.r64(9));
            blk = 437; continue;
        }
        88 => {
            c.w64(9, c.r64(2));
            let r84 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a2bb); };
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
            let Some(t86) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a2cf); };
            c.w64(15, t86);
            let Some(t87) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081a2d6); };
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
            let Some(t90) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081a2fe); };
            c.w32(7, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081a301); };
            c.cmp(t91, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t92) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a30e); };
            c.w32(7, t92);
            let r93 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r93);
            let Some(t94) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081a314); };
            c.cmp(t94, c.r32(7), 32);
            if c.cc_e() { blk = 114; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t95) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a321); };
            c.w32(9, t95);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t96) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a32e); };
            c.cmp(t96, 0x834u64, 16);
            if c.cc_e() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(8, c.r64(12));
            blk = 99; continue;
        }
        98 => {
            let Some(t97) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081a33c); };
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
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a375); };
            c.w64(3, t98);
            let Some(t99) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081a37c); };
            c.w64(6, t99);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t100) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a383); };
            c.cmp(t100, c.r64(0), 64);
            if c.cc_be() { blk = 450; continue; }
            blk = 103; continue;
        }
        103 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081a38d); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 104; continue;
        }
        104 => {
            let r101 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r101);
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700081a397); };
            c.w64(0, t102);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081a39e); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081a3a2); }
            let Some(t103) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081a3a6); };
            c.w64(12, t103);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a3ad); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            let r104 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r104);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081a3cd); }
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
            let Some(t106) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081a3ea); };
            c.w32(9, t106);
            let r107 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081a3f2); };
            c.cmp(t108, c.r32(9), 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 108; continue;
        }
        108 => {
            let r109 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r109);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081a407); }
            blk = 110; continue;
        }
        109 => {
            let Some(t110) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081a40f); };
            c.w64(9, t110);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x700081a416); }
            blk = 110; continue;
        }
        110 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 107; continue; }
            blk = 111; continue;
        }
        111 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081a420); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r111 = c.r64(7) & c.r64(2);
            c.w64(7, r111);
            c.logic(r111, 64);
            let Some(t112) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a434); };
            c.logic(t112 & 0x4u64, 8);
            if c.cc_ne() { blk = 560; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081a43e); };
            c.w64(0, t113);
            let Some(t114) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081a442); };
            c.w64(6, t114);
            blk = 114; continue;
        }
        114 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081a446); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r115 = c.r64(7) & c.r64(2);
            c.w64(7, r115);
            c.logic(r115, 64);
            let Some(t116) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a454); };
            c.logic(t116 & 0x4u64, 8);
            if c.cc_ne() { blk = 557; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t117) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a45e); };
            c.w32(7, t117);
            let r118 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r118);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t119) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a469); };
            c.w32(8, t119);
            let r120 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r120);
            let Some(t121) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a470); };
            c.cmp(t121, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 117; continue;
        }
        117 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a47c); }
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
            let Some(t122) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081a49f); };
            c.w64(9, t122);
            let r123 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r123);
            let Some(t124) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081a4a9); };
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
            let Some(t125) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a4c4); };
            c.w32(9, t125);
            let Some(t126) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081a4c8); };
            c.cmp(t126, c.r32(9), 32);
            if c.cc_ne() { blk = 438; continue; }
            blk = 124; continue;
        }
        124 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a4d5); }
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
            let Some(t128) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a4eb); };
            c.w32(9, t128);
            let r129 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r129);
            let Some(t130) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a4f2); };
            c.cmp(t130, 0x834u64, 16);
            if c.cc_e() { blk = 128; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t131) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a4fb); };
            c.w32(9, t131);
            blk = 129; continue;
        }
        128 => {
            let Some(t132) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081a501); };
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
            let Some(t133) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a514); };
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
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081a530); }
            blk = 437; continue;
        }
        132 => {
            c.w64(8, c.r64(2));
            let r134 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r134);
            c.logic(r134, 64);
            let Some(t135) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a546); };
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
            let Some(t136) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a55a); };
            c.w64(12, t136);
            let Some(t137) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081a561); };
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
            let Some(t140) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081a58a); };
            c.w32(8, t140);
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081a58e); };
            c.cmp(t141, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 139; continue;
        }
        139 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081a59b); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r142 = c.r64(8) & c.r64(2);
            c.w64(8, r142);
            c.logic(r142, 64);
            let Some(t143) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a5a9); };
            c.logic(t143 & 0x4u64, 8);
            if c.cc_ne() { blk = 554; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t144) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a5b4); };
            c.w32(8, t144);
            let r145 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r145);
            let Some(t146) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a5bb); };
            c.w32(9, t146);
            let Some(t147) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081a5bf); };
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
            let Some(t148) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a5d1); };
            c.w32(9, t148);
            let r149 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r149);
            let Some(t150) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a5d8); };
            c.cmp(t150, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 143; continue;
        }
        143 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a5e4); }
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
            let Some(t151) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081a607); };
            c.w64(12, t151);
            let r152 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r152);
            let Some(t153) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081a611); };
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
            let Some(t154) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a628); };
            c.w32(12, t154);
            let r155 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r155);
            let Some(t156) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a62f); };
            c.cmp(t156, 0x834u64, 16);
            if c.cc_e() { blk = 150; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t157) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a639); };
            c.w32(12, t157);
            blk = 151; continue;
        }
        150 => {
            let Some(t158) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081a63f); };
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
            let Some(t159) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a652); };
            c.w32(1, t159);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 455; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081a66d); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r160 = c.r64(7) & c.r64(8);
            c.w64(7, r160);
            c.logic(r160, 64);
            let Some(t161) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a67f); };
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
            let Some(t163) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a698); };
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
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a6ac); };
            c.w64(12, t164);
            let Some(t165) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081a6b3); };
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
            let Some(t168) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081a6dc); };
            c.w32(8, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081a6e0); };
            c.cmp(t169, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 163; continue;
        }
        163 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081a6ed); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r170 = c.r64(8) & c.r64(2);
            c.w64(8, r170);
            c.logic(r170, 64);
            let Some(t171) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a6fb); };
            c.logic(t171 & 0x4u64, 8);
            if c.cc_ne() { blk = 548; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t172) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a706); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            let Some(t174) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a70d); };
            c.w32(9, t174);
            let Some(t175) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081a711); };
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
            let Some(t176) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a723); };
            c.w32(9, t176);
            let r177 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r177);
            let Some(t178) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a72a); };
            c.cmp(t178, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a736); }
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
            let Some(t179) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081a759); };
            c.w64(12, t179);
            let r180 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r180);
            let Some(t181) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081a763); };
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
            let Some(t182) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a77a); };
            c.w32(12, t182);
            let r183 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r183);
            let Some(t184) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a781); };
            c.cmp(t184, 0x834u64, 16);
            if c.cc_e() { blk = 174; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t185) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a78b); };
            c.w32(12, t185);
            blk = 175; continue;
        }
        174 => {
            let Some(t186) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081a791); };
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
            let Some(t187) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a7a4); };
            c.w32(1, t187);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 456; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081a7bf); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r188 = c.r64(7) & c.r64(8);
            c.w64(7, r188);
            c.logic(r188, 64);
            let Some(t189) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a7d1); };
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
            let Some(t191) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a7ea); };
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
            let Some(t192) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a7fe); };
            c.w64(15, t192);
            let Some(t193) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081a805); };
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
            let Some(t196) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081a82d); };
            c.w32(7, t196);
            let Some(t197) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081a830); };
            c.cmp(t197, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 187; continue;
        }
        187 => {
            let Some(t198) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a83d); };
            c.w32(7, t198);
            let r199 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r199);
            let Some(t200) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081a843); };
            c.cmp(t200, c.r32(7), 32);
            if c.cc_e() { blk = 206; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t201) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081a850); };
            c.w32(9, t201);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t202) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a85d); };
            c.cmp(t202, 0x834u64, 16);
            if c.cc_e() { blk = 190; continue; }
            blk = 189; continue;
        }
        189 => {
            c.w64(8, c.r64(12));
            blk = 191; continue;
        }
        190 => {
            let Some(t203) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081a86b); };
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
            let Some(t204) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081a8a4); };
            c.w64(3, t204);
            let Some(t205) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081a8ab); };
            c.w64(6, t205);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t206) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a8b2); };
            c.cmp(t206, c.r64(0), 64);
            if c.cc_be() { blk = 459; continue; }
            blk = 195; continue;
        }
        195 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081a8bc); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 196; continue;
        }
        196 => {
            let r207 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r207);
            let Some(t208) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700081a8c6); };
            c.w64(0, t208);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081a8cd); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081a8d1); }
            let Some(t209) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081a8d5); };
            c.w64(12, t209);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a8dc); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 198; continue; }
            blk = 197; continue;
        }
        197 => {
            let r210 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r210);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081a8fc); }
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
            let Some(t212) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081a919); };
            c.w32(9, t212);
            let r213 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r213);
            let Some(t214) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081a921); };
            c.cmp(t214, c.r32(9), 32);
            if c.cc_e() { blk = 201; continue; }
            blk = 200; continue;
        }
        200 => {
            let r215 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r215);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081a936); }
            blk = 202; continue;
        }
        201 => {
            let Some(t216) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081a93e); };
            c.w64(9, t216);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x700081a945); }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 199; continue; }
            blk = 203; continue;
        }
        203 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081a94f); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 204; continue;
        }
        204 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r217 = c.r64(7) & c.r64(2);
            c.w64(7, r217);
            c.logic(r217, 64);
            let Some(t218) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a963); };
            c.logic(t218 & 0x4u64, 8);
            if c.cc_ne() { blk = 542; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081a96d); };
            c.w64(0, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081a971); };
            c.w64(6, t220);
            blk = 206; continue;
        }
        206 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081a975); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r221 = c.r64(7) & c.r64(2);
            c.w64(7, r221);
            c.logic(r221, 64);
            let Some(t222) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081a983); };
            c.logic(t222 & 0x4u64, 8);
            if c.cc_ne() { blk = 539; continue; }
            blk = 207; continue;
        }
        207 => {
            let Some(t223) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a98d); };
            c.w32(7, t223);
            let r224 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r224);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 213; continue; }
            blk = 208; continue;
        }
        208 => {
            let Some(t225) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a998); };
            c.w32(8, t225);
            let r226 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r226);
            let Some(t227) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081a99f); };
            c.cmp(t227, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 209; continue;
        }
        209 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081a9ab); }
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
            let Some(t228) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081a9ce); };
            c.w64(9, t228);
            let r229 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r229);
            let Some(t230) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081a9d8); };
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
            let Some(t231) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081a9f3); };
            c.w32(9, t231);
            let Some(t232) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081a9f7); };
            c.cmp(t232, c.r32(9), 32);
            if c.cc_ne() { blk = 438; continue; }
            blk = 216; continue;
        }
        216 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700081aa04); }
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
            let Some(t234) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081aa1a); };
            c.w32(9, t234);
            let r235 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r235);
            let Some(t236) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081aa21); };
            c.cmp(t236, 0x834u64, 16);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            let Some(t237) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081aa2a); };
            c.w32(9, t237);
            blk = 221; continue;
        }
        220 => {
            let Some(t238) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081aa30); };
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
            let Some(t239) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081aa43); };
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
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081aa5f); }
            blk = 437; continue;
        }
        224 => {
            c.w64(8, c.r64(2));
            let r240 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r240);
            c.logic(r240, 64);
            let Some(t241) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081aa75); };
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
            let Some(t242) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081aa89); };
            c.w64(12, t242);
            let Some(t243) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081aa90); };
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
            let Some(t246) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081aab9); };
            c.w32(8, t246);
            let Some(t247) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081aabd); };
            c.cmp(t247, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 231; continue;
        }
        231 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081aaca); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r248 = c.r64(8) & c.r64(2);
            c.w64(8, r248);
            c.logic(r248, 64);
            let Some(t249) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081aad8); };
            c.logic(t249 & 0x4u64, 8);
            if c.cc_ne() { blk = 536; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t250) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081aae3); };
            c.w32(8, t250);
            let r251 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r251);
            let Some(t252) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081aaea); };
            c.w32(9, t252);
            let Some(t253) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081aaee); };
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
            let Some(t254) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ab00); };
            c.w32(9, t254);
            let r255 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r255);
            let Some(t256) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ab07); };
            c.cmp(t256, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 235; continue;
        }
        235 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081ab13); }
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
            let Some(t257) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081ab36); };
            c.w64(12, t257);
            let r258 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r258);
            let Some(t259) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081ab40); };
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
            let Some(t260) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ab57); };
            c.w32(12, t260);
            let r261 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r261);
            let Some(t262) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ab5e); };
            c.cmp(t262, 0x834u64, 16);
            if c.cc_e() { blk = 242; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t263) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ab68); };
            c.w32(12, t263);
            blk = 243; continue;
        }
        242 => {
            let Some(t264) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081ab6e); };
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
            let Some(t265) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ab81); };
            c.w32(1, t265);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 464; continue; }
            blk = 245; continue;
        }
        245 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081ab9c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r266 = c.r64(7) & c.r64(8);
            c.w64(7, r266);
            c.logic(r266, 64);
            let Some(t267) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081abae); };
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
            let Some(t269) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081abc7); };
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
            let Some(t270) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081abdb); };
            c.w64(12, t270);
            let Some(t271) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081abe2); };
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
            let Some(t274) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081ac0b); };
            c.w32(8, t274);
            let Some(t275) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081ac0f); };
            c.cmp(t275, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 255; continue;
        }
        255 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081ac1c); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r276 = c.r64(8) & c.r64(2);
            c.w64(8, r276);
            c.logic(r276, 64);
            let Some(t277) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ac2a); };
            c.logic(t277 & 0x4u64, 8);
            if c.cc_ne() { blk = 530; continue; }
            blk = 256; continue;
        }
        256 => {
            let Some(t278) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ac35); };
            c.w32(8, t278);
            let r279 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r279);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 262; continue; }
            blk = 257; continue;
        }
        257 => {
            let Some(t280) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ac41); };
            c.w32(9, t280);
            let r281 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r281);
            let Some(t282) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ac48); };
            c.cmp(t282, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 258; continue;
        }
        258 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081ac54); }
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
            let Some(t283) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081ac77); };
            c.w64(12, t283);
            let r284 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r284);
            let Some(t285) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081ac81); };
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
            let Some(t286) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ac9c); };
            c.w32(12, t286);
            let Some(t287) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081aca0); };
            c.cmp(t287, c.r32(12), 32);
            if c.cc_ne() { blk = 438; continue; }
            blk = 265; continue;
        }
        265 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700081acad); }
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
            let Some(t289) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081acc3); };
            c.w32(12, t289);
            let r290 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r290);
            let Some(t291) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081acca); };
            c.cmp(t291, 0x834u64, 16);
            if c.cc_e() { blk = 269; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t292) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081acd4); };
            c.w32(12, t292);
            blk = 270; continue;
        }
        269 => {
            let Some(t293) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081acda); };
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
            let Some(t294) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081aced); };
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
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081ad0a); }
            blk = 437; continue;
        }
        273 => {
            c.w64(9, c.r64(2));
            let r295 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r295);
            c.logic(r295, 64);
            let Some(t296) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ad20); };
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
            let Some(t297) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081ad34); };
            c.w64(15, t297);
            let Some(t298) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081ad3b); };
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
            let Some(t301) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081ad63); };
            c.w32(7, t301);
            let Some(t302) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081ad66); };
            c.cmp(t302, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t303) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ad73); };
            c.w32(7, t303);
            let r304 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r304);
            let Some(t305) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081ad79); };
            c.cmp(t305, c.r32(7), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 281; continue;
        }
        281 => {
            let Some(t306) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ad86); };
            c.w32(9, t306);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t307) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ad93); };
            c.cmp(t307, 0x834u64, 16);
            if c.cc_e() { blk = 283; continue; }
            blk = 282; continue;
        }
        282 => {
            c.w64(8, c.r64(12));
            blk = 284; continue;
        }
        283 => {
            let Some(t308) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081ada1); };
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
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081adc5); };
            c.w64(15, t309);
            let Some(t310) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081adcc); };
            c.w64(3, t310);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t311) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081addb); };
            c.w64(1, t311);
            let Some(t312) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081ade2); };
            c.cmp(t312, c.r64(0), 64);
            if c.cc_be() { blk = 466; continue; }
            blk = 287; continue;
        }
        287 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x700081adeb); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 288; continue;
        }
        288 => {
            let r313 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r313);
            let Some(t314) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081adf5); };
            c.w64(15, t314);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081adfc); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081ae00); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 291; continue; }
            blk = 289; continue;
        }
        289 => {
            let Some(t315) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081ae12); };
            c.w64(12, t315);
            blk = 290; continue;
        }
        290 => {
            let r316 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r316);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081ae1d); }
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
            let Some(t317) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081ae41); };
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
            let Some(t320) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081ae52); };
            c.cmp(t320, c.r64(12), 64);
            if c.cc_e() { blk = 299; continue; }
            blk = 295; continue;
        }
        295 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081ae58); }
            let Some(t321) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081ae5e); };
            c.w64(15, t321);
            let Some(t322) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081ae65); };
            c.w64(0, t322);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081ae70); };
            c.w64(11, t323);
            let Some(t324) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700081ae77); };
            c.cmp(t324, c.r64(3), 64);
            if c.cc_be() { blk = 468; continue; }
            blk = 296; continue;
        }
        296 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700081ae80); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 297; continue;
        }
        297 => {
            let Some(t325) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081ae87); };
            c.w64(15, t325);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081ae8e); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081ae92); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x700081ae98); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 299; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r326 = c.r64(15) & c.r64(1);
            c.w64(15, r326);
            c.logic(r326, 64);
            let Some(t327) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081aeac); };
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
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081aebd); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 302; continue; }
            blk = 301; continue;
        }
        301 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r328 = c.r64(7) & c.r64(2);
            c.w64(7, r328);
            c.logic(r328, 64);
            let Some(t329) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081aecf); };
            c.logic(t329 & 0x4u64, 8);
            if c.cc_ne() { blk = 524; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081aed9); };
            c.w64(1, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081aedd); };
            c.w64(0, t331);
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081aee1); };
            c.w64(11, t332);
            let Some(t333) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081aee5); };
            c.w64(6, t333);
            blk = 303; continue;
        }
        303 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081aee9); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r334 = c.r64(7) & c.r64(2);
            c.w64(7, r334);
            c.logic(r334, 64);
            let Some(t335) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081aef7); };
            c.logic(t335 & 0x4u64, 8);
            if c.cc_ne() { blk = 521; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t336) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081af01); };
            c.w32(7, t336);
            let r337 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r337);
            let Some(t338) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081af07); };
            c.w32(8, t338);
            let Some(t339) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081af0b); };
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
            let Some(t340) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081af1d); };
            c.w32(8, t340);
            let r341 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r341);
            let Some(t342) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081af24); };
            c.cmp(t342, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 307; continue;
        }
        307 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081af30); }
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
            let Some(t343) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081af53); };
            c.w64(9, t343);
            let r344 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r344);
            let Some(t345) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081af5d); };
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
            let Some(t346) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081af74); };
            c.w32(9, t346);
            let r347 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r347);
            let Some(t348) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081af7b); };
            c.cmp(t348, 0x834u64, 16);
            if c.cc_e() { blk = 314; continue; }
            blk = 313; continue;
        }
        313 => {
            let Some(t349) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081af84); };
            c.w32(9, t349);
            blk = 315; continue;
        }
        314 => {
            let Some(t350) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081af8a); };
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
            let Some(t351) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081af9d); };
            c.w32(1, t351);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 472; continue; }
            blk = 317; continue;
        }
        317 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081afb7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 318; continue;
        }
        318 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r352 = c.r64(8) & c.r64(7);
            c.w64(8, r352);
            c.logic(r352, 64);
            let Some(t353) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081afc8); };
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
            let Some(t355) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081afe2); };
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
            let Some(t356) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081aff6); };
            c.w64(15, t356);
            let Some(t357) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081affd); };
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
            let Some(t360) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081b025); };
            c.w32(7, t360);
            let Some(t361) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081b028); };
            c.cmp(t361, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t362) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b035); };
            c.w32(7, t362);
            let r363 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r363);
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081b03b); };
            c.cmp(t364, c.r32(7), 32);
            if c.cc_e() { blk = 350; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t365) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b048); };
            c.w32(9, t365);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t366) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b055); };
            c.cmp(t366, 0x834u64, 16);
            if c.cc_e() { blk = 330; continue; }
            blk = 329; continue;
        }
        329 => {
            c.w64(8, c.r64(12));
            blk = 331; continue;
        }
        330 => {
            let Some(t367) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081b063); };
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
            let Some(t368) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081b087); };
            c.w64(15, t368);
            let Some(t369) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081b08e); };
            c.w64(3, t369);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t370) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081b09d); };
            c.w64(1, t370);
            let Some(t371) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081b0a4); };
            c.cmp(t371, c.r64(0), 64);
            if c.cc_be() { blk = 473; continue; }
            blk = 334; continue;
        }
        334 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x700081b0ad); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 335; continue;
        }
        335 => {
            let r372 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r372);
            let Some(t373) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081b0b7); };
            c.w64(15, t373);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081b0be); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081b0c2); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 338; continue; }
            blk = 336; continue;
        }
        336 => {
            let Some(t374) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081b0d4); };
            c.w64(12, t374);
            blk = 337; continue;
        }
        337 => {
            let r375 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r375);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081b0df); }
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
            let Some(t376) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081b103); };
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
            let Some(t379) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081b114); };
            c.cmp(t379, c.r64(12), 64);
            if c.cc_e() { blk = 346; continue; }
            blk = 342; continue;
        }
        342 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081b11a); }
            let Some(t380) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081b120); };
            c.w64(15, t380);
            let Some(t381) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081b127); };
            c.w64(0, t381);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t382) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081b132); };
            c.w64(11, t382);
            let Some(t383) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700081b139); };
            c.cmp(t383, c.r64(3), 64);
            if c.cc_be() { blk = 475; continue; }
            blk = 343; continue;
        }
        343 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700081b142); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 344; continue;
        }
        344 => {
            let Some(t384) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081b149); };
            c.w64(15, t384);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081b150); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081b154); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x700081b15a); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 346; continue; }
            blk = 345; continue;
        }
        345 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r385 = c.r64(15) & c.r64(1);
            c.w64(15, r385);
            c.logic(r385, 64);
            let Some(t386) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b16e); };
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
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081b17f); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r387 = c.r64(7) & c.r64(2);
            c.w64(7, r387);
            c.logic(r387, 64);
            let Some(t388) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b191); };
            c.logic(t388 & 0x4u64, 8);
            if c.cc_ne() { blk = 512; continue; }
            blk = 349; continue;
        }
        349 => {
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081b19b); };
            c.w64(1, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081b19f); };
            c.w64(0, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b1a3); };
            c.w64(11, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081b1a7); };
            c.w64(6, t392);
            blk = 350; continue;
        }
        350 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081b1ab); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r393 = c.r64(7) & c.r64(2);
            c.w64(7, r393);
            c.logic(r393, 64);
            let Some(t394) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b1b9); };
            c.logic(t394 & 0x4u64, 8);
            if c.cc_ne() { blk = 509; continue; }
            blk = 351; continue;
        }
        351 => {
            let Some(t395) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b1c3); };
            c.w32(7, t395);
            let r396 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r396);
            let Some(t397) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b1c9); };
            c.w32(8, t397);
            let Some(t398) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081b1cd); };
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
            let Some(t399) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b1df); };
            c.w32(8, t399);
            let r400 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r400);
            let Some(t401) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b1e6); };
            c.cmp(t401, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 354; continue;
        }
        354 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081b1f2); }
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
            let Some(t402) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081b215); };
            c.w64(9, t402);
            let r403 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r403);
            let Some(t404) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081b21f); };
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
            let Some(t405) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b236); };
            c.w32(9, t405);
            let r406 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r406);
            let Some(t407) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b23d); };
            c.cmp(t407, 0x834u64, 16);
            if c.cc_e() { blk = 361; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t408) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b246); };
            c.w32(9, t408);
            blk = 362; continue;
        }
        361 => {
            let Some(t409) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081b24c); };
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
            let Some(t410) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b25f); };
            c.w32(1, t410);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 479; continue; }
            blk = 364; continue;
        }
        364 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081b279); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 366; continue; }
            blk = 365; continue;
        }
        365 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r411 = c.r64(8) & c.r64(7);
            c.w64(8, r411);
            c.logic(r411, 64);
            let Some(t412) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b28a); };
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
            let Some(t414) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b2a4); };
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
            let Some(t415) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081b2b8); };
            c.w64(15, t415);
            let Some(t416) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081b2bf); };
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
            let Some(t419) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081b2e7); };
            c.w32(7, t419);
            let Some(t420) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081b2ea); };
            c.cmp(t420, c.r32(7), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t421) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b2f7); };
            c.w32(7, t421);
            let r422 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r422);
            let Some(t423) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081b2fd); };
            c.cmp(t423, c.r32(7), 32);
            if c.cc_e() { blk = 397; continue; }
            blk = 375; continue;
        }
        375 => {
            let Some(t424) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b30a); };
            c.w32(9, t424);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t425) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b317); };
            c.cmp(t425, 0x834u64, 16);
            if c.cc_e() { blk = 377; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(8, c.r64(12));
            blk = 378; continue;
        }
        377 => {
            let Some(t426) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081b325); };
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
            let Some(t427) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081b349); };
            c.w64(15, t427);
            let Some(t428) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081b350); };
            c.w64(3, t428);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t429) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081b35f); };
            c.w64(1, t429);
            let Some(t430) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081b366); };
            c.cmp(t430, c.r64(0), 64);
            if c.cc_be() { blk = 480; continue; }
            blk = 381; continue;
        }
        381 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x700081b36f); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 382; continue;
        }
        382 => {
            let r431 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r431);
            let Some(t432) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081b379); };
            c.w64(15, t432);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081b380); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081b384); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 385; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081b396); };
            c.w64(12, t433);
            blk = 384; continue;
        }
        384 => {
            let r434 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r434);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081b3a1); }
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
            let Some(t435) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081b3c5); };
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
            let Some(t438) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081b3d6); };
            c.cmp(t438, c.r64(12), 64);
            if c.cc_e() { blk = 393; continue; }
            blk = 389; continue;
        }
        389 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081b3dc); }
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081b3e2); };
            c.w64(15, t439);
            let Some(t440) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081b3e9); };
            c.w64(0, t440);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t441) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081b3f4); };
            c.w64(11, t441);
            let Some(t442) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700081b3fb); };
            c.cmp(t442, c.r64(3), 64);
            if c.cc_be() { blk = 482; continue; }
            blk = 390; continue;
        }
        390 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700081b404); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 391; continue;
        }
        391 => {
            let Some(t443) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081b40b); };
            c.w64(15, t443);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081b412); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081b416); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x700081b41c); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 393; continue; }
            blk = 392; continue;
        }
        392 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r444 = c.r64(15) & c.r64(1);
            c.w64(15, r444);
            c.logic(r444, 64);
            let Some(t445) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b430); };
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
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081b441); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 396; continue; }
            blk = 395; continue;
        }
        395 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r446 = c.r64(7) & c.r64(2);
            c.w64(7, r446);
            c.logic(r446, 64);
            let Some(t447) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b453); };
            c.logic(t447 & 0x4u64, 8);
            if c.cc_ne() { blk = 500; continue; }
            blk = 396; continue;
        }
        396 => {
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081b45d); };
            c.w64(1, t448);
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081b461); };
            c.w64(0, t449);
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b465); };
            c.w64(11, t450);
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081b469); };
            c.w64(6, t451);
            blk = 397; continue;
        }
        397 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081b46d); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r452 = c.r64(7) & c.r64(2);
            c.w64(7, r452);
            c.logic(r452, 64);
            let Some(t453) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b47b); };
            c.logic(t453 & 0x4u64, 8);
            if c.cc_ne() { blk = 497; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t454) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b485); };
            c.w32(7, t454);
            let r455 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r455);
            let Some(t456) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b48b); };
            c.w32(8, t456);
            let Some(t457) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081b48f); };
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
            let Some(t458) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b4a1); };
            c.w32(8, t458);
            let r459 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r459);
            let Some(t460) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b4a8); };
            c.cmp(t460, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 401; continue;
        }
        401 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081b4b4); }
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
            let Some(t461) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081b4d7); };
            c.w64(9, t461);
            let r462 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r462);
            let Some(t463) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081b4e1); };
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
            let Some(t464) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b4f8); };
            c.w32(9, t464);
            let r465 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r465);
            let Some(t466) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b4ff); };
            c.cmp(t466, 0x834u64, 16);
            if c.cc_e() { blk = 408; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t467) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b508); };
            c.w32(9, t467);
            blk = 409; continue;
        }
        408 => {
            let Some(t468) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081b50e); };
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
            let Some(t469) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b521); };
            c.w32(1, t469);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 486; continue; }
            blk = 411; continue;
        }
        411 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081b53b); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 413; continue; }
            blk = 412; continue;
        }
        412 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r470 = c.r64(8) & c.r64(7);
            c.w64(8, r470);
            c.logic(r470, 64);
            let Some(t471) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b54c); };
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
            let Some(t473) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b566); };
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
            let Some(t474) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081b57a); };
            c.w64(12, t474);
            let Some(t475) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081b581); };
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
            let Some(t478) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081b5aa); };
            c.w32(8, t478);
            let Some(t479) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081b5ae); };
            c.cmp(t479, c.r32(8), 32);
            if c.cc_e() { blk = 438; continue; }
            blk = 421; continue;
        }
        421 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081b5bb); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r480 = c.r64(8) & c.r64(2);
            c.w64(8, r480);
            c.logic(r480, 64);
            let Some(t481) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b5c9); };
            c.logic(t481 & 0x4u64, 8);
            if c.cc_ne() { blk = 491; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t482) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b5d4); };
            c.w32(8, t482);
            let r483 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r483);
            let Some(t484) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b5db); };
            c.w32(9, t484);
            let Some(t485) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081b5df); };
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
            let Some(t486) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b5f1); };
            c.w32(9, t486);
            let r487 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r487);
            let Some(t488) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b5f8); };
            c.cmp(t488, 0x42u64, 16);
            if c.cc_ne() { blk = 438; continue; }
            blk = 425; continue;
        }
        425 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081b604); }
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
            let Some(t489) = c.ld64(0x7000819f9cu64) else { return RegfileExit::fault(0x700081b627); };
            c.w64(12, t489);
            let r490 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r490);
            let Some(t491) = c.ld64(0x7000819fa9u64) else { return RegfileExit::fault(0x700081b631); };
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
            let Some(t492) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081b645); };
            c.w32(12, t492);
            let r493 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r493);
            let Some(t494) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081b64c); };
            c.cmp(t494, 0x834u64, 16);
            if c.cc_e() { blk = 432; continue; }
            blk = 431; continue;
        }
        431 => {
            let Some(t495) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b656); };
            c.w32(12, t495);
            blk = 433; continue;
        }
        432 => {
            let Some(t496) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081b65c); };
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
            let Some(t497) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081b66b); };
            c.w32(1, t497);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 487; continue; }
            blk = 435; continue;
        }
        435 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081b686); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 437; continue; }
            blk = 436; continue;
        }
        436 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r498 = c.r64(7) & c.r64(8);
            c.w64(7, r498);
            c.logic(r498, 64);
            let Some(t499) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081b698); };
            c.logic(t499 & 0x4u64, 8);
            if c.cc_ne() { blk = 488; continue; }
            blk = 437; continue;
        }
        437 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081b6a5); }
            return c.ret(0x8, 0x700081b6a6);
        }
        438 => {
            let Some(t500) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700081b6a9); };
            c.w64(5, t500);
            let Some(t501) = c.ld64(c.r64(13).wrapping_add(0x2010)) else { return RegfileExit::fault(0x700081b6ad); };
            c.w64(3, t501);
            let Some(t502) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x700081b6b4); };
            c.w64(10, t502);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x700081b6b9); }
            let Some(t503) = c.ld64(c.r64(4).wrapping_add(0x78)) else { return RegfileExit::fault(0x700081b6be); };
            c.w64(10, t503);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x700081b6c3); }
            if !c.st64(c.r64(4).wrapping_add(0x78), c.r64(2)) { return RegfileExit::fault(0x700081b6c8); }
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(1)) { return RegfileExit::fault(0x700081b6cd); }
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(0)) { return RegfileExit::fault(0x700081b6d2); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(11)) { return RegfileExit::fault(0x700081b6d7); }
            let Some(t504) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700081b6dc); };
            c.w64(10, t504);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x700081b6e1); }
            c.w32(0, 0x6u64);
            let r505 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r505);
            return RegfileExit::jump(0x700086ed74);
        }
        439 => {
            return RegfileExit::fault(0x700081b6f4);
        }
        440 => {
            return RegfileExit::fault(0x700081b6f5);
        }
        441 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t506) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081b6f8); };
            c.w64(1, t506);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700081b709, 0x700081b704);
        }
        442 => {
            return RegfileExit::fault(0x700081b709);
        }
        443 => {
            let r507 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r507);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081b70d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081b711); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b712); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700081b716); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b71a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b71e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081b722); }
            c.w32(0, 0x2u64);
            let Some(t508) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081b72b); };
            c.w64(3, t508);
            return c.call(0x700086ed74, 0x700081b737, 0x700081b732);
        }
        444 => {
            c.w64(15, c.r64(0));
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b73a); };
            c.w64(2, t509);
            let Some(t510) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081b73e); };
            c.w64(1, t510);
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b742); };
            c.w64(11, t511);
            let Some(t512) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081b746); };
            c.w64(7, t512);
            let Some(t513) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b74a); };
            c.w64(8, t513);
            let Some(t514) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b74e); };
            c.w64(9, t514);
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b752); };
            c.w64(12, t515);
            blk = 58; continue;
        }
        445 => {
            let Some(t516) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081b75b); };
            c.w64(3, t516);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081b76b, 0x700081b766);
        }
        446 => {
            return RegfileExit::fault(0x700081b76b);
        }
        447 => {
            return RegfileExit::fault(0x700081b76c);
        }
        448 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t517) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081b76f); };
            c.w64(1, t517);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700081b780, 0x700081b77b);
        }
        449 => {
            return RegfileExit::fault(0x700081b780);
        }
        450 => {
            let r518 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r518);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081b784); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081b788); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b789); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700081b78d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b791); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b795); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081b799); }
            let Some(t519) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081b79d); };
            c.w64(3, t519);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700081b7ae, 0x700081b7a9);
        }
        451 => {
            c.w64(15, c.r64(0));
            let Some(t520) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b7b1); };
            c.w64(2, t520);
            let Some(t521) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081b7b5); };
            c.w64(1, t521);
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b7b9); };
            c.w64(11, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081b7bd); };
            c.w64(7, t523);
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b7c1); };
            c.w64(8, t524);
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b7c5); };
            c.w64(9, t525);
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b7c9); };
            c.w64(12, t526);
            blk = 104; continue;
        }
        452 => {
            let Some(t527) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081b7d2); };
            c.w64(3, t527);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081b7e2, 0x700081b7dd);
        }
        453 => {
            return RegfileExit::fault(0x700081b7e2);
        }
        454 => {
            return RegfileExit::fault(0x700081b7e3);
        }
        455 => {
            return RegfileExit::fault(0x700081b7e4);
        }
        456 => {
            return RegfileExit::fault(0x700081b7e5);
        }
        457 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t528) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081b7e8); };
            c.w64(1, t528);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700081b7f9, 0x700081b7f4);
        }
        458 => {
            return RegfileExit::fault(0x700081b7f9);
        }
        459 => {
            let r529 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r529);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081b7fd); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081b801); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b802); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700081b806); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b80a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b80e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081b812); }
            let Some(t530) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081b816); };
            c.w64(3, t530);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700081b827, 0x700081b822);
        }
        460 => {
            c.w64(15, c.r64(0));
            let Some(t531) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b82a); };
            c.w64(2, t531);
            let Some(t532) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081b82e); };
            c.w64(1, t532);
            let Some(t533) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b832); };
            c.w64(11, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081b836); };
            c.w64(7, t534);
            let Some(t535) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b83a); };
            c.w64(8, t535);
            let Some(t536) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b83e); };
            c.w64(9, t536);
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b842); };
            c.w64(12, t537);
            blk = 196; continue;
        }
        461 => {
            let Some(t538) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081b84b); };
            c.w64(3, t538);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081b85b, 0x700081b856);
        }
        462 => {
            return RegfileExit::fault(0x700081b85b);
        }
        463 => {
            return RegfileExit::fault(0x700081b85c);
        }
        464 => {
            return RegfileExit::fault(0x700081b85d);
        }
        465 => {
            return RegfileExit::fault(0x700081b85e);
        }
        466 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b85f); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081b863); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b867); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b86b); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081b86f); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700081b87b, 0x700081b876);
        }
        467 => {
            c.w64(1, c.r64(0));
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b87e); };
            c.w64(2, t539);
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b882); };
            c.w64(11, t540);
            let Some(t541) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081b886); };
            c.w64(7, t541);
            let Some(t542) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b88a); };
            c.w64(8, t542);
            let Some(t543) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b88e); };
            c.w64(9, t543);
            let Some(t544) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b892); };
            c.w64(12, t544);
            blk = 288; continue;
        }
        468 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b89b); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700081b8a4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b8a8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b8ac); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700081b8b0); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x700081b8b4); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081b8b9); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700081b8c5, 0x700081b8c0);
        }
        469 => {
            c.w64(11, c.r64(0));
            let Some(t545) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b8c8); };
            c.w64(2, t545);
            let Some(t546) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081b8cc); };
            c.w64(1, t546);
            let Some(t547) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b8d0); };
            c.w64(8, t547);
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b8d4); };
            c.w64(9, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b8d8); };
            c.w64(6, t549);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700081b8dc); }
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081b8e1); };
            c.w64(7, t550);
            let Some(t551) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081b8e5); };
            c.w64(12, t551);
            blk = 297; continue;
        }
        470 => {
            let Some(t552) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081b8f1); };
            c.w64(3, t552);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081b901, 0x700081b8fc);
        }
        471 => {
            return RegfileExit::fault(0x700081b901);
        }
        472 => {
            return RegfileExit::fault(0x700081b902);
        }
        473 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b903); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081b907); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b90b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b90f); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081b913); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700081b91f, 0x700081b91a);
        }
        474 => {
            c.w64(1, c.r64(0));
            let Some(t553) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b922); };
            c.w64(2, t553);
            let Some(t554) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b926); };
            c.w64(11, t554);
            let Some(t555) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081b92a); };
            c.w64(7, t555);
            let Some(t556) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b92e); };
            c.w64(8, t556);
            let Some(t557) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b932); };
            c.w64(9, t557);
            let Some(t558) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b936); };
            c.w64(12, t558);
            blk = 335; continue;
        }
        475 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b93f); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700081b948); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b94c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b950); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700081b954); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x700081b958); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081b95d); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700081b969, 0x700081b964);
        }
        476 => {
            c.w64(11, c.r64(0));
            let Some(t559) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b96c); };
            c.w64(2, t559);
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081b970); };
            c.w64(1, t560);
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b974); };
            c.w64(8, t561);
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b978); };
            c.w64(9, t562);
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b97c); };
            c.w64(6, t563);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700081b980); }
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081b985); };
            c.w64(7, t564);
            let Some(t565) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081b989); };
            c.w64(12, t565);
            blk = 344; continue;
        }
        477 => {
            let Some(t566) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081b995); };
            c.w64(3, t566);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081b9a5, 0x700081b9a0);
        }
        478 => {
            return RegfileExit::fault(0x700081b9a5);
        }
        479 => {
            return RegfileExit::fault(0x700081b9a6);
        }
        480 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b9a7); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081b9ab); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b9af); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b9b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081b9b7); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700081b9c3, 0x700081b9be);
        }
        481 => {
            c.w64(1, c.r64(0));
            let Some(t567) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081b9c6); };
            c.w64(2, t567);
            let Some(t568) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081b9ca); };
            c.w64(11, t568);
            let Some(t569) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081b9ce); };
            c.w64(7, t569);
            let Some(t570) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081b9d2); };
            c.w64(8, t570);
            let Some(t571) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081b9d6); };
            c.w64(9, t571);
            let Some(t572) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081b9da); };
            c.w64(12, t572);
            blk = 382; continue;
        }
        482 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081b9e3); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700081b9ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081b9f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081b9f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700081b9f8); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x700081b9fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081ba01); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700081ba0d, 0x700081ba08);
        }
        483 => {
            c.w64(11, c.r64(0));
            let Some(t573) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081ba10); };
            c.w64(2, t573);
            let Some(t574) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081ba14); };
            c.w64(1, t574);
            let Some(t575) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081ba18); };
            c.w64(8, t575);
            let Some(t576) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081ba1c); };
            c.w64(9, t576);
            let Some(t577) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081ba20); };
            c.w64(6, t577);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700081ba24); }
            let Some(t578) = c.ld64(0x700081a170u64) else { return RegfileExit::fault(0x700081ba29); };
            c.w64(12, t578);
            let Some(t579) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081ba30); };
            c.w64(7, t579);
            blk = 391; continue;
        }
        484 => {
            let Some(t580) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081ba39); };
            c.w64(3, t580);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081ba49, 0x700081ba44);
        }
        485 => {
            return RegfileExit::fault(0x700081ba49);
        }
        486 => {
            return RegfileExit::fault(0x700081ba4a);
        }
        487 => {
            return RegfileExit::fault(0x700081ba4b);
        }
        488 => {
            c.w32(0, c.r32(0));
            let r581 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r581);
            c.w64(7, 0xfffffffffffc0000u64);
            let r582 = c.r64(7) & c.r64(0);
            c.w64(7, r582);
            c.logic(r582, 64);
            let Some(t583) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ba5b); };
            c.logic(t583 & 0x2u64, 8);
            if c.cc_e() { blk = 437; continue; }
            blk = 489; continue;
        }
        489 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081ba69); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081ba6a); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081ba77, 0x700081ba72);
        }
        490 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081ba77); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081ba79); }
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
            let Some(t586) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ba8f); };
            c.logic(t586 & 0x2u64, 8);
            if c.cc_e() { blk = 422; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081ba9e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081ba9f); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081baac, 0x700081baa7);
        }
        493 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081baac); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081baae); }
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
            let Some(t589) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bac3); };
            c.logic(t589 & 0x2u64, 8);
            if c.cc_e() { blk = 413; continue; }
            blk = 495; continue;
        }
        495 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bad2); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081badc, 0x700081bad7);
        }
        496 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081badc); }
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
            let Some(t592) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081baf3); };
            c.logic(t592 & 0x2u64, 8);
            if c.cc_e() { blk = 398; continue; }
            blk = 498; continue;
        }
        498 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bb01); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081bb0a, 0x700081bb05);
        }
        499 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bb0a); }
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
            let Some(t595) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bb1f); };
            c.logic(t595 & 0x2u64, 8);
            if c.cc_e() { blk = 396; continue; }
            blk = 501; continue;
        }
        501 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bb2d); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081bb36, 0x700081bb31);
        }
        502 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bb36); }
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
            let Some(t598) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bb4c); };
            c.logic(t598 & 0x2u64, 8);
            if c.cc_e() { blk = 393; continue; }
            blk = 504; continue;
        }
        504 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bb5b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bb5c); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700081bb69, 0x700081bb64);
        }
        505 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bb69); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bb6b); }
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
            let Some(t601) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bb80); };
            c.logic(t601 & 0x2u64, 8);
            if c.cc_e() { blk = 366; continue; }
            blk = 507; continue;
        }
        507 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bb8f); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081bb99, 0x700081bb94);
        }
        508 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bb99); }
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
            let Some(t604) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bbb0); };
            c.logic(t604 & 0x2u64, 8);
            if c.cc_e() { blk = 351; continue; }
            blk = 510; continue;
        }
        510 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bbbe); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081bbc7, 0x700081bbc2);
        }
        511 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bbc7); }
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
            let Some(t607) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bbdc); };
            c.logic(t607 & 0x2u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 513; continue;
        }
        513 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bbea); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081bbf3, 0x700081bbee);
        }
        514 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bbf3); }
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
            let Some(t610) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bc09); };
            c.logic(t610 & 0x2u64, 8);
            if c.cc_e() { blk = 346; continue; }
            blk = 516; continue;
        }
        516 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bc18); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bc19); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700081bc26, 0x700081bc21);
        }
        517 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bc26); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bc28); }
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
            let Some(t613) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bc3d); };
            c.logic(t613 & 0x2u64, 8);
            if c.cc_e() { blk = 319; continue; }
            blk = 519; continue;
        }
        519 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bc4c); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081bc56, 0x700081bc51);
        }
        520 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bc56); }
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
            let Some(t616) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bc6d); };
            c.logic(t616 & 0x2u64, 8);
            if c.cc_e() { blk = 304; continue; }
            blk = 522; continue;
        }
        522 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bc7b); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081bc84, 0x700081bc7f);
        }
        523 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bc84); }
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
            let Some(t619) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bc99); };
            c.logic(t619 & 0x2u64, 8);
            if c.cc_e() { blk = 302; continue; }
            blk = 525; continue;
        }
        525 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bca7); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081bcb0, 0x700081bcab);
        }
        526 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bcb0); }
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
            let Some(t622) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bcc6); };
            c.logic(t622 & 0x2u64, 8);
            if c.cc_e() { blk = 299; continue; }
            blk = 528; continue;
        }
        528 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bcd5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bcd6); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700081bce3, 0x700081bcde);
        }
        529 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bce3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bce5); }
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
            let Some(t625) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bcfb); };
            c.logic(t625 & 0x2u64, 8);
            if c.cc_e() { blk = 256; continue; }
            blk = 531; continue;
        }
        531 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bd0a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bd0b); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081bd18, 0x700081bd13);
        }
        532 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bd18); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bd1a); }
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
            let Some(t628) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bd2f); };
            c.logic(t628 & 0x2u64, 8);
            if c.cc_e() { blk = 247; continue; }
            blk = 534; continue;
        }
        534 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bd3d); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bd3e); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081bd4b, 0x700081bd46);
        }
        535 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bd4b); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bd4d); }
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
            let Some(t631) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bd63); };
            c.logic(t631 & 0x2u64, 8);
            if c.cc_e() { blk = 232; continue; }
            blk = 537; continue;
        }
        537 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bd72); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bd73); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081bd80, 0x700081bd7b);
        }
        538 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bd80); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bd82); }
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
            let Some(t634) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bd98); };
            c.logic(t634 & 0x2u64, 8);
            if c.cc_e() { blk = 207; continue; }
            blk = 540; continue;
        }
        540 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bda6); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081bdaf, 0x700081bdaa);
        }
        541 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bdaf); }
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
            let Some(t637) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bdc5); };
            c.logic(t637 & 0x2u64, 8);
            if c.cc_e() { blk = 205; continue; }
            blk = 543; continue;
        }
        543 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bdd3); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081bddc, 0x700081bdd7);
        }
        544 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bddc); }
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
            let Some(t640) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bdf1); };
            c.logic(t640 & 0x2u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 546; continue;
        }
        546 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bdff); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081be00); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081be0d, 0x700081be08);
        }
        547 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081be0d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081be0f); }
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
            let Some(t643) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081be25); };
            c.logic(t643 & 0x2u64, 8);
            if c.cc_e() { blk = 164; continue; }
            blk = 549; continue;
        }
        549 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081be34); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081be35); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081be42, 0x700081be3d);
        }
        550 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081be42); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081be44); }
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
            let Some(t646) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081be59); };
            c.logic(t646 & 0x2u64, 8);
            if c.cc_e() { blk = 155; continue; }
            blk = 552; continue;
        }
        552 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081be67); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081be68); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081be75, 0x700081be70);
        }
        553 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081be75); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081be77); }
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
            let Some(t649) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081be8d); };
            c.logic(t649 & 0x2u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 555; continue;
        }
        555 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081be9c); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081be9d); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081beaa, 0x700081bea5);
        }
        556 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081beaa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081beac); }
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
            let Some(t652) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bec2); };
            c.logic(t652 & 0x2u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 558; continue;
        }
        558 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bed0); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081bed9, 0x700081bed4);
        }
        559 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bed9); }
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
            let Some(t655) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081beef); };
            c.logic(t655 & 0x2u64, 8);
            if c.cc_e() { blk = 113; continue; }
            blk = 561; continue;
        }
        561 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081befd); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081bf06, 0x700081bf01);
        }
        562 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bf06); }
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
            let Some(t658) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bf1c); };
            c.logic(t658 & 0x2u64, 8);
            if c.cc_e() { blk = 69; continue; }
            blk = 564; continue;
        }
        564 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bf2a); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081bf33, 0x700081bf2e);
        }
        565 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bf33); }
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
            let Some(t661) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bf49); };
            c.logic(t661 & 0x2u64, 8);
            if c.cc_e() { blk = 67; continue; }
            blk = 567; continue;
        }
        567 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bf57); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081bf60, 0x700081bf5b);
        }
        568 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bf60); }
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
            let Some(t664) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081bf76); };
            c.logic(t664 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 570; continue;
        }
        570 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081bf85); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081bf86); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081bf93, 0x700081bf8e);
        }
        571 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081bf93); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081bf95); }
            blk = 27; continue;
        }
        572 => {
            return RegfileExit::jump(0x700081bf9c);
        }
            _ => unreachable!(),
        }
    }
}
