#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_700081bfb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081bfb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700081bfb8); }
            let r0 = c.sub(c.r64(4), 0x60u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081bfbe); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700081bfc5); };
            c.w32(9, t3);
            let Some(t4) = c.ld8(c.r64(11).wrapping_add(0xa)) else { return RegfileExit::fault(0x700081bfca); };
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
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700081bfe6); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x700081bfea); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x700081bfee); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700081bff2); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(6)) { return RegfileExit::fault(0x700081bff6); }
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
            if c.cc_e() { blk = 380; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(12), 0x405u64, 32);
            if c.cc_e() { blk = 296; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(12), 0x503u64, 32);
            if c.cc_e() { blk = 439; continue; }
            blk = 5; continue;
        }
        5 => {
            blk = 535; continue;
        }
        6 => {
            c.cmp(c.r32(12), 0x105u64, 32);
            if c.cc_e() { blk = 216; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(12), 0x203u64, 32);
            if c.cc_e() { blk = 498; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(12), 0x402u64, 32);
            if c.cc_e() { blk = 321; continue; }
            blk = 9; continue;
        }
        9 => {
            blk = 535; continue;
        }
        10 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_l() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(12), 0x4u64, 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r32(12), 0x5u64, 32);
            if c.cc_e() { blk = 100; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(12), 0x103u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 535; continue;
        }
        15 => {
            c.cmp(c.r32(12), 0x1u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(12), 0x2u64, 32);
            if c.cc_e() { blk = 144; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r32(12), 0x3u64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 535; continue;
        }
        19 => {
            c.w64(8, c.r64(2));
            let r11 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r11);
            c.logic(r11, 64);
            let Some(t12) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c0b9); };
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
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c0cd); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081c0d4); };
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
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081c0fd); };
            c.w32(8, t17);
            let Some(t18) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081c101); };
            c.cmp(t18, c.r32(8), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081c10e); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r19 = c.r64(8) & c.r64(2);
            c.w64(8, r19);
            c.logic(r19, 64);
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c11c); };
            c.logic(t20 & 0x4u64, 8);
            if c.cc_ne() { blk = 721; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c127); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 33; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t23) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c133); };
            c.w32(9, t23);
            let r24 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r24);
            let Some(t25) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c13a); };
            c.cmp(t25, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 29; continue;
        }
        29 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c146); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 30; continue;
        }
        30 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 31; continue;
        }
        31 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r26 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r26);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            blk = 534; continue;
        }
        33 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 34; continue;
        }
        34 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 534; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t27) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c198); };
            c.w32(12, t27);
            let r28 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r28);
            let Some(t29) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c19f); };
            c.cmp(t29, 0x834u64, 16);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c1a9); };
            c.w32(12, t30);
            blk = 38; continue;
        }
        37 => {
            let Some(t31) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081c1af); };
            c.w32(12, t31);
            blk = 38; continue;
        }
        38 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c1c5); };
            c.w32(15, t32);
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081c1c9); };
            c.cmp(t33, c.r32(15), 32);
            if c.cc_ne() { blk = 52; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t34) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c1d6); };
            c.w32(15, t34);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 539; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t35) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c1fd); };
            c.w64(1, t35);
            let Some(t36) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081c204); };
            c.w64(7, t36);
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081c213); };
            c.w64(0, t37);
            let Some(t38) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081c21a); };
            c.cmp(t38, c.r64(6), 64);
            if c.cc_be() { blk = 536; continue; }
            blk = 43; continue;
        }
        43 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081c223); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 44; continue;
        }
        44 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081c22d); };
            c.w64(11, t39);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081c234); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700081c238); }
            c.w64(7, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_e() { blk = 47; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081c250); };
            c.w64(12, t40);
            blk = 46; continue;
        }
        46 => {
            let r41 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r41);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081c25b); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ne() { blk = 46; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 49; continue; }
            blk = 48; continue;
        }
        48 => {
            let r42 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r42);
            let Some(t43) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700081c26e); };
            c.w32(11, t43);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081c272); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 48; continue; }
            blk = 49; continue;
        }
        49 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081c27c); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r44 = c.r64(7) & c.r64(2);
            c.w64(7, r44);
            c.logic(r44, 64);
            let Some(t45) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c28e); };
            c.logic(t45 & 0x4u64, 8);
            if c.cc_ne() { blk = 718; continue; }
            blk = 51; continue;
        }
        51 => {
            c.w64(8, c.r64(1));
            blk = 52; continue;
        }
        52 => {
            let Some(t46) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c29b); };
            c.w32(1, t46);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 538; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081c2ae); };
            c.w64(0, t47);
            if !c.st32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7), c.r32(0)) { return RegfileExit::fault(0x700081c2b2); }
            blk = 533; continue;
        }
        54 => {
            c.w64(9, c.r64(2));
            let r48 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r48);
            c.logic(r48, 64);
            let Some(t49) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c2c6); };
            c.w64(12, t49);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 61; continue; }
            blk = 55; continue;
        }
        55 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 61; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t50) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c2da); };
            c.w64(15, t50);
            let Some(t51) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081c2e1); };
            c.w64(15, t51);
            c.w64(3, c.r64(12));
            let r52 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r52);
            c.logic(r52, 64);
            c.w64(7, c.r64(15));
            let r53 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r53);
            c.logic(r53, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 59; continue; }
            blk = 57; continue;
        }
        57 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            blk = 61; continue;
        }
        59 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t54) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081c309); };
            c.w32(7, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081c30c); };
            c.cmp(t55, c.r32(7), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t56) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c319); };
            c.w32(7, t56);
            let r57 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r57);
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081c31f); };
            c.cmp(t58, c.r32(7), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c32c); };
            c.w32(9, t59);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t60) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c339); };
            c.cmp(t60, 0x834u64, 16);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(8, c.r64(12));
            blk = 65; continue;
        }
        64 => {
            let Some(t61) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081c347); };
            c.w32(8, t61);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 66; continue;
        }
        66 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 545; continue; }
            blk = 67; continue;
        }
        67 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 541; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t62) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c380); };
            c.w64(3, t62);
            let Some(t63) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081c387); };
            c.w64(6, t63);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t64) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c38e); };
            c.cmp(t64, c.r64(0), 64);
            if c.cc_be() { blk = 543; continue; }
            blk = 69; continue;
        }
        69 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081c398); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 70; continue;
        }
        70 => {
            let r65 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r65);
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700081c3a2); };
            c.w64(0, t66);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081c3a9); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081c3ad); }
            let Some(t67) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081c3b1); };
            c.w64(12, t67);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c3b8); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let r68 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r68);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081c3d8); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 71; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 77; continue; }
            blk = 73; continue;
        }
        73 => {
            let r69 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r69);
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081c3f5); };
            c.w32(9, t70);
            let r71 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r71);
            let Some(t72) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081c3fd); };
            c.cmp(t72, c.r32(9), 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            let r73 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r73);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081c412); }
            blk = 76; continue;
        }
        75 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x700081c424); }
            blk = 76; continue;
        }
        76 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 73; continue; }
            blk = 77; continue;
        }
        77 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081c42e); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r74 = c.r64(7) & c.r64(2);
            c.w64(7, r74);
            c.logic(r74, 64);
            let Some(t75) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c442); };
            c.logic(t75 & 0x4u64, 8);
            if c.cc_ne() { blk = 715; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t76) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081c44c); };
            c.w64(0, t76);
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081c450); };
            c.w64(6, t77);
            blk = 80; continue;
        }
        80 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081c454); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r78 = c.r64(7) & c.r64(2);
            c.w64(7, r78);
            c.logic(r78, 64);
            let Some(t79) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c462); };
            c.logic(t79 & 0x4u64, 8);
            if c.cc_ne() { blk = 712; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t80) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c46c); };
            c.w32(7, t80);
            let r81 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r81);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 87; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t82) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c477); };
            c.w32(8, t82);
            let r83 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r83);
            let Some(t84) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c47e); };
            c.cmp(t84, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 83; continue;
        }
        83 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c48a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 84; continue;
        }
        84 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t85) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081c4ad); };
            c.w64(9, t85);
            let r86 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r86);
            let Some(t87) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081c4b7); };
            c.w64(12, t87);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 88; continue; }
            blk = 86; continue;
        }
        86 => {
            blk = 534; continue;
        }
        87 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 88; continue;
        }
        88 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 92; continue; }
            blk = 89; continue;
        }
        89 => {
            c.w64(9, c.r64(0));
            let Some(t88) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c4d5); };
            c.w32(12, t88);
            let Some(t89) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081c4d9); };
            c.cmp(t89, c.r32(12), 32);
            if c.cc_ne() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c4e2); }
            blk = 93; continue;
        }
        91 => {
            c.w64(0, c.r64(9));
            blk = 534; continue;
        }
        92 => {
            c.w64(12, c.r64(0));
            let r90 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r90);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            c.w64(9, c.r64(0));
            blk = 93; continue;
        }
        93 => {
            let Some(t91) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c504); };
            c.w32(12, t91);
            let r92 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r92);
            let Some(t93) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c50b); };
            c.cmp(t93, 0x834u64, 16);
            if c.cc_e() { blk = 95; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t94) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c515); };
            c.w32(12, t94);
            blk = 96; continue;
        }
        95 => {
            let Some(t95) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081c51b); };
            c.w32(12, t95);
            blk = 96; continue;
        }
        96 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 98; continue; }
            blk = 97; continue;
        }
        97 => {
            c.w64(0, c.r64(9));
            blk = 534; continue;
        }
        98 => {
            let Some(t96) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c532); };
            c.w32(1, t96);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 547; continue; }
            blk = 99; continue;
        }
        99 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081c54e); }
            c.w64(0, c.r64(9));
            blk = 533; continue;
        }
        100 => {
            c.w64(9, c.r64(2));
            let r97 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r97);
            c.logic(r97, 64);
            let Some(t98) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c567); };
            c.w64(12, t98);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 101; continue;
        }
        101 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 107; continue; }
            blk = 102; continue;
        }
        102 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t99) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c57b); };
            c.w64(15, t99);
            let Some(t100) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081c582); };
            c.w64(15, t100);
            c.w64(3, c.r64(12));
            let r101 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r101);
            c.logic(r101, 64);
            c.w64(7, c.r64(15));
            let r102 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r102);
            c.logic(r102, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 105; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            blk = 107; continue;
        }
        105 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 107; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t103) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081c5aa); };
            c.w32(7, t103);
            let Some(t104) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081c5ad); };
            c.cmp(t104, c.r32(7), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t105) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c5ba); };
            c.w32(7, t105);
            let r106 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r106);
            let Some(t107) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081c5c0); };
            c.cmp(t107, c.r32(7), 32);
            if c.cc_e() { blk = 126; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t108) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c5cd); };
            c.w32(9, t108);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t109) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c5da); };
            c.cmp(t109, 0x834u64, 16);
            if c.cc_e() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            c.w64(8, c.r64(12));
            blk = 111; continue;
        }
        110 => {
            let Some(t110) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081c5e8); };
            c.w32(8, t110);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 111; continue;
        }
        111 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 552; continue; }
            blk = 113; continue;
        }
        113 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 548; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c621); };
            c.w64(3, t111);
            let Some(t112) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081c628); };
            c.w64(6, t112);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t113) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c62f); };
            c.cmp(t113, c.r64(0), 64);
            if c.cc_be() { blk = 550; continue; }
            blk = 115; continue;
        }
        115 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081c639); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 116; continue;
        }
        116 => {
            let r114 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r114);
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700081c643); };
            c.w64(0, t115);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081c64a); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081c64e); }
            let Some(t116) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081c652); };
            c.w64(12, t116);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c659); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 118; continue; }
            blk = 117; continue;
        }
        117 => {
            let r117 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r117);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081c679); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 117; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 123; continue; }
            blk = 119; continue;
        }
        119 => {
            let r118 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r118);
            let Some(t119) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081c696); };
            c.w32(9, t119);
            let r120 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081c69e); };
            c.cmp(t121, c.r32(9), 32);
            if c.cc_e() { blk = 121; continue; }
            blk = 120; continue;
        }
        120 => {
            let r122 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r122);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081c6b3); }
            blk = 122; continue;
        }
        121 => {
            let Some(t123) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081c6bb); };
            c.w64(9, t123);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x700081c6c2); }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 119; continue; }
            blk = 123; continue;
        }
        123 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081c6cc); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r124 = c.r64(7) & c.r64(2);
            c.w64(7, r124);
            c.logic(r124, 64);
            let Some(t125) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c6e0); };
            c.logic(t125 & 0x4u64, 8);
            if c.cc_ne() { blk = 709; continue; }
            blk = 125; continue;
        }
        125 => {
            let Some(t126) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081c6ea); };
            c.w64(0, t126);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081c6ee); };
            c.w64(6, t127);
            blk = 126; continue;
        }
        126 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081c6f2); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r128 = c.r64(7) & c.r64(2);
            c.w64(7, r128);
            c.logic(r128, 64);
            let Some(t129) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c700); };
            c.logic(t129 & 0x4u64, 8);
            if c.cc_ne() { blk = 706; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t130) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c70a); };
            c.w32(7, t130);
            let r131 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r131);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 133; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t132) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c715); };
            c.w32(8, t132);
            let r133 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r133);
            let Some(t134) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c71c); };
            c.cmp(t134, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 129; continue;
        }
        129 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c728); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 130; continue;
        }
        130 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t135) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081c74b); };
            c.w64(9, t135);
            let r136 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r136);
            let Some(t137) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081c755); };
            c.w64(12, t137);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 134; continue; }
            blk = 132; continue;
        }
        132 => {
            blk = 534; continue;
        }
        133 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 134; continue;
        }
        134 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t138) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c770); };
            c.w32(9, t138);
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081c774); };
            c.cmp(t139, c.r32(9), 32);
            if c.cc_ne() { blk = 534; continue; }
            blk = 136; continue;
        }
        136 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c781); }
            blk = 138; continue;
        }
        137 => {
            c.w64(9, c.r64(0));
            let r140 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r140);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 138; continue;
        }
        138 => {
            let Some(t141) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c797); };
            c.w32(9, t141);
            let r142 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r142);
            let Some(t143) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c79e); };
            c.cmp(t143, 0x834u64, 16);
            if c.cc_e() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t144) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c7a7); };
            c.w32(9, t144);
            blk = 141; continue;
        }
        140 => {
            let Some(t145) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081c7ad); };
            c.w32(9, t145);
            blk = 141; continue;
        }
        141 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t146) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c7c0); };
            c.w32(1, t146);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 554; continue; }
            blk = 143; continue;
        }
        143 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081c7dc); }
            blk = 533; continue;
        }
        144 => {
            c.w64(8, c.r64(2));
            let r147 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r147);
            c.logic(r147, 64);
            let Some(t148) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c7f2); };
            c.w64(9, t148);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 151; continue; }
            blk = 145; continue;
        }
        145 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 151; continue; }
            blk = 146; continue;
        }
        146 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c806); };
            c.w64(12, t149);
            let Some(t150) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081c80d); };
            c.w64(12, t150);
            c.w64(15, c.r64(9));
            let r151 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r151);
            c.logic(r151, 64);
            c.w64(3, c.r64(12));
            let r152 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r152);
            c.logic(r152, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 149; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 150; continue; }
            blk = 148; continue;
        }
        148 => {
            blk = 151; continue;
        }
        149 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t153) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081c836); };
            c.w32(8, t153);
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081c83a); };
            c.cmp(t154, c.r32(8), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 151; continue;
        }
        151 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081c847); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r155 = c.r64(8) & c.r64(2);
            c.w64(8, r155);
            c.logic(r155, 64);
            let Some(t156) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c855); };
            c.logic(t156 & 0x4u64, 8);
            if c.cc_ne() { blk = 703; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t157) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c860); };
            c.w32(8, t157);
            let r158 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r158);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t159) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c86c); };
            c.w32(9, t159);
            let r160 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r160);
            let Some(t161) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c873); };
            c.cmp(t161, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 154; continue;
        }
        154 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081c87f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 155; continue;
        }
        155 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t162) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081c8a2); };
            c.w64(12, t162);
            let r163 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r163);
            let Some(t164) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081c8ac); };
            c.w64(15, t164);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 159; continue; }
            blk = 157; continue;
        }
        157 => {
            blk = 534; continue;
        }
        158 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 159; continue;
        }
        159 => {
            let Some(t165) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c8c3); };
            c.w32(12, t165);
            let r166 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r166);
            let Some(t167) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081c8ca); };
            c.cmp(t167, 0x834u64, 16);
            if c.cc_e() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t168) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c8d4); };
            c.w32(12, t168);
            blk = 162; continue;
        }
        161 => {
            let Some(t169) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081c8da); };
            c.w32(12, t169);
            blk = 162; continue;
        }
        162 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 163; continue;
        }
        163 => {
            let Some(t170) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081c8f0); };
            c.w32(15, t170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081c8f4); };
            c.cmp(t171, c.r32(15), 32);
            if c.cc_ne() { blk = 176; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t172) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c901); };
            c.w32(15, t172);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 165; continue;
        }
        165 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 558; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t173) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081c928); };
            c.w64(1, t173);
            let Some(t174) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081c92f); };
            c.w64(7, t174);
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t175) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081c93e); };
            c.w64(0, t175);
            let Some(t176) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081c945); };
            c.cmp(t176, c.r64(6), 64);
            if c.cc_be() { blk = 555; continue; }
            blk = 167; continue;
        }
        167 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081c94e); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 168; continue;
        }
        168 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t177) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081c958); };
            c.w64(11, t177);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081c95f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700081c963); }
            c.w64(7, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_e() { blk = 171; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t178) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081c97b); };
            c.w64(12, t178);
            blk = 170; continue;
        }
        170 => {
            let r179 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r179);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081c986); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ne() { blk = 170; continue; }
            blk = 171; continue;
        }
        171 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 173; continue; }
            blk = 172; continue;
        }
        172 => {
            let r180 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r180);
            let Some(t181) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700081c999); };
            c.w32(11, t181);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081c99d); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 172; continue; }
            blk = 173; continue;
        }
        173 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081c9a7); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r182 = c.r64(7) & c.r64(2);
            c.w64(7, r182);
            c.logic(r182, 64);
            let Some(t183) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c9b9); };
            c.logic(t183 & 0x4u64, 8);
            if c.cc_ne() { blk = 700; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w64(8, c.r64(1));
            blk = 176; continue;
        }
        176 => {
            let Some(t184) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081c9c6); };
            c.w32(1, t184);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 557; continue; }
            blk = 177; continue;
        }
        177 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t185) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081c9e1); };
            c.w64(0, t185);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081c9e5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 178; continue;
        }
        178 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r186 = c.r64(7) & c.r64(8);
            c.w64(7, r186);
            c.logic(r186, 64);
            let Some(t187) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081c9f7); };
            c.logic(t187 & 0x4u64, 8);
            if c.cc_ne() { blk = 697; continue; }
            blk = 179; continue;
        }
        179 => {
            blk = 533; continue;
        }
        180 => {
            c.w64(8, c.r64(2));
            let r188 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r188);
            c.logic(r188, 64);
            let Some(t189) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ca10); };
            c.w64(9, t189);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 187; continue; }
            blk = 181; continue;
        }
        181 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 187; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t190) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081ca24); };
            c.w64(12, t190);
            let Some(t191) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081ca2b); };
            c.w64(12, t191);
            c.w64(15, c.r64(9));
            let r192 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r192);
            c.logic(r192, 64);
            c.w64(3, c.r64(12));
            let r193 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r193);
            c.logic(r193, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 185; continue; }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 186; continue; }
            blk = 184; continue;
        }
        184 => {
            blk = 187; continue;
        }
        185 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t194) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081ca54); };
            c.w32(8, t194);
            let Some(t195) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081ca58); };
            c.cmp(t195, c.r32(8), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 187; continue;
        }
        187 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081ca65); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r196 = c.r64(8) & c.r64(2);
            c.w64(8, r196);
            c.logic(r196, 64);
            let Some(t197) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ca73); };
            c.logic(t197 & 0x4u64, 8);
            if c.cc_ne() { blk = 694; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t198) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ca7e); };
            c.w32(8, t198);
            let r199 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r199);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 194; continue; }
            blk = 189; continue;
        }
        189 => {
            let Some(t200) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ca8a); };
            c.w32(9, t200);
            let r201 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r201);
            let Some(t202) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ca91); };
            c.cmp(t202, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 190; continue;
        }
        190 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081ca9d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 191; continue;
        }
        191 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 192; continue;
        }
        192 => {
            let Some(t203) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081cac0); };
            c.w64(12, t203);
            let r204 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r204);
            let Some(t205) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081caca); };
            c.w64(15, t205);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 195; continue; }
            blk = 193; continue;
        }
        193 => {
            blk = 534; continue;
        }
        194 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 195; continue;
        }
        195 => {
            let Some(t206) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081cae1); };
            c.w32(12, t206);
            let r207 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r207);
            let Some(t208) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cae8); };
            c.cmp(t208, 0x834u64, 16);
            if c.cc_e() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t209) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081caf2); };
            c.w32(12, t209);
            blk = 198; continue;
        }
        197 => {
            let Some(t210) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081caf8); };
            c.w32(12, t210);
            blk = 198; continue;
        }
        198 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 199; continue;
        }
        199 => {
            let Some(t211) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081cb0e); };
            c.w32(15, t211);
            let Some(t212) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081cb12); };
            c.cmp(t212, c.r32(15), 32);
            if c.cc_ne() { blk = 212; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t213) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081cb1f); };
            c.w32(15, t213);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 563; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t214) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081cb46); };
            c.w64(1, t214);
            let Some(t215) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081cb4d); };
            c.w64(7, t215);
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t216) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081cb5c); };
            c.w64(0, t216);
            let Some(t217) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081cb63); };
            c.cmp(t217, c.r64(6), 64);
            if c.cc_be() { blk = 560; continue; }
            blk = 203; continue;
        }
        203 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081cb6c); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 204; continue;
        }
        204 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t218) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081cb76); };
            c.w64(11, t218);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081cb7d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700081cb81); }
            c.w64(7, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_e() { blk = 207; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081cb99); };
            c.w64(12, t219);
            blk = 206; continue;
        }
        206 => {
            let r220 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r220);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081cba4); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ne() { blk = 206; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            let r221 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r221);
            let Some(t222) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700081cbb7); };
            c.w32(11, t222);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081cbbb); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 208; continue; }
            blk = 209; continue;
        }
        209 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081cbc5); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r223 = c.r64(7) & c.r64(2);
            c.w64(7, r223);
            c.logic(r223, 64);
            let Some(t224) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081cbd7); };
            c.logic(t224 & 0x4u64, 8);
            if c.cc_ne() { blk = 691; continue; }
            blk = 211; continue;
        }
        211 => {
            c.w64(8, c.r64(1));
            blk = 212; continue;
        }
        212 => {
            let Some(t225) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081cbe4); };
            c.w32(1, t225);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 562; continue; }
            blk = 213; continue;
        }
        213 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t226) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081cbff); };
            c.w64(0, t226);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081cc03); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r227 = c.r64(7) & c.r64(8);
            c.w64(7, r227);
            c.logic(r227, 64);
            let Some(t228) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081cc15); };
            c.logic(t228 & 0x4u64, 8);
            if c.cc_ne() { blk = 688; continue; }
            blk = 215; continue;
        }
        215 => {
            blk = 533; continue;
        }
        216 => {
            c.w64(9, c.r64(2));
            let r229 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r229);
            c.logic(r229, 64);
            let Some(t230) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081cc2e); };
            c.w64(12, t230);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 223; continue; }
            blk = 217; continue;
        }
        217 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 223; continue; }
            blk = 218; continue;
        }
        218 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t231) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081cc42); };
            c.w64(15, t231);
            let Some(t232) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081cc49); };
            c.w64(15, t232);
            c.w64(3, c.r64(12));
            let r233 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r233);
            c.logic(r233, 64);
            c.w64(7, c.r64(15));
            let r234 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r234);
            c.logic(r234, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 221; continue; }
            blk = 219; continue;
        }
        219 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 222; continue; }
            blk = 220; continue;
        }
        220 => {
            blk = 223; continue;
        }
        221 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 223; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t235) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081cc71); };
            c.w32(7, t235);
            let Some(t236) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081cc74); };
            c.cmp(t236, c.r32(7), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 223; continue;
        }
        223 => {
            let Some(t237) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cc81); };
            c.w32(7, t237);
            let r238 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r238);
            let Some(t239) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081cc87); };
            c.cmp(t239, c.r32(7), 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t240) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081cc94); };
            c.w32(9, t240);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t241) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cca1); };
            c.cmp(t241, 0x834u64, 16);
            if c.cc_e() { blk = 226; continue; }
            blk = 225; continue;
        }
        225 => {
            c.w64(8, c.r64(12));
            blk = 227; continue;
        }
        226 => {
            let Some(t242) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081ccaf); };
            c.w32(8, t242);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 227; continue;
        }
        227 => {
            c.cmp(c.r64(12), 0x3fffu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 228; continue;
        }
        228 => {
            c.cmp(c.r64(12), 0x3fffffeu64, 64);
            if c.cc_g() { blk = 569; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w64(15, c.r64(9).wrapping_mul(4).wrapping_add(0x8));
            c.cmp(c.r64(15), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 565; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t243) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081cce8); };
            c.w64(3, t243);
            let Some(t244) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081ccef); };
            c.w64(6, t244);
            c.w64(0, c.r64(15).wrapping_add(c.r64(6)));
            let Some(t245) = c.ld64(c.r64(3).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ccf6); };
            c.cmp(t245, c.r64(0), 64);
            if c.cc_be() { blk = 567; continue; }
            blk = 231; continue;
        }
        231 => {
            if !c.st64(c.r64(3), c.r64(0)) { return RegfileExit::fault(0x700081cd00); }
            c.w64(15, c.r64(6).wrapping_add(0x1));
            blk = 232; continue;
        }
        232 => {
            let r246 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r246);
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x700081cd0a); };
            c.w64(0, t247);
            if !c.st32(c.r64(15).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x700081cd11); }
            if !c.st32(c.r64(15).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081cd15); }
            let Some(t248) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081cd19); };
            c.w64(12, t248);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700081cd20); }
            c.w64(12, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_e() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            let r249 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r249);
            if !c.movsd_mr(c.r64(9).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081cd40); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ne() { blk = 233; continue; }
            blk = 234; continue;
        }
        234 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 239; continue; }
            blk = 235; continue;
        }
        235 => {
            let r250 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r250);
            let Some(t251) = c.ld32(c.r64(8).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081cd5d); };
            c.w32(9, t251);
            let r252 = c.sub(c.r64(12), 0x8u64, 64);
            c.w64(12, r252);
            let Some(t253) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081cd65); };
            c.cmp(t253, c.r32(9), 32);
            if c.cc_e() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            let r254 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r254);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            if !c.movsd_mr(c.r64(12).wrapping_add(c.r64(15)), 0) { return RegfileExit::fault(0x700081cd7a); }
            blk = 238; continue;
        }
        237 => {
            let Some(t255) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081cd82); };
            c.w64(9, t255);
            if !c.st64(c.r64(12).wrapping_add(c.r64(15)), c.r64(9)) { return RegfileExit::fault(0x700081cd89); }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_ne() { blk = 235; continue; }
            blk = 239; continue;
        }
        239 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(15)) { return RegfileExit::fault(0x700081cd93); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r256 = c.r64(7) & c.r64(2);
            c.w64(7, r256);
            c.logic(r256, 64);
            let Some(t257) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081cda7); };
            c.logic(t257 & 0x4u64, 8);
            if c.cc_ne() { blk = 685; continue; }
            blk = 241; continue;
        }
        241 => {
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081cdb1); };
            c.w64(0, t258);
            let Some(t259) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081cdb5); };
            c.w64(6, t259);
            blk = 242; continue;
        }
        242 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081cdb9); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r260 = c.r64(7) & c.r64(2);
            c.w64(7, r260);
            c.logic(r260, 64);
            let Some(t261) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081cdc7); };
            c.logic(t261 & 0x4u64, 8);
            if c.cc_ne() { blk = 682; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t262) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cdd1); };
            c.w32(7, t262);
            let r263 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r263);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 249; continue; }
            blk = 244; continue;
        }
        244 => {
            let Some(t264) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081cddc); };
            c.w32(8, t264);
            let r265 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r265);
            let Some(t266) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cde3); };
            c.cmp(t266, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 245; continue;
        }
        245 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081cdef); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 246; continue;
        }
        246 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 247; continue;
        }
        247 => {
            let Some(t267) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081ce12); };
            c.w64(9, t267);
            let r268 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r268);
            let Some(t269) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081ce1c); };
            c.w64(12, t269);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 250; continue; }
            blk = 248; continue;
        }
        248 => {
            blk = 534; continue;
        }
        249 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 250; continue;
        }
        250 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 253; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t270) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ce37); };
            c.w32(9, t270);
            let Some(t271) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081ce3b); };
            c.cmp(t271, c.r32(9), 32);
            if c.cc_ne() { blk = 534; continue; }
            blk = 252; continue;
        }
        252 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700081ce48); }
            blk = 254; continue;
        }
        253 => {
            c.w64(9, c.r64(0));
            let r272 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r272);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(9) as u32 as i32 as i64));
            blk = 254; continue;
        }
        254 => {
            let Some(t273) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ce5e); };
            c.w32(9, t273);
            let r274 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r274);
            let Some(t275) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ce65); };
            c.cmp(t275, 0x834u64, 16);
            if c.cc_e() { blk = 256; continue; }
            blk = 255; continue;
        }
        255 => {
            let Some(t276) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ce6e); };
            c.w32(9, t276);
            blk = 257; continue;
        }
        256 => {
            let Some(t277) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081ce74); };
            c.w32(9, t277);
            blk = 257; continue;
        }
        257 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(8), c.r64(9), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 258; continue;
        }
        258 => {
            let Some(t278) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ce87); };
            c.w32(1, t278);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 571; continue; }
            blk = 259; continue;
        }
        259 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081cea3); }
            blk = 533; continue;
        }
        260 => {
            c.w64(8, c.r64(2));
            let r279 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r279);
            c.logic(r279, 64);
            let Some(t280) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ceb9); };
            c.w64(9, t280);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 267; continue; }
            blk = 261; continue;
        }
        261 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 267; continue; }
            blk = 262; continue;
        }
        262 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t281) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081cecd); };
            c.w64(12, t281);
            let Some(t282) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081ced4); };
            c.w64(12, t282);
            c.w64(15, c.r64(9));
            let r283 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r283);
            c.logic(r283, 64);
            c.w64(3, c.r64(12));
            let r284 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r284);
            c.logic(r284, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 265; continue; }
            blk = 263; continue;
        }
        263 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 266; continue; }
            blk = 264; continue;
        }
        264 => {
            blk = 267; continue;
        }
        265 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 267; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t285) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081cefd); };
            c.w32(8, t285);
            let Some(t286) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081cf01); };
            c.cmp(t286, c.r32(8), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 267; continue;
        }
        267 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081cf0e); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r287 = c.r64(8) & c.r64(2);
            c.w64(8, r287);
            c.logic(r287, 64);
            let Some(t288) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081cf1c); };
            c.logic(t288 & 0x4u64, 8);
            if c.cc_ne() { blk = 679; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t289) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cf27); };
            c.w32(8, t289);
            let r290 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r290);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 274; continue; }
            blk = 269; continue;
        }
        269 => {
            let Some(t291) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081cf33); };
            c.w32(9, t291);
            let r292 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r292);
            let Some(t293) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cf3a); };
            c.cmp(t293, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 270; continue;
        }
        270 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081cf46); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 271; continue;
        }
        271 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 272; continue;
        }
        272 => {
            let Some(t294) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081cf69); };
            c.w64(12, t294);
            let r295 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r295);
            let Some(t296) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081cf73); };
            c.w64(15, t296);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 275; continue; }
            blk = 273; continue;
        }
        273 => {
            blk = 534; continue;
        }
        274 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 275; continue;
        }
        275 => {
            let Some(t297) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081cf8a); };
            c.w32(12, t297);
            let r298 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r298);
            let Some(t299) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081cf91); };
            c.cmp(t299, 0x834u64, 16);
            if c.cc_e() { blk = 277; continue; }
            blk = 276; continue;
        }
        276 => {
            let Some(t300) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081cf9b); };
            c.w32(12, t300);
            blk = 278; continue;
        }
        277 => {
            let Some(t301) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081cfa1); };
            c.w32(12, t301);
            blk = 278; continue;
        }
        278 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t302) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081cfb7); };
            c.w32(15, t302);
            let Some(t303) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081cfbb); };
            c.cmp(t303, c.r32(15), 32);
            if c.cc_ne() { blk = 292; continue; }
            blk = 280; continue;
        }
        280 => {
            let Some(t304) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081cfc8); };
            c.w32(15, t304);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 281; continue;
        }
        281 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 575; continue; }
            blk = 282; continue;
        }
        282 => {
            let Some(t305) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081cfef); };
            c.w64(1, t305);
            let Some(t306) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081cff6); };
            c.w64(7, t306);
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t307) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081d005); };
            c.w64(0, t307);
            let Some(t308) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081d00c); };
            c.cmp(t308, c.r64(6), 64);
            if c.cc_be() { blk = 572; continue; }
            blk = 283; continue;
        }
        283 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081d015); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 284; continue;
        }
        284 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t309) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081d01f); };
            c.w64(11, t309);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081d026); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700081d02a); }
            c.w64(7, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_e() { blk = 287; continue; }
            blk = 285; continue;
        }
        285 => {
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081d042); };
            c.w64(12, t310);
            blk = 286; continue;
        }
        286 => {
            let r311 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r311);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081d04d); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ne() { blk = 286; continue; }
            blk = 287; continue;
        }
        287 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 289; continue; }
            blk = 288; continue;
        }
        288 => {
            let r312 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r312);
            let Some(t313) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700081d060); };
            c.w32(11, t313);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081d064); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 288; continue; }
            blk = 289; continue;
        }
        289 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081d06e); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 290; continue;
        }
        290 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r314 = c.r64(7) & c.r64(2);
            c.w64(7, r314);
            c.logic(r314, 64);
            let Some(t315) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d080); };
            c.logic(t315 & 0x4u64, 8);
            if c.cc_ne() { blk = 676; continue; }
            blk = 291; continue;
        }
        291 => {
            c.w64(8, c.r64(1));
            blk = 292; continue;
        }
        292 => {
            let Some(t316) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d08d); };
            c.w32(1, t316);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 574; continue; }
            blk = 293; continue;
        }
        293 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081d0a8); };
            c.w64(0, t317);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081d0ac); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 295; continue; }
            blk = 294; continue;
        }
        294 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r318 = c.r64(7) & c.r64(8);
            c.w64(7, r318);
            c.logic(r318, 64);
            let Some(t319) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d0be); };
            c.logic(t319 & 0x4u64, 8);
            if c.cc_ne() { blk = 673; continue; }
            blk = 295; continue;
        }
        295 => {
            blk = 533; continue;
        }
        296 => {
            c.w64(8, c.r64(2));
            let r320 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r320);
            c.logic(r320, 64);
            let Some(t321) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d0d7); };
            c.w64(9, t321);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 297; continue;
        }
        297 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 298; continue;
        }
        298 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t322) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d0eb); };
            c.w64(12, t322);
            let Some(t323) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081d0f2); };
            c.w64(12, t323);
            c.w64(15, c.r64(9));
            let r324 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r324);
            c.logic(r324, 64);
            c.w64(3, c.r64(12));
            let r325 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r325);
            c.logic(r325, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 301; continue; }
            blk = 299; continue;
        }
        299 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 302; continue; }
            blk = 300; continue;
        }
        300 => {
            blk = 303; continue;
        }
        301 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 303; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t326) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081d11b); };
            c.w32(8, t326);
            let Some(t327) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081d11f); };
            c.cmp(t327, c.r32(8), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 303; continue;
        }
        303 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081d12c); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r328 = c.r64(8) & c.r64(2);
            c.w64(8, r328);
            c.logic(r328, 64);
            let Some(t329) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d13a); };
            c.logic(t329 & 0x4u64, 8);
            if c.cc_ne() { blk = 670; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t330) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d145); };
            c.w32(8, t330);
            let r331 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r331);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 310; continue; }
            blk = 305; continue;
        }
        305 => {
            let Some(t332) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d151); };
            c.w32(9, t332);
            let r333 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r333);
            let Some(t334) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d158); };
            c.cmp(t334, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 306; continue;
        }
        306 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081d164); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 307; continue;
        }
        307 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 308; continue;
        }
        308 => {
            let Some(t335) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081d187); };
            c.w64(12, t335);
            let r336 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r336);
            let Some(t337) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081d191); };
            c.w64(15, t337);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 311; continue; }
            blk = 309; continue;
        }
        309 => {
            blk = 534; continue;
        }
        310 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 311; continue;
        }
        311 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 314; continue; }
            blk = 312; continue;
        }
        312 => {
            let Some(t338) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d1ac); };
            c.w32(12, t338);
            let Some(t339) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081d1b0); };
            c.cmp(t339, c.r32(12), 32);
            if c.cc_ne() { blk = 534; continue; }
            blk = 313; continue;
        }
        313 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700081d1bd); }
            blk = 315; continue;
        }
        314 => {
            c.w64(12, c.r64(0));
            let r340 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r340);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 315; continue;
        }
        315 => {
            let Some(t341) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d1d3); };
            c.w32(12, t341);
            let r342 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r342);
            let Some(t343) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d1da); };
            c.cmp(t343, 0x834u64, 16);
            if c.cc_e() { blk = 317; continue; }
            blk = 316; continue;
        }
        316 => {
            let Some(t344) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d1e4); };
            c.w32(12, t344);
            blk = 318; continue;
        }
        317 => {
            let Some(t345) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081d1ea); };
            c.w32(12, t345);
            blk = 318; continue;
        }
        318 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 319; continue;
        }
        319 => {
            let Some(t346) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d1fd); };
            c.w32(1, t346);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 577; continue; }
            blk = 320; continue;
        }
        320 => {
            c.xorpd_rr(15, 15);
            c.arith_sd(1, 0, c.xmm_u64(15));
            if !c.movsd_mr(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(8)).wrapping_add(0x7), 0) { return RegfileExit::fault(0x700081d21a); }
            blk = 533; continue;
        }
        321 => {
            c.w64(9, c.r64(2));
            let r347 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r347);
            c.logic(r347, 64);
            let Some(t348) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d230); };
            c.w64(12, t348);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 328; continue; }
            blk = 322; continue;
        }
        322 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 328; continue; }
            blk = 323; continue;
        }
        323 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t349) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d244); };
            c.w64(15, t349);
            let Some(t350) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d24b); };
            c.w64(15, t350);
            c.w64(3, c.r64(12));
            let r351 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r351);
            c.logic(r351, 64);
            c.w64(7, c.r64(15));
            let r352 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r352);
            c.logic(r352, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 326; continue; }
            blk = 324; continue;
        }
        324 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 327; continue; }
            blk = 325; continue;
        }
        325 => {
            blk = 328; continue;
        }
        326 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 328; continue; }
            blk = 327; continue;
        }
        327 => {
            let Some(t353) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081d273); };
            c.w32(7, t353);
            let Some(t354) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081d276); };
            c.cmp(t354, c.r32(7), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 328; continue;
        }
        328 => {
            let Some(t355) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d283); };
            c.w32(7, t355);
            let r356 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r356);
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081d289); };
            c.cmp(t357, c.r32(7), 32);
            if c.cc_e() { blk = 351; continue; }
            blk = 329; continue;
        }
        329 => {
            let Some(t358) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d296); };
            c.w32(9, t358);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t359) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d2a3); };
            c.cmp(t359, 0x834u64, 16);
            if c.cc_e() { blk = 331; continue; }
            blk = 330; continue;
        }
        330 => {
            c.w64(8, c.r64(12));
            blk = 332; continue;
        }
        331 => {
            let Some(t360) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081d2b1); };
            c.w32(8, t360);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 332; continue;
        }
        332 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 333; continue;
        }
        333 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 582; continue; }
            blk = 334; continue;
        }
        334 => {
            let Some(t361) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d2d5); };
            c.w64(15, t361);
            let Some(t362) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d2dc); };
            c.w64(3, t362);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t363) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081d2eb); };
            c.w64(1, t363);
            let Some(t364) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081d2f2); };
            c.cmp(t364, c.r64(0), 64);
            if c.cc_be() { blk = 578; continue; }
            blk = 335; continue;
        }
        335 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x700081d2fb); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 336; continue;
        }
        336 => {
            let r365 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r365);
            let Some(t366) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081d305); };
            c.w64(15, t366);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081d30c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081d310); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 339; continue; }
            blk = 337; continue;
        }
        337 => {
            let Some(t367) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081d322); };
            c.w64(12, t367);
            blk = 338; continue;
        }
        338 => {
            let r368 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r368);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081d32d); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 338; continue; }
            blk = 339; continue;
        }
        339 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 348; continue; }
            blk = 340; continue;
        }
        340 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t369) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081d351); };
            c.w64(12, t369);
            blk = 342; continue;
        }
        341 => {
            let r370 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r370);
            blk = 342; continue;
        }
        342 => {
            let r371 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r371);
            let Some(t372) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081d362); };
            c.cmp(t372, c.r64(12), 64);
            if c.cc_e() { blk = 347; continue; }
            blk = 343; continue;
        }
        343 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081d368); }
            let Some(t373) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d36e); };
            c.w64(15, t373);
            let Some(t374) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d375); };
            c.w64(0, t374);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t375) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081d380); };
            c.w64(11, t375);
            let Some(t376) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700081d387); };
            c.cmp(t376, c.r64(3), 64);
            if c.cc_be() { blk = 580; continue; }
            blk = 344; continue;
        }
        344 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700081d390); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 345; continue;
        }
        345 => {
            let Some(t377) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081d397); };
            c.w64(15, t377);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081d39e); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081d3a2); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x700081d3a8); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 347; continue; }
            blk = 346; continue;
        }
        346 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r378 = c.r64(15) & c.r64(1);
            c.w64(15, r378);
            c.logic(r378, 64);
            let Some(t379) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d3bc); };
            c.logic(t379 & 0x4u64, 8);
            if c.cc_ne() { blk = 667; continue; }
            blk = 347; continue;
        }
        347 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 341; continue; }
            blk = 348; continue;
        }
        348 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081d3cd); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 350; continue; }
            blk = 349; continue;
        }
        349 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r380 = c.r64(7) & c.r64(2);
            c.w64(7, r380);
            c.logic(r380, 64);
            let Some(t381) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d3df); };
            c.logic(t381 & 0x4u64, 8);
            if c.cc_ne() { blk = 664; continue; }
            blk = 350; continue;
        }
        350 => {
            let Some(t382) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081d3e9); };
            c.w64(1, t382);
            let Some(t383) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081d3ed); };
            c.w64(0, t383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081d3f1); };
            c.w64(11, t384);
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081d3f5); };
            c.w64(6, t385);
            blk = 351; continue;
        }
        351 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081d3f9); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r386 = c.r64(7) & c.r64(2);
            c.w64(7, r386);
            c.logic(r386, 64);
            let Some(t387) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d407); };
            c.logic(t387 & 0x4u64, 8);
            if c.cc_ne() { blk = 661; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t388) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d411); };
            c.w32(7, t388);
            let r389 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r389);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 358; continue; }
            blk = 353; continue;
        }
        353 => {
            let Some(t390) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d41c); };
            c.w32(8, t390);
            let r391 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r391);
            let Some(t392) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d423); };
            c.cmp(t392, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 354; continue;
        }
        354 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081d42f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 355; continue;
        }
        355 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t393) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081d452); };
            c.w64(9, t393);
            let r394 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r394);
            let Some(t395) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081d45c); };
            c.w64(12, t395);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 359; continue; }
            blk = 357; continue;
        }
        357 => {
            blk = 534; continue;
        }
        358 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 359; continue;
        }
        359 => {
            let Some(t396) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d473); };
            c.w32(9, t396);
            let r397 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r397);
            let Some(t398) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d47a); };
            c.cmp(t398, 0x834u64, 16);
            if c.cc_e() { blk = 361; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t399) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d483); };
            c.w32(9, t399);
            blk = 362; continue;
        }
        361 => {
            let Some(t400) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081d489); };
            c.w32(9, t400);
            blk = 362; continue;
        }
        362 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 363; continue;
        }
        363 => {
            let Some(t401) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d49f); };
            c.w32(12, t401);
            let Some(t402) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081d4a3); };
            c.cmp(t402, c.r32(12), 32);
            if c.cc_ne() { blk = 376; continue; }
            blk = 364; continue;
        }
        364 => {
            let Some(t403) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d4b0); };
            c.w32(12, t403);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 365; continue;
        }
        365 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 587; continue; }
            blk = 366; continue;
        }
        366 => {
            let Some(t404) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d4d7); };
            c.w64(1, t404);
            let Some(t405) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081d4de); };
            c.w64(11, t405);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(11)));
            let Some(t406) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081d4ed); };
            c.w64(0, t406);
            let Some(t407) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081d4f4); };
            c.cmp(t407, c.r64(6), 64);
            if c.cc_be() { blk = 584; continue; }
            blk = 367; continue;
        }
        367 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081d4fd); }
            c.w64(1, c.r64(11).wrapping_add(0x1));
            blk = 368; continue;
        }
        368 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t408) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081d508); };
            c.w64(15, t408);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081d50f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700081d513); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_e() { blk = 371; continue; }
            blk = 369; continue;
        }
        369 => {
            let Some(t409) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081d52c); };
            c.w64(12, t409);
            blk = 370; continue;
        }
        370 => {
            let r410 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r410);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081d537); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ne() { blk = 370; continue; }
            blk = 371; continue;
        }
        371 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 373; continue; }
            blk = 372; continue;
        }
        372 => {
            let r411 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r411);
            let Some(t412) = c.ld32(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081d54a); };
            c.w32(11, t412);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081d54e); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 372; continue; }
            blk = 373; continue;
        }
        373 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081d558); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 375; continue; }
            blk = 374; continue;
        }
        374 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r413 = c.r64(7) & c.r64(2);
            c.w64(7, r413);
            c.logic(r413, 64);
            let Some(t414) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d56a); };
            c.logic(t414 & 0x4u64, 8);
            if c.cc_ne() { blk = 658; continue; }
            blk = 375; continue;
        }
        375 => {
            c.w64(7, c.r64(1));
            blk = 376; continue;
        }
        376 => {
            let Some(t415) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d577); };
            c.w32(1, t415);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 586; continue; }
            blk = 377; continue;
        }
        377 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t416) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081d591); };
            c.w64(0, t416);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081d595); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 379; continue; }
            blk = 378; continue;
        }
        378 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r417 = c.r64(8) & c.r64(7);
            c.w64(8, r417);
            c.logic(r417, 64);
            let Some(t418) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d5a6); };
            c.logic(t418 & 0x4u64, 8);
            if c.cc_ne() { blk = 655; continue; }
            blk = 379; continue;
        }
        379 => {
            blk = 533; continue;
        }
        380 => {
            c.w64(9, c.r64(2));
            let r419 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r419);
            c.logic(r419, 64);
            let Some(t420) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d5c0); };
            c.w64(12, t420);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 387; continue; }
            blk = 381; continue;
        }
        381 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 387; continue; }
            blk = 382; continue;
        }
        382 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t421) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d5d4); };
            c.w64(15, t421);
            let Some(t422) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d5db); };
            c.w64(15, t422);
            c.w64(3, c.r64(12));
            let r423 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r423);
            c.logic(r423, 64);
            c.w64(7, c.r64(15));
            let r424 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r424);
            c.logic(r424, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 385; continue; }
            blk = 383; continue;
        }
        383 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 386; continue; }
            blk = 384; continue;
        }
        384 => {
            blk = 387; continue;
        }
        385 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 387; continue; }
            blk = 386; continue;
        }
        386 => {
            let Some(t425) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081d603); };
            c.w32(7, t425);
            let Some(t426) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081d606); };
            c.cmp(t426, c.r32(7), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 387; continue;
        }
        387 => {
            let Some(t427) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d613); };
            c.w32(7, t427);
            let r428 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r428);
            let Some(t429) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081d619); };
            c.cmp(t429, c.r32(7), 32);
            if c.cc_e() { blk = 410; continue; }
            blk = 388; continue;
        }
        388 => {
            let Some(t430) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d626); };
            c.w32(9, t430);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t431) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d633); };
            c.cmp(t431, 0x834u64, 16);
            if c.cc_e() { blk = 390; continue; }
            blk = 389; continue;
        }
        389 => {
            c.w64(8, c.r64(12));
            blk = 391; continue;
        }
        390 => {
            let Some(t432) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081d641); };
            c.w32(8, t432);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 391; continue;
        }
        391 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 392; continue;
        }
        392 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 593; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d665); };
            c.w64(15, t433);
            let Some(t434) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d66c); };
            c.w64(3, t434);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081d67b); };
            c.w64(1, t435);
            let Some(t436) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081d682); };
            c.cmp(t436, c.r64(0), 64);
            if c.cc_be() { blk = 589; continue; }
            blk = 394; continue;
        }
        394 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x700081d68b); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 395; continue;
        }
        395 => {
            let r437 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r437);
            let Some(t438) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081d695); };
            c.w64(15, t438);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081d69c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081d6a0); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 398; continue; }
            blk = 396; continue;
        }
        396 => {
            let Some(t439) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081d6b2); };
            c.w64(12, t439);
            blk = 397; continue;
        }
        397 => {
            let r440 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r440);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081d6bd); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 397; continue; }
            blk = 398; continue;
        }
        398 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 407; continue; }
            blk = 399; continue;
        }
        399 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t441) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081d6e1); };
            c.w64(12, t441);
            blk = 401; continue;
        }
        400 => {
            let r442 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r442);
            blk = 401; continue;
        }
        401 => {
            let r443 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r443);
            let Some(t444) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081d6f2); };
            c.cmp(t444, c.r64(12), 64);
            if c.cc_e() { blk = 406; continue; }
            blk = 402; continue;
        }
        402 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081d6f8); }
            let Some(t445) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d6fe); };
            c.w64(15, t445);
            let Some(t446) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d705); };
            c.w64(0, t446);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t447) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081d710); };
            c.w64(11, t447);
            let Some(t448) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700081d717); };
            c.cmp(t448, c.r64(3), 64);
            if c.cc_be() { blk = 591; continue; }
            blk = 403; continue;
        }
        403 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700081d720); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 404; continue;
        }
        404 => {
            let Some(t449) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081d727); };
            c.w64(15, t449);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081d72e); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081d732); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x700081d738); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 406; continue; }
            blk = 405; continue;
        }
        405 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r450 = c.r64(15) & c.r64(1);
            c.w64(15, r450);
            c.logic(r450, 64);
            let Some(t451) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d74c); };
            c.logic(t451 & 0x4u64, 8);
            if c.cc_ne() { blk = 652; continue; }
            blk = 406; continue;
        }
        406 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 400; continue; }
            blk = 407; continue;
        }
        407 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081d75d); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 409; continue; }
            blk = 408; continue;
        }
        408 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r452 = c.r64(7) & c.r64(2);
            c.w64(7, r452);
            c.logic(r452, 64);
            let Some(t453) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d76f); };
            c.logic(t453 & 0x4u64, 8);
            if c.cc_ne() { blk = 649; continue; }
            blk = 409; continue;
        }
        409 => {
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081d779); };
            c.w64(1, t454);
            let Some(t455) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081d77d); };
            c.w64(0, t455);
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081d781); };
            c.w64(11, t456);
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081d785); };
            c.w64(6, t457);
            blk = 410; continue;
        }
        410 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081d789); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r458 = c.r64(7) & c.r64(2);
            c.w64(7, r458);
            c.logic(r458, 64);
            let Some(t459) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d797); };
            c.logic(t459 & 0x4u64, 8);
            if c.cc_ne() { blk = 646; continue; }
            blk = 411; continue;
        }
        411 => {
            let Some(t460) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d7a1); };
            c.w32(7, t460);
            let r461 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r461);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 417; continue; }
            blk = 412; continue;
        }
        412 => {
            let Some(t462) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d7ac); };
            c.w32(8, t462);
            let r463 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r463);
            let Some(t464) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d7b3); };
            c.cmp(t464, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 413; continue;
        }
        413 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081d7bf); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 414; continue;
        }
        414 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 415; continue;
        }
        415 => {
            let Some(t465) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081d7e2); };
            c.w64(9, t465);
            let r466 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r466);
            let Some(t467) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081d7ec); };
            c.w64(12, t467);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 418; continue; }
            blk = 416; continue;
        }
        416 => {
            blk = 534; continue;
        }
        417 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 418; continue;
        }
        418 => {
            let Some(t468) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d803); };
            c.w32(9, t468);
            let r469 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r469);
            let Some(t470) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d80a); };
            c.cmp(t470, 0x834u64, 16);
            if c.cc_e() { blk = 420; continue; }
            blk = 419; continue;
        }
        419 => {
            let Some(t471) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d813); };
            c.w32(9, t471);
            blk = 421; continue;
        }
        420 => {
            let Some(t472) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081d819); };
            c.w32(9, t472);
            blk = 421; continue;
        }
        421 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 422; continue;
        }
        422 => {
            let Some(t473) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081d82f); };
            c.w32(12, t473);
            let Some(t474) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081d833); };
            c.cmp(t474, c.r32(12), 32);
            if c.cc_ne() { blk = 435; continue; }
            blk = 423; continue;
        }
        423 => {
            let Some(t475) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d840); };
            c.w32(12, t475);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 424; continue;
        }
        424 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 598; continue; }
            blk = 425; continue;
        }
        425 => {
            let Some(t476) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d867); };
            c.w64(1, t476);
            let Some(t477) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081d86e); };
            c.w64(11, t477);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(11)));
            let Some(t478) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081d87d); };
            c.w64(0, t478);
            let Some(t479) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081d884); };
            c.cmp(t479, c.r64(6), 64);
            if c.cc_be() { blk = 595; continue; }
            blk = 426; continue;
        }
        426 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081d88d); }
            c.w64(1, c.r64(11).wrapping_add(0x1));
            blk = 427; continue;
        }
        427 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t480) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081d898); };
            c.w64(15, t480);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081d89f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700081d8a3); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_e() { blk = 430; continue; }
            blk = 428; continue;
        }
        428 => {
            let Some(t481) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081d8bc); };
            c.w64(12, t481);
            blk = 429; continue;
        }
        429 => {
            let r482 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r482);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081d8c7); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ne() { blk = 429; continue; }
            blk = 430; continue;
        }
        430 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 432; continue; }
            blk = 431; continue;
        }
        431 => {
            let r483 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r483);
            let Some(t484) = c.ld32(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081d8da); };
            c.w32(11, t484);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081d8de); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 431; continue; }
            blk = 432; continue;
        }
        432 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081d8e8); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 434; continue; }
            blk = 433; continue;
        }
        433 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r485 = c.r64(7) & c.r64(2);
            c.w64(7, r485);
            c.logic(r485, 64);
            let Some(t486) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d8fa); };
            c.logic(t486 & 0x4u64, 8);
            if c.cc_ne() { blk = 643; continue; }
            blk = 434; continue;
        }
        434 => {
            c.w64(7, c.r64(1));
            blk = 435; continue;
        }
        435 => {
            let Some(t487) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d907); };
            c.w32(1, t487);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 597; continue; }
            blk = 436; continue;
        }
        436 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t488) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081d921); };
            c.w64(0, t488);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081d925); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 438; continue; }
            blk = 437; continue;
        }
        437 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r489 = c.r64(8) & c.r64(7);
            c.w64(8, r489);
            c.logic(r489, 64);
            let Some(t490) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d936); };
            c.logic(t490 & 0x4u64, 8);
            if c.cc_ne() { blk = 640; continue; }
            blk = 438; continue;
        }
        438 => {
            blk = 533; continue;
        }
        439 => {
            c.w64(9, c.r64(2));
            let r491 = c.r64(9) & 0xfffffffffffc0000u64;
            c.w64(9, r491);
            c.logic(r491, 64);
            let Some(t492) = c.ld64(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081d950); };
            c.w64(12, t492);
            c.logic(c.r8(12) & 0x18u64, 8);
            if c.cc_e() { blk = 446; continue; }
            blk = 440; continue;
        }
        440 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 446; continue; }
            blk = 441; continue;
        }
        441 => {
            c.w64(12, c.r64(2).wrapping_add(0x13));
            let Some(t493) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d964); };
            c.w64(15, t493);
            let Some(t494) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d96b); };
            c.w64(15, t494);
            c.w64(3, c.r64(12));
            let r495 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r495);
            c.logic(r495, 64);
            c.w64(7, c.r64(15));
            let r496 = c.r64(7) & 0xfffffffffffc0000u64;
            c.w64(7, r496);
            c.logic(r496, 64);
            c.cmp(c.r64(7), c.r64(3), 64);
            if c.cc_e() { blk = 444; continue; }
            blk = 442; continue;
        }
        442 => {
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_e() { blk = 445; continue; }
            blk = 443; continue;
        }
        443 => {
            blk = 446; continue;
        }
        444 => {
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_be() { blk = 446; continue; }
            blk = 445; continue;
        }
        445 => {
            let Some(t497) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081d993); };
            c.w32(7, t497);
            let Some(t498) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081d996); };
            c.cmp(t498, c.r32(7), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 446; continue;
        }
        446 => {
            let Some(t499) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d9a3); };
            c.w32(7, t499);
            let r500 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r500);
            let Some(t501) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x700081d9a9); };
            c.cmp(t501, c.r32(7), 32);
            if c.cc_e() { blk = 469; continue; }
            blk = 447; continue;
        }
        447 => {
            let Some(t502) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081d9b6); };
            c.w32(9, t502);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let Some(t503) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081d9c3); };
            c.cmp(t503, 0x834u64, 16);
            if c.cc_e() { blk = 449; continue; }
            blk = 448; continue;
        }
        448 => {
            c.w64(8, c.r64(12));
            blk = 450; continue;
        }
        449 => {
            let Some(t504) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081d9d1); };
            c.w32(8, t504);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 450; continue;
        }
        450 => {
            c.cmp(c.r64(12), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 451; continue;
        }
        451 => {
            c.cmp(c.r64(12), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 604; continue; }
            blk = 452; continue;
        }
        452 => {
            let Some(t505) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081d9f5); };
            c.w64(15, t505);
            let Some(t506) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081d9fc); };
            c.w64(3, t506);
            c.w64(6, c.r64(9).wrapping_mul(2).wrapping_add(0x8));
            c.w64(0, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t507) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081da0b); };
            c.w64(1, t507);
            let Some(t508) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081da12); };
            c.cmp(t508, c.r64(0), 64);
            if c.cc_be() { blk = 600; continue; }
            blk = 453; continue;
        }
        453 => {
            if !c.st64(c.r64(15), c.r64(0)) { return RegfileExit::fault(0x700081da1b); }
            c.w64(1, c.r64(3).wrapping_add(0x1));
            blk = 454; continue;
        }
        454 => {
            let r509 = c.shl(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r509);
            let Some(t510) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081da25); };
            c.w64(15, t510);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081da2c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700081da30); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 457; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t511) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081da42); };
            c.w64(12, t511);
            blk = 456; continue;
        }
        456 => {
            let r512 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r512);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081da4d); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 456; continue; }
            blk = 457; continue;
        }
        457 => {
            c.w64(9, c.r64(8).wrapping_mul(8).wrapping_add(0x7));
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 466; continue; }
            blk = 458; continue;
        }
        458 => {
            c.w64(8, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t513) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081da71); };
            c.w64(12, t513);
            blk = 460; continue;
        }
        459 => {
            let r514 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r514);
            blk = 460; continue;
        }
        460 => {
            let r515 = c.sub(c.r64(9), 0x8u64, 64);
            c.w64(9, r515);
            let Some(t516) = c.ld64(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081da82); };
            c.cmp(t516, c.r64(12), 64);
            if c.cc_e() { blk = 465; continue; }
            blk = 461; continue;
        }
        461 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(c.r64(7))) { return RegfileExit::fault(0x700081da88); }
            let Some(t517) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081da8e); };
            c.w64(15, t517);
            let Some(t518) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x700081da95); };
            c.w64(0, t518);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            c.w64(6, c.r64(8).wrapping_sub(0x4));
            let Some(t519) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081daa0); };
            c.w64(11, t519);
            let Some(t520) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x700081daa7); };
            c.cmp(t520, c.r64(3), 64);
            if c.cc_be() { blk = 602; continue; }
            blk = 462; continue;
        }
        462 => {
            if !c.st64(c.r64(15), c.r64(3)) { return RegfileExit::fault(0x700081dab0); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            blk = 463; continue;
        }
        463 => {
            let Some(t521) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081dab7); };
            c.w64(15, t521);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081dabe); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700081dac2); }
            if !c.st32(c.r64(1).wrapping_add(c.r64(6)), c.r32(11)) { return RegfileExit::fault(0x700081dac8); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 465; continue; }
            blk = 464; continue;
        }
        464 => {
            c.w64(15, 0xfffffffffffc0000u64);
            let r522 = c.r64(15) & c.r64(1);
            c.w64(15, r522);
            c.logic(r522, 64);
            let Some(t523) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081dadc); };
            c.logic(t523 & 0x4u64, 8);
            if c.cc_ne() { blk = 637; continue; }
            blk = 465; continue;
        }
        465 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 459; continue; }
            blk = 466; continue;
        }
        466 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081daed); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 468; continue; }
            blk = 467; continue;
        }
        467 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r524 = c.r64(7) & c.r64(2);
            c.w64(7, r524);
            c.logic(r524, 64);
            let Some(t525) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081daff); };
            c.logic(t525 & 0x4u64, 8);
            if c.cc_ne() { blk = 634; continue; }
            blk = 468; continue;
        }
        468 => {
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081db09); };
            c.w64(1, t526);
            let Some(t527) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081db0d); };
            c.w64(0, t527);
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081db11); };
            c.w64(11, t528);
            let Some(t529) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081db15); };
            c.w64(6, t529);
            blk = 469; continue;
        }
        469 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081db19); }
            c.w64(7, 0xfffffffffffc0000u64);
            let r530 = c.r64(7) & c.r64(2);
            c.w64(7, r530);
            c.logic(r530, 64);
            let Some(t531) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081db27); };
            c.logic(t531 & 0x4u64, 8);
            if c.cc_ne() { blk = 631; continue; }
            blk = 470; continue;
        }
        470 => {
            let Some(t532) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081db31); };
            c.w32(7, t532);
            let r533 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r533);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 476; continue; }
            blk = 471; continue;
        }
        471 => {
            let Some(t534) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081db3c); };
            c.w32(8, t534);
            let r535 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r535);
            let Some(t536) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081db43); };
            c.cmp(t536, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 472; continue;
        }
        472 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081db4f); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(8, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(8) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 473; continue;
        }
        473 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 474; continue;
        }
        474 => {
            let Some(t537) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081db72); };
            c.w64(9, t537);
            let r538 = c.add(c.r64(9), c.r64(8), 64);
            c.w64(9, r538);
            let Some(t539) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081db7c); };
            c.w64(12, t539);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_be() { blk = 477; continue; }
            blk = 475; continue;
        }
        475 => {
            blk = 534; continue;
        }
        476 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            blk = 477; continue;
        }
        477 => {
            let Some(t540) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081db93); };
            c.w32(9, t540);
            let r541 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r541);
            let Some(t542) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081db9a); };
            c.cmp(t542, 0x834u64, 16);
            if c.cc_e() { blk = 479; continue; }
            blk = 478; continue;
        }
        478 => {
            let Some(t543) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081dba3); };
            c.w32(9, t543);
            blk = 480; continue;
        }
        479 => {
            let Some(t544) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081dba9); };
            c.w32(9, t544);
            blk = 480; continue;
        }
        480 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w64(12, c.r64(9));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 481; continue;
        }
        481 => {
            let Some(t545) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081dbbf); };
            c.w32(12, t545);
            let Some(t546) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081dbc3); };
            c.cmp(t546, c.r32(12), 32);
            if c.cc_ne() { blk = 494; continue; }
            blk = 482; continue;
        }
        482 => {
            let Some(t547) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081dbd0); };
            c.w32(12, t547);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 483; continue;
        }
        483 => {
            c.cmp(c.r64(15), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 609; continue; }
            blk = 484; continue;
        }
        484 => {
            let Some(t548) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081dbf7); };
            c.w64(1, t548);
            let Some(t549) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081dbfe); };
            c.w64(11, t549);
            c.w64(3, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(3).wrapping_add(c.r64(11)));
            let Some(t550) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081dc0d); };
            c.w64(0, t550);
            let Some(t551) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081dc14); };
            c.cmp(t551, c.r64(6), 64);
            if c.cc_be() { blk = 606; continue; }
            blk = 485; continue;
        }
        485 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081dc1d); }
            c.w64(1, c.r64(11).wrapping_add(0x1));
            blk = 486; continue;
        }
        486 => {
            c.w32(11, c.r64(15).wrapping_add(c.r64(15)));
            let Some(t552) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081dc28); };
            c.w64(15, t552);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x700081dc2f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x700081dc33); }
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_e() { blk = 489; continue; }
            blk = 487; continue;
        }
        487 => {
            let Some(t553) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081dc4c); };
            c.w64(12, t553);
            blk = 488; continue;
        }
        488 => {
            let r554 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r554);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081dc57); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ne() { blk = 488; continue; }
            blk = 489; continue;
        }
        489 => {
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_e() { blk = 491; continue; }
            blk = 490; continue;
        }
        490 => {
            let r555 = c.sub(c.r64(9), 0x4u64, 64);
            c.w64(9, r555);
            let Some(t556) = c.ld32(c.r64(9).wrapping_add(c.r64(7))) else { return RegfileExit::fault(0x700081dc6a); };
            c.w32(11, t556);
            if !c.st32(c.r64(9).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081dc6e); }
            c.cmp(c.r64(9), 0x7u64, 64);
            if c.cc_ne() { blk = 490; continue; }
            blk = 491; continue;
        }
        491 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081dc78); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 493; continue; }
            blk = 492; continue;
        }
        492 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r557 = c.r64(7) & c.r64(2);
            c.w64(7, r557);
            c.logic(r557, 64);
            let Some(t558) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081dc8a); };
            c.logic(t558 & 0x4u64, 8);
            if c.cc_ne() { blk = 628; continue; }
            blk = 493; continue;
        }
        493 => {
            c.w64(7, c.r64(1));
            blk = 494; continue;
        }
        494 => {
            let Some(t559) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081dc97); };
            c.w32(1, t559);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_ae() { blk = 608; continue; }
            blk = 495; continue;
        }
        495 => {
            c.w64(1, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            let Some(t560) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081dcb1); };
            c.w64(0, t560);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081dcb5); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 497; continue; }
            blk = 496; continue;
        }
        496 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r561 = c.r64(8) & c.r64(7);
            c.w64(8, r561);
            c.logic(r561, 64);
            let Some(t562) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081dcc6); };
            c.logic(t562 & 0x4u64, 8);
            if c.cc_ne() { blk = 625; continue; }
            blk = 497; continue;
        }
        497 => {
            blk = 533; continue;
        }
        498 => {
            c.w64(8, c.r64(2));
            let r563 = c.r64(8) & 0xfffffffffffc0000u64;
            c.w64(8, r563);
            c.logic(r563, 64);
            let Some(t564) = c.ld64(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081dce0); };
            c.w64(9, t564);
            c.logic(c.r8(9) & 0x18u64, 8);
            if c.cc_e() { blk = 505; continue; }
            blk = 499; continue;
        }
        499 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 505; continue; }
            blk = 500; continue;
        }
        500 => {
            c.w64(9, c.r64(2).wrapping_add(0x13));
            let Some(t565) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081dcf4); };
            c.w64(12, t565);
            let Some(t566) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081dcfb); };
            c.w64(12, t566);
            c.w64(15, c.r64(9));
            let r567 = c.r64(15) & 0xfffffffffffc0000u64;
            c.w64(15, r567);
            c.logic(r567, 64);
            c.w64(3, c.r64(12));
            let r568 = c.r64(3) & 0xfffffffffffc0000u64;
            c.w64(3, r568);
            c.logic(r568, 64);
            c.cmp(c.r64(3), c.r64(15), 64);
            if c.cc_e() { blk = 503; continue; }
            blk = 501; continue;
        }
        501 => {
            c.cmp(c.r64(15), c.r64(8), 64);
            if c.cc_e() { blk = 504; continue; }
            blk = 502; continue;
        }
        502 => {
            blk = 505; continue;
        }
        503 => {
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_be() { blk = 505; continue; }
            blk = 504; continue;
        }
        504 => {
            let Some(t569) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081dd24); };
            c.w32(8, t569);
            let Some(t570) = c.ld32(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x700081dd28); };
            c.cmp(t570, c.r32(8), 32);
            if c.cc_e() { blk = 534; continue; }
            blk = 505; continue;
        }
        505 => {
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081dd35); }
            c.w64(8, 0xfffffffffffc0000u64);
            let r571 = c.r64(8) & c.r64(2);
            c.w64(8, r571);
            c.logic(r571, 64);
            let Some(t572) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081dd43); };
            c.logic(t572 & 0x4u64, 8);
            if c.cc_ne() { blk = 622; continue; }
            blk = 506; continue;
        }
        506 => {
            let Some(t573) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081dd4e); };
            c.w32(8, t573);
            let r574 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r574);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 512; continue; }
            blk = 507; continue;
        }
        507 => {
            let Some(t575) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081dd5a); };
            c.w32(9, t575);
            let r576 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r576);
            let Some(t577) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081dd61); };
            c.cmp(t577, 0x42u64, 16);
            if c.cc_ne() { blk = 534; continue; }
            blk = 508; continue;
        }
        508 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x700081dd6d); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 534; continue; }
            blk = 509; continue;
        }
        509 => {
            if c.cc_ne() { blk = 534; continue; }
            blk = 510; continue;
        }
        510 => {
            let Some(t578) = c.ld64(0x700081c16bu64) else { return RegfileExit::fault(0x700081dd90); };
            c.w64(12, t578);
            let r579 = c.add(c.r64(12), c.r64(9), 64);
            c.w64(12, r579);
            let Some(t580) = c.ld64(0x700081c178u64) else { return RegfileExit::fault(0x700081dd9a); };
            c.w64(15, t580);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 513; continue; }
            blk = 511; continue;
        }
        511 => {
            blk = 534; continue;
        }
        512 => {
            c.w64(9, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            blk = 513; continue;
        }
        513 => {
            let Some(t581) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ddb1); };
            c.w32(12, t581);
            let r582 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r582);
            let Some(t583) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ddb8); };
            c.cmp(t583, 0x834u64, 16);
            if c.cc_e() { blk = 515; continue; }
            blk = 514; continue;
        }
        514 => {
            let Some(t584) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ddc2); };
            c.w32(12, t584);
            blk = 516; continue;
        }
        515 => {
            let Some(t585) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081ddc8); };
            c.w32(12, t585);
            blk = 516; continue;
        }
        516 => {
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(9), c.r64(15), 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 517; continue;
        }
        517 => {
            let Some(t586) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ddde); };
            c.w32(15, t586);
            let Some(t587) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x700081dde2); };
            c.cmp(t587, c.r32(15), 32);
            if c.cc_ne() { blk = 530; continue; }
            blk = 518; continue;
        }
        518 => {
            let Some(t588) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ddef); };
            c.w32(15, t588);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(15));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x7ffeu64, 64);
            if c.cc_ae() { blk = 534; continue; }
            blk = 519; continue;
        }
        519 => {
            c.cmp(c.r64(3), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 614; continue; }
            blk = 520; continue;
        }
        520 => {
            let Some(t589) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700081de16); };
            c.w64(1, t589);
            let Some(t590) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x700081de1d); };
            c.w64(7, t590);
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x8));
            c.w64(6, c.r64(11).wrapping_add(c.r64(7)));
            let Some(t591) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700081de2c); };
            c.w64(0, t591);
            let Some(t592) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x700081de33); };
            c.cmp(t592, c.r64(6), 64);
            if c.cc_be() { blk = 611; continue; }
            blk = 521; continue;
        }
        521 => {
            if !c.st64(c.r64(1), c.r64(6)) { return RegfileExit::fault(0x700081de3c); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 522; continue;
        }
        522 => {
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t593) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700081de46); };
            c.w64(11, t593);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x700081de4d); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x700081de51); }
            c.w64(7, c.r64(12).wrapping_mul(2).wrapping_add(0x7));
            c.w64(11, c.r64(15).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_e() { blk = 525; continue; }
            blk = 523; continue;
        }
        523 => {
            let Some(t594) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081de69); };
            c.w64(12, t594);
            blk = 524; continue;
        }
        524 => {
            let r595 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r595);
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x700081de74); }
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_ne() { blk = 524; continue; }
            blk = 525; continue;
        }
        525 => {
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_e() { blk = 527; continue; }
            blk = 526; continue;
        }
        526 => {
            let r596 = c.sub(c.r64(7), 0x4u64, 64);
            c.w64(7, r596);
            let Some(t597) = c.ld32(c.r64(7).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x700081de87); };
            c.w32(11, t597);
            if !c.st32(c.r64(7).wrapping_add(c.r64(1)), c.r32(11)) { return RegfileExit::fault(0x700081de8b); }
            c.cmp(c.r64(7), 0x7u64, 64);
            if c.cc_ne() { blk = 526; continue; }
            blk = 527; continue;
        }
        527 => {
            if !c.st32(c.r64(2).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x700081de95); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 529; continue; }
            blk = 528; continue;
        }
        528 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r598 = c.r64(7) & c.r64(2);
            c.w64(7, r598);
            c.logic(r598, 64);
            let Some(t599) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081dea7); };
            c.logic(t599 & 0x4u64, 8);
            if c.cc_ne() { blk = 619; continue; }
            blk = 529; continue;
        }
        529 => {
            c.w64(8, c.r64(1));
            blk = 530; continue;
        }
        530 => {
            let Some(t600) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081deb4); };
            c.w32(1, t600);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(9), c.r64(1), 64);
            if c.cc_ae() { blk = 613; continue; }
            blk = 531; continue;
        }
        531 => {
            c.w64(1, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t601) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081decf); };
            c.w64(0, t601);
            if !c.st32(c.r64(8).wrapping_add(c.r64(1)), c.r32(0)) { return RegfileExit::fault(0x700081ded3); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 533; continue; }
            blk = 532; continue;
        }
        532 => {
            c.w64(7, 0xfffffffffffc0000u64);
            let r602 = c.r64(7) & c.r64(8);
            c.w64(7, r602);
            c.logic(r602, 64);
            let Some(t603) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081dee5); };
            c.logic(t603 & 0x4u64, 8);
            if c.cc_ne() { blk = 616; continue; }
            blk = 533; continue;
        }
        533 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081def2); }
            return c.ret(0x8, 0x700081def3);
        }
        534 => {
            let Some(t604) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700081def6); };
            c.w64(5, t604);
            let Some(t605) = c.ld64(c.r64(13).wrapping_add(0x2010)) else { return RegfileExit::fault(0x700081defa); };
            c.w64(3, t605);
            let Some(t606) = c.ld64(c.r64(4).wrapping_add(0x70)) else { return RegfileExit::fault(0x700081df01); };
            c.w64(10, t606);
            if !c.st64(c.r64(4).wrapping_add(0x48), c.r64(10)) { return RegfileExit::fault(0x700081df06); }
            let Some(t607) = c.ld64(c.r64(4).wrapping_add(0x78)) else { return RegfileExit::fault(0x700081df0b); };
            c.w64(10, t607);
            if !c.st64(c.r64(4).wrapping_add(0x50), c.r64(10)) { return RegfileExit::fault(0x700081df10); }
            if !c.st64(c.r64(4).wrapping_add(0x78), c.r64(2)) { return RegfileExit::fault(0x700081df15); }
            if !c.st64(c.r64(4).wrapping_add(0x70), c.r64(1)) { return RegfileExit::fault(0x700081df1a); }
            if !c.st64(c.r64(4).wrapping_add(0x68), c.r64(0)) { return RegfileExit::fault(0x700081df1f); }
            if !c.st64(c.r64(4).wrapping_add(0x60), c.r64(11)) { return RegfileExit::fault(0x700081df24); }
            let Some(t608) = c.ld64(c.r64(4).wrapping_add(0x38)) else { return RegfileExit::fault(0x700081df29); };
            c.w64(10, t608);
            if !c.st64(c.r64(4).wrapping_add(0x58), c.r64(10)) { return RegfileExit::fault(0x700081df2e); }
            c.w32(0, 0x6u64);
            let r609 = c.add(c.r64(4), 0x48u64, 64);
            c.w64(4, r609);
            return RegfileExit::jump(0x700086ed74);
        }
        535 => {
            return RegfileExit::fault(0x700081df41);
        }
        536 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081df42); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700081df46); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081df4a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700081df4e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081df52); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700081df56); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700081df62, 0x700081df5d);
        }
        537 => {
            c.w64(1, c.r64(0));
            let Some(t610) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081df65); };
            c.w64(2, t610);
            let Some(t611) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081df69); };
            c.w64(9, t611);
            let Some(t612) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081df6d); };
            c.w64(8, t612);
            let Some(t613) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081df71); };
            c.w64(15, t613);
            let Some(t614) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081df75); };
            c.w64(12, t614);
            let Some(t615) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081df79); };
            c.w64(3, t615);
            blk = 44; continue;
        }
        538 => {
            return RegfileExit::fault(0x700081df82);
        }
        539 => {
            let Some(t616) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081df83); };
            c.w64(3, t616);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081df93, 0x700081df8e);
        }
        540 => {
            return RegfileExit::fault(0x700081df93);
        }
        541 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t617) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081df96); };
            c.w64(1, t617);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700081dfa7, 0x700081dfa2);
        }
        542 => {
            return RegfileExit::fault(0x700081dfa7);
        }
        543 => {
            let r618 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r618);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081dfab); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081dfaf); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081dfb0); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700081dfb4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081dfb8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081dfbc); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081dfc0); }
            c.w32(0, 0x2u64);
            let Some(t619) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081dfc9); };
            c.w64(3, t619);
            return c.call(0x700086ed74, 0x700081dfd5, 0x700081dfd0);
        }
        544 => {
            c.w64(15, c.r64(0));
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081dfd8); };
            c.w64(2, t620);
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081dfdc); };
            c.w64(1, t621);
            let Some(t622) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081dfe0); };
            c.w64(11, t622);
            let Some(t623) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081dfe4); };
            c.w64(7, t623);
            let Some(t624) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081dfe8); };
            c.w64(8, t624);
            let Some(t625) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081dfec); };
            c.w64(9, t625);
            let Some(t626) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081dff0); };
            c.w64(12, t626);
            blk = 70; continue;
        }
        545 => {
            let Some(t627) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081dff9); };
            c.w64(3, t627);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e009, 0x700081e004);
        }
        546 => {
            return RegfileExit::fault(0x700081e009);
        }
        547 => {
            return RegfileExit::fault(0x700081e00a);
        }
        548 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t628) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081e00d); };
            c.w64(1, t628);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700081e01e, 0x700081e019);
        }
        549 => {
            return RegfileExit::fault(0x700081e01e);
        }
        550 => {
            let r629 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r629);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081e022); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081e026); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e027); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700081e02b); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e02f); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e033); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e037); }
            let Some(t630) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081e03b); };
            c.w64(3, t630);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700081e04c, 0x700081e047);
        }
        551 => {
            c.w64(15, c.r64(0));
            let Some(t631) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e04f); };
            c.w64(2, t631);
            let Some(t632) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081e053); };
            c.w64(1, t632);
            let Some(t633) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081e057); };
            c.w64(11, t633);
            let Some(t634) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e05b); };
            c.w64(7, t634);
            let Some(t635) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e05f); };
            c.w64(8, t635);
            let Some(t636) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e063); };
            c.w64(9, t636);
            let Some(t637) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e067); };
            c.w64(12, t637);
            blk = 116; continue;
        }
        552 => {
            let Some(t638) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e070); };
            c.w64(3, t638);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e080, 0x700081e07b);
        }
        553 => {
            return RegfileExit::fault(0x700081e080);
        }
        554 => {
            return RegfileExit::fault(0x700081e081);
        }
        555 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e082); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700081e086); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e08a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700081e08e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e092); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700081e096); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700081e0a2, 0x700081e09d);
        }
        556 => {
            c.w64(1, c.r64(0));
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e0a5); };
            c.w64(2, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e0a9); };
            c.w64(9, t640);
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e0ad); };
            c.w64(8, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e0b1); };
            c.w64(15, t642);
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e0b5); };
            c.w64(12, t643);
            let Some(t644) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081e0b9); };
            c.w64(3, t644);
            blk = 168; continue;
        }
        557 => {
            return RegfileExit::fault(0x700081e0c2);
        }
        558 => {
            let Some(t645) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e0c3); };
            c.w64(3, t645);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e0d3, 0x700081e0ce);
        }
        559 => {
            return RegfileExit::fault(0x700081e0d3);
        }
        560 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e0d4); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700081e0d8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e0dc); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700081e0e0); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e0e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700081e0e8); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700081e0f4, 0x700081e0ef);
        }
        561 => {
            c.w64(1, c.r64(0));
            let Some(t646) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e0f7); };
            c.w64(2, t646);
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e0fb); };
            c.w64(9, t647);
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e0ff); };
            c.w64(8, t648);
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e103); };
            c.w64(15, t649);
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e107); };
            c.w64(12, t650);
            let Some(t651) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081e10b); };
            c.w64(3, t651);
            blk = 204; continue;
        }
        562 => {
            return RegfileExit::fault(0x700081e114);
        }
        563 => {
            let Some(t652) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e115); };
            c.w64(3, t652);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e125, 0x700081e120);
        }
        564 => {
            return RegfileExit::fault(0x700081e125);
        }
        565 => {
            c.w32(6, 0);
            c.logic(0, 32);
            let Some(t653) = c.ld64(c.r64(13).wrapping_add(0x2118)) else { return RegfileExit::fault(0x700081e128); };
            c.w64(1, t653);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(1));
            return c.call(0x700086ed74, 0x700081e139, 0x700081e134);
        }
        566 => {
            return RegfileExit::fault(0x700081e139);
        }
        567 => {
            let r654 = c.shl(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r654);
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700081e13d); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081e141); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e142); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700081e146); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e14a); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e14e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e152); }
            let Some(t655) = c.ld64(c.r64(13).wrapping_add(0x20d8)) else { return RegfileExit::fault(0x700081e156); };
            c.w64(3, t655);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x700081e167, 0x700081e162);
        }
        568 => {
            c.w64(15, c.r64(0));
            let Some(t656) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e16a); };
            c.w64(2, t656);
            let Some(t657) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081e16e); };
            c.w64(1, t657);
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081e172); };
            c.w64(11, t658);
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e176); };
            c.w64(7, t659);
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e17a); };
            c.w64(8, t660);
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e17e); };
            c.w64(9, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e182); };
            c.w64(12, t662);
            blk = 232; continue;
        }
        569 => {
            let Some(t663) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e18b); };
            c.w64(3, t663);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e19b, 0x700081e196);
        }
        570 => {
            return RegfileExit::fault(0x700081e19b);
        }
        571 => {
            return RegfileExit::fault(0x700081e19c);
        }
        572 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e19d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700081e1a1); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e1a5); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700081e1a9); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e1ad); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700081e1b1); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700081e1bd, 0x700081e1b8);
        }
        573 => {
            c.w64(1, c.r64(0));
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e1c0); };
            c.w64(2, t664);
            let Some(t665) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e1c4); };
            c.w64(9, t665);
            let Some(t666) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e1c8); };
            c.w64(8, t666);
            let Some(t667) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e1cc); };
            c.w64(15, t667);
            let Some(t668) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e1d0); };
            c.w64(12, t668);
            let Some(t669) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081e1d4); };
            c.w64(3, t669);
            blk = 284; continue;
        }
        574 => {
            return RegfileExit::fault(0x700081e1dd);
        }
        575 => {
            let Some(t670) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e1de); };
            c.w64(3, t670);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e1ee, 0x700081e1e9);
        }
        576 => {
            return RegfileExit::fault(0x700081e1ee);
        }
        577 => {
            return RegfileExit::fault(0x700081e1ef);
        }
        578 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e1f0); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081e1f4); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e1f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e1fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e200); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700081e20c, 0x700081e207);
        }
        579 => {
            c.w64(1, c.r64(0));
            let Some(t671) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e20f); };
            c.w64(2, t671);
            let Some(t672) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081e213); };
            c.w64(11, t672);
            let Some(t673) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081e217); };
            c.w64(7, t673);
            let Some(t674) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e21b); };
            c.w64(8, t674);
            let Some(t675) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e21f); };
            c.w64(9, t675);
            let Some(t676) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e223); };
            c.w64(12, t676);
            blk = 336; continue;
        }
        580 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e22c); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700081e235); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e239); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e23d); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700081e241); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x700081e245); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081e24a); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700081e256, 0x700081e251);
        }
        581 => {
            c.w64(11, c.r64(0));
            let Some(t677) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e259); };
            c.w64(2, t677);
            let Some(t678) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e25d); };
            c.w64(1, t678);
            let Some(t679) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e261); };
            c.w64(8, t679);
            let Some(t680) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e265); };
            c.w64(9, t680);
            let Some(t681) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e269); };
            c.w64(6, t681);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700081e26d); }
            let Some(t682) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081e272); };
            c.w64(7, t682);
            let Some(t683) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081e276); };
            c.w64(12, t683);
            blk = 345; continue;
        }
        582 => {
            let Some(t684) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e282); };
            c.w64(3, t684);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e292, 0x700081e28d);
        }
        583 => {
            return RegfileExit::fault(0x700081e292);
        }
        584 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e293); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700081e297); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700081e29b); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700081e29f); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700081e2a3); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700081e2a7); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700081e2b3, 0x700081e2ae);
        }
        585 => {
            c.w64(1, c.r64(0));
            let Some(t685) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e2b6); };
            c.w64(2, t685);
            let Some(t686) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e2ba); };
            c.w64(8, t686);
            let Some(t687) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e2be); };
            c.w64(7, t687);
            let Some(t688) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e2c2); };
            c.w64(12, t688);
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e2c6); };
            c.w64(9, t689);
            let Some(t690) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081e2ca); };
            c.w64(15, t690);
            blk = 368; continue;
        }
        586 => {
            return RegfileExit::fault(0x700081e2d3);
        }
        587 => {
            let Some(t691) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e2d4); };
            c.w64(3, t691);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e2e4, 0x700081e2df);
        }
        588 => {
            return RegfileExit::fault(0x700081e2e4);
        }
        589 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e2e5); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081e2e9); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e2ed); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e2f1); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e2f5); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700081e301, 0x700081e2fc);
        }
        590 => {
            c.w64(1, c.r64(0));
            let Some(t692) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e304); };
            c.w64(2, t692);
            let Some(t693) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081e308); };
            c.w64(11, t693);
            let Some(t694) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081e30c); };
            c.w64(7, t694);
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e310); };
            c.w64(8, t695);
            let Some(t696) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e314); };
            c.w64(9, t696);
            let Some(t697) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e318); };
            c.w64(12, t697);
            blk = 395; continue;
        }
        591 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e321); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700081e32a); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e32e); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e332); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700081e336); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x700081e33a); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081e33f); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700081e34b, 0x700081e346);
        }
        592 => {
            c.w64(11, c.r64(0));
            let Some(t698) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e34e); };
            c.w64(2, t698);
            let Some(t699) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e352); };
            c.w64(1, t699);
            let Some(t700) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e356); };
            c.w64(8, t700);
            let Some(t701) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e35a); };
            c.w64(9, t701);
            let Some(t702) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e35e); };
            c.w64(6, t702);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700081e362); }
            let Some(t703) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081e367); };
            c.w64(7, t703);
            let Some(t704) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081e36b); };
            c.w64(12, t704);
            blk = 404; continue;
        }
        593 => {
            let Some(t705) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e377); };
            c.w64(3, t705);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e387, 0x700081e382);
        }
        594 => {
            return RegfileExit::fault(0x700081e387);
        }
        595 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e388); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700081e38c); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700081e390); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700081e394); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700081e398); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700081e39c); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700081e3a8, 0x700081e3a3);
        }
        596 => {
            c.w64(1, c.r64(0));
            let Some(t706) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e3ab); };
            c.w64(2, t706);
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e3af); };
            c.w64(8, t707);
            let Some(t708) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e3b3); };
            c.w64(7, t708);
            let Some(t709) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e3b7); };
            c.w64(12, t709);
            let Some(t710) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e3bb); };
            c.w64(9, t710);
            let Some(t711) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081e3bf); };
            c.w64(15, t711);
            blk = 427; continue;
        }
        597 => {
            return RegfileExit::fault(0x700081e3c8);
        }
        598 => {
            let Some(t712) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e3c9); };
            c.w64(3, t712);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e3d9, 0x700081e3d4);
        }
        599 => {
            return RegfileExit::fault(0x700081e3d9);
        }
        600 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e3da); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081e3de); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e3e2); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e3e6); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e3ea); }
            c.w64(2, c.r64(6));
            return c.call(0x70007febf4, 0x700081e3f6, 0x700081e3f1);
        }
        601 => {
            c.w64(1, c.r64(0));
            let Some(t713) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e3f9); };
            c.w64(2, t713);
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081e3fd); };
            c.w64(11, t714);
            let Some(t715) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081e401); };
            c.w64(7, t715);
            let Some(t716) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e405); };
            c.w64(8, t716);
            let Some(t717) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e409); };
            c.w64(9, t717);
            let Some(t718) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e40d); };
            c.w64(12, t718);
            blk = 454; continue;
        }
        602 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e416); }
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x700081e41f); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e423); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(9)) { return RegfileExit::fault(0x700081e427); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(6)) { return RegfileExit::fault(0x700081e42b); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x60), 0) { return RegfileExit::fault(0x700081e42f); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x700081e434); }
            c.w64(11, c.r64(2));
            return c.call(0x70007febf4, 0x700081e440, 0x700081e43b);
        }
        603 => {
            c.w64(11, c.r64(0));
            let Some(t719) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e443); };
            c.w64(2, t719);
            let Some(t720) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e447); };
            c.w64(1, t720);
            let Some(t721) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e44b); };
            c.w64(8, t721);
            let Some(t722) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e44f); };
            c.w64(9, t722);
            let Some(t723) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e453); };
            c.w64(6, t723);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x60)) { return RegfileExit::fault(0x700081e457); }
            let Some(t724) = c.ld64(0x700081c41cu64) else { return RegfileExit::fault(0x700081e45c); };
            c.w64(12, t724);
            let Some(t725) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x700081e463); };
            c.w64(7, t725);
            blk = 463; continue;
        }
        604 => {
            let Some(t726) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e46c); };
            c.w64(3, t726);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e47c, 0x700081e477);
        }
        605 => {
            return RegfileExit::fault(0x700081e47c);
        }
        606 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e47d); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x700081e481); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(7)) { return RegfileExit::fault(0x700081e485); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x700081e489); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(9)) { return RegfileExit::fault(0x700081e48d); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(15)) { return RegfileExit::fault(0x700081e491); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febf4, 0x700081e49d, 0x700081e498);
        }
        607 => {
            c.w64(1, c.r64(0));
            let Some(t727) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e4a0); };
            c.w64(2, t727);
            let Some(t728) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e4a4); };
            c.w64(8, t728);
            let Some(t729) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e4a8); };
            c.w64(7, t729);
            let Some(t730) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e4ac); };
            c.w64(12, t730);
            let Some(t731) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e4b0); };
            c.w64(9, t731);
            let Some(t732) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081e4b4); };
            c.w64(15, t732);
            blk = 486; continue;
        }
        608 => {
            return RegfileExit::fault(0x700081e4bd);
        }
        609 => {
            let Some(t733) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e4be); };
            c.w64(3, t733);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e4ce, 0x700081e4c9);
        }
        610 => {
            return RegfileExit::fault(0x700081e4ce);
        }
        611 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x700081e4cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x700081e4d3); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x700081e4d7); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x700081e4db); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081e4df); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(3)) { return RegfileExit::fault(0x700081e4e3); }
            c.w64(2, c.r64(11));
            return c.call(0x70007febf4, 0x700081e4ef, 0x700081e4ea);
        }
        612 => {
            c.w64(1, c.r64(0));
            let Some(t734) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081e4f2); };
            c.w64(2, t734);
            let Some(t735) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081e4f6); };
            c.w64(9, t735);
            let Some(t736) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081e4fa); };
            c.w64(8, t736);
            let Some(t737) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081e4fe); };
            c.w64(15, t737);
            let Some(t738) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081e502); };
            c.w64(12, t738);
            let Some(t739) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x700081e506); };
            c.w64(3, t739);
            blk = 522; continue;
        }
        613 => {
            return RegfileExit::fault(0x700081e50f);
        }
        614 => {
            let Some(t740) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x700081e510); };
            c.w64(3, t740);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700081e520, 0x700081e51b);
        }
        615 => {
            return RegfileExit::fault(0x700081e520);
        }
        616 => {
            c.w32(0, c.r32(0));
            let r741 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r741);
            c.w64(7, 0xfffffffffffc0000u64);
            let r742 = c.r64(7) & c.r64(0);
            c.w64(7, r742);
            c.logic(r742, 64);
            let Some(t743) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e530); };
            c.logic(t743 & 0x2u64, 8);
            if c.cc_e() { blk = 533; continue; }
            blk = 617; continue;
        }
        617 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e53e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e53f); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e54c, 0x700081e547);
        }
        618 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e54c); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e54e); }
            blk = 533; continue;
        }
        619 => {
            c.w32(1, c.r32(1));
            let r744 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r744);
            c.w64(7, 0xfffffffffffc0000u64);
            let r745 = c.r64(7) & c.r64(1);
            c.w64(7, r745);
            c.logic(r745, 64);
            let Some(t746) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e563); };
            c.logic(t746 & 0x2u64, 8);
            if c.cc_e() { blk = 529; continue; }
            blk = 620; continue;
        }
        620 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e571); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081e57a, 0x700081e575);
        }
        621 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e57a); }
            blk = 529; continue;
        }
        622 => {
            c.w32(11, c.r32(11));
            let r747 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r747);
            c.w64(8, 0xfffffffffffc0000u64);
            let r748 = c.r64(8) & c.r64(11);
            c.w64(8, r748);
            c.logic(r748, 64);
            let Some(t749) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e590); };
            c.logic(t749 & 0x2u64, 8);
            if c.cc_e() { blk = 506; continue; }
            blk = 623; continue;
        }
        623 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e59f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e5a0); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081e5ad, 0x700081e5a8);
        }
        624 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e5ad); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e5af); }
            blk = 506; continue;
        }
        625 => {
            c.w32(0, c.r32(0));
            let r750 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r750);
            c.w64(8, 0xfffffffffffc0000u64);
            let r751 = c.r64(8) & c.r64(0);
            c.w64(8, r751);
            c.logic(r751, 64);
            let Some(t752) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e5c4); };
            c.logic(t752 & 0x2u64, 8);
            if c.cc_e() { blk = 497; continue; }
            blk = 626; continue;
        }
        626 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e5d3); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e5dd, 0x700081e5d8);
        }
        627 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e5dd); }
            blk = 497; continue;
        }
        628 => {
            c.w32(1, c.r32(1));
            let r753 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r753);
            c.w64(7, 0xfffffffffffc0000u64);
            let r754 = c.r64(7) & c.r64(1);
            c.w64(7, r754);
            c.logic(r754, 64);
            let Some(t755) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e5f3); };
            c.logic(t755 & 0x2u64, 8);
            if c.cc_e() { blk = 493; continue; }
            blk = 629; continue;
        }
        629 => {
            c.w64(9, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e601); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e602); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e60f, 0x700081e60a);
        }
        630 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e60f); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e611); }
            blk = 493; continue;
        }
        631 => {
            c.w32(11, c.r32(11));
            let r756 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r756);
            c.w64(7, 0xfffffffffffc0000u64);
            let r757 = c.r64(7) & c.r64(11);
            c.w64(7, r757);
            c.logic(r757, 64);
            let Some(t758) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e627); };
            c.logic(t758 & 0x2u64, 8);
            if c.cc_e() { blk = 470; continue; }
            blk = 632; continue;
        }
        632 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e635); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081e63e, 0x700081e639);
        }
        633 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e63e); }
            blk = 470; continue;
        }
        634 => {
            c.w32(1, c.r32(1));
            let r759 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r759);
            c.w64(7, 0xfffffffffffc0000u64);
            let r760 = c.r64(7) & c.r64(1);
            c.w64(7, r760);
            c.logic(r760, 64);
            let Some(t761) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e653); };
            c.logic(t761 & 0x2u64, 8);
            if c.cc_e() { blk = 468; continue; }
            blk = 635; continue;
        }
        635 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e661); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081e66a, 0x700081e665);
        }
        636 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e66a); }
            blk = 468; continue;
        }
        637 => {
            c.w32(11, c.r32(11));
            let r762 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r762);
            c.w64(15, 0xfffffffffffc0000u64);
            let r763 = c.r64(15) & c.r64(11);
            c.w64(15, r763);
            c.logic(r763, 64);
            let Some(t764) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e680); };
            c.logic(t764 & 0x2u64, 8);
            if c.cc_e() { blk = 465; continue; }
            blk = 638; continue;
        }
        638 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e68f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e690); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700081e69d, 0x700081e698);
        }
        639 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e69d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e69f); }
            blk = 465; continue;
        }
        640 => {
            c.w32(0, c.r32(0));
            let r765 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r765);
            c.w64(8, 0xfffffffffffc0000u64);
            let r766 = c.r64(8) & c.r64(0);
            c.w64(8, r766);
            c.logic(r766, 64);
            let Some(t767) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e6b4); };
            c.logic(t767 & 0x2u64, 8);
            if c.cc_e() { blk = 438; continue; }
            blk = 641; continue;
        }
        641 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e6c3); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e6cd, 0x700081e6c8);
        }
        642 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e6cd); }
            blk = 438; continue;
        }
        643 => {
            c.w32(1, c.r32(1));
            let r768 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r768);
            c.w64(7, 0xfffffffffffc0000u64);
            let r769 = c.r64(7) & c.r64(1);
            c.w64(7, r769);
            c.logic(r769, 64);
            let Some(t770) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e6e3); };
            c.logic(t770 & 0x2u64, 8);
            if c.cc_e() { blk = 434; continue; }
            blk = 644; continue;
        }
        644 => {
            c.w64(9, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e6f1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e6f2); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e6ff, 0x700081e6fa);
        }
        645 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e6ff); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e701); }
            blk = 434; continue;
        }
        646 => {
            c.w32(11, c.r32(11));
            let r771 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r771);
            c.w64(7, 0xfffffffffffc0000u64);
            let r772 = c.r64(7) & c.r64(11);
            c.w64(7, r772);
            c.logic(r772, 64);
            let Some(t773) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e717); };
            c.logic(t773 & 0x2u64, 8);
            if c.cc_e() { blk = 411; continue; }
            blk = 647; continue;
        }
        647 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e725); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081e72e, 0x700081e729);
        }
        648 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e72e); }
            blk = 411; continue;
        }
        649 => {
            c.w32(1, c.r32(1));
            let r774 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r774);
            c.w64(7, 0xfffffffffffc0000u64);
            let r775 = c.r64(7) & c.r64(1);
            c.w64(7, r775);
            c.logic(r775, 64);
            let Some(t776) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e743); };
            c.logic(t776 & 0x2u64, 8);
            if c.cc_e() { blk = 409; continue; }
            blk = 650; continue;
        }
        650 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e751); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081e75a, 0x700081e755);
        }
        651 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e75a); }
            blk = 409; continue;
        }
        652 => {
            c.w32(11, c.r32(11));
            let r777 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r777);
            c.w64(15, 0xfffffffffffc0000u64);
            let r778 = c.r64(15) & c.r64(11);
            c.w64(15, r778);
            c.logic(r778, 64);
            let Some(t779) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e770); };
            c.logic(t779 & 0x2u64, 8);
            if c.cc_e() { blk = 406; continue; }
            blk = 653; continue;
        }
        653 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e77f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e780); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700081e78d, 0x700081e788);
        }
        654 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e78d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e78f); }
            blk = 406; continue;
        }
        655 => {
            c.w32(0, c.r32(0));
            let r780 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r780);
            c.w64(8, 0xfffffffffffc0000u64);
            let r781 = c.r64(8) & c.r64(0);
            c.w64(8, r781);
            c.logic(r781, 64);
            let Some(t782) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e7a4); };
            c.logic(t782 & 0x2u64, 8);
            if c.cc_e() { blk = 379; continue; }
            blk = 656; continue;
        }
        656 => {
            c.w64(9, c.r64(7).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e7b3); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e7bd, 0x700081e7b8);
        }
        657 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e7bd); }
            blk = 379; continue;
        }
        658 => {
            c.w32(1, c.r32(1));
            let r783 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r783);
            c.w64(7, 0xfffffffffffc0000u64);
            let r784 = c.r64(7) & c.r64(1);
            c.w64(7, r784);
            c.logic(r784, 64);
            let Some(t785) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e7d3); };
            c.logic(t785 & 0x2u64, 8);
            if c.cc_e() { blk = 375; continue; }
            blk = 659; continue;
        }
        659 => {
            c.w64(9, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e7e1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e7e2); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e7ef, 0x700081e7ea);
        }
        660 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e7ef); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e7f1); }
            blk = 375; continue;
        }
        661 => {
            c.w32(11, c.r32(11));
            let r786 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r786);
            c.w64(7, 0xfffffffffffc0000u64);
            let r787 = c.r64(7) & c.r64(11);
            c.w64(7, r787);
            c.logic(r787, 64);
            let Some(t788) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e807); };
            c.logic(t788 & 0x2u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 662; continue;
        }
        662 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e815); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081e81e, 0x700081e819);
        }
        663 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e81e); }
            blk = 352; continue;
        }
        664 => {
            c.w32(1, c.r32(1));
            let r789 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r789);
            c.w64(7, 0xfffffffffffc0000u64);
            let r790 = c.r64(7) & c.r64(1);
            c.w64(7, r790);
            c.logic(r790, 64);
            let Some(t791) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e833); };
            c.logic(t791 & 0x2u64, 8);
            if c.cc_e() { blk = 350; continue; }
            blk = 665; continue;
        }
        665 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e841); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081e84a, 0x700081e845);
        }
        666 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e84a); }
            blk = 350; continue;
        }
        667 => {
            c.w32(11, c.r32(11));
            let r792 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r792);
            c.w64(15, 0xfffffffffffc0000u64);
            let r793 = c.r64(15) & c.r64(11);
            c.w64(15, r793);
            c.logic(r793, 64);
            let Some(t794) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e860); };
            c.logic(t794 & 0x2u64, 8);
            if c.cc_e() { blk = 347; continue; }
            blk = 668; continue;
        }
        668 => {
            c.w64(0, c.r64(1).wrapping_add(c.r64(6)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e86f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e870); }
            c.w64(7, c.r64(1));
            c.w64(8, c.r64(0));
            return c.call(0x70007f0e54, 0x700081e87d, 0x700081e878);
        }
        669 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e87d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e87f); }
            blk = 347; continue;
        }
        670 => {
            c.w32(11, c.r32(11));
            let r795 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r795);
            c.w64(8, 0xfffffffffffc0000u64);
            let r796 = c.r64(8) & c.r64(11);
            c.w64(8, r796);
            c.logic(r796, 64);
            let Some(t797) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e895); };
            c.logic(t797 & 0x2u64, 8);
            if c.cc_e() { blk = 304; continue; }
            blk = 671; continue;
        }
        671 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e8a4); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e8a5); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081e8b2, 0x700081e8ad);
        }
        672 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e8b2); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e8b4); }
            blk = 304; continue;
        }
        673 => {
            c.w32(0, c.r32(0));
            let r798 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r798);
            c.w64(7, 0xfffffffffffc0000u64);
            let r799 = c.r64(7) & c.r64(0);
            c.w64(7, r799);
            c.logic(r799, 64);
            let Some(t800) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e8c9); };
            c.logic(t800 & 0x2u64, 8);
            if c.cc_e() { blk = 295; continue; }
            blk = 674; continue;
        }
        674 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e8d7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e8d8); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e8e5, 0x700081e8e0);
        }
        675 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e8e5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e8e7); }
            blk = 295; continue;
        }
        676 => {
            c.w32(1, c.r32(1));
            let r801 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r801);
            c.w64(7, 0xfffffffffffc0000u64);
            let r802 = c.r64(7) & c.r64(1);
            c.w64(7, r802);
            c.logic(r802, 64);
            let Some(t803) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e8fc); };
            c.logic(t803 & 0x2u64, 8);
            if c.cc_e() { blk = 291; continue; }
            blk = 677; continue;
        }
        677 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e90a); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081e913, 0x700081e90e);
        }
        678 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e913); }
            blk = 291; continue;
        }
        679 => {
            c.w32(11, c.r32(11));
            let r804 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r804);
            c.w64(8, 0xfffffffffffc0000u64);
            let r805 = c.r64(8) & c.r64(11);
            c.w64(8, r805);
            c.logic(r805, 64);
            let Some(t806) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e929); };
            c.logic(t806 & 0x2u64, 8);
            if c.cc_e() { blk = 268; continue; }
            blk = 680; continue;
        }
        680 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e938); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e939); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081e946, 0x700081e941);
        }
        681 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e946); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e948); }
            blk = 268; continue;
        }
        682 => {
            c.w32(11, c.r32(11));
            let r807 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r807);
            c.w64(7, 0xfffffffffffc0000u64);
            let r808 = c.r64(7) & c.r64(11);
            c.w64(7, r808);
            c.logic(r808, 64);
            let Some(t809) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e95e); };
            c.logic(t809 & 0x2u64, 8);
            if c.cc_e() { blk = 243; continue; }
            blk = 683; continue;
        }
        683 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e96c); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081e975, 0x700081e970);
        }
        684 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e975); }
            blk = 243; continue;
        }
        685 => {
            c.w32(15, c.r32(15));
            let r810 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r810);
            c.w64(7, 0xfffffffffffc0000u64);
            let r811 = c.r64(7) & c.r64(15);
            c.w64(7, r811);
            c.logic(r811, 64);
            let Some(t812) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e98b); };
            c.logic(t812 & 0x2u64, 8);
            if c.cc_e() { blk = 241; continue; }
            blk = 686; continue;
        }
        686 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e999); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081e9a2, 0x700081e99d);
        }
        687 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e9a2); }
            blk = 241; continue;
        }
        688 => {
            c.w32(0, c.r32(0));
            let r813 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r813);
            c.w64(7, 0xfffffffffffc0000u64);
            let r814 = c.r64(7) & c.r64(0);
            c.w64(7, r814);
            c.logic(r814, 64);
            let Some(t815) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e9b7); };
            c.logic(t815 & 0x2u64, 8);
            if c.cc_e() { blk = 215; continue; }
            blk = 689; continue;
        }
        689 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e9c5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081e9c6); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081e9d3, 0x700081e9ce);
        }
        690 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081e9d3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081e9d5); }
            blk = 215; continue;
        }
        691 => {
            c.w32(1, c.r32(1));
            let r816 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r816);
            c.w64(7, 0xfffffffffffc0000u64);
            let r817 = c.r64(7) & c.r64(1);
            c.w64(7, r817);
            c.logic(r817, 64);
            let Some(t818) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081e9ea); };
            c.logic(t818 & 0x2u64, 8);
            if c.cc_e() { blk = 211; continue; }
            blk = 692; continue;
        }
        692 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081e9f8); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081ea01, 0x700081e9fc);
        }
        693 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081ea01); }
            blk = 211; continue;
        }
        694 => {
            c.w32(11, c.r32(11));
            let r819 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r819);
            c.w64(8, 0xfffffffffffc0000u64);
            let r820 = c.r64(8) & c.r64(11);
            c.w64(8, r820);
            c.logic(r820, 64);
            let Some(t821) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ea17); };
            c.logic(t821 & 0x2u64, 8);
            if c.cc_e() { blk = 188; continue; }
            blk = 695; continue;
        }
        695 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081ea26); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081ea27); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081ea34, 0x700081ea2f);
        }
        696 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081ea34); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081ea36); }
            blk = 188; continue;
        }
        697 => {
            c.w32(0, c.r32(0));
            let r822 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r822);
            c.w64(7, 0xfffffffffffc0000u64);
            let r823 = c.r64(7) & c.r64(0);
            c.w64(7, r823);
            c.logic(r823, 64);
            let Some(t824) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ea4b); };
            c.logic(t824 & 0x2u64, 8);
            if c.cc_e() { blk = 179; continue; }
            blk = 698; continue;
        }
        698 => {
            c.w64(9, c.r64(8).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081ea59); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081ea5a); }
            c.w64(7, c.r64(8));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700081ea67, 0x700081ea62);
        }
        699 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081ea67); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081ea69); }
            blk = 179; continue;
        }
        700 => {
            c.w32(1, c.r32(1));
            let r825 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r825);
            c.w64(7, 0xfffffffffffc0000u64);
            let r826 = c.r64(7) & c.r64(1);
            c.w64(7, r826);
            c.logic(r826, 64);
            let Some(t827) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ea7e); };
            c.logic(t827 & 0x2u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 701; continue;
        }
        701 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081ea8c); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081ea95, 0x700081ea90);
        }
        702 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081ea95); }
            blk = 175; continue;
        }
        703 => {
            c.w32(11, c.r32(11));
            let r828 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r828);
            c.w64(8, 0xfffffffffffc0000u64);
            let r829 = c.r64(8) & c.r64(11);
            c.w64(8, r829);
            c.logic(r829, 64);
            let Some(t830) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081eaab); };
            c.logic(t830 & 0x2u64, 8);
            if c.cc_e() { blk = 152; continue; }
            blk = 704; continue;
        }
        704 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081eaba); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081eabb); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081eac8, 0x700081eac3);
        }
        705 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081eac8); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081eaca); }
            blk = 152; continue;
        }
        706 => {
            c.w32(11, c.r32(11));
            let r831 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r831);
            c.w64(7, 0xfffffffffffc0000u64);
            let r832 = c.r64(7) & c.r64(11);
            c.w64(7, r832);
            c.logic(r832, 64);
            let Some(t833) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081eae0); };
            c.logic(t833 & 0x2u64, 8);
            if c.cc_e() { blk = 127; continue; }
            blk = 707; continue;
        }
        707 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081eaee); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081eaf7, 0x700081eaf2);
        }
        708 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081eaf7); }
            blk = 127; continue;
        }
        709 => {
            c.w32(15, c.r32(15));
            let r834 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r834);
            c.w64(7, 0xfffffffffffc0000u64);
            let r835 = c.r64(7) & c.r64(15);
            c.w64(7, r835);
            c.logic(r835, 64);
            let Some(t836) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081eb0d); };
            c.logic(t836 & 0x2u64, 8);
            if c.cc_e() { blk = 125; continue; }
            blk = 710; continue;
        }
        710 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081eb1b); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081eb24, 0x700081eb1f);
        }
        711 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081eb24); }
            blk = 125; continue;
        }
        712 => {
            c.w32(11, c.r32(11));
            let r837 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r837);
            c.w64(7, 0xfffffffffffc0000u64);
            let r838 = c.r64(7) & c.r64(11);
            c.w64(7, r838);
            c.logic(r838, 64);
            let Some(t839) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081eb3a); };
            c.logic(t839 & 0x2u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 713; continue;
        }
        713 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081eb48); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f13b4, 0x700081eb51, 0x700081eb4c);
        }
        714 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081eb51); }
            blk = 81; continue;
        }
        715 => {
            c.w32(15, c.r32(15));
            let r840 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r840);
            c.w64(7, 0xfffffffffffc0000u64);
            let r841 = c.r64(7) & c.r64(15);
            c.w64(7, r841);
            c.logic(r841, 64);
            let Some(t842) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081eb67); };
            c.logic(t842 & 0x2u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 716; continue;
        }
        716 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081eb75); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081eb7e, 0x700081eb79);
        }
        717 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081eb7e); }
            blk = 79; continue;
        }
        718 => {
            c.w32(1, c.r32(1));
            let r843 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r843);
            c.w64(7, 0xfffffffffffc0000u64);
            let r844 = c.r64(7) & c.r64(1);
            c.w64(7, r844);
            c.logic(r844, 64);
            let Some(t845) = c.ld8(c.r64(7).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081eb93); };
            c.logic(t845 & 0x2u64, 8);
            if c.cc_e() { blk = 51; continue; }
            blk = 719; continue;
        }
        719 => {
            c.w64(8, c.r64(2).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081eba1); }
            c.w64(7, c.r64(2));
            return c.call(0x70007f0e54, 0x700081ebaa, 0x700081eba5);
        }
        720 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081ebaa); }
            blk = 51; continue;
        }
        721 => {
            c.w32(11, c.r32(11));
            let r846 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r846);
            c.w64(8, 0xfffffffffffc0000u64);
            let r847 = c.r64(8) & c.r64(11);
            c.w64(8, r847);
            c.logic(r847, 64);
            let Some(t848) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ebc0); };
            c.logic(t848 & 0x2u64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 722; continue;
        }
        722 => {
            c.w64(9, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700081ebcf); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081ebd0); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f13b4, 0x700081ebdd, 0x700081ebd8);
        }
        723 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081ebdd); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700081ebdf); }
            blk = 27; continue;
        }
        724 => {
            return RegfileExit::jump(0x700081ebe8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081ebf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ebf9); };
            c.w32(7, t0);
            blk = 3; continue;
        }
        2 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700081ebfe); };
            c.w64(7, t1);
            blk = 3; continue;
        }
        3 => {
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x700081ec05); };
            c.w32(8, t2);
            let r3 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r3);
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ec0d); };
            c.w32(9, t4);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r5 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            blk = 4; continue;
        }
        4 => {
            c.w64(11, c.r64(9).wrapping_mul(4));
            c.w32(12, c.r32(7));
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ec26); };
            c.w32(15, t6);
            let r7 = c.r32(12) | 0x2u64;
            c.w32(12, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let r8 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r8);
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_ge() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            blk = 33; continue;
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(c.r64(11)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081ec43); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 34; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(2));
            blk = 9; continue;
        }
        9 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r11 = c.r64(8) & 0xfu64;
            c.w64(8, r11);
            c.logic(r11, 64);
            c.cmp(c.r64(8), 0x4u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r64(8), 0x5u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r64(8), 0xdu64, 64);
            if c.cc_e() { blk = 31; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r64(8), 0x2u64, 64);
            if c.cc_e() { blk = 24; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r64(8), 0x6u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 14; continue;
        }
        14 => {
            c.cmp(c.r64(8), 0x7u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r64(8), 0x8u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(8), 0x9u64, 64);
            if c.cc_e() { blk = 32; continue; }
            blk = 17; continue;
        }
        17 => {
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_e() { blk = 20; continue; }
            blk = 19; continue;
        }
        19 => {
            c.w64(0, c.r64(2));
            c.w64(3, c.r64(1));
            return RegfileExit::jump(0x7000820354);
        }
        20 => {
            let Some(t12) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081ecc8); };
            c.w32(7, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081eccb); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700081ecd8); };
            c.w64(0, t14);
            return c.ret(0x0, 0x700081ecdf);
        }
        22 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081ece0); }
            let Some(t15) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081ece1); };
            if !c.push64(t15) { return RegfileExit::fault(0x700081ece1); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x700081ece5); };
            c.w64(3, t16);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x700081ecf1); }
            return RegfileExit::jump(0x700086ed74);
        }
        23 => {
            blk = 32; continue;
        }
        24 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ecfd); };
            c.w32(7, t17);
            let r18 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r18);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700081ed09); };
            c.w64(7, t19);
            blk = 26; continue;
        }
        26 => {
            let Some(t20) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081ed10); };
            c.w32(8, t20);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r21 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r21);
            let Some(t22) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ed1e); };
            c.w32(9, t22);
            let r23 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r23);
            let r24 = c.r64(9) & c.r64(8);
            c.w64(9, r24);
            c.logic(r24, 64);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 28; continue;
        }
        27 => {
            let r25 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r25);
            let r26 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r26);
            c.w64(12, c.r64(8));
            let r27 = c.r64(12) & c.r64(9);
            c.w64(12, r27);
            c.logic(r27, 64);
            c.w64(9, c.r64(12));
            blk = 28; continue;
        }
        28 => {
            c.w64(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ed43); };
            c.w32(12, t28);
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081ed48); };
            c.cmp(t29, c.r32(12), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(12), c.r32(1), 32);
            if c.cc_ne() { blk = 27; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t30) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700081ed5a); };
            c.w64(0, t30);
            return c.ret(0x0, 0x700081ed61);
        }
        31 => {
            let Some(t31) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x700081ed62); };
            c.w64(0, t31);
            return c.ret(0x0, 0x700081ed69);
        }
        32 => {
            let Some(t32) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700081ed6a); };
            c.w64(0, t32);
            return c.ret(0x0, 0x700081ed71);
        }
        33 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081ed72); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700081ed76); }
            let r33 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r33);
            blk = 69; continue;
        }
        34 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081ed81); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700081ed85); }
            let r34 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r34);
            let Some(t35) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ed8b); };
            c.w32(8, t35);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700081ed8f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x700081ed93); }
            let Some(t36) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x700081ed97); };
            c.cmp(t36, c.r32(8), 32);
            if c.cc_e() { blk = 63; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t37) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081eda4); };
            c.w32(8, t37);
            let r38 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r38);
            c.w32(9, 0);
            c.logic(0, 32);
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t39) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081edb3); };
            c.cmp(t39, c.r32(9), 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t40) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081edbd); };
            c.w32(8, t40);
            c.w64(11, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.logic(c.r8(11) & 0x30u64, 8);
            if c.cc_e() { blk = 56; continue; }
            blk = 38; continue;
        }
        38 => {
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 48; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t41) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081edd7); };
            c.w32(11, t41);
            let r42 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r42);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 41; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t43) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700081ede4); };
            c.w64(11, t43);
            blk = 41; continue;
        }
        41 => {
            let Some(t44) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081edeb); };
            c.w32(12, t44);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r45 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r45);
            let Some(t46) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081edf9); };
            c.w32(15, t46);
            let r47 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r47);
            let r48 = c.r64(15) & c.r64(12);
            c.w64(15, r48);
            c.logic(r48, 64);
            c.w64(0, c.r64(9));
            blk = 42; continue;
        }
        42 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t49) = c.ld32(c.r64(11).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ee0c); };
            c.w32(3, t49);
            let Some(t50) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081ee11); };
            c.cmp(t50, c.r32(3), 32);
            if c.cc_e() { blk = 47; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            let r51 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r51);
            let r52 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r52);
            c.w64(3, c.r64(12));
            let r53 = c.r64(3) & c.r64(15);
            c.w64(3, r53);
            c.logic(r53, 64);
            c.w64(15, c.r64(3));
            blk = 42; continue;
        }
        45 => {
            let Some(t54) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700081ee30); };
            c.w64(0, t54);
            blk = 46; continue;
        }
        46 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081ee3a); }
            return c.ret(0x0, 0x700081ee3b);
        }
        47 => {
            let Some(t55) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081ee3c); };
            c.w64(0, t55);
            let Some(t56) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081ee40); };
            c.w64(3, t56);
            blk = 56; continue;
        }
        48 => {
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081ee49); };
            c.w32(11, t57);
            let r58 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r58);
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 49; continue;
        }
        49 => {
            let r59 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r59);
            c.logic(r59, 64);
            let Some(t60) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ee5e); };
            c.w32(12, t60);
            let r61 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r61);
            let Some(t62) = c.ld32(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081ee65); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            c.cmp(c.r32(11), c.r32(12), 32);
            if c.cc_e() { blk = 56; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t64) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ee72); };
            c.w32(15, t64);
            let r65 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r65);
            let Some(t66) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ee79); };
            c.cmp(t66, 0xbeu64, 16);
            if c.cc_ne() { blk = 69; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t67) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ee86); };
            c.w32(15, t67);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 53; continue; }
            blk = 52; continue;
        }
        52 => {
            return RegfileExit::fault(0x700081ee99);
        }
        53 => {
            let Some(t68) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ee9a); };
            c.w32(15, t68);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700081eea2); };
            c.w32(11, t69);
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0xa9u64, 64);
            if c.cc_a() { blk = 55; continue; }
            blk = 54; continue;
        }
        54 => {
            return RegfileExit::fault(0x700081eeb5);
        }
        55 => {
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(0x2ab)) else { return RegfileExit::fault(0x700081eeb6); };
            c.cmp(t70, c.r32(11), 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t71) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081eec4); };
            c.w32(7, t71);
            let r72 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r72);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 62; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t73) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700081eed0); };
            c.cmp(t73, c.r32(7), 32);
            if c.cc_e() { blk = 60; continue; }
            blk = 58; continue;
        }
        58 => {
            c.cmp(c.r32(7), 0x3u64, 32);
            if c.cc_e() { blk = 69; continue; }
            blk = 59; continue;
        }
        59 => {
            let r74 = c.r64(7) & 0xfffffffffffffffdu64;
            c.w64(7, r74);
            c.logic(r74, 64);
            blk = 61; continue;
        }
        60 => {
            c.w64(7, c.r64(2));
            blk = 61; continue;
        }
        61 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081eeea); }
            blk = 9; continue;
        }
        62 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700081eef0); };
            c.w64(0, t75);
            blk = 46; continue;
        }
        63 => {
            let Some(t76) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700081eefc); };
            c.w64(5, t76);
            let r77 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r77);
            let Some(t78) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700081ef04); };
            c.logic(t78 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            let r79 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r79);
            blk = 66; continue;
        }
        65 => {
            let Some(t80) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700081ef13); };
            c.w32(7, t80);
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x700081ef16); };
            c.w64(7, t81);
            blk = 66; continue;
        }
        66 => {
            return RegfileExit::jump(c.r64(7));
        }
        67 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081ef20); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700081ef24); }
            let r82 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r82);
            blk = 69; continue;
        }
        68 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081ef2c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700081ef30); }
            let r83 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r83);
            blk = 69; continue;
        }
        69 => {
            let Some(t84) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x700081ef36); };
            c.w64(5, t84);
            let r85 = c.add(c.r64(4), 0x20u64, 64);
            c.w64(4, r85);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081ef3e); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081ef3f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700081ef40); }
            let Some(t86) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700081ef41); };
            if !c.push64(t86) { return RegfileExit::fault(0x700081ef41); }
            c.w32(0, 0x4u64);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(0x20b8)) else { return RegfileExit::fault(0x700081ef4a); };
            c.w64(3, t87);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x700081ef51); }
            return RegfileExit::jump(0x700086ed74);
        }
        70 => {
            return RegfileExit::jump(0x700081ef5c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081ef74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 15; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ef7d); };
            c.w32(7, t0);
            let r1 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1);
            let Some(t2) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ef83); };
            c.w32(8, t2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xe50)) else { return RegfileExit::fault(0x700081ef87); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ef94); };
            c.w32(8, t4);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_be() { blk = 9; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_b() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            return RegfileExit::fault(0x700081efa2);
        }
        5 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081efa6); };
            c.w32(8, t5);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081efb1); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081efb7); };
            c.w32(2, t8);
            c.cmp(c.r32(8), c.r32(2), 32);
            if c.cc_b() { blk = 14; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x700081efbf);
        }
        8 => {
            blk = 14; continue;
        }
        9 => {
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081efc2); };
            c.w32(7, t9);
            let r10 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r10);
            let Some(t11) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081efc8); };
            c.w32(8, t11);
            let r12 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r12);
            let Some(t13) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081efcf); };
            c.cmp(t13, 0x7cu64, 16);
            if c.cc_e() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t14) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081efd7); };
            c.w32(8, t14);
            c.cmp(c.r32(8), c.r32(1), 32);
            if c.cc_be() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x700081efe5);
        }
        13 => {
            c.w64(8, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081efe9); };
            c.w32(7, t15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700081efee); };
            c.cmp(t16, c.r32(7), 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700081eff7); };
            c.w64(0, t17);
            return c.ret(0x0, 0x700081effe);
        }
        15 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081efff); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081f000); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700081f001); }
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700081f002); };
            if !c.push64(t18) { return RegfileExit::fault(0x700081f002); }
            c.w32(0, 0x4u64);
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x20b8)) else { return RegfileExit::fault(0x700081f00b); };
            c.w64(3, t19);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x700081f012); }
            return RegfileExit::jump(0x700086ed74);
        }
        16 => {
            return RegfileExit::jump(0x700081f020);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081f034(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r32(1) & 0x80000001u64, 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081f03c); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081f03d); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700081f03e); }
            let Some(t0) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x700081f03f); };
            if !c.push64(t0) { return RegfileExit::fault(0x700081f03f); }
            c.w32(0, 0x4u64);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x20b8)) else { return RegfileExit::fault(0x700081f048); };
            c.w64(3, t1);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x700081f04f); }
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081f059); }
            let Some(t2) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081f05a); };
            if !c.push64(t2) { return RegfileExit::fault(0x700081f05a); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x20c0)) else { return RegfileExit::fault(0x700081f05e); };
            c.w64(3, t3);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x700081f06a); }
            return RegfileExit::jump(0x700086ed74);
        }
        3 => {
            return RegfileExit::jump(0x700081f078);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081f094(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081f094); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x700081f098); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081f09a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081f09b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700081f09c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700081f09d); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081f09e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081f09f); }
            let Some(t0) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081f0a1); };
            c.w64(2, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(0x5)) else { return RegfileExit::fault(0x700081f0a5); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_add(0xd)) else { return RegfileExit::fault(0x700081f0a9); };
            c.w64(2, t2);
            return c.call(0x700081f114, 0x700081f0b2, 0x700081f0ad);
        }
        1 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081f0bc); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x700081f0be); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x700081f0bf); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x700081f0c0); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x700081f0c1); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x700081f0c2); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f0c6); }
            return c.ret(0x0, 0x700081f0c7);
        }
        3 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081f0d2); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x700081f0d4); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x700081f0d5); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x700081f0d6); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x700081f0d7); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x700081f0d8); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f0dc); }
            return RegfileExit::jump(0x70007fdb54);
        }
        5 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081f0e2); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x700081f0e4); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x700081f0e5); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x700081f0e6); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x700081f0e7); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x700081f0e8); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f0ec); }
            return RegfileExit::jump(0x70007fe194);
        }
        6 => {
            return RegfileExit::fault(0x700081f0f2);
        }
        7 => {
            return RegfileExit::fault(0x700081f0f3);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081f114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x700081f117); };
            c.w64(3, t0);
            let Some(t1) = c.ld64(c.r64(3).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081f11a); };
            c.w64(3, t1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f11e); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f124); };
            c.w32(3, t4);
            let r5 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r5);
            let Some(t6) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f12a); };
            c.w32(6, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x700081f12d); };
            c.cmp(t7, c.r32(6), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f136); };
            c.w64(3, t8);
            blk = 2; continue;
        }
        2 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700081f13d); };
            c.w32(6, t9);
            let r10 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r10);
            c.w32(8, c.r32(6));
            let r11 = c.r32(8) & 0x3u64;
            c.w32(8, r11);
            c.logic(r11, 32);
            c.cmp(c.r32(8), 0x1u64, 32);
            if c.cc_e() { blk = 12; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            let r12 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r12);
            c.logic(r12, 64);
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(0, 0x1u64);
            return c.ret(0x0, 0x700081f163);
        }
        6 => {
            let r13 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r13);
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700081f168); };
            c.w32(3, t14);
            c.w32(1, c.r32(3));
            let r15 = c.r32(1) & 0x3u64;
            c.w32(1, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            return RegfileExit::fault(0x700081f176);
        }
        8 => {
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f180);
        }
        10 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x700081f183);
        }
        11 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f189);
        }
        12 => {
            let Some(t16) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f18a); };
            c.w32(3, t16);
            let r17 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r17);
            let Some(t18) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f190); };
            c.w32(3, t18);
            let r19 = c.sub(c.r32(3), 0x9fu64, 32);
            c.w32(3, r19);
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_a() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w32(1, 0x1u64);
            blk = 15; continue;
        }
        14 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t20) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f1ad); };
            c.w32(3, t20);
            let r21 = c.r64(0) | 0x3u64;
            c.w64(0, r21);
            c.logic(r21, 64);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 18; continue;
        }
        17 => {
            let r22 = c.add(c.r64(1), 0x2u64, 64);
            c.w64(1, r22);
            blk = 18; continue;
        }
        18 => {
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_ge() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t23) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f1c7); };
            c.cmp(t23, c.r32(0), 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 20; continue;
        }
        20 => {
            blk = 17; continue;
        }
        21 => {
            c.w32(0, 0x1u64);
            return c.ret(0x0, 0x700081f1d4);
        }
        22 => {
            c.w64(3, c.r64(1).wrapping_add(0x1));
            let Some(t24) = c.ld32(c.r64(6).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f1d9); };
            c.w32(3, t24);
            c.w32(1, c.r32(3));
            let r25 = c.r32(1) & 0x3u64;
            c.w32(1, r25);
            c.logic(r25, 32);
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x700081f1e7);
        }
        24 => {
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f1f1);
        }
        26 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x700081f1f4);
        }
        27 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f1fa);
        }
        28 => {
            return RegfileExit::jump(0x700081f1fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081f214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081f214); }
            c.w64(5, c.r64(4));
            if !c.push64(0x24u64) { return RegfileExit::fault(0x700081f218); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081f21a); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081f21b); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x700081f21c); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x700081f21d); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x700081f21e); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081f21f); }
            let Some(t0) = c.ld64(c.r64(5).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081f221); };
            c.w64(8, t0);
            let Some(t1) = c.ld64(c.r64(8).wrapping_add(0x5)) else { return RegfileExit::fault(0x700081f225); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(8).wrapping_add(0xd)) else { return RegfileExit::fault(0x700081f229); };
            c.w64(8, t2);
            return c.call(0x700081f294, 0x700081f232, 0x700081f22d);
        }
        1 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081f23c); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x700081f23e); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x700081f23f); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x700081f240); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x700081f241); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x700081f242); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f246); }
            return c.ret(0x0, 0x700081f247);
        }
        3 => {
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081f252); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x700081f254); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x700081f255); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x700081f256); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x700081f257); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x700081f258); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f25c); }
            return RegfileExit::jump(0x70007fdb54);
        }
        5 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700081f262); }
            if !c.pop64_reg(6) { return RegfileExit::fault(0x700081f264); }
            if !c.pop64_reg(3) { return RegfileExit::fault(0x700081f265); }
            if !c.pop64_reg(2) { return RegfileExit::fault(0x700081f266); }
            if !c.pop64_reg(1) { return RegfileExit::fault(0x700081f267); }
            if !c.pop64_reg(0) { return RegfileExit::fault(0x700081f268); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f26c); }
            return RegfileExit::jump(0x70007fe194);
        }
        6 => {
            return RegfileExit::fault(0x700081f272);
        }
        7 => {
            return RegfileExit::fault(0x700081f273);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081f294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700081f294); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            c.w32(6, c.r32(3));
            let r2 = c.r32(6) & 0x3u64;
            c.w32(6, r2);
            c.logic(r2, 32);
            c.cmp(c.r32(6), 0x1u64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            let r3 = c.r64(3) & 0xfffffffffffffffdu64;
            c.w64(3, r3);
            c.logic(r3, 64);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(0, 0x1u64);
            return c.ret(0x0, 0x700081f2b7);
        }
        4 => {
            c.w64(3, c.r64(2).wrapping_add(0x1));
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x700081f2bc); };
            c.w32(3, t4);
            c.w32(2, c.r32(3));
            let r5 = c.r32(2) & 0x3u64;
            c.w32(2, r5);
            c.logic(r5, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            return RegfileExit::fault(0x700081f2ca);
        }
        6 => {
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f2d5);
        }
        8 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x700081f2d8);
        }
        9 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f2de);
        }
        10 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f2df); };
            c.w32(2, t6);
            let r7 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r7);
            let Some(t8) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f2e5); };
            c.w32(2, t8);
            let r9 = c.sub(c.r32(2), 0x9fu64, 32);
            c.w32(2, r9);
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w32(1, 0x1u64);
            blk = 13; continue;
        }
        12 => {
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f302); };
            c.w32(2, t10);
            let r11 = c.r64(0) | 0x3u64;
            c.w64(0, r11);
            c.logic(r11, 64);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 16; continue;
        }
        15 => {
            let r12 = c.add(c.r64(1), 0x2u64, 64);
            c.w64(1, r12);
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r64(1), c.r64(2), 64);
            if c.cc_ge() { blk = 19; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t13) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f31c); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 20; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 15; continue;
        }
        19 => {
            c.w32(0, 0x1u64);
            return c.ret(0x0, 0x700081f329);
        }
        20 => {
            c.w64(2, c.r64(1).wrapping_add(0x1));
            let Some(t14) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f32e); };
            c.w32(3, t14);
            c.w32(2, c.r32(3));
            let r15 = c.r32(2) & 0x3u64;
            c.w32(2, r15);
            c.logic(r15, 32);
            c.cmp(c.r32(2), 0x3u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x700081f33c);
        }
        22 => {
            c.cmp(c.r32(8), c.r32(3), 32);
            if c.cc_e() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f347);
        }
        24 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x700081f34a);
        }
        25 => {
            c.w32(0, 0x2u64);
            return c.ret(0x0, 0x700081f350);
        }
        26 => {
            return RegfileExit::jump(0x700081f354);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081f374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f374); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f37a); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld64(c.r64(3).wrapping_add(0x407)) else { return RegfileExit::fault(0x700081f380); };
            c.w64(2, t4);
            c.cmp(c.r64(2), 0x0u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t5) = c.ld64(c.r64(2).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f391); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(2).wrapping_add(0x10)) else { return RegfileExit::fault(0x700081f395); };
            c.w64(6, t6);
            let Some(t7) = c.ld64(c.r64(2).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081f399); };
            c.w64(7, t7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_add(0x18)) else { return RegfileExit::fault(0x700081f39d); };
            c.w64(9, t8);
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r9 = c.sub(c.r64(6), 0x1u64, 64);
            c.w64(6, r9);
            let r10 = c.add(c.r64(9), c.r64(7), 64);
            c.w64(9, r10);
            let r11 = c.r64(6) & c.r64(9);
            c.w64(6, r11);
            c.logic(r11, 64);
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x700081f3b0); }
            c.w64(3, c.r64(7).wrapping_add(0x1));
            if !c.st64(c.r64(2).wrapping_add(0x8), c.r64(3)) { return RegfileExit::fault(0x700081f3b8); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f3bc); };
            c.w64(0, t12);
            return c.ret(0x0, 0x700081f3c3);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081f3c4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700081f3c8); }
            c.w64(10, c.r64(4));
            let r13 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r13);
            let r14 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r14);
            c.logic(r14, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700081f3d5); }
            c.w64(1, c.r64(13));
            c.w64(8, c.r64(0));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x1620)) else { return RegfileExit::fault(0x700081f3e0); };
            c.w64(0, t15);
            c.w64(10, 0x700081f3eeu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700081f3ee); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700081f3f2); }
            return c.call(c.r64(0), 0x700081f3f8, 0x700081f3f6);
        }
        4 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700081f3f8); }
            let Some(t16) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f400); };
            c.w64(4, t16);
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f405); };
            c.w64(0, t17);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f40f); }
            return c.ret(0x0, 0x700081f410);
        }
        5 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f411); };
            c.w64(0, t18);
            return c.ret(0x0, 0x700081f418);
        }
        6 => {
            return RegfileExit::jump(0x700081f41c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_700081f434(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    c.w64(0, c.r64(2));
    return RegfileExit::jump(0x700081f454);
}

pub unsafe extern "C" fn rf_700081f454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700081f454); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700081f458); }
            let r0 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2fe8)) else { return RegfileExit::fault(0x700081f45e); };
            c.w64(8, t1);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700081f465); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(8)) { return RegfileExit::fault(0x700081f469); }
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f46d); };
            c.w64(9, t2);
            blk = 2; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081f476); };
            c.w64(8, t3);
            let Some(t4) = c.ld64(c.r64(8).wrapping_add(0x28)) else { return RegfileExit::fault(0x700081f47a); };
            c.w64(9, t4);
            let r5 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r5);
            if !c.st64(c.r64(8).wrapping_add(0x28), c.r64(9)) { return RegfileExit::fault(0x700081f482); }
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(8));
            blk = 2; continue;
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081f48c); };
            c.w64(12, t6);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 63; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(0).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f49a); };
            c.w64(15, t7);
            let Some(t8) = c.ld64(c.r64(0).wrapping_add(0x10)) else { return RegfileExit::fault(0x700081f49e); };
            c.w64(3, t8);
            let r9 = c.sub(c.r64(3), 0x1u64, 64);
            c.w64(3, r9);
            let Some(t10) = c.ld64(c.r64(0).wrapping_add(0x18)) else { return RegfileExit::fault(0x700081f4a6); };
            c.w64(2, t10);
            c.w64(1, c.r64(2));
            let r11 = c.r64(1) & c.r64(3);
            c.w64(1, r11);
            c.logic(r11, 64);
            let Some(t12) = c.ld64(c.r64(15).wrapping_add(c.r64(1).wrapping_mul(8))) else { return RegfileExit::fault(0x700081f4b0); };
            c.w64(15, t12);
            let r13 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r13);
            let r14 = c.add(c.r64(2), 0x1u64, 64);
            c.w64(2, r14);
            if !c.st64(c.r64(0).wrapping_add(0x8), c.r64(12)) { return RegfileExit::fault(0x700081f4bc); }
            let r15 = c.r64(3) & c.r64(2);
            c.w64(3, r15);
            c.logic(r15, 64);
            if !c.st64(c.r64(0).wrapping_add(0x18), c.r64(3)) { return RegfileExit::fault(0x700081f4c3); }
            if !c.st64(c.r64(13).wrapping_add(0x1150), c.r64(15)) { return RegfileExit::fault(0x700081f4c7); }
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081f4ce); };
            c.w64(12, t16);
            let Some(t17) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081f4d5); };
            c.w64(3, t17);
            let Some(t18) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f4d9); };
            c.w32(2, t18);
            let r19 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r19);
            let Some(t20) = c.ld64(c.r64(3).wrapping_add(0x30)) else { return RegfileExit::fault(0x700081f4e0); };
            c.w64(3, t20);
            let Some(t21) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f4e4); };
            c.w32(2, t21);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x700081f4e8); }
            c.cmp(c.r32(2), 0x4eu64, 32);
            if c.cc_l() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(2), 0x4eu64, 32);
            if c.cc_e() { blk = 11; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(2), 0x4fu64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(2), 0x50u64, 32);
            if c.cc_e() { blk = 51; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 188; continue;
        }
        8 => {
            c.cmp(c.r32(2), 0x4cu64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(2), 0x4du64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 10; continue;
        }
        10 => {
            blk = 188; continue;
        }
        11 => {
            let Some(t22) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f524); };
            c.w32(6, t22);
            let r23 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r23);
            let Some(t24) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f52b); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r25);
            let Some(t26) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f531); };
            c.w32(2, t26);
            let r27 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r27);
            let Some(t28) = c.ld64(c.r64(2).wrapping_add(0x407)) else { return RegfileExit::fault(0x700081f537); };
            c.cmp(t28, 0x0u64, 64);
            if c.cc_e() { blk = 16; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t29) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081f541); };
            c.w64(1, t29);
            let Some(t30) = c.ld64(c.r64(1).wrapping_add(0x30)) else { return RegfileExit::fault(0x700081f545); };
            c.w64(7, t30);
            let Some(t31) = c.ld64(c.r64(1).wrapping_add(0x28)) else { return RegfileExit::fault(0x700081f549); };
            c.cmp(t31, c.r64(7), 64);
            if c.cc_e() { blk = 64; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t32) = c.ld64(c.r64(1).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f553); };
            c.w64(8, t32);
            if !c.st64(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x700081f557); }
            c.w64(8, c.r64(7).wrapping_add(0x1));
            if !c.st64(c.r64(1).wrapping_add(0x30), c.r64(8)) { return RegfileExit::fault(0x700081f55f); }
            let Some(t33) = c.ld64(c.r64(1).wrapping_add(0x38)) else { return RegfileExit::fault(0x700081f563); };
            c.w64(11, t33);
            if !c.st8(c.r64(11).wrapping_add(c.r64(7)), 0x1u64) { return RegfileExit::fault(0x700081f567); }
            if !c.st64(c.r64(1).wrapping_add(0x48), c.r64(8)) { return RegfileExit::fault(0x700081f56c); }
            blk = 14; continue;
        }
        14 => {
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(2)) { return RegfileExit::fault(0x700081f570); }
            let Some(t34) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f577); };
            c.w32(7, t34);
            let r35 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r35);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700081f57e); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x70007f2134, 0x700081f587, 0x700081f582);
        }
        15 => {
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081f587); };
            c.w64(8, t36);
            let Some(t37) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700081f58e); };
            c.w64(8, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081f591); };
            c.w64(9, t38);
            if !c.st64(c.r64(8).wrapping_add(0x30), c.r64(9)) { return RegfileExit::fault(0x700081f595); }
            if !c.st64(c.r64(8).wrapping_add(0x48), c.r64(9)) { return RegfileExit::fault(0x700081f599); }
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081f59d); };
            c.w64(6, t39);
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700081f5a1); }
            let Some(t40) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f5a8); };
            c.w64(0, t40);
            blk = 1; continue;
        }
        16 => {
            c.w64(0, c.r64(9));
            blk = 1; continue;
        }
        17 => {
            let Some(t41) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f5bc); };
            c.w32(8, t41);
            let r42 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r42);
            let Some(t43) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f5c3); };
            c.w32(11, t43);
            let r44 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r44);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081f5ca); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x700081f5cc); }
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081f5ce); };
            c.w64(6, t45);
            c.w32(0, 0x2u64);
            let Some(t46) = c.ld64(c.r64(13).wrapping_add(0x2198)) else { return RegfileExit::fault(0x700081f5d7); };
            c.w64(3, t46);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700081f5e9, 0x700081f5e4);
        }
        18 => {
            let Some(t47) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f5e9); };
            c.w64(0, t47);
            blk = 1; continue;
        }
        19 => {
            let Some(t48) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f5f5); };
            c.w32(6, t48);
            let r49 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r49);
            let Some(t50) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f5fc); };
            c.w32(8, t50);
            let r51 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r51);
            let Some(t52) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f603); };
            c.w32(2, t52);
            let r53 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r53);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700081f60a); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700081f60e); }
            let Some(t54) = c.ld64(c.r64(2).wrapping_add(0x407)) else { return RegfileExit::fault(0x700081f612); };
            c.cmp(t54, 0x0u64, 64);
            if c.cc_e() { blk = 34; continue; }
            blk = 20; continue;
        }
        20 => {
            c.w64(8, c.r64(12));
            let Some(t55) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700081f623); };
            c.w64(1, t55);
            let Some(t56) = c.ld64(c.r64(1).wrapping_add(0x30)) else { return RegfileExit::fault(0x700081f626); };
            c.w64(11, t56);
            let Some(t57) = c.ld64(c.r64(1).wrapping_add(0x28)) else { return RegfileExit::fault(0x700081f62a); };
            c.cmp(t57, c.r64(11), 64);
            if c.cc_e() { blk = 66; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t58) = c.ld64(c.r64(1).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f634); };
            c.w64(12, t58);
            if !c.st64(c.r64(12).wrapping_add(c.r64(11).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x700081f638); }
            c.w64(12, c.r64(11).wrapping_add(0x1));
            if !c.st64(c.r64(1).wrapping_add(0x30), c.r64(12)) { return RegfileExit::fault(0x700081f640); }
            let Some(t59) = c.ld64(c.r64(1).wrapping_add(0x38)) else { return RegfileExit::fault(0x700081f644); };
            c.w64(7, t59);
            if !c.st8(c.r64(7).wrapping_add(c.r64(11)), 0x1u64) { return RegfileExit::fault(0x700081f648); }
            if !c.st64(c.r64(1).wrapping_add(0x48), c.r64(12)) { return RegfileExit::fault(0x700081f64d); }
            blk = 22; continue;
        }
        22 => {
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(2)) { return RegfileExit::fault(0x700081f651); }
            let Some(t60) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f658); };
            c.w32(11, t60);
            let r61 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r61);
            let Some(t62) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081f65f); };
            c.w32(12, t62);
            let r63 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r63);
            let Some(t64) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081f666); };
            c.w32(1, t64);
            let r65 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r65);
            let Some(t66) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f66d); };
            c.w32(15, t66);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f671); };
            c.cmp(t67, c.r32(15), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(11)) { return RegfileExit::fault(0x700081f680); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700081f684); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700081f688); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t68) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f691); };
            c.w32(8, t68);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x5u64, 64);
            if c.cc_be() { blk = 59; continue; }
            blk = 24; continue;
        }
        24 => {
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(15)) { return RegfileExit::fault(0x700081f6a5); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r69 = c.r64(8) & c.r64(2);
            c.w64(8, r69);
            c.logic(r69, 64);
            let Some(t70) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081f6b9); };
            c.logic(t70 & 0x4u64, 8);
            if c.cc_ne() { blk = 198; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t71) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700081f6c4); };
            c.w64(8, t71);
            let Some(t72) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700081f6cb); };
            c.w32(8, t72);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t73) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081f6d8); };
            c.w64(0, t73);
            c.w64(3, c.r64(12));
            return c.call(0x70008ba0b4, 0x700081f6e4, 0x700081f6df);
        }
        28 => {
            let Some(t74) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700081f6e4); };
            c.w64(8, t74);
            let Some(t75) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700081f6eb); };
            c.w32(8, t75);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t76) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081f6f8); };
            c.cmp(t76, 0x0u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081f6fe); };
            c.w64(8, t77);
            let Some(t78) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f702); };
            c.w32(9, t78);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x5u64, 64);
            if c.cc_be() { blk = 60; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f716); };
            c.w64(0, t79);
            if !c.st32(c.r64(8).wrapping_add(0x1b), c.r32(0)) { return RegfileExit::fault(0x700081f71d); }
            blk = 33; continue;
        }
        32 => {
            let Some(t80) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f723); };
            c.w64(0, t80);
            blk = 33; continue;
        }
        33 => {
            let Some(t81) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081f72a); };
            c.w64(8, t81);
            let Some(t82) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700081f731); };
            c.w64(8, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081f734); };
            c.w64(9, t83);
            if !c.st64(c.r64(8).wrapping_add(0x30), c.r64(9)) { return RegfileExit::fault(0x700081f738); }
            if !c.st64(c.r64(8).wrapping_add(0x48), c.r64(9)) { return RegfileExit::fault(0x700081f73c); }
            let Some(t84) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081f740); };
            c.w64(6, t84);
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700081f744); }
            blk = 1; continue;
        }
        34 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(9));
            blk = 1; continue;
        }
        35 => {
            let Some(t85) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f75b); };
            c.w32(6, t85);
            let r86 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r86);
            let Some(t87) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f762); };
            c.w32(8, t87);
            let r88 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r88);
            let Some(t89) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f769); };
            c.w32(2, t89);
            let r90 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r90);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(6)) { return RegfileExit::fault(0x700081f770); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x700081f774); }
            let Some(t91) = c.ld64(c.r64(2).wrapping_add(0x407)) else { return RegfileExit::fault(0x700081f778); };
            c.cmp(t91, 0x0u64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t92) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081f786); };
            c.w64(1, t92);
            let Some(t93) = c.ld64(c.r64(1).wrapping_add(0x30)) else { return RegfileExit::fault(0x700081f78a); };
            c.w64(8, t93);
            let Some(t94) = c.ld64(c.r64(1).wrapping_add(0x28)) else { return RegfileExit::fault(0x700081f78e); };
            c.cmp(t94, c.r64(8), 64);
            if c.cc_e() { blk = 106; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t95) = c.ld64(c.r64(1).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f798); };
            c.w64(11, t95);
            if !c.st64(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x700081f79c); }
            c.w64(11, c.r64(8).wrapping_add(0x1));
            if !c.st64(c.r64(1).wrapping_add(0x30), c.r64(11)) { return RegfileExit::fault(0x700081f7a4); }
            let Some(t96) = c.ld64(c.r64(1).wrapping_add(0x38)) else { return RegfileExit::fault(0x700081f7a8); };
            c.w64(7, t96);
            if !c.st8(c.r64(7).wrapping_add(c.r64(8)), 0x1u64) { return RegfileExit::fault(0x700081f7ac); }
            if !c.st64(c.r64(1).wrapping_add(0x48), c.r64(11)) { return RegfileExit::fault(0x700081f7b1); }
            blk = 38; continue;
        }
        38 => {
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(2)) { return RegfileExit::fault(0x700081f7b5); }
            let Some(t97) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f7bc); };
            c.w32(8, t97);
            let r98 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r98);
            let Some(t99) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081f7c3); };
            c.w32(11, t99);
            let r100 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r100);
            let Some(t101) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081f7ca); };
            c.w32(1, t101);
            let r102 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r102);
            let Some(t103) = c.ld32(c.r64(15).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f7d1); };
            c.w32(15, t103);
            let Some(t104) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f7d5); };
            c.cmp(t104, c.r32(15), 32);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x700081f7e4); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x700081f7e8); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x700081f7ec); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x700081f7f0); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t105) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f7f9); };
            c.w32(8, t105);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x5u64, 64);
            if c.cc_be() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            if !c.st32(c.r64(2).wrapping_add(0x1b), c.r32(15)) { return RegfileExit::fault(0x700081f80d); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 41; continue;
        }
        41 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r106 = c.r64(8) & c.r64(2);
            c.w64(8, r106);
            c.logic(r106, 64);
            let Some(t107) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700081f821); };
            c.logic(t107 & 0x4u64, 8);
            if c.cc_ne() { blk = 195; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t108) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700081f82c); };
            c.w64(8, t108);
            let Some(t109) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700081f833); };
            c.w32(8, t109);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700081f840); };
            c.w64(0, t110);
            let Some(t111) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081f844); };
            c.w64(3, t111);
            return c.call(0x70008ba214, 0x700081f84d, 0x700081f848);
        }
        44 => {
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700081f84d); };
            c.w64(8, t112);
            let Some(t113) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700081f854); };
            c.w32(8, t113);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 128; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t114) = c.ld32(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081f861); };
            c.cmp(t114, 0x0u64, 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081f867); };
            c.w64(8, t115);
            let Some(t116) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f86b); };
            c.w32(9, t116);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x5u64, 64);
            if c.cc_be() { blk = 62; continue; }
            blk = 47; continue;
        }
        47 => {
            let Some(t117) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f87f); };
            c.w64(0, t117);
            if !c.st32(c.r64(8).wrapping_add(0x1b), c.r32(0)) { return RegfileExit::fault(0x700081f886); }
            blk = 49; continue;
        }
        48 => {
            let Some(t118) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f88c); };
            c.w64(0, t118);
            blk = 49; continue;
        }
        49 => {
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081f893); };
            c.w64(8, t119);
            let Some(t120) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700081f89a); };
            c.w64(8, t120);
            let Some(t121) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081f89d); };
            c.w64(9, t121);
            if !c.st64(c.r64(8).wrapping_add(0x30), c.r64(9)) { return RegfileExit::fault(0x700081f8a1); }
            if !c.st64(c.r64(8).wrapping_add(0x48), c.r64(9)) { return RegfileExit::fault(0x700081f8a5); }
            let Some(t122) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081f8a9); };
            c.w64(6, t122);
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(6)) { return RegfileExit::fault(0x700081f8ad); }
            blk = 1; continue;
        }
        50 => {
            c.w64(11, c.r64(9));
            c.w64(0, c.r64(9));
            blk = 1; continue;
        }
        51 => {
            let Some(t123) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081f8c4); };
            c.w32(6, t123);
            let r124 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r124);
            let Some(t125) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081f8cb); };
            c.w32(8, t125);
            let r126 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r126);
            let Some(t127) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081f8d2); };
            c.w32(2, t127);
            let r128 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r128);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700081f8d9); }
            let Some(t129) = c.ld64(c.r64(2).wrapping_add(0x407)) else { return RegfileExit::fault(0x700081f8dd); };
            c.cmp(t129, 0x0u64, 64);
            if c.cc_e() { blk = 58; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t130) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700081f8eb); };
            c.w64(1, t130);
            let Some(t131) = c.ld64(c.r64(1).wrapping_add(0x30)) else { return RegfileExit::fault(0x700081f8ef); };
            c.w64(8, t131);
            let Some(t132) = c.ld64(c.r64(1).wrapping_add(0x28)) else { return RegfileExit::fault(0x700081f8f3); };
            c.cmp(t132, c.r64(8), 64);
            if c.cc_e() { blk = 146; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t133) = c.ld64(c.r64(1).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f8fd); };
            c.w64(11, t133);
            if !c.st64(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x700081f901); }
            c.w64(11, c.r64(8).wrapping_add(0x1));
            if !c.st64(c.r64(1).wrapping_add(0x30), c.r64(11)) { return RegfileExit::fault(0x700081f909); }
            let Some(t134) = c.ld64(c.r64(1).wrapping_add(0x38)) else { return RegfileExit::fault(0x700081f90d); };
            c.w64(7, t134);
            if !c.st8(c.r64(7).wrapping_add(c.r64(8)), 0x1u64) { return RegfileExit::fault(0x700081f911); }
            if !c.st64(c.r64(1).wrapping_add(0x48), c.r64(11)) { return RegfileExit::fault(0x700081f916); }
            blk = 54; continue;
        }
        54 => {
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(2)) { return RegfileExit::fault(0x700081f91a); }
            let Some(t135) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081f921); };
            c.w32(8, t135);
            let r136 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r136);
            let Some(t137) = c.ld32(c.r64(15).wrapping_add(0xf)) else { return RegfileExit::fault(0x700081f928); };
            c.w32(1, t137);
            let r138 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r138);
            let Some(t139) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x700081f92f); };
            c.w32(11, t139);
            let r140 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r140);
            let Some(t141) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700081f936); };
            c.w64(15, t141);
            let Some(t142) = c.ld32(c.r64(15)) else { return RegfileExit::fault(0x700081f93d); };
            c.w32(15, t142);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x700081f940); }
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 148; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(0, c.r64(8));
            c.w64(3, c.r64(11));
            c.w64(6, c.r64(2));
            return c.call(0x70008bb394, 0x700081f95c, 0x700081f957);
        }
        56 => {
            let Some(t143) = c.ld64(c.r64(13).wrapping_add(0x2e00)) else { return RegfileExit::fault(0x700081f95c); };
            c.w64(8, t143);
            let Some(t144) = c.ld32(c.r64(8)) else { return RegfileExit::fault(0x700081f963); };
            c.w32(8, t144);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t145) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081f970); };
            c.w64(8, t145);
            let Some(t146) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700081f977); };
            c.w64(8, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081f97a); };
            c.w64(9, t147);
            if !c.st64(c.r64(8).wrapping_add(0x30), c.r64(9)) { return RegfileExit::fault(0x700081f97e); }
            if !c.st64(c.r64(8).wrapping_add(0x48), c.r64(9)) { return RegfileExit::fault(0x700081f982); }
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700081f986); };
            c.w64(8, t148);
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(8)) { return RegfileExit::fault(0x700081f98a); }
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f991); };
            c.w64(0, t149);
            blk = 1; continue;
        }
        58 => {
            c.w64(0, c.r64(9));
            blk = 1; continue;
        }
        59 => {
            return RegfileExit::fault(0x700081f9a5);
        }
        60 => {
            return RegfileExit::fault(0x700081f9a6);
        }
        61 => {
            return RegfileExit::fault(0x700081f9a7);
        }
        62 => {
            return RegfileExit::fault(0x700081f9a8);
        }
        63 => {
            c.w64(0, c.r64(9));
            if !c.st64(c.r64(13).wrapping_add(0x1150), c.r64(0)) { return RegfileExit::fault(0x700081f9ac); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700081f9b6); }
            return c.ret(0x0, 0x700081f9b7);
        }
        64 => {
            c.w64(10, c.r64(4));
            let r150 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r150);
            let r151 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r151);
            c.logic(r151, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700081f9c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700081f9c8); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x700081f9cc); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x700081f9d0); }
            let Some(t152) = c.ld64(c.r64(13).wrapping_add(0x1628)) else { return RegfileExit::fault(0x700081f9d4); };
            c.w64(0, t152);
            c.w64(10, 0x700081f9e2u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700081f9e2); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700081f9e6); }
            return c.call(c.r64(0), 0x700081f9ec, 0x700081f9ea);
        }
        65 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700081f9ec); }
            let Some(t153) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081f9f4); };
            c.w64(4, t153);
            let Some(t154) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081f9f9); };
            c.w64(9, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fa00); };
            c.w64(15, t155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081fa04); };
            c.w64(6, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081fa08); };
            c.w64(2, t157);
            blk = 14; continue;
        }
        66 => {
            c.w64(10, c.r64(4));
            let r158 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r158);
            let r159 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r159);
            c.logic(r159, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700081fa1c); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700081fa21); }
            let Some(t160) = c.ld64(c.r64(13).wrapping_add(0x1628)) else { return RegfileExit::fault(0x700081fa25); };
            c.w64(0, t160);
            c.w64(10, 0x700081fa33u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700081fa33); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700081fa37); }
            return c.call(c.r64(0), 0x700081fa3d, 0x700081fa3b);
        }
        67 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700081fa3d); }
            let Some(t161) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081fa45); };
            c.w64(4, t161);
            let Some(t162) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fa4a); };
            c.w64(9, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081fa51); };
            c.w64(0, t163);
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081fa55); };
            c.w64(3, t164);
            let Some(t165) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081fa59); };
            c.w64(8, t165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fa60); };
            c.w64(15, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081fa64); };
            c.w64(2, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fa68); };
            c.w64(6, t168);
            blk = 22; continue;
        }
        68 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(12)) { return RegfileExit::fault(0x700081fa71); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700081fa75); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t169) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fa83); };
            c.w32(11, t169);
            let r170 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r170);
            let Some(t171) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081fa8a); };
            c.w32(11, t171);
            let r172 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r172);
            let Some(t173) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fa91); };
            c.w32(15, t173);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x8fu64, 64);
            if c.cc_be() { blk = 189; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t174) = c.ld32(c.r64(11).wrapping_add(0x243)) else { return RegfileExit::fault(0x700081faa8); };
            c.w32(11, t174);
            let r175 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r175);
            let Some(t176) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fab2); };
            c.w32(15, t176);
            let r177 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r177);
            let Some(t178) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x700081fab9); };
            c.logic(t178 & 0x2u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t179) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fac4); };
            c.w32(15, t179);
            let r180 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r180);
            let Some(t181) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081facb); };
            c.w32(15, t181);
            c.cmp(c.r16(15), 0x83du64, 16);
            if c.cc_ne() { blk = 73; continue; }
            blk = 72; continue;
        }
        72 => {
            c.w64(8, c.r64(1));
            blk = 76; continue;
        }
        73 => {
            c.w32(15, c.r16(15));
            c.cmp(c.r32(15), 0x6bu64, 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t182) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081faeb); };
            c.w32(15, t182);
            let r183 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r183);
            let Some(t184) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081faf2); };
            c.w32(8, t184);
            let r185 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r185);
            let Some(t186) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081faf9); };
            c.cmp(t186, 0x83du64, 16);
            if c.cc_ne() { blk = 80; continue; }
            blk = 75; continue;
        }
        75 => {
            c.w64(8, c.r64(15));
            blk = 76; continue;
        }
        76 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081fb05); }
            c.w64(8, c.r64(9));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081fb0a); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(11));
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700081fb1c, 0x700081fb17);
        }
        77 => {
            blk = 79; continue;
        }
        78 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081fb1e); }
            let Some(t187) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700081fb1f); };
            c.w64(3, t187);
            c.w32(0, 0x1u64);
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fb2b); };
            c.w64(6, t188);
            return c.call(0x700086ed74, 0x700081fb34, 0x700081fb2f);
        }
        79 => {
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fb34); };
            c.w64(9, t189);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081fb3b); };
            c.w64(0, t190);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081fb3f); };
            c.w64(3, t191);
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081fb43); };
            c.w64(2, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081fb47); };
            c.w64(7, t193);
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fb4b); };
            c.w64(6, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081fb4f); };
            c.w64(1, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081fb53); };
            c.w64(12, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fb57); };
            c.w64(8, t197);
            blk = 81; continue;
        }
        80 => {
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fb5d); };
            c.w64(8, t198);
            blk = 81; continue;
        }
        81 => {
            c.logic(c.r8(8) & 0xeu64, 8);
            if c.cc_e() { blk = 27; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t199) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fb6b); };
            c.w32(8, t199);
            let r200 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r200);
            let Some(t201) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081fb72); };
            c.cmp(t201, 0x6bu64, 16);
            if c.cc_ne() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t202) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fb7a); };
            c.w32(8, t202);
            let r203 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r203);
            blk = 85; continue;
        }
        84 => {
            c.w64(8, c.r64(1));
            blk = 85; continue;
        }
        85 => {
            let Some(t204) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fb86); };
            c.cmp(t204, c.r32(8), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 86; continue;
        }
        86 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081fb93); }
            c.w32(0, 0x1u64);
            let Some(t205) = c.ld64(c.r64(13).wrapping_add(0x2590)) else { return RegfileExit::fault(0x700081fb9a); };
            c.w64(3, t205);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700081fbac, 0x700081fba7);
        }
        87 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fbac); };
            c.w64(9, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081fbb3); };
            c.w64(0, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081fbb7); };
            c.w64(3, t208);
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081fbbb); };
            c.w64(2, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081fbbf); };
            c.w64(7, t210);
            let Some(t211) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fbc3); };
            c.w64(6, t211);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081fbc7); };
            c.w64(1, t212);
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x700081fbcb); };
            c.w64(12, t213);
            blk = 27; continue;
        }
        88 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700081fbd4); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 99; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fbe2); };
            c.w64(6, t214);
            let Some(t215) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fbe6); };
            c.w32(9, t215);
            let r216 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r216);
            let Some(t217) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081fbed); };
            c.w32(9, t217);
            let r218 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r218);
            let Some(t219) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fbf4); };
            c.w32(11, t219);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x90u64, 64);
            if c.cc_be() { blk = 190; continue; }
            blk = 90; continue;
        }
        90 => {
            let Some(t220) = c.ld32(c.r64(9).wrapping_add(0x247)) else { return RegfileExit::fault(0x700081fc0b); };
            c.w32(7, t220);
            let r221 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r221);
            let Some(t222) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fc15); };
            c.w32(9, t222);
            let r223 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r223);
            let Some(t224) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700081fc1c); };
            c.logic(t224 & 0x2u64, 8);
            if c.cc_e() { blk = 100; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t225) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081fc27); };
            c.w64(1, t225);
            let Some(t226) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fc2b); };
            c.w32(9, t226);
            let r227 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r227);
            let Some(t228) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081fc32); };
            c.w32(9, t228);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(9, c.r64(1));
            blk = 95; continue;
        }
        93 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x6bu64, 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t229) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fc4e); };
            c.w32(9, t229);
            let r230 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r230);
            let Some(t231) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fc55); };
            c.w32(11, t231);
            let r232 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r232);
            let Some(t233) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081fc5c); };
            c.cmp(t233, 0x83du64, 16);
            if c.cc_ne() { blk = 100; continue; }
            blk = 95; continue;
        }
        95 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700081fc65); }
            let Some(t234) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fc67); };
            c.w64(0, t234);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081fc6e); }
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700081fc7c, 0x700081fc77);
        }
        96 => {
            blk = 98; continue;
        }
        97 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081fc7e); }
            let Some(t235) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700081fc7f); };
            c.w64(3, t235);
            c.w32(0, 0x1u64);
            let Some(t236) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fc8b); };
            c.w64(6, t236);
            return c.call(0x700086ed74, 0x700081fc94, 0x700081fc8f);
        }
        98 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fc94); };
            c.w64(6, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fc98); };
            c.w64(8, t238);
            blk = 100; continue;
        }
        99 => {
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fc9e); };
            c.w64(6, t239);
            blk = 100; continue;
        }
        100 => {
            c.logic(c.r8(8) & 0xeu64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081fcac); };
            c.w64(1, t240);
            let Some(t241) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fcb0); };
            c.w32(8, t241);
            let r242 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r242);
            let Some(t243) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081fcb7); };
            c.cmp(t243, 0x6bu64, 16);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t244) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fcbf); };
            c.w32(1, t244);
            let r245 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r245);
            blk = 103; continue;
        }
        103 => {
            let Some(t246) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fcc5); };
            c.cmp(t246, c.r32(1), 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081fcd2); }
            let Some(t247) = c.ld64(c.r64(13).wrapping_add(0x2588)) else { return RegfileExit::fault(0x700081fcd3); };
            c.w64(3, t247);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700081fce4, 0x700081fcdf);
        }
        105 => {
            blk = 29; continue;
        }
        106 => {
            c.w64(10, c.r64(4));
            let r248 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r248);
            let r249 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r249);
            c.logic(r249, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700081fcf4); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700081fcf9); }
            let Some(t250) = c.ld64(c.r64(13).wrapping_add(0x1628)) else { return RegfileExit::fault(0x700081fcfd); };
            c.w64(0, t250);
            c.w64(10, 0x700081fd0bu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700081fd0b); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700081fd0f); }
            return c.call(c.r64(0), 0x700081fd15, 0x700081fd13);
        }
        107 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700081fd15); }
            let Some(t251) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081fd1d); };
            c.w64(4, t251);
            let Some(t252) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fd22); };
            c.w64(9, t252);
            let Some(t253) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081fd29); };
            c.w64(0, t253);
            let Some(t254) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081fd2d); };
            c.w64(3, t254);
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081fd31); };
            c.w64(12, t255);
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fd38); };
            c.w64(15, t256);
            let Some(t257) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081fd3c); };
            c.w64(2, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fd40); };
            c.w64(6, t258);
            blk = 38; continue;
        }
        108 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700081fd49); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t259) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fd57); };
            c.w32(11, t259);
            let r260 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r260);
            let Some(t261) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081fd5e); };
            c.w32(11, t261);
            let r262 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r262);
            let Some(t263) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fd65); };
            c.w32(15, t263);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.cmp(c.r64(15), 0x8fu64, 64);
            if c.cc_be() { blk = 191; continue; }
            blk = 110; continue;
        }
        110 => {
            let Some(t264) = c.ld32(c.r64(11).wrapping_add(0x243)) else { return RegfileExit::fault(0x700081fd7c); };
            c.w32(11, t264);
            let r265 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r265);
            let Some(t266) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fd86); };
            c.w32(15, t266);
            let r267 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r267);
            let Some(t268) = c.ld8(c.r64(15).wrapping_add(0x9)) else { return RegfileExit::fault(0x700081fd8d); };
            c.logic(t268 & 0x2u64, 8);
            if c.cc_e() { blk = 121; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t269) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fd98); };
            c.w32(15, t269);
            let r270 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r270);
            let Some(t271) = c.ld16(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081fd9f); };
            c.w32(15, t271);
            c.cmp(c.r16(15), 0x83du64, 16);
            if c.cc_ne() { blk = 113; continue; }
            blk = 112; continue;
        }
        112 => {
            c.w64(8, c.r64(1));
            blk = 116; continue;
        }
        113 => {
            c.w32(15, c.r16(15));
            c.cmp(c.r32(15), 0x6bu64, 32);
            if c.cc_ne() { blk = 121; continue; }
            blk = 114; continue;
        }
        114 => {
            let Some(t272) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fdbf); };
            c.w32(15, t272);
            let r273 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r273);
            let Some(t274) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fdc6); };
            c.w32(8, t274);
            let r275 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r275);
            let Some(t276) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081fdcd); };
            c.cmp(t276, 0x83du64, 16);
            if c.cc_ne() { blk = 120; continue; }
            blk = 115; continue;
        }
        115 => {
            c.w64(8, c.r64(15));
            blk = 116; continue;
        }
        116 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081fdd9); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700081fddb); }
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x700081fded, 0x700081fde8);
        }
        117 => {
            blk = 119; continue;
        }
        118 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081fdef); }
            let Some(t277) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700081fdf0); };
            c.w64(3, t277);
            c.w32(0, 0x1u64);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fdfc); };
            c.w64(6, t278);
            return c.call(0x700086ed74, 0x700081fe05, 0x700081fe00);
        }
        119 => {
            let Some(t279) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fe05); };
            c.w64(9, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081fe0c); };
            c.w64(0, t280);
            let Some(t281) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081fe10); };
            c.w64(3, t281);
            let Some(t282) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081fe14); };
            c.w64(12, t282);
            let Some(t283) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081fe1b); };
            c.w64(2, t283);
            let Some(t284) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081fe1f); };
            c.w64(7, t284);
            let Some(t285) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fe23); };
            c.w64(6, t285);
            let Some(t286) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081fe27); };
            c.w64(1, t286);
            let Some(t287) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fe2b); };
            c.w64(8, t287);
            blk = 121; continue;
        }
        120 => {
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081fe31); };
            c.w64(8, t288);
            blk = 121; continue;
        }
        121 => {
            c.logic(c.r8(8) & 0xeu64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 122; continue;
        }
        122 => {
            let Some(t289) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081fe3f); };
            c.w32(8, t289);
            let r290 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r290);
            let Some(t291) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081fe46); };
            c.cmp(t291, 0x6bu64, 16);
            if c.cc_ne() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let Some(t292) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fe4e); };
            c.w32(8, t292);
            let r293 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r293);
            blk = 125; continue;
        }
        124 => {
            c.w64(8, c.r64(1));
            blk = 125; continue;
        }
        125 => {
            let Some(t294) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fe5a); };
            c.cmp(t294, c.r32(8), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 126; continue;
        }
        126 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700081fe67); }
            c.w32(0, 0x1u64);
            let Some(t295) = c.ld64(c.r64(13).wrapping_add(0x2590)) else { return RegfileExit::fault(0x700081fe6e); };
            c.w64(3, t295);
            c.w64(8, c.r64(3));
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x700081fe80, 0x700081fe7b);
        }
        127 => {
            let Some(t296) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fe80); };
            c.w64(9, t296);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700081fe87); };
            c.w64(0, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700081fe8b); };
            c.w64(3, t298);
            let Some(t299) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700081fe8f); };
            c.w64(12, t299);
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700081fe96); };
            c.w64(2, t300);
            let Some(t301) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700081fe9a); };
            c.w64(7, t301);
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081fe9e); };
            c.w64(6, t302);
            let Some(t303) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081fea2); };
            c.w64(1, t303);
            blk = 43; continue;
        }
        128 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x700081feab); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 139; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t304) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081feb9); };
            c.w64(6, t304);
            let Some(t305) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081febd); };
            c.w32(9, t305);
            let r306 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r306);
            let Some(t307) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700081fec4); };
            c.w32(9, t307);
            let r308 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r308);
            let Some(t309) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081fecb); };
            c.w32(11, t309);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x90u64, 64);
            if c.cc_be() { blk = 192; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t310) = c.ld32(c.r64(9).wrapping_add(0x247)) else { return RegfileExit::fault(0x700081fee2); };
            c.w32(7, t310);
            let r311 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r311);
            let Some(t312) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081feec); };
            c.w32(9, t312);
            let r313 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r313);
            let Some(t314) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700081fef3); };
            c.logic(t314 & 0x2u64, 8);
            if c.cc_e() { blk = 140; continue; }
            blk = 131; continue;
        }
        131 => {
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081fefe); };
            c.w64(1, t315);
            let Some(t316) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ff02); };
            c.w32(9, t316);
            let r317 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r317);
            let Some(t318) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ff09); };
            c.w32(9, t318);
            c.cmp(c.r16(9), 0x83du64, 16);
            if c.cc_ne() { blk = 133; continue; }
            blk = 132; continue;
        }
        132 => {
            c.w64(9, c.r64(1));
            blk = 135; continue;
        }
        133 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x6bu64, 32);
            if c.cc_ne() { blk = 140; continue; }
            blk = 134; continue;
        }
        134 => {
            let Some(t319) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ff25); };
            c.w32(9, t319);
            let r320 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r320);
            let Some(t321) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ff2c); };
            c.w32(11, t321);
            let r322 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r322);
            let Some(t323) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ff33); };
            c.cmp(t323, 0x83du64, 16);
            if c.cc_ne() { blk = 140; continue; }
            blk = 135; continue;
        }
        135 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x700081ff3c); }
            let Some(t324) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081ff3e); };
            c.w64(0, t324);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081ff45); }
            c.w32(0, 0x1u64);
            c.w64(9, c.r64(0));
            return c.call(0x70007f2134, 0x700081ff53, 0x700081ff4e);
        }
        136 => {
            blk = 138; continue;
        }
        137 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700081ff55); }
            let Some(t325) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700081ff56); };
            c.w64(3, t325);
            c.w32(0, 0x1u64);
            let Some(t326) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081ff62); };
            c.w64(6, t326);
            return c.call(0x700086ed74, 0x700081ff6b, 0x700081ff66);
        }
        138 => {
            let Some(t327) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081ff6b); };
            c.w64(6, t327);
            let Some(t328) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700081ff6f); };
            c.w64(8, t328);
            blk = 140; continue;
        }
        139 => {
            let Some(t329) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700081ff75); };
            c.w64(6, t329);
            blk = 140; continue;
        }
        140 => {
            c.logic(c.r8(8) & 0xeu64, 8);
            if c.cc_e() { blk = 45; continue; }
            blk = 141; continue;
        }
        141 => {
            let Some(t330) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x700081ff83); };
            c.w64(1, t330);
            let Some(t331) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700081ff87); };
            c.w32(8, t331);
            let r332 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r332);
            let Some(t333) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700081ff8e); };
            c.cmp(t333, 0x6bu64, 16);
            if c.cc_ne() { blk = 143; continue; }
            blk = 142; continue;
        }
        142 => {
            let Some(t334) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700081ff96); };
            c.w32(1, t334);
            let r335 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r335);
            blk = 143; continue;
        }
        143 => {
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081ff9c); };
            c.cmp(t336, c.r32(1), 32);
            if c.cc_e() { blk = 45; continue; }
            blk = 144; continue;
        }
        144 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x700081ffa9); }
            let Some(t337) = c.ld64(c.r64(13).wrapping_add(0x2588)) else { return RegfileExit::fault(0x700081ffaa); };
            c.w64(3, t337);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x700081ffbb, 0x700081ffb6);
        }
        145 => {
            blk = 45; continue;
        }
        146 => {
            c.w64(10, c.r64(4));
            let r338 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r338);
            let r339 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r339);
            c.logic(r339, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700081ffcb); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700081ffd0); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x700081ffd4); }
            let Some(t340) = c.ld64(c.r64(13).wrapping_add(0x1628)) else { return RegfileExit::fault(0x700081ffd8); };
            c.w64(0, t340);
            c.w64(10, 0x700081ffe6u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700081ffe6); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x700081ffea); }
            return c.call(c.r64(0), 0x700081fff0, 0x700081ffee);
        }
        147 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700081fff0); }
            let Some(t341) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x700081fff8); };
            c.w64(4, t341);
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700081fffd); };
            c.w64(9, t342);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820004); };
            c.w64(0, t343);
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000820008); };
            c.w64(3, t344);
            let Some(t345) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700082000c); };
            c.w64(12, t345);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820013); };
            c.w64(6, t346);
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000820017); };
            c.w64(2, t347);
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x700082001b); };
            c.w64(15, t348);
            blk = 54; continue;
        }
        148 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000820024); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000820028); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x700082002c); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(15)) { return RegfileExit::fault(0x7000820030); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 160; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t349) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082003e); };
            c.w32(7, t349);
            let r350 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r350);
            let Some(t351) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000820044); };
            c.w32(7, t351);
            let r352 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r352);
            let Some(t353) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082004a); };
            c.w32(11, t353);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x8fu64, 64);
            if c.cc_be() { blk = 193; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t354) = c.ld32(c.r64(7).wrapping_add(0x243)) else { return RegfileExit::fault(0x7000820061); };
            c.w32(7, t354);
            let r355 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r355);
            let Some(t356) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082006a); };
            c.w32(11, t356);
            let r357 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r357);
            let Some(t358) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000820071); };
            c.logic(t358 & 0x2u64, 8);
            if c.cc_e() { blk = 160; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t359) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082007c); };
            c.w32(11, t359);
            let r360 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r360);
            let Some(t361) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820083); };
            c.w32(11, t361);
            c.cmp(c.r16(11), 0x83du64, 16);
            if c.cc_e() { blk = 155; continue; }
            blk = 152; continue;
        }
        152 => {
            c.w32(11, c.r16(11));
            c.cmp(c.r32(11), 0x6bu64, 32);
            if c.cc_ne() { blk = 160; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t362) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082009a); };
            c.w32(11, t362);
            let r363 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r363);
            let Some(t364) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008200a1); };
            c.w32(8, t364);
            let r365 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r365);
            let Some(t366) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008200a8); };
            c.cmp(t366, 0x83du64, 16);
            if c.cc_ne() { blk = 159; continue; }
            blk = 154; continue;
        }
        154 => {
            c.w64(8, c.r64(11));
            blk = 155; continue;
        }
        155 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008200b4); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008200b6); }
            c.w32(0, 0x1u64);
            c.w64(8, c.r64(0));
            return c.call(0x70007f2134, 0x70008200c5, 0x70008200c0);
        }
        156 => {
            blk = 158; continue;
        }
        157 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008200c7); }
            let Some(t367) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x70008200c8); };
            c.w64(3, t367);
            c.w32(0, 0x1u64);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008200d4); };
            c.w64(6, t368);
            return c.call(0x700086ed74, 0x70008200dd, 0x70008200d8);
        }
        158 => {
            let Some(t369) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008200dd); };
            c.w64(9, t369);
            let Some(t370) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008200e4); };
            c.w64(0, t370);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008200e8); };
            c.w64(3, t371);
            let Some(t372) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x70008200ec); };
            c.w64(12, t372);
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008200f3); };
            c.w64(6, t373);
            let Some(t374) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008200f7); };
            c.w64(8, t374);
            let Some(t375) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008200fb); };
            c.w64(1, t375);
            let Some(t376) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008200ff); };
            c.w64(2, t376);
            let Some(t377) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000820103); };
            c.w64(15, t377);
            blk = 160; continue;
        }
        159 => {
            let Some(t378) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000820109); };
            c.w64(8, t378);
            blk = 160; continue;
        }
        160 => {
            c.logic(c.r8(15) & 0xeu64, 8);
            if c.cc_e() { blk = 167; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t379) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820113); };
            c.w32(11, t379);
            let r380 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r380);
            let Some(t381) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082011a); };
            c.cmp(t381, 0x6bu64, 16);
            if c.cc_ne() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t382) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820122); };
            c.w32(11, t382);
            let r383 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r383);
            blk = 164; continue;
        }
        163 => {
            c.w64(11, c.r64(8));
            blk = 164; continue;
        }
        164 => {
            let Some(t384) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082012e); };
            c.cmp(t384, c.r32(11), 32);
            if c.cc_e() { blk = 167; continue; }
            blk = 165; continue;
        }
        165 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000820137); }
            c.w32(0, 0x1u64);
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x2590)) else { return RegfileExit::fault(0x700082013e); };
            c.w64(3, t385);
            c.w64(11, c.r64(3));
            c.w64(15, c.r64(0));
            return c.call(0x700086ed74, 0x7000820150, 0x700082014b);
        }
        166 => {
            let Some(t386) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000820150); };
            c.w64(9, t386);
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820157); };
            c.w64(0, t387);
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082015b); };
            c.w64(3, t388);
            let Some(t389) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x700082015f); };
            c.w64(12, t389);
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820166); };
            c.w64(6, t390);
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082016a); };
            c.w64(8, t391);
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082016e); };
            c.w64(1, t392);
            let Some(t393) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000820172); };
            c.w64(2, t393);
            blk = 167; continue;
        }
        167 => {
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000820176); };
            c.w64(11, t394);
            blk = 55; continue;
        }
        168 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082017f); }
            let Some(t395) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x7000820180); };
            c.w64(3, t395);
            c.w32(0, 0x1u64);
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082018c); };
            c.w64(6, t396);
            return c.call(0x700086ed74, 0x7000820195, 0x7000820190);
        }
        169 => {
            let Some(t397) = c.ld64(c.r64(13).wrapping_add(0x2d58)) else { return RegfileExit::fault(0x7000820195); };
            c.w64(8, t397);
            let Some(t398) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082019c); };
            c.w64(8, t398);
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082019f); };
            c.w64(9, t399);
            if !c.st64(c.r64(8).wrapping_add(0x30), c.r64(9)) { return RegfileExit::fault(0x70008201a3); }
            if !c.st64(c.r64(8).wrapping_add(0x48), c.r64(9)) { return RegfileExit::fault(0x70008201a7); }
            let Some(t400) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008201ab); };
            c.w64(11, t400);
            if !c.st64(c.r64(13).wrapping_add(0x2fe8), c.r64(11)) { return RegfileExit::fault(0x70008201af); }
            let Some(t401) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008201b6); };
            c.w64(0, t401);
            blk = 1; continue;
        }
        170 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008201c2); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 181; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t402) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008201d0); };
            c.w64(6, t402);
            let Some(t403) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008201d4); };
            c.w32(9, t403);
            let r404 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r404);
            let Some(t405) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008201db); };
            c.w32(9, t405);
            let r406 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r406);
            let Some(t407) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008201e2); };
            c.w32(11, t407);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0x90u64, 64);
            if c.cc_be() { blk = 194; continue; }
            blk = 172; continue;
        }
        172 => {
            let Some(t408) = c.ld32(c.r64(9).wrapping_add(0x247)) else { return RegfileExit::fault(0x70008201f9); };
            c.w32(7, t408);
            let r409 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r409);
            let Some(t410) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820203); };
            c.w32(9, t410);
            let r411 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r411);
            let Some(t412) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082020a); };
            c.logic(t412 & 0x2u64, 8);
            if c.cc_e() { blk = 182; continue; }
            blk = 173; continue;
        }
        173 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000820215); };
            c.w64(9, t413);
            let Some(t414) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820219); };
            c.w32(11, t414);
            let r415 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r415);
            let Some(t416) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820220); };
            c.w32(11, t416);
            c.cmp(c.r16(11), 0x83du64, 16);
            if c.cc_ne() { blk = 175; continue; }
            blk = 174; continue;
        }
        174 => {
            c.w64(11, c.r64(9));
            blk = 177; continue;
        }
        175 => {
            c.w32(11, c.r16(11));
            c.cmp(c.r32(11), 0x6bu64, 32);
            if c.cc_ne() { blk = 182; continue; }
            blk = 176; continue;
        }
        176 => {
            let Some(t417) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082023c); };
            c.w32(11, t417);
            let r418 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r418);
            let Some(t419) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820243); };
            c.w32(12, t419);
            let r420 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r420);
            let Some(t421) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082024a); };
            c.cmp(t421, 0x83du64, 16);
            if c.cc_ne() { blk = 182; continue; }
            blk = 177; continue;
        }
        177 => {
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x7000820254); }
            let Some(t422) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000820256); };
            c.w64(0, t422);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082025d); }
            c.w32(0, 0x1u64);
            c.w64(11, c.r64(0));
            return c.call(0x70007f2134, 0x700082026b, 0x7000820266);
        }
        178 => {
            blk = 180; continue;
        }
        179 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082026d); }
            let Some(t423) = c.ld64(c.r64(13).wrapping_add(0x2188)) else { return RegfileExit::fault(0x700082026e); };
            c.w64(3, t423);
            c.w32(0, 0x1u64);
            let Some(t424) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082027a); };
            c.w64(6, t424);
            return c.call(0x700086ed74, 0x7000820283, 0x700082027e);
        }
        180 => {
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820283); };
            c.w64(6, t425);
            let Some(t426) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000820287); };
            c.w64(8, t426);
            blk = 182; continue;
        }
        181 => {
            let Some(t427) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082028d); };
            c.w64(6, t427);
            blk = 182; continue;
        }
        182 => {
            c.logic(c.r8(8) & 0xeu64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082029b); };
            c.w64(8, t428);
            let Some(t429) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082029f); };
            c.w32(9, t429);
            let r430 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r430);
            let Some(t431) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008202a6); };
            c.cmp(t431, 0x6bu64, 16);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t432) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008202ae); };
            c.w32(8, t432);
            let r433 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r433);
            blk = 185; continue;
        }
        185 => {
            let Some(t434) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008202b5); };
            c.cmp(t434, c.r32(8), 32);
            if c.cc_e() { blk = 57; continue; }
            blk = 186; continue;
        }
        186 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008202c2); }
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0x2588)) else { return RegfileExit::fault(0x70008202c4); };
            c.w64(3, t435);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008202d5, 0x70008202d0);
        }
        187 => {
            blk = 57; continue;
        }
        188 => {
            return RegfileExit::fault(0x70008202da);
        }
        189 => {
            return RegfileExit::fault(0x70008202db);
        }
        190 => {
            return RegfileExit::fault(0x70008202dc);
        }
        191 => {
            return RegfileExit::fault(0x70008202dd);
        }
        192 => {
            return RegfileExit::fault(0x70008202de);
        }
        193 => {
            return RegfileExit::fault(0x70008202df);
        }
        194 => {
            return RegfileExit::fault(0x70008202e0);
        }
        195 => {
            c.w32(15, c.r32(15));
            let r436 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r436);
            c.w64(8, 0xfffffffffffc0000u64);
            let r437 = c.r64(8) & c.r64(15);
            c.w64(8, r437);
            c.logic(r437, 64);
            let Some(t438) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008202f1); };
            c.logic(t438 & 0x2u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 196; continue;
        }
        196 => {
            c.w64(11, c.r64(2).wrapping_add(0x1b));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000820300); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000820301); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x700082030e, 0x7000820309);
        }
        197 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700082030e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000820310); }
            blk = 42; continue;
        }
        198 => {
            c.w32(15, c.r32(15));
            let r439 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r439);
            c.w64(8, 0xfffffffffffc0000u64);
            let r440 = c.r64(8) & c.r64(15);
            c.w64(8, r440);
            c.logic(r440, 64);
            let Some(t441) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000820326); };
            c.logic(t441 & 0x2u64, 8);
            if c.cc_e() { blk = 26; continue; }
            blk = 199; continue;
        }
        199 => {
            c.w64(11, c.r64(2).wrapping_add(0x1b));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000820335); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000820336); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f0e54, 0x7000820343, 0x700082033e);
        }
        200 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000820343); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000820345); }
            blk = 26; continue;
        }
        201 => {
            return RegfileExit::jump(0x700082034c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000820354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000820354); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000820358); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700082035e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000820362); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000820366); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 246; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820372); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820378); };
            c.w32(8, t3);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 246; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 241; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082039e); };
            c.w32(9, t4);
            let r5 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r5);
            let Some(t6) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008203a5); };
            c.w32(9, t6);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008203b1); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(11) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(12, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r7);
            c.w64(15, 0x3ffffffffffffeu64);
            c.cmp(c.r64(12), c.r64(15), 64);
            if c.cc_be() { blk = 97; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_e() { blk = 18; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 231; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008203fe); };
            c.w32(11, t8);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 246; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(9), 0x25u64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(9), 0x2du64, 16);
            if c.cc_e() { blk = 16; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(9) & 0x20u64, 8);
            if c.cc_ne() { blk = 246; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(9, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820432); };
            c.w32(9, t9);
            let r10 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r10);
            blk = 19; continue;
        }
        17 => {
            if let Some(sr) = c.shr(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let r11 = c.r64(11) & 0xffffffu64;
            c.w64(11, r11);
            c.logic(r11, 64);
            blk = 97; continue;
        }
        18 => {
            c.w64(9, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x700082044e); };
            c.w64(15, t12);
            c.w64(11, c.r64(0));
            blk = 21; continue;
        }
        20 => {
            let Some(t13) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082045a); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            let Some(t15) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820461); };
            c.w32(8, t15);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 50; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t16) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820473); };
            c.w32(1, t16);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t17) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000820482); };
            c.w32(11, t17);
            let r18 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r18);
            c.logic(c.r32(1) & 0xffc00u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 24; continue;
        }
        24 => {
            let r19 = c.shr(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r19);
            let r20 = c.r32(1) & 0x3ffu64;
            c.w32(1, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_be() { blk = 39; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t21) = c.ld16(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008204a7); };
            c.w32(0, (t21 as u16 as i16 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008204ac); };
            c.w32(6, t22);
            let r23 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r23);
            let r24 = c.sub(c.r32(0), 0x1u64, 32);
            c.w32(0, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008204b6); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008204be); }
            blk = 27; continue;
        }
        27 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008204c2); };
            c.w64(2, t25);
            let r26 = c.sub(c.r32(2), c.r32(3), 32);
            c.w32(2, r26);
            let r27 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r27);
            let r28 = c.add(c.r32(2), c.r32(3), 32);
            c.w32(2, r28);
            c.w32(12, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008204d3); };
            c.w32(12, t29);
            let r30 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r30);
            let r31 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r31);
            let r32 = c.r32(12) & 0x3ffu64;
            c.w32(12, r32);
            c.logic(r32, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008204ed); };
            c.w32(12, t33);
            let r34 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r34);
            let Some(t35) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008204f5); };
            c.w32(12, t35);
            let r36 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r36);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_be() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(3, c.r64(2).wrapping_add(0x1));
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000820506); };
            c.w64(2, t37);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x700082050e); }
            blk = 32; continue;
        }
        31 => {
            let r38 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r38);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(12, c.r64(3).wrapping_add(c.r64(3).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t39) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000820522); };
            c.w32(12, t39);
            let r40 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r40);
            let r41 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r41);
            let r42 = c.r32(12) & 0x3ffu64;
            c.w32(12, r42);
            c.logic(r42, 32);
            c.w32(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t43) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082053c); };
            c.w32(2, t43);
            let r44 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820544); };
            c.w32(7, t45);
            let r46 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r46);
            c.cmp(c.r32(6), c.r32(7), 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_a() { blk = 227; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t47) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082055c); };
            c.w64(7, t47);
            blk = 61; continue;
        }
        37 => {
            let Some(t48) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000820565); };
            c.w64(7, t48);
            blk = 61; continue;
        }
        38 => {
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x700082056e); };
            c.w64(7, t49);
            blk = 61; continue;
        }
        39 => {
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            let r50 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r50);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            let r51 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r51);
            let Some(t52) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082058c); };
            c.cmp(t52, c.r32(9), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_ne() { blk = 40; continue; }
            blk = 42; continue;
        }
        42 => {
            blk = 61; continue;
        }
        43 => {
            let Some(t53) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008205a2); };
            c.w32(11, t53);
            let r54 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r54);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(11, c.r64(15));
            blk = 45; continue;
        }
        45 => {
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008205b2); };
            c.w32(12, t55);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r56 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r56);
            let Some(t57) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008205c0); };
            c.w32(0, t57);
            let r58 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r58);
            let r59 = c.r64(0) & c.r64(12);
            c.w64(0, r59);
            c.logic(r59, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        46 => {
            let r60 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r60);
            let r61 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r61);
            c.w64(2, c.r64(12));
            let r62 = c.r64(2) & c.r64(0);
            c.w64(2, r62);
            c.logic(r62, 64);
            c.w64(0, c.r64(2));
            blk = 47; continue;
        }
        47 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t63) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008205e3); };
            c.w32(2, t63);
            let Some(t64) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008205e8); };
            c.cmp(t64, c.r32(2), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r32(2), c.r32(9), 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 49; continue;
        }
        49 => {
            blk = 46; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 246; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t65) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082060d); };
            c.logic(t65 & 0x24u64, 8);
            if c.cc_ne() { blk = 246; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t66) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820617); };
            c.w32(11, t66);
            let r67 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r67);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(15));
            blk = 54; continue;
        }
        54 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000820627); };
            c.w32(12, t68);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r69 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r69);
            let Some(t70) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820635); };
            c.w32(0, t70);
            let r71 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r71);
            let r72 = c.r64(0) & c.r64(12);
            c.w64(0, r72);
            c.logic(r72, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r73 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r73);
            let r74 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r74);
            c.w64(2, c.r64(12));
            let r75 = c.r64(2) & c.r64(0);
            c.w64(2, r75);
            c.logic(r75, 64);
            c.w64(0, c.r64(2));
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t76) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820657); };
            c.w32(1, t76);
            let r77 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r77);
            let Some(t78) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082065f); };
            c.cmp(t78, c.r32(1), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t79) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000820668); };
            c.cmp(t79, c.r32(1), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t80) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820671); };
            c.cmp(t80, c.r32(9), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t81) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820677); };
            c.w32(11, t81);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 239; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082068a); };
            c.w32(11, t82);
            let Some(t83) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082068e); };
            c.cmp(t83, c.r32(11), 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t84) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008206a8); };
            c.w32(8, t84);
            let r85 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r85);
            let Some(t86) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008206af); };
            c.w32(8, t86);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 92; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t87) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008206bf); };
            c.w32(11, t87);
            let r88 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r88);
            c.w32(12, c.r32(11));
            c.cmp(c.r32(12), 0x0u64, 32);
            if c.cc_e() { blk = 92; continue; }
            blk = 64; continue;
        }
        64 => {
            c.cmp(c.r32(11), 0x30u64, 32);
            if c.cc_g() { blk = 92; continue; }
            blk = 65; continue;
        }
        65 => {
            c.w32(11, c.r16(8));
            c.logic(c.r8(8) & 0x7u64, 8);
            if c.cc_e() { blk = 81; continue; }
            blk = 66; continue;
        }
        66 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 67; continue;
        }
        67 => {
            c.w64(0, c.r64(11));
            let r89 = c.r32(0) & 0x7u64;
            c.w32(0, r89);
            c.logic(r89, 32);
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_l() { blk = 72; continue; }
            blk = 68; continue;
        }
        68 => {
            c.cmp(c.r32(0), 0x2u64, 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 70; continue;
        }
        70 => {
            c.cmp(c.r32(0), 0x5u64, 32);
            if c.cc_e() { blk = 78; continue; }
            blk = 71; continue;
        }
        71 => {
            blk = 233; continue;
        }
        72 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 79; continue; }
            blk = 73; continue;
        }
        73 => {
            c.cmp(c.r32(0), 0x1u64, 32);
            if c.cc_e() { blk = 75; continue; }
            blk = 74; continue;
        }
        74 => {
            blk = 233; continue;
        }
        75 => {
            let Some(t90) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000820725); };
            c.w32(11, t90);
            let Some(t91) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x700082072a); };
            c.cmp(t91, c.r32(11), 32);
            if c.cc_ne() { blk = 234; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820737); };
            c.w32(12, t92);
            let r93 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r93);
            let Some(t94) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082073f); };
            c.w32(11, t94);
            let r95 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r95);
            let Some(t96) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820747); };
            c.w32(11, t96);
            blk = 67; continue;
        }
        77 => {
            let Some(t97) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082074e); };
            c.w32(11, t97);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r98 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r98);
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082075c); };
            c.w32(12, t99);
            let r100 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r100);
            let Some(t101) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820764); };
            c.w32(11, t101);
            let r102 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r102);
            let Some(t103) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082076c); };
            c.w32(11, t103);
            blk = 67; continue;
        }
        78 => {
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820773); };
            c.w32(12, t104);
            let r105 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r105);
            let Some(t106) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082077b); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820783); };
            c.w32(11, t108);
            blk = 67; continue;
        }
        79 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 82; continue;
        }
        80 => {
            c.w32(0, 0x1u64);
            blk = 82; continue;
        }
        81 => {
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(9));
            c.w64(8, c.r64(6));
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 235; continue; }
            blk = 83; continue;
        }
        83 => {
            let r109 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r109);
            blk = 84; continue;
        }
        84 => {
            let r110 = c.r32(11) & 0x8u64;
            c.w32(11, r110);
            c.logic(r110, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t111) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70008207b9); };
            c.w32(8, t111);
            blk = 87; continue;
        }
        86 => {
            let Some(t112) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008207c0); };
            c.w32(8, t112);
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 246; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 246; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008207f3); };
            c.w64(12, t113);
            let Some(t114) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008207f7); };
            c.w32(8, t114);
            let Some(t115) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008207fc); };
            c.cmp(t115, c.r32(8), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t116) = c.ld8(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820805); };
            c.logic(t116 & 0x1u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t117) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000820811); };
            c.w32(11, t117);
            let r118 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r118);
            let Some(t119) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000820818); };
            c.cmp(t119, c.r32(11), 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 95; continue;
        }
        95 => {
            blk = 20; continue;
        }
        96 => {
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(9));
            blk = 97; continue;
        }
        97 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x7000820833); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(0));
            blk = 99; continue;
        }
        98 => {
            let Some(t120) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820842); };
            c.w32(8, t120);
            let r121 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r121);
            let Some(t122) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820849); };
            c.w32(9, t122);
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 246; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t123) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x700082085b); };
            c.w32(9, t123);
            let r124 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r124);
            let r125 = c.r32(9) & 0x3fu64;
            c.w32(9, r125);
            c.logic(r125, 32);
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_l() { blk = 135; continue; }
            blk = 101; continue;
        }
        101 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_l() { blk = 119; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_l() { blk = 111; continue; }
            blk = 103; continue;
        }
        103 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_l() { blk = 108; continue; }
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(9), 0x25u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r32(9), 0x26u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 106; continue;
        }
        106 => {
            c.cmp(c.r32(9), 0x28u64, 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 107; continue;
        }
        107 => {
            blk = 246; continue;
        }
        108 => {
            c.cmp(c.r32(9), 0x23u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 109; continue;
        }
        109 => {
            c.cmp(c.r32(9), 0x24u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 110; continue;
        }
        110 => {
            blk = 246; continue;
        }
        111 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_l() { blk = 116; continue; }
            blk = 112; continue;
        }
        112 => {
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 113; continue;
        }
        113 => {
            c.cmp(c.r32(9), 0x21u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 114; continue;
        }
        114 => {
            c.cmp(c.r32(9), 0x22u64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 115; continue;
        }
        115 => {
            blk = 246; continue;
        }
        116 => {
            c.cmp(c.r32(9), 0x1eu64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 117; continue;
        }
        117 => {
            c.cmp(c.r32(9), 0x1fu64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 118; continue;
        }
        118 => {
            blk = 246; continue;
        }
        119 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_l() { blk = 128; continue; }
            blk = 120; continue;
        }
        120 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_l() { blk = 125; continue; }
            blk = 121; continue;
        }
        121 => {
            c.cmp(c.r32(9), 0x1bu64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(9), 0x1cu64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(9), 0x1du64, 32);
            if c.cc_e() { blk = 198; continue; }
            blk = 124; continue;
        }
        124 => {
            blk = 246; continue;
        }
        125 => {
            c.cmp(c.r32(9), 0x19u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 126; continue;
        }
        126 => {
            c.cmp(c.r32(9), 0x1au64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 127; continue;
        }
        127 => {
            blk = 246; continue;
        }
        128 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_l() { blk = 132; continue; }
            blk = 129; continue;
        }
        129 => {
            c.cmp(c.r32(9), 0x17u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r32(9), 0x18u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 246; continue;
        }
        132 => {
            c.cmp(c.r32(9), 0x15u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 133; continue;
        }
        133 => {
            c.cmp(c.r32(9), 0x16u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 134; continue;
        }
        134 => {
            blk = 246; continue;
        }
        135 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_l() { blk = 153; continue; }
            blk = 136; continue;
        }
        136 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_l() { blk = 145; continue; }
            blk = 137; continue;
        }
        137 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_l() { blk = 142; continue; }
            blk = 138; continue;
        }
        138 => {
            c.cmp(c.r32(9), 0x12u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 139; continue;
        }
        139 => {
            c.cmp(c.r32(9), 0x13u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 140; continue;
        }
        140 => {
            c.cmp(c.r32(9), 0x14u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 141; continue;
        }
        141 => {
            blk = 246; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x10u64, 32);
            if c.cc_e() { blk = 179; continue; }
            blk = 143; continue;
        }
        143 => {
            c.cmp(c.r32(9), 0x11u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 144; continue;
        }
        144 => {
            blk = 246; continue;
        }
        145 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_l() { blk = 150; continue; }
            blk = 146; continue;
        }
        146 => {
            c.cmp(c.r32(9), 0xbu64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 147; continue;
        }
        147 => {
            c.cmp(c.r32(9), 0xcu64, 32);
            if c.cc_e() { blk = 180; continue; }
            blk = 148; continue;
        }
        148 => {
            c.cmp(c.r32(9), 0xfu64, 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 149; continue;
        }
        149 => {
            blk = 246; continue;
        }
        150 => {
            c.cmp(c.r32(9), 0x9u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 151; continue;
        }
        151 => {
            c.cmp(c.r32(9), 0xau64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 152; continue;
        }
        152 => {
            blk = 246; continue;
        }
        153 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 162; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_l() { blk = 159; continue; }
            blk = 155; continue;
        }
        155 => {
            c.cmp(c.r32(9), 0x6u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 156; continue;
        }
        156 => {
            c.cmp(c.r32(9), 0x7u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 157; continue;
        }
        157 => {
            c.cmp(c.r32(9), 0x8u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 158; continue;
        }
        158 => {
            blk = 246; continue;
        }
        159 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(9), 0x5u64, 32);
            if c.cc_e() { blk = 170; continue; }
            blk = 161; continue;
        }
        161 => {
            blk = 246; continue;
        }
        162 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_l() { blk = 166; continue; }
            blk = 163; continue;
        }
        163 => {
            c.cmp(c.r32(9), 0x2u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 164; continue;
        }
        164 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 165; continue;
        }
        165 => {
            blk = 246; continue;
        }
        166 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 167; continue;
        }
        167 => {
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 174; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 246; continue;
        }
        169 => {
            blk = 174; continue;
        }
        170 => {
            let Some(t126) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820a84); };
            c.w32(7, t126);
            let r127 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r127);
            let Some(t128) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820a8a); };
            c.w32(9, t128);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 175; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t129) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820aa3); };
            c.cmp(t129, c.r64(9), 64);
            if c.cc_e() { blk = 193; continue; }
            blk = 172; continue;
        }
        172 => {
            blk = 227; continue;
        }
        173 => {
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820ab3); };
            c.w32(9, t130);
            let r131 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r131);
            let Some(t132) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820aba); };
            c.w32(9, t132);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 227; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t133) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820ac7); };
            c.w32(7, t133);
            let r134 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r134);
            let Some(t135) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820acd); };
            c.w32(9, t135);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 177; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w32(7, 0xfffffffeu64);
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_b() { blk = 246; continue; }
            blk = 176; continue;
        }
        176 => {
            blk = 193; continue;
        }
        177 => {
            let Some(t136) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820aef); };
            c.w32(7, t136);
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000820af4); };
            c.cmp(t137, c.r32(7), 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 227; continue;
        }
        179 => {
            let Some(t138) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820b06); };
            c.w32(9, t138);
            let r139 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r139);
            let Some(t140) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820b0d); };
            c.w32(9, t140);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 227; continue; }
            blk = 180; continue;
        }
        180 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x7000820b1a); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 246; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t141) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820b2d); };
            c.w32(7, t141);
            let r142 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r142);
            let Some(t143) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000820b33); };
            c.w32(9, t143);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r144 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r144);
            c.w32(2, c.r32(11));
            c.w64(10, c.r64(4));
            let r145 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r145);
            let r146 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r146);
            c.logic(r146, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000820b4f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000820b54); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x7000820b58); }
            c.w64(1, c.r64(13));
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x7000820b5f); };
            c.w64(0, t147);
            c.w64(10, 0x7000820b6du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000820b6d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000820b71); }
            return c.call(c.r64(0), 0x7000820b77, 0x7000820b75);
        }
        182 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000820b77); }
            let Some(t148) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000820b7f); };
            c.w64(4, t148);
            c.w32(7, c.r32(0));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000820b86); };
            let r150 = c.r64(7) & t149;
            c.w64(7, r150);
            c.logic(r150, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000820b8d); };
            c.w64(9, t151);
            blk = 184; continue;
        }
        183 => {
            let r152 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r152);
            let r153 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000820b9a); };
            c.w64(11, t154);
            let r155 = c.r64(11) & c.r64(7);
            c.w64(11, r155);
            c.logic(r155, 64);
            c.w64(7, c.r64(11));
            blk = 184; continue;
        }
        184 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t156) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820ba9); };
            c.w32(11, t156);
            let r157 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r157);
            let Some(t158) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000820bb1); };
            c.cmp(t158, c.r32(11), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 185; continue;
        }
        185 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t159) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000820bc0); };
            c.cmp(t159, c.r32(11), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 187; continue;
        }
        187 => {
            c.xorpd_rr(0, 0);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820bcd); };
            c.cvtsi2sd(0, (t160 as i64));
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000820bd3); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 183; continue; }
            blk = 188; continue;
        }
        188 => {
            if c.cc_e() { blk = 227; continue; }
            blk = 189; continue;
        }
        189 => {
            blk = 183; continue;
        }
        190 => {
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let Some(t161) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820bed); };
            c.cmp(c.r64(11), t161, 64);
            if c.cc_e() { blk = 227; continue; }
            blk = 191; continue;
        }
        191 => {
            blk = 183; continue;
        }
        192 => {
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000820bf9); };
            c.w64(8, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820bfd); };
            c.w64(11, t163);
            blk = 193; continue;
        }
        193 => {
            c.w64(7, c.r64(8));
            let Some(t164) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000820c04); };
            c.w32(7, t164);
            let r165 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r165);
            let Some(t166) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820c0a); };
            c.w64(0, t166);
            let Some(t167) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000820c0e); };
            c.w64(3, t167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000820c12); };
            c.w64(6, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000820c16); };
            c.cmp(t169, c.r32(7), 32);
            if c.cc_e() { blk = 229; continue; }
            blk = 194; continue;
        }
        194 => {
            blk = 98; continue;
        }
        195 => {
            let Some(t170) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820c28); };
            c.w32(8, t170);
            let r171 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r171);
            let Some(t172) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000820c2f); };
            c.logic(t172 & 0x4u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t173) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000820c3a); };
            c.cmp(t173, c.r64(11), 64);
            if c.cc_a() { blk = 227; continue; }
            blk = 197; continue;
        }
        197 => {
            blk = 229; continue;
        }
        198 => {
            let Some(t174) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820c49); };
            c.w32(8, t174);
            let r175 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r175);
            let Some(t176) = c.ld32(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x7000820c50); };
            c.w32(9, t176);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 224; continue; }
            blk = 199; continue;
        }
        199 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t177) = c.ld64(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820c68); };
            c.w64(8, t177);
            let Some(t178) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000820c6c); };
            c.w64(9, t178);
            let Some(t179) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000820c70); };
            c.logic(t179 & 0x1u64, 8);
            if c.cc_ne() { blk = 203; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t180) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000820c76); };
            c.w64(12, t180);
            let r181 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r181);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t182) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000820c86); };
            c.w64(7, t182);
            c.w64(0, c.r64(7));
            blk = 226; continue;
        }
        203 => {
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 229; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t183) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820c9b); };
            c.w32(7, t183);
            let r184 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r184);
            let Some(t185) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x7000820ca1); };
            c.w32(7, t185);
            let r186 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r186);
            let r187 = c.r32(7) & 0x3fu64;
            c.w32(7, r187);
            c.logic(r187, 32);
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_l() { blk = 212; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_l() { blk = 209; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(7), 0x24u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(7), 0x25u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 208; continue;
        }
        208 => {
            c.cmp(c.r32(7), 0x26u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 209; continue;
        }
        209 => {
            c.cmp(c.r32(7), 0x21u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 210; continue;
        }
        210 => {
            c.cmp(c.r32(7), 0x22u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 211; continue;
        }
        211 => {
            c.cmp(c.r32(7), 0x23u64, 32);
            if c.cc_e() { blk = 222; continue; }
            blk = 212; continue;
        }
        212 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_l() { blk = 216; continue; }
            blk = 213; continue;
        }
        213 => {
            c.cmp(c.r32(7), 0x1eu64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 214; continue;
        }
        214 => {
            c.cmp(c.r32(7), 0x1fu64, 32);
            if c.cc_e() { blk = 220; continue; }
            blk = 215; continue;
        }
        215 => {
            c.cmp(c.r32(7), 0x20u64, 32);
            if c.cc_e() { blk = 221; continue; }
            blk = 216; continue;
        }
        216 => {
            c.cmp(c.r32(7), 0x1cu64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(7), 0x1du64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            return RegfileExit::fault(0x7000820cf1);
        }
        219 => {
            c.w32(7, 0x1u64);
            blk = 223; continue;
        }
        220 => {
            c.w32(7, 0x2u64);
            blk = 223; continue;
        }
        221 => {
            c.w32(7, 0x4u64);
            blk = 223; continue;
        }
        222 => {
            c.w32(7, 0x8u64);
            blk = 223; continue;
        }
        223 => {
            let r188 = c.sub(c.r64(8), c.r64(9), 64);
            c.w64(8, r188);
            c.w64(0, c.r64(8));
            c.cqo();
            c.idiv(c.r64(7), 64);
            blk = 226; continue;
        }
        224 => {
            c.w64(10, c.r64(4));
            let r189 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r189);
            let r190 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r190);
            c.logic(r190, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000820d24); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t191) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x7000820d2f); };
            c.w64(0, t191);
            c.w64(10, 0x7000820d3du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000820d3d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000820d41); }
            return c.call(c.r64(0), 0x7000820d47, 0x7000820d45);
        }
        225 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000820d47); }
            let Some(t192) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000820d4f); };
            c.w64(4, t192);
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820d54); };
            c.w64(11, t193);
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ae() { blk = 229; continue; }
            blk = 227; continue;
        }
        227 => {
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000820d5d); };
            c.w64(0, t194);
            blk = 228; continue;
        }
        228 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000820d67); }
            return c.ret(0x0, 0x7000820d68);
        }
        229 => {
            let Some(t195) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000820d69); };
            c.w64(0, t195);
            blk = 228; continue;
        }
        230 => {
            blk = 246; continue;
        }
        231 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 246; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t196) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820d82); };
            c.w32(9, t196);
            let r197 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r197);
            blk = 19; continue;
        }
        233 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000820d8e); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000820d92); }
            blk = 237; continue;
        }
        234 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000820d98); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000820d9c); }
            blk = 237; continue;
        }
        235 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000820da2); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x7000820da6); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 237; continue; }
            blk = 236; continue;
        }
        236 => {
            let Some(t198) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000820db0); };
            c.w64(12, t198);
            blk = 84; continue;
        }
        237 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x7000820dba); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x7000820dbc); }
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x7000820dbd); };
            c.w64(3, t199);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x7000820dce, 0x7000820dc9);
        }
        238 => {
            let r200 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r200);
            c.w64(8, c.r64(0));
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000820dd3); };
            c.w64(7, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820dd7); };
            c.w64(9, t202);
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000820ddb); };
            c.w64(15, t203);
            blk = 87; continue;
        }
        239 => {
            return RegfileExit::fault(0x7000820de7);
        }
        240 => {
            blk = 246; continue;
        }
        241 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a1af4, 0x7000820df2, 0x7000820ded);
        }
        242 => {
            let Some(t204) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820df2); };
            c.w32(7, t204);
            let Some(t205) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000820df5); };
            c.cmp(t205, c.r32(7), 32);
            if c.cc_ne() { blk = 244; continue; }
            blk = 243; continue;
        }
        243 => {
            let Some(t206) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820dfe); };
            c.logic(t206 & 0x1u64, 8);
            if c.cc_ne() { blk = 229; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w64(3, c.r64(0));
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820e0b); };
            c.w64(0, t207);
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000820e0f); };
            c.w64(6, t208);
            return c.call(0x70008bced4, 0x7000820e18, 0x7000820e13);
        }
        245 => {
            blk = 228; continue;
        }
        246 => {
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820e1d); };
            if !c.push64(t209) { return RegfileExit::fault(0x7000820e1d); }
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000820e20); };
            if !c.push64(t210) { return RegfileExit::fault(0x7000820e20); }
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x7000820e23); };
            c.w64(3, t211);
            c.w32(0, 0x2u64);
            let Some(t212) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000820e2f); };
            c.w64(6, t212);
            return c.call(0x700086ed74, 0x7000820e38, 0x7000820e33);
        }
        247 => {
            blk = 228; continue;
        }
        248 => {
            return RegfileExit::jump(0x7000820e40);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000820e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000820e54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000820e58); }
            let r0 = c.sub(c.r64(4), 0x30u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000820e5e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000820e62); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000820e66); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x7000820e6a); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 65; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820e76); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820e7c); };
            c.w32(8, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x7000820e81); }
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 60; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(8), 0x411u64, 16);
            if c.cc_be() { blk = 65; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 65; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000820ea6); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000820ead); };
            c.w32(8, t6);
            c.cmp(c.r16(8), 0x42u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x7000820eb9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(9, cv);
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r64(9) as i64));
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            if c.cc_ne() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(11, 0x1fffffffffffffu64);
            let r7 = c.add(c.r64(11), c.r64(9), 64);
            c.w64(11, r7);
            c.w64(9, 0x3ffffffffffffeu64);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_be() { blk = 65; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_e() { blk = 24; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 66; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000820f0a); };
            c.w32(9, t8);
            c.logic(c.r32(9) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 65; continue; }
            blk = 12; continue;
        }
        12 => {
            c.cmp(c.r16(8), 0x25u64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r16(8), 0x2du64, 16);
            if c.cc_e() { blk = 23; continue; }
            blk = 14; continue;
        }
        14 => {
            c.logic(c.r8(8) & 0x20u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(8, c.r64(3));
            blk = 25; continue;
        }
        16 => {
            c.w64(10, c.r64(4));
            let r9 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r9);
            let r10 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r10);
            c.logic(r10, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000820f54); }
            c.w64(2, c.r64(3));
            c.w64(1, c.r64(13));
            c.w64(8, c.r64(1));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x1480)) else { return RegfileExit::fault(0x7000820f62); };
            c.w64(0, t11);
            c.w64(10, 0x7000820f70u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000820f70); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000820f74); }
            return c.call(c.r64(0), 0x7000820f7a, 0x7000820f78);
        }
        17 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x7000820f7a); }
            let Some(t12) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000820f82); };
            c.w64(4, t12);
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 22; continue; }
            blk = 18; continue;
        }
        18 => {
            c.w64(7, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0xffffffffffffffffu64, 64);
            if c.cc_e() { blk = 59; continue; }
            blk = 19; continue;
        }
        19 => {
            c.cmp(c.r64(7), 0xfffffffffffffffeu64, 64);
            if c.cc_e() { blk = 21; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820fa1); };
            c.w64(0, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000820fa5); };
            c.w64(3, t14);
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000820fa9); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820fad); };
            c.w64(6, t16);
            blk = 65; continue;
        }
        21 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820fb6); };
            c.w64(0, t17);
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000820fba); };
            c.w64(3, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000820fbe); };
            c.w64(1, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820fc2); };
            c.w64(6, t20);
            blk = 65; continue;
        }
        22 => {
            c.w64(8, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000820fce); };
            c.w64(0, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000820fd2); };
            c.w64(3, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000820fd6); };
            c.w64(1, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000820fda); };
            c.w64(6, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000820fde); };
            c.w64(7, t25);
            blk = 25; continue;
        }
        23 => {
            c.w64(8, c.r64(3));
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000820fe7); };
            c.w32(9, t26);
            let r27 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r27);
            c.w64(3, c.r64(8));
            c.w64(8, c.r64(9));
            blk = 25; continue;
        }
        24 => {
            c.w64(8, c.r64(3));
            blk = 25; continue;
        }
        25 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000820ff9); }
            let Some(t28) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000820ffd); };
            c.cmp(t28, c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t29) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x700082100a); };
            c.cmp(t29, c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t30) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000821017); };
            c.cmp(t30, c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t31) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000821024); };
            c.cmp(t31, c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t32) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000821031); };
            c.cmp(t32, c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t33) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x700082103e); };
            c.cmp(t33, c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t34) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x700082104b); };
            c.cmp(t34, c.r32(8), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t35) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821058); };
            c.w32(9, t35);
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 33; continue;
        }
        33 => {
            c.logic(c.r32(9) & 0x100000u64, 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 34; continue;
        }
        34 => {
            let Some(t36) = c.ld32(c.r64(7).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000821072); };
            c.w32(9, t36);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(10, c.r64(4));
            let r37 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r37);
            let r38 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r38);
            c.logic(r38, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x7000821087); }
            c.w64(1, c.r64(7));
            let Some(t39) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x700082108f); };
            c.w64(0, t39);
            c.w64(10, 0x700082109du64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x700082109d); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008210a1); }
            return c.call(c.r64(0), 0x70008210a7, 0x70008210a5);
        }
        36 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008210a7); }
            let Some(t40) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008210af); };
            c.w64(4, t40);
            let Some(t41) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008210b4); };
            c.w64(0, t41);
            let Some(t42) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008210b8); };
            c.w64(3, t42);
            let Some(t43) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008210bc); };
            c.w64(1, t43);
            let Some(t44) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008210c0); };
            c.w64(6, t44);
            let Some(t45) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008210c4); };
            c.w64(8, t45);
            blk = 37; continue;
        }
        37 => {
            let Some(t46) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008210c8); };
            c.w32(7, t46);
            let r47 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r47);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t48) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008210d4); };
            c.w64(7, t48);
            blk = 39; continue;
        }
        39 => {
            let Some(t49) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008210db); };
            c.w32(9, t49);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r50 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r50);
            let Some(t51) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008210e9); };
            c.w32(11, t51);
            let r52 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r52);
            let r53 = c.r64(11) & c.r64(9);
            c.w64(11, r53);
            c.logic(r53, 64);
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(12));
            blk = 41; continue;
        }
        40 => {
            let r54 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r54);
            let r55 = c.add(c.r64(11), c.r64(15), 64);
            c.w64(11, r55);
            c.w64(3, c.r64(9));
            let r56 = c.r64(3) & c.r64(11);
            c.w64(3, r56);
            c.logic(r56, 64);
            c.w64(11, c.r64(3));
            blk = 41; continue;
        }
        41 => {
            c.w64(2, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(2).wrapping_mul(4));
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821119); };
            c.w32(6, t57);
            let Some(t58) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082111d); };
            c.cmp(t58, c.r32(6), 32);
            if c.cc_e() { blk = 59; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r32(8), c.r32(6), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082112f); };
            c.w32(8, t59);
            c.logic(c.r8(8) & 0x40u64, 8);
            if c.cc_ne() { blk = 56; continue; }
            blk = 44; continue;
        }
        44 => {
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082113e); };
            c.w32(8, t60);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(2), c.r64(8), 64);
            if c.cc_ae() { blk = 68; continue; }
            blk = 45; continue;
        }
        45 => {
            let Some(t61) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000821151); };
            c.w64(8, t61);
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x7000821158); }
            let Some(t62) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082115d); };
            c.w32(9, t62);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(11, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 69; continue; }
            blk = 46; continue;
        }
        46 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000821174); }
            let Some(t63) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821179); };
            c.w32(8, t63);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(9, c.r64(2).wrapping_add(0x2));
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 70; continue; }
            blk = 47; continue;
        }
        47 => {
            if !c.st32(c.r64(7).wrapping_add(c.r64(3)).wrapping_add(0xf), c.r32(12)) { return RegfileExit::fault(0x7000821190); }
            let Some(t64) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821195); };
            c.w32(8, t64);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 71; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008211a9); };
            c.w32(8, t65);
            let r66 = c.sub(c.r32(8), 0x2u64, 32);
            c.w32(8, r66);
            c.w64(9, (c.r32(8) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008211b4); }
            let Some(t67) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008211b8); };
            c.w32(9, t67);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x1u64, 64);
            if c.cc_be() { blk = 72; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t68) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008211cc); };
            c.w32(9, t68);
            let r69 = c.add(c.r32(9), 0x2u64, 32);
            c.w32(9, r69);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008211d7); }
            let Some(t70) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008211db); };
            c.w32(9, t70);
            let r71 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r71);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r72 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r72);
            c.logic(r72, 64);
            c.cmp(c.r32(9), c.r32(8), 32);
            if c.cc_l() { blk = 54; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_l() { blk = 54; continue; }
            blk = 51; continue;
        }
        51 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008211f5); }
            let Some(t73) = c.ld64(c.r64(13).wrapping_add(0x2490)) else { return RegfileExit::fault(0x70008211f6); };
            c.w64(3, t73);
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008211fd); };
            c.w64(6, t74);
            c.w32(0, 0x1u64);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x700082120e, 0x7000821209);
        }
        52 => {
            let Some(t75) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082120e); };
            c.w64(7, t75);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x7000821212); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r76 = c.r64(8) & c.r64(7);
            c.w64(8, r76);
            c.logic(r76, 64);
            let Some(t77) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000821223); };
            c.logic(t77 & 0x4u64, 8);
            if c.cc_ne() { blk = 73; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x700082122e); };
            c.w64(0, t78);
            blk = 55; continue;
        }
        55 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000821238); }
            return c.ret(0x0, 0x7000821239);
        }
        56 => {
            c.w32(7, c.r32(1));
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 58; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t79) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x7000821241); };
            c.w64(0, t79);
            blk = 55; continue;
        }
        58 => {
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082124a); };
            c.w64(3, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082124e); };
            c.w64(6, t81);
            blk = 65; continue;
        }
        59 => {
            let Some(t82) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x7000821254); };
            c.w64(0, t82);
            blk = 55; continue;
        }
        60 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a1af4, 0x7000821265, 0x7000821260);
        }
        61 => {
            let Some(t83) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821265); };
            c.w32(7, t83);
            let Some(t84) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000821268); };
            c.cmp(t84, c.r32(7), 32);
            if c.cc_ne() { blk = 64; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t85) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821271); };
            c.logic(t85 & 0x1u64, 8);
            if c.cc_e() { blk = 64; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t86) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000821277); };
            c.w64(0, t86);
            let Some(t87) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082127b); };
            c.w64(3, t87);
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082127f); };
            c.w64(1, t88);
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000821283); };
            c.w64(6, t89);
            blk = 65; continue;
        }
        64 => {
            let Some(t90) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000821289); };
            c.w64(5, t90);
            c.w64(3, c.r64(0));
            let Some(t91) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x7000821290); };
            c.w64(0, t91);
            let Some(t92) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000821295); };
            c.w64(1, t92);
            let Some(t93) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x700082129a); };
            c.w64(6, t93);
            let r94 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r94);
            return RegfileExit::jump(0x70008bb9f4);
        }
        65 => {
            let Some(t95) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008212a8); };
            c.w64(5, t95);
            let r96 = c.add(c.r64(4), 0x40u64, 64);
            c.w64(4, r96);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008212b0); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008212b1); }
            let Some(t97) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008212b2); };
            if !c.push64(t97) { return RegfileExit::fault(0x70008212b2); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(0)) { return RegfileExit::fault(0x70008212b6); }
            c.w32(0, 0x3u64);
            let Some(t98) = c.ld64(c.r64(13).wrapping_add(0x2378)) else { return RegfileExit::fault(0x70008212c0); };
            c.w64(3, t98);
            return RegfileExit::jump(0x700086ed74);
        }
        66 => {
            c.cmp(c.r16(8), 0x43u64, 16);
            if c.cc_ne() { blk = 65; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t99) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008212d3); };
            c.w32(8, t99);
            let r100 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r100);
            blk = 25; continue;
        }
        68 => {
            return RegfileExit::fault(0x70008212df);
        }
        69 => {
            return RegfileExit::fault(0x70008212e0);
        }
        70 => {
            return RegfileExit::fault(0x70008212e1);
        }
        71 => {
            return RegfileExit::fault(0x70008212e2);
        }
        72 => {
            return RegfileExit::fault(0x70008212e3);
        }
        73 => {
            c.w32(0, c.r32(0));
            let r101 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r101);
            c.w64(8, 0xfffffffffffc0000u64);
            let r102 = c.r64(8) & c.r64(0);
            c.w64(8, r102);
            c.logic(r102, 64);
            let Some(t103) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008212f3); };
            c.logic(t103 & 0x2u64, 8);
            if c.cc_e() { blk = 54; continue; }
            blk = 74; continue;
        }
        74 => {
            c.w64(9, c.r64(7).wrapping_add(0x3));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000821302); }
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x700082130c, 0x7000821307);
        }
        75 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700082130c); }
            blk = 54; continue;
        }
        76 => {
            return RegfileExit::jump(0x7000821314);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000821334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000821334); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000821338); }
            let r0 = c.sub(c.r64(4), 0x78u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x700082133e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000821342); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000821346); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 116; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821353); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082135a); };
            c.w32(9, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(8)) { return RegfileExit::fault(0x700082135f); }
            c.cmp(c.r16(9), 0xbdu64, 16);
            if c.cc_b() { blk = 117; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082136f); };
            c.w32(11, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000821373); };
            c.cmp(t5, c.r32(11), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x700082137c); };
            c.cmp(t6, c.r32(11), 32);
            if c.cc_ne() { blk = 126; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821389); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821390); };
            c.logic(t9 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 126; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(9, c.r16(9));
            c.cmp(c.r32(9), 0x411u64, 32);
            if c.cc_le() { blk = 126; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008213af); };
            c.w32(9, t10);
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 126; continue; }
            blk = 7; continue;
        }
        7 => {
            let r11 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r11);
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008213c4); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let r14 = c.r32(9) & 0x3ffu64;
            c.w32(9, r14);
            c.logic(r14, 32);
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w32(15, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008213de); }
            c.w64(2, c.r64(11));
            c.w32(11, 0x1u64);
            c.w64(12, c.r64(11));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(15)) { return RegfileExit::fault(0x70008213ee); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(15)) { return RegfileExit::fault(0x70008213f2); }
            blk = 9; continue;
        }
        8 => {
            c.w64(7, c.r64(9).wrapping_add(0x3));
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(8)) { return RegfileExit::fault(0x70008213fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), 0x1u64) { return RegfileExit::fault(0x7000821400); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(7)) { return RegfileExit::fault(0x7000821408); }
            blk = 9; continue;
        }
        9 => {
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x700082140c); };
            c.cmp(c.r64(1), t15, 64);
            if c.cc_e() { blk = 113; continue; }
            blk = 10; continue;
        }
        10 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000821416); }
            c.w64(8, c.r64(2));
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x700082141d); }
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000821421); };
            c.w64(10, t16);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(10)) { return RegfileExit::fault(0x7000821425); }
            blk = 12; continue;
        }
        11 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082142b); };
            c.w64(8, t17);
            blk = 12; continue;
        }
        12 => {
            let Some(t18) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082142f); };
            c.w32(0, t18);
            let r19 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r19);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(8)) { return RegfileExit::fault(0x7000821437); }
            let Some(t20) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082143b); };
            c.w32(8, t20);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(0)) { return RegfileExit::fault(0x700082143f); }
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000821443); };
            c.cmp(t21, c.r32(8), 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t22) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082144c); };
            c.cmp(t22, 0x0u64, 32);
            if c.cc_ne() { blk = 18; continue; }
            blk = 14; continue;
        }
        14 => {
            let Some(t23) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000821452); };
            c.cmp(t23, 0x0u64, 32);
            if c.cc_ne() { blk = 16; continue; }
            blk = 15; continue;
        }
        15 => {
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x1u64) { return RegfileExit::fault(0x7000821458); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x7000821460); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(1)) { return RegfileExit::fault(0x7000821464); }
            blk = 111; continue;
        }
        16 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(1)) { return RegfileExit::fault(0x700082146d); }
            blk = 111; continue;
        }
        17 => {
            let Some(t24) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000821476); };
            c.cmp(t24, 0x0u64, 32);
            if c.cc_ne() { blk = 111; continue; }
            blk = 18; continue;
        }
        18 => {
            let Some(t25) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000821480); };
            c.cmp(t25, 0x0u64, 32);
            if c.cc_ne() { blk = 53; continue; }
            blk = 19; continue;
        }
        19 => {
            let Some(t26) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082148a); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821491); };
            c.w32(9, t28);
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 42; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t29) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008214a2); };
            c.w32(11, t29);
            let r30 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r30);
            c.logic(c.r32(9) & 0xffc00u64, 32);
            if c.cc_e() { blk = 111; continue; }
            blk = 21; continue;
        }
        21 => {
            let r31 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r31);
            let r32 = c.r32(9) & 0x3ffu64;
            c.w32(9, r32);
            c.logic(r32, 32);
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_be() { blk = 37; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t33) = c.ld16(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008214cb); };
            c.w32(12, (t33 as u16 as i16 as i64 as u64));
            let Some(t34) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008214d0); };
            c.w32(2, t34);
            let r35 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r35);
            let r36 = c.sub(c.r32(12), 0x1u64, 32);
            c.w32(12, r36);
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008214da); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(12)) { return RegfileExit::fault(0x70008214de); }
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 24; continue;
        }
        23 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(1)) { return RegfileExit::fault(0x70008214e7); }
            blk = 24; continue;
        }
        24 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008214eb); };
            c.w64(1, t37);
            let r38 = c.sub(c.r32(1), c.r32(15), 32);
            c.w32(1, r38);
            let r39 = c.shr(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r39);
            let r40 = c.add(c.r32(1), c.r32(15), 32);
            c.w32(1, r40);
            c.w32(6, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008214fd); };
            c.w32(6, t41);
            let r42 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r42);
            let r43 = c.shr(c.r32(6), 0x9u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r43);
            let r44 = c.r32(6) & 0x3ffu64;
            c.w32(6, r44);
            c.logic(r44, 32);
            c.w32(6, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821513); };
            c.w32(6, t45);
            let r46 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r46);
            let Some(t47) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082151b); };
            c.w32(6, t47);
            let r48 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r48);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_be() { blk = 26; continue; }
            blk = 25; continue;
        }
        25 => {
            c.w32(15, c.r64(1).wrapping_add(0x1));
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000821529); };
            c.w64(1, t49);
            blk = 26; continue;
        }
        26 => {
            c.cmp(c.r32(15), c.r32(1), 32);
            if c.cc_ne() { blk = 23; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 29; continue;
        }
        28 => {
            let r50 = c.add(c.r32(15), 0x1u64, 32);
            c.w32(15, r50);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_l() { blk = 36; continue; }
            blk = 30; continue;
        }
        30 => {
            c.w32(1, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t51) = c.ld32(c.r64(11).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000821544); };
            c.w32(1, t51);
            let r52 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r52);
            let r53 = c.shr(c.r32(1), 0x9u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r53);
            let r54 = c.r32(1) & 0x3ffu64;
            c.w32(1, r54);
            c.logic(r54, 32);
            c.w32(6, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082155a); };
            c.w32(7, t55);
            let r56 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r56);
            let Some(t57) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821562); };
            c.w32(3, t57);
            let r58 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r58);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_ne() { blk = 35; continue; }
            blk = 31; continue;
        }
        31 => {
            c.cmp(c.r32(7), c.r32(0), 32);
            if c.cc_ne() { blk = 28; continue; }
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 34; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t59) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821575); };
            c.w64(1, t59);
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(6));
            let Some(t60) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082157f); };
            c.w64(3, t60);
            blk = 54; continue;
        }
        34 => {
            let Some(t61) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821588); };
            c.w64(1, t61);
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082158c); };
            c.w64(3, t62);
            blk = 111; continue;
        }
        35 => {
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821595); };
            c.w64(1, t63);
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821599); };
            c.w64(3, t64);
            blk = 111; continue;
        }
        36 => {
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008215a2); };
            c.w64(1, t65);
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008215a6); };
            c.w64(3, t66);
            blk = 111; continue;
        }
        37 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r67 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(6, r67);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 38; continue;
        }
        38 => {
            let r68 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r68);
            let Some(t69) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008215c4); };
            c.cmp(t69, c.r32(0), 32);
            if c.cc_e() { blk = 41; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 40; continue;
        }
        40 => {
            blk = 111; continue;
        }
        41 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x70008215d6); }
            c.w64(9, c.r64(11));
            c.w64(11, c.r64(6));
            blk = 54; continue;
        }
        42 => {
            c.w64(8, c.r64(3));
            let Some(t70) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008215e8); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r71);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 44; continue; }
            blk = 43; continue;
        }
        43 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008215f5); };
            c.w64(9, t72);
            blk = 44; continue;
        }
        44 => {
            let Some(t73) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008215fc); };
            c.w32(11, t73);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r74 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r74);
            let Some(t75) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082160a); };
            c.w32(12, t75);
            let r76 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r76);
            let r77 = c.r64(12) & c.r64(11);
            c.w64(12, r77);
            c.logic(r77, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 46; continue;
        }
        45 => {
            let r78 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r78);
            let r79 = c.add(c.r64(12), c.r64(15), 64);
            c.w64(12, r79);
            c.w64(3, c.r64(11));
            let r80 = c.r64(3) & c.r64(12);
            c.w64(3, r80);
            c.logic(r80, 64);
            c.w64(12, c.r64(3));
            blk = 46; continue;
        }
        46 => {
            c.w64(3, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            let Some(t81) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821637); };
            c.w32(6, t81);
            let Some(t82) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082163c); };
            c.cmp(t82, c.r32(6), 32);
            if c.cc_e() { blk = 52; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t83) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821649); };
            c.w32(11, t83);
            c.logic(c.r8(11) & 0x20u64, 8);
            if c.cc_ne() { blk = 51; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t84) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821654); };
            c.w32(12, t84);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000821662); }
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_ae() { blk = 125; continue; }
            blk = 50; continue;
        }
        50 => {
            let Some(t85) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082166f); };
            c.w32(9, t85);
            let r86 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r86);
            let r87 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r87);
            c.w64(2, c.r64(11));
            blk = 69; continue;
        }
        51 => {
            c.w64(3, c.r64(8));
            blk = 111; continue;
        }
        52 => {
            c.w64(3, c.r64(8));
            blk = 111; continue;
        }
        53 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(1)) { return RegfileExit::fault(0x7000821692); }
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000821696); };
            c.w64(8, t88);
            c.w64(11, c.r64(1));
            let Some(t89) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082169d); };
            c.w64(9, t89);
            blk = 54; continue;
        }
        54 => {
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t90) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008216a5); };
            c.w32(12, t90);
            c.w32(15, c.r32(12));
            c.logic(c.r8(12) & 0x20u64, 8);
            if c.cc_ne() { blk = 111; continue; }
            blk = 55; continue;
        }
        55 => {
            c.w64(2, c.r64(12));
            let r91 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r91);
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            let Some(t92) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008216c2); };
            c.w32(9, t92);
            let r93 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r93);
            blk = 68; continue;
        }
        57 => {
            c.w64(9, c.r64(2));
            if let Some(sr) = c.shr(c.r64(9), 0x13u64, 64) { c.w64(9, sr); }
            let Some(t94) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008216d6); };
            c.w32(11, t94);
            let r95 = c.r64(9) & 0x3ffu64;
            c.w64(9, r95);
            c.logic(r95, 64);
            let r96 = c.r32(15) & 0x380u64;
            c.w32(15, r96);
            c.logic(r96, 32);
            let r97 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r97);
            c.cmp(c.r32(15), 0x100u64, 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t98) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008216fb); };
            c.w32(8, t98);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 62; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t99) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821705); };
            c.w32(12, t99);
            let r100 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r100);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t101) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000821712); };
            c.w64(12, t101);
            blk = 60; continue;
        }
        60 => {
            let r102 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r102);
            let Some(t103) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082171c); };
            c.w32(9, t103);
            let r104 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r104);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 68; continue; }
            blk = 61; continue;
        }
        61 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x700082172a); }
            blk = 64; continue;
        }
        62 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 67; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t105) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821738); };
            c.w32(8, t105);
            let r106 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r106);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000821740); }
            blk = 64; continue;
        }
        64 => {
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000821746); };
            c.w64(8, t107);
            let Some(t108) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082174d); };
            c.w64(9, t108);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t109) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000821754); };
            c.w64(12, t109);
            let Some(t110) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082175b); };
            c.cmp(t110, c.r64(11), 64);
            if c.cc_be() { blk = 121; continue; }
            blk = 65; continue;
        }
        65 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000821765); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 66; continue;
        }
        66 => {
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082176c); };
            c.w64(9, t111);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000821773); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000821777); }
            c.w64(9, c.r64(8));
            blk = 68; continue;
        }
        67 => {
            let Some(t112) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821782); };
            c.w32(9, t112);
            let r113 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r113);
            blk = 68; continue;
        }
        68 => {
            c.w64(8, c.r64(3));
            blk = 69; continue;
        }
        69 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t114) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821796); };
            c.w32(11, t114);
            let Some(t115) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x700082179a); };
            c.cmp(t115, c.r32(11), 32);
            if c.cc_ne() { blk = 89; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t116) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008217a7); };
            c.w32(2, t116);
            let r117 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r117);
            let Some(t118) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008217ae); };
            c.w32(9, t118);
            let r119 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r119);
            let Some(t120) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008217b5); };
            c.logic(t120 & 0x2u64, 8);
            if c.cc_ne() { blk = 87; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008217c0); };
            c.cmp(t121, c.r32(9), 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008217c9); };
            c.w64(9, t122);
            blk = 107; continue;
        }
        74 => {
            let Some(t123) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008217d5); };
            c.w64(9, t123);
            let Some(t124) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x70008217dc); };
            c.cmp(t124, 0x20u64, 8);
            if c.cc_e() { blk = 123; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t125) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008217e6); };
            c.w32(9, t125);
            let Some(t126) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008217ea); };
            c.cmp(t126, c.r32(9), 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t127) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008217f7); };
            c.w32(9, t127);
            let r128 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r128);
            let Some(t129) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008217fe); };
            c.w32(11, t129);
            let Some(t130) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000821802); };
            c.cmp(t130, c.r32(11), 32);
            if c.cc_ne() { blk = 78; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t131) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082180b); };
            c.w32(9, t131);
            let r132 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r132);
            let Some(t133) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821812); };
            c.w32(11, t133);
            let Some(t134) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000821816); };
            c.cmp(t134, c.r32(11), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 78; continue;
        }
        78 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t135) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821829); };
            c.w32(11, t135);
            let r136 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r136);
            let Some(t137) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821830); };
            c.w32(12, t137);
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 84; continue; }
            blk = 80; continue;
        }
        80 => {
            c.w32(9, c.r16(12));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 83; continue; }
            blk = 81; continue;
        }
        81 => {
            let Some(t138) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000821850); };
            c.cmp(t138, c.r32(11), 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 82; continue;
        }
        82 => {
            c.w64(9, c.r64(8));
            blk = 84; continue;
        }
        83 => {
            let Some(t139) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821862); };
            c.w32(9, t139);
            let r140 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r140);
            blk = 84; continue;
        }
        84 => {
            let Some(t141) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821869); };
            c.w32(9, t141);
            let r142 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r142);
            let Some(t143) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821870); };
            c.w32(9, t143);
            let r144 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r144);
            let Some(t145) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821877); };
            c.w32(11, t145);
            let r146 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r146);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 104; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t147) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082188d); };
            c.w32(6, t147);
            let r148 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r148);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000821894); }
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(9, c.r64(1));
            return c.call(0x70007f6fb4, 0x70008218a0, 0x700082189b);
        }
        86 => {
            c.w64(9, c.r64(0));
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008218a3); };
            c.w64(8, t149);
            let Some(t150) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008218a7); };
            c.w64(1, t150);
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008218ab); };
            c.w64(0, t151);
            blk = 107; continue;
        }
        87 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008218b4); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008218bb); };
            c.w64(6, t152);
            return c.call(0x70007f2274, 0x70008218c4, 0x70008218bf);
        }
        88 => {
            c.w64(9, c.r64(0));
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008218c7); };
            c.w64(8, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008218cb); };
            c.w64(1, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008218cf); };
            c.w64(0, t155);
            blk = 107; continue;
        }
        89 => {
            let Some(t156) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008218d8); };
            c.w32(11, t156);
            let r157 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r157);
            let Some(t158) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008218df); };
            c.w32(12, t158);
            c.cmp(c.r16(12), 0x834u64, 16);
            if c.cc_e() { blk = 103; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w32(15, c.r64(12).wrapping_sub(0x80c));
            c.cmp(c.r32(15), 0xeu64, 32);
            if c.cc_be() { blk = 96; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r16(12), 0x411u64, 16);
            if c.cc_ne() { blk = 104; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t159) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082190a); };
            c.w32(9, t159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x700082190e); };
            c.cmp(t160, c.r32(9), 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t161) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082191b); };
            c.w32(9, t161);
            let r162 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r162);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 104; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t163) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082192c); };
            c.w32(11, t163);
            let r164 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r164);
            let Some(t165) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821933); };
            c.cmp(t165, 0x40u64, 16);
            if c.cc_ae() { blk = 104; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t166) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082193f); };
            c.w32(9, t166);
            let r167 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r167);
            blk = 107; continue;
        }
        96 => {
            let Some(t168) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082194b); };
            c.w32(9, t168);
            let Some(t169) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700082194f); };
            c.cmp(t169, c.r32(9), 32);
            if c.cc_ne() { blk = 104; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t170) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082195c); };
            c.w32(9, t170);
            c.w64(11, c.r64(9));
            let r171 = c.r32(11) & 0xc0u64;
            c.w32(11, r171);
            c.logic(r171, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 99; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t172) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821974); };
            c.w32(11, t172);
            let r173 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r173);
            let Some(t174) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x700082197b); };
            c.w32(11, t174);
            let r175 = c.r32(11) & 0x1fu64;
            c.w32(11, r175);
            c.logic(r175, 32);
            let r176 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r176);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 104; continue; }
            blk = 99; continue;
        }
        99 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 104; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t177) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000821993); };
            c.w32(9, t177);
            let r178 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r178);
            let Some(t179) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082199a); };
            c.cmp(t179, c.r32(9), 32);
            if c.cc_e() { blk = 104; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t180) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008219a3); };
            c.w32(11, t180);
            let Some(t181) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008219a7); };
            c.cmp(t181, c.r32(11), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t182) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008219b0); };
            c.w32(9, t182);
            let r183 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r183);
            blk = 107; continue;
        }
        103 => {
            let Some(t184) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008219b9); };
            c.w32(9, t184);
            let Some(t185) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008219bd); };
            c.cmp(t185, c.r32(9), 32);
            if c.cc_e() { blk = 106; continue; }
            blk = 104; continue;
        }
        104 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008219c6); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008219c8); }
            let Some(t186) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008219c9); };
            c.w64(3, t186);
            let Some(t187) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008219d0); };
            c.w64(6, t187);
            c.w32(0, 0x2u64);
            c.w64(9, c.r64(0));
            return c.call(0x700086ed74, 0x70008219e1, 0x70008219dc);
        }
        105 => {
            c.w64(9, c.r64(0));
            let Some(t188) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008219e4); };
            c.w64(0, t188);
            blk = 107; continue;
        }
        106 => {
            let Some(t189) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008219ea); };
            c.w32(9, t189);
            let r190 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r190);
            blk = 107; continue;
        }
        107 => {
            c.w64(3, c.r64(0));
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008219f4); };
            c.w64(0, t191);
            c.w64(1, c.r64(9));
            let Some(t192) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008219fb); };
            c.w64(6, t192);
            return c.call(0x7000875c54, 0x7000821a04, 0x70008219ff);
        }
        108 => {
            let Some(t193) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000821a04); };
            c.cmp(t193, 0x0u64, 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821a0a); };
            c.w64(1, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821a0e); };
            c.w64(3, t195);
            blk = 111; continue;
        }
        110 => {
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821a14); };
            c.w64(8, t196);
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x7000821a18); };
            c.w64(9, t197);
            let Some(t198) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821a1c); };
            c.cmp(t198, c.r32(9), 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t199) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000821a28); };
            c.w32(12, t199);
            let r200 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821a2f); };
            c.w64(1, t201);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000821a33); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x7000821a37); }
            c.w64(3, c.r64(8));
            blk = 111; continue;
        }
        111 => {
            let r202 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r202);
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000821a42); };
            c.cmp(c.r64(1), t203, 64);
            if c.cc_ne() { blk = 11; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000821a4c); };
            c.w64(11, t204);
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000821a50); };
            c.w64(2, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000821a54); };
            c.w64(1, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000821a58); };
            c.w64(9, t207);
            blk = 114; continue;
        }
        113 => {
            let Some(t208) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000821a5e); };
            c.w64(9, t208);
            blk = 114; continue;
        }
        114 => {
            let Some(t209) = c.ld32(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000821a62); };
            c.cmp(t209, 0x0u64, 32);
            if c.cc_ne() { blk = 116; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000821a68); };
            c.w64(8, t210);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 8; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000821a76); };
            c.w64(0, t211);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000821a80); }
            return c.ret(0x0, 0x7000821a81);
        }
        117 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 126; continue; }
            blk = 118; continue;
        }
        118 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 116; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t212) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821a91); };
            c.w32(8, t212);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 116; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 126; continue;
        }
        121 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(2)) { return RegfileExit::fault(0x7000821a9d); }
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 0) { return RegfileExit::fault(0x7000821aa6); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000821ab3, 0x7000821aae);
        }
        122 => {
            c.w64(8, c.r64(0));
            let Some(t213) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821ab6); };
            c.w64(3, t213);
            let Some(t214) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821aba); };
            c.w64(1, t214);
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000821abe); };
            c.w64(0, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000821ac2); };
            c.w64(2, t216);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x7000821ac6); }
            blk = 66; continue;
        }
        123 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000821ad0); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000821ad2); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000821ad3); }
            let Some(t217) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000821ad5); };
            c.w64(3, t217);
            c.w32(0, 0x3u64);
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000821ae1); };
            c.w64(6, t218);
            return c.call(0x700086ed74, 0x7000821aea, 0x7000821ae5);
        }
        124 => {
            c.w64(9, c.r64(0));
            let Some(t219) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821aed); };
            c.w64(8, t219);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821af1); };
            c.w64(1, t220);
            let Some(t221) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x7000821af5); };
            c.w64(0, t221);
            blk = 107; continue;
        }
        125 => {
            return RegfileExit::fault(0x7000821afe);
        }
        126 => {
            let Some(t222) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000821aff); };
            c.w64(5, t222);
            let r223 = c.add(c.r64(4), 0x88u64, 64);
            c.w64(4, r223);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000821b0a); }
            let Some(t224) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000821b0b); };
            if !c.push64(t224) { return RegfileExit::fault(0x7000821b0b); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x7000821b0f); }
            c.w32(0, 0x2u64);
            let Some(t225) = c.ld64(c.r64(13).wrapping_add(0x2330)) else { return RegfileExit::fault(0x7000821b19); };
            c.w64(3, t225);
            return RegfileExit::jump(0x700086ed74);
        }
        127 => {
            return RegfileExit::jump(0x7000821b28);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000821b34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000821b34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000821b38); }
            let r0 = c.sub(c.r64(4), 0xb0u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x7000821b41); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000821b45); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x7000821b49); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 535; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821b56); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821b5d); };
            c.w32(9, t3);
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(8)) { return RegfileExit::fault(0x7000821b62); }
            c.cmp(c.r16(9), 0xbdu64, 16);
            if c.cc_b() { blk = 536; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821b72); };
            c.w32(11, t4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000821b76); };
            c.cmp(t5, c.r32(11), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x3f8)) else { return RegfileExit::fault(0x7000821b7f); };
            c.cmp(t6, c.r32(11), 32);
            if c.cc_ne() { blk = 643; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821b8c); };
            c.w32(11, t7);
            let r8 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r8);
            let Some(t9) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821b93); };
            c.logic(t9 & 0x1000000u64, 32);
            if c.cc_ne() { blk = 643; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t10) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821ba1); };
            c.cmp(t10, 0x410u64, 16);
            if c.cc_be() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t11) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000821baa); };
            c.logic(t11 & 0x24u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w64(12, c.r64(11));
            blk = 8; continue;
        }
        7 => {
            c.w32(12, 0);
            c.logic(0, 32);
            blk = 8; continue;
        }
        8 => {
            c.w32(9, c.r16(9));
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0xb8), c.r64(12)) { return RegfileExit::fault(0x7000821bc6); }
            c.cmp(c.r32(9), 0x411u64, 32);
            if c.cc_le() { blk = 643; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t12) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821bda); };
            c.w32(9, t12);
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 643; continue; }
            blk = 10; continue;
        }
        10 => {
            let r13 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000821bef); };
            c.w32(15, t14);
            let r15 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r15);
            let r16 = c.r32(9) & 0x3ffu64;
            c.w32(9, r16);
            c.logic(r16, 32);
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            c.w32(2, 0x1u64);
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x7000821c09); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x7000821c0d); }
            c.w32(7, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(15));
            c.w64(15, c.r64(2));
            blk = 12; continue;
        }
        11 => {
            let r17 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r17);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x7000821c1f); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), 0x1u64) { return RegfileExit::fault(0x7000821c23); }
            blk = 12; continue;
        }
        12 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(9)) { return RegfileExit::fault(0x7000821c2b); }
            let Some(t18) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821c2f); };
            c.cmp(c.r64(7), t18, 64);
            if c.cc_e() { blk = 514; continue; }
            blk = 13; continue;
        }
        13 => {
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x7000821c39); }
            c.w64(8, c.r64(1));
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000821c40); }
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x7000821c44); };
            c.w64(10, t19);
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(10)) { return RegfileExit::fault(0x7000821c48); }
            blk = 15; continue;
        }
        14 => {
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000821c4e); };
            c.w64(8, t20);
            blk = 15; continue;
        }
        15 => {
            let Some(t21) = c.ld32(c.r64(8).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821c52); };
            c.w32(12, t21);
            let r22 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r22);
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(8)) { return RegfileExit::fault(0x7000821c5a); }
            let Some(t23) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821c5e); };
            c.w32(8, t23);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x7000821c63); }
            let Some(t24) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000821c67); };
            c.cmp(t24, c.r32(8), 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t25) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000821c70); };
            c.cmp(t25, 0x0u64, 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t26) = c.ld32(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x7000821c76); };
            c.cmp(t26, 0x0u64, 32);
            if c.cc_ne() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            if !c.st64(c.r64(5).wrapping_sub(0x48), 0x1u64) { return RegfileExit::fault(0x7000821c7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000821c84); }
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(7)) { return RegfileExit::fault(0x7000821c88); }
            blk = 512; continue;
        }
        19 => {
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(7)) { return RegfileExit::fault(0x7000821c91); }
            blk = 512; continue;
        }
        20 => {
            let Some(t27) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x7000821c9a); };
            c.cmp(t27, 0x0u64, 32);
            if c.cc_ne() { blk = 512; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t28) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000821ca4); };
            c.cmp(t28, 0x0u64, 32);
            if c.cc_ne() { blk = 56; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t29) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821cae); };
            c.w32(8, t29);
            let r30 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r30);
            let Some(t31) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821cb5); };
            c.w32(9, t31);
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 45; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000821cc6); };
            c.w32(15, t32);
            let r33 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r33);
            c.logic(c.r32(9) & 0xffc00u64, 32);
            if c.cc_e() { blk = 512; continue; }
            blk = 24; continue;
        }
        24 => {
            let r34 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r34);
            let r35 = c.r32(9) & 0x3ffu64;
            c.w32(9, r35);
            c.logic(r35, 32);
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_be() { blk = 40; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t36) = c.ld16(c.r64(15).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000821cef); };
            c.w32(0, (t36 as u16 as i16 as i64 as u64));
            let Some(t37) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821cf4); };
            c.w32(2, t37);
            let r38 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r38);
            let r39 = c.sub(c.r32(0), 0x1u64, 32);
            c.w32(0, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x7000821cff); }
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000821d08); }
            blk = 27; continue;
        }
        27 => {
            let Some(t40) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000821d0c); };
            c.w64(1, t40);
            let r41 = c.sub(c.r32(1), c.r32(11), 32);
            c.w32(1, r41);
            let r42 = c.shr(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r42);
            let r43 = c.add(c.r32(1), c.r32(11), 32);
            c.w32(1, r43);
            c.w32(6, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t44) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000821d1e); };
            c.w32(6, t44);
            let r45 = c.sar(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r45);
            let r46 = c.shr(c.r32(6), 0x9u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r46);
            let r47 = c.r32(6) & 0x3ffu64;
            c.w32(6, r47);
            c.logic(r47, 32);
            c.w32(6, c.r64(6).wrapping_add(c.r64(6).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t48) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821d34); };
            c.w32(6, t48);
            let r49 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r49);
            let Some(t50) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821d3c); };
            c.w32(6, t50);
            let r51 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r51);
            c.cmp(c.r32(2), c.r32(6), 32);
            if c.cc_be() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(11, c.r64(1).wrapping_add(0x1));
            let Some(t52) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000821d4a); };
            c.w64(1, t52);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(11), c.r32(1), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000821d53); }
            blk = 32; continue;
        }
        31 => {
            let r53 = c.add(c.r32(11), 0x1u64, 32);
            c.w32(11, r53);
            blk = 32; continue;
        }
        32 => {
            c.cmp(c.r32(0), c.r32(11), 32);
            if c.cc_l() { blk = 39; continue; }
            blk = 33; continue;
        }
        33 => {
            c.w32(1, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t54) = c.ld32(c.r64(15).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000821d69); };
            c.w32(1, t54);
            let r55 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r55);
            let r56 = c.shr(c.r32(1), 0x9u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r56);
            let r57 = c.r32(1) & 0x3ffu64;
            c.w32(1, r57);
            c.logic(r57, 32);
            c.w32(6, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t58) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821d7f); };
            c.w32(7, t58);
            let r59 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821d87); };
            c.w32(3, t60);
            let r61 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r61);
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_ne() { blk = 38; continue; }
            blk = 34; continue;
        }
        34 => {
            c.cmp(c.r32(7), c.r32(12), 32);
            if c.cc_ne() { blk = 31; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r32(9), c.r32(1), 32);
            if c.cc_be() { blk = 37; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t62) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000821d9b); };
            c.w64(7, t62);
            c.w64(11, c.r64(6));
            c.w64(9, c.r64(15));
            let Some(t63) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821da5); };
            c.w64(3, t63);
            blk = 57; continue;
        }
        37 => {
            let Some(t64) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000821dae); };
            c.w64(7, t64);
            let Some(t65) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821db2); };
            c.w64(3, t65);
            blk = 512; continue;
        }
        38 => {
            let Some(t66) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000821dbb); };
            c.w64(7, t66);
            let Some(t67) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821dbf); };
            c.w64(3, t67);
            blk = 512; continue;
        }
        39 => {
            let Some(t68) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000821dc8); };
            c.w64(7, t68);
            let Some(t69) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000821dcc); };
            c.w64(3, t69);
            blk = 512; continue;
        }
        40 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r70 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(6, r70);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 512; continue; }
            blk = 41; continue;
        }
        41 => {
            let r71 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r71);
            let Some(t72) = c.ld32(c.r64(15).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821dea); };
            c.cmp(t72, c.r32(12), 32);
            if c.cc_e() { blk = 44; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 41; continue; }
            blk = 43; continue;
        }
        43 => {
            blk = 512; continue;
        }
        44 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000821dfc); }
            c.w64(11, c.r64(6));
            c.w64(9, c.r64(15));
            blk = 57; continue;
        }
        45 => {
            c.w64(8, c.r64(3));
            let Some(t73) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821e0e); };
            c.w32(9, t73);
            let r74 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r74);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 47; continue; }
            blk = 46; continue;
        }
        46 => {
            let Some(t75) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000821e1b); };
            c.w64(9, t75);
            blk = 47; continue;
        }
        47 => {
            let Some(t76) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821e22); };
            c.w32(11, t76);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r77 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r77);
            let Some(t78) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821e30); };
            c.w32(15, t78);
            let r79 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r79);
            let r80 = c.r64(15) & c.r64(11);
            c.w64(15, r80);
            c.logic(r80, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 49; continue;
        }
        48 => {
            let r81 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r81);
            let r82 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r82);
            c.w64(3, c.r64(11));
            let r83 = c.r64(3) & c.r64(15);
            c.w64(3, r83);
            c.logic(r83, 64);
            c.w64(15, c.r64(3));
            blk = 49; continue;
        }
        49 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            let Some(t84) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821e5d); };
            c.w32(1, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000821e62); };
            c.cmp(t85, c.r32(1), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r32(1), c.r32(12), 32);
            if c.cc_ne() { blk = 48; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t86) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000821e70); };
            c.w32(11, t86);
            c.logic(c.r8(11) & 0x20u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t87) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821e7b); };
            c.w32(15, t87);
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000821e89); }
            c.cmp(c.r64(0), c.r64(15), 64);
            if c.cc_ae() { blk = 630; continue; }
            blk = 53; continue;
        }
        53 => {
            let r88 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r88);
            let Some(t89) = c.ld32(c.r64(9).wrapping_add(c.r64(2)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000821e99); };
            c.w32(9, t89);
            let r90 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r90);
            c.w64(2, c.r64(11));
            blk = 72; continue;
        }
        54 => {
            c.w64(3, c.r64(8));
            blk = 512; continue;
        }
        55 => {
            c.w64(3, c.r64(8));
            blk = 512; continue;
        }
        56 => {
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(7)) { return RegfileExit::fault(0x7000821eb9); }
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x7000821ebd); };
            c.w64(8, t91);
            c.w64(11, c.r64(7));
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x7000821ec4); };
            c.w64(9, t92);
            blk = 57; continue;
        }
        57 => {
            if let Some(sr) = c.shl(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let Some(t93) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000821ecc); };
            c.w32(15, t93);
            c.w32(0, c.r32(15));
            c.logic(c.r8(15) & 0x20u64, 8);
            if c.cc_ne() { blk = 512; continue; }
            blk = 58; continue;
        }
        58 => {
            c.w64(2, c.r64(15));
            let r94 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r94);
            c.logic(c.r8(15) & 0x4u64, 8);
            if c.cc_e() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t95) = c.ld32(c.r64(9).wrapping_add(c.r64(11)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000821ee9); };
            c.w32(9, t95);
            let r96 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r96);
            blk = 71; continue;
        }
        60 => {
            c.w64(9, c.r64(2));
            if let Some(sr) = c.shr(c.r64(9), 0x13u64, 64) { c.w64(9, sr); }
            let Some(t97) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000821efd); };
            c.w32(11, t97);
            let Some(t98) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821f02); };
            c.w32(8, t98);
            let r99 = c.r32(0) & 0x380u64;
            c.w32(0, r99);
            c.logic(r99, 32);
            let r100 = c.r64(9) & 0x3ffu64;
            c.w64(9, r100);
            c.logic(r100, 64);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(15, (c.cc_ne() as u64));
            c.w32(15, c.r8(15));
            let r101 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r101);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_b() { blk = 65; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t102) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821f28); };
            c.w32(11, t102);
            let r103 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r103);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000821f35); };
            c.w64(11, t104);
            blk = 63; continue;
        }
        63 => {
            let r105 = c.sub(c.r64(9), c.r64(8), 64);
            c.w64(9, r105);
            let Some(t106) = c.ld32(c.r64(11).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000821f3f); };
            c.w32(9, t106);
            let r107 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r107);
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 71; continue; }
            blk = 64; continue;
        }
        64 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x7000821f4d); }
            blk = 67; continue;
        }
        65 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 70; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t108) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821f5b); };
            c.w32(8, t108);
            let r109 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r109);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000821f63); }
            blk = 67; continue;
        }
        67 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000821f69); };
            c.w64(8, t110);
            let Some(t111) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000821f70); };
            c.w64(9, t111);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t112) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000821f77); };
            c.w64(15, t112);
            let Some(t113) = c.ld64(c.r64(15)) else { return RegfileExit::fault(0x7000821f7e); };
            c.cmp(t113, c.r64(11), 64);
            if c.cc_be() { blk = 540; continue; }
            blk = 68; continue;
        }
        68 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000821f87); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 69; continue;
        }
        69 => {
            let Some(t114) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000821f8e); };
            c.w64(9, t114);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000821f95); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000821f99); }
            c.w64(9, c.r64(8));
            blk = 71; continue;
        }
        70 => {
            let Some(t115) = c.ld32(c.r64(3).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821fa4); };
            c.w32(9, t115);
            let r116 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r116);
            blk = 71; continue;
        }
        71 => {
            c.w64(8, c.r64(3));
            blk = 72; continue;
        }
        72 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 110; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t117) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821fb8); };
            c.w32(11, t117);
            let Some(t118) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x7000821fbc); };
            c.cmp(t118, c.r32(11), 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 74; continue;
        }
        74 => {
            let Some(t119) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000821fc9); };
            c.w32(2, t119);
            let r120 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r120);
            let Some(t121) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000821fd0); };
            c.w32(9, t121);
            let r122 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r122);
            let Some(t123) = c.ld8(c.r64(9).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000821fd7); };
            c.logic(t123 & 0x2u64, 8);
            if c.cc_ne() { blk = 90; continue; }
            blk = 75; continue;
        }
        75 => {
            let Some(t124) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000821fe2); };
            c.cmp(t124, c.r32(9), 32);
            if c.cc_e() { blk = 77; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t125) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000821feb); };
            c.w64(9, t125);
            blk = 110; continue;
        }
        77 => {
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x7000821ff7); };
            c.w64(9, t126);
            let Some(t127) = c.ld8(c.r64(9)) else { return RegfileExit::fault(0x7000821ffe); };
            c.cmp(t127, 0x20u64, 8);
            if c.cc_e() { blk = 542; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t128) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x7000822008); };
            c.w32(9, t128);
            let Some(t129) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082200c); };
            c.cmp(t129, c.r32(9), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t130) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822019); };
            c.w32(9, t130);
            let r131 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r131);
            let Some(t132) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822020); };
            c.w32(11, t132);
            let Some(t133) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000822024); };
            c.cmp(t133, c.r32(11), 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t134) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082202d); };
            c.w32(9, t134);
            let r135 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r135);
            let Some(t136) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822034); };
            c.w32(11, t136);
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x7000822038); };
            c.cmp(t137, c.r32(11), 32);
            if c.cc_e() { blk = 80; continue; }
            blk = 81; continue;
        }
        81 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 82; continue;
        }
        82 => {
            let Some(t138) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082204b); };
            c.w32(11, t138);
            let r139 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r139);
            let Some(t140) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000822052); };
            c.w32(15, t140);
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 87; continue; }
            blk = 83; continue;
        }
        83 => {
            c.w32(9, c.r16(15));
            c.cmp(c.r32(9), 0x82au64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t141) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x7000822070); };
            c.cmp(t141, c.r32(11), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 85; continue;
        }
        85 => {
            c.w64(9, c.r64(8));
            blk = 87; continue;
        }
        86 => {
            let Some(t142) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000822082); };
            c.w32(9, t142);
            let r143 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r143);
            blk = 87; continue;
        }
        87 => {
            let Some(t144) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000822089); };
            c.w32(9, t144);
            let r145 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r145);
            let Some(t146) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822090); };
            c.w32(9, t146);
            let r147 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r147);
            let Some(t148) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000822097); };
            c.w32(11, t148);
            let r149 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r149);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 107; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t150) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008220ad); };
            c.w32(6, t150);
            let r151 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r151);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008220b4); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008220bd, 0x70008220b8);
        }
        89 => {
            c.w64(9, c.r64(0));
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008220c0); };
            c.w64(8, t152);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008220c4); };
            c.w64(12, t153);
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008220c8); };
            c.w64(7, t154);
            blk = 110; continue;
        }
        90 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008220d1); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008220d8); };
            c.w64(6, t155);
            return c.call(0x70007f2274, 0x70008220e1, 0x70008220dc);
        }
        91 => {
            c.w64(9, c.r64(0));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008220e4); };
            c.w64(8, t156);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008220e8); };
            c.w64(12, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008220ec); };
            c.w64(7, t158);
            blk = 110; continue;
        }
        92 => {
            let Some(t159) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008220f5); };
            c.w32(11, t159);
            let r160 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r160);
            let Some(t161) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008220fc); };
            c.w32(15, t161);
            c.cmp(c.r16(15), 0x834u64, 16);
            if c.cc_e() { blk = 106; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w32(0, c.r64(15).wrapping_sub(0x80c));
            c.cmp(c.r32(0), 0xeu64, 32);
            if c.cc_be() { blk = 99; continue; }
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r16(15), 0x411u64, 16);
            if c.cc_ne() { blk = 107; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t162) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822125); };
            c.w32(9, t162);
            let Some(t163) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x7000822129); };
            c.cmp(t163, c.r32(9), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t164) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000822136); };
            c.w32(9, t164);
            let r165 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r165);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 107; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t166) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822147); };
            c.w32(11, t166);
            let r167 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r167);
            let Some(t168) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082214e); };
            c.cmp(t168, 0x40u64, 16);
            if c.cc_ae() { blk = 107; continue; }
            blk = 98; continue;
        }
        98 => {
            let Some(t169) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082215a); };
            c.w32(9, t169);
            let r170 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r170);
            blk = 110; continue;
        }
        99 => {
            let Some(t171) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822166); };
            c.w32(9, t171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x700082216a); };
            c.cmp(t172, c.r32(9), 32);
            if c.cc_ne() { blk = 107; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t173) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000822177); };
            c.w32(9, t173);
            c.w64(11, c.r64(9));
            let r174 = c.r32(11) & 0xc0u64;
            c.w32(11, r174);
            c.logic(r174, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 102; continue; }
            blk = 101; continue;
        }
        101 => {
            let Some(t175) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082218f); };
            c.w32(11, t175);
            let r176 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r176);
            let Some(t177) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x7000822196); };
            c.w32(11, t177);
            let r178 = c.r32(11) & 0x1fu64;
            c.w32(11, r178);
            c.logic(r178, 32);
            let r179 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r179);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 107; continue; }
            blk = 102; continue;
        }
        102 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 107; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t180) = c.ld32(c.r64(8).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008221ae); };
            c.w32(9, t180);
            let r181 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r181);
            let Some(t182) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008221b5); };
            c.cmp(t182, c.r32(9), 32);
            if c.cc_e() { blk = 107; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t183) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008221be); };
            c.w32(11, t183);
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008221c2); };
            c.cmp(t184, c.r32(11), 32);
            if c.cc_ne() { blk = 110; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t185) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008221cb); };
            c.w32(9, t185);
            let r186 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r186);
            blk = 110; continue;
        }
        106 => {
            let Some(t187) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008221d4); };
            c.w32(9, t187);
            let Some(t188) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008221d8); };
            c.cmp(t188, c.r32(9), 32);
            if c.cc_e() { blk = 109; continue; }
            blk = 107; continue;
        }
        107 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008221e1); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x70008221e3); }
            let Some(t189) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008221e5); };
            c.w64(3, t189);
            c.w32(0, 0x2u64);
            let Some(t190) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008221f1); };
            c.w64(6, t190);
            return c.call(0x700086ed74, 0x70008221fa, 0x70008221f5);
        }
        108 => {
            blk = 111; continue;
        }
        109 => {
            let Some(t191) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008221fc); };
            c.w32(9, t191);
            let r192 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r192);
            blk = 110; continue;
        }
        110 => {
            c.w64(0, c.r64(9));
            blk = 111; continue;
        }
        111 => {
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(0)) { return RegfileExit::fault(0x7000822206); }
            let Some(t193) = c.ld32(c.r64(5).wrapping_sub(0xb8)) else { return RegfileExit::fault(0x700082220a); };
            c.cmp(t193, 0x0u64, 32);
            if c.cc_e() { blk = 544; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822217); };
            c.w64(8, t194);
            let Some(t195) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082221b); };
            c.w32(1, t195);
            let r196 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r196);
            let Some(t197) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000822222); };
            c.w32(9, t197);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x7000822226); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(9)) { return RegfileExit::fault(0x700082222a); }
            c.logic(c.r32(9) & 0x200000u64, 32);
            if c.cc_ne() { blk = 368; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t198) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082223e); };
            c.w32(11, t198);
            let r199 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r199);
            c.logic(c.r32(9) & 0xffc00u64, 32);
            if c.cc_e() { blk = 260; continue; }
            blk = 114; continue;
        }
        114 => {
            let r200 = c.shr(c.r32(9), 0xau64, 32).unwrap_or(c.r32(9));
            c.w32(9, r200);
            let r201 = c.r32(9) & 0x3ffu64;
            c.w32(9, r201);
            c.logic(r201, 32);
            c.cmp(c.r32(9), 0x20u64, 32);
            if c.cc_be() { blk = 127; continue; }
            blk = 115; continue;
        }
        115 => {
            let Some(t202) = c.ld16(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x7000822267); };
            c.w32(12, (t202 as u16 as i16 as i64 as u64));
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x700082226c); };
            c.w64(15, t203);
            let Some(t204) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822270); };
            c.w32(3, t204);
            let r205 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r205);
            let r206 = c.sub(c.r32(12), 0x1u64, 32);
            c.w32(12, r206);
            c.w64(6, c.r64(12));
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 117; continue;
        }
        116 => {
            c.w64(6, c.r64(7));
            blk = 117; continue;
        }
        117 => {
            c.w64(7, c.r64(6));
            let r207 = c.sub(c.r32(7), c.r32(2), 32);
            c.w32(7, r207);
            let r208 = c.shr(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r208);
            let r209 = c.add(c.r32(7), c.r32(2), 32);
            c.w32(7, r209);
            c.w32(0, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t210) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000822294); };
            c.w32(0, t210);
            let r211 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r211);
            let r212 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r212);
            let r213 = c.r32(0) & 0x3ffu64;
            c.w32(0, r213);
            c.logic(r213, 32);
            c.w32(0, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t214) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008222a9); };
            c.w32(0, t214);
            let r215 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r215);
            let Some(t216) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008222b1); };
            c.w32(0, t216);
            let r217 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r217);
            c.cmp(c.r32(3), c.r32(0), 32);
            if c.cc_be() { blk = 119; continue; }
            blk = 118; continue;
        }
        118 => {
            c.w32(2, c.r64(7).wrapping_add(0x1));
            c.w64(7, c.r64(6));
            blk = 119; continue;
        }
        119 => {
            c.cmp(c.r32(2), c.r32(7), 32);
            if c.cc_ne() { blk = 116; continue; }
            blk = 120; continue;
        }
        120 => {
            blk = 122; continue;
        }
        121 => {
            let r218 = c.add(c.r32(2), 0x1u64, 32);
            c.w32(2, r218);
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_l() { blk = 261; continue; }
            blk = 123; continue;
        }
        123 => {
            c.w32(0, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t219) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008222d9); };
            c.w32(0, t219);
            let r220 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r220);
            let r221 = c.shr(c.r32(0), 0x9u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r221);
            let r222 = c.r32(0) & 0x3ffu64;
            c.w32(0, r222);
            c.logic(r222, 32);
            c.w32(6, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t223) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008222ee); };
            c.w32(7, t223);
            let r224 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r224);
            let Some(t225) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008222f6); };
            c.w32(8, t225);
            let r226 = c.shr(c.r32(8), 0x2u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r226);
            c.cmp(c.r32(3), c.r32(8), 32);
            if c.cc_ne() { blk = 261; continue; }
            blk = 124; continue;
        }
        124 => {
            c.cmp(c.r32(7), c.r32(15), 32);
            if c.cc_ne() { blk = 121; continue; }
            blk = 125; continue;
        }
        125 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_be() { blk = 261; continue; }
            blk = 126; continue;
        }
        126 => {
            blk = 133; continue;
        }
        127 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let r227 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(6, r227);
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 258; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t228) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000822328); };
            c.w64(12, t228);
            blk = 129; continue;
        }
        129 => {
            let r229 = c.sub(c.r64(6), 0x3u64, 64);
            c.w64(6, r229);
            let Some(t230) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000822330); };
            c.cmp(t230, c.r32(12), 32);
            if c.cc_e() { blk = 132; continue; }
            blk = 130; continue;
        }
        130 => {
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_ne() { blk = 129; continue; }
            blk = 131; continue;
        }
        131 => {
            blk = 259; continue;
        }
        132 => {
            c.w64(15, c.r64(12));
            blk = 133; continue;
        }
        133 => {
            c.w64(8, c.r64(6).wrapping_mul(4));
            let Some(t231) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082234d); };
            c.w32(9, t231);
            c.w32(12, c.r32(9));
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 518; continue; }
            blk = 134; continue;
        }
        134 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 135; continue;
        }
        135 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 136; continue;
        }
        136 => {
            let Some(t232) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082236f); };
            c.w32(8, t232);
            let r233 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r233);
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822377); };
            c.w64(0, t234);
            blk = 471; continue;
        }
        137 => {
            let r235 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r235);
            if let Some(sr) = c.shr(c.r64(9), 0x13u64, 64) { c.w64(9, sr); }
            let Some(t236) = c.ld8(c.r64(1).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000822387); };
            c.w32(8, t236);
            let r237 = c.r64(9) & 0x3ffu64;
            c.w64(9, r237);
            c.logic(r237, 64);
            let r238 = c.r32(12) & 0x380u64;
            c.w32(12, r238);
            c.logic(r238, 32);
            let r239 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r239);
            c.cmp(c.r32(12), 0x100u64, 32);
            c.w8(9, (c.cc_ne() as u64));
            c.w32(9, c.r8(9));
            let Some(t240) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008223ac); };
            c.w32(11, t240);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 142; continue; }
            blk = 138; continue;
        }
        138 => {
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008223b6); };
            c.w64(0, t241);
            let Some(t242) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008223ba); };
            c.w32(12, t242);
            let r243 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r243);
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_ne() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t244) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008223c7); };
            c.w64(12, t244);
            blk = 140; continue;
        }
        140 => {
            let r245 = c.sub(c.r64(8), c.r64(11), 64);
            c.w64(8, r245);
            let Some(t246) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008223d1); };
            c.w32(8, t246);
            let r247 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r247);
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 471; continue; }
            blk = 141; continue;
        }
        141 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008223e3); }
            blk = 144; continue;
        }
        142 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 147; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t248) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008223f1); };
            c.w64(0, t248);
            let Some(t249) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008223f5); };
            c.w32(8, t249);
            let r250 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r250);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008223fd); }
            blk = 144; continue;
        }
        144 => {
            let Some(t251) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000822403); };
            c.w64(8, t251);
            let Some(t252) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x700082240a); };
            c.w64(9, t252);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t253) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000822411); };
            c.w64(12, t253);
            let Some(t254) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000822418); };
            c.cmp(t254, c.r64(11), 64);
            if c.cc_be() { blk = 545; continue; }
            blk = 145; continue;
        }
        145 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x7000822422); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 146; continue;
        }
        146 => {
            let Some(t255) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822429); };
            c.w64(9, t255);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000822430); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000822434); }
            blk = 471; continue;
        }
        147 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082243f); };
            c.w64(0, t256);
            let Some(t257) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822443); };
            c.w32(8, t257);
            let r258 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r258);
            blk = 471; continue;
        }
        148 => {
            let Some(t259) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x7000822450); };
            c.cmp(t259, c.r32(15), 32);
            if c.cc_e() { blk = 573; continue; }
            blk = 149; continue;
        }
        149 => {
            let Some(t260) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x700082245d); };
            c.cmp(t260, c.r32(15), 32);
            if c.cc_e() { blk = 572; continue; }
            blk = 150; continue;
        }
        150 => {
            let Some(t261) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x700082246a); };
            c.cmp(t261, c.r32(15), 32);
            if c.cc_e() { blk = 571; continue; }
            blk = 151; continue;
        }
        151 => {
            let Some(t262) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000822477); };
            c.cmp(t262, c.r32(15), 32);
            if c.cc_e() { blk = 570; continue; }
            blk = 152; continue;
        }
        152 => {
            let Some(t263) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000822484); };
            c.cmp(t263, c.r32(15), 32);
            if c.cc_e() { blk = 569; continue; }
            blk = 153; continue;
        }
        153 => {
            let Some(t264) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000822491); };
            c.cmp(t264, c.r32(15), 32);
            if c.cc_e() { blk = 568; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t265) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x700082249e); };
            c.cmp(t265, c.r32(15), 32);
            if c.cc_e() { blk = 567; continue; }
            blk = 155; continue;
        }
        155 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_e() { blk = 158; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t266) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008224b1); };
            c.w64(0, t266);
            let Some(t267) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008224b5); };
            c.cmp(t267, c.r32(0), 32);
            if c.cc_ne() { blk = 547; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t268) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008224c0); };
            c.w64(0, t268);
            blk = 509; continue;
        }
        158 => {
            c.w64(0, c.r64(12));
            let r269 = c.r32(0) & 0x380u64;
            c.w32(0, r269);
            c.logic(r269, 32);
            c.cmp(c.r32(0), 0x80u64, 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 159; continue;
        }
        159 => {
            c.cmp(c.r32(0), 0x100u64, 32);
            if c.cc_e() { blk = 169; continue; }
            blk = 160; continue;
        }
        160 => {
            c.cmp(c.r32(0), 0x180u64, 32);
            if c.cc_e() { blk = 163; continue; }
            blk = 161; continue;
        }
        161 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 548; continue; }
            blk = 162; continue;
        }
        162 => {
            blk = 173; continue;
        }
        163 => {
            let Some(t270) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008224f8); };
            c.logic(t270 & 0x1u64, 8);
            if c.cc_e() { blk = 552; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t271) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000822502); };
            c.w32(8, t271);
            let r272 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r272);
            c.w32(11, 0x4u64);
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_e() { blk = 551; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w32(11, 0x2u64);
            c.cmp(c.r32(11), c.r32(8), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 166; continue;
        }
        166 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 550; continue; }
            blk = 167; continue;
        }
        167 => {
            let r273 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r273);
            c.logic(r273, 64);
            let Some(t274) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822532); };
            c.w64(11, t274);
            let Some(t275) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822536); };
            c.cmp(t275, c.r32(8), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 168; continue;
        }
        168 => {
            blk = 549; continue;
        }
        169 => {
            let Some(t276) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822541); };
            c.logic(t276 & 0x1u64, 8);
            if c.cc_e() { blk = 173; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t277) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822547); };
            c.w64(8, t277);
            let Some(t278) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082254b); };
            c.w32(11, t278);
            let Some(t279) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082254f); };
            c.cmp(t279, c.r32(11), 32);
            if c.cc_e() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822558); };
            c.w64(0, t280);
            c.w64(12, c.r64(8));
            blk = 508; continue;
        }
        172 => {
            let Some(t281) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822564); };
            c.logic(t281 & 0x1u64, 8);
            if c.cc_ne() { blk = 553; continue; }
            blk = 173; continue;
        }
        173 => {
            let r282 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r282);
            if let Some(sr) = c.shr(c.r64(9), 0x13u64, 64) { c.w64(9, sr); }
            let Some(t283) = c.ld8(c.r64(1).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000822575); };
            c.w32(8, t283);
            let r284 = c.r64(9) & 0x3ffu64;
            c.w64(9, r284);
            c.logic(r284, 64);
            let r285 = c.r32(12) & 0x8u64;
            c.w32(12, r285);
            c.logic(r285, 32);
            let r286 = c.add(c.r64(8), c.r64(9), 64);
            c.w64(8, r286);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(12), 0x8u64, 32);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            let Some(t287) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008225a1); };
            c.w32(12, t287);
            c.cmp(c.r64(8), c.r64(12), 64);
            if c.cc_b() { blk = 216; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t288) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008225af); };
            c.w64(0, t288);
            let Some(t289) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008225b3); };
            c.w32(3, t289);
            let r290 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r290);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008225be); };
            c.w64(3, t291);
            blk = 176; continue;
        }
        176 => {
            let r292 = c.sub(c.r64(8), c.r64(12), 64);
            c.w64(8, r292);
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 202; continue; }
            blk = 177; continue;
        }
        177 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 181; continue; }
            blk = 178; continue;
        }
        178 => {
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008225e0); };
            c.w64(2, t293);
            if !c.st32(c.r64(3).wrapping_add(c.r64(12)), c.r32(2)) { return RegfileExit::fault(0x70008225e4); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 180; continue; }
            blk = 179; continue;
        }
        179 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r294 = c.r64(8) & c.r64(3);
            c.w64(8, r294);
            c.logic(r294, 64);
            let Some(t295) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008225f7); };
            c.logic(t295 & 0x4u64, 8);
            if c.cc_ne() { blk = 680; continue; }
            blk = 180; continue;
        }
        180 => {
            blk = 509; continue;
        }
        181 => {
            let Some(t296) = c.ld32(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000822607); };
            c.w32(8, t296);
            let r297 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r297);
            let Some(t298) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700082260f); };
            c.cmp(c.r32(8), t298, 32);
            if c.cc_e() { blk = 509; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t299) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822619); };
            c.logic(t299 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 183; continue;
        }
        183 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 187; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t300) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822631); };
            c.w32(9, t300);
            let Some(t301) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822635); };
            c.cmp(t301, c.r32(9), 32);
            if c.cc_ne() { blk = 554; continue; }
            blk = 185; continue;
        }
        185 => {
            let Some(t302) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822642); };
            c.w64(12, t302);
            let Some(t303) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822646); };
            c.w32(9, t303);
            let Some(t304) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082264b); };
            c.cmp(t304, c.r32(9), 32);
            if c.cc_ne() { blk = 508; continue; }
            blk = 186; continue;
        }
        186 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822658); }
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082265e); }
            blk = 193; continue;
        }
        187 => {
            let Some(t305) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822667); };
            c.w32(9, t305);
            let Some(t306) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082266b); };
            c.cmp(t306, c.r32(9), 32);
            if c.cc_ne() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t307) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822674); };
            c.w64(12, t307);
            let r308 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r308);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x700082267b); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(12) as u32 as i32 as i64));
            let Some(t309) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700082268a); };
            c.w64(12, t309);
            blk = 193; continue;
        }
        189 => {
            let Some(t310) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822690); };
            c.w64(12, t310);
            blk = 508; continue;
        }
        190 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 555; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t311) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008226a3); };
            c.w64(12, t311);
            let Some(t312) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008226a7); };
            c.w32(9, t312);
            let Some(t313) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008226ac); };
            c.cmp(t313, c.r32(9), 32);
            if c.cc_ne() { blk = 508; continue; }
            blk = 192; continue;
        }
        192 => {
            let r314 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r314);
            if !c.movsd_rm(1, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x70008226bc); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 193; continue;
        }
        193 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            if c.cc_e() { blk = 200; continue; }
            blk = 195; continue;
        }
        195 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 197; continue; }
            blk = 196; continue;
        }
        196 => {
            if c.cc_e() { blk = 508; continue; }
            blk = 197; continue;
        }
        197 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 509; continue; }
            blk = 198; continue;
        }
        198 => {
            if c.cc_e() { blk = 508; continue; }
            blk = 199; continue;
        }
        199 => {
            blk = 509; continue;
        }
        200 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 509; continue; }
            blk = 201; continue;
        }
        201 => {
            blk = 508; continue;
        }
        202 => {
            let Some(t315) = c.ld32(c.r64(3).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000822715); };
            c.w32(8, t315);
            let r316 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r316);
            let Some(t317) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700082271d); };
            c.logic(t317 & 0x1u64, 8);
            if c.cc_e() { blk = 204; continue; }
            blk = 203; continue;
        }
        203 => {
            let Some(t318) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822723); };
            c.w64(12, t318);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822727); }
            blk = 205; continue;
        }
        204 => {
            let Some(t319) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822730); };
            c.w64(12, t319);
            let r320 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r320);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 207; continue; }
            blk = 206; continue;
        }
        206 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000822746); }
            blk = 509; continue;
        }
        207 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822751); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 209; continue; }
            blk = 208; continue;
        }
        208 => {
            if c.cc_e() { blk = 214; continue; }
            blk = 209; continue;
        }
        209 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            if c.cc_e() { blk = 557; continue; }
            blk = 211; continue;
        }
        211 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 509; continue; }
            blk = 212; continue;
        }
        212 => {
            if c.cc_e() { blk = 556; continue; }
            blk = 213; continue;
        }
        213 => {
            blk = 509; continue;
        }
        214 => {
            c.movq_rx(8, 0);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 1);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_e() { blk = 509; continue; }
            blk = 215; continue;
        }
        215 => {
            blk = 558; continue;
        }
        216 => {
            c.w64(12, c.r64(8).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 245; continue; }
            blk = 217; continue;
        }
        217 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 221; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008227b8); };
            c.w64(0, t321);
            let Some(t322) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008227bc); };
            c.w64(3, t322);
            if !c.st32(c.r64(0).wrapping_add(c.r64(12)), c.r32(3)) { return RegfileExit::fault(0x70008227c0); }
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 220; continue; }
            blk = 219; continue;
        }
        219 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r323 = c.r64(8) & c.r64(0);
            c.w64(8, r323);
            c.logic(r323, 64);
            let Some(t324) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008227d3); };
            c.logic(t324 & 0x4u64, 8);
            if c.cc_ne() { blk = 677; continue; }
            blk = 220; continue;
        }
        220 => {
            blk = 509; continue;
        }
        221 => {
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008227e3); };
            c.w64(12, t325);
            let Some(t326) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008227e7); };
            c.w32(8, t326);
            let r327 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r327);
            let Some(t328) = c.ld32(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008227ef); };
            c.cmp(c.r32(8), t328, 32);
            if c.cc_e() { blk = 244; continue; }
            blk = 222; continue;
        }
        222 => {
            let Some(t329) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008227f9); };
            c.logic(t329 & 0x1u64, 8);
            c.w8(9, (c.cc_e() as u64));
            c.w32(9, c.r8(9));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 231; continue; }
            blk = 223; continue;
        }
        223 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 228; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t330) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822815); };
            c.w32(9, t330);
            let Some(t331) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822819); };
            c.cmp(t331, c.r32(9), 32);
            if c.cc_ne() { blk = 559; continue; }
            blk = 225; continue;
        }
        225 => {
            let Some(t332) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822826); };
            c.w64(0, t332);
            let Some(t333) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082282a); };
            c.w32(9, t333);
            let Some(t334) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082282e); };
            c.cmp(t334, c.r32(9), 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 226; continue;
        }
        226 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822837); }
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x700082283d); }
            blk = 234; continue;
        }
        227 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 508; continue;
        }
        228 => {
            let Some(t335) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822852); };
            c.w32(9, t335);
            let Some(t336) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822856); };
            c.cmp(t336, c.r32(9), 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 229; continue;
        }
        229 => {
            let Some(t337) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700082285f); };
            c.w64(0, t337);
            let r338 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r338);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822865); }
            c.xorpd_rr(1, 1);
            c.cvtsi2sd(1, (c.r32(0) as u32 as i32 as i64));
            let Some(t339) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822873); };
            c.w64(0, t339);
            blk = 234; continue;
        }
        230 => {
            c.w64(0, c.r64(12));
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700082287c); };
            c.w64(12, t340);
            blk = 508; continue;
        }
        231 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 563; continue; }
            blk = 232; continue;
        }
        232 => {
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700082288f); };
            c.w64(0, t341);
            let Some(t342) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822893); };
            c.w32(9, t342);
            let Some(t343) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822897); };
            c.cmp(t343, c.r32(9), 32);
            if c.cc_ne() { blk = 243; continue; }
            blk = 233; continue;
        }
        233 => {
            let r344 = c.sar(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r344);
            if !c.movsd_rm(1, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008228a3); }
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            blk = 234; continue;
        }
        234 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_p() { blk = 236; continue; }
            blk = 235; continue;
        }
        235 => {
            if c.cc_e() { blk = 241; continue; }
            blk = 236; continue;
        }
        236 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 238; continue; }
            blk = 237; continue;
        }
        237 => {
            if c.cc_e() { blk = 561; continue; }
            blk = 238; continue;
        }
        238 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 240; continue; }
            blk = 239; continue;
        }
        239 => {
            if c.cc_e() { blk = 560; continue; }
            blk = 240; continue;
        }
        240 => {
            c.w64(0, c.r64(12));
            blk = 509; continue;
        }
        241 => {
            c.movq_rx(8, 1);
            if let Some(sr) = c.shr(c.r64(8), 0x20u64, 64) { c.w64(8, sr); }
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.cmp(c.r32(8), c.r32(9), 32);
            if c.cc_ne() { blk = 562; continue; }
            blk = 242; continue;
        }
        242 => {
            c.w64(0, c.r64(12));
            blk = 509; continue;
        }
        243 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 508; continue;
        }
        244 => {
            c.w64(0, c.r64(12));
            blk = 509; continue;
        }
        245 => {
            let Some(t345) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822912); };
            c.logic(t345 & 0x1u64, 8);
            if c.cc_e() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822918); };
            c.w64(12, t346);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x700082291c); }
            blk = 248; continue;
        }
        247 => {
            let Some(t347) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822925); };
            c.w64(12, t347);
            let r348 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r348);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 248; continue;
        }
        248 => {
            let Some(t349) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822935); };
            c.w64(12, t349);
            let Some(t350) = c.ld32(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822939); };
            c.w32(8, t350);
            let r351 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r351);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 257; continue; }
            blk = 249; continue;
        }
        249 => {
            if !c.movsd_rm(1, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822947); }
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_p() { blk = 251; continue; }
            blk = 250; continue;
        }
        250 => {
            if c.cc_e() { blk = 256; continue; }
            blk = 251; continue;
        }
        251 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(1));
            if c.cc_p() { blk = 253; continue; }
            blk = 252; continue;
        }
        252 => {
            if c.cc_e() { blk = 565; continue; }
            blk = 253; continue;
        }
        253 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(0));
            if c.cc_p() { blk = 257; continue; }
            blk = 254; continue;
        }
        254 => {
            if c.cc_e() { blk = 564; continue; }
            blk = 255; continue;
        }
        255 => {
            blk = 257; continue;
        }
        256 => {
            c.movq_rx(9, 0);
            if let Some(sr) = c.shr(c.r64(9), 0x20u64, 64) { c.w64(9, sr); }
            c.movq_rx(11, 1);
            if let Some(sr) = c.shr(c.r64(11), 0x20u64, 64) { c.w64(11, sr); }
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_ne() { blk = 566; continue; }
            blk = 257; continue;
        }
        257 => {
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x700082298a); }
            c.w64(0, c.r64(12));
            blk = 509; continue;
        }
        258 => {
            let Some(t352) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000822998); };
            c.w64(12, t352);
            blk = 259; continue;
        }
        259 => {
            c.w64(15, c.r64(12));
            blk = 261; continue;
        }
        260 => {
            let Some(t353) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008229a1); };
            c.w64(15, t353);
            blk = 261; continue;
        }
        261 => {
            let Some(t354) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008229a5); };
            c.cmp(t354, c.r32(15), 32);
            if c.cc_e() { blk = 606; continue; }
            blk = 262; continue;
        }
        262 => {
            let Some(t355) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x70008229b2); };
            c.cmp(t355, c.r32(15), 32);
            if c.cc_e() { blk = 605; continue; }
            blk = 263; continue;
        }
        263 => {
            let Some(t356) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x70008229bf); };
            c.cmp(t356, c.r32(15), 32);
            if c.cc_e() { blk = 604; continue; }
            blk = 264; continue;
        }
        264 => {
            let Some(t357) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x70008229cc); };
            c.cmp(t357, c.r32(15), 32);
            if c.cc_e() { blk = 603; continue; }
            blk = 265; continue;
        }
        265 => {
            let Some(t358) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x70008229d9); };
            c.cmp(t358, c.r32(15), 32);
            if c.cc_e() { blk = 602; continue; }
            blk = 266; continue;
        }
        266 => {
            let Some(t359) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x70008229e6); };
            c.cmp(t359, c.r32(15), 32);
            if c.cc_e() { blk = 601; continue; }
            blk = 267; continue;
        }
        267 => {
            let Some(t360) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x70008229f3); };
            c.cmp(t360, c.r32(15), 32);
            if c.cc_e() { blk = 600; continue; }
            blk = 268; continue;
        }
        268 => {
            let Some(t361) = c.ld32(c.r64(1).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000822a00); };
            c.w32(8, t361);
            let r362 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r362);
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 599; continue; }
            blk = 269; continue;
        }
        269 => {
            c.cmp(c.r32(8), 0x3u64, 32);
            if c.cc_e() { blk = 598; continue; }
            blk = 270; continue;
        }
        270 => {
            let r363 = c.r32(9) & 0x3u64;
            c.w32(9, r363);
            c.logic(r363, 32);
            c.cmp(c.r32(9), 0x1u64, 32);
            if c.cc_e() { blk = 272; continue; }
            blk = 271; continue;
        }
        271 => {
            let r364 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r364);
            c.logic(r364, 64);
            blk = 298; continue;
        }
        272 => {
            let Some(t365) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822a31); };
            c.w32(9, t365);
            let Some(t366) = c.ld32(c.r64(13).wrapping_add(0x148)) else { return RegfileExit::fault(0x7000822a35); };
            c.cmp(t366, c.r32(9), 32);
            if c.cc_ne() { blk = 574; continue; }
            blk = 273; continue;
        }
        273 => {
            let Some(t367) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822a42); };
            c.w32(9, t367);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x2u64, 64);
            c.w8(9, (c.cc_b() as u64));
            c.w32(9, c.r8(9));
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 275; continue; }
            blk = 274; continue;
        }
        274 => {
            let Some(t368) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000822a5e); };
            c.w32(11, t368);
            let r369 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r369);
            blk = 276; continue;
        }
        275 => {
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 276; continue;
        }
        276 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 597; continue; }
            blk = 277; continue;
        }
        277 => {
            c.cmp(c.r32(11), 0x20u64, 32);
            if c.cc_be() { blk = 293; continue; }
            blk = 278; continue;
        }
        278 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 280; continue; }
            blk = 279; continue;
        }
        279 => {
            let Some(t370) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000822a84); };
            c.w32(9, t370);
            let r371 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r371);
            blk = 281; continue;
        }
        280 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 281; continue;
        }
        281 => {
            let Some(t372) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822a90); };
            c.w32(12, t372);
            let r373 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r373);
            let r374 = c.sub(c.r32(9), 0x1u64, 32);
            c.w32(9, r374);
            c.w64(3, c.r64(9));
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 283; continue;
        }
        282 => {
            c.w64(3, c.r64(2));
            blk = 283; continue;
        }
        283 => {
            c.w64(2, c.r64(3));
            let r375 = c.sub(c.r32(2), c.r32(0), 32);
            c.w32(2, r375);
            let r376 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r376);
            let r377 = c.add(c.r32(2), c.r32(0), 32);
            c.w32(2, r377);
            c.w32(1, c.r64(2).wrapping_add(c.r64(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t378) = c.ld32(c.r64(8).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000822ab5); };
            c.w32(1, t378);
            let r379 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r379);
            let Some(t380) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822abd); };
            c.w32(1, t380);
            let r381 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r381);
            c.cmp(c.r32(12), c.r32(1), 32);
            if c.cc_be() { blk = 285; continue; }
            blk = 284; continue;
        }
        284 => {
            c.w32(0, c.r64(2).wrapping_add(0x1));
            c.w64(2, c.r64(3));
            blk = 285; continue;
        }
        285 => {
            c.cmp(c.r32(0), c.r32(2), 32);
            if c.cc_ne() { blk = 282; continue; }
            blk = 286; continue;
        }
        286 => {
            blk = 288; continue;
        }
        287 => {
            let r382 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r382);
            blk = 288; continue;
        }
        288 => {
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_l() { blk = 577; continue; }
            blk = 289; continue;
        }
        289 => {
            c.w32(3, c.r64(0).wrapping_add(c.r64(0)));
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t383) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000822ae6); };
            c.w32(2, t383);
            let r384 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r384);
            let Some(t385) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822aee); };
            c.w32(1, t385);
            let r386 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r386);
            c.cmp(c.r32(12), c.r32(1), 32);
            if c.cc_ne() { blk = 576; continue; }
            blk = 290; continue;
        }
        290 => {
            c.cmp(c.r32(2), c.r32(15), 32);
            if c.cc_ne() { blk = 287; continue; }
            blk = 291; continue;
        }
        291 => {
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_be() { blk = 575; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w64(9, c.r64(3).wrapping_add(0x2));
            blk = 297; continue;
        }
        293 => {
            c.w64(9, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(9).wrapping_mul(2).wrapping_add(0x2));
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_e() { blk = 596; continue; }
            blk = 294; continue;
        }
        294 => {
            let r387 = c.sub(c.r64(9), 0x2u64, 64);
            c.w64(9, r387);
            let Some(t388) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000822b2a); };
            c.cmp(t388, c.r32(15), 32);
            if c.cc_e() { blk = 297; continue; }
            blk = 295; continue;
        }
        295 => {
            c.cmp(c.r64(9), 0x2u64, 64);
            if c.cc_ne() { blk = 294; continue; }
            blk = 296; continue;
        }
        296 => {
            blk = 596; continue;
        }
        297 => {
            let Some(t389) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000822b3c); };
            c.w32(8, t389);
            let r390 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r390);
            let r391 = c.r64(8) & 0xfffffffffffffffdu64;
            c.w64(8, r391);
            c.logic(r391, 64);
            blk = 298; continue;
        }
        298 => {
            let Some(t392) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x7000822b48); };
            c.w32(9, t392);
            let r393 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r393);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(8)) { return RegfileExit::fault(0x7000822b4f); }
            c.w32(11, 0);
            c.logic(0, 32);
            c.cmp(c.r32(9), c.r32(11), 32);
            if c.cc_e() { blk = 300; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t394) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822b5b); };
            c.cmp(t394, c.r32(11), 32);
            if c.cc_ne() { blk = 578; continue; }
            blk = 300; continue;
        }
        300 => {
            let Some(t395) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000822b65); };
            c.w32(9, t395);
            c.logic(c.r32(9) & 0x1000000u64, 32);
            if c.cc_ne() { blk = 595; continue; }
            blk = 301; continue;
        }
        301 => {
            if let Some(sr) = c.shr(c.r64(9), 0xau64, 64) { c.w64(9, sr); }
            let r396 = c.r64(9) & 0x3ffu64;
            c.w64(9, r396);
            c.logic(r396, 64);
            c.w64(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_sub(0x3));
            if let Some(sr) = c.shl(c.r64(9), 0x2u64, 64) { c.w64(9, sr); }
            let Some(t397) = c.ld32(c.r64(8).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000822b8a); };
            c.w32(11, t397);
            let r398 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r398);
            let Some(t399) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000822b91); };
            c.cmp(t399, c.r32(15), 32);
            if c.cc_ne() { blk = 594; continue; }
            blk = 302; continue;
        }
        302 => {
            let Some(t400) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000822b9c); };
            c.w32(12, t400);
            c.w32(0, c.r32(12));
            c.logic(c.r8(12) & 0x52u64, 8);
            if c.cc_ne() { blk = 593; continue; }
            blk = 303; continue;
        }
        303 => {
            let Some(t401) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822bae); };
            c.w32(3, t401);
            let Some(t402) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000822bb2); };
            c.cmp(t402, c.r32(3), 32);
            if c.cc_ne() { blk = 306; continue; }
            blk = 304; continue;
        }
        304 => {
            let Some(t403) = c.ld8(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000822bbb); };
            c.logic(t403 & 0x1u64, 8);
            if c.cc_e() { blk = 306; continue; }
            blk = 305; continue;
        }
        305 => {
            c.w32(3, 0x1u64);
            blk = 307; continue;
        }
        306 => {
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 307; continue;
        }
        307 => {
            c.logic(c.r8(12) & 0x20u64, 8);
            c.w8(2, (c.cc_ne() as u64));
            c.w32(2, c.r8(2));
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ne() { blk = 579; continue; }
            blk = 308; continue;
        }
        308 => {
            c.logic(c.r8(12) & 0x4u64, 8);
            if c.cc_e() { blk = 312; continue; }
            blk = 309; continue;
        }
        309 => {
            let Some(t404) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822be3); };
            c.w64(3, t404);
            let Some(t405) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000822be7); };
            c.cmp(t405, c.r32(3), 32);
            if c.cc_ne() { blk = 580; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822bf2); };
            c.w64(2, t406);
            if !c.st32(c.r64(2).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000822bf6); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r407 = c.r64(9) & c.r64(2);
            c.w64(9, r407);
            c.logic(r407, 64);
            let Some(t408) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822c04); };
            c.logic(t408 & 0x4u64, 8);
            if c.cc_ne() { blk = 674; continue; }
            blk = 311; continue;
        }
        311 => {
            c.w64(0, c.r64(2));
            blk = 509; continue;
        }
        312 => {
            let r409 = c.r32(0) & 0x380u64;
            c.w32(0, r409);
            c.logic(r409, 32);
            c.cmp(c.r32(0), 0x80u64, 32);
            if c.cc_e() { blk = 326; continue; }
            blk = 313; continue;
        }
        313 => {
            c.cmp(c.r32(0), 0x100u64, 32);
            if c.cc_e() { blk = 323; continue; }
            blk = 314; continue;
        }
        314 => {
            c.cmp(c.r32(0), 0x180u64, 32);
            if c.cc_e() { blk = 317; continue; }
            blk = 315; continue;
        }
        315 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_e() { blk = 581; continue; }
            blk = 316; continue;
        }
        316 => {
            blk = 327; continue;
        }
        317 => {
            let Some(t410) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822c43); };
            c.logic(t410 & 0x1u64, 8);
            if c.cc_e() { blk = 585; continue; }
            blk = 318; continue;
        }
        318 => {
            let Some(t411) = c.ld32(c.r64(11).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x7000822c4d); };
            c.w32(9, t411);
            let r412 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r412);
            c.w32(11, 0x4u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 584; continue; }
            blk = 319; continue;
        }
        319 => {
            c.w32(11, 0x2u64);
            c.cmp(c.r32(11), c.r32(9), 32);
            if c.cc_e() { blk = 327; continue; }
            blk = 320; continue;
        }
        320 => {
            c.cmp(c.r32(9), 0x3u64, 32);
            if c.cc_e() { blk = 583; continue; }
            blk = 321; continue;
        }
        321 => {
            let r413 = c.r64(9) & 0xfffffffffffffffdu64;
            c.w64(9, r413);
            c.logic(r413, 64);
            let Some(t414) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822c7d); };
            c.w64(11, t414);
            let Some(t415) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822c81); };
            c.cmp(t415, c.r32(9), 32);
            if c.cc_e() { blk = 327; continue; }
            blk = 322; continue;
        }
        322 => {
            blk = 582; continue;
        }
        323 => {
            let Some(t416) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822c8c); };
            c.logic(t416 & 0x1u64, 8);
            if c.cc_e() { blk = 327; continue; }
            blk = 324; continue;
        }
        324 => {
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822c92); };
            c.w64(11, t417);
            let Some(t418) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000822c96); };
            c.w32(9, t418);
            let Some(t419) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822c9a); };
            c.cmp(t419, c.r32(9), 32);
            if c.cc_e() { blk = 327; continue; }
            blk = 325; continue;
        }
        325 => {
            let Some(t420) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822ca3); };
            c.w64(0, t420);
            c.w64(12, c.r64(11));
            blk = 508; continue;
        }
        326 => {
            let Some(t421) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822caf); };
            c.logic(t421 & 0x1u64, 8);
            if c.cc_ne() { blk = 586; continue; }
            blk = 327; continue;
        }
        327 => {
            let r422 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r422);
            if let Some(sr) = c.shr(c.r64(12), 0x13u64, 64) { c.w64(12, sr); }
            let Some(t423) = c.ld8(c.r64(8).wrapping_add(0x4)) else { return RegfileExit::fault(0x7000822cc0); };
            c.w32(9, t423);
            let Some(t424) = c.ld8(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822cc5); };
            c.w32(11, t424);
            let r425 = c.r64(12) & 0x3ffu64;
            c.w64(12, r425);
            c.logic(r425, 64);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let r426 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r426);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 354; continue; }
            blk = 328; continue;
        }
        328 => {
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_e() { blk = 335; continue; }
            blk = 329; continue;
        }
        329 => {
            let Some(t427) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822cf0); };
            c.logic(t427 & 0x1u64, 8);
            if c.cc_e() { blk = 331; continue; }
            blk = 330; continue;
        }
        330 => {
            let Some(t428) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822cf6); };
            c.w64(12, t428);
            if !c.movsd_rm(0, c.r64(12).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822cfa); }
            blk = 332; continue;
        }
        331 => {
            let Some(t429) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822d03); };
            c.w64(12, t429);
            let r430 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r430);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(12) as u32 as i32 as i64));
            blk = 332; continue;
        }
        332 => {
            let Some(t431) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000822d13); };
            c.w64(12, t431);
            let Some(t432) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000822d1a); };
            c.w64(15, t432);
            c.w64(0, c.r64(15).wrapping_add(0xc));
            let Some(t433) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000822d22); };
            c.w64(3, t433);
            let Some(t434) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x7000822d29); };
            c.cmp(t434, c.r64(0), 64);
            if c.cc_be() { blk = 587; continue; }
            blk = 333; continue;
        }
        333 => {
            if !c.st64(c.r64(12), c.r64(0)) { return RegfileExit::fault(0x7000822d32); }
            c.w64(12, c.r64(15).wrapping_add(0x1));
            blk = 334; continue;
        }
        334 => {
            let Some(t435) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822d3a); };
            c.w64(15, t435);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000822d41); }
            if !c.movsd_mr(c.r64(12).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000822d46); }
            blk = 336; continue;
        }
        335 => {
            let Some(t436) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822d4f); };
            c.w64(12, t436);
            blk = 336; continue;
        }
        336 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822d53); };
            c.w64(0, t437);
            let Some(t438) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822d57); };
            c.w32(15, t438);
            let r439 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r439);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(12)) { return RegfileExit::fault(0x7000822d5e); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 338; continue; }
            blk = 337; continue;
        }
        337 => {
            let Some(t440) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822d68); };
            c.w32(3, t440);
            let r441 = c.sar(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r441);
            c.w64(2, c.r64(3));
            let r442 = c.r32(2) & 0x3ffu64;
            c.w32(2, r442);
            c.logic(r442, 32);
            let r443 = c.r32(3) & 0x3ffffc00u64;
            c.w32(3, r443);
            c.logic(r443, 32);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            blk = 339; continue;
        }
        338 => {
            let r444 = c.shl(c.r32(15), 0x9u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r444);
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(3, c.r64(15));
            let Some(t445) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000822d8b); };
            c.w64(15, t445);
            blk = 339; continue;
        }
        339 => {
            let r446 = c.sub(c.r64(9), c.r64(11), 64);
            c.w64(9, r446);
            c.cmp(c.r64(9), c.r64(2), 64);
            if c.cc_b() { blk = 350; continue; }
            blk = 340; continue;
        }
        340 => {
            c.w64(11, c.r64(2).wrapping_add(0x3));
            let Some(t447) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000822da2); };
            c.w64(1, t447);
            let Some(t448) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000822da9); };
            c.w64(6, t448);
            c.w64(7, c.r64(11).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(7).wrapping_add(c.r64(6)));
            let Some(t449) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000822db8); };
            c.w64(12, t449);
            let Some(t450) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000822dbf); };
            c.cmp(t450, c.r64(8), 64);
            if c.cc_be() { blk = 589; continue; }
            blk = 341; continue;
        }
        341 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x7000822dc9); }
            c.w64(8, c.r64(6).wrapping_add(0x1));
            blk = 342; continue;
        }
        342 => {
            c.w32(12, c.r32(11));
            c.w32(1, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t451) = c.ld64(c.r64(13).wrapping_add(0x250)) else { return RegfileExit::fault(0x7000822dd7); };
            c.w64(6, t451);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000822dde); }
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000822de2); }
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            c.w64(11, c.r64(11).wrapping_mul(4).wrapping_add(0x7));
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_e() { blk = 345; continue; }
            blk = 343; continue;
        }
        343 => {
            let Some(t452) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000822dfb); };
            c.w64(1, t452);
            blk = 344; continue;
        }
        344 => {
            let r453 = c.sub(c.r64(11), 0x4u64, 64);
            c.w64(11, r453);
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x7000822e06); }
            c.cmp(c.r64(11), c.r64(2), 64);
            if c.cc_ne() { blk = 344; continue; }
            blk = 345; continue;
        }
        345 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 347; continue; }
            blk = 346; continue;
        }
        346 => {
            let r454 = c.sub(c.r64(2), 0x4u64, 64);
            c.w64(2, r454);
            let Some(t455) = c.ld32(c.r64(2).wrapping_add(c.r64(15))) else { return RegfileExit::fault(0x7000822e19); };
            c.w32(11, t455);
            if !c.st32(c.r64(2).wrapping_add(c.r64(8)), c.r32(11)) { return RegfileExit::fault(0x7000822e1d); }
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_ne() { blk = 346; continue; }
            blk = 347; continue;
        }
        347 => {
            let r456 = c.r32(12) | c.r32(3);
            c.w32(12, r456);
            c.logic(r456, 32);
            c.w32(11, c.r64(12).wrapping_add(c.r64(12)));
            if !c.st32(c.r64(8).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x7000822e2e); }
            if !c.st32(c.r64(0).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000822e32); }
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 348; continue;
        }
        348 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r457 = c.r64(11) & c.r64(0);
            c.w64(11, r457);
            c.logic(r457, 64);
            let Some(t458) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822e46); };
            c.logic(t458 & 0x4u64, 8);
            if c.cc_ne() { blk = 671; continue; }
            blk = 349; continue;
        }
        349 => {
            c.w64(15, c.r64(8));
            blk = 350; continue;
        }
        350 => {
            c.w64(8, c.r64(9).wrapping_mul(4).wrapping_add(0x7));
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822e5c); };
            c.w64(9, t459);
            if !c.st32(c.r64(15).wrapping_add(c.r64(8)), c.r32(9)) { return RegfileExit::fault(0x7000822e60); }
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 351; continue;
        }
        351 => {
            c.w64(11, 0xfffffffffffc0000u64);
            let r460 = c.r64(11) & c.r64(15);
            c.w64(11, r460);
            c.logic(r460, 64);
            let Some(t461) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822e74); };
            c.logic(t461 & 0x4u64, 8);
            if c.cc_ne() { blk = 668; continue; }
            blk = 352; continue;
        }
        352 => {
            let Some(t462) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000822e7f); };
            c.w64(8, t462);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000822e83); }
            c.w64(9, 0xfffffffffffc0000u64);
            let r463 = c.r64(9) & c.r64(0);
            c.w64(9, r463);
            c.logic(r463, 64);
            let Some(t464) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822e91); };
            c.logic(t464 & 0x4u64, 8);
            if c.cc_ne() { blk = 665; continue; }
            blk = 353; continue;
        }
        353 => {
            blk = 509; continue;
        }
        354 => {
            c.w64(9, c.r64(9).wrapping_mul(4).wrapping_sub(0x1));
            c.cmp(c.r32(15), 0x0u64, 32);
            if c.cc_ne() { blk = 359; continue; }
            blk = 355; continue;
        }
        355 => {
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822eaf); };
            c.w64(11, t465);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000822eb3); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r466 = c.r64(15) & c.r64(11);
            c.w64(15, r466);
            c.logic(r466, 64);
            let Some(t467) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822ec1); };
            c.logic(t467 & 0x4u64, 8);
            if c.cc_ne() { blk = 662; continue; }
            blk = 356; continue;
        }
        356 => {
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822ecc); };
            c.w64(12, t468);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(12)) { return RegfileExit::fault(0x7000822ed0); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 358; continue; }
            blk = 357; continue;
        }
        357 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r469 = c.r64(8) & c.r64(11);
            c.w64(8, r469);
            c.logic(r469, 64);
            let Some(t470) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822ee4); };
            c.logic(t470 & 0x4u64, 8);
            if c.cc_ne() { blk = 659; continue; }
            blk = 358; continue;
        }
        358 => {
            c.w64(0, c.r64(11));
            blk = 509; continue;
        }
        359 => {
            let Some(t471) = c.ld8(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822ef7); };
            c.logic(t471 & 0x1u64, 8);
            if c.cc_e() { blk = 361; continue; }
            blk = 360; continue;
        }
        360 => {
            let Some(t472) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822efd); };
            c.w64(11, t472);
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x7000822f01); }
            blk = 362; continue;
        }
        361 => {
            let Some(t473) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000822f09); };
            c.w64(11, t473);
            let r474 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r474);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            blk = 362; continue;
        }
        362 => {
            let Some(t475) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000822f19); };
            c.w64(11, t475);
            let Some(t476) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000822f20); };
            c.w64(12, t476);
            c.w64(15, c.r64(12).wrapping_add(0xc));
            let Some(t477) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000822f28); };
            c.w64(0, t477);
            let Some(t478) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000822f2f); };
            c.cmp(t478, c.r64(15), 64);
            if c.cc_be() { blk = 591; continue; }
            blk = 363; continue;
        }
        363 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000822f38); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 364; continue;
        }
        364 => {
            let Some(t479) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000822f40); };
            c.w64(12, t479);
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x7000822f47); }
            if !c.movsd_mr(c.r64(11).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000822f4b); }
            let Some(t480) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000822f51); };
            c.w64(12, t480);
            if !c.st32(c.r64(12).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000822f55); }
            c.w64(15, 0xfffffffffffc0000u64);
            let r481 = c.r64(15) & c.r64(12);
            c.w64(15, r481);
            c.logic(r481, 64);
            let Some(t482) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822f64); };
            c.logic(t482 & 0x4u64, 8);
            if c.cc_ne() { blk = 656; continue; }
            blk = 365; continue;
        }
        365 => {
            if !c.st32(c.r64(12).wrapping_add(c.r64(9)), c.r32(11)) { return RegfileExit::fault(0x7000822f6f); }
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 367; continue; }
            blk = 366; continue;
        }
        366 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r483 = c.r64(8) & c.r64(12);
            c.w64(8, r483);
            c.logic(r483, 64);
            let Some(t484) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000822f83); };
            c.logic(t484 & 0x4u64, 8);
            if c.cc_ne() { blk = 653; continue; }
            blk = 367; continue;
        }
        367 => {
            c.w64(0, c.r64(12));
            blk = 509; continue;
        }
        368 => {
            let Some(t485) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822f96); };
            c.w32(11, t485);
            let r486 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r486);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 370; continue; }
            blk = 369; continue;
        }
        369 => {
            let Some(t487) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000822fa3); };
            c.w64(11, t487);
            blk = 370; continue;
        }
        370 => {
            let Some(t488) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000822faa); };
            c.w32(12, t488);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r489 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r489);
            let Some(t490) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000822fb8); };
            c.w64(15, t490);
            let Some(t491) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000822fbc); };
            c.w32(3, t491);
            let r492 = c.shr(c.r32(3), 0x2u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r492);
            c.w64(2, c.r64(3));
            let r493 = c.r64(2) & c.r64(12);
            c.w64(2, r493);
            c.logic(r493, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(11)) { return RegfileExit::fault(0x7000822fc9); }
            if !c.st64(c.r64(5).wrapping_sub(0xb0), c.r64(3)) { return RegfileExit::fault(0x7000822fd0); }
            c.w32(6, 0);
            c.logic(0, 32);
            blk = 372; continue;
        }
        371 => {
            let r494 = c.add(c.r64(6), 0x1u64, 64);
            c.w64(6, r494);
            c.w64(8, c.r64(6).wrapping_add(c.r64(2)));
            c.w64(3, c.r64(12));
            let r495 = c.r64(3) & c.r64(8);
            c.w64(3, r495);
            c.logic(r495, 64);
            c.w64(2, c.r64(3));
            blk = 372; continue;
        }
        372 => {
            c.w64(7, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(8, c.r64(7).wrapping_mul(4));
            let Some(t496) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000822ff9); };
            c.w32(3, t496);
            let Some(t497) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000822ffe); };
            c.cmp(t497, c.r32(3), 32);
            if c.cc_e() { blk = 389; continue; }
            blk = 373; continue;
        }
        373 => {
            c.cmp(c.r32(3), c.r32(15), 32);
            if c.cc_ne() { blk = 371; continue; }
            blk = 374; continue;
        }
        374 => {
            let Some(t498) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000823010); };
            c.w32(9, t498);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 518; continue; }
            blk = 375; continue;
        }
        375 => {
            c.logic(c.r8(9) & 0x2u64, 8);
            if c.cc_e() { blk = 378; continue; }
            blk = 376; continue;
        }
        376 => {
            c.w64(9, c.r64(7).wrapping_add(0x1));
            let Some(t499) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000823029); };
            c.w32(12, t499);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_ae() { blk = 631; continue; }
            blk = 377; continue;
        }
        377 => {
            let Some(t500) = c.ld32(c.r64(11).wrapping_add(c.r64(8)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082303c); };
            c.w32(8, t500);
            let r501 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r501);
            let Some(t502) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823044); };
            c.w64(0, t502);
            blk = 471; continue;
        }
        378 => {
            let Some(t503) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x700082304d); };
            c.cmp(t503, c.r32(15), 32);
            if c.cc_e() { blk = 613; continue; }
            blk = 379; continue;
        }
        379 => {
            let Some(t504) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x700082305a); };
            c.cmp(t504, c.r32(15), 32);
            if c.cc_e() { blk = 612; continue; }
            blk = 380; continue;
        }
        380 => {
            let Some(t505) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x7000823067); };
            c.cmp(t505, c.r32(15), 32);
            if c.cc_e() { blk = 611; continue; }
            blk = 381; continue;
        }
        381 => {
            let Some(t506) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x7000823074); };
            c.cmp(t506, c.r32(15), 32);
            if c.cc_e() { blk = 610; continue; }
            blk = 382; continue;
        }
        382 => {
            let Some(t507) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000823081); };
            c.cmp(t507, c.r32(15), 32);
            if c.cc_e() { blk = 609; continue; }
            blk = 383; continue;
        }
        383 => {
            let Some(t508) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x700082308e); };
            c.cmp(t508, c.r32(15), 32);
            if c.cc_e() { blk = 608; continue; }
            blk = 384; continue;
        }
        384 => {
            let Some(t509) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x700082309b); };
            c.cmp(t509, c.r32(15), 32);
            if c.cc_e() { blk = 607; continue; }
            blk = 385; continue;
        }
        385 => {
            let Some(t510) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008230a8); };
            c.w32(8, t510);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7).wrapping_add(0x1));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_ae() { blk = 632; continue; }
            blk = 386; continue;
        }
        386 => {
            c.w64(8, c.r64(7).wrapping_mul(4).wrapping_add(0xb));
            if !c.st32(c.r64(11).wrapping_add(c.r64(8)), c.r32(0)) { return RegfileExit::fault(0x70008230c7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 388; continue; }
            blk = 387; continue;
        }
        387 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r511 = c.r64(9) & c.r64(11);
            c.w64(9, r511);
            c.logic(r511, 64);
            let Some(t512) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008230d9); };
            c.logic(t512 & 0x4u64, 8);
            if c.cc_ne() { blk = 650; continue; }
            blk = 388; continue;
        }
        388 => {
            blk = 509; continue;
        }
        389 => {
            let Some(t513) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008230e9); };
            c.cmp(t513, 0x82eu64, 16);
            if c.cc_e() { blk = 629; continue; }
            blk = 390; continue;
        }
        390 => {
            let Some(t514) = c.ld32(c.r64(13).wrapping_add(0x608)) else { return RegfileExit::fault(0x70008230f5); };
            c.cmp(t514, c.r32(15), 32);
            if c.cc_e() { blk = 628; continue; }
            blk = 391; continue;
        }
        391 => {
            let Some(t515) = c.ld32(c.r64(13).wrapping_add(0xcb8)) else { return RegfileExit::fault(0x7000823102); };
            c.cmp(t515, c.r32(15), 32);
            if c.cc_e() { blk = 627; continue; }
            blk = 392; continue;
        }
        392 => {
            let Some(t516) = c.ld32(c.r64(13).wrapping_add(0x8b8)) else { return RegfileExit::fault(0x700082310f); };
            c.cmp(t516, c.r32(15), 32);
            if c.cc_e() { blk = 626; continue; }
            blk = 393; continue;
        }
        393 => {
            let Some(t517) = c.ld32(c.r64(13).wrapping_add(0xce8)) else { return RegfileExit::fault(0x700082311c); };
            c.cmp(t517, c.r32(15), 32);
            if c.cc_e() { blk = 625; continue; }
            blk = 394; continue;
        }
        394 => {
            let Some(t518) = c.ld32(c.r64(13).wrapping_add(0xd10)) else { return RegfileExit::fault(0x7000823129); };
            c.cmp(t518, c.r32(15), 32);
            if c.cc_e() { blk = 624; continue; }
            blk = 395; continue;
        }
        395 => {
            let Some(t519) = c.ld32(c.r64(13).wrapping_add(0x9e0)) else { return RegfileExit::fault(0x7000823136); };
            c.cmp(t519, c.r32(15), 32);
            if c.cc_e() { blk = 623; continue; }
            blk = 396; continue;
        }
        396 => {
            let Some(t520) = c.ld32(c.r64(13).wrapping_add(0xac0)) else { return RegfileExit::fault(0x7000823143); };
            c.cmp(t520, c.r32(15), 32);
            if c.cc_e() { blk = 622; continue; }
            blk = 397; continue;
        }
        397 => {
            let Some(t521) = c.ld32(c.r64(15).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000823150); };
            c.w32(8, t521);
            let Some(t522) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000823154); };
            c.cmp(t522, c.r32(8), 32);
            if c.cc_ne() { blk = 401; continue; }
            blk = 398; continue;
        }
        398 => {
            let Some(t523) = c.ld32(c.r64(15).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082315d); };
            c.w32(8, t523);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 401; continue; }
            blk = 399; continue;
        }
        399 => {
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 614; continue; }
            blk = 400; continue;
        }
        400 => {
            blk = 402; continue;
        }
        401 => {
            c.logic(c.r32(9) & 0x8000000u64, 32);
            if c.cc_e() { blk = 615; continue; }
            blk = 402; continue;
        }
        402 => {
            let Some(t524) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000823180); };
            c.w32(8, t524);
            let r525 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r525);
            let Some(t526) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000823187); };
            c.w32(12, t526);
            let r527 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r527);
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0xb0)) else { return RegfileExit::fault(0x700082318e); };
            c.w64(3, t528);
            blk = 404; continue;
        }
        403 => {
            let Some(t529) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000823197); };
            c.w32(12, t529);
            let r530 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r530);
            blk = 404; continue;
        }
        404 => {
            let Some(t531) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x700082319e); };
            c.cmp(t531, c.r32(8), 32);
            if c.cc_e() { blk = 479; continue; }
            blk = 405; continue;
        }
        405 => {
            let Some(t532) = c.ld16(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008231ab); };
            c.w32(2, t532);
            c.cmp(c.r16(2), 0x410u64, 16);
            if c.cc_be() { blk = 434; continue; }
            blk = 406; continue;
        }
        406 => {
            let Some(t533) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008231bc); };
            c.w32(6, t533);
            c.logic(c.r32(6) & 0x200000u64, 32);
            if c.cc_ne() { blk = 427; continue; }
            blk = 407; continue;
        }
        407 => {
            let Some(t534) = c.ld32(c.r64(12).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008231cd); };
            c.w32(7, t534);
            let r535 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r535);
            c.logic(c.r32(6) & 0xffc00u64, 32);
            if c.cc_e() { blk = 445; continue; }
            blk = 408; continue;
        }
        408 => {
            let r536 = c.shr(c.r32(6), 0xau64, 32).unwrap_or(c.r32(6));
            c.w32(6, r536);
            let r537 = c.r32(6) & 0x3ffu64;
            c.w32(6, r537);
            c.logic(r537, 32);
            c.cmp(c.r32(6), 0x20u64, 32);
            if c.cc_be() { blk = 423; continue; }
            blk = 409; continue;
        }
        409 => {
            let Some(t538) = c.ld16(c.r64(7).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008231f3); };
            c.w32(11, (t538 as u16 as i16 as i64 as u64));
            let r539 = c.sub(c.r32(11), 0x1u64, 32);
            c.w32(11, r539);
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(11)) { return RegfileExit::fault(0x70008231fc); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 411; continue;
        }
        410 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(1)) { return RegfileExit::fault(0x7000823207); }
            blk = 411; continue;
        }
        411 => {
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x700082320e); };
            c.w64(1, t540);
            let r541 = c.sub(c.r32(1), c.r32(0), 32);
            c.w32(1, r541);
            let r542 = c.shr(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r542);
            let r543 = c.add(c.r32(1), c.r32(0), 32);
            c.w32(1, r543);
            c.w32(9, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t544) = c.ld32(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000823222); };
            c.w32(9, t544);
            let r545 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r545);
            let r546 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r546);
            let r547 = c.r32(9) & 0x3ffu64;
            c.w32(9, r547);
            c.logic(r547, 32);
            c.w32(9, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t548) = c.ld32(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082323c); };
            c.w32(9, t548);
            let r549 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r549);
            let Some(t550) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000823244); };
            c.w32(9, t550);
            let r551 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r551);
            c.cmp(c.r32(3), c.r32(9), 32);
            if c.cc_be() { blk = 413; continue; }
            blk = 412; continue;
        }
        412 => {
            c.w32(0, c.r64(1).wrapping_add(0x1));
            let Some(t552) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000823254); };
            c.w64(1, t552);
            blk = 413; continue;
        }
        413 => {
            c.cmp(c.r32(0), c.r32(1), 32);
            if c.cc_ne() { blk = 410; continue; }
            blk = 414; continue;
        }
        414 => {
            if !c.st64(c.r64(5).wrapping_sub(0xa8), c.r64(12)) { return RegfileExit::fault(0x700082325f); }
            blk = 416; continue;
        }
        415 => {
            let r553 = c.add(c.r32(0), 0x1u64, 32);
            c.w32(0, r553);
            blk = 416; continue;
        }
        416 => {
            c.cmp(c.r32(11), c.r32(0), 32);
            if c.cc_l() { blk = 422; continue; }
            blk = 417; continue;
        }
        417 => {
            c.w32(9, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)));
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t554) = c.ld32(c.r64(7).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000823277); };
            c.w32(9, t554);
            let r555 = c.sar(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r555);
            let r556 = c.shr(c.r32(9), 0x9u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r556);
            let r557 = c.r32(9) & 0x3ffu64;
            c.w32(9, r557);
            c.logic(r557, 32);
            c.w32(1, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t558) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000823291); };
            c.w32(12, t558);
            let r559 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r559);
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(1)) { return RegfileExit::fault(0x7000823299); }
            let Some(t560) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008232a0); };
            c.w32(1, t560);
            let r561 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r561);
            c.cmp(c.r32(3), c.r32(1), 32);
            if c.cc_ne() { blk = 421; continue; }
            blk = 418; continue;
        }
        418 => {
            c.cmp(c.r32(12), c.r32(15), 32);
            if c.cc_ne() { blk = 415; continue; }
            blk = 419; continue;
        }
        419 => {
            c.cmp(c.r32(6), c.r32(9), 32);
            if c.cc_a() { blk = 448; continue; }
            blk = 420; continue;
        }
        420 => {
            let Some(t562) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008232ba); };
            c.w64(12, t562);
            blk = 445; continue;
        }
        421 => {
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008232c6); };
            c.w64(12, t563);
            blk = 445; continue;
        }
        422 => {
            let Some(t564) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x70008232d2); };
            c.w64(12, t564);
            blk = 445; continue;
        }
        423 => {
            c.w64(9, (c.r32(6) as u32 as i32 as i64 as u64));
            let r565 = c.imul2(c.r64(9), 0x3u64, 64);
            c.w64(1, r565);
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_e() { blk = 445; continue; }
            blk = 424; continue;
        }
        424 => {
            let r566 = c.sub(c.r64(1), 0x3u64, 64);
            c.w64(1, r566);
            let Some(t567) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008232f3); };
            c.cmp(t567, c.r32(15), 32);
            if c.cc_e() { blk = 449; continue; }
            blk = 425; continue;
        }
        425 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_ne() { blk = 424; continue; }
            blk = 426; continue;
        }
        426 => {
            blk = 445; continue;
        }
        427 => {
            let Some(t568) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000823309); };
            c.w32(9, t568);
            let r569 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r569);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 429; continue; }
            blk = 428; continue;
        }
        428 => {
            let Some(t570) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x7000823316); };
            c.w64(9, t570);
            blk = 429; continue;
        }
        429 => {
            let Some(t571) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x700082331d); };
            c.w32(11, t571);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r572 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r572);
            c.w64(0, c.r64(3));
            let r573 = c.r64(0) & c.r64(11);
            c.w64(0, r573);
            c.logic(r573, 64);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 431; continue;
        }
        430 => {
            let r574 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r574);
            let r575 = c.add(c.r64(0), c.r64(1), 64);
            c.w64(0, r575);
            c.w64(6, c.r64(11));
            let r576 = c.r64(6) & c.r64(0);
            c.w64(6, r576);
            c.logic(r576, 64);
            c.w64(0, c.r64(6));
            blk = 431; continue;
        }
        431 => {
            c.w64(6, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(7, c.r64(6).wrapping_mul(4));
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(6)) { return RegfileExit::fault(0x7000823352); }
            let Some(t577) = c.ld32(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000823359); };
            c.w32(6, t577);
            let Some(t578) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082335e); };
            c.cmp(t578, c.r32(6), 32);
            if c.cc_e() { blk = 445; continue; }
            blk = 432; continue;
        }
        432 => {
            c.cmp(c.r32(6), c.r32(15), 32);
            if c.cc_e() { blk = 464; continue; }
            blk = 433; continue;
        }
        433 => {
            blk = 430; continue;
        }
        434 => {
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(8)) { return RegfileExit::fault(0x7000823376); }
            c.cmp(c.r16(2), 0xbdu64, 16);
            if c.cc_ne() { blk = 618; continue; }
            blk = 435; continue;
        }
        435 => {
            let Some(t579) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000823388); };
            c.logic(t579 & 0x24u64, 8);
            if c.cc_ne() { blk = 621; continue; }
            blk = 436; continue;
        }
        436 => {
            let Some(t580) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000823394); };
            c.w32(9, t580);
            let r581 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r581);
            c.logic(c.r8(9) & 0x1u64, 8);
            if c.cc_ne() { blk = 438; continue; }
            blk = 437; continue;
        }
        437 => {
            let Some(t582) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008233a1); };
            c.w64(9, t582);
            blk = 438; continue;
        }
        438 => {
            let Some(t583) = c.ld32(c.r64(9).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008233a8); };
            c.w32(11, t583);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r584 = c.sub(c.r64(11), 0x1u64, 64);
            c.w64(11, r584);
            c.w64(0, c.r64(3));
            let r585 = c.r64(0) & c.r64(11);
            c.w64(0, r585);
            c.logic(r585, 64);
            c.w32(1, 0);
            c.logic(0, 32);
            blk = 440; continue;
        }
        439 => {
            let r586 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r586);
            c.w64(8, c.r64(1).wrapping_add(c.r64(0)));
            c.w64(0, c.r64(11));
            let r587 = c.r64(0) & c.r64(8);
            c.w64(0, r587);
            c.logic(r587, 64);
            blk = 440; continue;
        }
        440 => {
            c.w64(6, c.r64(0).wrapping_add(0x5));
            c.w64(7, c.r64(6).wrapping_mul(4));
            let Some(t588) = c.ld32(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008233da); };
            c.w32(8, t588);
            let r589 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r589);
            let Some(t590) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008233e2); };
            c.cmp(t590, c.r32(8), 32);
            if c.cc_e() { blk = 445; continue; }
            blk = 441; continue;
        }
        441 => {
            let Some(t591) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008233eb); };
            c.cmp(t591, c.r32(8), 32);
            if c.cc_e() { blk = 439; continue; }
            blk = 442; continue;
        }
        442 => {
            let Some(t592) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008233f4); };
            c.cmp(t592, c.r32(15), 32);
            if c.cc_ne() { blk = 439; continue; }
            blk = 443; continue;
        }
        443 => {
            let Some(t593) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008233fa); };
            c.w32(8, t593);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.w64(11, c.r64(6).wrapping_add(0x1));
            c.cmp(c.r64(11), c.r64(8), 64);
            if c.cc_ae() { blk = 634; continue; }
            blk = 444; continue;
        }
        444 => {
            let Some(t594) = c.ld32(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000823411); };
            c.w32(8, t594);
            let r595 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r595);
            let Some(t596) = c.ld32(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x7000823419); };
            c.w32(9, t596);
            let r597 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r597);
            let Some(t598) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000823420); };
            c.cmp(t598, c.r32(9), 32);
            if c.cc_ne() { blk = 468; continue; }
            blk = 445; continue;
        }
        445 => {
            c.cmp(c.r16(2), 0x82eu64, 16);
            if c.cc_e() { blk = 620; continue; }
            blk = 446; continue;
        }
        446 => {
            let Some(t599) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000823438); };
            c.w32(8, t599);
            let r600 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r600);
            let Some(t601) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x7000823440); };
            c.cmp(t601, c.r32(8), 32);
            if c.cc_e() { blk = 479; continue; }
            blk = 447; continue;
        }
        447 => {
            blk = 403; continue;
        }
        448 => {
            let Some(t602) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000823452); };
            c.w64(1, t602);
            let Some(t603) = c.ld64(c.r64(5).wrapping_sub(0xa8)) else { return RegfileExit::fault(0x7000823459); };
            c.w64(12, t603);
            blk = 449; continue;
        }
        449 => {
            c.w64(9, c.r64(1).wrapping_mul(4));
            let Some(t604) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000823468); };
            c.w32(11, t604);
            c.w32(0, c.r32(11));
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 518; continue; }
            blk = 450; continue;
        }
        450 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 479; continue; }
            blk = 451; continue;
        }
        451 => {
            c.logic(c.r8(11) & 0x4u64, 8);
            if c.cc_e() { blk = 453; continue; }
            blk = 452; continue;
        }
        452 => {
            let Some(t605) = c.ld32(c.r64(7).wrapping_add(c.r64(9)).wrapping_add(0x17)) else { return RegfileExit::fault(0x700082348a); };
            c.w32(9, t605);
            let r606 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r606);
            c.w64(0, c.r64(8));
            c.w64(8, c.r64(9));
            blk = 471; continue;
        }
        453 => {
            let r607 = c.sar(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r607);
            if let Some(sr) = c.shr(c.r64(11), 0x13u64, 64) { c.w64(11, sr); }
            let Some(t608) = c.ld8(c.r64(12).wrapping_add(0x4)) else { return RegfileExit::fault(0x70008234a4); };
            c.w32(9, t608);
            let r609 = c.r64(11) & 0x3ffu64;
            c.w64(11, r609);
            c.logic(r609, 64);
            let r610 = c.r32(0) & 0x380u64;
            c.w32(0, r610);
            c.logic(r610, 32);
            let r611 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r611);
            c.cmp(c.r32(0), 0x100u64, 32);
            c.w8(11, (c.cc_ne() as u64));
            c.w32(11, c.r8(11));
            let Some(t612) = c.ld8(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008234c6); };
            c.w32(12, t612);
            c.cmp(c.r64(9), c.r64(12), 64);
            if c.cc_b() { blk = 458; continue; }
            blk = 454; continue;
        }
        454 => {
            c.w64(0, c.r64(8));
            let Some(t613) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008234d4); };
            c.w32(8, t613);
            let r614 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r614);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 456; continue; }
            blk = 455; continue;
        }
        455 => {
            let Some(t615) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008234e1); };
            c.w64(8, t615);
            blk = 456; continue;
        }
        456 => {
            let r616 = c.sub(c.r64(9), c.r64(12), 64);
            c.w64(9, r616);
            let Some(t617) = c.ld32(c.r64(8).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008234eb); };
            c.w32(8, t617);
            let r618 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r618);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 471; continue; }
            blk = 457; continue;
        }
        457 => {
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x70008234fd); }
            blk = 460; continue;
        }
        458 => {
            c.w64(12, c.r64(8));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 463; continue; }
            blk = 459; continue;
        }
        459 => {
            let Some(t619) = c.ld32(c.r64(12).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082350e); };
            c.w32(8, t619);
            let r620 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r620);
            if !c.movsd_rm(0, c.r64(8).wrapping_add(0x3)) { return RegfileExit::fault(0x7000823516); }
            c.w64(0, c.r64(12));
            blk = 460; continue;
        }
        460 => {
            let Some(t621) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082351f); };
            c.w64(8, t621);
            let Some(t622) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000823526); };
            c.w64(9, t622);
            c.w64(11, c.r64(9).wrapping_add(0xc));
            let Some(t623) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082352d); };
            c.w64(12, t623);
            let Some(t624) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000823534); };
            c.cmp(t624, c.r64(11), 64);
            if c.cc_be() { blk = 616; continue; }
            blk = 461; continue;
        }
        461 => {
            if !c.st64(c.r64(8), c.r64(11)) { return RegfileExit::fault(0x700082353e); }
            c.w64(8, c.r64(9).wrapping_add(0x1));
            blk = 462; continue;
        }
        462 => {
            let Some(t625) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x7000823545); };
            c.w64(9, t625);
            if !c.st32(c.r64(8).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x700082354c); }
            if !c.movsd_mr(c.r64(8).wrapping_add(0x3), 0) { return RegfileExit::fault(0x7000823550); }
            blk = 471; continue;
        }
        463 => {
            c.w64(0, c.r64(12));
            let Some(t626) = c.ld32(c.r64(0).wrapping_add(c.r64(9).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x700082355b); };
            c.w32(8, t626);
            let r627 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r627);
            blk = 471; continue;
        }
        464 => {
            c.w64(11, c.r64(8));
            let Some(t628) = c.ld32(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000823568); };
            c.w32(8, t628);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 518; continue; }
            blk = 465; continue;
        }
        465 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 479; continue; }
            blk = 466; continue;
        }
        466 => {
            let Some(t629) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000823581); };
            c.w32(8, t629);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let Some(t630) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x700082358b); };
            c.w64(12, t630);
            let r631 = c.add(c.r64(12), 0x1u64, 64);
            c.w64(12, r631);
            c.cmp(c.r64(12), c.r64(8), 64);
            if c.cc_ae() { blk = 633; continue; }
            blk = 467; continue;
        }
        467 => {
            let Some(t632) = c.ld32(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0xb)) else { return RegfileExit::fault(0x700082359f); };
            c.w32(8, t632);
            let r633 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r633);
            c.w64(0, c.r64(11));
            blk = 471; continue;
        }
        468 => {
            let Some(t634) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008235ac); };
            c.w32(8, t634);
            c.logic(c.r8(8) & 0x10u64, 8);
            if c.cc_ne() { blk = 518; continue; }
            blk = 469; continue;
        }
        469 => {
            c.logic(c.r8(8) & 0x2u64, 8);
            if c.cc_e() { blk = 479; continue; }
            blk = 470; continue;
        }
        470 => {
            let Some(t635) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x70008235c4); };
            c.w64(0, t635);
            c.w64(8, c.r64(9));
            blk = 471; continue;
        }
        471 => {
            let Some(t636) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008235ce); };
            c.w32(9, t636);
            let Some(t637) = c.ld32(c.r64(13).wrapping_add(0xd60)) else { return RegfileExit::fault(0x70008235d2); };
            c.cmp(t637, c.r32(9), 32);
            if c.cc_ne() { blk = 473; continue; }
            blk = 472; continue;
        }
        472 => {
            let Some(t638) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008235db); };
            c.w64(0, t638);
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008235df); };
            c.w64(12, t639);
            blk = 508; continue;
        }
        473 => {
            let Some(t640) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008235e8); };
            c.w32(7, t640);
            let r641 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r641);
            let Some(t642) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008235ef); };
            c.w32(8, t642);
            let r643 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r643);
            let Some(t644) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008235f6); };
            c.cmp(t644, c.r32(8), 32);
            if c.cc_e() { blk = 619; continue; }
            blk = 474; continue;
        }
        474 => {
            let Some(t645) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x7000823603); };
            c.logic(t645 & 0x2u64, 8);
            if c.cc_ne() { blk = 477; continue; }
            blk = 475; continue;
        }
        475 => {
            c.w32(8, 0xbcu64);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000823610); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x7000823612); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x7000823614); }
            let Some(t646) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x7000823615); };
            c.w64(3, t646);
            c.w32(0, 0x3u64);
            let Some(t647) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000823621); };
            c.w64(6, t647);
            return c.call(0x700086ed74, 0x700082362a, 0x7000823625);
        }
        476 => {
            blk = 509; continue;
        }
        477 => {
            let Some(t648) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x700082362f); };
            if !c.push64(t648) { return RegfileExit::fault(0x700082362f); }
            let Some(t649) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823632); };
            if !c.push64(t649) { return RegfileExit::fault(0x7000823632); }
            let Some(t650) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000823635); };
            c.w64(6, t650);
            c.w32(0, 0x1u64);
            return c.call(0x70007f21d4, 0x7000823643, 0x700082363e);
        }
        478 => {
            blk = 509; continue;
        }
        479 => {
            let Some(t651) = c.ld32(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000823648); };
            c.logic(t651 & 0x100000u64, 32);
            if c.cc_e() { blk = 483; continue; }
            blk = 480; continue;
        }
        480 => {
            let Some(t652) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000823654); };
            c.w64(1, t652);
            let Some(t653) = c.ld32(c.r64(1).wrapping_add(0x23)) else { return RegfileExit::fault(0x7000823658); };
            c.w32(8, t653);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 483; continue; }
            blk = 481; continue;
        }
        481 => {
            c.w64(10, c.r64(4));
            let r654 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r654);
            let r655 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r655);
            c.logic(r655, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x700082366d); }
            let Some(t656) = c.ld64(c.r64(13).wrapping_add(0x1390)) else { return RegfileExit::fault(0x7000823672); };
            c.w64(0, t656);
            c.w64(10, 0x7000823680u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x7000823680); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x7000823684); }
            return c.call(c.r64(0), 0x700082368a, 0x7000823688);
        }
        482 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x700082368a); }
            let Some(t657) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x7000823692); };
            c.w64(4, t657);
            let Some(t658) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823697); };
            c.w64(15, t658);
            blk = 483; continue;
        }
        483 => {
            let Some(t659) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x700082369b); };
            c.w64(11, t659);
            let Some(t660) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008236a2); };
            c.w32(8, t660);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let Some(t661) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008236a9); };
            c.w32(9, t661);
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_be() { blk = 636; continue; }
            blk = 484; continue;
        }
        484 => {
            let Some(t662) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008236ba); };
            c.w32(12, t662);
            let r663 = c.add(c.r32(12), 0x2u64, 32);
            c.w32(12, r663);
            c.w64(0, c.r64(12));
            let r664 = c.shr(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r664);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r665 = c.r64(0) & 0xfffffffffffffffeu64;
            c.w64(0, r665);
            c.logic(r665, 64);
            let r666 = c.add(c.r32(0), c.r32(12), 32);
            c.w32(0, r666);
            c.w32(3, c.r32(9));
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_b() { blk = 506; continue; }
            blk = 485; continue;
        }
        485 => {
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 637; continue; }
            blk = 486; continue;
        }
        486 => {
            let r667 = c.sub(c.r32(3), c.r32(12), 32);
            c.w32(3, r667);
            let r668 = c.shr(c.r32(3), 1u64, 32).unwrap_or(c.r32(3));
            c.w32(3, r668);
            c.w64(9, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t669) = c.ld32(c.r64(11).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008236ef); };
            c.w32(0, t669);
            let r670 = c.r64(9) & 0xfffffffffffffffeu64;
            c.w64(9, r670);
            c.logic(r670, 64);
            c.cmp(c.r32(9), c.r32(0), 32);
            if c.cc_b() { blk = 506; continue; }
            blk = 487; continue;
        }
        487 => {
            c.cmp(c.r64(8), 0x3u64, 64);
            if c.cc_be() { blk = 638; continue; }
            blk = 488; continue;
        }
        488 => {
            let Some(t671) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082370a); };
            c.w32(8, t671);
            let r672 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r672);
            c.w32(9, c.r64(8).wrapping_add(0x2));
            c.cmp(c.r32(9), 0xfffffeu64, 32);
            if c.cc_a() { blk = 506; continue; }
            blk = 489; continue;
        }
        489 => {
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0x13), c.r32(9)) { return RegfileExit::fault(0x7000823725); }
            let Some(t673) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000823729); };
            c.w32(9, t673);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_be() { blk = 639; continue; }
            blk = 490; continue;
        }
        490 => {
            c.w64(9, (c.r32(12) as u32 as i32 as i64 as u64));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x7000823740); }
            let Some(t674) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000823744); };
            c.w32(9, t674);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r675 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r675);
            c.w64(12, c.r64(15));
            let Some(t676) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000823755); };
            c.w32(15, t676);
            let r677 = c.shr(c.r32(15), 0x2u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r677);
            let r678 = c.r64(15) & c.r64(9);
            c.w64(15, r678);
            c.logic(r678, 64);
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 492; continue;
        }
        491 => {
            let r679 = c.add(c.r64(0), 0x1u64, 64);
            c.w64(0, r679);
            let r680 = c.add(c.r64(15), c.r64(0), 64);
            c.w64(15, r680);
            c.w64(3, c.r64(9));
            let r681 = c.r64(3) & c.r64(15);
            c.w64(3, r681);
            c.logic(r681, 64);
            c.w64(15, c.r64(3));
            blk = 492; continue;
        }
        492 => {
            c.w64(3, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(2, c.r64(3).wrapping_mul(4));
            c.w64(1, c.r64(3).wrapping_mul(4).wrapping_add(0x7));
            let Some(t682) = c.ld32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082378a); };
            c.w32(6, t682);
            let Some(t683) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x700082378f); };
            c.cmp(t683, c.r32(6), 32);
            if c.cc_e() { blk = 494; continue; }
            blk = 493; continue;
        }
        493 => {
            let Some(t684) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000823798); };
            c.cmp(t684, c.r32(6), 32);
            if c.cc_ne() { blk = 491; continue; }
            blk = 494; continue;
        }
        494 => {
            let Some(t685) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008237a1); };
            c.w32(9, t685);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(3), c.r64(9), 64);
            if c.cc_ae() { blk = 640; continue; }
            blk = 495; continue;
        }
        495 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(1)), c.r32(12)) { return RegfileExit::fault(0x70008237b4); }
            c.logic(c.r8(12) & 0x1u64, 8);
            if c.cc_e() { blk = 497; continue; }
            blk = 496; continue;
        }
        496 => {
            c.w64(9, 0xfffffffffffc0000u64);
            let r686 = c.r64(9) & c.r64(11);
            c.w64(9, r686);
            c.logic(r686, 64);
            let Some(t687) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008237c8); };
            c.logic(t687 & 0x4u64, 8);
            if c.cc_ne() { blk = 647; continue; }
            blk = 497; continue;
        }
        497 => {
            let Some(t688) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008237d3); };
            c.w32(9, t688);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(15, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(15), c.r64(9), 64);
            if c.cc_ae() { blk = 641; continue; }
            blk = 498; continue;
        }
        498 => {
            c.w64(9, c.r64(3).wrapping_mul(4).wrapping_add(0xb));
            if let Some(sr) = c.shl(c.r64(8), 0x8u64, 64) { c.w64(8, sr); }
            let Some(t689) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008237f6); };
            c.w64(15, t689);
            if !c.st32(c.r64(11).wrapping_add(c.r64(9)), c.r32(15)) { return RegfileExit::fault(0x70008237fa); }
            c.logic(c.r8(15) & 0x1u64, 8);
            if c.cc_e() { blk = 500; continue; }
            blk = 499; continue;
        }
        499 => {
            c.w64(0, 0xfffffffffffc0000u64);
            let r690 = c.r64(0) & c.r64(11);
            c.w64(0, r690);
            c.logic(r690, 64);
            let Some(t691) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082380e); };
            c.logic(t691 & 0x4u64, 8);
            if c.cc_ne() { blk = 644; continue; }
            blk = 500; continue;
        }
        500 => {
            let Some(t692) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x7000823818); };
            c.w32(9, t692);
            c.w32(15, c.r32(8));
            let Some(t693) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x7000823820); };
            c.cmp(t693, c.r32(9), 32);
            if c.cc_ne() { blk = 503; continue; }
            blk = 501; continue;
        }
        501 => {
            let Some(t694) = c.ld8(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x7000823829); };
            c.logic(t694 & 0x1u64, 8);
            if c.cc_e() { blk = 503; continue; }
            blk = 502; continue;
        }
        502 => {
            let r695 = c.r32(15) | 0x20u64;
            c.w32(15, r695);
            c.logic(r695, 32);
            c.w64(8, (c.r32(15) as u32 as i32 as i64 as u64));
            blk = 504; continue;
        }
        503 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            blk = 504; continue;
        }
        504 => {
            let Some(t696) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082383d); };
            c.w32(9, t696);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.w64(12, c.r64(3).wrapping_add(0x2));
            c.cmp(c.r64(12), c.r64(9), 64);
            if c.cc_ae() { blk = 642; continue; }
            blk = 505; continue;
        }
        505 => {
            if !c.st32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0xf), c.r32(8)) { return RegfileExit::fault(0x7000823854); }
            blk = 509; continue;
        }
        506 => {
            let Some(t697) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082385b); };
            if !c.push64(t697) { return RegfileExit::fault(0x700082385b); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700082385e); }
            let Some(t698) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823860); };
            if !c.push64(t698) { return RegfileExit::fault(0x7000823860); }
            let Some(t699) = c.ld64(c.r64(13).wrapping_add(0x2300)) else { return RegfileExit::fault(0x7000823863); };
            c.w64(3, t699);
            c.w32(0, 0x3u64);
            let Some(t700) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082386f); };
            c.w64(6, t700);
            return c.call(0x700086ed74, 0x7000823878, 0x7000823873);
        }
        507 => {
            blk = 509; continue;
        }
        508 => {
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x700082387a); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x700082387b); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x700082387d); }
            let Some(t701) = c.ld64(c.r64(13).wrapping_add(0x2470)) else { return RegfileExit::fault(0x700082387f); };
            c.w64(3, t701);
            c.w32(0, 0x3u64);
            let Some(t702) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082388b); };
            c.w64(6, t702);
            return c.call(0x700086ed74, 0x7000823894, 0x700082388f);
        }
        509 => {
            let Some(t703) = c.ld32(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x7000823894); };
            c.cmp(t703, 0x0u64, 32);
            if c.cc_ne() { blk = 511; continue; }
            blk = 510; continue;
        }
        510 => {
            let Some(t704) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x700082389a); };
            c.w64(7, t704);
            let Some(t705) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082389e); };
            c.w64(3, t705);
            blk = 512; continue;
        }
        511 => {
            let Some(t706) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008238a4); };
            c.w64(8, t706);
            let Some(t707) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008238a8); };
            c.w64(9, t707);
            let Some(t708) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008238ac); };
            c.cmp(t708, c.r32(9), 32);
            c.w8(15, (c.cc_e() as u64));
            c.w32(15, c.r8(15));
            let Some(t709) = c.ld32(c.r64(9).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008238b8); };
            c.w32(11, t709);
            let r710 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r710);
            let Some(t711) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008238bf); };
            c.w64(7, t711);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008238c3); }
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(11)) { return RegfileExit::fault(0x70008238c7); }
            c.w64(3, c.r64(8));
            blk = 512; continue;
        }
        512 => {
            let r712 = c.add(c.r64(7), 0x3u64, 64);
            c.w64(7, r712);
            let Some(t713) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008238d2); };
            c.cmp(c.r64(7), t713, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 513; continue;
        }
        513 => {
            let Some(t714) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008238dc); };
            c.w64(15, t714);
            let Some(t715) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008238e0); };
            c.w64(1, t715);
            let Some(t716) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008238e4); };
            c.w64(7, t716);
            let Some(t717) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008238e8); };
            c.w64(9, t717);
            blk = 515; continue;
        }
        514 => {
            let Some(t718) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008238ee); };
            c.w64(9, t718);
            blk = 515; continue;
        }
        515 => {
            let Some(t719) = c.ld32(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008238f2); };
            c.cmp(t719, 0x0u64, 32);
            if c.cc_ne() { blk = 535; continue; }
            blk = 516; continue;
        }
        516 => {
            let Some(t720) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008238fc); };
            c.w64(8, t720);
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 11; continue; }
            blk = 517; continue;
        }
        517 => {
            blk = 535; continue;
        }
        518 => {
            let Some(t721) = c.ld8(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082390f); };
            c.logic(t721 & 0x1u64, 8);
            if c.cc_e() { blk = 635; continue; }
            blk = 519; continue;
        }
        519 => {
            let Some(t722) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000823919); };
            c.w64(8, t722);
            let Some(t723) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x700082391d); };
            c.cmp(t723, c.r32(8), 32);
            if c.cc_e() { blk = 635; continue; }
            blk = 520; continue;
        }
        520 => {
            let Some(t724) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x700082392a); };
            c.w32(9, t724);
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_e() { blk = 532; continue; }
            blk = 521; continue;
        }
        521 => {
            let Some(t725) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x700082393a); };
            c.w32(8, t725);
            let r726 = c.r32(8) & 0x12u64;
            c.w32(8, r726);
            c.logic(r726, 32);
            c.cmp(c.r32(8), 0x2u64, 32);
            if c.cc_e() { blk = 531; continue; }
            blk = 522; continue;
        }
        522 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_e() { blk = 524; continue; }
            blk = 523; continue;
        }
        523 => {
            let Some(t727) = c.ld64(c.r64(13).wrapping_add(0xb30)) else { return RegfileExit::fault(0x700082394f); };
            c.w64(8, t727);
            blk = 533; continue;
        }
        524 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 530; continue; }
            blk = 525; continue;
        }
        525 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_b() { blk = 529; continue; }
            blk = 526; continue;
        }
        526 => {
            c.cmp(c.r16(9), 0x41u64, 16);
            if c.cc_e() { blk = 528; continue; }
            blk = 527; continue;
        }
        527 => {
            let Some(t728) = c.ld64(c.r64(13).wrapping_add(0xaa8)) else { return RegfileExit::fault(0x700082396e); };
            c.w64(8, t728);
            blk = 533; continue;
        }
        528 => {
            let Some(t729) = c.ld64(c.r64(13).wrapping_add(0x550)) else { return RegfileExit::fault(0x7000823977); };
            c.w64(8, t729);
            blk = 533; continue;
        }
        529 => {
            let Some(t730) = c.ld64(c.r64(13).wrapping_add(0xa88)) else { return RegfileExit::fault(0x7000823980); };
            c.w64(8, t730);
            blk = 533; continue;
        }
        530 => {
            let Some(t731) = c.ld64(c.r64(13).wrapping_add(0x918)) else { return RegfileExit::fault(0x7000823989); };
            c.w64(8, t731);
            blk = 533; continue;
        }
        531 => {
            let Some(t732) = c.ld64(c.r64(13).wrapping_add(0x730)) else { return RegfileExit::fault(0x7000823992); };
            c.w64(8, t732);
            blk = 533; continue;
        }
        532 => {
            let Some(t733) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082399b); };
            c.w64(8, t733);
            let Some(t734) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x700082399f); };
            c.w32(9, t734);
            let r735 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r735);
            c.w64(8, c.r64(9));
            blk = 533; continue;
        }
        533 => {
            c.w32(9, 0x15au64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008239af); }
            if !c.push64(c.r64(15)) { return RegfileExit::fault(0x70008239b1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008239b3); }
            let Some(t736) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008239b5); };
            if !c.push64(t736) { return RegfileExit::fault(0x70008239b5); }
            let Some(t737) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008239b8); };
            c.w64(3, t737);
            c.w32(0, 0x4u64);
            let Some(t738) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008239c4); };
            c.w64(6, t738);
            return c.call(0x700086ed74, 0x70008239cd, 0x70008239c8);
        }
        534 => {
            return RegfileExit::fault(0x70008239cd);
        }
        535 => {
            let Some(t739) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008239ce); };
            c.w64(0, t739);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008239d8); }
            return c.ret(0x0, 0x70008239d9);
        }
        536 => {
            c.cmp(c.r16(9), 0xbau64, 16);
            if c.cc_ae() { blk = 643; continue; }
            blk = 537; continue;
        }
        537 => {
            c.cmp(c.r16(9), 0x40u64, 16);
            if c.cc_ae() { blk = 535; continue; }
            blk = 538; continue;
        }
        538 => {
            let Some(t740) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008239ed); };
            c.w32(8, t740);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 535; continue; }
            blk = 539; continue;
        }
        539 => {
            blk = 643; continue;
        }
        540 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008239fc); }
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 0) { return RegfileExit::fault(0x7000823a05); }
            c.w64(8, c.r64(2));
            return c.call(0x70007febf4, 0x7000823a12, 0x7000823a0d);
        }
        541 => {
            c.w64(8, c.r64(0));
            let Some(t741) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000823a15); };
            c.w64(3, t741);
            let Some(t742) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823a19); };
            c.w64(12, t742);
            let Some(t743) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000823a1d); };
            c.w64(7, t743);
            let Some(t744) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000823a21); };
            c.w64(2, t744);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x7000823a25); }
            blk = 69; continue;
        }
        542 => {
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000823a2f); }
            if !c.push64(c.r64(12)) { return RegfileExit::fault(0x7000823a31); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000823a33); }
            let Some(t745) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x7000823a35); };
            c.w64(3, t745);
            c.w32(0, 0x3u64);
            let Some(t746) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000823a41); };
            c.w64(6, t746);
            return c.call(0x700086ed74, 0x7000823a4a, 0x7000823a45);
        }
        543 => {
            c.w64(9, c.r64(0));
            let Some(t747) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000823a4d); };
            c.w64(8, t747);
            let Some(t748) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823a51); };
            c.w64(12, t748);
            let Some(t749) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x7000823a55); };
            c.w64(7, t749);
            blk = 110; continue;
        }
        544 => {
            c.w64(12, c.r64(0));
            let Some(t750) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823a61); };
            c.w64(0, t750);
            let Some(t751) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823a65); };
            c.w64(15, t751);
            blk = 508; continue;
        }
        545 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x7000823a73); }
            return c.call(0x70007febf4, 0x7000823a7d, 0x7000823a78);
        }
        546 => {
            c.w64(8, c.r64(0));
            let Some(t752) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823a80); };
            c.w64(0, t752);
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823a84); };
            c.w64(15, t753);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000823a88); }
            blk = 146; continue;
        }
        547 => {
            c.w64(12, c.r64(0));
            let Some(t754) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823a95); };
            c.w64(0, t754);
            blk = 508; continue;
        }
        548 => {
            let Some(t755) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823a9e); };
            c.w64(0, t755);
            let Some(t756) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823aa2); };
            c.w64(12, t756);
            blk = 508; continue;
        }
        549 => {
            let Some(t757) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823aab); };
            c.w64(0, t757);
            c.w64(12, c.r64(11));
            blk = 508; continue;
        }
        550 => {
            let Some(t758) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ab7); };
            c.w64(0, t758);
            let Some(t759) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823abb); };
            c.w64(12, t759);
            blk = 508; continue;
        }
        551 => {
            let Some(t760) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ac4); };
            c.w64(0, t760);
            let Some(t761) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823ac8); };
            c.w64(12, t761);
            blk = 508; continue;
        }
        552 => {
            let Some(t762) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ad1); };
            c.w64(0, t762);
            let Some(t763) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823ad5); };
            c.w64(12, t763);
            blk = 508; continue;
        }
        553 => {
            let Some(t764) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ade); };
            c.w64(0, t764);
            let Some(t765) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823ae2); };
            c.w64(12, t765);
            blk = 508; continue;
        }
        554 => {
            let Some(t766) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823aeb); };
            c.w64(12, t766);
            blk = 508; continue;
        }
        555 => {
            let Some(t767) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823af4); };
            c.w64(12, t767);
            blk = 508; continue;
        }
        556 => {
            let Some(t768) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823afd); };
            c.w64(12, t768);
            blk = 508; continue;
        }
        557 => {
            let Some(t769) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b06); };
            c.w64(12, t769);
            blk = 508; continue;
        }
        558 => {
            let Some(t770) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b0f); };
            c.w64(12, t770);
            blk = 508; continue;
        }
        559 => {
            c.w64(0, c.r64(12));
            let Some(t771) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b1b); };
            c.w64(12, t771);
            blk = 508; continue;
        }
        560 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 508; continue;
        }
        561 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 508; continue;
        }
        562 => {
            c.w64(10, c.r64(0));
            c.w64(0, c.r64(12));
            c.w64(12, c.r64(10));
            blk = 508; continue;
        }
        563 => {
            c.w64(0, c.r64(12));
            let Some(t772) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b51); };
            c.w64(12, t772);
            blk = 508; continue;
        }
        564 => {
            c.w64(0, c.r64(12));
            let Some(t773) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b5d); };
            c.w64(12, t773);
            blk = 508; continue;
        }
        565 => {
            c.w64(0, c.r64(12));
            let Some(t774) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b69); };
            c.w64(12, t774);
            blk = 508; continue;
        }
        566 => {
            c.w64(0, c.r64(12));
            let Some(t775) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b75); };
            c.w64(12, t775);
            blk = 508; continue;
        }
        567 => {
            let Some(t776) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823b7e); };
            c.w64(0, t776);
            let Some(t777) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b82); };
            c.w64(12, t777);
            blk = 508; continue;
        }
        568 => {
            let Some(t778) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823b8b); };
            c.w64(0, t778);
            let Some(t779) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b8f); };
            c.w64(12, t779);
            blk = 508; continue;
        }
        569 => {
            let Some(t780) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823b98); };
            c.w64(0, t780);
            let Some(t781) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823b9c); };
            c.w64(12, t781);
            blk = 508; continue;
        }
        570 => {
            let Some(t782) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ba5); };
            c.w64(0, t782);
            let Some(t783) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823ba9); };
            c.w64(12, t783);
            blk = 508; continue;
        }
        571 => {
            let Some(t784) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823bb2); };
            c.w64(0, t784);
            let Some(t785) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823bb6); };
            c.w64(12, t785);
            blk = 508; continue;
        }
        572 => {
            let Some(t786) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823bbf); };
            c.w64(0, t786);
            let Some(t787) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823bc3); };
            c.w64(12, t787);
            blk = 508; continue;
        }
        573 => {
            let Some(t788) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823bcc); };
            c.w64(0, t788);
            let Some(t789) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823bd0); };
            c.w64(12, t789);
            blk = 508; continue;
        }
        574 => {
            let Some(t790) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823bd9); };
            c.w64(0, t790);
            let Some(t791) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823bdd); };
            c.w64(12, t791);
            blk = 508; continue;
        }
        575 => {
            let Some(t792) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823be6); };
            c.w64(0, t792);
            let Some(t793) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823bea); };
            c.w64(12, t793);
            blk = 508; continue;
        }
        576 => {
            let Some(t794) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823bf3); };
            c.w64(0, t794);
            let Some(t795) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823bf7); };
            c.w64(12, t795);
            blk = 508; continue;
        }
        577 => {
            let Some(t796) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c00); };
            c.w64(0, t796);
            let Some(t797) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c04); };
            c.w64(12, t797);
            blk = 508; continue;
        }
        578 => {
            let Some(t798) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c0d); };
            c.w64(0, t798);
            let Some(t799) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c11); };
            c.w64(12, t799);
            blk = 508; continue;
        }
        579 => {
            let Some(t800) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c1a); };
            c.w64(0, t800);
            let Some(t801) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c1e); };
            c.w64(12, t801);
            blk = 508; continue;
        }
        580 => {
            let Some(t802) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c27); };
            c.w64(0, t802);
            c.w64(12, c.r64(3));
            blk = 508; continue;
        }
        581 => {
            let Some(t803) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c33); };
            c.w64(0, t803);
            let Some(t804) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c37); };
            c.w64(12, t804);
            blk = 508; continue;
        }
        582 => {
            let Some(t805) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c40); };
            c.w64(0, t805);
            c.w64(12, c.r64(11));
            blk = 508; continue;
        }
        583 => {
            let Some(t806) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c4c); };
            c.w64(0, t806);
            let Some(t807) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c50); };
            c.w64(12, t807);
            blk = 508; continue;
        }
        584 => {
            let Some(t808) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c59); };
            c.w64(0, t808);
            let Some(t809) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c5d); };
            c.w64(12, t809);
            blk = 508; continue;
        }
        585 => {
            let Some(t810) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c66); };
            c.w64(0, t810);
            let Some(t811) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c6a); };
            c.w64(12, t811);
            blk = 508; continue;
        }
        586 => {
            let Some(t812) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823c73); };
            c.w64(0, t812);
            let Some(t813) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823c77); };
            c.w64(12, t813);
            blk = 508; continue;
        }
        587 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000823c85); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(11)) { return RegfileExit::fault(0x7000823c89); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 0) { return RegfileExit::fault(0x7000823c90); }
            return c.call(0x70007febf4, 0x7000823c9a, 0x7000823c95);
        }
        588 => {
            c.w64(12, c.r64(0));
            let Some(t814) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000823c9d); };
            c.w64(8, t814);
            let Some(t815) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823ca1); };
            c.w64(9, t815);
            let Some(t816) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000823ca5); };
            c.w64(11, t816);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x7000823cac); }
            blk = 334; continue;
        }
        589 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000823cb6); }
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(15)) { return RegfileExit::fault(0x7000823cba); }
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(3)) { return RegfileExit::fault(0x7000823cc1); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(11)) { return RegfileExit::fault(0x7000823cc8); }
            if !c.st64(c.r64(5).wrapping_sub(0xa0), c.r64(2)) { return RegfileExit::fault(0x7000823ccf); }
            c.w64(2, c.r64(7));
            return c.call(0x70007febf4, 0x7000823cde, 0x7000823cd9);
        }
        590 => {
            c.w64(8, c.r64(0));
            let Some(t817) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ce1); };
            c.w64(0, t817);
            let Some(t818) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823ce5); };
            c.w64(9, t818);
            let Some(t819) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x7000823ce9); };
            c.w64(15, t819);
            let Some(t820) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x7000823cf0); };
            c.w64(3, t820);
            let Some(t821) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000823cf7); };
            c.w64(11, t821);
            let Some(t822) = c.ld64(c.r64(5).wrapping_sub(0xa0)) else { return RegfileExit::fault(0x7000823cfe); };
            c.w64(2, t822);
            blk = 342; continue;
        }
        591 => {
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000823d0f); }
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x80), 0) { return RegfileExit::fault(0x7000823d13); }
            return c.call(0x70007febf4, 0x7000823d1d, 0x7000823d18);
        }
        592 => {
            c.w64(11, c.r64(0));
            let Some(t823) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823d20); };
            c.w64(9, t823);
            let Some(t824) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x7000823d24); };
            c.w64(8, t824);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x80)) { return RegfileExit::fault(0x7000823d28); }
            blk = 364; continue;
        }
        593 => {
            let Some(t825) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d32); };
            c.w64(0, t825);
            let Some(t826) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d36); };
            c.w64(12, t826);
            blk = 508; continue;
        }
        594 => {
            let Some(t827) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d3f); };
            c.w64(0, t827);
            let Some(t828) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d43); };
            c.w64(12, t828);
            blk = 508; continue;
        }
        595 => {
            let Some(t829) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d4c); };
            c.w64(0, t829);
            let Some(t830) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d50); };
            c.w64(12, t830);
            blk = 508; continue;
        }
        596 => {
            let Some(t831) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d59); };
            c.w64(0, t831);
            let Some(t832) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d5d); };
            c.w64(12, t832);
            blk = 508; continue;
        }
        597 => {
            let Some(t833) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d66); };
            c.w64(0, t833);
            let Some(t834) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d6a); };
            c.w64(12, t834);
            blk = 508; continue;
        }
        598 => {
            let Some(t835) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d73); };
            c.w64(0, t835);
            let Some(t836) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d77); };
            c.w64(12, t836);
            blk = 508; continue;
        }
        599 => {
            let Some(t837) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d80); };
            c.w64(0, t837);
            let Some(t838) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d84); };
            c.w64(12, t838);
            blk = 508; continue;
        }
        600 => {
            let Some(t839) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d8d); };
            c.w64(0, t839);
            let Some(t840) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d91); };
            c.w64(12, t840);
            blk = 508; continue;
        }
        601 => {
            let Some(t841) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823d9a); };
            c.w64(0, t841);
            let Some(t842) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823d9e); };
            c.w64(12, t842);
            blk = 508; continue;
        }
        602 => {
            let Some(t843) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823da7); };
            c.w64(0, t843);
            let Some(t844) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823dab); };
            c.w64(12, t844);
            blk = 508; continue;
        }
        603 => {
            let Some(t845) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823db4); };
            c.w64(0, t845);
            let Some(t846) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823db8); };
            c.w64(12, t846);
            blk = 508; continue;
        }
        604 => {
            let Some(t847) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823dc1); };
            c.w64(0, t847);
            let Some(t848) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823dc5); };
            c.w64(12, t848);
            blk = 508; continue;
        }
        605 => {
            let Some(t849) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823dce); };
            c.w64(0, t849);
            let Some(t850) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823dd2); };
            c.w64(12, t850);
            blk = 508; continue;
        }
        606 => {
            let Some(t851) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ddb); };
            c.w64(0, t851);
            let Some(t852) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823ddf); };
            c.w64(12, t852);
            blk = 508; continue;
        }
        607 => {
            c.w64(12, c.r64(0));
            let Some(t853) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823deb); };
            c.w64(0, t853);
            blk = 508; continue;
        }
        608 => {
            c.w64(12, c.r64(0));
            let Some(t854) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823df7); };
            c.w64(0, t854);
            blk = 508; continue;
        }
        609 => {
            c.w64(12, c.r64(0));
            let Some(t855) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e03); };
            c.w64(0, t855);
            blk = 508; continue;
        }
        610 => {
            c.w64(12, c.r64(0));
            let Some(t856) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e0f); };
            c.w64(0, t856);
            blk = 508; continue;
        }
        611 => {
            c.w64(12, c.r64(0));
            let Some(t857) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e1b); };
            c.w64(0, t857);
            blk = 508; continue;
        }
        612 => {
            c.w64(12, c.r64(0));
            let Some(t858) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e27); };
            c.w64(0, t858);
            blk = 508; continue;
        }
        613 => {
            c.w64(12, c.r64(0));
            let Some(t859) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e33); };
            c.w64(0, t859);
            blk = 508; continue;
        }
        614 => {
            c.w64(12, c.r64(0));
            let Some(t860) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e3f); };
            c.w64(0, t860);
            blk = 508; continue;
        }
        615 => {
            c.w64(12, c.r64(0));
            let Some(t861) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e4b); };
            c.w64(0, t861);
            blk = 508; continue;
        }
        616 => {
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x30), 0) { return RegfileExit::fault(0x7000823e59); }
            if !c.st64(c.r64(5).wrapping_sub(0x98), c.r64(0)) { return RegfileExit::fault(0x7000823e5e); }
            return c.call(0x70007febf4, 0x7000823e6a, 0x7000823e65);
        }
        617 => {
            c.w64(8, c.r64(0));
            let Some(t862) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000823e6d); };
            c.w64(15, t862);
            let Some(t863) = c.ld64(c.r64(5).wrapping_sub(0x98)) else { return RegfileExit::fault(0x7000823e71); };
            c.w64(0, t863);
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x30)) { return RegfileExit::fault(0x7000823e78); }
            blk = 462; continue;
        }
        618 => {
            let Some(t864) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e82); };
            c.w64(0, t864);
            let Some(t865) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823e86); };
            c.w64(12, t865);
            blk = 508; continue;
        }
        619 => {
            let Some(t866) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e8f); };
            c.w64(0, t866);
            let Some(t867) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823e93); };
            c.w64(12, t867);
            blk = 508; continue;
        }
        620 => {
            let Some(t868) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823e9c); };
            c.w64(0, t868);
            let Some(t869) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823ea0); };
            c.w64(12, t869);
            blk = 508; continue;
        }
        621 => {
            let Some(t870) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ea9); };
            c.w64(0, t870);
            let Some(t871) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x7000823ead); };
            c.w64(12, t871);
            blk = 508; continue;
        }
        622 => {
            c.w64(12, c.r64(0));
            let Some(t872) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823eb9); };
            c.w64(0, t872);
            blk = 508; continue;
        }
        623 => {
            c.w64(12, c.r64(0));
            let Some(t873) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ec5); };
            c.w64(0, t873);
            blk = 508; continue;
        }
        624 => {
            c.w64(12, c.r64(0));
            let Some(t874) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ed1); };
            c.w64(0, t874);
            blk = 508; continue;
        }
        625 => {
            c.w64(12, c.r64(0));
            let Some(t875) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823edd); };
            c.w64(0, t875);
            blk = 508; continue;
        }
        626 => {
            c.w64(12, c.r64(0));
            let Some(t876) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ee9); };
            c.w64(0, t876);
            blk = 508; continue;
        }
        627 => {
            c.w64(12, c.r64(0));
            let Some(t877) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823ef5); };
            c.w64(0, t877);
            blk = 508; continue;
        }
        628 => {
            c.w64(12, c.r64(0));
            let Some(t878) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823f01); };
            c.w64(0, t878);
            blk = 508; continue;
        }
        629 => {
            c.w64(12, c.r64(0));
            let Some(t879) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000823f0d); };
            c.w64(0, t879);
            blk = 508; continue;
        }
        630 => {
            return RegfileExit::fault(0x7000823f16);
        }
        631 => {
            return RegfileExit::fault(0x7000823f17);
        }
        632 => {
            return RegfileExit::fault(0x7000823f18);
        }
        633 => {
            return RegfileExit::fault(0x7000823f19);
        }
        634 => {
            return RegfileExit::fault(0x7000823f1a);
        }
        635 => {
            let Some(t880) = c.ld64(c.r64(13).wrapping_add(0x900)) else { return RegfileExit::fault(0x7000823f1b); };
            c.w64(8, t880);
            blk = 533; continue;
        }
        636 => {
            return RegfileExit::fault(0x7000823f27);
        }
        637 => {
            return RegfileExit::fault(0x7000823f28);
        }
        638 => {
            return RegfileExit::fault(0x7000823f29);
        }
        639 => {
            return RegfileExit::fault(0x7000823f2a);
        }
        640 => {
            return RegfileExit::fault(0x7000823f2b);
        }
        641 => {
            return RegfileExit::fault(0x7000823f2c);
        }
        642 => {
            return RegfileExit::fault(0x7000823f2d);
        }
        643 => {
            let Some(t881) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x7000823f2e); };
            c.w64(5, t881);
            let r882 = c.add(c.r64(4), 0xc0u64, 64);
            c.w64(4, r882);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000823f39); }
            let Some(t883) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000823f3a); };
            if !c.push64(t883) { return RegfileExit::fault(0x7000823f3a); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x7000823f3e); }
            c.w32(0, 0x2u64);
            let Some(t884) = c.ld64(c.r64(13).wrapping_add(0x2468)) else { return RegfileExit::fault(0x7000823f48); };
            c.w64(3, t884);
            return RegfileExit::jump(0x700086ed74);
        }
        644 => {
            c.w32(15, c.r32(15));
            let r885 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r885);
            c.w64(0, 0xfffffffffffc0000u64);
            let r886 = c.r64(0) & c.r64(15);
            c.w64(0, r886);
            c.logic(r886, 64);
            let Some(t887) = c.ld8(c.r64(0).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000823f64); };
            c.logic(t887 & 0x2u64, 8);
            if c.cc_e() { blk = 500; continue; }
            blk = 645; continue;
        }
        645 => {
            c.w64(1, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000823f72); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000823f73); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(1));
            return c.call(0x70007f0e54, 0x7000823f80, 0x7000823f7b);
        }
        646 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000823f80); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000823f82); }
            blk = 500; continue;
        }
        647 => {
            c.w32(12, c.r32(12));
            let r888 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r888);
            c.w64(9, 0xfffffffffffc0000u64);
            let r889 = c.r64(9) & c.r64(12);
            c.w64(9, r889);
            c.logic(r889, 64);
            let Some(t890) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000823f98); };
            c.logic(t890 & 0x2u64, 8);
            if c.cc_e() { blk = 497; continue; }
            blk = 648; continue;
        }
        648 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(1)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000823fa7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000823fa8); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000823fb5, 0x7000823fb0);
        }
        649 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000823fb5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000823fb7); }
            blk = 497; continue;
        }
        650 => {
            c.w32(0, c.r32(0));
            let r891 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r891);
            c.w64(9, 0xfffffffffffc0000u64);
            let r892 = c.r64(9) & c.r64(0);
            c.w64(9, r892);
            c.logic(r892, 64);
            let Some(t893) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000823fcc); };
            c.logic(t893 & 0x2u64, 8);
            if c.cc_e() { blk = 388; continue; }
            blk = 651; continue;
        }
        651 => {
            c.w64(12, c.r64(11).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000823fdb); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000823fdc); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000823fe9, 0x7000823fe4);
        }
        652 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000823fe9); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000823feb); }
            blk = 388; continue;
        }
        653 => {
            c.w32(11, c.r32(11));
            let r894 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r894);
            c.w64(8, 0xfffffffffffc0000u64);
            let r895 = c.r64(8) & c.r64(11);
            c.w64(8, r895);
            c.logic(r895, 64);
            let Some(t896) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000824001); };
            c.logic(t896 & 0x2u64, 8);
            if c.cc_e() { blk = 367; continue; }
            blk = 654; continue;
        }
        654 => {
            c.w64(15, c.r64(12).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000824010); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000824011); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x700082401e, 0x7000824019);
        }
        655 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700082401e); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000824020); }
            blk = 367; continue;
        }
        656 => {
            c.w32(8, c.r32(8));
            let r897 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r897);
            c.w64(15, 0xfffffffffffc0000u64);
            let r898 = c.r64(15) & c.r64(8);
            c.w64(15, r898);
            c.logic(r898, 64);
            let Some(t899) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000824036); };
            c.logic(t899 & 0x2u64, 8);
            if c.cc_e() { blk = 365; continue; }
            blk = 657; continue;
        }
        657 => {
            c.w64(0, c.r64(12).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000824046); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000824047); }
            c.w64(7, c.r64(12));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x7000824054, 0x700082404f);
        }
        658 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000824054); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000824056); }
            blk = 365; continue;
        }
        659 => {
            c.w32(12, c.r32(12));
            let r900 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r900);
            c.w64(8, 0xfffffffffffc0000u64);
            let r901 = c.r64(8) & c.r64(12);
            c.w64(8, r901);
            c.logic(r901, 64);
            let Some(t902) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082406c); };
            c.logic(t902 & 0x2u64, 8);
            if c.cc_e() { blk = 358; continue; }
            blk = 660; continue;
        }
        660 => {
            c.w64(15, c.r64(11).wrapping_add(c.r64(9)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082407b); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082407c); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(15));
            return c.call(0x70007f0e54, 0x7000824089, 0x7000824084);
        }
        661 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000824089); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700082408b); }
            blk = 358; continue;
        }
        662 => {
            c.w32(8, c.r32(8));
            let r903 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r903);
            c.w64(15, 0xfffffffffffc0000u64);
            let r904 = c.r64(15) & c.r64(8);
            c.w64(15, r904);
            c.logic(r904, 64);
            let Some(t905) = c.ld8(c.r64(15).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008240a1); };
            c.logic(t905 & 0x2u64, 8);
            if c.cc_e() { blk = 356; continue; }
            blk = 663; continue;
        }
        663 => {
            c.w64(0, c.r64(11).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008240b0); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008240b1); }
            c.w64(7, c.r64(11));
            c.w64(8, c.r64(0));
            return c.call(0x70007f13b4, 0x70008240be, 0x70008240b9);
        }
        664 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008240be); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008240c0); }
            blk = 356; continue;
        }
        665 => {
            c.w32(8, c.r32(8));
            let r906 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r906);
            c.w64(9, 0xfffffffffffc0000u64);
            let r907 = c.r64(9) & c.r64(8);
            c.w64(9, r907);
            c.logic(r907, 64);
            let Some(t908) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008240d6); };
            c.logic(t908 & 0x2u64, 8);
            if c.cc_e() { blk = 353; continue; }
            blk = 666; continue;
        }
        666 => {
            c.w64(11, c.r64(0).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008240e5); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008240e6); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x70008240f3, 0x70008240ee);
        }
        667 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008240f3); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008240f5); }
            blk = 353; continue;
        }
        668 => {
            c.w32(9, c.r32(9));
            let r909 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r909);
            c.w64(11, 0xfffffffffffc0000u64);
            let r910 = c.r64(11) & c.r64(9);
            c.w64(11, r910);
            c.logic(r910, 64);
            let Some(t911) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x700082410b); };
            c.logic(t911 & 0x2u64, 8);
            if c.cc_e() { blk = 352; continue; }
            blk = 669; continue;
        }
        669 => {
            c.w64(12, c.r64(15).wrapping_add(c.r64(8)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082411a); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x700082411b); }
            c.w64(7, c.r64(15));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x7000824128, 0x7000824123);
        }
        670 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000824128); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700082412a); }
            blk = 352; continue;
        }
        671 => {
            c.w32(8, c.r32(8));
            let r912 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r912);
            c.w64(11, 0xfffffffffffc0000u64);
            let r913 = c.r64(11) & c.r64(8);
            c.w64(11, r913);
            c.logic(r913, 64);
            let Some(t914) = c.ld8(c.r64(11).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000824140); };
            c.logic(t914 & 0x2u64, 8);
            if c.cc_e() { blk = 349; continue; }
            blk = 672; continue;
        }
        672 => {
            c.w64(12, c.r64(0).wrapping_add(0x3));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x700082414f); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000824150); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(12));
            return c.call(0x70007f0e54, 0x700082415d, 0x7000824158);
        }
        673 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x700082415d); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x700082415f); }
            blk = 349; continue;
        }
        674 => {
            c.w32(8, c.r32(8));
            let r915 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r915);
            c.w64(9, 0xfffffffffffc0000u64);
            let r916 = c.r64(9) & c.r64(8);
            c.w64(9, r916);
            c.logic(r916, 64);
            let Some(t917) = c.ld8(c.r64(9).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000824175); };
            c.logic(t917 & 0x2u64, 8);
            if c.cc_e() { blk = 311; continue; }
            blk = 675; continue;
        }
        675 => {
            c.w64(11, c.r64(2).wrapping_sub(0x1));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x7000824184); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x7000824185); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(11));
            return c.call(0x70007f13b4, 0x7000824192, 0x700082418d);
        }
        676 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x7000824192); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x7000824194); }
            blk = 311; continue;
        }
        677 => {
            c.w32(3, c.r32(3));
            let r918 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r918);
            c.w64(8, 0xfffffffffffc0000u64);
            let r919 = c.r64(8) & c.r64(3);
            c.w64(8, r919);
            c.logic(r919, 64);
            let Some(t920) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008241a9); };
            c.logic(t920 & 0x2u64, 8);
            if c.cc_e() { blk = 220; continue; }
            blk = 678; continue;
        }
        678 => {
            c.w64(9, c.r64(0).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008241b8); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008241b9); }
            c.w64(7, c.r64(0));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008241c6, 0x70008241c1);
        }
        679 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008241c6); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008241c8); }
            blk = 220; continue;
        }
        680 => {
            c.w32(2, c.r32(2));
            let r921 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r921);
            c.w64(8, 0xfffffffffffc0000u64);
            let r922 = c.r64(8) & c.r64(2);
            c.w64(8, r922);
            c.logic(r922, 64);
            let Some(t923) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008241dd); };
            c.logic(t923 & 0x2u64, 8);
            if c.cc_e() { blk = 180; continue; }
            blk = 681; continue;
        }
        681 => {
            c.w64(9, c.r64(3).wrapping_add(c.r64(12)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008241ec); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008241ed); }
            c.w64(7, c.r64(3));
            c.w64(8, c.r64(9));
            return c.call(0x70007f0e54, 0x70008241fa, 0x70008241f5);
        }
        682 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008241fa); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008241fc); }
            blk = 180; continue;
        }
        683 => {
            return RegfileExit::jump(0x7000824204);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000824214); };
            if !c.push64(t0) { return RegfileExit::fault(0x7000824214); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000824217); };
            c.w64(3, t1);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(2)) { return RegfileExit::fault(0x7000824225); }
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x7000824230);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000824234); };
            if !c.push64(t0) { return RegfileExit::fault(0x7000824234); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2800)) else { return RegfileExit::fault(0x7000824237); };
            c.w64(3, t1);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(2)) { return RegfileExit::fault(0x7000824245); }
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x7000824250);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824254(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16a8)) else { return RegfileExit::fault(0x7000824254); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000824274(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16b0)) else { return RegfileExit::fault(0x7000824274); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_7000824294(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16b8)) else { return RegfileExit::fault(0x7000824294); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008242b4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x16c0)) else { return RegfileExit::fault(0x70008242b4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008242d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008242d4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008242d8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008242d9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008242da); }
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008242db); };
            c.cmp(c.r64(4), t0, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(5).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008242e1); };
            c.w64(0, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008242e5); };
            c.w64(1, t2);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008242ec); }
            c.cmp(c.r64(1), 0x0u64, 64);
            if c.cc_g() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            return c.ret(0x8, 0x70008242f3);
        }
        3 => {
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008242f6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(1).wrapping_mul(8)).wrapping_add(0x8));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008242fd); }
            return c.ret(0x0, 0x70008242ff);
        }
        4 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x7000824300); };
            c.w64(3, t3);
            c.w32(0, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x700082430e, 0x7000824309);
        }
        5 => {
            blk = 1; continue;
        }
        6 => {
            return RegfileExit::jump(0x7000824314);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000824334); };
            c.cmp(t0, c.r32(2), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(1, c.r64(2));
            blk = 3; continue;
        }
        2 => {
            c.w64(1, c.r64(7));
            blk = 3; continue;
        }
        3 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000824345); };
            c.w64(3, t1);
            c.w64(2, c.r64(1));
            return RegfileExit::jump(0x7000824374);
        }
        4 => {
            return RegfileExit::jump(0x7000824358);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824374(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824374); };
            c.w32(6, t0);
            let r1 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r1);
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r32(7), c.r32(2), 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(1), 0x0u64, 64);
            c.w8(8, (c.cc_e() as u64));
            c.w32(8, c.r8(8));
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x7000824391); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_ne() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008243b2); };
            c.w32(2, t3);
            let r4 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r4);
            c.w64(1, c.r64(2));
            let r5 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r5);
            c.w32(8, c.r32(2));
            let r6 = c.r32(1) & 0x3fu64;
            c.w32(1, r6);
            c.logic(r6, 32);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_ne() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let r7 = c.r32(8) | 0x2u64;
            c.w32(8, r7);
            c.logic(r7, 32);
            c.w64(2, (c.r32(8) as u32 as i32 as i64 as u64));
            let r8 = c.r32(1) | 0x1u64;
            c.w32(1, r8);
            c.logic(r8, 32);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(2)) { return RegfileExit::fault(0x70008243d2); }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 7; continue;
        }
        7 => {
            c.cmp(c.r32(1), 0x1u64, 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 8; continue;
        }
        8 => {
            c.cmp(c.r32(1), 0x4u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(1), 0x5u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 15; continue; }
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 12; continue;
        }
        12 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008243f3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008243f7); }
            c.w32(2, 0x54u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008243fe); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000824401); };
            c.w64(3, t9);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(2));
            return c.call(0x700086ed74, 0x7000824415, 0x7000824410);
        }
        13 => {
            return RegfileExit::fault(0x7000824415);
        }
        14 => {
            return RegfileExit::jump(0x7000825534);
        }
        15 => {
            return RegfileExit::jump(0x7000825354);
        }
        16 => {
            return RegfileExit::jump(0x70008258f4);
        }
        17 => {
            return RegfileExit::jump(0x70008256f4);
        }
        18 => {
            return RegfileExit::jump(0x7000824d94);
        }
        19 => {
            return RegfileExit::jump(0x7000824b54);
        }
        20 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000824434); };
            c.w32(2, t10);
            c.w32(2, c.r32(2));
            let r11 = c.r32(2) & 0x7eu64;
            c.w32(2, r11);
            c.logic(r11, 32);
            if c.cc_e() { blk = 33; continue; }
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(2), 0x2u64, 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 22; continue;
        }
        22 => {
            c.cmp(c.r32(2), 0x8u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 23; continue;
        }
        23 => {
            c.cmp(c.r32(2), 0xau64, 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 24; continue;
        }
        24 => {
            c.cmp(c.r32(2), 0x4u64, 32);
            if c.cc_e() { blk = 29; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(2), 0x6u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 26; continue;
        }
        26 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824457); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082445b); }
            c.w32(2, 0x54u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x7000824462); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000824463); };
            c.w64(3, t12);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x7000824476, 0x7000824471);
        }
        27 => {
            return RegfileExit::fault(0x7000824476);
        }
        28 => {
            return RegfileExit::jump(0x7000824914);
        }
        29 => {
            return RegfileExit::jump(0x7000824854);
        }
        30 => {
            return RegfileExit::jump(0x7000824a94);
        }
        31 => {
            return RegfileExit::jump(0x70008249d4);
        }
        32 => {
            return RegfileExit::jump(0x70008245f4);
        }
        33 => {
            return RegfileExit::jump(0x7000824514);
        }
        34 => {
            c.cmp(c.r32(8), 0x0u64, 32);
            if c.cc_ne() { blk = 37; continue; }
            blk = 35; continue;
        }
        35 => {
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_ne() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008244a1); };
            c.w64(3, t13);
            return RegfileExit::jump(0x7000825194);
        }
        37 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008244ad); };
            c.w64(3, t14);
            return RegfileExit::jump(0x70008246d4);
        }
        38 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008244b9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008244bd); }
            blk = 41; continue;
        }
        39 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008244c1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008244c5); }
            blk = 41; continue;
        }
        40 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008244c9); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008244cd); }
            blk = 41; continue;
        }
        41 => {
            c.w64(8, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            if !c.st64(c.r64(9).wrapping_add(c.r64(8)).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x70008244d9); }
            let r15 = c.add(c.r64(1), 0x1u64, 64);
            c.w64(1, r15);
            c.w32(0, c.r64(1).wrapping_add(0x2));
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008244e5); };
            c.w64(5, t16);
            let r17 = c.add(c.r64(4), 0x10u64, 64);
            c.w64(4, r17);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008244ed); }
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008244ee); };
            if !c.push64(t18) { return RegfileExit::fault(0x70008244ee); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x70008244f2); };
            c.w64(3, t19);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70008244f9); }
            return RegfileExit::jump(0x700086ed74);
        }
        42 => {
            return RegfileExit::jump(0x7000824504);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824514); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082451a); };
            c.w32(6, t2);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x69u64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824529);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x700082452a); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824530); };
            c.w64(6, t4);
            let Some(t5) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000824537); };
            c.w64(7, t5);
            c.w64(8, c.r64(7).wrapping_add(0x30));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082453e); };
            c.w64(9, t6);
            let Some(t7) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000824545); };
            c.cmp(t7, c.r64(8), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x700082454a); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000824551); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000824556); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824559); };
            c.w64(1, t8);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000824560); }
            c.w64(1, c.r64(6).wrapping_add(0x10));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000824567); };
            c.w64(7, t9);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700082456e); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824571); }
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000824574); };
            c.w32(1, t10);
            let r11 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r11);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x700082457a); }
            c.w64(3, c.r64(6).wrapping_add(0x18));
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000824581); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000824584); };
            c.w64(1, t12);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082458b); }
            c.w32(1, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000824593); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000824596); };
            c.w64(1, t13);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x700082459d); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x70008245a0); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x70008245a3); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008245a6); }
            c.w64(0, c.r64(6));
            return c.ret(0x8, 0x70008245ac);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008245af); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008245b3); }
            let r14 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r14);
            c.w32(2, 0x30u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008245be); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008245c2); }
            return c.call(0x70007febf4, 0x70008245cb, 0x70008245c6);
        }
        6 => {
            c.w64(6, c.r64(0));
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008245ce); };
            c.w64(3, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008245d2); };
            c.w64(1, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008245d9); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008245e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008245f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008245f4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008245fa); };
            c.w32(6, t2);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x6au64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824609);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1af)) else { return RegfileExit::fault(0x700082460a); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824610); };
            c.w64(6, t4);
            let Some(t5) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x7000824617); };
            c.w64(7, t5);
            c.w64(8, c.r64(7).wrapping_add(0x30));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082461e); };
            c.w64(9, t6);
            let Some(t7) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000824625); };
            c.cmp(t7, c.r64(8), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(6), c.r64(8)) { return RegfileExit::fault(0x700082462a); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000824631); }
            c.w32(1, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000824636); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824639); };
            c.w64(1, t8);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000824640); }
            c.w64(1, c.r64(6).wrapping_add(0x10));
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000824647); };
            c.w64(7, t9);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x700082464e); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824651); }
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000824654); };
            c.w32(1, t10);
            let r11 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r11);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x700082465a); }
            c.w64(3, c.r64(6).wrapping_add(0x18));
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000824661); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000824664); };
            c.w64(1, t12);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082466b); }
            c.w32(1, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x7000824673); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000824676); };
            c.w64(1, t13);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x700082467d); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(1)) { return RegfileExit::fault(0x7000824680); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(1)) { return RegfileExit::fault(0x7000824683); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000824686); }
            c.w64(0, c.r64(6));
            return c.ret(0x8, 0x700082468c);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082468f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824693); }
            let r14 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r14);
            c.w32(2, 0x30u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x700082469e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008246a2); }
            return c.call(0x70007febf4, 0x70008246ab, 0x70008246a6);
        }
        6 => {
            c.w64(6, c.r64(0));
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008246ae); };
            c.w64(3, t15);
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008246b2); };
            c.w64(1, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008246b9); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008246c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008246d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008246d4); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008246da); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x69u64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008246e9);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x70008246ea); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008246f0); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008246f7); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x28));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008246fe); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000824705); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700082470a); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000824711); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000824716); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824719); };
            c.w64(3, t8);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824720); }
            c.w64(3, c.r64(1).wrapping_add(0x10));
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000824727); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700082472a); };
            c.w64(6, t9);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000824731); }
            c.w32(6, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000824739); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082473c); };
            c.w64(6, t10);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(6)) { return RegfileExit::fault(0x7000824743); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x7000824746); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000824749); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700082474c); }
            c.w64(0, c.r64(1));
            return c.ret(0x8, 0x7000824752);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824755); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824759); }
            let r11 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x7000824764); }
            return c.call(0x70007febf4, 0x700082476d, 0x7000824768);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824770); };
            c.w64(3, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824777); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x700082477c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824794); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082479a); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x6au64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008247a9);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1af)) else { return RegfileExit::fault(0x70008247aa); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008247b0); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008247b7); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x28));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008247be); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008247c5); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008247ca); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008247d1); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x70008247d6); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008247d9); };
            c.w64(3, t8);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008247e0); }
            c.w64(3, c.r64(1).wrapping_add(0x10));
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008247e7); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008247ea); };
            c.w64(6, t9);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008247f1); }
            c.w32(6, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008247f9); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008247fc); };
            c.w64(6, t10);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(6)) { return RegfileExit::fault(0x7000824803); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x7000824806); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000824809); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700082480c); }
            c.w64(0, c.r64(1));
            return c.ret(0x8, 0x7000824812);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824815); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824819); }
            let r11 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x7000824824); }
            return c.call(0x70007febf4, 0x700082482d, 0x7000824828);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824830); };
            c.w64(3, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824837); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x700082483c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824854(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824854); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082485a); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x6bu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824869);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x700082486a); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824870); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000824877); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x28));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082487e); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000824885); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700082488a); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000824891); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000824896); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824899); };
            c.w64(3, t8);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x70008248a0); }
            c.w64(3, c.r64(1).wrapping_add(0x10));
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008248a7); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008248aa); };
            c.w64(6, t9);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008248b1); }
            c.w32(6, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008248b9); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008248bc); };
            c.w64(6, t10);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(6)) { return RegfileExit::fault(0x70008248c3); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x70008248c6); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x70008248c9); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x70008248cc); }
            c.w64(0, c.r64(1));
            return c.ret(0x8, 0x70008248d2);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008248d5); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008248d9); }
            let r11 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008248e4); }
            return c.call(0x70007febf4, 0x70008248ed, 0x70008248e8);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008248f0); };
            c.w64(3, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008248f7); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008248fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824914(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824914); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082491a); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x6cu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824929);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700082492a); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824930); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000824937); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x28));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x700082493e); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000824945); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x700082494a); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000824951); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000824956); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824959); };
            c.w64(3, t8);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824960); }
            c.w64(3, c.r64(1).wrapping_add(0x10));
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000824967); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x700082496a); };
            c.w64(6, t9);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000824971); }
            c.w32(6, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000824979); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082497c); };
            c.w64(6, t10);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(6)) { return RegfileExit::fault(0x7000824983); }
            if !c.st32(c.r64(3).wrapping_add(0xf), c.r32(6)) { return RegfileExit::fault(0x7000824986); }
            if !c.st32(c.r64(3).wrapping_add(0xb), c.r32(6)) { return RegfileExit::fault(0x7000824989); }
            if !c.st32(c.r64(3).wrapping_add(0x7), c.r32(6)) { return RegfileExit::fault(0x700082498c); }
            c.w64(0, c.r64(1));
            return c.ret(0x8, 0x7000824992);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824995); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824999); }
            let r11 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.w32(2, 0x28u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008249a4); }
            return c.call(0x70007febf4, 0x70008249ad, 0x70008249a8);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008249b0); };
            c.w64(3, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008249b7); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x70008249bc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008249d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008249d4); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008249da); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x6du64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008249e9);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1bb)) else { return RegfileExit::fault(0x70008249ea); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008249f0); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008249f7); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x38));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008249fe); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000824a05); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000824a0a); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000824a11); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000824a16); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824a19); };
            c.w64(3, t8);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824a20); }
            c.w64(3, c.r64(1).wrapping_add(0x10));
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000824a27); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000824a2a); };
            c.w64(6, t9);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000824a31); }
            c.w32(6, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000824a39); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000824a3c); };
            c.w64(6, t10);
            if !c.movsd_rm(0, c.r64(6).wrapping_add(0x3)) { return RegfileExit::fault(0x7000824a43); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x1f), 0) { return RegfileExit::fault(0x7000824a48); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x17), 0) { return RegfileExit::fault(0x7000824a4d); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0xf), 0) { return RegfileExit::fault(0x7000824a52); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000824a57); }
            c.w64(0, c.r64(1));
            return c.ret(0x8, 0x7000824a5f);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824a62); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824a66); }
            let r11 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.w32(2, 0x38u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x7000824a71); }
            return c.call(0x70007febf4, 0x7000824a7a, 0x7000824a75);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824a7d); };
            c.w64(3, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824a84); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x7000824a88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824a94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824a94); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000824a9a); };
            c.w32(1, t2);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x6eu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824aa9);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(3).wrapping_add(0x1bf)) else { return RegfileExit::fault(0x7000824aaa); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824ab0); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x7000824ab7); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x38));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000824abe); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x7000824ac5); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x7000824aca); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x7000824ad1); }
            c.w32(3, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(1).wrapping_add(0xb), c.r32(3)) { return RegfileExit::fault(0x7000824ad6); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824ad9); };
            c.w64(3, t8);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824ae0); }
            c.w64(3, c.r64(1).wrapping_add(0x10));
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x7000824ae7); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x1e8)) else { return RegfileExit::fault(0x7000824aea); };
            c.w64(6, t9);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x7000824af1); }
            c.w32(6, 0x8u64);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x7000824af9); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000824afc); };
            c.w64(6, t10);
            if !c.movsd_rm(0, c.r64(6).wrapping_add(0x3)) { return RegfileExit::fault(0x7000824b03); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x1f), 0) { return RegfileExit::fault(0x7000824b08); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x17), 0) { return RegfileExit::fault(0x7000824b0d); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0xf), 0) { return RegfileExit::fault(0x7000824b12); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x7), 0) { return RegfileExit::fault(0x7000824b17); }
            c.w64(0, c.r64(1));
            return c.ret(0x8, 0x7000824b1f);
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824b22); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824b26); }
            let r11 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r11);
            c.w32(2, 0x38u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x7000824b31); }
            return c.call(0x70007febf4, 0x7000824b3a, 0x7000824b35);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824b3d); };
            c.w64(3, t12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824b44); }
            blk = 4; continue;
        }
        7 => {
            return RegfileExit::jump(0x7000824b48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824b54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824b54); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000824b5a); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x69u64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824b6a);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000824b6b); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000824b71); };
            c.w64(8, t4);
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824ba0); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000824ba7); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(2).wrapping_add(0x20));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000824bb6); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000824bbd); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000824bc6); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000824bce); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000824bd2); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824bd6); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000824bdd); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000824be5); };
            c.w64(15, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000824bec); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824bf0); }
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000824bf3); };
            c.w32(1, t11);
            let r12 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r12);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x7000824bf9); }
            c.w64(1, c.r64(11).wrapping_add(0x18));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000824c04); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000824c08); };
            c.w64(9, t13);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000824c0f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000824c13); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            blk = 11; continue;
        }
        8 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824c24); };
            c.w64(9, t14);
            let Some(t15) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000824c2b); };
            c.w64(11, t15);
            c.w64(15, c.r64(11).wrapping_add(0x18));
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000824c32); };
            c.w64(12, t16);
            let Some(t17) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000824c39); };
            c.cmp(t17, c.r64(15), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000824c43); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000824c4a); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000824c4e); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824c52); };
            c.w64(12, t18);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000824c59); }
            c.w64(1, c.r64(9).wrapping_add(0x10));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000824c61); };
            c.w64(8, t19);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000824c68); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824c6c); }
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000824c6f); };
            c.w32(1, t20);
            let r21 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r21);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x7000824c75); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000824c78); }
            c.w32(0, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000824c94); };
            c.w64(7, t22);
            blk = 14; continue;
        }
        14 => {
            let r23 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r23);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x7000824c9f); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x7000824cac);
        }
        16 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000824caf); }
            let Some(t24) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000824cb0); };
            if !c.push64(t24) { return RegfileExit::fault(0x7000824cb0); }
            let Some(t25) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x7000824cb4); };
            c.w64(3, t25);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x7000824cc0); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x7000824cc5); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x7000824cca); }
            return RegfileExit::jump(0x700086ed74);
        }
        17 => {
            c.w32(0, 0x1u64);
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x7000824cd9); };
            c.w64(3, t26);
            let Some(t27) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000824ce0); };
            c.w64(10, t27);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x7000824ce4); }
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x6u64) { return RegfileExit::fault(0x7000824ce9); }
            let r28 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r28);
            return RegfileExit::jump(0x700086ed74);
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824cfb); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824cff); }
            let r29 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r29);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000824d05); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000824d09); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000824d0d); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000824d11); }
            return c.call(0x70007febf4, 0x7000824d1a, 0x7000824d15);
        }
        19 => {
            c.w64(11, c.r64(0));
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824d1d); };
            c.w64(7, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000824d21); };
            c.w64(3, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000824d25); };
            c.w64(8, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000824d29); };
            c.w64(1, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000824d2d); };
            c.w64(9, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824d34); }
            blk = 7; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824d3a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824d3e); }
            let r35 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r35);
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000824d49); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x7000824d4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000824d51); }
            return c.call(0x70007febf4, 0x7000824d5a, 0x7000824d55);
        }
        21 => {
            c.w64(9, c.r64(0));
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824d5d); };
            c.w64(7, t36);
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000824d61); };
            c.w64(3, t37);
            let Some(t38) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000824d65); };
            c.w64(8, t38);
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000824d69); };
            c.w64(1, t39);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824d70); }
            blk = 10; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000824d78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824d94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824d94); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000824d9a); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6au64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824daa);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1af)) else { return RegfileExit::fault(0x7000824dab); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000824db1); };
            c.w64(8, t4);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), 0x7ff8u64, 32);
            if c.cc_ae() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824de0); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000824de7); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(2).wrapping_add(0x20));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000824df6); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000824dfd); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000824e06); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000824e0e); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000824e12); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824e16); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000824e1d); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000824e25); };
            c.w64(15, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x7000824e2c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824e30); }
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000824e33); };
            c.w32(1, t11);
            let r12 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r12);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x7000824e39); }
            c.w64(1, c.r64(11).wrapping_add(0x18));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000824e44); }
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000824e48); };
            c.w64(9, t13);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000824e4f); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000824e53); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(1));
            blk = 11; continue;
        }
        8 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824e64); };
            c.w64(9, t14);
            let Some(t15) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x7000824e6b); };
            c.w64(11, t15);
            c.w64(15, c.r64(11).wrapping_add(0x18));
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000824e72); };
            c.w64(12, t16);
            let Some(t17) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x7000824e79); };
            c.cmp(t17, c.r64(15), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000824e83); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x7000824e8a); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x7000824e8e); }
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000824e92); };
            c.w64(12, t18);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000824e99); }
            c.w64(1, c.r64(9).wrapping_add(0x10));
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0xd78)) else { return RegfileExit::fault(0x7000824ea1); };
            c.w64(8, t19);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x7000824ea8); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(3)) { return RegfileExit::fault(0x7000824eac); }
            let Some(t20) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x7000824eaf); };
            c.w32(1, t20);
            let r21 = c.add(c.r32(1), 0x1u64, 32);
            c.w32(1, r21);
            if !c.st32(c.r64(3).wrapping_add(0x13), c.r32(1)) { return RegfileExit::fault(0x7000824eb5); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x7000824eb8); }
            c.w32(1, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t22) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x7000824ed4); };
            c.w64(7, t22);
            blk = 14; continue;
        }
        14 => {
            let r23 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r23);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x7000824edf); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x7000824eec);
        }
        16 => {
            blk = 21; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824ef1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824ef5); }
            let r24 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r24);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x7000824efb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000824eff); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000824f03); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x7000824f07); }
            return c.call(0x70007febf4, 0x7000824f10, 0x7000824f0b);
        }
        18 => {
            c.w64(11, c.r64(0));
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000824f13); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824f17); };
            c.w64(3, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000824f1b); };
            c.w64(7, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000824f1f); };
            c.w64(1, t28);
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x7000824f23); };
            c.w64(9, t29);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824f2a); }
            blk = 7; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000824f30); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000824f34); }
            let r30 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r30);
            c.w32(2, 0x18u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x7000824f3f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(7)) { return RegfileExit::fault(0x7000824f43); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x7000824f47); }
            return c.call(0x70007febf4, 0x7000824f50, 0x7000824f4b);
        }
        20 => {
            c.w64(9, c.r64(0));
            let Some(t31) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000824f53); };
            c.w64(8, t31);
            let Some(t32) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000824f57); };
            c.w64(3, t32);
            let Some(t33) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000824f5b); };
            c.w64(7, t33);
            let Some(t34) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000824f5f); };
            c.w64(1, t34);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000824f66); }
            blk = 10; continue;
        }
        21 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000824f6c); }
            let Some(t35) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000824f6d); };
            if !c.push64(t35) { return RegfileExit::fault(0x7000824f6d); }
            let Some(t36) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x7000824f71); };
            c.w64(3, t36);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x7000824f7d); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x7000824f82); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x7000824f87); }
            return RegfileExit::jump(0x700086ed74);
        }
        22 => {
            return RegfileExit::jump(0x7000824f94);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000824fb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000824fb4); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x7000824fba); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x69u64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x7000824fca);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1ab)) else { return RegfileExit::fault(0x7000824fcb); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000824fd1); };
            c.w64(8, t4);
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000824ffc); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000825003); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(2).wrapping_add(0x18));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000825012); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000825019); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000825022); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082502a); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082502e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825032); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000825039); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000825045); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000825049); };
            c.w64(9, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000825050); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000825054); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            blk = 11; continue;
        }
        8 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000825065); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082506c); };
            c.w64(11, t12);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000825073); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082507a); };
            c.cmp(t14, c.r64(15), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000825084); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082508b); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082508f); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825093); };
            c.w64(12, t15);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082509a); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082509e); }
            c.w32(0, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008250ba); };
            c.w64(7, t16);
            blk = 14; continue;
        }
        14 => {
            let r17 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x70008250c5); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x70008250d2);
        }
        16 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008250d5); }
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008250d6); };
            if !c.push64(t18) { return RegfileExit::fault(0x70008250d6); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x70008250da); };
            c.w64(3, t19);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x70008250e6); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x70008250eb); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x70008250f0); }
            return RegfileExit::jump(0x700086ed74);
        }
        17 => {
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x70008250ff); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x7000825106); };
            c.w64(10, t21);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x700082510a); }
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x6u64) { return RegfileExit::fault(0x700082510f); }
            let r22 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(0x700086ed74);
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825121); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000825125); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700082512b); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700082512f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000825133); }
            return c.call(0x70007febf4, 0x700082513c, 0x7000825137);
        }
        19 => {
            c.w64(11, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x700082513f); };
            c.w64(7, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000825143); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000825147); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x700082514b); };
            c.w64(9, t27);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825152); }
            blk = 7; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825158); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082515c); }
            let r28 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r28);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825167); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700082516b); }
            return c.call(0x70007febf4, 0x7000825174, 0x700082516f);
        }
        21 => {
            c.w64(9, c.r64(0));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825177); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082517b); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082517f); };
            c.w64(1, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825186); }
            blk = 10; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000825190);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000825194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000825194); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082519a); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6au64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008251aa);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1af)) else { return RegfileExit::fault(0x70008251ab); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008251b1); };
            c.w64(8, t4);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), 0x7ff8u64, 32);
            if c.cc_ae() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008251dc); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008251e3); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(2).wrapping_add(0x18));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008251f2); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008251f9); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x7000825202); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082520a); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082520e); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825212); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x7000825219); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x7000825225); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x7000825229); };
            c.w64(9, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x7000825230); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x7000825234); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(1));
            blk = 11; continue;
        }
        8 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000825245); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082524c); };
            c.w64(11, t12);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000825253); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082525a); };
            c.cmp(t14, c.r64(15), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000825264); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082526b); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082526f); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825273); };
            c.w64(12, t15);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082527a); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082527e); }
            c.w32(1, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082529a); };
            c.w64(7, t16);
            blk = 14; continue;
        }
        14 => {
            let r17 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x70008252a5); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x70008252b2);
        }
        16 => {
            blk = 21; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008252b7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008252bb); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008252c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008252c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008252c9); }
            return c.call(0x70007febf4, 0x70008252d2, 0x70008252cd);
        }
        18 => {
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008252d5); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008252d9); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008252dd); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008252e1); };
            c.w64(9, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008252e8); }
            blk = 7; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008252ee); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008252f2); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008252fd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000825301); }
            return c.call(0x70007febf4, 0x700082530a, 0x7000825305);
        }
        20 => {
            c.w64(9, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082530d); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825311); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x7000825315); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x700082531c); }
            blk = 10; continue;
        }
        21 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000825322); }
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000825323); };
            if !c.push64(t27) { return RegfileExit::fault(0x7000825323); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x7000825327); };
            c.w64(3, t28);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x7000825333); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x7000825338); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x700082533d); }
            return RegfileExit::jump(0x700086ed74);
        }
        22 => {
            return RegfileExit::jump(0x7000825348);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000825354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000825354); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082535a); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6bu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700082536a);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x700082536b); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000825371); };
            c.w64(8, t4);
            c.w32(9, c.r32(8));
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 16; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 17; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082539c); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008253a3); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(2).wrapping_add(0x18));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008253b2); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008253b9); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008253c2); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008253ca); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008253ce); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008253d2); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008253d9); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008253e5); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008253e9); };
            c.w64(9, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008253f0); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008253f4); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(0));
            blk = 11; continue;
        }
        8 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x7000825405); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x700082540c); };
            c.w64(11, t12);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000825413); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x700082541a); };
            c.cmp(t14, c.r64(15), 64);
            if c.cc_be() { blk = 20; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000825424); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082542b); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082542f); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825433); };
            c.w64(12, t15);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082543a); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082543e); }
            c.w32(0, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082545a); };
            c.w64(7, t16);
            blk = 14; continue;
        }
        14 => {
            let r17 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x7000825465); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x7000825472);
        }
        16 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x7000825475); }
            let Some(t18) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x7000825476); };
            if !c.push64(t18) { return RegfileExit::fault(0x7000825476); }
            let Some(t19) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x700082547a); };
            c.w64(3, t19);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x7000825486); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x700082548b); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825490); }
            return RegfileExit::jump(0x700086ed74);
        }
        17 => {
            c.w32(0, 0x1u64);
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0x27f8)) else { return RegfileExit::fault(0x700082549f); };
            c.w64(3, t20);
            let Some(t21) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008254a6); };
            c.w64(10, t21);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(10)) { return RegfileExit::fault(0x70008254aa); }
            if !c.st64(c.r64(4).wrapping_add(0x10), 0x6u64) { return RegfileExit::fault(0x70008254af); }
            let r22 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r22);
            return RegfileExit::jump(0x700086ed74);
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008254c1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008254c5); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008254cb); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008254cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008254d3); }
            return c.call(0x70007febf4, 0x70008254dc, 0x70008254d7);
        }
        19 => {
            c.w64(11, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008254df); };
            c.w64(7, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008254e3); };
            c.w64(8, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008254e7); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008254eb); };
            c.w64(9, t27);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008254f2); }
            blk = 7; continue;
        }
        20 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008254f8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008254fc); }
            let r28 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r28);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825507); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x700082550b); }
            return c.call(0x70007febf4, 0x7000825514, 0x700082550f);
        }
        21 => {
            c.w64(9, c.r64(0));
            let Some(t29) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825517); };
            c.w64(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x700082551b); };
            c.w64(8, t30);
            let Some(t31) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082551f); };
            c.w64(1, t31);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825526); }
            blk = 10; continue;
        }
        22 => {
            return RegfileExit::jump(0x7000825530);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_7000825534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x7000825534); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x700082553a); };
            c.w32(8, t2);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x6cu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x700082554a);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(0x1b7)) else { return RegfileExit::fault(0x700082554b); };
            c.w32(1, t3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x7000825551); };
            c.w64(8, t4);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 21; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), 0x7ff8u64, 32);
            if c.cc_ae() { blk = 21; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(7, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(9, c.r64(7));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x0u64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x700082557c); };
            c.w64(11, t5);
            let Some(t6) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x7000825583); };
            c.w64(12, t6);
            c.w64(2, c.r64(7).wrapping_mul(2).wrapping_add(0x18));
            c.w64(15, c.r64(2).wrapping_add(c.r64(12)));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x7000825592); };
            c.w64(0, t7);
            let Some(t8) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x7000825599); };
            c.cmp(t8, c.r64(15), 64);
            if c.cc_be() { blk = 17; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(11), c.r64(15)) { return RegfileExit::fault(0x70008255a2); }
            c.w64(11, c.r64(12).wrapping_add(0x1));
            blk = 7; continue;
        }
        7 => {
            if !c.st32(c.r64(11).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008255aa); }
            if !c.st32(c.r64(11).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x70008255ae); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008255b2); };
            c.w64(12, t9);
            if !c.st32(c.r64(11).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x70008255b9); }
            c.w64(1, c.r64(11).wrapping_add(0x10));
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st32(c.r64(11).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008255c5); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008255c9); };
            c.w64(9, t10);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008255d0); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008255d4); }
            c.w64(9, c.r64(11));
            c.w64(12, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(8, c.r64(1));
            blk = 11; continue;
        }
        8 => {
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008255e5); };
            c.w64(9, t11);
            let Some(t12) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008255ec); };
            c.w64(11, t12);
            c.w64(15, c.r64(11).wrapping_add(0x10));
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008255f3); };
            c.w64(12, t13);
            let Some(t14) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008255fa); };
            c.cmp(t14, c.r64(15), 64);
            if c.cc_be() { blk = 19; continue; }
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(9), c.r64(15)) { return RegfileExit::fault(0x7000825604); }
            c.w64(9, c.r64(11).wrapping_add(0x1));
            blk = 10; continue;
        }
        10 => {
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x700082560b); }
            if !c.st32(c.r64(9).wrapping_add(0xb), c.r32(8)) { return RegfileExit::fault(0x700082560f); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x7000825613); };
            c.w64(12, t15);
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(12)) { return RegfileExit::fault(0x700082561a); }
            if !c.st32(c.r64(9).wrapping_add(0x7), c.r32(12)) { return RegfileExit::fault(0x700082561e); }
            c.w32(1, 0x1u64);
            blk = 11; continue;
        }
        11 => {
            c.cmp(c.r32(1), 0x0u64, 32);
            if c.cc_ne() { blk = 15; continue; }
            blk = 12; continue;
        }
        12 => {
            c.w64(1, c.r64(7).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_e() { blk = 15; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x700082563a); };
            c.w64(7, t16);
            blk = 14; continue;
        }
        14 => {
            let r17 = c.sub(c.r64(1), 0x4u64, 64);
            c.w64(1, r17);
            if !c.st32(c.r64(1).wrapping_add(c.r64(12)), c.r32(7)) { return RegfileExit::fault(0x7000825645); }
            c.cmp(c.r64(1), 0x7u64, 64);
            if c.cc_ne() { blk = 14; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(0, c.r64(9));
            return c.ret(0x10, 0x7000825652);
        }
        16 => {
            blk = 21; continue;
        }
        17 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x7000825657); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x700082565b); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x7000825661); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x7000825665); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x7000825669); }
            return c.call(0x70007febf4, 0x7000825672, 0x700082566d);
        }
        18 => {
            c.w64(11, c.r64(0));
            let Some(t19) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x7000825675); };
            c.w64(8, t19);
            let Some(t20) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x7000825679); };
            c.w64(7, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x700082567d); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x7000825681); };
            c.w64(9, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x7000825688); }
            blk = 7; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x700082568e); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x7000825692); }
            let r23 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r23);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x700082569d); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(1)) { return RegfileExit::fault(0x70008256a1); }
            return c.call(0x70007febf4, 0x70008256aa, 0x70008256a5);
        }
        20 => {
            c.w64(9, c.r64(0));
            let Some(t24) = c.ld64(c.r64(5).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008256ad); };
            c.w64(8, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008256b1); };
            c.w64(7, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008256b5); };
            c.w64(1, t26);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008256bc); }
            blk = 10; continue;
        }
        21 => {
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008256c2); }
            let Some(t27) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008256c3); };
            if !c.push64(t27) { return RegfileExit::fault(0x70008256c3); }
            let Some(t28) = c.ld64(c.r64(13).wrapping_add(0x1c98)) else { return RegfileExit::fault(0x70008256c7); };
            c.w64(3, t28);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(8)) { return RegfileExit::fault(0x70008256d3); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(7)) { return RegfileExit::fault(0x70008256d8); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(7)) { return RegfileExit::fault(0x70008256dd); }
            return RegfileExit::jump(0x700086ed74);
        }
        22 => {
            return RegfileExit::jump(0x70008256e8);
        }
            _ => unreachable!(),
        }
    }
}
