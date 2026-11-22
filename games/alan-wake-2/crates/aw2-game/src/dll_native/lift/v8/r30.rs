#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::emu::regfile::{ctx, RegfileExit};

pub unsafe extern "C" fn rf_70008ec6f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ec6f4); }
            c.w64(5, c.r64(4));
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ec6f8); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ec6f9); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ec6fa); }
            let r0 = c.sub(c.r64(4), 0x78u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ec6ff); }
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x60)) else { return RegfileExit::fault(0x70008ec703); };
            c.cmp(c.r64(4), t1, 64);
            if c.cc_be() { blk = 115; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(7, c.r64(5));
            c.w64(9, 0xfffffffffffffff8u64);
            let Some(t2) = c.ld64(c.r64(9).wrapping_add(c.r64(7)).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ec717); };
            c.w64(9, t2);
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008ec71f); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(7).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ec729); };
            c.w64(7, t3);
            blk = 4; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ec72f); };
            c.w64(7, t4);
            blk = 4; continue;
        }
        4 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 113; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ec740); };
            c.cmp(t5, c.r32(7), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t6) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec749); };
            c.w32(12, t6);
            let r7 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r7);
            let Some(t8) = c.ld8(c.r64(12).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ec750); };
            c.logic(t8 & 0x2u64, 8);
            if c.cc_ne() { blk = 9; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 113; continue;
        }
        8 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ec75d); };
            c.w64(7, t9);
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x60), c.r64(7)) { return RegfileExit::fault(0x70008ec764); }
            c.w64(0, c.r64(9));
            return c.call(0x70008a2054, 0x70008ec770, 0x70008ec76b);
        }
        10 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            c.w8(7, (c.cc_e() as u64));
            c.w32(7, c.r8(7));
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008ec77a); }
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec783); };
            c.w32(9, t10);
            let r11 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r11);
            let Some(t12) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec78a); };
            c.cmp(t12, 0x834u64, 16);
            if c.cc_ne() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            let Some(t13) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ec793); };
            c.w32(7, t13);
            let r14 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r14);
            c.w64(9, c.r64(7));
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec79c); };
            c.w64(6, t15);
            blk = 37; continue;
        }
        13 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 15; continue;
        }
        14 => {
            c.w32(9, 0x1u64);
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(9), 0x0u64, 32);
            if c.cc_ne() { blk = 20; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec7b6); };
            c.w64(6, t16);
            let Some(t17) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec7ba); };
            c.w32(9, t17);
            let r18 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r18);
            let Some(t19) = c.ld32(c.r64(9).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ec7c1); };
            c.w32(9, t19);
            let r20 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r20);
            let Some(t21) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec7c8); };
            c.w32(11, t21);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec7cf); };
            c.w32(12, t22);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xb0u64, 64);
            if c.cc_a() { blk = 18; continue; }
            blk = 17; continue;
        }
        17 => {
            return RegfileExit::fault(0x70008ec7df);
        }
        18 => {
            let Some(t23) = c.ld32(c.r64(9).wrapping_add(0x2c7)) else { return RegfileExit::fault(0x70008ec7e0); };
            c.cmp(t23, c.r32(12), 32);
            if c.cc_ne() { blk = 21; continue; }
            blk = 19; continue;
        }
        19 => {
            blk = 117; continue;
        }
        20 => {
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec7ee); };
            c.w64(6, t24);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t25) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec7fb); };
            c.w32(7, t25);
            let r26 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r26);
            let Some(t27) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ec801); };
            c.w32(7, t27);
            let r28 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r28);
            let Some(t29) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec807); };
            c.w32(9, t29);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            let Some(t30) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec80e); };
            c.w32(11, t30);
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(9), 0x44u64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008ec81b);
        }
        24 => {
            let Some(t31) = c.ld32(c.r64(7).wrapping_add(0x117)) else { return RegfileExit::fault(0x70008ec81c); };
            c.cmp(t31, c.r32(11), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r64(9), 0xaeu64, 64);
            if c.cc_a() { blk = 27; continue; }
            blk = 26; continue;
        }
        26 => {
            return RegfileExit::fault(0x70008ec82e);
        }
        27 => {
            let Some(t32) = c.ld32(c.r64(7).wrapping_add(0x2bf)) else { return RegfileExit::fault(0x70008ec82f); };
            c.cmp(t32, c.r32(11), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 28; continue;
        }
        28 => {
            c.cmp(c.r64(9), 0xafu64, 64);
            if c.cc_a() { blk = 118; continue; }
            blk = 29; continue;
        }
        29 => {
            return RegfileExit::fault(0x70008ec845);
        }
        30 => {
            let Some(t33) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ec846); };
            c.w32(7, t33);
            let r34 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r34);
            blk = 31; continue;
        }
        31 => {
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w64(0, c.r64(7));
            return c.call(0x70008a17f4, 0x70008ec85a, 0x70008ec855);
        }
        33 => {
            c.w64(9, c.r64(0));
            let Some(t35) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ec85d); };
            c.w64(6, t35);
            let Some(t36) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ec861); };
            c.w64(0, t36);
            blk = 37; continue;
        }
        34 => {
            c.cmp(c.r32(7), 0x0u64, 32);
            if c.cc_l() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            c.w64(9, c.r64(7));
            blk = 37; continue;
        }
        36 => {
            c.w32(9, 0);
            c.logic(0, 32);
            blk = 37; continue;
        }
        37 => {
            c.logic(c.r8(9) & 0x1u64, 8);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.w32(7, 0);
            c.logic(0, 32);
            if !c.st64(c.r64(5).wrapping_sub(0x68), c.r64(9)) { return RegfileExit::fault(0x70008ec882); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008ec886); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 40; continue; }
            blk = 38; continue;
        }
        38 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ec890); }
            c.pcmpeqd_rr(1, 1);
            c.psllq(1, 0x3f);
            c.psrlq(1, 0x1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 112; continue; }
            blk = 39; continue;
        }
        39 => {
            blk = 41; continue;
        }
        40 => {
            c.cmp(c.r32(9), 0x4u64, 32);
            if c.cc_l() { blk = 112; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t37) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ec8ba); };
            c.w64(12, t37);
            let Some(t38) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ec8be); };
            c.cmp(t38, c.r32(12), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            c.w32(15, 0x822u64);
            blk = 44; continue;
        }
        43 => {
            c.w32(15, 0x824u64);
            blk = 44; continue;
        }
        44 => {
            let Some(t39) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec8d5); };
            c.w32(3, t39);
            let r40 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r40);
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(15)) { return RegfileExit::fault(0x70008ec8db); }
            if !c.st64(c.r64(5).wrapping_sub(0x70), c.r64(3)) { return RegfileExit::fault(0x70008ec8df); }
            let Some(t41) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec8e3); };
            c.cmp(t41, 0x834u64, 16);
            if c.cc_ne() { blk = 46; continue; }
            blk = 45; continue;
        }
        45 => {
            c.w32(2, 0x1u64);
            blk = 47; continue;
        }
        46 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 48; continue;
        }
        48 => {
            let Some(t42) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ec8fd); };
            c.w32(2, t42);
            let r43 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r43);
            let r44 = c.r32(2) & 0x3fu64;
            c.w32(2, r44);
            c.logic(r44, 32);
            c.cmp(c.r32(2), 0x5u64, 32);
            if c.cc_a() { blk = 88; continue; }
            blk = 49; continue;
        }
        49 => {
            let Some(t45) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec910); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            let Some(t47) = c.ld32(c.r64(2).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ec916); };
            c.w32(2, t47);
            let r48 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r48);
            let Some(t49) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec91c); };
            c.w32(1, t49);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x4cu64, 64);
            if c.cc_a() { blk = 51; continue; }
            blk = 50; continue;
        }
        50 => {
            return RegfileExit::fault(0x70008ec92b);
        }
        51 => {
            let Some(t50) = c.ld32(c.r64(2).wrapping_add(0x137)) else { return RegfileExit::fault(0x70008ec92c); };
            c.w32(2, t50);
            let Some(t51) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ec932); };
            c.cmp(t51, c.r32(2), 32);
            if c.cc_ne() { blk = 88; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t52) = c.ld64(c.r64(13).wrapping_add(0xfa8)) else { return RegfileExit::fault(0x70008ec93b); };
            c.w64(2, t52);
            let Some(t53) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ec942); };
            c.cmp(t53, c.r32(7), 32);
            if c.cc_e() { blk = 88; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t54) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ec94b); };
            c.w32(2, t54);
            let r55 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r55);
            let Some(t56) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ec951); };
            c.w32(1, t56);
            let Some(t57) = c.ld32(c.r64(13).wrapping_add(0xe8)) else { return RegfileExit::fault(0x70008ec954); };
            c.cmp(t57, c.r32(1), 32);
            if c.cc_ne() { blk = 81; continue; }
            blk = 54; continue;
        }
        54 => {
            let Some(t58) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ec961); };
            c.w32(1, t58);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_ne() { blk = 80; continue; }
            blk = 55; continue;
        }
        55 => {
            let Some(t59) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ec96d); };
            c.w32(8, t59);
            c.cmp(c.r32(1), c.r32(8), 32);
            if c.cc_a() { blk = 57; continue; }
            blk = 56; continue;
        }
        56 => {
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            c.w64(8, c.r64(1));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ae() { blk = 60; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t60) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008ec985); };
            c.w64(2, t60);
            let Some(t61) = c.ld32(c.r64(2).wrapping_add(0x113)) else { return RegfileExit::fault(0x70008ec98c); };
            c.w32(2, t61);
            let r62 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r62);
            return c.call(0x7000824234, 0x70008ec99a, 0x70008ec995);
        }
        58 => {
            return RegfileExit::fault(0x70008ec99a);
        }
        59 => {
            return RegfileExit::fault(0x70008ec99b);
        }
        60 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 76; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r64(8), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 123; continue; }
            blk = 62; continue;
        }
        62 => {
            let r63 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r63);
            c.w64(12, (c.r32(8) as u32 as i32 as i64 as u64));
            c.w64(15, c.r64(12));
            if let Some(sr) = c.sar(c.r64(15), 1u64, 64) { c.w64(15, sr); }
            c.w32(15, c.r32(15));
            let r64 = c.imul2(c.r32(15), 0x4u64, 32);
            c.w32(15, r64);
            if c.cc_o() { blk = 75; continue; }
            blk = 63; continue;
        }
        63 => {
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            c.w32(6, 0x8u64);
            let r65 = c.add(c.r64(15), c.r64(6), 64);
            c.w64(15, r65);
            if c.cc_o() { blk = 74; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t66) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ec9dd); };
            c.w64(9, t66);
            let Some(t67) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ec9e4); };
            c.w64(9, t67);
            c.w64(11, c.r64(9).wrapping_add(c.r64(15)));
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ec9eb); };
            c.w64(3, t68);
            let Some(t69) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ec9f2); };
            c.cmp(t69, c.r64(11), 64);
            if c.cc_be() { blk = 121; continue; }
            blk = 65; continue;
        }
        65 => {
            c.cmp(c.r64(15), 0x20000u64, 64);
            if c.cc_ae() { blk = 121; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t70) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008eca08); };
            c.w64(15, t70);
            if !c.st64(c.r64(15), c.r64(11)) { return RegfileExit::fault(0x70008eca0f); }
            let r71 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r71);
            blk = 67; continue;
        }
        67 => {
            let Some(t72) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008eca16); };
            c.w64(11, t72);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008eca1d); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008eca21); }
            c.w64(11, c.r64(12).wrapping_mul(2).wrapping_add(0x8));
            c.w64(12, c.r64(6));
            blk = 69; continue;
        }
        68 => {
            if !c.st32(c.r64(9).wrapping_add(c.r64(6)).wrapping_sub(0x1), c.r32(15)) { return RegfileExit::fault(0x70008eca32); }
            let r73 = c.add(c.r64(6), 0x4u64, 64);
            c.w64(6, r73);
            blk = 69; continue;
        }
        69 => {
            c.cmp(c.r64(11), c.r64(6), 64);
            if c.cc_e() { blk = 73; continue; }
            blk = 70; continue;
        }
        70 => {
            c.w64(15, c.r64(1).wrapping_mul(2).wrapping_add(0x8));
            c.cmp(c.r64(15), c.r64(12), 64);
            if c.cc_e() { blk = 72; continue; }
            blk = 71; continue;
        }
        71 => {
            let Some(t74) = c.ld32(c.r64(2).wrapping_add(c.r64(12)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eca4d); };
            c.w32(15, t74);
            let r75 = c.add(c.r64(12), 0x4u64, 64);
            c.w64(12, r75);
            blk = 68; continue;
        }
        72 => {
            let Some(t76) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008eca58); };
            c.w64(15, t76);
            blk = 68; continue;
        }
        73 => {
            c.w64(2, c.r64(9));
            let Some(t77) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008eca64); };
            c.w64(6, t77);
            let Some(t78) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008eca68); };
            c.w64(15, t78);
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008eca6c); };
            c.w64(12, t79);
            let Some(t80) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008eca70); };
            c.w64(9, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008eca74); };
            c.w64(3, t81);
            let Some(t82) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008eca78); };
            c.w64(11, t82);
            blk = 77; continue;
        }
        74 => {
            return RegfileExit::fault(0x70008eca7e);
        }
        75 => {
            return RegfileExit::fault(0x70008eca7f);
        }
        76 => {
            let Some(t83) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008eca80); };
            c.w64(2, t83);
            blk = 77; continue;
        }
        77 => {
            if !c.st32(c.r64(0).wrapping_add(0x7), c.r32(2)) { return RegfileExit::fault(0x70008eca87); }
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 78; continue;
        }
        78 => {
            c.w64(1, 0xfffffffffffc0000u64);
            let r84 = c.r64(1) & c.r64(0);
            c.w64(1, r84);
            c.logic(r84, 64);
            let Some(t85) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008eca99); };
            c.logic(t85 & 0x4u64, 8);
            if c.cc_ne() { blk = 133; continue; }
            blk = 79; continue;
        }
        79 => {
            blk = 81; continue;
        }
        80 => {
            return RegfileExit::fault(0x70008ecaa5);
        }
        81 => {
            let Some(t86) = c.ld8(c.r64(3).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ecaa6); };
            c.w32(2, t86);
            let r87 = c.shr(c.r32(2), 0x2u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r87);
            let r88 = c.r32(2) & 0x3fu64;
            c.w32(2, r88);
            c.logic(r88, 32);
            c.w64(1, c.r64(2));
            let r89 = c.shr(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r89);
            c.cmp(c.r32(1), 0x2u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 82; continue;
        }
        82 => {
            c.cmp(c.r32(2), 0x1u64, 32);
            if c.cc_be() { blk = 84; continue; }
            blk = 83; continue;
        }
        83 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), 0x812u64) { return RegfileExit::fault(0x70008ecabf); }
            c.w32(1, 0x818u64);
            c.w32(8, 0x81eu64);
            c.w32(2, 0x8c2u64);
            blk = 85; continue;
        }
        84 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), 0x810u64) { return RegfileExit::fault(0x70008ecadc); }
            c.w32(1, 0x816u64);
            c.w32(8, 0x81cu64);
            c.w32(2, 0x8c0u64);
            blk = 85; continue;
        }
        85 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(2)) { return RegfileExit::fault(0x70008ecaf7); }
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ecafe); };
            c.w64(2, t90);
            blk = 87; continue;
        }
        86 => {
            c.w32(2, 0x814u64);
            c.w32(1, 0x81au64);
            c.w32(8, 0x820u64);
            if !c.st64(c.r64(5).wrapping_sub(0x88), 0x8beu64) { return RegfileExit::fault(0x70008ecb17); }
            blk = 87; continue;
        }
        87 => {
            if !c.st64(c.r64(5).wrapping_sub(0x90), c.r64(2)) { return RegfileExit::fault(0x70008ecb22); }
            let Some(t91) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ecb29); };
            c.w64(2, t91);
            blk = 89; continue;
        }
        88 => {
            c.w32(2, 0x826u64);
            c.w32(8, 0x8c8u64);
            c.w32(1, 0x8c6u64);
            if !c.st64(c.r64(5).wrapping_sub(0x90), 0x8c4u64) { return RegfileExit::fault(0x70008ecb42); }
            blk = 89; continue;
        }
        89 => {
            if !c.st64(c.r64(5).wrapping_sub(0x88), c.r64(2)) { return RegfileExit::fault(0x70008ecb4d); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(8)) { return RegfileExit::fault(0x70008ecb54); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(1)) { return RegfileExit::fault(0x70008ecb58); }
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 90; continue;
        }
        90 => {
            c.w64(11, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            blk = 94; continue;
        }
        91 => {
            if !c.movsd_rm(0, c.r64(9).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ecb6a); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_ns() { blk = 94; continue; }
            blk = 92; continue;
        }
        92 => {
            c.w64(10, 0xc3e0000000000000u64);
            c.movq_xr(15, 10);
            c.arith_sd(0, 15, c.xmm_u64(0));
            let cv = c.cvttsd2si64(c.xmm_u64(15)); c.w64(11, cv);
            c.logic(c.r64(11) & c.r64(11), 64);
            if c.cc_s() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            c.w64(10, 0x8000000000000000u64);
            let r92 = c.r64(11) | c.r64(10);
            c.w64(11, r92);
            c.logic(r92, 64);
            blk = 94; continue;
        }
        94 => {
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_be() { blk = 96; continue; }
            blk = 95; continue;
        }
        95 => {
            c.w32(11, 0x3fffffffu64);
            blk = 96; continue;
        }
        96 => {
            let Some(t93) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ecbb4); };
            c.w32(7, t93);
            let r94 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r94);
            let Some(t95) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ecbba); };
            c.w32(7, t95);
            let r96 = c.sub(c.r32(7), 0xbdu64, 32);
            c.w32(7, r96);
            c.cmp(c.r32(7), 0x78bu64, 32);
            if c.cc_a() { blk = 100; continue; }
            blk = 97; continue;
        }
        97 => {
            let Some(t97) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ecbcc); };
            c.w32(7, t97);
            let r98 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r98);
            let Some(t99) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ecbd2); };
            c.w32(7, t99);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x0u64, 64);
            if c.cc_e() { blk = 100; continue; }
            blk = 98; continue;
        }
        98 => {
            c.cmp(c.r64(11), c.r64(7), 64);
            if c.cc_le() { blk = 100; continue; }
            blk = 99; continue;
        }
        99 => {
            c.w64(11, c.r64(7));
            blk = 100; continue;
        }
        100 => {
            let Some(t100) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ecbe9); };
            c.w64(7, t100);
            let Some(t101) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ecbf0); };
            c.w64(9, t101);
            if !c.st64(c.r64(5).wrapping_sub(0x80), c.r64(11)) { return RegfileExit::fault(0x70008ecbf3); }
            c.w64(11, c.r64(9).wrapping_add(0x15c));
            let Some(t102) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ecbfe); };
            c.w64(12, t102);
            let Some(t103) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008ecc05); };
            c.cmp(t103, c.r64(11), 64);
            if c.cc_be() { blk = 125; continue; }
            blk = 101; continue;
        }
        101 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008ecc0f); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 102; continue;
        }
        102 => {
            let Some(t104) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008ecc16); };
            c.w64(9, t104);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008ecc1d); }
            c.w32(11, 0xaau64);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008ecc27); }
            c.w64(11, c.r64(7).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r105 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r105);
            let r106 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r106);
            c.logic(r106, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ecc3a); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008ecc3f); }
            c.w32(8, 0x154u64);
            c.w64(1, c.r64(11));
            c.w32(2, 0);
            c.logic(0, 32);
            c.w64(12, c.r64(8));
            let Some(t107) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008ecc51); };
            c.w64(0, t107);
            c.w64(10, 0x70008ecc5fu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ecc5f); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ecc63); }
            return c.call(c.r64(0), 0x70008ecc69, 0x70008ecc67);
        }
        103 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ecc69); }
            let Some(t108) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ecc71); };
            c.w64(4, t108);
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ecc76); };
            c.cmp(t109, 0x7fffffdu64, 64);
            if c.cc_g() { blk = 131; continue; }
            blk = 104; continue;
        }
        104 => {
            let Some(t110) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ecc84); };
            c.w64(7, t110);
            let Some(t111) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ecc8b); };
            c.w64(9, t111);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ecc8e); };
            c.w64(11, t112);
            c.w64(8, c.r64(11).wrapping_mul(4));
            c.w64(2, c.r64(8).wrapping_add(0x8));
            c.w64(12, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ecca2); };
            c.w64(15, t113);
            let r114 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r114);
            let Some(t115) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008eccaa); };
            c.w64(0, t115);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(15)) { return RegfileExit::fault(0x70008eccb1); }
            let Some(t116) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008eccb5); };
            c.cmp(t116, c.r64(12), 64);
            if c.cc_be() { blk = 127; continue; }
            blk = 105; continue;
        }
        105 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 127; continue; }
            blk = 106; continue;
        }
        106 => {
            if !c.st64(c.r64(7), c.r64(12)) { return RegfileExit::fault(0x70008ecccb); }
            let r117 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r117);
            blk = 107; continue;
        }
        107 => {
            let r118 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r118);
            let Some(t119) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008eccd5); };
            c.w64(12, t119);
            if !c.st32(c.r64(9).wrapping_sub(0x1), c.r32(12)) { return RegfileExit::fault(0x70008eccdc); }
            if !c.st32(c.r64(9).wrapping_add(0x3), c.r32(11)) { return RegfileExit::fault(0x70008ecce0); }
            c.w64(1, c.r64(9).wrapping_add(0x7));
            c.w64(10, c.r64(4));
            let r120 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r120);
            let r121 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r121);
            c.logic(r121, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008eccf3); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(9)) { return RegfileExit::fault(0x70008eccf8); }
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t122) = c.ld64(c.r64(13).wrapping_add(0x13d8)) else { return RegfileExit::fault(0x70008eccfe); };
            c.w64(0, t122);
            c.w64(10, 0x70008ecd0cu64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ecd0c); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ecd10); }
            return c.call(c.r64(0), 0x70008ecd16, 0x70008ecd14);
        }
        108 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ecd16); }
            let Some(t123) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ecd1e); };
            c.w64(4, t123);
            let Some(t124) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ecd23); };
            c.w64(7, t124);
            let Some(t125) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ecd2a); };
            c.w64(9, t125);
            c.w64(11, c.r64(9).wrapping_add(0x44));
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ecd31); };
            c.w64(12, t126);
            let Some(t127) = c.ld64(c.r64(12)) else { return RegfileExit::fault(0x70008ecd38); };
            c.cmp(t127, c.r64(11), 64);
            if c.cc_be() { blk = 129; continue; }
            blk = 109; continue;
        }
        109 => {
            if !c.st64(c.r64(7), c.r64(11)) { return RegfileExit::fault(0x70008ecd42); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 110; continue;
        }
        110 => {
            let Some(t128) = c.ld64(c.r64(13).wrapping_add(0xed0)) else { return RegfileExit::fault(0x70008ecd49); };
            c.w64(9, t128);
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(9)) { return RegfileExit::fault(0x70008ecd50); }
            let Some(t129) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ecd54); };
            c.w64(0, t129);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008ecd58); }
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x70)) else { return RegfileExit::fault(0x70008ecd5b); };
            c.w64(9, t130);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(9)) { return RegfileExit::fault(0x70008ecd5f); }
            let Some(t131) = c.ld64(c.r64(5).wrapping_sub(0x68)) else { return RegfileExit::fault(0x70008ecd63); };
            c.w64(9, t131);
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(9)) { return RegfileExit::fault(0x70008ecd67); }
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x60)) else { return RegfileExit::fault(0x70008ecd6b); };
            c.w64(9, t132);
            if !c.st32(c.r64(7).wrapping_add(0xf), c.r32(9)) { return RegfileExit::fault(0x70008ecd6f); }
            let Some(t133) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008ecd73); };
            c.w64(15, t133);
            if !c.st32(c.r64(7).wrapping_add(0x13), c.r32(15)) { return RegfileExit::fault(0x70008ecd77); }
            let Some(t134) = c.ld64(c.r64(5).wrapping_sub(0x90)) else { return RegfileExit::fault(0x70008ecd7b); };
            c.w64(2, t134);
            if !c.st32(c.r64(7).wrapping_add(0x17), c.r32(2)) { return RegfileExit::fault(0x70008ecd82); }
            let Some(t135) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ecd85); };
            c.w64(1, t135);
            if !c.st32(c.r64(7).wrapping_add(0x1b), c.r32(1)) { return RegfileExit::fault(0x70008ecd89); }
            let Some(t136) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ecd8c); };
            c.w64(8, t136);
            if !c.st32(c.r64(7).wrapping_add(0x1f), c.r32(8)) { return RegfileExit::fault(0x70008ecd90); }
            let Some(t137) = c.ld64(c.r64(5).wrapping_sub(0x88)) else { return RegfileExit::fault(0x70008ecd94); };
            c.w64(2, t137);
            if !c.st32(c.r64(7).wrapping_add(0x23), c.r32(2)) { return RegfileExit::fault(0x70008ecd9b); }
            c.w32(9, 0xeu64);
            if !c.st32(c.r64(7).wrapping_add(0x27), c.r32(9)) { return RegfileExit::fault(0x70008ecda4); }
            c.w32(6, 0);
            c.logic(0, 32);
            if !c.st32(c.r64(7).wrapping_add(0x2b), c.r32(6)) { return RegfileExit::fault(0x70008ecdaa); }
            let Some(t138) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ecdad); };
            c.w64(9, t138);
            if !c.st32(c.r64(7).wrapping_add(0x2f), c.r32(9)) { return RegfileExit::fault(0x70008ecdb1); }
            let Some(t139) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ecdb5); };
            c.w64(9, t139);
            if !c.st32(c.r64(7).wrapping_add(0x33), c.r32(9)) { return RegfileExit::fault(0x70008ecdb9); }
            let Some(t140) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ecdbd); };
            c.w64(9, t140);
            if !c.st32(c.r64(7).wrapping_add(0x37), c.r32(9)) { return RegfileExit::fault(0x70008ecdc4); }
            if !c.st32(c.r64(7).wrapping_add(0x3b), c.r32(6)) { return RegfileExit::fault(0x70008ecdc8); }
            if !c.st32(c.r64(7).wrapping_add(0x3f), c.r32(6)) { return RegfileExit::fault(0x70008ecdcb); }
            c.w64(0, c.r64(7));
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ecdd1); };
            c.w64(6, t141);
            return c.call(0x70008eb834, 0x70008ecdda, 0x70008ecdd5);
        }
        111 => {
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ecdda); };
            c.w64(0, t142);
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ecdde); };
            c.w64(7, t143);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ecde5); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ecde6); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ecdec); }
            return c.ret(0x0, 0x70008ecdee);
        }
        112 => {
            let Some(t144) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ecdef); };
            c.w64(7, t144);
            let r145 = c.add(c.r64(7), 0x1u64, 64);
            c.w64(7, r145);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ecdfa); }
            if !c.pop64_reg(10) { return RegfileExit::fault(0x70008ecdfb); }
            c.w64(4, c.r64(4).wrapping_add(c.r64(7).wrapping_mul(8)));
            if !c.push64(c.r64(10)) { return RegfileExit::fault(0x70008ece01); }
            return c.ret(0x0, 0x70008ece03);
        }
        113 => {
            c.w32(9, 0x24u64);
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ece0a); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ece0c); }
            let Some(t146) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008ece0d); };
            c.w64(3, t146);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ece1e, 0x70008ece19);
        }
        114 => {
            return RegfileExit::fault(0x70008ece1e);
        }
        115 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008ece1f); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t147) = c.ld64(c.r64(13).wrapping_add(0x21a8)) else { return RegfileExit::fault(0x70008ece25); };
            c.w64(3, t147);
            c.w64(7, c.r64(0));
            return c.call(0x700086ed74, 0x70008ece34, 0x70008ece2f);
        }
        116 => {
            let Some(t148) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ece34); };
            c.w64(6, t148);
            let Some(t149) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ece38); };
            c.w64(0, t149);
            blk = 1; continue;
        }
        117 => {
            let Some(t150) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ece41); };
            c.w32(7, t150);
            let r151 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r151);
            blk = 31; continue;
        }
        118 => {
            let Some(t152) = c.ld32(c.r64(7).wrapping_add(0x2c3)) else { return RegfileExit::fault(0x70008ece4c); };
            c.cmp(t152, c.r32(11), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t153) = c.ld64(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008ece59); };
            c.w64(3, t153);
            return c.call(0x7000870914, 0x70008ece65, 0x70008ece60);
        }
        120 => {
            c.w64(7, c.r64(0));
            let Some(t154) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ece68); };
            c.w64(6, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ece6c); };
            c.w64(0, t155);
            blk = 31; continue;
        }
        121 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(2)) { return RegfileExit::fault(0x70008ece75); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(1)) { return RegfileExit::fault(0x70008ece79); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(12)) { return RegfileExit::fault(0x70008ece7d); }
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(8)) { return RegfileExit::fault(0x70008ece81); }
            c.w64(2, c.r64(15));
            return c.call(0x70007febb4, 0x70008ece8d, 0x70008ece88);
        }
        122 => {
            c.w64(9, c.r64(0));
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ece90); };
            c.w64(0, t156);
            c.w32(7, 0);
            c.logic(0, 32);
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ece96); };
            c.w64(2, t157);
            let Some(t158) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008ece9a); };
            c.w64(1, t158);
            c.w32(6, 0x8u64);
            let Some(t159) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008ecea3); };
            c.w64(12, t159);
            let Some(t160) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008ecea7); };
            c.w64(8, t160);
            blk = 67; continue;
        }
        123 => {
            let Some(t161) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008eceb0); };
            c.w64(3, t161);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008ecec1, 0x70008ecebc);
        }
        124 => {
            return RegfileExit::fault(0x70008ecec1);
        }
        125 => {
            c.w32(2, 0x15cu64);
            c.w64(7, c.r64(2));
            return c.call(0x70007febb4, 0x70008ececf, 0x70008ececa);
        }
        126 => {
            c.w64(7, c.r64(0));
            blk = 102; continue;
        }
        127 => {
            if !c.st64(c.r64(5).wrapping_sub(0x78), c.r64(8)) { return RegfileExit::fault(0x70008eced7); }
            return c.call(0x70007febb4, 0x70008ecee0, 0x70008ecedb);
        }
        128 => {
            c.w64(9, c.r64(0));
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x78)) else { return RegfileExit::fault(0x70008ecee3); };
            c.w64(8, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x80)) else { return RegfileExit::fault(0x70008ecee7); };
            c.w64(11, t163);
            blk = 107; continue;
        }
        129 => {
            c.w32(2, 0x44u64);
            return c.call(0x70007febb4, 0x70008ecefa, 0x70008ecef5);
        }
        130 => {
            c.w64(7, c.r64(0));
            blk = 110; continue;
        }
        131 => {
            let Some(t164) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008ecf02); };
            c.w64(3, t164);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ecf12, 0x70008ecf0d);
        }
        132 => {
            return RegfileExit::fault(0x70008ecf12);
        }
        133 => {
            c.w32(2, c.r32(2));
            let r165 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r165);
            c.w64(1, 0xfffffffffffc0000u64);
            let r166 = c.r64(1) & c.r64(2);
            c.w64(1, r166);
            c.logic(r166, 64);
            let Some(t167) = c.ld8(c.r64(1).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ecf22); };
            c.logic(t167 & 0x2u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 134; continue;
        }
        134 => {
            c.w64(8, c.r64(0).wrapping_add(0x7));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ecf30); }
            c.w64(7, c.r64(0));
            return c.call(0x70007f0e54, 0x70008ecf39, 0x70008ecf34);
        }
        135 => {
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ecf39); }
            blk = 79; continue;
        }
        136 => {
            return RegfileExit::jump(0x70008ecf40);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ecf54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ecf54); };
            c.w64(3, t0);
            let Some(t1) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ecf5b); };
            c.w64(1, t1);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ecf62); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ecf69); };
            c.cmp(t3, c.r64(6), 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008ecf6e); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ecf7d); };
            c.w64(1, t4);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ecf84); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ecf87); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008ecf8f);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ecf90); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ecf94); }
            let r5 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r5);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ecf9f); }
            return c.call(0x70007febf4, 0x70008ecfa8, 0x70008ecfa3);
        }
        4 => {
            c.w64(3, c.r64(0));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ecfab); };
            c.w64(0, t6);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ecfb2); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008ecfb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ecfd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ecfd4); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ecfd7); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ecfe0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 11; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(0, c.r32(0));
            return c.ret(0x0, 0x70008ecff2);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ecff3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ecff7); }
            return c.call(0x70008a1014, 0x70008ecffe, 0x70008ecff9);
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ed002); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(0, cv);
            c.cmp(c.r64(0), 0x1u64, 64);
            if c.cc_o() { blk = 9; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w32(0, c.r32(0));
            blk = 7; continue;
        }
        7 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ed017); }
            return c.ret(0x0, 0x70008ed018);
        }
        8 => {
            let r2 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r2);
            blk = 7; continue;
        }
        9 => {
            let r3 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r3);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008ed021); }
            return c.call(0x70008708b4, 0x70008ed02b, 0x70008ed026);
        }
        10 => {
            let Some(t4) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008ed02b); };
            c.w32(0, t4);
            let r5 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r5);
            blk = 6; continue;
        }
        11 => {
            let r6 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r6);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008ed038); }
            return c.call(0x70008708b4, 0x70008ed042, 0x70008ed03d);
        }
        12 => {
            let Some(t7) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008ed042); };
            c.w32(0, t7);
            let r8 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r8);
            blk = 2; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008ed04c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ed058); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ed05b); };
            c.cmp(t1, c.r32(3), 32);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ed064); }
            return c.ret(0x0, 0x70008ed069);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ed06a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ed06e); }
            return c.call(0x70008a1014, 0x70008ed075, 0x70008ed070);
        }
        4 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(0).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ed079); }
            blk = 6; continue;
        }
        6 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ed081); }
            return c.ret(0x0, 0x70008ed082);
        }
        7 => {
            let r2 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r2);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            blk = 6; continue;
        }
        8 => {
            let r3 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r3);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            return c.ret(0x0, 0x70008ed099);
        }
        9 => {
            return RegfileExit::jump(0x70008ed09c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed0b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, (c.r32(0) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(2), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ed0c0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ed0c4); }
            let Some(t0) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ed0c6); };
            c.w64(2, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed0ca); };
            c.w64(2, t1);
            c.w32(1, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed0d1); };
            c.w32(6, t2);
            let r3 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r3);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ed0da); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed0db); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2b68)) else { return RegfileExit::fault(0x70008ed0dc); };
            c.w64(3, t4);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ed0ed, 0x70008ed0e8);
        }
        2 => {
            let r5 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r5);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ed0f2); }
            return c.ret(0x0, 0x70008ed0f3);
        }
        3 => {
            c.w32(0, 0xffffffffu64);
            return c.ret(0x0, 0x70008ed0f9);
        }
        4 => {
            return RegfileExit::jump(0x70008ed0fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed114(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed117); };
            c.w64(8, t0);
            c.w32(9, c.r32(0));
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(11, c.r32(3));
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(1));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed13e); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let r3 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let r5 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r5);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ed150); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ed151); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ed152); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008ed154); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed156); }
            let Some(t6) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008ed157); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008ed157); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2ba8)) else { return RegfileExit::fault(0x70008ed15b); };
            c.w64(3, t7);
            c.w32(0, 0x6u64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(8)) { return RegfileExit::fault(0x70008ed167); }
            return RegfileExit::jump(0x700086ed74);
        }
        4 => {
            return RegfileExit::jump(0x70008ee674);
        }
        5 => {
            return RegfileExit::jump(0x70008ed178);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed197); };
            c.w64(8, t0);
            c.w32(9, c.r32(0));
            c.cmp(c.r64(9), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(11, c.r32(3));
            c.cmp(c.r64(11), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(1, c.r32(1));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed1be); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let r3 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r3);
            let r4 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r4);
            let r5 = c.shl(c.r32(9), 1u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r5);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ed1d0); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ed1d1); }
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ed1d2); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008ed1d4); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed1d6); }
            let Some(t6) = c.ld64(c.r64(4).wrapping_add(0x28)) else { return RegfileExit::fault(0x70008ed1d7); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008ed1d7); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2bb0)) else { return RegfileExit::fault(0x70008ed1db); };
            c.w64(3, t7);
            c.w32(0, 0x6u64);
            if !c.st64(c.r64(4).wrapping_add(0x30), c.r64(8)) { return RegfileExit::fault(0x70008ed1e7); }
            return RegfileExit::jump(0x700086ed74);
        }
        4 => {
            return RegfileExit::jump(0x70008ee674);
        }
        5 => {
            return RegfileExit::jump(0x70008ed1f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(7, c.r64(5));
            let Some(t0) = c.ld64(c.r64(7).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed217); };
            c.w64(7, t0);
            c.w32(8, c.r32(3));
            c.cmp(c.r64(8), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(1, c.r32(1));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t1) = c.ld32(c.r64(7).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed232); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let r3 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r3);
            let r4 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r4);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ed240); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ed241); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008ed243); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed244); }
            let Some(t5) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ed245); };
            if !c.push64(t5) { return RegfileExit::fault(0x70008ed245); }
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2bc0)) else { return RegfileExit::fault(0x70008ed249); };
            c.w64(3, t6);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(7)) { return RegfileExit::fault(0x70008ed255); }
            return RegfileExit::jump(0x700086ed74);
        }
        3 => {
            return RegfileExit::jump(0x70008ee674);
        }
        4 => {
            return RegfileExit::jump(0x70008ed268);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed277); };
            c.w64(2, t0);
            c.w32(7, c.r32(3));
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed286); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let r3 = c.shl(c.r32(7), 1u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r3);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ed291); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed292); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ed293); }
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ed294); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008ed294); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2bb8)) else { return RegfileExit::fault(0x70008ed298); };
            c.w64(3, t5);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x70008ed2a4); }
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            c.w64(0, 0xfffffffffffffffeu64);
            return c.ret(0x0, 0x70008ed2b5);
        }
        3 => {
            return RegfileExit::jump(0x70008ed2b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed2d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed2d7); };
            c.w64(2, t0);
            c.w64(1, (c.r32(3) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(1), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 10; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xdb)) else { return RegfileExit::fault(0x70008ed2eb); };
            c.w32(7, t1);
            let r2 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r2);
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ed2f4); };
            c.w32(8, t3);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(0), c.r64(8), 64);
            if c.cc_ae() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ed307); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            let Some(t6) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ed30e); };
            c.w32(8, t6);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), c.r64(1), 64);
            if c.cc_le() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ed31d); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ed323); };
            c.w32(8, t9);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(1), c.r64(8), 64);
            if c.cc_ae() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ed332); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t12) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ed33e); };
            c.w32(7, t12);
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xe38)) else { return RegfileExit::fault(0x70008ed341); };
            c.cmp(t13, c.r32(7), 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed34a); };
            c.w32(6, t14);
            let r15 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r15);
            c.w32(1, c.r64(3).wrapping_add(c.r64(3)));
            c.w32(7, c.r64(0).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ed359); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed35a); }
            let Some(t16) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ed35b); };
            if !c.push64(t16) { return RegfileExit::fault(0x70008ed35b); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2b98)) else { return RegfileExit::fault(0x70008ed35f); };
            c.w64(3, t17);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x70008ed36b); }
            return RegfileExit::jump(0x700086ed74);
        }
        7 => {
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008ed378);
        }
        8 => {
            return RegfileExit::fault(0x70008ed379);
        }
        9 => {
            return RegfileExit::fault(0x70008ed37a);
        }
        10 => {
            return RegfileExit::jump(0x70008ee674);
        }
        11 => {
            return RegfileExit::jump(0x70008ed384);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed394(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed397); };
            c.w64(2, t0);
            c.w64(7, (c.r32(3) as u32 as i32 as i64 as u64));
            c.cmp(c.r64(7), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xdb)) else { return RegfileExit::fault(0x70008ed3ab); };
            c.w32(8, t1);
            let r2 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ed3b5); };
            c.w32(9, t3);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(0), c.r64(9), 64);
            if c.cc_ae() { blk = 10; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(8).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ed3c8); };
            c.w32(8, t4);
            let r5 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r5);
            let Some(t6) = c.ld32(c.r64(8).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ed3d0); };
            c.w32(9, t6);
            c.cmp(c.r32(9), 0x3d09056u64, 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(9), 0x3d09054u64, 32);
            if c.cc_ne() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ed3e6); };
            c.w32(2, t7);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(2), c.r64(7), 64);
            if c.cc_le() { blk = 12; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(8).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ed3f5); };
            c.w32(2, t8);
            let r9 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r9);
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ed3fc); };
            c.w32(8, t10);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(7), c.r64(8), 64);
            if c.cc_ae() { blk = 11; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, c.r64(7).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(2).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x70008ed413); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(8, 0xfffffffffffc0000u64);
            let r11 = c.r64(8) & c.r64(2);
            c.w64(8, r11);
            c.logic(r11, 64);
            let Some(t12) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ed425); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ed42c); };
            c.w64(0, t13);
            return c.ret(0x0, 0x70008ed433);
        }
        9 => {
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed434); };
            c.w32(6, t14);
            let r15 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r15);
            c.w32(7, c.r64(3).wrapping_add(c.r64(3)));
            c.w32(8, c.r64(0).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ed444); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ed446); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed447); }
            let Some(t16) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008ed448); };
            if !c.push64(t16) { return RegfileExit::fault(0x70008ed448); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x2ba0)) else { return RegfileExit::fault(0x70008ed44c); };
            c.w64(3, t17);
            c.w32(0, 0x4u64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(2)) { return RegfileExit::fault(0x70008ed458); }
            return RegfileExit::jump(0x700086ed74);
        }
        10 => {
            return RegfileExit::fault(0x70008ed462);
        }
        11 => {
            return RegfileExit::fault(0x70008ed463);
        }
        12 => {
            return RegfileExit::jump(0x70008ee674);
        }
        13 => {
            c.w32(1, c.r32(1));
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            c.w64(8, 0xfffffffffffc0000u64);
            let r19 = c.r64(8) & c.r64(1);
            c.w64(8, r19);
            c.logic(r19, 64);
            let Some(t20) = c.ld8(c.r64(8).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ed478); };
            c.logic(t20 & 0x2u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w64(9, c.r64(2).wrapping_add(c.r64(7)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008ed483); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008ed484); }
            c.w64(7, c.r64(2));
            c.w64(8, c.r64(9));
            return c.call(0x70007f1594, 0x70008ed491, 0x70008ed48c);
        }
        15 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008ed491); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008ed493); }
            blk = 8; continue;
        }
        16 => {
            return RegfileExit::jump(0x70008ed498);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed4b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed4b7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xeb)) else { return RegfileExit::fault(0x70008ed4bb); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ed4c4); };
            c.cmp(t3, c.r32(1), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ed4cd); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            c.w32(8, c.r32(0));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(8), c.r64(7), 64);
            if c.cc_ae() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ed4de); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ed4e6); };
            c.cmp(t7, c.r32(1), 32);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008ed4f2);
        }
        4 => {
            return RegfileExit::fault(0x70008ed4f3);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed4f4); };
            c.w32(6, t8);
            let r9 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r9);
            c.w32(1, c.r64(0).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008ed500); }
            let Some(t10) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ed501); };
            if !c.push64(t10) { return RegfileExit::fault(0x70008ed501); }
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2b90)) else { return RegfileExit::fault(0x70008ed505); };
            c.w64(3, t11);
            c.w32(0, 0x2u64);
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(2)) { return RegfileExit::fault(0x70008ed511); }
            return RegfileExit::jump(0x700086ed74);
        }
        6 => {
            return RegfileExit::jump(0x70008ed51c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r64(0), 0x0u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(0), 0x7fffffdu64, 64);
            if c.cc_g() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ed542); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ed549); };
            c.w64(7, t1);
            c.w64(2, c.r64(0).wrapping_mul(4).wrapping_add(0x8));
            c.w64(8, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ed558); };
            c.w64(9, t2);
            let Some(t3) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ed55f); };
            c.cmp(t3, c.r64(8), 64);
            if c.cc_be() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ed564); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 4; continue;
        }
        4 => {
            c.w32(7, c.r64(0).wrapping_add(c.r64(0)));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008ed56e); };
            c.w64(8, t4);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008ed575); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(7)) { return RegfileExit::fault(0x70008ed579); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008ed57f);
        }
        5 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ed580); };
            c.w64(0, t5);
            return c.ret(0x0, 0x70008ed587);
        }
        6 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ed588); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ed58c); }
            let r6 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r6);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ed592); }
            return c.call(0x70007febf4, 0x70008ed59b, 0x70008ed596);
        }
        7 => {
            c.w64(1, c.r64(0));
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed59e); };
            c.w64(0, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ed5a5); }
            blk = 4; continue;
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ed5a8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ed5ac); }
            let r8 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r8);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2120)) else { return RegfileExit::fault(0x70008ed5b2); };
            c.w64(3, t9);
            c.w32(0, 0);
            c.logic(0, 32);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008ed5c2, 0x70008ed5bd);
        }
        9 => {
            return RegfileExit::fault(0x70008ed5c2);
        }
        10 => {
            return RegfileExit::jump(0x70008ed5c4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed5d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed5d7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed5db); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008ed5e4); }
            let Some(t3) = c.ld64(c.r64(4).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008ed5e5); };
            if !c.push64(t3) { return RegfileExit::fault(0x70008ed5e5); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ed5e9); }
            c.w32(0, 0x2u64);
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2b78)) else { return RegfileExit::fault(0x70008ed5f3); };
            c.w64(3, t4);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ed600);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ed614); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed620); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed624); };
            c.w32(6, t2);
            let r3 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r3);
            let Some(t4) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008ed62d); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008ed62d); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2b80)) else { return RegfileExit::fault(0x70008ed630); };
            c.w64(3, t5);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008ed637); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        2 => {
            return RegfileExit::jump(0x70008ee694);
        }
        3 => {
            return RegfileExit::jump(0x70008ed64c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed657); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed65b); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed65f); };
            c.w32(6, t2);
            let r3 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r3);
            let Some(t4) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008ed668); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008ed668); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2be0)) else { return RegfileExit::fault(0x70008ed66b); };
            c.w64(3, t5);
            c.w32(0, 0x1u64);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(2)) { return RegfileExit::fault(0x70008ed677); }
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ed684);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed697); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed69b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2b70)) else { return RegfileExit::fault(0x70008ed6a4); };
            c.w64(3, t3);
            c.w32(0, 0);
            c.logic(0, 32);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ed6b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed6d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed6d7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed6db); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2b48)) else { return RegfileExit::fault(0x70008ed6e4); };
            c.w64(3, t3);
            c.w32(0, 0);
            c.logic(0, 32);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ed6f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed717); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed71b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008ed724); };
            if !c.push64(t3) { return RegfileExit::fault(0x70008ed724); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2ca8)) else { return RegfileExit::fault(0x70008ed727); };
            c.w64(3, t4);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008ed72e); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ed740);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed757); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed75b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2c98)) else { return RegfileExit::fault(0x70008ed764); };
            c.w64(3, t3);
            c.w32(0, 0);
            c.logic(0, 32);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ed774);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed794(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed797); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed79b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008ed7a4); };
            if !c.push64(t3) { return RegfileExit::fault(0x70008ed7a4); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2ca0)) else { return RegfileExit::fault(0x70008ed7a7); };
            c.w64(3, t4);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008ed7ae); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ed7c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed7d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ed7d4); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ed7da); };
            c.w32(3, t2);
            let r3 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r3);
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ed7e0); };
            c.w32(1, t4);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.cmp(c.r64(1), 0x6bu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008ed7ef);
        }
        2 => {
            c.w64(1, (c.r32(0) as u32 as i32 as i64 as u64));
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0x1b3)) else { return RegfileExit::fault(0x70008ed7f3); };
            c.w32(3, t5);
            c.w64(6, c.r64(1));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(6), 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ed805); };
            c.w64(7, t6);
            let Some(t7) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ed80c); };
            c.w64(8, t7);
            c.w64(2, c.r64(1).wrapping_mul(2).wrapping_add(0x18));
            c.w64(9, c.r64(2).wrapping_add(c.r64(8)));
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ed81b); };
            c.w64(11, t8);
            let Some(t9) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ed822); };
            c.cmp(t9, c.r64(9), 64);
            if c.cc_be() { blk = 14; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(7), c.r64(9)) { return RegfileExit::fault(0x70008ed82b); }
            c.w64(7, c.r64(8).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008ed832); }
            if !c.st32(c.r64(7).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008ed835); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ed838); };
            c.w64(8, t10);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008ed83f); }
            c.w64(3, c.r64(7).wrapping_add(0x10));
            let r11 = c.shl(c.r32(6), 1u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r11);
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008ed849); }
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0xe0)) else { return RegfileExit::fault(0x70008ed84c); };
            c.w64(8, t12);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008ed853); }
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008ed857); }
            c.w64(6, c.r64(7));
            c.w64(8, c.r64(3));
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 9; continue;
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ed864); };
            c.w64(6, t13);
            let Some(t14) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008ed86b); };
            c.w64(7, t14);
            c.w64(9, c.r64(7).wrapping_add(0x10));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ed872); };
            c.w64(8, t15);
            let Some(t16) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ed879); };
            c.cmp(t16, c.r64(9), 64);
            if c.cc_be() { blk = 16; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(6), c.r64(9)) { return RegfileExit::fault(0x70008ed882); }
            c.w64(6, c.r64(7).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            if !c.st32(c.r64(6).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008ed889); }
            if !c.st32(c.r64(6).wrapping_add(0xb), c.r32(0)) { return RegfileExit::fault(0x70008ed88c); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ed88f); };
            c.w64(8, t17);
            if !c.st32(c.r64(6).wrapping_add(0x3), c.r32(8)) { return RegfileExit::fault(0x70008ed896); }
            if !c.st32(c.r64(6).wrapping_add(0x7), c.r32(8)) { return RegfileExit::fault(0x70008ed89a); }
            c.w32(3, 0x1u64);
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_ne() { blk = 13; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w64(3, c.r64(1).wrapping_mul(2).wrapping_add(0x7));
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_e() { blk = 13; continue; }
            blk = 11; continue;
        }
        11 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ed8b6); };
            c.w64(1, t18);
            blk = 12; continue;
        }
        12 => {
            let r19 = c.sub(c.r64(3), 0x4u64, 64);
            c.w64(3, r19);
            if !c.st32(c.r64(3).wrapping_add(c.r64(8)), c.r32(1)) { return RegfileExit::fault(0x70008ed8c1); }
            c.cmp(c.r64(3), 0x7u64, 64);
            if c.cc_ne() { blk = 12; continue; }
            blk = 13; continue;
        }
        13 => {
            c.w64(0, c.r64(6));
            return c.ret(0x0, 0x70008ed8ce);
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ed8cf); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ed8d3); }
            let r20 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r20);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008ed8d9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ed8dd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008ed8e1); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008ed8e5); }
            return c.call(0x70007febf4, 0x70008ed8ee, 0x70008ed8e9);
        }
        15 => {
            c.w64(7, c.r64(0));
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed8f1); };
            c.w64(1, t21);
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ed8f5); };
            c.w64(0, t22);
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ed8f9); };
            c.w64(3, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ed8fd); };
            c.w64(6, t24);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ed904); }
            blk = 5; continue;
        }
        16 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ed90a); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ed90e); }
            let r25 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r25);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(1)) { return RegfileExit::fault(0x70008ed919); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ed91d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008ed921); }
            return c.call(0x70007febf4, 0x70008ed92a, 0x70008ed925);
        }
        17 => {
            c.w64(6, c.r64(0));
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed92d); };
            c.w64(1, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ed931); };
            c.w64(0, t27);
            let Some(t28) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ed935); };
            c.w64(3, t28);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ed93c); }
            blk = 8; continue;
        }
        18 => {
            return RegfileExit::jump(0x70008ed944);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed954(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ed954); };
            c.w64(1, t0);
            let Some(t1) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ed95b); };
            c.w64(6, t1);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ed962); };
            c.w64(8, t2);
            let Some(t3) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ed969); };
            c.cmp(t3, c.r64(7), 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008ed96e); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xe38)) else { return RegfileExit::fault(0x70008ed975); };
            c.w64(6, t4);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008ed97c); }
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(0)) { return RegfileExit::fault(0x70008ed97f); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008ed982); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008ed988);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ed989); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ed98d); }
            let r5 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r5);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ed998); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ed99c); }
            return c.call(0x70007febb4, 0x70008ed9a5, 0x70008ed9a0);
        }
        4 => {
            c.w64(1, c.r64(0));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed9a8); };
            c.w64(3, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ed9ac); };
            c.w64(0, t7);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ed9b3); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008ed9b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ed9d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ed9d7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ed9db); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w32(2, c.r64(0).wrapping_add(c.r64(0)));
            let r3 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008ed9eb); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008ed9eb); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ed9ef); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2bf0)) else { return RegfileExit::fault(0x70008ed9f4); };
            c.w64(3, t5);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x0u64) { return RegfileExit::fault(0x70008eda00); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x70008eda09); }
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008eda14);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eda34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eda37); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008eda3b); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            c.w32(2, c.r64(0).wrapping_add(c.r64(0)));
            let r3 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r3);
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008eda4b); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008eda4b); }
            if !c.st64(c.r64(4).wrapping_add(0x10), c.r64(3)) { return RegfileExit::fault(0x70008eda4f); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2bf0)) else { return RegfileExit::fault(0x70008eda54); };
            c.w64(3, t5);
            c.w32(0, 0x3u64);
            if !c.st64(c.r64(4).wrapping_add(0x8), 0x2u64) { return RegfileExit::fault(0x70008eda60); }
            if !c.st64(c.r64(4).wrapping_add(0x18), c.r64(2)) { return RegfileExit::fault(0x70008eda69); }
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008eda74);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eda94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eda94); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eda9a); };
            c.w32(3, t2);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008edaa0); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008edaa7); };
            c.w64(6, t4);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(7, c.r64(6).wrapping_add(c.r64(3)));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008edab1); };
            c.w64(8, t5);
            let Some(t6) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008edab8); };
            c.cmp(t6, c.r64(7), 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(3), 0x20000u64, 64);
            if c.cc_ae() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008edac6); }
            c.w64(3, c.r64(6).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008edacd); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008edad0); };
            c.w64(1, t7);
            if !c.st32(c.r64(3).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008edad7); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008edadd);
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edade); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edae2); }
            let r8 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r8);
            blk = 6; continue;
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edaea); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edaee); }
            let r9 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r9);
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008edaf4); }
            c.w64(2, c.r64(3));
            return c.call(0x70007febb4, 0x70008edb00, 0x70008edafb);
        }
        7 => {
            c.w64(3, c.r64(0));
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008edb03); };
            c.w64(0, t10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008edb0a); }
            blk = 3; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008edb10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008edb14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(3));
            let r0 = c.imul2(c.r64(1), c.r64(6), 64);
            c.w64(1, r0);
            let r1 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r1);
            let r2 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r2);
            c.logic(r2, 64);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008edb24); };
            c.w64(6, t3);
            let Some(t4) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008edb2b); };
            c.w64(7, t4);
            c.w64(2, c.r64(1).wrapping_add(0xc));
            c.w64(1, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008edb36); };
            c.w64(8, t5);
            let Some(t6) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008edb3d); };
            c.cmp(t6, c.r64(1), 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(6), c.r64(1)) { return RegfileExit::fault(0x70008edb4b); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008edb52); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008edb55); };
            c.w64(6, t7);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008edb5c); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008edb5f); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008edb65);
        }
        4 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edb66); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edb6a); }
            let r8 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r8);
            blk = 6; continue;
        }
        5 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edb72); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edb76); }
            let r9 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r9);
            blk = 6; continue;
        }
        6 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008edb7c); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008edb80); }
            return c.call(0x70007febb4, 0x70008edb89, 0x70008edb84);
        }
        7 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008edb8c); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008edb90); };
            c.w64(0, t11);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008edb97); }
            blk = 3; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008edb9c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008edbb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(3));
            let r0 = c.imul2(c.r64(1), c.r64(6), 64);
            c.w64(1, r0);
            let r1 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r1);
            let r2 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r2);
            c.logic(r2, 64);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008edbc4); };
            c.w64(6, t3);
            let Some(t4) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008edbcb); };
            c.w64(7, t4);
            c.w64(2, c.r64(1).wrapping_add(0xc));
            c.w64(1, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008edbd6); };
            c.w64(8, t5);
            let Some(t6) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008edbdd); };
            c.cmp(t6, c.r64(1), 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(6), c.r64(1)) { return RegfileExit::fault(0x70008edbeb); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)).wrapping_sub(0x1));
            c.w64(7, c.r64(1).wrapping_add(0xb));
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(8, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            if !c.st32(c.r64(7), c.r32(8)) { return RegfileExit::fault(0x70008edc03); }
            let r7 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r7);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_ne() { blk = 5; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008edc0f); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008edc12); };
            c.w64(6, t8);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008edc19); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008edc1c); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008edc22);
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edc23); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edc27); }
            let r9 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r9);
            blk = 9; continue;
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edc2f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edc33); }
            let r10 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r10);
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008edc39); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008edc3d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008edc41); }
            return c.call(0x70007febb4, 0x70008edc4a, 0x70008edc45);
        }
        10 => {
            c.w64(1, c.r64(0));
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008edc4d); };
            c.w64(3, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008edc51); };
            c.w64(0, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008edc55); };
            c.w64(2, t13);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008edc5c); }
            blk = 3; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008edc60);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008edc74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(1, c.r32(1));
            c.w32(6, c.r32(3));
            let r0 = c.imul2(c.r64(1), c.r64(6), 64);
            c.w64(1, r0);
            let r1 = c.add(c.r64(1), 0x3u64, 64);
            c.w64(1, r1);
            let r2 = c.r64(1) & 0xfffffffffffffffcu64;
            c.w64(1, r2);
            c.logic(r2, 64);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008edc84); };
            c.w64(6, t3);
            let Some(t4) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008edc8b); };
            c.w64(7, t4);
            c.w64(2, c.r64(1).wrapping_add(0xc));
            c.w64(1, c.r64(2).wrapping_add(c.r64(7)));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008edc96); };
            c.w64(8, t5);
            let Some(t6) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008edc9d); };
            c.cmp(t6, c.r64(1), 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r64(2), 0x20000u64, 64);
            if c.cc_ae() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(6), c.r64(1)) { return RegfileExit::fault(0x70008edcab); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            c.w64(6, c.r64(1).wrapping_add(c.r64(2)).wrapping_sub(0x1));
            c.w64(7, c.r64(1).wrapping_add(0xb));
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008edcc0); };
            c.w64(8, t7);
            blk = 5; continue;
        }
        5 => {
            if !c.st32(c.r64(7), c.r32(8)) { return RegfileExit::fault(0x70008edcc7); }
            let r8 = c.add(c.r64(7), 0x4u64, 64);
            c.w64(7, r8);
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_ne() { blk = 5; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(0)) { return RegfileExit::fault(0x70008edcd3); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008edcd6); };
            c.w64(6, t9);
            if !c.st32(c.r64(1).wrapping_add(0x3), c.r32(6)) { return RegfileExit::fault(0x70008edcdd); }
            if !c.st32(c.r64(1).wrapping_add(0x7), c.r32(3)) { return RegfileExit::fault(0x70008edce0); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008edce6);
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edce7); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edceb); }
            let r10 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r10);
            blk = 9; continue;
        }
        8 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edcf3); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edcf7); }
            let r11 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r11);
            blk = 9; continue;
        }
        9 => {
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008edcfd); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008edd01); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008edd05); }
            return c.call(0x70007febb4, 0x70008edd0e, 0x70008edd09);
        }
        10 => {
            c.w64(1, c.r64(0));
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008edd11); };
            c.w64(3, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008edd15); };
            c.w64(0, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008edd19); };
            c.w64(2, t14);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008edd20); }
            blk = 3; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008edd24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008edd34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008edd34); };
            c.cmp(t0, c.r32(2), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008edd41); };
            c.cmp(t1, c.r32(7), 32);
            if c.cc_e() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(8, c.r64(0).wrapping_add(c.r64(1)));
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008edd4e); };
            c.cmp(t2, c.r32(8), 32);
            if c.cc_b() { blk = 7; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(8), c.r32(0), 32);
            if c.cc_b() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w32(8, c.r64(3).wrapping_add(c.r64(1)));
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008edd5d); };
            c.cmp(t3, c.r32(8), 32);
            if c.cc_b() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(8), c.r32(3), 32);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(8, c.r64(5));
            let Some(t4) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008edd6b); };
            c.w64(8, t4);
            let Some(t5) = c.ld32(c.r64(8).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008edd6f); };
            c.w32(6, t5);
            let r6 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r6);
            let r7 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r7);
            c.w32(8, c.r64(3).wrapping_add(c.r64(3)));
            c.w32(9, c.r64(0).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008edd83); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008edd85); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008edd86); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008edd88); }
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008edd89); };
            if !c.push64(t8) { return RegfileExit::fault(0x70008edd89); }
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2bf8)) else { return RegfileExit::fault(0x70008edd8d); };
            c.w64(3, t9);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x70008edd99); }
            return RegfileExit::jump(0x700086ed74);
        }
        7 => {
            return RegfileExit::jump(0x70008ee6f4);
        }
        8 => {
            return RegfileExit::jump(0x70008ee6b4);
        }
        9 => {
            return RegfileExit::jump(0x70008ee6b4);
        }
        10 => {
            return RegfileExit::jump(0x70008eddb4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eddd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eddd7); };
            c.w64(8, t0);
            let Some(t1) = c.ld32(c.r64(8).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008edddb); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let r3 = c.shl(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r3);
            c.w32(8, c.r64(3).wrapping_add(c.r64(3)));
            c.w32(9, c.r64(0).wrapping_add(c.r64(0)));
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008eddef); }
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008eddf1); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008eddf2); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008eddf4); }
            let Some(t4) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008eddf5); };
            if !c.push64(t4) { return RegfileExit::fault(0x70008eddf5); }
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2bf8)) else { return RegfileExit::fault(0x70008eddf9); };
            c.w64(3, t5);
            c.w32(0, 0x5u64);
            if !c.st64(c.r64(4).wrapping_add(0x28), c.r64(2)) { return RegfileExit::fault(0x70008ede05); }
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ede10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ede14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(6).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ede14); };
            c.w32(1, t0);
            let r1 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ede1a); };
            c.w32(1, t2);
            let r3 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r3);
            let Some(t4) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ede20); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x8cu64, 64);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008ede32);
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(1).wrapping_add(0x237)) else { return RegfileExit::fault(0x70008ede33); };
            c.w32(1, t5);
            let r6 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r6);
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008ede3c); };
            c.w32(1, t7);
            let r8 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r8);
            let Some(t9) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ede42); };
            c.w32(7, t9);
            let Some(t10) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008ede45); };
            c.cmp(t10, c.r32(7), 32);
            if c.cc_ne() { blk = 10; continue; }
            blk = 3; continue;
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ede52); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ede56); }
            let r11 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r11);
            let Some(t12) = c.ld8(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ede5c); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ede60); };
            c.w64(8, t13);
            let Some(t14) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ede67); };
            c.w64(9, t14);
            c.w64(2, c.r64(7).wrapping_mul(4));
            c.w64(7, c.r64(2).wrapping_add(c.r64(9)));
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ede76); };
            c.w64(11, t15);
            let Some(t16) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008ede7d); };
            c.cmp(t16, c.r64(7), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st64(c.r64(8), c.r64(7)) { return RegfileExit::fault(0x70008ede82); }
            c.w64(7, c.r64(9).wrapping_add(0x1));
            blk = 5; continue;
        }
        5 => {
            c.w64(8, c.r64(2).wrapping_sub(0x1));
            if !c.st32(c.r64(7).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ede8d); }
            let Some(t17) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008ede90); };
            c.w64(1, t17);
            if !c.st32(c.r64(7).wrapping_add(0x3), c.r32(1)) { return RegfileExit::fault(0x70008ede97); }
            if !c.st32(c.r64(7).wrapping_add(0x7), c.r32(1)) { return RegfileExit::fault(0x70008ede9a); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(7)) { return RegfileExit::fault(0x70008ede9d); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_e() { blk = 8; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t18) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008edea7); };
            c.w64(1, t18);
            blk = 7; continue;
        }
        7 => {
            let r19 = c.sub(c.r64(8), 0x4u64, 64);
            c.w64(8, r19);
            if !c.st32(c.r64(8).wrapping_add(c.r64(7)), c.r32(1)) { return RegfileExit::fault(0x70008edeb2); }
            c.cmp(c.r64(8), 0xbu64, 64);
            if c.cc_ne() { blk = 7; continue; }
            blk = 8; continue;
        }
        8 => {
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008edebc); }
            let Some(t20) = c.ld64(c.r64(13).wrapping_add(0xc90)) else { return RegfileExit::fault(0x70008edebd); };
            c.w64(1, t20);
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008edec4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008edec5); }
            let Some(t21) = c.ld64(c.r64(13).wrapping_add(0x2340)) else { return RegfileExit::fault(0x70008edec6); };
            c.w64(3, t21);
            c.w32(0, 0x3u64);
            return c.call(0x700086ed74, 0x70008eded7, 0x70008eded2);
        }
        9 => {
            let Some(t22) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eded7); };
            c.w64(0, t22);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008edede); }
            return c.ret(0x0, 0x70008ededf);
        }
        10 => {
            return RegfileExit::fault(0x70008edee0);
        }
        11 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008edee1); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008edee5); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(1)) { return RegfileExit::fault(0x70008edee9); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008edeed); }
            return c.call(0x70007febf4, 0x70008edef6, 0x70008edef1);
        }
        12 => {
            c.w64(7, c.r64(0));
            let Some(t23) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008edef9); };
            c.w64(0, t23);
            let Some(t24) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008edefd); };
            c.w64(6, t24);
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008edf01); };
            c.w64(1, t25);
            let Some(t26) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008edf05); };
            c.w64(2, t26);
            blk = 5; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008edf10);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008edf14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(3).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008edf14); };
            c.w32(2, t0);
            let r1 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r1);
            let Some(t2) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008edf1a); };
            c.w32(2, t2);
            let r3 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r3);
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008edf20); };
            c.w32(1, t4);
            let Some(t5) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008edf23); };
            c.w32(2, t5);
            c.cmp(c.r32(1), c.r32(2), 32);
            if c.cc_le() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(2), c.r32(1), 32);
            if c.cc_ae() { blk = 6; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t6) = c.ld32(c.r64(0).wrapping_add(c.r64(2).wrapping_mul(2)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008edf31); };
            c.cmp(t6, c.r32(3), 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008edf39);
        }
        4 => {
            c.w32(0, 0x1u64);
            return c.ret(0x0, 0x70008edf3f);
        }
        5 => {
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008edf42);
        }
        6 => {
            return RegfileExit::fault(0x70008edf43);
        }
        7 => {
            return RegfileExit::jump(0x70008edf48);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008edf54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(0));
            let r0 = c.add(c.r32(3), c.r32(0), 32);
            c.w32(3, r0);
            if c.cc_o() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, (c.r32(3) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008edf5e);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edf5f); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edf63); }
            let r1 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r1);
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008edf69); };
            c.w64(3, t2);
            let Some(t3) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008edf70); };
            c.w64(1, t3);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008edf77); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008edf7b); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008edf82); };
            c.cmp(t5, c.r64(6), 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008edf91, 0x70008edf8c);
        }
        4 => {
            c.w64(3, c.r64(0));
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008edf94); };
            c.w64(0, t6);
            blk = 6; continue;
        }
        5 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008edf9a); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(0) as u32 as i32 as i64));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008edfa9); };
            c.w64(1, t7);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008edfb0); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008edfb3); }
            c.w64(0, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008edfbe); }
            return c.ret(0x0, 0x70008edfbf);
        }
        7 => {
            return RegfileExit::jump(0x70008edfc4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008edfd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r32(0), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(3, c.r32(0));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008edfe0);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008edfe1); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008edfe5); }
            let r0 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008edfeb); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008edff2); };
            c.w64(1, t2);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008edff9); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008edffd); };
            c.w64(7, t3);
            let Some(t4) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ee004); };
            c.cmp(t4, c.r64(6), 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ee013, 0x70008ee00e);
        }
        4 => {
            c.w64(3, c.r64(0));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee016); };
            c.w64(0, t5);
            blk = 6; continue;
        }
        5 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008ee01c); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            c.w32(10, c.r32(0));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ee02f); };
            c.w64(1, t6);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ee036); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ee039); }
            c.w64(0, c.r64(3));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee044); }
            return c.ret(0x0, 0x70008ee045);
        }
        7 => {
            return RegfileExit::jump(0x70008ee048);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee054(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.cmp(c.r64(0), 0x3fffffffu64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ee05c); };
            c.w64(3, t0);
            let Some(t1) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ee063); };
            c.w64(1, t1);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ee06a); };
            c.w64(7, t2);
            let Some(t3) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ee071); };
            c.cmp(t3, c.r64(6), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008ee076); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(0) as i64));
            c.logic(c.r64(0) & c.r64(0), 64);
            if c.cc_ns() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            c.w64(10, c.r64(0));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let r4 = c.r64(10) | 0x1u64;
            c.w64(10, r4);
            c.logic(r4, 64);
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 7; continue;
        }
        7 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ee0a4); };
            c.w64(1, t5);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ee0ab); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ee0ae); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008ee0b6);
        }
        8 => {
            let r6 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r6);
            return c.ret(0x0, 0x70008ee0b9);
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee0ba); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee0be); }
            let r7 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r7);
            c.w32(2, 0xcu64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ee0c9); }
            return c.call(0x70007febf4, 0x70008ee0d2, 0x70008ee0cd);
        }
        10 => {
            c.w64(3, c.r64(0));
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee0d5); };
            c.w64(0, t8);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee0dc); }
            blk = 3; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008ee0e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee0f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee0f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee0f8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ee0fe); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee102); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ee106); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008ee10f); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008ee113); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008ee117); }
            return c.call(0x70008ee054, 0x70008ee120, 0x70008ee11b);
        }
        1 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ee120); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ee124); };
            c.w64(0, t5);
            return c.call(0x70008edfd4, 0x70008ee12d, 0x70008ee128);
        }
        2 => {
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee12d); };
            if !c.push64(t6) { return RegfileExit::fault(0x70008ee12d); }
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ee130); };
            if !c.push64(t7) { return RegfileExit::fault(0x70008ee130); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ee133); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2b60)) else { return RegfileExit::fault(0x70008ee134); };
            c.w64(3, t8);
            c.w32(0, 0x3u64);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ee140); };
            c.w64(6, t9);
            return c.call(0x700086ed74, 0x70008ee149, 0x70008ee144);
        }
        3 => {
            let r10 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r10);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee14e); }
            return c.ret(0x0, 0x70008ee14f);
        }
        4 => {
            return RegfileExit::jump(0x70008ee154);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee174(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee174); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee178); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ee17e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee182); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ee186); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008ee18f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008ee193); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008ee197); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008ee19b); }
            return c.call(0x70008ee054, 0x70008ee1a4, 0x70008ee19f);
        }
        1 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ee1a4); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ee1a8); };
            c.w64(0, t5);
            return c.call(0x70008edf54, 0x70008ee1b1, 0x70008ee1ac);
        }
        2 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008ee1b1); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ee1b5); };
            c.w64(0, t6);
            return c.call(0x70007ff9b4, 0x70008ee1be, 0x70008ee1b9);
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee1be); };
            if !c.push64(t7) { return RegfileExit::fault(0x70008ee1be); }
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ee1c1); };
            if !c.push64(t8) { return RegfileExit::fault(0x70008ee1c1); }
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ee1c4); };
            if !c.push64(t9) { return RegfileExit::fault(0x70008ee1c4); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ee1c7); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b50)) else { return RegfileExit::fault(0x70008ee1c8); };
            c.w64(3, t10);
            c.w32(0, 0x4u64);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ee1d4); };
            c.w64(6, t11);
            return c.call(0x700086ed74, 0x70008ee1dd, 0x70008ee1d8);
        }
        4 => {
            let r12 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee1e2); }
            return c.ret(0x0, 0x70008ee1e3);
        }
        5 => {
            return RegfileExit::jump(0x70008ee1e8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee1f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee1f4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee1f8); }
            let r0 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008ee1fe); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee202); };
            c.w64(2, t2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ee206); };
            c.w32(6, t3);
            let r4 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r4);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(1)) { return RegfileExit::fault(0x70008ee20f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008ee213); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(2)) { return RegfileExit::fault(0x70008ee217); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008ee21b); }
            return c.call(0x70008ee054, 0x70008ee224, 0x70008ee21f);
        }
        1 => {
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008ee224); }
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ee228); };
            c.w64(0, t5);
            return c.call(0x70007ff9b4, 0x70008ee231, 0x70008ee22c);
        }
        2 => {
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(0)) { return RegfileExit::fault(0x70008ee231); }
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ee235); };
            c.w64(0, t6);
            return c.call(0x70007ff9b4, 0x70008ee23e, 0x70008ee239);
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee23e); };
            if !c.push64(t7) { return RegfileExit::fault(0x70008ee23e); }
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ee241); };
            if !c.push64(t8) { return RegfileExit::fault(0x70008ee241); }
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ee244); };
            if !c.push64(t9) { return RegfileExit::fault(0x70008ee244); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008ee247); }
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2b58)) else { return RegfileExit::fault(0x70008ee248); };
            c.w64(3, t10);
            c.w32(0, 0x4u64);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ee254); };
            c.w64(6, t11);
            return c.call(0x700086ed74, 0x70008ee25d, 0x70008ee258);
        }
        4 => {
            let r12 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r12);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee262); }
            return c.ret(0x0, 0x70008ee263);
        }
        5 => {
            return RegfileExit::jump(0x70008ee268);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee274(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ee27c); };
            c.w32(7, t0);
            let r1 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r1);
            let Some(t2) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ee282); };
            c.w32(8, t2);
            c.w32(9, c.r64(8).wrapping_sub(0xba));
            c.cmp(c.r32(9), 0x78eu64, 32);
            if c.cc_a() { blk = 43; continue; }
            blk = 2; continue;
        }
        2 => {
            c.w32(8, c.r16(8));
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 29; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ee2ac); };
            c.w32(8, t3);
            c.logic(c.r32(8) & 0x200000u64, 32);
            if c.cc_ne() { blk = 22; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t4) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ee2bd); };
            c.w32(7, t4);
            let r5 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r5);
            c.logic(c.r32(8) & 0xffc00u64, 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 5; continue;
        }
        5 => {
            let r6 = c.shr(c.r32(8), 0xau64, 32).unwrap_or(c.r32(8));
            c.w32(8, r6);
            let r7 = c.r32(8) & 0x3ffu64;
            c.w32(8, r7);
            c.logic(r7, 32);
            c.cmp(c.r32(8), 0x20u64, 32);
            if c.cc_be() { blk = 18; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008ee2e5); };
            c.w32(9, (t8 as u16 as i16 as i64 as u64));
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee2ea); };
            c.w32(11, t9);
            let r10 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r10);
            let r11 = c.sub(c.r32(9), 0x1u64, 32);
            c.w32(9, r11);
            c.w32(12, 0);
            c.logic(0, 32);
            c.w64(15, c.r64(9));
            blk = 8; continue;
        }
        7 => {
            c.w64(15, c.r64(2));
            blk = 8; continue;
        }
        8 => {
            c.w64(2, c.r64(15));
            let r12 = c.sub(c.r32(2), c.r32(12), 32);
            c.w32(2, r12);
            let r13 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r13);
            let r14 = c.add(c.r32(2), c.r32(12), 32);
            c.w32(2, r14);
            c.w32(1, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t15) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ee312); };
            c.w32(1, t15);
            let r16 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r16);
            let r17 = c.shr(c.r32(1), 0x9u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r17);
            let r18 = c.r32(1) & 0x3ffu64;
            c.w32(1, r18);
            c.logic(r18, 32);
            c.w32(1, c.r64(1).wrapping_add(c.r64(1).wrapping_mul(2)));
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ee327); };
            c.w32(1, t19);
            let r20 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r20);
            let Some(t21) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee32e); };
            c.w32(1, t21);
            let r22 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r22);
            c.cmp(c.r32(11), c.r32(1), 32);
            if c.cc_be() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w32(12, c.r64(2).wrapping_add(0x1));
            c.w64(2, c.r64(15));
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(12), c.r32(2), 32);
            if c.cc_ne() { blk = 7; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 13; continue;
        }
        12 => {
            let r23 = c.add(c.r32(12), 0x1u64, 32);
            c.w32(12, r23);
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(9), c.r32(12), 32);
            if c.cc_l() { blk = 43; continue; }
            blk = 14; continue;
        }
        14 => {
            c.w32(15, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(15, (c.r32(15) as u32 as i32 as i64 as u64));
            let Some(t24) = c.ld32(c.r64(7).wrapping_add(c.r64(15).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ee35b); };
            c.w32(15, t24);
            let r25 = c.sar(c.r32(15), 1u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r25);
            let r26 = c.shr(c.r32(15), 0x9u64, 32).unwrap_or(c.r32(15));
            c.w32(15, r26);
            let r27 = c.r32(15) & 0x3ffu64;
            c.w32(15, r27);
            c.logic(r27, 32);
            c.w32(2, c.r64(15).wrapping_add(c.r64(15).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ee375); };
            c.w32(2, t28);
            let r29 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r29);
            let Some(t30) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee37c); };
            c.w32(1, t30);
            let r31 = c.shr(c.r32(1), 0x2u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r31);
            c.cmp(c.r32(11), c.r32(1), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 15; continue;
        }
        15 => {
            c.cmp(c.r32(2), c.r32(3), 32);
            if c.cc_ne() { blk = 12; continue; }
            blk = 16; continue;
        }
        16 => {
            c.cmp(c.r32(8), c.r32(15), 32);
            if c.cc_be() { blk = 43; continue; }
            blk = 17; continue;
        }
        17 => {
            blk = 40; continue;
        }
        18 => {
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            let r32 = c.imul2(c.r64(8), 0x3u64, 64);
            c.w64(8, r32);
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 43; continue; }
            blk = 19; continue;
        }
        19 => {
            let r33 = c.sub(c.r64(8), 0x3u64, 64);
            c.w64(8, r33);
            let Some(t34) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ee3b2); };
            c.cmp(t34, c.r32(3), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 20; continue;
        }
        20 => {
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_ne() { blk = 19; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 43; continue;
        }
        22 => {
            let Some(t35) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee3c8); };
            c.w32(7, t35);
            let r36 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r36);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t37) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008ee3d4); };
            c.w64(7, t37);
            blk = 24; continue;
        }
        24 => {
            let Some(t38) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ee3db); };
            c.w32(8, t38);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r39 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r39);
            let Some(t40) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee3e9); };
            c.w32(9, t40);
            let r41 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r41);
            let r42 = c.r64(9) & c.r64(8);
            c.w64(9, r42);
            c.logic(r42, 64);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 26; continue;
        }
        25 => {
            let r43 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r43);
            let r44 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r44);
            c.w64(12, c.r64(8));
            let r45 = c.r64(12) & c.r64(9);
            c.w64(12, r45);
            c.logic(r45, 64);
            c.w64(9, c.r64(12));
            blk = 26; continue;
        }
        26 => {
            c.w64(12, c.r64(9).wrapping_add(c.r64(9).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t46) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ee40e); };
            c.w32(12, t46);
            let Some(t47) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ee413); };
            c.cmp(t47, c.r32(12), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 27; continue;
        }
        27 => {
            c.cmp(c.r32(12), c.r32(3), 32);
            if c.cc_e() { blk = 40; continue; }
            blk = 28; continue;
        }
        28 => {
            blk = 25; continue;
        }
        29 => {
            c.cmp(c.r32(8), 0xbdu64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 30; continue;
        }
        30 => {
            let Some(t48) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ee438); };
            c.logic(t48 & 0x24u64, 8);
            if c.cc_ne() { blk = 43; continue; }
            blk = 31; continue;
        }
        31 => {
            let Some(t49) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee442); };
            c.w32(7, t49);
            let r50 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r50);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_ne() { blk = 33; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t51) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008ee44e); };
            c.w64(7, t51);
            blk = 33; continue;
        }
        33 => {
            let Some(t52) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ee455); };
            c.w32(8, t52);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            let r53 = c.sub(c.r64(8), 0x1u64, 64);
            c.w64(8, r53);
            let Some(t54) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee463); };
            c.w32(9, t54);
            let r55 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r55);
            let r56 = c.r64(9) & c.r64(8);
            c.w64(9, r56);
            c.logic(r56, 64);
            c.w32(11, 0);
            c.logic(0, 32);
            blk = 35; continue;
        }
        34 => {
            let r57 = c.add(c.r64(11), 0x1u64, 64);
            c.w64(11, r57);
            let r58 = c.add(c.r64(9), c.r64(11), 64);
            c.w64(9, r58);
            c.w64(12, c.r64(8));
            let r59 = c.r64(12) & c.r64(9);
            c.w64(12, r59);
            c.logic(r59, 64);
            c.w64(9, c.r64(12));
            blk = 35; continue;
        }
        35 => {
            c.w64(12, c.r64(9).wrapping_add(0x5));
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ee487); };
            c.w32(15, t60);
            let r61 = c.add(c.r64(15), c.r64(14), 64);
            c.w64(15, r61);
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ee48f); };
            c.cmp(t62, c.r32(15), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t63) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ee498); };
            c.cmp(t63, c.r32(15), 32);
            if c.cc_e() { blk = 34; continue; }
            blk = 37; continue;
        }
        37 => {
            let Some(t64) = c.ld32(c.r64(15).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee4a1); };
            c.cmp(t64, c.r32(3), 32);
            if c.cc_ne() { blk = 34; continue; }
            blk = 38; continue;
        }
        38 => {
            let Some(t65) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ee4a7); };
            c.w32(7, t65);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(12), c.r64(7), 64);
            if c.cc_ae() { blk = 42; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t66) = c.ld32(c.r64(15).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ee4b5); };
            c.w32(7, t66);
            let Some(t67) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ee4b9); };
            c.cmp(t67, c.r32(7), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 40; continue;
        }
        40 => {
            c.w32(2, 0x2u64);
            c.w64(1, c.r64(0));
            return RegfileExit::jump(0x70008712b4);
        }
        41 => {
            blk = 43; continue;
        }
        42 => {
            return RegfileExit::fault(0x70008ee4d1);
        }
        43 => {
            let Some(t68) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ee4d2); };
            c.w64(0, t68);
            return c.ret(0x0, 0x70008ee4d9);
        }
        44 => {
            return RegfileExit::jump(0x70008ee4dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee4f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee4f7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0xc7)) else { return RegfileExit::fault(0x70008ee4fb); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r2);
            let Some(t3) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008ee504); };
            if !c.push64(t3) { return RegfileExit::fault(0x70008ee504); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2b40)) else { return RegfileExit::fault(0x70008ee507); };
            c.w64(3, t4);
            if !c.st64(c.r64(4).wrapping_add(0x8), c.r64(0)) { return RegfileExit::fault(0x70008ee50e); }
            c.w32(0, 0x1u64);
            return RegfileExit::jump(0x700086ed74);
        }
        1 => {
            return RegfileExit::jump(0x70008ee520);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2d0u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee540);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2d2u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee560);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2d4u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee580);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2d6u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee5a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee5b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2d8u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee5c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee5d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2dau64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee5e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee5f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2dcu64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee600);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2e0u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee620);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee634(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2e6u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee640);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2e8u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee660);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2deu64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee680);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2eau64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee6a0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee6b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2ecu64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee6c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee6d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2eeu64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee6e0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee6f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2f0u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee700);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee714(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w32(0, 0x2f2u64);
            return RegfileExit::jump(0x70008ee4f4);
        }
        1 => {
            return RegfileExit::jump(0x70008ee720);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee734); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee738); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            c.cmp(c.r64(3), 0x3fffffffu64, 64);
            if c.cc_a() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(1, c.r64(3).wrapping_add(c.r64(3)));
            blk = 2; continue;
        }
        2 => {
            c.w64(3, c.r64(1));
            return c.call(0x7000870914, 0x70008ee752, 0x70008ee74d);
        }
        3 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee755); }
            return c.ret(0x0, 0x70008ee756);
        }
        4 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ee757); };
            c.w64(1, t1);
            let Some(t2) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ee75e); };
            c.w64(7, t2);
            c.w64(8, c.r64(7).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008ee765); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(6)) { return RegfileExit::fault(0x70008ee769); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(3)) { return RegfileExit::fault(0x70008ee76d); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ee771); };
            c.w64(9, t3);
            let Some(t4) = c.ld64(c.r64(9)) else { return RegfileExit::fault(0x70008ee778); };
            c.cmp(t4, c.r64(8), 64);
            if c.cc_a() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ee787, 0x70008ee782);
        }
        6 => {
            c.w64(1, c.r64(0));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee78a); };
            c.w64(0, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ee78e); };
            c.w64(6, t6);
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ee792); };
            c.w64(3, t7);
            blk = 8; continue;
        }
        7 => {
            if !c.st64(c.r64(1), c.r64(8)) { return RegfileExit::fault(0x70008ee798); }
            c.w64(1, c.r64(7).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(3) as i64));
            c.logic(c.r64(3) & c.r64(3), 64);
            if c.cc_ns() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(10, c.r64(3));
            if let Some(sr) = c.shr(c.r64(10), 1u64, 64) { c.w64(10, sr); }
            if c.cc_ae() { blk = 11; continue; }
            blk = 10; continue;
        }
        10 => {
            let r8 = c.r64(10) | 0x1u64;
            c.w64(10, r8);
            c.logic(r8, 64);
            blk = 11; continue;
        }
        11 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            c.arith_sd(0, 0, c.xmm_u64(0));
            blk = 12; continue;
        }
        12 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ee7c6); };
            c.w64(7, t9);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(7)) { return RegfileExit::fault(0x70008ee7cd); }
            if !c.movsd_mr(c.r64(1).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ee7d0); }
            blk = 2; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008ee7dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee7f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ee7f4); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ee7f8); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x70008ee7fe); };
            c.w32(3, t3);
            c.w64(1, c.r64(3));
            let r4 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r4);
            if c.cc_o() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008ee80b);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee80c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee810); }
            let r5 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ee816); };
            c.w64(1, t6);
            let Some(t7) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ee81d); };
            c.w64(6, t7);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ee824); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ee828); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ee82f); };
            c.cmp(t9, c.r64(7), 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ee83e, 0x70008ee839);
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ee83e); };
            c.w64(3, t10);
            blk = 6; continue;
        }
        5 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008ee844); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ee856); };
            c.w64(3, t11);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008ee85d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ee860); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee868); }
            return c.ret(0x0, 0x70008ee869);
        }
        7 => {
            return RegfileExit::jump(0x70008ee86c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee874(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ee874); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ee878); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            if !c.movss_rm(0, c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4))) { return RegfileExit::fault(0x70008ee87e); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ee883); };
            c.w64(3, t3);
            let Some(t4) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ee88a); };
            c.w64(1, t4);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ee891); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ee898); };
            c.cmp(t6, c.r64(6), 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008ee89d); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ee8a8); };
            c.w64(1, t7);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ee8af); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ee8b2); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008ee8ba);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee8bb); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee8bf); }
            let r8 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r8);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008ee8ca); }
            return c.call(0x70007febf4, 0x70008ee8d4, 0x70008ee8cf);
        }
        4 => {
            c.w64(3, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ee8d7); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee8df); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008ee8e4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee8f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ee8f4); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ee8f8); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8))) { return RegfileExit::fault(0x70008ee8fe); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ee903); };
            c.w64(3, t3);
            let Some(t4) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ee90a); };
            c.w64(1, t4);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ee911); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008ee918); };
            c.cmp(t6, c.r64(6), 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008ee91d); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ee924); };
            c.w64(1, t7);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008ee92b); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ee92e); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008ee936);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ee937); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ee93b); }
            let r8 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r8);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008ee946); }
            return c.call(0x70007febf4, 0x70008ee950, 0x70008ee94b);
        }
        4 => {
            c.w64(3, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008ee953); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ee95b); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008ee960);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ee974); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ee978); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld8(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008ee97e); };
            c.w32(3, t3);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008ee985);
        }
        1 => {
            return RegfileExit::jump(0x70008ee988);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ee994(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ee994); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ee998); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008ee99e); };
            c.w64(3, t3);
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ee9a8); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ee9af); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x10));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ee9b6); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ee9bd); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008ee9c2); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008ee9c9); };
            c.w64(6, t8);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008ee9d0); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008ee9d3); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(3)) { return RegfileExit::fault(0x70008ee9da); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008ee9e1);
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ee9e2); };
            c.w64(3, t9);
            let Some(t10) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008ee9e9); };
            c.w64(1, t10);
            c.w64(7, c.r64(1).wrapping_add(0x8));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ee9f0); };
            c.w64(6, t11);
            let Some(t12) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008ee9f7); };
            c.cmp(t12, c.r64(7), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008ee9fc); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008eea03); };
            c.w64(1, t13);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008eea0a); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008eea0d); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008eea17);
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eea18); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eea1c); }
            let r14 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r14);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008eea27); }
            return c.call(0x70007febb4, 0x70008eea30, 0x70008eea2b);
        }
        8 => {
            c.w64(1, c.r64(0));
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eea33); };
            c.w64(3, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eea3a); }
            blk = 3; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eea3d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eea41); }
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008eea51, 0x70008eea4c);
        }
        10 => {
            c.w64(3, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eea57); }
            blk = 6; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008eea5c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eea74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008eea74); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008eea78); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008eea7e); };
            c.w64(3, t3);
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008eea88); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008eea8f); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x10));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008eea96); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008eea9d); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008eeaa2); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008eeaa9); };
            c.w64(6, t8);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008eeab0); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008eeab9); }
            let r9 = c.neg(c.r64(3), 64);
            c.w64(3, r9);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(3)) { return RegfileExit::fault(0x70008eeac3); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008eeaca);
        }
        5 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008eeacb); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(3)) { return RegfileExit::fault(0x70008eead2); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008eead9);
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008eeada); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008eeae1); };
            c.w64(1, t11);
            c.w64(7, c.r64(1).wrapping_add(0x8));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008eeae8); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008eeaef); };
            c.cmp(t13, c.r64(7), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008eeaf4); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008eeafb); };
            c.w64(1, t14);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008eeb02); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008eeb05); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008eeb0f);
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eeb10); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eeb14); }
            let r15 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r15);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008eeb1f); }
            return c.call(0x70007febb4, 0x70008eeb28, 0x70008eeb23);
        }
        10 => {
            c.w64(1, c.r64(0));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eeb2b); };
            c.w64(3, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eeb32); }
            blk = 3; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eeb38); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eeb3c); }
            let r17 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r17);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008eeb4c, 0x70008eeb47);
        }
        12 => {
            c.w64(3, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eeb52); }
            blk = 8; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008eeb58);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eeb74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008eeb74); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008eeb78); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld8(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008eeb7e); };
            c.w32(3, t3);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008eeb85);
        }
        1 => {
            return RegfileExit::jump(0x70008eeb88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eeb94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008eeb94); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008eeb98); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld8(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008eeb9e); };
            c.w32(3, (t3 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008eeba5);
        }
        1 => {
            return RegfileExit::jump(0x70008eeba8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eebb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008eebb4); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008eebb8); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2))) else { return RegfileExit::fault(0x70008eebbe); };
            c.w32(3, t3);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008eebc5);
        }
        1 => {
            return RegfileExit::jump(0x70008eebc8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eebd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008eebd4); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008eebd8); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2))) else { return RegfileExit::fault(0x70008eebde); };
            c.w32(3, (t3 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008eebe5);
        }
        1 => {
            return RegfileExit::jump(0x70008eebe8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eebf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008eebf4); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008eebf8); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x70008eebfe); };
            c.w32(3, t3);
            c.cmp(c.r32(3), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008eec0c);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eec0d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eec11); }
            let r4 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008eec17); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008eec1e); };
            c.w64(6, t6);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008eec25); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008eec29); };
            c.w64(8, t7);
            let Some(t8) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008eec30); };
            c.cmp(t8, c.r64(7), 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008eec3f, 0x70008eec3a);
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008eec3f); };
            c.w64(3, t9);
            blk = 6; continue;
        }
        5 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008eec45); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 6; continue;
        }
        6 => {
            c.w32(10, c.r32(3));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008eec5b); };
            c.w64(3, t10);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008eec62); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008eec65); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008eec6d); }
            return c.ret(0x0, 0x70008eec6e);
        }
        7 => {
            return RegfileExit::jump(0x70008eec70);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eec74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008eec74); };
            c.w64(0, t0);
            return c.ret(0x0, 0x70008eec7b);
        }
        1 => {
            return RegfileExit::jump(0x70008eec80);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eec94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eec94); };
            c.cmp(t0, c.r32(3), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008eec99); };
            c.w32(3, t1);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008eeca0); };
            c.w64(0, t2);
            return c.ret(0x0, 0x70008eeca7);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008eeca8); };
            c.w64(0, t3);
            return c.ret(0x0, 0x70008eecaf);
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008eecb0); };
            c.w64(0, t4);
            return c.ret(0x0, 0x70008eecb7);
        }
        5 => {
            return RegfileExit::jump(0x70008eecbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eecd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eecd4); };
            c.cmp(t0, c.r32(3), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008eecd9); };
            c.w32(3, t1);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008eece0); };
            c.w64(0, t2);
            return c.ret(0x0, 0x70008eece7);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008eece8); };
            c.w64(0, t3);
            return c.ret(0x0, 0x70008eecef);
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008eecf0); };
            c.w64(0, t4);
            return c.ret(0x0, 0x70008eecf7);
        }
        5 => {
            return RegfileExit::jump(0x70008eecfc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eed14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eed14); };
            c.cmp(t0, c.r32(3), 32);
            if c.cc_ne() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008eed19); };
            c.w32(3, t1);
            c.cmp(c.r32(1), c.r32(3), 32);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008eed20); };
            c.w64(0, t2);
            return c.ret(0x0, 0x70008eed27);
        }
        3 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008eed28); };
            c.w64(0, t3);
            return c.ret(0x0, 0x70008eed2f);
        }
        4 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008eed30); };
            c.w64(0, t4);
            return c.ret(0x0, 0x70008eed37);
        }
        5 => {
            return RegfileExit::jump(0x70008eed3c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008eed54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008eed54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008eed58); }
            let r0 = c.sub(c.r64(4), 0x38u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eed5e); };
            c.w32(0, t1);
            let r2 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r2);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008eed64); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(6)) { return RegfileExit::fault(0x70008eed68); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008eed6c); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 249; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eed78); };
            c.w32(7, t3);
            let r4 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r4);
            let Some(t5) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eed7e); };
            c.w32(8, t5);
            c.cmp(c.r16(8), 0xbau64, 16);
            if c.cc_b() { blk = 249; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r16(8), 0xbcu64, 16);
            if c.cc_e() { blk = 244; continue; }
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eeda4); };
            c.w32(9, t6);
            let r7 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r7);
            let Some(t8) = c.ld16(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eedab); };
            c.w32(9, t8);
            c.cmp(c.r16(9), 0x42u64, 16);
            if c.cc_ne() { blk = 8; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.movsd_rm(0, c.r64(3).wrapping_add(0x3)) { return RegfileExit::fault(0x70008eedb7); }
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
            let r9 = c.add(c.r64(12), c.r64(11), 64);
            c.w64(12, r9);
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
            if c.cc_ae() { blk = 234; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t10) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eee04); };
            c.w32(11, t10);
            c.logic(c.r32(11) & 0xe0000002u64, 32);
            if c.cc_e() { blk = 17; continue; }
            blk = 11; continue;
        }
        11 => {
            c.logic(c.r8(11) & 0x2u64, 8);
            if c.cc_e() { blk = 249; continue; }
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
            if c.cc_ne() { blk = 249; continue; }
            blk = 15; continue;
        }
        15 => {
            c.w64(9, c.r64(3));
            blk = 19; continue;
        }
        16 => {
            let Some(t11) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008eee38); };
            c.w32(9, t11);
            let r12 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r12);
            blk = 19; continue;
        }
        17 => {
            if let Some(sr) = c.shr(c.r64(11), 0x2u64, 64) { c.w64(11, sr); }
            let r13 = c.r64(11) & 0xffffffu64;
            c.w64(11, r13);
            c.logic(r13, 64);
            blk = 97; continue;
        }
        18 => {
            c.w64(9, c.r64(3));
            blk = 19; continue;
        }
        19 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008eee54); };
            c.w64(15, t14);
            c.w64(11, c.r64(0));
            blk = 21; continue;
        }
        20 => {
            let Some(t15) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008eee60); };
            c.w32(7, t15);
            let r16 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r16);
            let Some(t17) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eee67); };
            c.w32(8, t17);
            blk = 21; continue;
        }
        21 => {
            c.cmp(c.r32(8), 0x410u64, 32);
            if c.cc_le() { blk = 50; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t18) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008eee79); };
            c.w32(1, t18);
            c.logic(c.r32(1) & 0x200000u64, 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t19) = c.ld32(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008eee88); };
            c.w32(11, t19);
            let r20 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r20);
            c.logic(c.r32(1) & 0xffc00u64, 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 24; continue;
        }
        24 => {
            let r21 = c.shr(c.r32(1), 0xau64, 32).unwrap_or(c.r32(1));
            c.w32(1, r21);
            let r22 = c.r32(1) & 0x3ffu64;
            c.w32(1, r22);
            c.logic(r22, 32);
            c.cmp(c.r32(1), 0x20u64, 32);
            if c.cc_be() { blk = 39; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t23) = c.ld16(c.r64(11).wrapping_add(0x5)) else { return RegfileExit::fault(0x70008eeead); };
            c.w32(0, (t23 as u16 as i16 as i64 as u64));
            let Some(t24) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eeeb2); };
            c.w32(6, t24);
            let r25 = c.shr(c.r32(6), 0x2u64, 32).unwrap_or(c.r32(6));
            c.w32(6, r25);
            let r26 = c.sub(c.r32(0), 0x1u64, 32);
            c.w32(0, r26);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008eeebc); }
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 27; continue;
        }
        26 => {
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(2)) { return RegfileExit::fault(0x70008eeec4); }
            blk = 27; continue;
        }
        27 => {
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eeec8); };
            c.w64(2, t27);
            let r28 = c.sub(c.r32(2), c.r32(3), 32);
            c.w32(2, r28);
            let r29 = c.shr(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r29);
            let r30 = c.add(c.r32(2), c.r32(3), 32);
            c.w32(2, r30);
            c.w32(12, c.r64(2).wrapping_add(c.r64(2).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eeed9); };
            c.w32(12, t31);
            let r32 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r32);
            let r33 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r33);
            let r34 = c.r32(12) & 0x3ffu64;
            c.w32(12, r34);
            c.logic(r34, 32);
            c.w32(12, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            let Some(t35) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eeef3); };
            c.w32(12, t35);
            let r36 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r36);
            let Some(t37) = c.ld32(c.r64(12).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eeefb); };
            c.w32(12, t37);
            let r38 = c.shr(c.r32(12), 0x2u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r38);
            c.cmp(c.r32(6), c.r32(12), 32);
            if c.cc_be() { blk = 29; continue; }
            blk = 28; continue;
        }
        28 => {
            c.w32(3, c.r64(2).wrapping_add(0x1));
            let Some(t39) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008eef0c); };
            c.w64(2, t39);
            blk = 29; continue;
        }
        29 => {
            c.cmp(c.r32(3), c.r32(2), 32);
            if c.cc_ne() { blk = 26; continue; }
            blk = 30; continue;
        }
        30 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008eef14); }
            blk = 32; continue;
        }
        31 => {
            let r40 = c.add(c.r32(3), 0x1u64, 32);
            c.w32(3, r40);
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
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008eef28); };
            c.w32(12, t41);
            let r42 = c.sar(c.r32(12), 1u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r42);
            let r43 = c.shr(c.r32(12), 0x9u64, 32).unwrap_or(c.r32(12));
            c.w32(12, r43);
            let r44 = c.r32(12) & 0x3ffu64;
            c.w32(12, r44);
            c.logic(r44, 32);
            c.w32(2, c.r64(12).wrapping_add(c.r64(12).wrapping_mul(2)));
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t45) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eef42); };
            c.w32(2, t45);
            let r46 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r46);
            let Some(t47) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eef4a); };
            c.w32(7, t47);
            let r48 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r48);
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
            let Some(t49) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eef62); };
            c.w64(7, t49);
            blk = 61; continue;
        }
        37 => {
            let Some(t50) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eef6b); };
            c.w64(7, t50);
            blk = 61; continue;
        }
        38 => {
            let Some(t51) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008eef74); };
            c.w64(7, t51);
            blk = 61; continue;
        }
        39 => {
            c.w64(12, (c.r32(1) as u32 as i32 as i64 as u64));
            let r52 = c.imul2(c.r64(12), 0x3u64, 64);
            c.w64(12, r52);
            c.cmp(c.r64(12), 0x0u64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 40; continue;
        }
        40 => {
            let r53 = c.sub(c.r64(12), 0x3u64, 64);
            c.w64(12, r53);
            let Some(t54) = c.ld32(c.r64(11).wrapping_add(c.r64(12).wrapping_mul(4)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eef92); };
            c.cmp(t54, c.r32(9), 32);
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
            let Some(t55) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eefa8); };
            c.w32(11, t55);
            let r56 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r56);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 45; continue; }
            blk = 44; continue;
        }
        44 => {
            c.w64(11, c.r64(15));
            blk = 45; continue;
        }
        45 => {
            let Some(t57) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008eefb8); };
            c.w32(12, t57);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r58 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r58);
            let Some(t59) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008eefc6); };
            c.w32(0, t59);
            let r60 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r60);
            let r61 = c.r64(0) & c.r64(12);
            c.w64(0, r61);
            c.logic(r61, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 47; continue;
        }
        46 => {
            let r62 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r62);
            let r63 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r63);
            c.w64(2, c.r64(12));
            let r64 = c.r64(2) & c.r64(0);
            c.w64(2, r64);
            c.logic(r64, 64);
            c.w64(0, c.r64(2));
            blk = 47; continue;
        }
        47 => {
            c.w64(2, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            let Some(t65) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008eefe9); };
            c.w32(2, t65);
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008eefee); };
            c.cmp(t66, c.r32(2), 32);
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
            if c.cc_ne() { blk = 249; continue; }
            blk = 51; continue;
        }
        51 => {
            let Some(t67) = c.ld8(c.r64(7).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008ef013); };
            c.logic(t67 & 0x24u64, 8);
            if c.cc_ne() { blk = 249; continue; }
            blk = 52; continue;
        }
        52 => {
            let Some(t68) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef01d); };
            c.w32(11, t68);
            let r69 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r69);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            c.w64(11, c.r64(15));
            blk = 54; continue;
        }
        54 => {
            let Some(t70) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef02d); };
            c.w32(12, t70);
            c.w64(12, (c.r32(12) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(12), 1u64, 64) { c.w64(12, sr); }
            let r71 = c.sub(c.r64(12), 0x1u64, 64);
            c.w64(12, r71);
            let Some(t72) = c.ld32(c.r64(9).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef03b); };
            c.w32(0, t72);
            let r73 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r73);
            let r74 = c.r64(0) & c.r64(12);
            c.w64(0, r74);
            c.logic(r74, 64);
            c.w32(3, 0);
            c.logic(0, 32);
            blk = 56; continue;
        }
        55 => {
            let r75 = c.add(c.r64(3), 0x1u64, 64);
            c.w64(3, r75);
            let r76 = c.add(c.r64(0), c.r64(3), 64);
            c.w64(0, r76);
            c.w64(2, c.r64(12));
            let r77 = c.r64(2) & c.r64(0);
            c.w64(2, r77);
            c.logic(r77, 64);
            c.w64(0, c.r64(2));
            blk = 56; continue;
        }
        56 => {
            c.w64(2, c.r64(0).wrapping_add(0x5));
            let Some(t78) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef05d); };
            c.w32(1, t78);
            let r79 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r79);
            let Some(t80) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ef065); };
            c.cmp(t80, c.r32(1), 32);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            let Some(t81) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ef06e); };
            c.cmp(t81, c.r32(1), 32);
            if c.cc_e() { blk = 55; continue; }
            blk = 58; continue;
        }
        58 => {
            let Some(t82) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef077); };
            c.cmp(t82, c.r32(9), 32);
            if c.cc_ne() { blk = 55; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t83) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef07d); };
            c.w32(11, t83);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(2), c.r64(11), 64);
            if c.cc_ae() { blk = 242; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t84) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef090); };
            c.w32(11, t84);
            let Some(t85) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ef094); };
            c.cmp(t85, c.r32(11), 32);
            if c.cc_ne() { blk = 227; continue; }
            blk = 61; continue;
        }
        61 => {
            c.cmp(c.r32(8), 0x82eu64, 32);
            if c.cc_ne() { blk = 92; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t86) = c.ld32(c.r64(9).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef0ae); };
            c.w32(8, t86);
            let r87 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r87);
            let Some(t88) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef0b5); };
            c.w32(8, t88);
            c.cmp(c.r16(8), 0x40u64, 16);
            if c.cc_ae() { blk = 92; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t89) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef0c5); };
            c.w32(11, t89);
            let r90 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r90);
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
            let r91 = c.r32(0) & 0x7u64;
            c.w32(0, r91);
            c.logic(r91, 32);
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
            blk = 236; continue;
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
            blk = 236; continue;
        }
        75 => {
            let Some(t92) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef12b); };
            c.w32(11, t92);
            let Some(t93) = c.ld32(c.r64(13).wrapping_add(0xc8)) else { return RegfileExit::fault(0x70008ef130); };
            c.cmp(t93, c.r32(11), 32);
            if c.cc_ne() { blk = 237; continue; }
            blk = 76; continue;
        }
        76 => {
            let Some(t94) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef13d); };
            c.w32(12, t94);
            let r95 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r95);
            let Some(t96) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef145); };
            c.w32(11, t96);
            let r97 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r97);
            let Some(t98) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef14d); };
            c.w32(11, t98);
            blk = 67; continue;
        }
        77 => {
            let Some(t99) = c.ld32(c.r64(12).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef154); };
            c.w32(11, t99);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r100 = c.add(c.r64(8), c.r64(11), 64);
            c.w64(8, r100);
            let Some(t101) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef162); };
            c.w32(12, t101);
            let r102 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r102);
            let Some(t103) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef16a); };
            c.w32(11, t103);
            let r104 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r104);
            let Some(t105) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef172); };
            c.w32(11, t105);
            blk = 67; continue;
        }
        78 => {
            let Some(t106) = c.ld32(c.r64(12).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef179); };
            c.w32(12, t106);
            let r107 = c.add(c.r64(12), c.r64(14), 64);
            c.w64(12, r107);
            let Some(t108) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef181); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef189); };
            c.w32(11, t110);
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
            if c.cc_ne() { blk = 238; continue; }
            blk = 83; continue;
        }
        83 => {
            let r111 = c.add(c.r64(12), 0xbu64, 64);
            c.w64(12, r111);
            blk = 84; continue;
        }
        84 => {
            let r112 = c.r32(11) & 0x8u64;
            c.w32(11, r112);
            c.logic(r112, 32);
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t113) = c.ld16(c.r64(12).wrapping_add(c.r64(8).wrapping_mul(2))) else { return RegfileExit::fault(0x70008ef1bf); };
            c.w32(8, t113);
            blk = 87; continue;
        }
        86 => {
            let Some(t114) = c.ld8(c.r64(12).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008ef1c6); };
            c.w32(8, t114);
            blk = 87; continue;
        }
        87 => {
            c.cmp(c.r32(8), 0x2du64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 88; continue;
        }
        88 => {
            c.cmp(c.r32(8), 0x49u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 89; continue;
        }
        89 => {
            c.cmp(c.r32(8), 0x4eu64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 90; continue;
        }
        90 => {
            c.cmp(c.r32(8), 0x30u64, 32);
            if c.cc_b() { blk = 92; continue; }
            blk = 91; continue;
        }
        91 => {
            c.cmp(c.r32(8), 0x39u64, 32);
            if c.cc_be() { blk = 249; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ef1f9); };
            c.w64(12, t115);
            let Some(t116) = c.ld32(c.r64(12).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef1fd); };
            c.w32(8, t116);
            let Some(t117) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008ef202); };
            c.cmp(t117, c.r32(8), 32);
            if c.cc_ne() { blk = 94; continue; }
            blk = 93; continue;
        }
        93 => {
            let Some(t118) = c.ld8(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef20b); };
            c.logic(t118 & 0x1u64, 8);
            if c.cc_ne() { blk = 228; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t119) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef217); };
            c.w32(11, t119);
            let r120 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r120);
            let Some(t121) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ef21e); };
            c.cmp(t121, c.r32(11), 32);
            if c.cc_e() { blk = 228; continue; }
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
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008ef239); }
            c.w64(9, c.r64(8));
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(0));
            blk = 99; continue;
        }
        98 => {
            let Some(t122) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef248); };
            c.w32(8, t122);
            let r123 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r123);
            let Some(t124) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef24f); };
            c.w32(9, t124);
            blk = 99; continue;
        }
        99 => {
            c.cmp(c.r32(9), 0x410u64, 32);
            if c.cc_le() { blk = 249; continue; }
            blk = 100; continue;
        }
        100 => {
            let Some(t125) = c.ld8(c.r64(8).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ef261); };
            c.w32(9, t125);
            let r126 = c.shr(c.r32(9), 0x2u64, 32).unwrap_or(c.r32(9));
            c.w32(9, r126);
            let r127 = c.r32(9) & 0x3fu64;
            c.w32(9, r127);
            c.logic(r127, 32);
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
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
            blk = 249; continue;
        }
        169 => {
            blk = 174; continue;
        }
        170 => {
            let Some(t128) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef48a); };
            c.w32(7, t128);
            let r129 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r129);
            let Some(t130) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef490); };
            c.w32(9, t130);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_ae() { blk = 175; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w64(9, 0xfff7fffffff7ffffu64);
            let Some(t131) = c.ld64(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(8)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef4a9); };
            c.cmp(t131, c.r64(9), 64);
            if c.cc_e() { blk = 193; continue; }
            blk = 172; continue;
        }
        172 => {
            blk = 227; continue;
        }
        173 => {
            let Some(t132) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef4b9); };
            c.w32(9, t132);
            let r133 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r133);
            let Some(t134) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef4c0); };
            c.w32(9, t134);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 227; continue; }
            blk = 174; continue;
        }
        174 => {
            let Some(t135) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef4cd); };
            c.w32(7, t135);
            let r136 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r136);
            let Some(t137) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef4d3); };
            c.w32(9, t137);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 177; continue; }
            blk = 175; continue;
        }
        175 => {
            c.w32(7, 0xfffffffeu64);
            c.cmp(c.r64(7), c.r64(11), 64);
            if c.cc_b() { blk = 249; continue; }
            blk = 176; continue;
        }
        176 => {
            blk = 193; continue;
        }
        177 => {
            let Some(t138) = c.ld32(c.r64(7).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef4f5); };
            c.w32(7, t138);
            let Some(t139) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ef4fa); };
            c.cmp(t139, c.r32(7), 32);
            if c.cc_e() { blk = 193; continue; }
            blk = 178; continue;
        }
        178 => {
            blk = 227; continue;
        }
        179 => {
            let Some(t140) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef50c); };
            c.w32(9, t140);
            let r141 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r141);
            let Some(t142) = c.ld32(c.r64(9).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef513); };
            c.w32(9, t142);
            c.cmp(c.r64(11), c.r64(9), 64);
            if c.cc_b() { blk = 227; continue; }
            blk = 180; continue;
        }
        180 => {
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(8)) { return RegfileExit::fault(0x70008ef520); }
            c.w32(9, 0xfffffffeu64);
            c.cmp(c.r64(9), c.r64(11), 64);
            if c.cc_b() { blk = 249; continue; }
            blk = 181; continue;
        }
        181 => {
            let Some(t143) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef533); };
            c.w32(7, t143);
            let r144 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r144);
            let Some(t145) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef539); };
            c.w32(9, t145);
            c.w64(9, (c.r32(9) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(9), 1u64, 64) { c.w64(9, sr); }
            let r146 = c.sub(c.r64(9), 0x1u64, 64);
            c.w64(9, r146);
            c.w32(2, c.r32(11));
            c.w64(10, c.r64(4));
            let r147 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r147);
            let r148 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r148);
            c.logic(r148, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ef555); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(7)) { return RegfileExit::fault(0x70008ef55a); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008ef55e); }
            c.w64(1, c.r64(13));
            let Some(t149) = c.ld64(c.r64(13).wrapping_add(0x1278)) else { return RegfileExit::fault(0x70008ef565); };
            c.w64(0, t149);
            c.w64(10, 0x70008ef573u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ef573); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ef577); }
            return c.call(c.r64(0), 0x70008ef57d, 0x70008ef57b);
        }
        182 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ef57d); }
            let Some(t150) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ef585); };
            c.w64(4, t150);
            c.w32(7, c.r32(0));
            let Some(t151) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ef58c); };
            let r152 = c.r64(7) & t151;
            c.w64(7, r152);
            c.logic(r152, 64);
            c.w32(8, 0);
            c.logic(0, 32);
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008ef593); };
            c.w64(9, t153);
            blk = 184; continue;
        }
        183 => {
            let r154 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r154);
            let r155 = c.add(c.r64(7), c.r64(8), 64);
            c.w64(7, r155);
            let Some(t156) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008ef5a0); };
            c.w64(11, t156);
            let r157 = c.r64(11) & c.r64(7);
            c.w64(11, r157);
            c.logic(r157, 64);
            c.w64(7, c.r64(11));
            blk = 184; continue;
        }
        184 => {
            c.w64(11, c.r64(7).wrapping_add(c.r64(7).wrapping_mul(2)).wrapping_add(0x4));
            let Some(t158) = c.ld32(c.r64(9).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef5af); };
            c.w32(11, t158);
            let r159 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r159);
            let Some(t160) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008ef5b7); };
            c.cmp(t160, c.r32(11), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 185; continue;
        }
        185 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 190; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t161) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ef5c6); };
            c.cmp(t161, c.r32(11), 32);
            if c.cc_e() { blk = 183; continue; }
            blk = 187; continue;
        }
        187 => {
            c.xorpd_rr(0, 0);
            let Some(t162) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ef5d3); };
            c.cvtsi2sd(0, (t162 as i64));
            if !c.movsd_rm(1, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ef5d9); }
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
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ef5f3); };
            c.cmp(c.r64(11), t163, 64);
            if c.cc_e() { blk = 227; continue; }
            blk = 191; continue;
        }
        191 => {
            blk = 183; continue;
        }
        192 => {
            let Some(t164) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ef5ff); };
            c.w64(8, t164);
            let Some(t165) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ef603); };
            c.w64(11, t165);
            blk = 193; continue;
        }
        193 => {
            c.w64(7, c.r64(8));
            let Some(t166) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef60a); };
            c.w32(7, t166);
            let r167 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r167);
            let Some(t168) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ef610); };
            c.w64(0, t168);
            let Some(t169) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ef614); };
            c.w64(3, t169);
            let Some(t170) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ef618); };
            c.w64(6, t170);
            let Some(t171) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008ef61c); };
            c.cmp(t171, c.r32(7), 32);
            if c.cc_e() { blk = 228; continue; }
            blk = 194; continue;
        }
        194 => {
            blk = 98; continue;
        }
        195 => {
            let Some(t172) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef62e); };
            c.w32(8, t172);
            let r173 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r173);
            let Some(t174) = c.ld8(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008ef635); };
            c.logic(t174 & 0x4u64, 8);
            if c.cc_ne() { blk = 228; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t175) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ef640); };
            c.cmp(t175, c.r64(11), 64);
            if c.cc_a() { blk = 227; continue; }
            blk = 197; continue;
        }
        197 => {
            blk = 228; continue;
        }
        198 => {
            let Some(t176) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef64f); };
            c.w32(8, t176);
            let r177 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r177);
            let Some(t178) = c.ld32(c.r64(8).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008ef656); };
            c.w32(9, t178);
            c.logic(c.r8(9) & 0x10u64, 8);
            if c.cc_ne() { blk = 224; continue; }
            blk = 199; continue;
        }
        199 => {
            c.logic(c.r8(9) & 0x4u64, 8);
            if c.cc_ne() { blk = 228; continue; }
            blk = 200; continue;
        }
        200 => {
            let Some(t179) = c.ld64(c.r64(8).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef66e); };
            c.w64(8, t179);
            let Some(t180) = c.ld64(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef672); };
            c.w64(9, t180);
            let Some(t181) = c.ld8(c.r64(7).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008ef676); };
            c.logic(t181 & 0x1u64, 8);
            if c.cc_ne() { blk = 203; continue; }
            blk = 201; continue;
        }
        201 => {
            let Some(t182) = c.ld64(c.r64(7).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008ef67c); };
            c.w64(12, t182);
            let r183 = c.add(c.r64(9), c.r64(12), 64);
            c.w64(9, r183);
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 228; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t184) = c.ld64(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008ef68c); };
            c.w64(7, t184);
            c.w64(0, c.r64(7));
            blk = 226; continue;
        }
        203 => {
            c.cmp(c.r64(9), c.r64(8), 64);
            if c.cc_a() { blk = 228; continue; }
            blk = 204; continue;
        }
        204 => {
            let Some(t185) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef6a1); };
            c.w32(7, t185);
            let r186 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r186);
            let Some(t187) = c.ld8(c.r64(7).wrapping_add(0xa)) else { return RegfileExit::fault(0x70008ef6a7); };
            c.w32(7, t187);
            let r188 = c.shr(c.r32(7), 0x2u64, 32).unwrap_or(c.r32(7));
            c.w32(7, r188);
            let r189 = c.r32(7) & 0x3fu64;
            c.w32(7, r189);
            c.logic(r189, 32);
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
            return RegfileExit::fault(0x70008ef6f7);
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
            let r190 = c.sub(c.r64(8), c.r64(9), 64);
            c.w64(8, r190);
            c.w64(0, c.r64(8));
            c.cqo();
            c.idiv(c.r64(7), 64);
            let Some(t191) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ef71d); };
            c.w64(8, t191);
            blk = 226; continue;
        }
        224 => {
            c.w64(10, c.r64(4));
            let r192 = c.sub(c.r64(4), 0x28u64, 64);
            c.w64(4, r192);
            let r193 = c.r64(4) & 0xfffffffffffffff0u64;
            c.w64(4, r193);
            c.logic(r193, 64);
            if !c.st64(c.r64(4).wrapping_add(0x20), c.r64(10)) { return RegfileExit::fault(0x70008ef72e); }
            c.w64(1, c.r64(13));
            c.w64(2, c.r64(7));
            let Some(t194) = c.ld64(c.r64(13).wrapping_add(0x13b8)) else { return RegfileExit::fault(0x70008ef739); };
            c.w64(0, t194);
            c.w64(10, 0x70008ef747u64);
            if !c.st64(c.r64(13).wrapping_add(0x28), c.r64(10)) { return RegfileExit::fault(0x70008ef747); }
            if !c.st64(c.r64(13).wrapping_add(0x20), c.r64(5)) { return RegfileExit::fault(0x70008ef74b); }
            return c.call(c.r64(0), 0x70008ef751, 0x70008ef74f);
        }
        225 => {
            if !c.st64(c.r64(13).wrapping_add(0x20), 0x0u64) { return RegfileExit::fault(0x70008ef751); }
            let Some(t195) = c.ld64(c.r64(4).wrapping_add(0x20)) else { return RegfileExit::fault(0x70008ef759); };
            c.w64(4, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ef75e); };
            c.w64(11, t196);
            blk = 226; continue;
        }
        226 => {
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_ae() { blk = 228; continue; }
            blk = 227; continue;
        }
        227 => {
            let Some(t197) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008ef767); };
            c.w64(7, t197);
            blk = 229; continue;
        }
        228 => {
            let Some(t198) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ef770); };
            c.w64(7, t198);
            blk = 229; continue;
        }
        229 => {
            let Some(t199) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008ef777); };
            c.cmp(t199, c.r32(7), 32);
            if c.cc_e() { blk = 232; continue; }
            blk = 230; continue;
        }
        230 => {
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ef780); };
            c.w64(0, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ef784); };
            c.w64(3, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ef788); };
            c.w64(6, t202);
            return c.call(0x7000870914, 0x70008ef791, 0x70008ef78c);
        }
        231 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ef794); }
            return c.ret(0x0, 0x70008ef795);
        }
        232 => {
            let Some(t203) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008ef796); };
            c.w64(0, t203);
            blk = 231; continue;
        }
        233 => {
            blk = 249; continue;
        }
        234 => {
            c.cmp(c.r16(9), 0x43u64, 16);
            if c.cc_ne() { blk = 249; continue; }
            blk = 235; continue;
        }
        235 => {
            let Some(t204) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008ef7af); };
            c.w32(9, t204);
            let r205 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r205);
            blk = 19; continue;
        }
        236 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ef7bb); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008ef7bf); }
            blk = 240; continue;
        }
        237 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ef7c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008ef7c9); }
            blk = 240; continue;
        }
        238 => {
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(9)) { return RegfileExit::fault(0x70008ef7cf); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(7)) { return RegfileExit::fault(0x70008ef7d3); }
            c.logic(c.r8(11) & 0x10u64, 8);
            if c.cc_ne() { blk = 240; continue; }
            blk = 239; continue;
        }
        239 => {
            let Some(t206) = c.ld64(c.r64(12).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008ef7dd); };
            c.w64(12, t206);
            blk = 84; continue;
        }
        240 => {
            if !c.push64(c.r64(9)) { return RegfileExit::fault(0x70008ef7e7); }
            if !c.push64(c.r64(6)) { return RegfileExit::fault(0x70008ef7e9); }
            let Some(t207) = c.ld64(c.r64(13).wrapping_add(0x2770)) else { return RegfileExit::fault(0x70008ef7ea); };
            c.w64(3, t207);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008ef7fb, 0x70008ef7f6);
        }
        241 => {
            let r208 = c.sar(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r208);
            c.w64(8, c.r64(0));
            let Some(t209) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008ef800); };
            c.w64(7, t209);
            let Some(t210) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ef804); };
            c.w64(9, t210);
            let Some(t211) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008ef808); };
            c.w64(15, t211);
            blk = 87; continue;
        }
        242 => {
            return RegfileExit::fault(0x70008ef814);
        }
        243 => {
            blk = 249; continue;
        }
        244 => {
            c.w64(0, c.r64(3));
            return c.call(0x70008a1af4, 0x70008ef81f, 0x70008ef81a);
        }
        245 => {
            let Some(t212) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef81f); };
            c.w32(7, t212);
            let Some(t213) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008ef822); };
            c.cmp(t213, c.r32(7), 32);
            if c.cc_ne() { blk = 247; continue; }
            blk = 246; continue;
        }
        246 => {
            let Some(t214) = c.ld8(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef82b); };
            c.logic(t214 & 0x1u64, 8);
            if c.cc_ne() { blk = 228; continue; }
            blk = 247; continue;
        }
        247 => {
            c.w64(3, c.r64(0));
            let Some(t215) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ef838); };
            c.w64(0, t215);
            let Some(t216) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ef83c); };
            c.w64(6, t216);
            return c.call(0x70008bced4, 0x70008ef845, 0x70008ef840);
        }
        248 => {
            c.w64(7, c.r64(0));
            blk = 229; continue;
        }
        249 => {
            let Some(t217) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ef84d); };
            if !c.push64(t217) { return RegfileExit::fault(0x70008ef84d); }
            let Some(t218) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ef850); };
            if !c.push64(t218) { return RegfileExit::fault(0x70008ef850); }
            let Some(t219) = c.ld64(c.r64(13).wrapping_add(0x23b8)) else { return RegfileExit::fault(0x70008ef853); };
            c.w64(3, t219);
            c.w32(0, 0x2u64);
            let Some(t220) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ef85f); };
            c.w64(6, t220);
            return c.call(0x700086ed74, 0x70008ef868, 0x70008ef863);
        }
        250 => {
            c.w64(7, c.r64(0));
            blk = 229; continue;
        }
        251 => {
            return RegfileExit::jump(0x70008ef874);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ef894(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ef894); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ef898); }
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef89a); };
            c.w32(0, t0);
            let r1 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1);
            return c.call(0x7000871cb4, 0x70008ef8a5, 0x70008ef8a0);
        }
        1 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ef8aa); }
            return c.ret(0x0, 0x70008ef8ab);
        }
        2 => {
            return RegfileExit::jump(0x70008ef8b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ef8b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ef8b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ef8b8); }
            let Some(t0) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008ef8ba); };
            c.w32(0, t0);
            let r1 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r1);
            c.w32(1, 0x2u64);
            return c.call(0x7000820e54, 0x70008ef8ca, 0x70008ef8c5);
        }
        1 => {
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ef8cf); }
            return c.ret(0x0, 0x70008ef8d0);
        }
        2 => {
            return RegfileExit::jump(0x70008ef8d4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ef8f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ef8f4); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ef8f8); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x70008ef8fe); };
            c.w32(3, t3);
            c.w64(1, c.r64(3));
            let r4 = c.add(c.r32(1), c.r32(3), 32);
            c.w32(1, r4);
            if c.cc_o() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            return c.ret(0x0, 0x70008ef90b);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ef90c); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ef910); }
            let r5 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r5);
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008ef916); };
            c.w64(1, t6);
            let Some(t7) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008ef91d); };
            c.w64(6, t7);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ef924); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008ef928); };
            c.w64(8, t8);
            let Some(t9) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008ef92f); };
            c.cmp(t9, c.r64(7), 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008ef93e, 0x70008ef939);
        }
        4 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ef93e); };
            c.w64(3, t10);
            blk = 6; continue;
        }
        5 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008ef944); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 6; continue;
        }
        6 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(3) as u32 as i32 as i64));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ef956); };
            c.w64(3, t11);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008ef95d); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008ef960); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ef968); }
            return c.ret(0x0, 0x70008ef969);
        }
        7 => {
            return RegfileExit::jump(0x70008ef96c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008ef974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008ef974); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008ef978); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008ef97e); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008ef982); };
            c.w32(7, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008ef985); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008ef989); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008ef98d); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008ef991); }
            blk = 2; continue;
        }
        1 => {
            c.w64(1, c.r64(0));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008ef99f); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008ef9a6); };
            c.cmp(t5, c.r32(8), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008ef9af); };
            c.cmp(t6, 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008ef9bf, 0x70008ef9ba);
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008ef9bf); };
            c.w64(6, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008ef9c3); };
            c.w64(3, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008ef9c7); };
            c.w64(2, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008ef9cb); };
            c.w64(7, t10);
            blk = 1; continue;
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008ef9d1); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            c.w64(0, c.r64(1));
            blk = 1; continue;
        }
        8 => {
            let r13 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r13);
            blk = 11; continue;
        }
        9 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008ef9e0); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(1, c.r32(1));
            blk = 11; continue;
        }
        11 => {
            let r14 = c.add(c.r64(2), c.r64(7), 64);
            c.w64(2, r14);
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)), c.r32(1)) { return RegfileExit::fault(0x70008ef9f5); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008ef9fd); }
            return c.ret(0x0, 0x70008ef9fe);
        }
        12 => {
            let r15 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r15);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008efa03); }
            return c.call(0x70008708b4, 0x70008efa0d, 0x70008efa08);
        }
        13 => {
            let Some(t16) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008efa0d); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r17);
            blk = 10; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008efa18);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efa34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008efa34); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008efa38); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008efa3e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008efa42); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008efa46); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008efa4a); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008efa5b, 0x70008efa56);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008efa5e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008efa62); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008efa66); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008efa6a); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008efa73); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008efa79); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008efa82); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008efa89); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_o() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, c.r32(2));
            blk = 9; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            blk = 9; continue;
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008efaa2); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008efaa8); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efaae); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efab2); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r15);
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)), c.r32(2)) { return RegfileExit::fault(0x70008efab8); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008efac0); }
            return c.ret(0x0, 0x70008efac1);
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        13 => {
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008efacd); }
            return c.call(0x70008708b4, 0x70008efad7, 0x70008efad2);
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008efad7); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r18);
            blk = 7; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008efae4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efaf4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efaf4); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efaf8); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            if !c.movss_rm(0, c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4))) { return RegfileExit::fault(0x70008efafe); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008efb03); };
            c.w64(3, t3);
            let Some(t4) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008efb0a); };
            c.w64(1, t4);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008efb11); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008efb18); };
            c.cmp(t6, c.r64(6), 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008efb1d); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            c.cvtss2sd(0, c.xmm_u32(0));
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008efb28); };
            c.w64(1, t7);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008efb2f); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008efb32); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008efb3a);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008efb3b); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008efb3f); }
            let r8 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r8);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008efb4a); }
            return c.call(0x70007febf4, 0x70008efb54, 0x70008efb4f);
        }
        4 => {
            c.w64(3, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008efb57); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008efb5f); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008efb64);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efb74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008efb74); }
            c.cvtsd2ss(0, c.xmm_u64(0));
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efb7d); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efb81); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            if !c.movss_mr(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)), 0) { return RegfileExit::fault(0x70008efb87); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008efb8e);
        }
        1 => {
            return RegfileExit::jump(0x70008efb90);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efb94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008efb94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008efb98); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008efb9e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008efba2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008efba6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008efbaa); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008efbbb, 0x70008efbb6);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008efbbe); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008efbc2); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008efbc6); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008efbca); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008efbd3); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008efbd9); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008efbe2); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008efbe9); }
            c.cvtsd2ss(0, c.xmm_u64(0));
            blk = 8; continue;
        }
        7 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            c.xorpd_rr(0, 0);
            c.cvtsi2ss(0, (c.r32(2) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.w64(2, c.r64(0));
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008efc00); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008efc06); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efc0c); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efc10); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r15);
            if !c.movss_mr(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)), 0) { return RegfileExit::fault(0x70008efc16); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008efc20); }
            return c.ret(0x0, 0x70008efc21);
        }
        11 => {
            c.w32(0, 0x2u64);
            blk = 10; continue;
        }
        12 => {
            return RegfileExit::jump(0x70008efc2c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efc34(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efc34); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efc38); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8))) { return RegfileExit::fault(0x70008efc3e); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008efc43); };
            c.w64(3, t3);
            let Some(t4) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008efc4a); };
            c.w64(1, t4);
            c.w64(6, c.r64(1).wrapping_add(0xc));
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008efc51); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(7)) else { return RegfileExit::fault(0x70008efc58); };
            c.cmp(t6, c.r64(6), 64);
            if c.cc_be() { blk = 3; continue; }
            blk = 1; continue;
        }
        1 => {
            if !c.st64(c.r64(3), c.r64(6)) { return RegfileExit::fault(0x70008efc5d); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 2; continue;
        }
        2 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008efc64); };
            c.w64(1, t7);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008efc6b); }
            if !c.movsd_mr(c.r64(3).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008efc6e); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008efc76);
        }
        3 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008efc77); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008efc7b); }
            let r8 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r8);
            c.w32(2, 0xcu64);
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x10), 0) { return RegfileExit::fault(0x70008efc86); }
            return c.call(0x70007febf4, 0x70008efc90, 0x70008efc8b);
        }
        4 => {
            c.w64(3, c.r64(0));
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x10)) { return RegfileExit::fault(0x70008efc93); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008efc9b); }
            blk = 2; continue;
        }
        5 => {
            return RegfileExit::jump(0x70008efca0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efcb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efcb4); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efcb8); };
            c.w32(6, t1);
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008efcbb); }
            let r2 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r2);
            if !c.movsd_mr(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)), 0) { return RegfileExit::fault(0x70008efcc3); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008efcca);
        }
        1 => {
            return RegfileExit::jump(0x70008efccc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efcd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008efcd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008efcd8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008efcde); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008efce2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008efce6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008efcea); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008efcfb, 0x70008efcf6);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008efcfe); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008efd02); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008efd06); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008efd0a); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008efd13); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008efd19); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008efd22); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008efd29); }
            blk = 8; continue;
        }
        7 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(2) as u32 as i32 as i64));
            blk = 8; continue;
        }
        8 => {
            c.w64(2, c.r64(0));
            let Some(t10) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008efd3d); };
            c.w32(1, t10);
            let r11 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r11);
            let Some(t12) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008efd43); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 11; continue; }
            blk = 9; continue;
        }
        9 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efd49); };
            c.w64(1, t13);
            let Some(t14) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efd4d); };
            c.w32(2, t14);
            let r15 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r15);
            if !c.movsd_mr(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)), 0) { return RegfileExit::fault(0x70008efd53); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 10; continue;
        }
        10 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008efd5d); }
            return c.ret(0x0, 0x70008efd5e);
        }
        11 => {
            c.w32(0, 0x2u64);
            blk = 10; continue;
        }
        12 => {
            return RegfileExit::jump(0x70008efd68);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efd74(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efd74); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efd78); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld8(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008efd7e); };
            c.w32(3, t3);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008efd85);
        }
        1 => {
            return RegfileExit::jump(0x70008efd88);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efd94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008efd94); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008efd98); };
            c.w32(6, t1);
            let r2 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r2);
            let r3 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r3);
            if !c.st8(c.r64(3).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008efda0); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008efda5);
        }
        1 => {
            return RegfileExit::jump(0x70008efda8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008efdb4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008efdb4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008efdb8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008efdbe); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008efdc2); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008efdc6); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008efdca); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008efddb, 0x70008efdd6);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008efdde); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008efde2); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008efde6); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008efdea); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008efdf7); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008efdfd); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008efe06); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008efe0d); }
            c.xorpd_rr(1, 1);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_ae() { blk = 34; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w64(10, 0x406fe00000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_ae() { blk = 33; continue; }
            blk = 8; continue;
        }
        8 => {
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(1));
            if c.cc_a() { blk = 16; continue; }
            blk = 9; continue;
        }
        9 => {
            c.w64(10, 0xc330000000000000u64);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(0));
            if c.cc_ae() { blk = 15; continue; }
            blk = 10; continue;
        }
        10 => {
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(0));
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            c.mov128_rr(2, 0);
            blk = 21; continue;
        }
        12 => {
            c.mov128_rr(2, 0);
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008efe69); };
            c.w64(10, t9);
            if !c.xorpd_rm(2, c.r64(10)) { return RegfileExit::fault(0x70008efe70); }
            c.w64(10, 0x4330000000000000u64);
            c.movq_xr(3, 10);
            c.mov128_rr(4, 2);
            c.arith_sd(0, 4, c.xmm_u64(3));
            c.arith_sd(1, 4, c.xmm_u64(3));
            c.ucomisd(c.xmm_u64(2), c.xmm_u64(4));
            if c.cc_be() { blk = 14; continue; }
            blk = 13; continue;
        }
        13 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(0, 2, c.xmm_u64(4));
            c.mov128_rr(4, 2);
            blk = 14; continue;
        }
        14 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x11d0)) else { return RegfileExit::fault(0x70008efea9); };
            c.w64(10, t10);
            if !c.xorpd_rm(4, c.r64(10)) { return RegfileExit::fault(0x70008efeb0); }
            c.mov128_rr(2, 4);
            blk = 21; continue;
        }
        15 => {
            c.mov128_rr(2, 0);
            blk = 21; continue;
        }
        16 => {
            let Some(t11) = c.ld64(0x70008efe76u64) else { return RegfileExit::fault(0x70008efebe); };
            c.w64(10, t11);
            c.movq_xr(2, 10);
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(2));
            if c.cc_ae() { blk = 20; continue; }
            blk = 17; continue;
        }
        17 => {
            c.mov128_rr(3, 0);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            c.mov128_rr(2, 3);
            blk = 21; continue;
        }
        19 => {
            c.pcmpeqd_rr(2, 2);
            c.psllq(2, 0x36);
            c.psrlq(2, 0x2);
            c.arith_sd(1, 3, c.xmm_u64(2));
            c.mov128_rr(2, 3);
            blk = 21; continue;
        }
        20 => {
            c.mov128_rr(2, 0);
            blk = 21; continue;
        }
        21 => {
            c.pcmpeqd_rr(3, 3);
            c.psllq(3, 0x37);
            c.psrlq(3, 0x2);
            c.arith_sd(0, 3, c.xmm_u64(2));
            c.ucomisd(c.xmm_u64(0), c.xmm_u64(3));
            if c.cc_a() { blk = 29; continue; }
            blk = 22; continue;
        }
        22 => {
            c.ucomisd(c.xmm_u64(3), c.xmm_u64(0));
            if c.cc_a() { blk = 28; continue; }
            blk = 23; continue;
        }
        23 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x3f);
            c.psrlq(0, 0x1);
            c.mov128_rr(3, 2);
            c.w64(1, c.r64(0));
            let r12 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r12);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008eff36); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008eff3b); }
            if !c.movsd_mr(c.r64(4), 3) { return RegfileExit::fault(0x70008eff3e); }
            if !c.fld_m64(c.r64(4)) { return RegfileExit::fault(0x70008eff43); }
            blk = 24; continue;
        }
        24 => {
            c.fprem();
            c.fnstsw_ax();
            let r13 = c.shr(c.r32(0), 0x8u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r13);
            let r14 = c.r32(0) & 0xffu64;
            c.w32(0, r14);
            c.logic(r14, 32);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008eff52); }
            if !c.popfq() { return RegfileExit::fault(0x70008eff53); }
            if c.cc_p() { blk = 24; continue; }
            blk = 25; continue;
        }
        25 => {
            c.fstp_st(1);
            if !c.fstp_m64(c.r64(4)) { return RegfileExit::fault(0x70008eff58); }
            if !c.movsd_rm(3, c.r64(4)) { return RegfileExit::fault(0x70008eff5b); }
            let r15 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r15);
            c.ucomisd(c.xmm_u64(1), c.xmm_u64(3));
            if c.cc_p() { blk = 30; continue; }
            blk = 26; continue;
        }
        26 => {
            if c.cc_e() { blk = 31; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 30; continue;
        }
        28 => {
            c.w64(1, c.r64(0));
            blk = 31; continue;
        }
        29 => {
            c.w64(1, c.r64(0));
            blk = 30; continue;
        }
        30 => {
            c.pcmpeqd_rr(0, 0);
            c.psllq(0, 0x36);
            c.psrlq(0, 0x2);
            c.arith_sd(0, 0, c.xmm_u64(2));
            c.mov128_rr(2, 0);
            blk = 31; continue;
        }
        31 => {
            let cv = c.cvttsd2si64(c.xmm_u64(2)); c.w64(2, cv);
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_o() { blk = 44; continue; }
            blk = 32; continue;
        }
        32 => {
            c.w32(2, c.r32(2));
            c.w64(7, c.r64(2));
            c.w64(0, c.r64(1));
            blk = 40; continue;
        }
        33 => {
            c.w32(1, 0xffu64);
            c.w64(7, c.r64(1));
            blk = 40; continue;
        }
        34 => {
            c.w32(7, 0);
            c.logic(0, 32);
            blk = 40; continue;
        }
        35 => {
            let r16 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r16);
            c.cmp(c.r32(2), 0xffu64, 32);
            if c.cc_be() { blk = 39; continue; }
            blk = 36; continue;
        }
        36 => {
            c.cmp(c.r32(2), 0x0u64, 32);
            if c.cc_l() { blk = 38; continue; }
            blk = 37; continue;
        }
        37 => {
            c.w32(2, 0xffu64);
            blk = 39; continue;
        }
        38 => {
            c.w32(2, 0);
            c.logic(0, 32);
            blk = 39; continue;
        }
        39 => {
            c.w64(7, c.r64(2));
            blk = 40; continue;
        }
        40 => {
            c.w64(2, c.r64(0));
            let Some(t17) = c.ld32(c.r64(2).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008effcc); };
            c.w32(1, t17);
            let r18 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r18);
            let Some(t19) = c.ld8(c.r64(1).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008effd2); };
            c.logic(t19 & 0x4u64, 8);
            if c.cc_ne() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t20) = c.ld64(c.r64(2).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008effd8); };
            c.w64(1, t20);
            let Some(t21) = c.ld32(c.r64(2).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008effdc); };
            c.w32(2, t21);
            let r22 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r22);
            if !c.st8(c.r64(3).wrapping_add(c.r64(2)), c.r8(7)) { return RegfileExit::fault(0x70008effe2); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 42; continue;
        }
        42 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008effeb); }
            return c.ret(0x0, 0x70008effec);
        }
        43 => {
            c.w32(0, 0x2u64);
            blk = 42; continue;
        }
        44 => {
            let r23 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r23);
            if !c.movsd_mr(c.r64(4), 2) { return RegfileExit::fault(0x70008efff8); }
            return c.call(0x70008708b4, 0x70008f0002, 0x70008efffd);
        }
        45 => {
            let Some(t24) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008f0002); };
            c.w32(2, t24);
            let r25 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r25);
            blk = 32; continue;
        }
        46 => {
            return RegfileExit::jump(0x70008f000c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0014); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0018); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f001e); };
            c.w64(3, t3);
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f0028); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f002f); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x10));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f0036); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f003d); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 7; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008f0042); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f0049); };
            c.w64(6, t8);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008f0050); }
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008f0053); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(3)) { return RegfileExit::fault(0x70008f005a); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008f0061);
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f0062); };
            c.w64(3, t9);
            let Some(t10) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f0069); };
            c.w64(1, t10);
            c.w64(7, c.r64(1).wrapping_add(0x8));
            let Some(t11) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f0070); };
            c.w64(6, t11);
            let Some(t12) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008f0077); };
            c.cmp(t12, c.r64(7), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 5; continue;
        }
        5 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008f007c); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 6; continue;
        }
        6 => {
            let Some(t13) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f0083); };
            c.w64(1, t13);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008f008a); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008f008d); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008f0097);
        }
        7 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0098); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f009c); }
            let r14 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r14);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f00a7); }
            return c.call(0x70007febb4, 0x70008f00b0, 0x70008f00ab);
        }
        8 => {
            c.w64(1, c.r64(0));
            let Some(t15) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f00b3); };
            c.w64(3, t15);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f00ba); }
            blk = 3; continue;
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f00bd); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f00c1); }
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008f00d1, 0x70008f00cc);
        }
        10 => {
            c.w64(3, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f00d7); }
            blk = 6; continue;
        }
        11 => {
            return RegfileExit::jump(0x70008f00dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f00f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f00f4); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f00f8); };
            c.w32(6, t1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f00fb); };
            c.w32(7, t2);
            c.logic(c.r32(7) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f0106); };
            c.w64(1, t3);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r4 = c.neg(c.r64(1), 64);
            c.w64(1, r4);
            c.w64(0, c.r64(1));
            blk = 5; continue;
        }
        3 => {
            c.w64(0, c.r64(1));
            blk = 5; continue;
        }
        4 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            let r5 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r5);
            if !c.st64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70008f0122); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008f0128);
        }
        6 => {
            return RegfileExit::jump(0x70008f012c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0134(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0134); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0138); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f013e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f0142); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f014f); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f0155); };
            c.cmp(t3, 0x41u64, 16);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f015c); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008f015d); };
            c.w64(2, t4);
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008f0174, 0x70008f016f);
        }
        3 => {
            c.w64(2, c.r64(0));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f0177); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f017b); };
            c.w64(0, t6);
            blk = 5; continue;
        }
        4 => {
            c.w64(2, c.r64(1));
            blk = 5; continue;
        }
        5 => {
            c.w64(1, c.r64(0));
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f0187); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008f018d); };
            c.logic(t9 & 0x4u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0193); };
            c.w64(7, t10);
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0197); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f019a); };
            c.w32(8, t12);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f01a7); };
            c.w64(2, t13);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let r14 = c.neg(c.r64(2), 64);
            c.w64(2, r14);
            c.w64(0, c.r64(2));
            blk = 11; continue;
        }
        9 => {
            c.w64(0, c.r64(2));
            blk = 11; continue;
        }
        10 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(2, c.r64(7).wrapping_add(c.r64(1)));
            c.w64(1, c.r64(3));
            if !c.st64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70008f01c7); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        12 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f01d0); }
            return c.ret(0x0, 0x70008f01d1);
        }
        13 => {
            c.w32(0, 0x2u64);
            blk = 12; continue;
        }
        14 => {
            c.w32(2, 0x28u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f01de); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f01df); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008f01e0); };
            c.w64(3, t15);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008f01f1, 0x70008f01ec);
        }
        15 => {
            return RegfileExit::fault(0x70008f01f1);
        }
        16 => {
            return RegfileExit::jump(0x70008f01f4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0214); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0218); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f021e); };
            c.w64(3, t3);
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_e() { blk = 6; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f0228); };
            c.w64(1, t4);
            let Some(t5) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f022f); };
            c.w64(6, t5);
            c.w64(7, c.r64(6).wrapping_add(0x10));
            let Some(t6) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f0236); };
            c.w64(8, t6);
            let Some(t7) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f023d); };
            c.cmp(t7, c.r64(7), 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008f0242); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f0249); };
            c.w64(6, t8);
            if !c.st32(c.r64(1).wrapping_sub(0x1), c.r32(6)) { return RegfileExit::fault(0x70008f0250); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_g() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008f0259); }
            let r9 = c.neg(c.r64(3), 64);
            c.w64(3, r9);
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(3)) { return RegfileExit::fault(0x70008f0263); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008f026a);
        }
        5 => {
            if !c.st32(c.r64(1).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008f026b); }
            if !c.st64(c.r64(1).wrapping_add(0x7), c.r64(3)) { return RegfileExit::fault(0x70008f0272); }
            c.w64(0, c.r64(1));
            return c.ret(0x0, 0x70008f0279);
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f027a); };
            c.w64(3, t10);
            let Some(t11) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f0281); };
            c.w64(1, t11);
            c.w64(7, c.r64(1).wrapping_add(0x8));
            let Some(t12) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f0288); };
            c.w64(6, t12);
            let Some(t13) = c.ld64(c.r64(6)) else { return RegfileExit::fault(0x70008f028f); };
            c.cmp(t13, c.r64(7), 64);
            if c.cc_be() { blk = 11; continue; }
            blk = 7; continue;
        }
        7 => {
            if !c.st64(c.r64(3), c.r64(7)) { return RegfileExit::fault(0x70008f0294); }
            c.w64(3, c.r64(1).wrapping_add(0x1));
            blk = 8; continue;
        }
        8 => {
            let Some(t14) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f029b); };
            c.w64(1, t14);
            if !c.st32(c.r64(3).wrapping_sub(0x1), c.r32(1)) { return RegfileExit::fault(0x70008f02a2); }
            if !c.st32(c.r64(3).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008f02a5); }
            c.w64(0, c.r64(3));
            return c.ret(0x0, 0x70008f02af);
        }
        9 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f02b0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f02b4); }
            let r15 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r15);
            c.w32(2, 0x10u64);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f02bf); }
            return c.call(0x70007febb4, 0x70008f02c8, 0x70008f02c3);
        }
        10 => {
            c.w64(1, c.r64(0));
            let Some(t16) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f02cb); };
            c.w64(3, t16);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f02d2); }
            blk = 3; continue;
        }
        11 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f02d8); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f02dc); }
            let r17 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r17);
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008f02ec, 0x70008f02e7);
        }
        12 => {
            c.w64(3, c.r64(0));
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f02f2); }
            blk = 8; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008f02f8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0314(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0314); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0318); };
            c.w32(6, t1);
            let Some(t2) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f031b); };
            c.w32(7, t2);
            c.logic(c.r32(7) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f0326); };
            c.w64(1, t3);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let r4 = c.neg(c.r64(1), 64);
            c.w64(1, r4);
            c.w64(0, c.r64(1));
            blk = 5; continue;
        }
        3 => {
            c.w64(0, c.r64(1));
            blk = 5; continue;
        }
        4 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 5; continue;
        }
        5 => {
            let r5 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r5);
            if !c.st64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70008f0342); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008f0348);
        }
        6 => {
            return RegfileExit::jump(0x70008f034c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0354(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0354); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0358); }
            let r0 = c.sub(c.r64(4), 0x10u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f035e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f0362); }
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 14; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f036f); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f0375); };
            c.cmp(t3, 0x41u64, 16);
            if c.cc_e() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f037c); }
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0x1d70)) else { return RegfileExit::fault(0x70008f037d); };
            c.w64(2, t4);
            c.w32(0, 0x1u64);
            c.w64(3, c.r64(2));
            c.w64(1, c.r64(0));
            return c.call(0x700086ed74, 0x70008f0394, 0x70008f038f);
        }
        3 => {
            c.w64(2, c.r64(0));
            let Some(t5) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f0397); };
            c.w64(3, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f039b); };
            c.w64(0, t6);
            blk = 5; continue;
        }
        4 => {
            c.w64(2, c.r64(1));
            blk = 5; continue;
        }
        5 => {
            c.w64(1, c.r64(0));
            let Some(t7) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f03a7); };
            c.w32(7, t7);
            let r8 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r8);
            let Some(t9) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008f03ad); };
            c.logic(t9 & 0x4u64, 8);
            if c.cc_ne() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t10) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f03b3); };
            c.w64(7, t10);
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f03b7); };
            c.w32(1, t11);
            let Some(t12) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f03ba); };
            c.w32(8, t12);
            c.logic(c.r32(8) & 0x7ffffffeu64, 32);
            if c.cc_e() { blk = 10; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t13) = c.ld64(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f03c7); };
            c.w64(2, t13);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let r14 = c.neg(c.r64(2), 64);
            c.w64(2, r14);
            c.w64(0, c.r64(2));
            blk = 11; continue;
        }
        9 => {
            c.w64(0, c.r64(2));
            blk = 11; continue;
        }
        10 => {
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(2, c.r64(7).wrapping_add(c.r64(1)));
            c.w64(1, c.r64(3));
            if !c.st64(c.r64(2).wrapping_add(c.r64(1).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70008f03e7); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 12; continue;
        }
        12 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f03f0); }
            return c.ret(0x0, 0x70008f03f1);
        }
        13 => {
            c.w32(0, 0x2u64);
            blk = 12; continue;
        }
        14 => {
            c.w32(2, 0x28u64);
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f03fe); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f03ff); }
            let Some(t15) = c.ld64(c.r64(13).wrapping_add(0x2250)) else { return RegfileExit::fault(0x70008f0400); };
            c.w64(3, t15);
            c.w32(0, 0x2u64);
            return c.call(0x700086ed74, 0x70008f0411, 0x70008f040c);
        }
        15 => {
            return RegfileExit::fault(0x70008f0411);
        }
        16 => {
            return RegfileExit::jump(0x70008f0414);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0434); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0438); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld8(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f043e); };
            c.w32(3, t3);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008f0445);
        }
        1 => {
            return RegfileExit::jump(0x70008f0448);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0454(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0454); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0458); };
            c.w32(6, t1);
            let r2 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r2);
            let r3 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r3);
            if !c.st8(c.r64(3).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008f0460); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008f0465);
        }
        1 => {
            return RegfileExit::jump(0x70008f0468);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0474(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0474); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0478); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008f047e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f0482); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f0486); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f048a); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008f049b, 0x70008f0496);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f049e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f04a2); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f04a6); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f04aa); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f04b3); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f04b9); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f04c2); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f04c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_o() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, c.r32(2));
            blk = 9; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            blk = 9; continue;
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f04e2); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008f04e8); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f04ee); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f04f2); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r15);
            if !c.st8(c.r64(3).wrapping_add(c.r64(1)), c.r8(2)) { return RegfileExit::fault(0x70008f04f8); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f0500); }
            return c.ret(0x0, 0x70008f0501);
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        13 => {
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008f050d); }
            return c.call(0x70008708b4, 0x70008f0517, 0x70008f0512);
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008f0517); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r18);
            blk = 7; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008f0524);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0534(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0534); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0538); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld8(c.r64(3).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f053e); };
            c.w32(3, (t3 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008f0545);
        }
        1 => {
            return RegfileExit::jump(0x70008f0548);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0554); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0558); };
            c.w32(6, t1);
            let r2 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r2);
            let r3 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r3);
            if !c.st8(c.r64(3).wrapping_add(c.r64(2)), c.r8(1)) { return RegfileExit::fault(0x70008f0560); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008f0565);
        }
        1 => {
            return RegfileExit::jump(0x70008f0568);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0574(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0574); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0578); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008f057e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f0582); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f0586); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f058a); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008f059b, 0x70008f0596);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f059e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f05a2); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f05a6); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f05aa); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f05b3); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f05b9); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f05c2); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f05c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_o() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, c.r32(2));
            blk = 9; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            blk = 9; continue;
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f05e2); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008f05e8); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f05ee); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f05f2); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r15);
            if !c.st8(c.r64(3).wrapping_add(c.r64(1)), c.r8(2)) { return RegfileExit::fault(0x70008f05f8); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f0600); }
            return c.ret(0x0, 0x70008f0601);
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        13 => {
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008f060d); }
            return c.call(0x70008708b4, 0x70008f0617, 0x70008f0612);
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008f0617); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r18);
            blk = 7; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008f0624);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0634(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0634); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0638); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2))) else { return RegfileExit::fault(0x70008f063e); };
            c.w32(3, t3);
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008f0645);
        }
        1 => {
            return RegfileExit::jump(0x70008f0648);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0654); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0658); };
            c.w32(6, t1);
            let r2 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r2);
            let r3 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r3);
            if !c.st16(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2)), c.r16(1)) { return RegfileExit::fault(0x70008f0660); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008f0666);
        }
        1 => {
            return RegfileExit::jump(0x70008f0668);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0674(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0674); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0678); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008f067e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f0682); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f0686); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f068a); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008f069b, 0x70008f0696);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f069e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f06a2); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f06a6); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f06aa); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f06b3); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f06b9); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f06c2); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f06c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_o() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, c.r32(2));
            blk = 9; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            blk = 9; continue;
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f06e2); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008f06e8); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f06ee); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f06f2); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r15);
            if !c.st16(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(2)), c.r16(2)) { return RegfileExit::fault(0x70008f06f8); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f0701); }
            return c.ret(0x0, 0x70008f0702);
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        13 => {
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008f070e); }
            return c.call(0x70008708b4, 0x70008f0718, 0x70008f0713);
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008f0718); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r18);
            blk = 7; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008f0724);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0734(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0734); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0738); };
            c.w32(1, t1);
            let r2 = c.add(c.r64(2), c.r64(1), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld16(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2))) else { return RegfileExit::fault(0x70008f073e); };
            c.w32(3, (t3 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008f0745);
        }
        1 => {
            return RegfileExit::jump(0x70008f0748);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0754(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0754); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0758); };
            c.w32(6, t1);
            let r2 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r2);
            let r3 = c.add(c.r64(2), c.r64(6), 64);
            c.w64(2, r3);
            if !c.st16(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(2)), c.r16(1)) { return RegfileExit::fault(0x70008f0760); }
            c.w32(0, 0);
            c.logic(0, 32);
            return c.ret(0x0, 0x70008f0766);
        }
        1 => {
            return RegfileExit::jump(0x70008f0768);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0774(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0774); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0778); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008f077e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f0782); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f0786); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f078a); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008f079b, 0x70008f0796);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f079e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f07a2); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f07a6); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f07aa); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f07b3); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f07b9); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f07c2); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f07c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_o() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, c.r32(2));
            blk = 9; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            blk = 9; continue;
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f07e2); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008f07e8); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f07ee); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f07f2); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r15);
            if !c.st16(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(2)), c.r16(2)) { return RegfileExit::fault(0x70008f07f8); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f0801); }
            return c.ret(0x0, 0x70008f0802);
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        13 => {
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008f080e); }
            return c.call(0x70008708b4, 0x70008f0818, 0x70008f0813);
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008f0818); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r18);
            blk = 7; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008f0824);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0834(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f0834); };
            c.w64(1, t0);
            let Some(t1) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f0838); };
            c.w32(6, t1);
            let r2 = c.add(c.r64(1), c.r64(6), 64);
            c.w64(1, r2);
            let Some(t3) = c.ld32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f083e); };
            c.w32(3, t3);
            c.cmp(c.r32(3), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            return c.ret(0x0, 0x70008f084c);
        }
        2 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f084d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0851); }
            let r4 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r4);
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f0857); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f085e); };
            c.w64(6, t6);
            c.w64(7, c.r64(6).wrapping_add(0xc));
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f0865); }
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f0869); };
            c.w64(8, t7);
            let Some(t8) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f0870); };
            c.cmp(t8, c.r64(7), 64);
            if c.cc_a() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f087f, 0x70008f087a);
        }
        4 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f087f); };
            c.w64(3, t9);
            blk = 6; continue;
        }
        5 => {
            if !c.st64(c.r64(1), c.r64(7)) { return RegfileExit::fault(0x70008f0885); }
            c.w64(1, c.r64(6).wrapping_add(0x1));
            c.w64(0, c.r64(1));
            blk = 6; continue;
        }
        6 => {
            c.w32(10, c.r32(3));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t10) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f089b); };
            c.w64(3, t10);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008f08a2); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f08a5); }
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f08ad); }
            return c.ret(0x0, 0x70008f08ae);
        }
        7 => {
            return RegfileExit::jump(0x70008f08b0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f08b4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f08b4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f08b8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(0).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f08be); };
            c.w64(2, t1);
            let Some(t2) = c.ld32(c.r64(0).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f08c2); };
            c.w32(7, t2);
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f08c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f08c9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(2)) { return RegfileExit::fault(0x70008f08cd); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(7)) { return RegfileExit::fault(0x70008f08d1); }
            blk = 2; continue;
        }
        1 => {
            c.w64(1, c.r64(0));
            blk = 2; continue;
        }
        2 => {
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t3) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f08df); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f08e6); };
            c.cmp(t5, c.r32(8), 32);
            if c.cc_e() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t6) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f08ef); };
            c.cmp(t6, 0x43u64, 16);
            if c.cc_e() { blk = 7; continue; }
            blk = 5; continue;
        }
        5 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008f08ff, 0x70008f08fa);
        }
        6 => {
            let Some(t7) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f08ff); };
            c.w64(6, t7);
            let Some(t8) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f0903); };
            c.w64(3, t8);
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0907); };
            c.w64(2, t9);
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f090b); };
            c.w64(7, t10);
            blk = 1; continue;
        }
        7 => {
            let Some(t11) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f0911); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            c.w64(0, c.r64(1));
            blk = 1; continue;
        }
        8 => {
            let r13 = c.sar(c.r32(1), 1u64, 32).unwrap_or(c.r32(1));
            c.w32(1, r13);
            blk = 11; continue;
        }
        9 => {
            if !c.movsd_rm(0, c.r64(1).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f0920); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(1, cv);
            c.cmp(c.r64(1), 0x1u64, 64);
            if c.cc_o() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            c.w32(1, c.r32(1));
            blk = 11; continue;
        }
        11 => {
            let r14 = c.add(c.r64(2), c.r64(7), 64);
            c.w64(2, r14);
            if !c.st32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)), c.r32(1)) { return RegfileExit::fault(0x70008f0935); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f093d); }
            return c.ret(0x0, 0x70008f093e);
        }
        12 => {
            let r15 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r15);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008f0943); }
            return c.call(0x70008708b4, 0x70008f094d, 0x70008f0948);
        }
        13 => {
            let Some(t16) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008f094d); };
            c.w32(1, t16);
            let r17 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r17);
            blk = 10; continue;
        }
        14 => {
            return RegfileExit::jump(0x70008f0958);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0974(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0974); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0978); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(1)) { return RegfileExit::fault(0x70008f097e); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f0982); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f0986); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(3)) { return RegfileExit::fault(0x70008f098a); }
            c.w64(2, c.r64(1));
            blk = 3; continue;
        }
        1 => {
            c.w64(0, c.r64(1));
            return c.call(0x70008a1014, 0x70008f099b, 0x70008f0996);
        }
        2 => {
            c.w64(2, c.r64(0));
            let Some(t1) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f099e); };
            c.w64(3, t1);
            let Some(t2) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f09a2); };
            c.w64(0, t2);
            let Some(t3) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f09a6); };
            c.w64(1, t3);
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f09aa); };
            c.w64(6, t4);
            blk = 3; continue;
        }
        3 => {
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_e() { blk = 8; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t5) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f09b3); };
            c.w32(7, t5);
            let r6 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f09b9); };
            c.cmp(t7, c.r32(7), 32);
            if c.cc_e() { blk = 6; continue; }
            blk = 5; continue;
        }
        5 => {
            let Some(t8) = c.ld16(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f09c2); };
            c.cmp(t8, 0x43u64, 16);
            if c.cc_ne() { blk = 1; continue; }
            blk = 6; continue;
        }
        6 => {
            if !c.movsd_rm(0, c.r64(2).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f09c9); }
            let cv = c.cvttsd2si64(c.xmm_u64(0)); c.w64(2, cv);
            c.cmp(c.r64(2), 0x1u64, 64);
            if c.cc_o() { blk = 13; continue; }
            blk = 7; continue;
        }
        7 => {
            c.w32(2, c.r32(2));
            blk = 9; continue;
        }
        8 => {
            let r9 = c.sar(c.r32(2), 1u64, 32).unwrap_or(c.r32(2));
            c.w32(2, r9);
            blk = 9; continue;
        }
        9 => {
            c.w64(1, c.r64(0));
            let Some(t10) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f09e2); };
            c.w32(7, t10);
            let r11 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r11);
            let Some(t12) = c.ld8(c.r64(7).wrapping_add(0x2b)) else { return RegfileExit::fault(0x70008f09e8); };
            c.logic(t12 & 0x4u64, 8);
            if c.cc_ne() { blk = 12; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t13) = c.ld64(c.r64(1).wrapping_add(0x27)) else { return RegfileExit::fault(0x70008f09ee); };
            c.w64(7, t13);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0x2f)) else { return RegfileExit::fault(0x70008f09f2); };
            c.w32(1, t14);
            let r15 = c.add(c.r64(1), c.r64(7), 64);
            c.w64(1, r15);
            if !c.st32(c.r64(1).wrapping_add(c.r64(3).wrapping_mul(4)), c.r32(2)) { return RegfileExit::fault(0x70008f09f8); }
            c.w32(0, 0);
            c.logic(0, 32);
            blk = 11; continue;
        }
        11 => {
            c.w64(4, c.r64(5));
            if !c.pop64_reg(5) { return RegfileExit::fault(0x70008f0a00); }
            return c.ret(0x0, 0x70008f0a01);
        }
        12 => {
            c.w32(0, 0x2u64);
            blk = 11; continue;
        }
        13 => {
            let r16 = c.sub(c.r64(4), 0x8u64, 64);
            c.w64(4, r16);
            if !c.movsd_mr(c.r64(4), 0) { return RegfileExit::fault(0x70008f0a0d); }
            return c.call(0x70008708b4, 0x70008f0a17, 0x70008f0a12);
        }
        14 => {
            let Some(t17) = c.ld32(c.r64(4)) else { return RegfileExit::fault(0x70008f0a17); };
            c.w32(2, t17);
            let r18 = c.add(c.r64(4), 0x8u64, 64);
            c.w64(4, r18);
            blk = 7; continue;
        }
        15 => {
            return RegfileExit::jump(0x70008f0a24);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0a34(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c30)) else { return RegfileExit::fault(0x70008f0a34); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008f0a54(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c38)) else { return RegfileExit::fault(0x70008f0a54); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008f0a74(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c40)) else { return RegfileExit::fault(0x70008f0a74); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008f0a94(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c48)) else { return RegfileExit::fault(0x70008f0a94); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008f0ab4(raw: *mut u8, _blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    let Some(t0) = c.ld64(c.r64(13).wrapping_add(0x1c50)) else { return RegfileExit::fault(0x70008f0ab4); };
    c.w64(3, t0);
    return RegfileExit::jump(0x70007f1b34);
}

pub unsafe extern "C" fn rf_70008f0ad4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f0ad8); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), 0x100u64, 64);
            c.w64(3, r2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f0ae4); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f0aec);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0af4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f0af8); };
            c.w32(3, t1);
            let r2 = c.add(c.r64(3), 0x200u64, 64);
            c.w64(3, r2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f0b04); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f0b0c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0b14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0b14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0b18); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0b1e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0b22); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0b2a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0b2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0b2f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0b33); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0b37); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0b3b); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0b4f, 0x70008f0b4a);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0b52); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0b56); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0b5a); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0b5e); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0b62); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0b69); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0b78);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0b94(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0b94); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0b98); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0b9e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0ba2); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0baa); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0bae); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0baf); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0bb3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0bb7); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0bbb); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0bcf, 0x70008f0bca);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0bd2); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0bd6); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0bda); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0bde); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0be2); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0be9); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0bf8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0c14(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0c14); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0c18); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0c1e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0c22); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0c2a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0c2e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f0c2f); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f0c33); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f0c37); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0c3b); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0c3f); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0c53, 0x70008f0c4e);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(2));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.w64(8, c.r64(2).wrapping_mul(4));
            c.cmp(c.r32(7), 0xb4u64, 32);
            if c.cc_ae() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f0c6c); };
            c.w64(15, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0c70); };
            c.w64(1, t5);
            let Some(t6) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0c74); };
            c.w64(5, t6);
            let Some(t7) = c.ld64(c.r64(4)) else { return RegfileExit::fault(0x70008f0c78); };
            c.w64(12, t7);
            let Some(t8) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0c7c); };
            c.w64(9, t8);
            let r9 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r9);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            let Some(t10) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0c87); };
            c.w64(2, t10);
            if !c.st64(c.r64(2).wrapping_add(c.r64(8)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f0c8b); }
            let Some(t11) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0c93); };
            c.w64(2, t11);
            let Some(t12) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0c97); };
            c.w64(7, t12);
            c.w64(9, c.r64(7).wrapping_add(0x1));
            let Some(t13) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f0c9f); };
            c.w32(7, t13);
            let Some(t14) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f0ca4); };
            c.w64(15, t14);
            let Some(t15) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f0ca8); };
            c.w64(1, t15);
            let Some(t16) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0cac); };
            c.w64(5, t16);
            c.w64(12, c.r64(2));
            let r17 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r17);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f0cbc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0cd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0cd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0cd8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0cde); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0ce2); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0cea); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0cee); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0cef); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0cf3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0cf7); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0cfb); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0d0f, 0x70008f0d0a);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0d12); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0d16); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0d1a); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0d1e); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0d22); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0d29); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0d38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0d54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0d54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0d58); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0d5e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0d62); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0d6a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0d6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0d6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0d73); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0d77); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0d7b); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0d8f, 0x70008f0d8a);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0d92); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0d96); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0d9a); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0d9e); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0da2); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0da9); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0db8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0dd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0dd4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0dd8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0dde); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0de2); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0dea); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0dee); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0def); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0df3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0df7); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0dfb); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0e0f, 0x70008f0e0a);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0e12); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0e16); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0e1a); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0e1e); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0e22); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0e29); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0e38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0e54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0e54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0e58); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0e5e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0e62); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0e6a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0e6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0e6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0e73); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0e77); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0e7b); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0e8f, 0x70008f0e8a);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0e92); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0e96); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0e9a); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0e9e); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0ea2); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0ea9); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0eb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0ed4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0ed4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0ed8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0ede); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0ee2); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0eea); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0eee); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0eef); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0ef3); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0ef7); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0efb); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0f0f, 0x70008f0f0a);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0f12); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0f16); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0f1a); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0f1e); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0f22); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0f29); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0f38);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0f54(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f0f54); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f0f58); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0f5e); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f0f62); };
            c.w64(7, t2);
            c.w32(8, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(8)) { return RegfileExit::fault(0x70008f0f6a); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f0f6e); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(9)) { return RegfileExit::fault(0x70008f0f6f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f0f73); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f0f77); }
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0x1c58)) else { return RegfileExit::fault(0x70008f0f7b); };
            c.w64(3, t3);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086f394, 0x70008f0f8f, 0x70008f0f8a);
        }
        1 => {
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t4) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0f92); };
            c.w64(7, t4);
            let Some(t5) = c.ld64(c.r64(7).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f0f96); };
            c.w64(7, t5);
            let Some(t6) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f0f9a); };
            c.w64(15, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(4))) else { return RegfileExit::fault(0x70008f0f9e); };
            c.w64(1, t7);
            let Some(t8) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f0fa2); };
            c.w64(5, t8);
            c.w64(12, c.r64(7));
            let Some(t9) = c.ld64(c.r64(4).wrapping_add(0x10)) else { return RegfileExit::fault(0x70008f0fa9); };
            c.w64(9, t9);
            let r10 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r10);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f0fb8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f0fd4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f0fd4); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f0fdd); };
            c.w64(3, t1);
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f0fe5); };
            c.w32(2, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f0fea); };
            c.w64(1, t4);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f0ff4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1014(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1018); };
            c.w32(3, t1);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f102d); };
            c.w64(1, t2);
            c.w32(0, 0);
            c.logic(0, 32);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(3, c.r64(5));
            if !c.st64(c.r64(3).wrapping_add(c.r64(2)).wrapping_sub(0x648), 0x0u64) { return RegfileExit::fault(0x70008f1038); }
            let r3 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1048); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f104d); };
            c.w64(1, t5);
            c.w32(0, 0);
            c.logic(0, 32);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70008f1058);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1074(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1074); };
            c.w32(3, (t0 as u8 as i8 as i64 as u64));
            let r1 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f107e); };
            c.w32(2, t2);
            c.w64(6, c.r64(2).wrapping_mul(8));
            c.w32(0, c.r64(3).wrapping_add(c.r64(3)));
            c.cmp(c.r32(2), 0xb4u64, 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1096); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(3, c.r64(5));
            if !c.st64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f109f); }
            let r4 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r4);
            let Some(t5) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f10ab); };
            c.w32(3, t5);
            let Some(t6) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f10b0); };
            c.w64(1, t6);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70008f10b8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f10d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f10d8); };
            c.w32(3, t1);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f10ed); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f10f1); };
            c.w64(0, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(3, c.r64(5));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f10fd); };
            c.w64(0, t4);
            if !c.st64(c.r64(3).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1104); }
            let r5 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1110); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1115); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70008f111c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1134(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1138); };
            c.w32(3, t1);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f114d); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f1151); };
            c.w64(0, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(3, c.r64(5));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f115d); };
            c.w64(0, t4);
            if !c.st64(c.r64(3).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1164); }
            let r5 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1170); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1175); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70008f117c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1194(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1198); };
            c.w32(3, t1);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f11ad); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f11b1); };
            c.w64(0, t3);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(3, c.r64(5));
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f11bd); };
            c.w64(0, t4);
            if !c.st64(c.r64(3).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f11c4); }
            let r5 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f11d0); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f11d5); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70008f11dc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f11f4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f11f8); };
            c.w32(3, t1);
            let Some(t2) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f11fd); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008f1201); };
            c.w64(0, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f120c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1214(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let r0 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r0);
            let Some(t1) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1218); };
            c.w32(3, t1);
            let Some(t2) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f121d); };
            c.w64(1, t2);
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f1221); };
            c.w64(0, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f122c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1234(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1234); };
            c.w32(3, t0);
            let Some(t1) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f123a); };
            c.w32(2, t1);
            let r2 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r2);
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1242); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f124d); };
            c.w32(3, t6);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1262); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            c.w64(3, c.r64(5));
            if !c.st64(c.r64(3).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f126b); }
            let r8 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1277); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f127c); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        3 => {
            return RegfileExit::jump(0x70008f1284);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1294(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1294); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f129d); };
            c.w64(3, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f12a1); };
            c.w32(6, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f12a7); };
            c.w32(7, t3);
            c.cmp(c.r8(7), 0x0u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f12b3); };
            c.w32(8, t4);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f12c3); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let r7 = c.add(c.r32(7), 0xffffffffffffffffu64, 32);
            c.w32(7, r7);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008f12d0);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f12d1); };
            c.w32(7, t8);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008f12df);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f12e0); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f12eb); };
            c.w32(3, t12);
            c.w64(6, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1300); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            if !c.st64(c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1306); }
            let r14 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r14);
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1312); };
            c.w32(3, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1317); };
            c.w64(1, t16);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            return RegfileExit::jump(0x70008f1320);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1334(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1334); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f133d); };
            c.w64(3, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f1341); };
            c.w32(6, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1347); };
            c.w32(7, t3);
            c.cmp(c.r8(7), 0x0u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1353); };
            c.w32(8, t4);
            c.w64(8, (c.r32(8) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(8), 1u64, 64) { c.w64(8, sr); }
            c.cmp(c.r64(8), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f1363); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let r7 = c.add(c.r32(7), 0xffffffffffffffffu64, 32);
            c.w32(7, r7);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008f1370);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1371); };
            c.w32(7, t8);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(6), c.r64(7), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008f137f);
        }
        7 => {
            let Some(t9) = c.ld32(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1380); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f138b); };
            c.w32(3, t12);
            c.w64(6, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f13a0); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            if !c.st64(c.r64(2).wrapping_add(c.r64(6)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f13a6); }
            let r14 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r14);
            let Some(t15) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f13b2); };
            c.w32(3, t15);
            let Some(t16) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f13b7); };
            c.w64(1, t16);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            return RegfileExit::jump(0x70008f13c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f13d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f13d7); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f13db); };
            c.w32(6, t1);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f13e1); };
            c.w32(7, t2);
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008f13ef);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f13f0); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f13fb); };
            c.w32(2, t6);
            c.w64(6, c.r64(2).wrapping_mul(8));
            c.cmp(c.r32(2), 0xb4u64, 32);
            if c.cc_ae() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1410); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            if !c.st64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1416); }
            let r8 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1422); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1427); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            return RegfileExit::jump(0x70008f1430);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1434(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f1437); };
            c.w64(2, t0);
            let Some(t1) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f143b); };
            c.w32(6, t1);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1441); };
            c.w32(7, t2);
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(7), c.r64(6), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008f144f);
        }
        2 => {
            let Some(t3) = c.ld32(c.r64(2).wrapping_add(c.r64(7).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1450); };
            c.w32(0, t3);
            let r4 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r4);
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f145b); };
            c.w32(2, t6);
            c.w64(6, c.r64(2).wrapping_mul(8));
            c.cmp(c.r32(2), 0xb4u64, 32);
            if c.cc_ae() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1470); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            if !c.st64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1476); }
            let r8 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r8);
            let Some(t9) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1482); };
            c.w32(3, t9);
            let Some(t10) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1487); };
            c.w64(1, t10);
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            return RegfileExit::jump(0x70008f1490);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1494); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            if !c.st64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8)), c.r64(0)) { return RegfileExit::fault(0x70008f149d); }
            let r1 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r1);
            let Some(t2) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f14a5); };
            c.w32(3, t2);
            let Some(t3) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f14aa); };
            c.w64(1, t3);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f14b4);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f14d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f14d4); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f14dd); };
            c.w64(3, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f14e1); };
            c.w64(6, (t2 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(2).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(3)) { return RegfileExit::fault(0x70008f14e7); }
            let r3 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f14ef); };
            c.w32(3, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f14f4); };
            c.w64(1, t5);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f14fc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1514(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f1517); };
            c.w64(2, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f151b); };
            c.w64(6, (t1 as u8 as i8 as i64 as u64));
            if !c.st64(c.r64(3).wrapping_add(c.r64(6).wrapping_mul(8)), c.r64(2)) { return RegfileExit::fault(0x70008f1521); }
            if !c.st64(c.r64(3).wrapping_sub(0x8), c.r64(0)) { return RegfileExit::fault(0x70008f1525); }
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f152d); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1532); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f153c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1554); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f155d); };
            c.w64(3, t1);
            if !c.st64(c.r64(2).wrapping_sub(0x8), c.r64(3)) { return RegfileExit::fault(0x70008f1561); }
            let r2 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r2);
            let Some(t3) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1569); };
            c.w32(3, t3);
            let Some(t4) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f156e); };
            c.w64(1, t4);
            return RegfileExit::jump(c.r64(1));
        }
        1 => {
            return RegfileExit::jump(0x70008f1578);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1594(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1594); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f159d); };
            c.w64(3, t1);
            c.cmp(c.r32(0), c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f15a5); };
            c.w64(3, t2);
            blk = 3; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008f15ae); };
            c.w64(3, t3);
            blk = 3; continue;
        }
        3 => {
            let r4 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r4);
            let Some(t5) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f15b9); };
            c.w32(2, t5);
            let Some(t6) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f15be); };
            c.w64(1, t6);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f15c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f15d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t0) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f15d8); };
            c.w32(3, t0);
            let r1 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r1);
            let Some(t2) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f15de); };
            c.logic(t2 & 0x10u64, 8);
            if c.cc_ne() { blk = 3; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t3) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f15e4); };
            c.w64(3, t3);
            blk = 5; continue;
        }
        3 => {
            let Some(t4) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008f15ed); };
            c.w64(3, t4);
            blk = 5; continue;
        }
        4 => {
            let Some(t5) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f15f6); };
            c.w64(3, t5);
            blk = 5; continue;
        }
        5 => {
            let r6 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r6);
            let Some(t7) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1601); };
            c.w32(2, t7);
            let Some(t8) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1606); };
            c.w64(1, t8);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        6 => {
            return RegfileExit::jump(0x70008f1610);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1614(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f1614); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f161d); };
            c.w64(3, t1);
            blk = 3; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008f1626); };
            c.w64(3, t2);
            blk = 3; continue;
        }
        3 => {
            let r3 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1631); };
            c.w32(2, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1636); };
            c.w64(1, t5);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f1640);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f1654); };
            c.cmp(t0, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t1) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f165d); };
            c.w64(3, t1);
            blk = 3; continue;
        }
        2 => {
            let Some(t2) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008f1666); };
            c.w64(3, t2);
            blk = 3; continue;
        }
        3 => {
            let r3 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r3);
            let Some(t4) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1671); };
            c.w32(2, t4);
            let Some(t5) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1676); };
            c.w64(1, t5);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f1680);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1694(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1694); };
            c.w32(3, t0);
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_l() { blk = 8; continue; }
            blk = 1; continue;
        }
        1 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_l() { blk = 5; continue; }
            blk = 2; continue;
        }
        2 => {
            c.cmp(c.r32(3), 0x6u64, 32);
            if c.cc_e() { blk = 35; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r32(3), 0x7u64, 32);
            if c.cc_e() { blk = 38; continue; }
            blk = 4; continue;
        }
        4 => {
            blk = 43; continue;
        }
        5 => {
            c.cmp(c.r32(3), 0x4u64, 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 6; continue;
        }
        6 => {
            c.cmp(c.r32(3), 0x5u64, 32);
            if c.cc_e() { blk = 31; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 43; continue;
        }
        8 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_l() { blk = 12; continue; }
            blk = 9; continue;
        }
        9 => {
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 22; continue; }
            blk = 10; continue;
        }
        10 => {
            c.cmp(c.r32(3), 0x3u64, 32);
            if c.cc_e() { blk = 25; continue; }
            blk = 11; continue;
        }
        11 => {
            blk = 43; continue;
        }
        12 => {
            c.cmp(c.r32(3), 0x0u64, 32);
            if c.cc_e() { blk = 16; continue; }
            blk = 13; continue;
        }
        13 => {
            c.cmp(c.r32(3), 0x1u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 14; continue;
        }
        14 => {
            blk = 43; continue;
        }
        15 => {
            blk = 43; continue;
        }
        16 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 42; continue; }
            blk = 17; continue;
        }
        17 => {
            let Some(t1) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1702); };
            c.w32(3, t1);
            let Some(t2) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f1705); };
            c.cmp(t2, c.r32(3), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 18; continue;
        }
        18 => {
            blk = 42; continue;
        }
        19 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f171f); };
            c.w32(3, t3);
            let r4 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r4);
            let Some(t5) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1725); };
            c.cmp(t5, 0x40u64, 16);
            if c.cc_b() { blk = 42; continue; }
            blk = 21; continue;
        }
        21 => {
            blk = 43; continue;
        }
        22 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t6) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f173d); };
            c.w32(3, t6);
            let Some(t7) = c.ld32(c.r64(13).wrapping_add(0xf8)) else { return RegfileExit::fault(0x70008f1740); };
            c.cmp(t7, c.r32(3), 32);
            if c.cc_ne() { blk = 43; continue; }
            blk = 24; continue;
        }
        24 => {
            blk = 42; continue;
        }
        25 => {
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008f1752); };
            c.cmp(t8, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f175f); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 27; continue;
        }
        27 => {
            blk = 43; continue;
        }
        28 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t10) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1779); };
            c.w32(3, t10);
            let r11 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r11);
            let Some(t12) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f177f); };
            c.cmp(t12, 0x41u64, 16);
            if c.cc_e() { blk = 42; continue; }
            blk = 30; continue;
        }
        30 => {
            blk = 43; continue;
        }
        31 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 32; continue;
        }
        32 => {
            let Some(t13) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f1797); };
            c.cmp(t13, c.r32(0), 32);
            if c.cc_e() { blk = 43; continue; }
            blk = 33; continue;
        }
        33 => {
            let Some(t14) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f17a0); };
            c.w32(3, t14);
            let r15 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r15);
            let Some(t16) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f17a6); };
            c.logic(t16 & 0x10u64, 8);
            if c.cc_ne() { blk = 42; continue; }
            blk = 34; continue;
        }
        34 => {
            blk = 43; continue;
        }
        35 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 36; continue;
        }
        36 => {
            let Some(t17) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f17b2); };
            c.w32(3, t17);
            let r18 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r18);
            let Some(t19) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f17b8); };
            c.w32(3, t19);
            let r20 = c.r32(3) & 0x12u64;
            c.w32(3, r20);
            c.logic(r20, 32);
            c.cmp(c.r32(3), 0x2u64, 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 37; continue;
        }
        37 => {
            blk = 43; continue;
        }
        38 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 43; continue; }
            blk = 39; continue;
        }
        39 => {
            let Some(t21) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f17ca); };
            c.cmp(t21, c.r32(0), 32);
            if c.cc_e() { blk = 42; continue; }
            blk = 40; continue;
        }
        40 => {
            let Some(t22) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f17d3); };
            c.w32(3, t22);
            let r23 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r23);
            let Some(t24) = c.ld16(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f17d9); };
            c.cmp(t24, 0xbau64, 16);
            if c.cc_b() { blk = 43; continue; }
            blk = 41; continue;
        }
        41 => {
            let Some(t25) = c.ld8(c.r64(3).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f17e1); };
            c.logic(t25 & 0x12u64, 8);
            if c.cc_ne() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t26) = c.ld64(c.r64(13).wrapping_add(0xb8)) else { return RegfileExit::fault(0x70008f17e7); };
            c.w64(3, t26);
            blk = 44; continue;
        }
        43 => {
            let Some(t27) = c.ld64(c.r64(13).wrapping_add(0xc0)) else { return RegfileExit::fault(0x70008f17f0); };
            c.w64(3, t27);
            blk = 44; continue;
        }
        44 => {
            let r28 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r28);
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f17fb); };
            c.w32(2, t29);
            let Some(t30) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1800); };
            c.w64(1, t30);
            c.w64(0, c.r64(3));
            return RegfileExit::jump(c.r64(1));
        }
        45 => {
            return RegfileExit::jump(0x70008f180c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f1814(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1817); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f181b); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1822); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1829); };
            c.w32(0, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f182d); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_ne() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            c.w64(0, c.r64(11));
            blk = 3; continue;
        }
        2 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f183b); };
            c.w64(0, t7);
            blk = 3; continue;
        }
        3 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f1842); };
            c.w64(11, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f1849); };
            c.cmp(t9, c.r32(0), 32);
            if c.cc_e() { blk = 308; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f1856); };
            c.w32(3, t10);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(2, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t11) = c.ld32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f1863); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 10; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 19; continue; }
            blk = 6; continue;
        }
        6 => {
            let r13 = c.r64(1) & 0xfffffffffffffffdu64;
            c.w64(1, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f187c); };
            c.w32(11, t14);
            let r15 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r15);
            c.w64(2, c.r64(9).wrapping_add(0x3));
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f1887); };
            c.cmp(t16, c.r32(11), 32);
            if c.cc_e() { blk = 18; continue; }
            blk = 7; continue;
        }
        7 => {
            let Some(t17) = c.ld8(c.r64(2).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1894); };
            c.w32(0, t17);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            let Some(t18) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f18a8); };
            c.w64(1, t18);
            c.w64(9, c.r64(2));
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        9 => {
            c.w64(9, c.r64(2).wrapping_add(0x1));
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f18b8); }
            let Some(t19) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f18c0); };
            c.w32(8, t19);
            let Some(t20) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f18c5); };
            c.w64(1, t20);
            c.w64(0, c.r64(11));
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            let Some(t21) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f18ce); };
            c.w64(11, t21);
            let Some(t22) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f18d2); };
            c.w32(11, t22);
            let r23 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r23);
            let Some(t24) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f18d9); };
            c.w32(11, t24);
            let r25 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r25);
            let Some(t26) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f18e0); };
            c.w32(0, t26);
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(0), 0xa7u64, 64);
            if c.cc_a() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008f18f2);
        }
        12 => {
            let Some(t27) = c.ld32(c.r64(11).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x70008f18f3); };
            c.w32(11, t27);
            let r28 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r28);
            c.w64(0, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t29) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1900); };
            c.w32(3, t29);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            c.w64(2, c.r64(0));
            let r30 = c.r64(2) & 0xfffu64;
            c.w64(2, r30);
            c.logic(r30, 64);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(1, c.r64(2).wrapping_add(0x1));
            c.cmp(c.r64(1), c.r64(3), 64);
            if c.cc_ae() { blk = 307; continue; }
            blk = 13; continue;
        }
        13 => {
            let Some(t31) = c.ld32(c.r64(11).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f1924); };
            c.w32(11, t31);
            let r32 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r32);
            let Some(t33) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f192c); };
            c.w32(3, t33);
            if let Some(sr) = c.shr(c.r64(0), 0xcu64, 64) { c.w64(0, sr); }
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            let r34 = c.r64(0) & 0x3ffffu64;
            c.w64(0, r34);
            c.logic(r34, 64);
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(0), c.r64(3), 64);
            if c.cc_b() { blk = 15; continue; }
            blk = 14; continue;
        }
        14 => {
            return RegfileExit::fault(0x70008f1945);
        }
        15 => {
            let r35 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r35);
            let Some(t36) = c.ld32(c.r64(11).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f194a); };
            c.w32(0, t36);
            let r37 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r37);
            let Some(t38) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1952); };
            c.w32(11, t38);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            let Some(t39) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1968); };
            c.w64(1, t39);
            return RegfileExit::jump(c.r64(1));
        }
        17 => {
            let r40 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r40);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1972); }
            let Some(t41) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f197a); };
            c.w32(8, t41);
            let Some(t42) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f197f); };
            c.w64(1, t42);
            return RegfileExit::jump(c.r64(1));
        }
        18 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f1985); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f1989); }
            let r43 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r43);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f198f); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f1993); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008f1997); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008f199b); }
            blk = 303; continue;
        }
        19 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f19a4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f19a8); }
            let r44 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r44);
            let Some(t45) = c.ld32(c.r64(0).wrapping_add(c.r64(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008f19ae); };
            c.w32(7, t45);
            let r46 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r46);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f19b5); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f19b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f19bd); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(9)) { return RegfileExit::fault(0x70008f19c1); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008f19c5); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008f19c9); }
            let Some(t47) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008f19cd); };
            c.cmp(t47, c.r32(7), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t48) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f19da); };
            c.w64(2, t48);
            let Some(t49) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f19de); };
            c.w32(1, t49);
            let r50 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r50);
            let Some(t51) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f19e4); };
            c.w32(1, t51);
            let r52 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r52);
            let Some(t53) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f19ea); };
            c.w32(6, t53);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(2)) { return RegfileExit::fault(0x70008f19f3); }
            c.cmp(c.r64(6), 0x3u64, 64);
            if c.cc_a() { blk = 22; continue; }
            blk = 21; continue;
        }
        21 => {
            return RegfileExit::fault(0x70008f19fd);
        }
        22 => {
            let Some(t54) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f19fe); };
            c.w32(0, t54);
            let r55 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r55);
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(0)) { return RegfileExit::fault(0x70008f1a04); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 24; continue; }
            blk = 23; continue;
        }
        23 => {
            return RegfileExit::fault(0x70008f1a0e);
        }
        24 => {
            let Some(t56) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f1a0f); };
            c.w32(1, t56);
            let r57 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r57);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 44; continue; }
            blk = 25; continue;
        }
        25 => {
            let Some(t58) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1a1f); };
            c.w32(6, t58);
            let Some(t59) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70008f1a22); };
            c.cmp(t59, c.r32(6), 32);
            if c.cc_e() { blk = 37; continue; }
            blk = 26; continue;
        }
        26 => {
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1a2f); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            c.w32(3, 0);
            c.logic(0, 32);
            c.cmp(c.r32(6), c.r32(3), 32);
            if c.cc_e() { blk = 28; continue; }
            blk = 27; continue;
        }
        27 => {
            let Some(t62) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1a3b); };
            c.cmp(t62, c.r32(3), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 28; continue;
        }
        28 => {
            let Some(t63) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1a44); };
            c.w32(3, t63);
            let Some(t64) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f1a47); };
            c.w32(6, t64);
            let r65 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r65);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 34; continue; }
            blk = 29; continue;
        }
        29 => {
            let Some(t66) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f1a53); };
            c.cmp(t66, c.r32(6), 32);
            if c.cc_e() { blk = 32; continue; }
            blk = 30; continue;
        }
        30 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 31; continue;
        }
        31 => {
            let r67 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r67);
            c.logic(r67, 64);
            blk = 33; continue;
        }
        32 => {
            c.w64(6, c.r64(1));
            blk = 33; continue;
        }
        33 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(6));
            c.w64(6, c.r64(10));
            blk = 45; continue;
        }
        34 => {
            let r68 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r68);
            let Some(t69) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1a80); };
            c.w32(11, t69);
            c.w64(0, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 36; continue; }
            blk = 35; continue;
        }
        35 => {
            let Some(t70) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1a96); };
            c.w64(1, t70);
            let Some(t71) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1a9a); };
            c.w64(5, t71);
            c.w64(0, c.r64(6));
            let r72 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r72);
            return RegfileExit::jump(c.r64(1));
        }
        36 => {
            let r73 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r73);
            if !c.st64(c.r64(8).wrapping_add(c.r64(0)).wrapping_sub(0x648), c.r64(6)) { return RegfileExit::fault(0x70008f1aab); }
            let Some(t74) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1ab3); };
            c.w32(8, t74);
            let Some(t75) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1ab8); };
            c.w64(1, t75);
            let Some(t76) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1abc); };
            c.w64(5, t76);
            c.w64(0, c.r64(6));
            let r77 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r77);
            return RegfileExit::jump(c.r64(1));
        }
        37 => {
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t78) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1ad1); };
            c.w32(6, t78);
            let Some(t79) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1ad7); };
            c.w32(9, t79);
            let r80 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r80);
            let Some(t81) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1adf); };
            c.w32(6, t81);
            let r82 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r82);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f1ae7); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f1aeb); }
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(0, c.r64(3));
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f1b04); };
            c.w64(3, t83);
            let Some(t84) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f1b08); };
            c.logic(t84 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 39; continue; }
            blk = 38; continue;
        }
        38 => {
            let r85 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r85);
            blk = 40; continue;
        }
        39 => {
            let Some(t86) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f1b17); };
            c.w32(7, t86);
            let Some(t87) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008f1b1a); };
            c.w64(7, t87);
            blk = 40; continue;
        }
        40 => {
            return c.call(c.r64(7), 0x70008f1b24, 0x70008f1b22);
        }
        41 => {
            let Some(t88) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1b24); };
            c.w64(8, t88);
            let Some(t89) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1b28); };
            c.w64(11, t89);
            let Some(t90) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f1b2c); };
            c.w64(9, t90);
            let Some(t91) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1b30); };
            c.w32(3, t91);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 43; continue; }
            blk = 42; continue;
        }
        42 => {
            let Some(t92) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1b45); };
            c.w64(15, t92);
            let Some(t93) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1b49); };
            c.w64(1, t93);
            let Some(t94) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1b4d); };
            c.w64(5, t94);
            c.w64(12, c.r64(11));
            let r95 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r95);
            return RegfileExit::jump(c.r64(1));
        }
        43 => {
            let r96 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r96);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1b5e); }
            let Some(t97) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1b66); };
            c.w32(8, t97);
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1b6b); };
            c.w64(15, t98);
            let Some(t99) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1b6f); };
            c.w64(1, t99);
            let Some(t100) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1b73); };
            c.w64(5, t100);
            c.w64(12, c.r64(11));
            let r101 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r101);
            return RegfileExit::jump(c.r64(1));
        }
        44 => {
            c.w64(3, c.r64(1));
            c.w64(6, c.r64(7));
            blk = 45; continue;
        }
        45 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(2, c.r64(6));
            let r102 = c.r64(2) & 0xfu64;
            c.w64(2, r102);
            c.logic(r102, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(3)) { return RegfileExit::fault(0x70008f1b93); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_e() { blk = 215; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r64(2), 0x5u64, 64);
            if c.cc_e() { blk = 212; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0xdu64, 64);
            if c.cc_e() { blk = 208; continue; }
            blk = 48; continue;
        }
        48 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_e() { blk = 145; continue; }
            blk = 49; continue;
        }
        49 => {
            c.cmp(c.r64(2), 0x6u64, 64);
            if c.cc_e() { blk = 141; continue; }
            blk = 50; continue;
        }
        50 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 132; continue; }
            blk = 51; continue;
        }
        51 => {
            c.cmp(c.r64(2), 0x8u64, 64);
            c.w8(0, (c.cc_e() as u64));
            c.w32(0, c.r8(0));
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 119; continue; }
            blk = 52; continue;
        }
        52 => {
            c.cmp(c.r64(2), 0x9u64, 64);
            if c.cc_e() { blk = 119; continue; }
            blk = 53; continue;
        }
        53 => {
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_e() { blk = 76; continue; }
            blk = 54; continue;
        }
        54 => {
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_e() { blk = 137; continue; }
            blk = 55; continue;
        }
        55 => {
            c.cmp(c.r64(2), 0xcu64, 64);
            if c.cc_e() { blk = 71; continue; }
            blk = 56; continue;
        }
        56 => {
            c.cmp(c.r64(2), 0xeu64, 64);
            if c.cc_e() { blk = 61; continue; }
            blk = 57; continue;
        }
        57 => {
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t103) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1c20); };
            c.w32(2, t103);
            let Some(t104) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1c26); };
            c.w32(1, t104);
            let r105 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r105);
            let Some(t106) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1c2e); };
            c.w32(2, t106);
            let r107 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r107);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f1c35); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f1c39); }
            let Some(t108) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f1c3a); };
            if !c.push64(t108) { return RegfileExit::fault(0x70008f1c3a); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f1c3d); }
            let Some(t109) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f1c3e); };
            if !c.push64(t109) { return RegfileExit::fault(0x70008f1c3e); }
            let Some(t110) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f1c41); };
            if !c.push64(t110) { return RegfileExit::fault(0x70008f1c41); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(11)) { return RegfileExit::fault(0x70008f1c44); }
            let Some(t111) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x70008f1c48); };
            c.w64(3, t111);
            c.w32(0, 0x5u64);
            let Some(t112) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f1c54); };
            c.w64(6, t112);
            return c.call(0x700086ed74, 0x70008f1c5d, 0x70008f1c58);
        }
        58 => {
            let Some(t113) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1c5d); };
            c.w64(8, t113);
            let Some(t114) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1c61); };
            c.w64(11, t114);
            let Some(t115) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008f1c65); };
            c.w64(9, t115);
            let Some(t116) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1c69); };
            c.w32(3, t116);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 60; continue; }
            blk = 59; continue;
        }
        59 => {
            let Some(t117) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1c7e); };
            c.w64(15, t117);
            let Some(t118) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1c82); };
            c.w64(1, t118);
            let Some(t119) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1c86); };
            c.w64(5, t119);
            c.w64(12, c.r64(11));
            let r120 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r120);
            return RegfileExit::jump(c.r64(1));
        }
        60 => {
            let r121 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r121);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1c97); }
            let Some(t122) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1c9f); };
            c.w32(8, t122);
            let Some(t123) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1ca4); };
            c.w64(15, t123);
            let Some(t124) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1ca8); };
            c.w64(1, t124);
            let Some(t125) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1cac); };
            c.w64(5, t125);
            c.w64(12, c.r64(11));
            let r126 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r126);
            return RegfileExit::jump(c.r64(1));
        }
        61 => {
            let Some(t127) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f1cb9); };
            c.w32(11, t127);
            let r128 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r128);
            let Some(t129) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1cc0); };
            c.w32(11, t129);
            let r130 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r130);
            let Some(t131) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1cc7); };
            c.w32(0, t131);
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r132 = c.r64(6) & 0x1ffffffu64;
            c.w64(6, r132);
            c.logic(r132, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_b() { blk = 63; continue; }
            blk = 62; continue;
        }
        62 => {
            return RegfileExit::fault(0x70008f1ce1);
        }
        63 => {
            c.w64(0, c.r64(9).wrapping_add(0x3));
            let Some(t133) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1ce6); };
            c.w32(11, t133);
            let r134 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r134);
            let Some(t135) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1cee); };
            c.w32(11, t135);
            let r136 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r136);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f1cf5); }
            let Some(t137) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f1cf9); };
            c.cmp(t137, c.r32(11), 32);
            if c.cc_e() { blk = 67; continue; }
            blk = 64; continue;
        }
        64 => {
            let Some(t138) = c.ld8(c.r64(0).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1d02); };
            c.w32(3, t138);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 66; continue; }
            blk = 65; continue;
        }
        65 => {
            let Some(t139) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1d17); };
            c.w64(1, t139);
            let Some(t140) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1d1b); };
            c.w64(5, t140);
            c.w64(9, c.r64(0));
            c.w64(0, c.r64(11));
            let r141 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r141);
            return RegfileExit::jump(c.r64(1));
        }
        66 => {
            c.w64(9, c.r64(0).wrapping_add(0x1));
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f1d2f); }
            let Some(t142) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f1d37); };
            c.w32(8, t142);
            let Some(t143) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1d3c); };
            c.w64(1, t143);
            let Some(t144) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1d40); };
            c.w64(5, t144);
            c.w64(0, c.r64(11));
            let r145 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r145);
            return RegfileExit::jump(c.r64(1));
        }
        67 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t146) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1d51); };
            c.w32(3, t146);
            let Some(t147) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1d57); };
            c.w32(2, t147);
            let r148 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r148);
            let Some(t149) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1d5f); };
            c.w32(3, t149);
            let r150 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r150);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f1d66); }
            c.w32(11, 0x174u64);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f1d70); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f1d72); }
            let Some(t151) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008f1d73); };
            c.w64(3, t151);
            let Some(t152) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f1d7a); };
            c.w64(6, t152);
            c.w32(0, 0x2u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008f1d8b, 0x70008f1d86);
        }
        68 => {
            let Some(t153) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1d8b); };
            c.w64(8, t153);
            let Some(t154) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1d8f); };
            c.w64(11, t154);
            let Some(t155) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f1d93); };
            c.w64(9, t155);
            let Some(t156) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1d97); };
            c.w32(3, t156);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 70; continue; }
            blk = 69; continue;
        }
        69 => {
            let Some(t157) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1dac); };
            c.w64(15, t157);
            let Some(t158) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1db0); };
            c.w64(1, t158);
            let Some(t159) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1db4); };
            c.w64(5, t159);
            c.w64(12, c.r64(11));
            let r160 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r160);
            return RegfileExit::jump(c.r64(1));
        }
        70 => {
            let r161 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r161);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1dc5); }
            let Some(t162) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1dcd); };
            c.w32(8, t162);
            let Some(t163) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1dd2); };
            c.w64(15, t163);
            let Some(t164) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1dd6); };
            c.w64(1, t164);
            let Some(t165) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1dda); };
            c.w64(5, t165);
            c.w64(12, c.r64(11));
            let r166 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r166);
            return RegfileExit::jump(c.r64(1));
        }
        71 => {
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t167) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1def); };
            c.w32(2, t167);
            let Some(t168) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1df5); };
            c.w32(1, t168);
            let r169 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r169);
            let Some(t170) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1dfd); };
            c.w32(2, t170);
            let r171 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r171);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f1e04); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008f1e08); }
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(2));
            let Some(t172) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f1e12); };
            c.w64(1, t172);
            c.w32(2, 0);
            c.logic(0, 32);
            let Some(t173) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f1e18); };
            c.w64(6, t173);
            return c.call(0x70008bc174, 0x70008f1e21, 0x70008f1e1c);
        }
        72 => {
            let Some(t174) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1e21); };
            c.w64(8, t174);
            let Some(t175) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1e25); };
            c.w64(11, t175);
            let Some(t176) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f1e29); };
            c.w64(9, t176);
            let Some(t177) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1e2d); };
            c.w32(3, t177);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 74; continue; }
            blk = 73; continue;
        }
        73 => {
            let Some(t178) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1e42); };
            c.w64(15, t178);
            let Some(t179) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1e46); };
            c.w64(1, t179);
            let Some(t180) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1e4a); };
            c.w64(5, t180);
            c.w64(12, c.r64(11));
            let r181 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r181);
            return RegfileExit::jump(c.r64(1));
        }
        74 => {
            let r182 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r182);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f1e5b); }
            let Some(t183) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f1e63); };
            c.w32(8, t183);
            let Some(t184) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1e68); };
            c.w64(15, t184);
            let Some(t185) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f1e6c); };
            c.w64(1, t185);
            let Some(t186) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f1e70); };
            c.w64(5, t186);
            c.w64(12, c.r64(11));
            let r187 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r187);
            return RegfileExit::jump(c.r64(1));
        }
        75 => {
            blk = 137; continue;
        }
        76 => {
            let Some(t188) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f1e82); };
            c.w32(0, t188);
            let r189 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r189);
            let Some(t190) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1e88); };
            c.w32(2, t190);
            let Some(t191) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f1e8b); };
            c.cmp(t191, c.r32(0), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 77; continue;
        }
        77 => {
            let Some(t192) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f1e98); };
            c.w32(1, t192);
            let Some(t193) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1e9e); };
            c.w32(6, t193);
            let r194 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r194);
            let Some(t195) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1ea6); };
            c.w32(1, t195);
            let r196 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r196);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 115; continue; }
            blk = 78; continue;
        }
        78 => {
            let Some(t197) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1eb6); };
            c.w32(2, t197);
            let r198 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r198);
            let Some(t199) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f1ebc); };
            c.cmp(t199, c.r32(2), 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 79; continue;
        }
        79 => {
            let Some(t200) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f1ec9); };
            c.w32(2, t200);
            let r201 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r201);
            let Some(t202) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1ecf); };
            c.w32(6, t202);
            let r203 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r203);
            let Some(t204) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f1ed5); };
            c.logic(t204 & 0x2u64, 8);
            if c.cc_ne() { blk = 98; continue; }
            blk = 80; continue;
        }
        80 => {
            let Some(t205) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f1edf); };
            c.cmp(t205, c.r32(6), 32);
            if c.cc_e() { blk = 82; continue; }
            blk = 81; continue;
        }
        81 => {
            c.w64(0, c.r64(11));
            blk = 115; continue;
        }
        82 => {
            let Some(t206) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f1ef0); };
            c.w64(11, t206);
            let Some(t207) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f1ef7); };
            c.cmp(t207, 0x20u64, 8);
            if c.cc_e() { blk = 96; continue; }
            blk = 83; continue;
        }
        83 => {
            let Some(t208) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f1f01); };
            c.w32(11, t208);
            let Some(t209) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f1f05); };
            c.cmp(t209, c.r32(11), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 84; continue;
        }
        84 => {
            let Some(t210) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1f12); };
            c.w32(11, t210);
            let r211 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r211);
            let Some(t212) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1f19); };
            c.w32(1, t212);
            let Some(t213) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f1f1d); };
            c.cmp(t213, c.r32(1), 32);
            if c.cc_ne() { blk = 86; continue; }
            blk = 85; continue;
        }
        85 => {
            let Some(t214) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f1f26); };
            c.w32(11, t214);
            let r215 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r215);
            let Some(t216) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1f2d); };
            c.w32(1, t216);
            let Some(t217) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f1f31); };
            c.cmp(t217, c.r32(1), 32);
            if c.cc_e() { blk = 85; continue; }
            blk = 86; continue;
        }
        86 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 87; continue;
        }
        87 => {
            let Some(t218) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1f44); };
            c.w32(1, t218);
            let r219 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r219);
            let Some(t220) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1f4b); };
            c.w32(6, t220);
            c.w32(7, c.r64(6).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 92; continue; }
            blk = 88; continue;
        }
        88 => {
            c.w32(11, c.r16(6));
            c.cmp(c.r32(11), 0x82au64, 32);
            if c.cc_e() { blk = 91; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t221) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f1f67); };
            c.cmp(t221, c.r32(1), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 90; continue;
        }
        90 => {
            blk = 93; continue;
        }
        91 => {
            c.w64(3, c.r64(0));
            blk = 93; continue;
        }
        92 => {
            c.w64(3, c.r64(11));
            blk = 93; continue;
        }
        93 => {
            let Some(t222) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f1f7e); };
            c.w32(11, t222);
            let r223 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r223);
            let Some(t224) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f1f85); };
            c.w32(11, t224);
            let r225 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r225);
            let Some(t226) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f1f8c); };
            c.w32(0, t226);
            let r227 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r227);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 303; continue; }
            blk = 94; continue;
        }
        94 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t228) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f1fa3); };
            c.w32(6, t228);
            let r229 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r229);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f1faa); }
            let Some(t230) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f1fae); };
            if !c.push64(t230) { return RegfileExit::fault(0x70008f1fae); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f1fb8, 0x70008f1fb3);
        }
        95 => {
            let Some(t231) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f1fb8); };
            c.w64(9, t231);
            let Some(t232) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1fbc); };
            c.w64(15, t232);
            let Some(t233) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1fc0); };
            c.w64(8, t233);
            blk = 115; continue;
        }
        96 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f1fcd); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f1fd1); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f1fd2); }
            let Some(t234) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f1fd3); };
            if !c.push64(t234) { return RegfileExit::fault(0x70008f1fd3); }
            let Some(t235) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f1fd6); };
            c.w64(6, t235);
            let Some(t236) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f1fda); };
            c.w64(3, t236);
            c.w32(0, 0x3u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008f1fee, 0x70008f1fe9);
        }
        97 => {
            let Some(t237) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f1fee); };
            c.w64(9, t237);
            let Some(t238) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f1ff2); };
            c.w64(15, t238);
            let Some(t239) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f1ff6); };
            c.w64(8, t239);
            blk = 115; continue;
        }
        98 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f2006); }
            let Some(t240) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f200a); };
            if !c.push64(t240) { return RegfileExit::fault(0x70008f200a); }
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(7, c.r64(2));
            let Some(t241) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f2012); };
            c.w64(6, t241);
            return c.call(0x70007f2274, 0x70008f201b, 0x70008f2016);
        }
        99 => {
            let Some(t242) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f201b); };
            c.w64(9, t242);
            let Some(t243) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f201f); };
            c.w64(15, t243);
            let Some(t244) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2023); };
            c.w64(8, t244);
            blk = 115; continue;
        }
        100 => {
            let Some(t245) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f202c); };
            c.w32(11, t245);
            let r246 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r246);
            let Some(t247) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2033); };
            c.w32(1, t247);
            c.cmp(c.r16(1), 0x834u64, 16);
            if c.cc_e() { blk = 114; continue; }
            blk = 101; continue;
        }
        101 => {
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 107; continue; }
            blk = 102; continue;
        }
        102 => {
            c.cmp(c.r16(1), 0x411u64, 16);
            if c.cc_ne() { blk = 303; continue; }
            blk = 103; continue;
        }
        103 => {
            let Some(t248) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f2059); };
            c.w32(11, t248);
            let Some(t249) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f205d); };
            c.cmp(t249, c.r32(11), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 104; continue;
        }
        104 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 105; continue;
        }
        105 => {
            let Some(t250) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2072); };
            c.cmp(t250, 0x40u64, 16);
            if c.cc_ae() { blk = 303; continue; }
            blk = 106; continue;
        }
        106 => {
            let Some(t251) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f207d); };
            c.w32(11, t251);
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            blk = 115; continue;
        }
        107 => {
            let Some(t252) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f208a); };
            c.w32(2, t252);
            let Some(t253) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f208d); };
            c.cmp(t253, c.r32(2), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 108; continue;
        }
        108 => {
            let Some(t254) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f209a); };
            c.w32(11, t254);
            c.w64(2, c.r64(11));
            let r255 = c.r32(2) & 0xc0u64;
            c.w32(2, r255);
            c.logic(r255, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 110; continue; }
            blk = 109; continue;
        }
        109 => {
            let Some(t256) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f20b0); };
            c.w32(0, t256);
            let r257 = c.r32(0) & 0x1fu64;
            c.w32(0, r257);
            c.logic(r257, 32);
            let r258 = c.sub(c.r32(0), 0x10u64, 32);
            c.w32(0, r258);
            c.cmp(c.r32(0), 0x4u64, 32);
            if c.cc_a() { blk = 303; continue; }
            blk = 110; continue;
        }
        110 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_ne() { blk = 303; continue; }
            blk = 111; continue;
        }
        111 => {
            let Some(t259) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f20cc); };
            c.w32(0, t259);
            let r260 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r260);
            let Some(t261) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f20d2); };
            c.cmp(t261, c.r32(0), 32);
            if c.cc_e() { blk = 303; continue; }
            blk = 112; continue;
        }
        112 => {
            let Some(t262) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f20df); };
            c.w32(11, t262);
            let Some(t263) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f20e3); };
            c.cmp(t263, c.r32(11), 32);
            if c.cc_ne() { blk = 115; continue; }
            blk = 113; continue;
        }
        113 => {
            let Some(t264) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f20ec); };
            c.w32(0, t264);
            let r265 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r265);
            blk = 115; continue;
        }
        114 => {
            let Some(t266) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f20f4); };
            c.w32(11, t266);
            let Some(t267) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f20f8); };
            c.cmp(t267, c.r32(11), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 115; continue;
        }
        115 => {
            let r268 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r268);
            let Some(t269) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2109); };
            c.w64(11, t269);
            let Some(t270) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f210d); };
            c.w32(3, t270);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 117; continue; }
            blk = 116; continue;
        }
        116 => {
            let Some(t271) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2122); };
            c.w64(1, t271);
            let Some(t272) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2126); };
            c.w64(5, t272);
            c.w64(12, c.r64(11));
            let r273 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r273);
            return RegfileExit::jump(c.r64(1));
        }
        117 => {
            let r274 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r274);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2137); }
            let Some(t275) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f213f); };
            c.w32(8, t275);
            let Some(t276) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2144); };
            c.w64(1, t276);
            let Some(t277) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2148); };
            c.w64(5, t277);
            c.w64(12, c.r64(11));
            let r278 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r278);
            return RegfileExit::jump(c.r64(1));
        }
        118 => {
            blk = 303; continue;
        }
        119 => {
            let Some(t279) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f215a); };
            c.w64(11, t279);
            let Some(t280) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f2161); };
            c.cmp(t280, 0x20u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 120; continue;
        }
        120 => {
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 122; continue; }
            blk = 121; continue;
        }
        121 => {
            let Some(t281) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f2171); };
            c.w32(11, t281);
            let r282 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r282);
            blk = 123; continue;
        }
        122 => {
            let Some(t283) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f217a); };
            c.w32(11, t283);
            let r284 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r284);
            blk = 123; continue;
        }
        123 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_ne() { blk = 125; continue; }
            blk = 124; continue;
        }
        124 => {
            return RegfileExit::fault(0x70008f2187);
        }
        125 => {
            let r285 = c.r64(11) & 0xfffffffffffffffdu64;
            c.w64(11, r285);
            c.logic(r285, 64);
            let Some(t286) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f218c); };
            c.w32(2, t286);
            let r287 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r287);
            let Some(t288) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f2192); };
            c.w32(3, t288);
            let r289 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r289);
            let Some(t290) = c.ld64(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f2198); };
            c.w64(2, t290);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 127; continue; }
            blk = 126; continue;
        }
        126 => {
            let Some(t291) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f21a1); };
            c.w32(0, t291);
            let r292 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r292);
            let Some(t293) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f21a7); };
            c.w32(1, t293);
            let r294 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r294);
            blk = 127; continue;
        }
        127 => {
            c.w64(0, c.r64(9).wrapping_add(0x3));
            c.w32(6, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(6)) { return RegfileExit::fault(0x70008f21b5); }
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f21b9); };
            if !c.push64(t295) { return RegfileExit::fault(0x70008f21b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(0)) { return RegfileExit::fault(0x70008f21bc); }
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(6, c.r64(11));
            return c.call(0x70007fe8d4, 0x70008f21cd, 0x70008f21c8);
        }
        128 => {
            let Some(t296) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f21cd); };
            c.w64(8, t296);
            let Some(t297) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f21d1); };
            c.w64(11, t297);
            let Some(t298) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f21d5); };
            c.w64(9, t298);
            let Some(t299) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f21d9); };
            c.w32(3, t299);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 130; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t300) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f21ee); };
            c.w64(15, t300);
            let Some(t301) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f21f2); };
            c.w64(1, t301);
            let Some(t302) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f21f6); };
            c.w64(5, t302);
            c.w64(12, c.r64(11));
            let r303 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r303);
            return RegfileExit::jump(c.r64(1));
        }
        130 => {
            let r304 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r304);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2207); }
            let Some(t305) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f220f); };
            c.w32(8, t305);
            let Some(t306) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2214); };
            c.w64(15, t306);
            let Some(t307) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2218); };
            c.w64(1, t307);
            let Some(t308) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f221c); };
            c.w64(5, t308);
            c.w64(12, c.r64(11));
            let r309 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r309);
            return RegfileExit::jump(c.r64(1));
        }
        131 => {
            blk = 137; continue;
        }
        132 => {
            let Some(t310) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f222e); };
            c.w64(11, t310);
            let Some(t311) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f2235); };
            c.cmp(t311, 0x20u64, 8);
            if c.cc_e() { blk = 137; continue; }
            blk = 133; continue;
        }
        133 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(11, c.r64(9).wrapping_add(0x3));
            let r312 = c.r64(6) & 0x3ffu64;
            c.w64(6, r312);
            c.logic(r312, 64);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let r313 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(1, r313);
            let Some(t314) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2256); };
            c.w32(6, t314);
            let r315 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r315);
            let Some(t316) = c.ld32(c.r64(6).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f225c); };
            c.w32(6, t316);
            let r317 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r317);
            let Some(t318) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f2262); };
            c.w32(1, t318);
            let r319 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r319);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f2269); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f226d); }
            c.w64(2, c.r64(0));
            c.w64(10, c.r64(1));
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(10));
            let Some(t320) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f227d); };
            c.w64(6, t320);
            return c.call(0x70007fea14, 0x70008f2286, 0x70008f2281);
        }
        134 => {
            let Some(t321) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2286); };
            c.w64(8, t321);
            let Some(t322) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f228a); };
            c.w64(11, t322);
            let Some(t323) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f228e); };
            c.w64(9, t323);
            let Some(t324) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f2292); };
            c.w32(3, t324);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t325) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f22a7); };
            c.w64(15, t325);
            let Some(t326) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f22ab); };
            c.w64(1, t326);
            let Some(t327) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f22af); };
            c.w64(5, t327);
            c.w64(12, c.r64(11));
            let r328 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r328);
            return RegfileExit::jump(c.r64(1));
        }
        136 => {
            let r329 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r329);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f22c0); }
            let Some(t330) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f22c8); };
            c.w32(8, t330);
            let Some(t331) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f22cd); };
            c.w64(15, t331);
            let Some(t332) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f22d1); };
            c.w64(1, t332);
            let Some(t333) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f22d5); };
            c.w64(5, t333);
            c.w64(12, c.r64(11));
            let r334 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r334);
            return RegfileExit::jump(c.r64(1));
        }
        137 => {
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t335) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f22ea); };
            c.w32(3, t335);
            let Some(t336) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f22f0); };
            c.w32(2, t336);
            let r337 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r337);
            let Some(t338) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f22f8); };
            c.w32(3, t338);
            let r339 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r339);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f22ff); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f2303); }
            let Some(t340) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f2304); };
            if !c.push64(t340) { return RegfileExit::fault(0x70008f2304); }
            let Some(t341) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f2307); };
            if !c.push64(t341) { return RegfileExit::fault(0x70008f2307); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f230a); }
            let Some(t342) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x70008f230e); };
            c.w64(3, t342);
            c.w32(0, 0x3u64);
            let Some(t343) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f231a); };
            c.w64(6, t343);
            return c.call(0x700086ed74, 0x70008f2323, 0x70008f231e);
        }
        138 => {
            let Some(t344) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2323); };
            c.w64(8, t344);
            let Some(t345) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2327); };
            c.w64(11, t345);
            let Some(t346) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f232b); };
            c.w64(9, t346);
            let Some(t347) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f232f); };
            c.w32(3, t347);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 140; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t348) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2344); };
            c.w64(15, t348);
            let Some(t349) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2348); };
            c.w64(1, t349);
            let Some(t350) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f234c); };
            c.w64(5, t350);
            c.w64(12, c.r64(11));
            let r351 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r351);
            return RegfileExit::jump(c.r64(1));
        }
        140 => {
            let r352 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r352);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f235d); }
            let Some(t353) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f2365); };
            c.w32(8, t353);
            let Some(t354) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f236a); };
            c.w64(15, t354);
            let Some(t355) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f236e); };
            c.w64(1, t355);
            let Some(t356) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2372); };
            c.w64(5, t356);
            c.w64(12, c.r64(11));
            let r357 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r357);
            return RegfileExit::jump(c.r64(1));
        }
        141 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(11, c.r64(9).wrapping_add(0x3));
            let r358 = c.r64(6) & 0x3ffu64;
            c.w64(6, r358);
            c.logic(r358, 64);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let r359 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(1, r359);
            let Some(t360) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2396); };
            c.w32(3, t360);
            let r361 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r361);
            let Some(t362) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f239c); };
            c.w32(3, t362);
            let r363 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r363);
            let Some(t364) = c.ld32(c.r64(3).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f23a2); };
            c.w32(3, t364);
            let r365 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r365);
            let Some(t366) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f23a9); };
            c.w32(7, t366);
            let r367 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r367);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f23af); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f23b3); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f23b4); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t368) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f23ba); };
            c.w64(6, t368);
            return c.call(0x70007f2274, 0x70008f23c3, 0x70008f23be);
        }
        142 => {
            let Some(t369) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f23c3); };
            c.w64(8, t369);
            let Some(t370) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f23c7); };
            c.w64(11, t370);
            let Some(t371) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f23cb); };
            c.w64(9, t371);
            let Some(t372) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f23cf); };
            c.w32(3, t372);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 144; continue; }
            blk = 143; continue;
        }
        143 => {
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f23e4); };
            c.w64(15, t373);
            let Some(t374) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f23e8); };
            c.w64(1, t374);
            let Some(t375) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f23ec); };
            c.w64(5, t375);
            c.w64(12, c.r64(11));
            let r376 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r376);
            return RegfileExit::jump(c.r64(1));
        }
        144 => {
            let r377 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r377);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f23fd); }
            let Some(t378) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f2405); };
            c.w32(8, t378);
            let Some(t379) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f240a); };
            c.w64(15, t379);
            let Some(t380) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f240e); };
            c.w64(1, t380);
            let Some(t381) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2412); };
            c.w64(5, t381);
            c.w64(12, c.r64(11));
            let r382 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r382);
            return RegfileExit::jump(c.r64(1));
        }
        145 => {
            let Some(t383) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f241f); };
            c.w32(2, t383);
            let r384 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r384);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 147; continue; }
            blk = 146; continue;
        }
        146 => {
            let Some(t385) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008f242a); };
            c.w64(2, t385);
            blk = 147; continue;
        }
        147 => {
            let Some(t386) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f2431); };
            c.w32(1, t386);
            let Some(t387) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f2434); };
            c.w32(6, t387);
            let Some(t388) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f243a); };
            c.w32(7, t388);
            let r389 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r389);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t390) = c.ld32(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2445); };
            c.w32(6, t390);
            let r391 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r391);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r392 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r392);
            let Some(t393) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f2453); };
            c.w32(0, t393);
            let r394 = c.shr(c.r32(0), 0x2u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r394);
            let r395 = c.r64(0) & c.r64(1);
            c.w64(0, r395);
            c.logic(r395, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 148; continue;
        }
        148 => {
            c.w64(8, c.r64(0).wrapping_add(c.r64(0).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(8).wrapping_mul(4));
            let Some(t396) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f246c); };
            c.w32(11, t396);
            let Some(t397) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f2471); };
            c.cmp(t397, c.r32(11), 32);
            if c.cc_e() { blk = 207; continue; }
            blk = 149; continue;
        }
        149 => {
            c.cmp(c.r32(6), c.r32(11), 32);
            if c.cc_e() { blk = 151; continue; }
            blk = 150; continue;
        }
        150 => {
            let r398 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r398);
            c.w64(8, c.r64(15).wrapping_add(c.r64(0)));
            c.w64(11, c.r64(1));
            let r399 = c.r64(11) & c.r64(8);
            c.w64(11, r399);
            c.logic(r399, 64);
            c.w64(0, c.r64(11));
            blk = 148; continue;
        }
        151 => {
            let Some(t400) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f2496); };
            c.w32(11, t400);
            let Some(t401) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f249a); };
            c.w32(0, t401);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r402 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r402);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 153; continue; }
            blk = 152; continue;
        }
        152 => {
            return RegfileExit::fault(0x70008f24ad);
        }
        153 => {
            let Some(t403) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f24ae); };
            c.w32(8, t403);
            let Some(t404) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f24b4); };
            c.w32(11, t404);
            let r405 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r405);
            let Some(t406) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f24bc); };
            c.w32(8, t406);
            let r407 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r407);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 204; continue; }
            blk = 154; continue;
        }
        154 => {
            let Some(t408) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f24cc); };
            c.w32(0, t408);
            let Some(t409) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f24d0); };
            c.cmp(t409, c.r32(0), 32);
            if c.cc_ne() { blk = 179; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t410) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f24dd); };
            c.w32(2, t410);
            let r411 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r411);
            let Some(t412) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f24e4); };
            c.w32(11, t412);
            let r413 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r413);
            let Some(t414) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f24eb); };
            c.logic(t414 & 0x2u64, 8);
            if c.cc_ne() { blk = 177; continue; }
            blk = 156; continue;
        }
        156 => {
            let Some(t415) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f24f6); };
            c.cmp(t415, c.r32(11), 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t416) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f24ff); };
            c.w64(11, t416);
            blk = 204; continue;
        }
        158 => {
            let Some(t417) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f250b); };
            c.w64(11, t417);
            let Some(t418) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f2512); };
            c.cmp(t418, 0x20u64, 8);
            if c.cc_e() { blk = 175; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t419) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f251c); };
            c.w32(8, t419);
            let Some(t420) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f2520); };
            c.cmp(t420, c.r32(8), 32);
            if c.cc_e() { blk = 161; continue; }
            blk = 160; continue;
        }
        160 => {
            let Some(t421) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2529); };
            c.w64(8, t421);
            blk = 303; continue;
        }
        161 => {
            let Some(t422) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f2532); };
            c.w64(0, t422);
            let Some(t423) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2536); };
            c.w32(8, t423);
            let r424 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r424);
            let Some(t425) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f253d); };
            c.w32(11, t425);
            let Some(t426) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f2541); };
            c.cmp(t426, c.r32(11), 32);
            if c.cc_ne() { blk = 163; continue; }
            blk = 162; continue;
        }
        162 => {
            let Some(t427) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f254a); };
            c.w32(8, t427);
            let r428 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r428);
            let Some(t429) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2551); };
            c.w32(11, t429);
            let Some(t430) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f2555); };
            c.cmp(t430, c.r32(11), 32);
            if c.cc_e() { blk = 162; continue; }
            blk = 163; continue;
        }
        163 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 174; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t431) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2568); };
            c.w32(11, t431);
            let r432 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r432);
            let Some(t433) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f256f); };
            c.w32(3, t433);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 169; continue; }
            blk = 165; continue;
        }
        165 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t434) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f258c); };
            c.cmp(t434, c.r32(11), 32);
            if c.cc_ne() { blk = 170; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t435) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2595); };
            c.w64(8, t435);
            blk = 303; continue;
        }
        168 => {
            let Some(t436) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f259e); };
            c.w32(6, t436);
            let r437 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r437);
            c.w64(0, c.r64(6));
            blk = 170; continue;
        }
        169 => {
            c.w64(0, c.r64(8));
            blk = 170; continue;
        }
        170 => {
            let Some(t438) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f25ac); };
            c.w32(8, t438);
            let r439 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r439);
            let Some(t440) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f25b3); };
            c.w32(8, t440);
            let r441 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r441);
            let Some(t442) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f25ba); };
            c.w32(11, t442);
            let r443 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r443);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 173; continue; }
            blk = 171; continue;
        }
        171 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t444) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f25d0); };
            c.w32(6, t444);
            let r445 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r445);
            let Some(t446) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f25d7); };
            c.w64(8, t446);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f25db); }
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f25df); };
            if !c.push64(t447) { return RegfileExit::fault(0x70008f25df); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f25e9, 0x70008f25e4);
        }
        172 => {
            c.w64(11, c.r64(0));
            let Some(t448) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f25ec); };
            c.w64(9, t448);
            blk = 204; continue;
        }
        173 => {
            let Some(t449) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f25f5); };
            c.w64(8, t449);
            blk = 303; continue;
        }
        174 => {
            let Some(t450) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f25fe); };
            c.w64(8, t450);
            blk = 303; continue;
        }
        175 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t451) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f260e); };
            c.w64(3, t451);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f2612); }
            let Some(t452) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f2616); };
            if !c.push64(t452) { return RegfileExit::fault(0x70008f2616); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f2619); }
            let Some(t453) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f261b); };
            if !c.push64(t453) { return RegfileExit::fault(0x70008f261b); }
            let Some(t454) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f261e); };
            c.w64(6, t454);
            let Some(t455) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f2622); };
            c.w64(3, t455);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008f2636, 0x70008f2631);
        }
        176 => {
            c.w64(11, c.r64(0));
            let Some(t456) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f2639); };
            c.w64(9, t456);
            blk = 204; continue;
        }
        177 => {
            c.w64(8, c.r64(9));
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t457) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2649); };
            c.w64(0, t457);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f264d); }
            let Some(t458) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f2651); };
            if !c.push64(t458) { return RegfileExit::fault(0x70008f2651); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t459) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f2659); };
            c.w64(6, t459);
            return c.call(0x70007f2274, 0x70008f2662, 0x70008f265d);
        }
        178 => {
            c.w64(11, c.r64(0));
            let Some(t460) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f2665); };
            c.w64(9, t460);
            blk = 204; continue;
        }
        179 => {
            let Some(t461) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f266e); };
            c.w64(0, t461);
            let Some(t462) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2672); };
            c.w32(8, t462);
            let r463 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r463);
            let Some(t464) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2679); };
            c.w32(3, t464);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 201; continue; }
            blk = 180; continue;
        }
        180 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 190; continue; }
            blk = 181; continue;
        }
        181 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 183; continue; }
            blk = 182; continue;
        }
        182 => {
            let Some(t465) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f269b); };
            c.w64(8, t465);
            blk = 303; continue;
        }
        183 => {
            let Some(t466) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f26a4); };
            c.w32(8, t466);
            let Some(t467) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f26a8); };
            c.cmp(t467, c.r32(8), 32);
            if c.cc_e() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t468) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f26b1); };
            c.w64(8, t468);
            blk = 303; continue;
        }
        185 => {
            let Some(t469) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f26ba); };
            c.w32(8, t469);
            let r470 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r470);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let Some(t471) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f26c7); };
            c.w64(8, t471);
            blk = 303; continue;
        }
        187 => {
            let Some(t472) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f26d0); };
            c.w32(11, t472);
            let r473 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r473);
            let Some(t474) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f26d7); };
            c.cmp(t474, 0x40u64, 16);
            if c.cc_b() { blk = 189; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t475) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f26df); };
            c.w64(8, t475);
            blk = 303; continue;
        }
        189 => {
            let Some(t476) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f26e8); };
            c.w32(8, t476);
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            blk = 204; continue;
        }
        190 => {
            let Some(t477) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f26f5); };
            c.w32(11, t477);
            let Some(t478) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f26f9); };
            c.cmp(t478, c.r32(11), 32);
            if c.cc_e() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            let Some(t479) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2702); };
            c.w64(8, t479);
            blk = 303; continue;
        }
        192 => {
            let Some(t480) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f270b); };
            c.w32(8, t480);
            c.w64(11, c.r64(8));
            let r481 = c.r32(11) & 0xc0u64;
            c.w32(11, r481);
            c.logic(r481, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 195; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t482) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f2723); };
            c.w32(11, t482);
            let r483 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r483);
            let Some(t484) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f272a); };
            c.w32(11, t484);
            let r485 = c.r32(11) & 0x1fu64;
            c.w32(11, r485);
            c.logic(r485, 32);
            let r486 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r486);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 195; continue; }
            blk = 194; continue;
        }
        194 => {
            let Some(t487) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f273c); };
            c.w64(8, t487);
            blk = 303; continue;
        }
        195 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 200; continue; }
            blk = 196; continue;
        }
        196 => {
            let Some(t488) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f274b); };
            c.w32(11, t488);
            let r489 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r489);
            let Some(t490) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f2752); };
            c.cmp(t490, c.r32(11), 32);
            if c.cc_e() { blk = 199; continue; }
            blk = 197; continue;
        }
        197 => {
            let Some(t491) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f275b); };
            c.w32(8, t491);
            let Some(t492) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f275f); };
            c.cmp(t492, c.r32(8), 32);
            if c.cc_ne() { blk = 204; continue; }
            blk = 198; continue;
        }
        198 => {
            let Some(t493) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f2768); };
            c.w32(11, t493);
            let r494 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r494);
            blk = 204; continue;
        }
        199 => {
            let Some(t495) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2771); };
            c.w64(8, t495);
            blk = 303; continue;
        }
        200 => {
            let Some(t496) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f277a); };
            c.w64(8, t496);
            blk = 303; continue;
        }
        201 => {
            let Some(t497) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f2783); };
            c.w32(8, t497);
            let Some(t498) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f2787); };
            c.cmp(t498, c.r32(8), 32);
            if c.cc_e() { blk = 203; continue; }
            blk = 202; continue;
        }
        202 => {
            let Some(t499) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2790); };
            c.w64(8, t499);
            blk = 303; continue;
        }
        203 => {
            let Some(t500) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f2799); };
            c.w32(11, t500);
            let r501 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r501);
            blk = 204; continue;
        }
        204 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t502) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f27a7); };
            c.w64(0, t502);
            let Some(t503) = c.ld64(c.r64(0).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f27ab); };
            c.w64(8, t503);
            let Some(t504) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f27af); };
            c.w32(3, t504);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 206; continue; }
            blk = 205; continue;
        }
        205 => {
            let Some(t505) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f27c4); };
            c.w64(15, t505);
            let Some(t506) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f27c8); };
            c.w64(1, t506);
            let Some(t507) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f27cc); };
            c.w64(5, t507);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r508 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r508);
            return RegfileExit::jump(c.r64(1));
        }
        206 => {
            let r509 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r509);
            if !c.st64(c.r64(0).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f27e0); }
            let Some(t510) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f27e8); };
            c.w32(0, t510);
            let Some(t511) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f27ed); };
            c.w64(15, t511);
            let Some(t512) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f27f1); };
            c.w64(1, t512);
            let Some(t513) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f27f5); };
            c.w64(5, t513);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r514 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r514);
            return RegfileExit::jump(c.r64(1));
        }
        207 => {
            let Some(t515) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2805); };
            c.w64(8, t515);
            blk = 303; continue;
        }
        208 => {
            c.w64(11, c.r64(9).wrapping_add(0x3));
            let Some(t516) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2812); };
            c.w64(0, t516);
            let Some(t517) = c.ld8(c.r64(0).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f2816); };
            c.w32(3, t517);
            c.w32(2, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t518) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2820); };
            c.w32(0, t518);
            let r519 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r519);
            let Some(t520) = c.ld32(c.r64(0).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2826); };
            c.w32(0, t520);
            let r521 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r521);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f282d); }
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f2831); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f2832); }
            let Some(t522) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008f2836); };
            c.w64(3, t522);
            c.w32(0, 0x1u64);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f2842); };
            c.w64(6, t523);
            return c.call(0x700086ed74, 0x70008f284b, 0x70008f2846);
        }
        209 => {
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f284b); };
            c.w64(8, t524);
            let Some(t525) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f284f); };
            c.w64(11, t525);
            let Some(t526) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f2853); };
            c.w64(9, t526);
            let Some(t527) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f2857); };
            c.w32(3, t527);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 211; continue; }
            blk = 210; continue;
        }
        210 => {
            let Some(t528) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f286c); };
            c.w64(15, t528);
            let Some(t529) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2870); };
            c.w64(1, t529);
            let Some(t530) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2874); };
            c.w64(5, t530);
            c.w64(12, c.r64(11));
            let r531 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r531);
            return RegfileExit::jump(c.r64(1));
        }
        211 => {
            let r532 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r532);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2885); }
            let Some(t533) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f288d); };
            c.w32(8, t533);
            let Some(t534) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2892); };
            c.w64(15, t534);
            let Some(t535) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2896); };
            c.w64(1, t535);
            let Some(t536) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f289a); };
            c.w64(5, t536);
            c.w64(12, c.r64(11));
            let r537 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r537);
            return RegfileExit::jump(c.r64(1));
        }
        212 => {
            let r538 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r538);
            let Some(t539) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f28ab); };
            c.w32(0, t539);
            c.w64(11, c.r64(3));
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 214; continue; }
            blk = 213; continue;
        }
        213 => {
            let Some(t540) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f28c2); };
            c.w64(1, t540);
            let Some(t541) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f28c6); };
            c.w64(5, t541);
            c.w64(0, c.r64(11));
            let r542 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r542);
            return RegfileExit::jump(c.r64(1));
        }
        214 => {
            let r543 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r543);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f28d7); }
            let Some(t544) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f28df); };
            c.w32(8, t544);
            let Some(t545) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f28e4); };
            c.w64(1, t545);
            let Some(t546) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f28e8); };
            c.w64(5, t546);
            c.w64(0, c.r64(11));
            let r547 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r547);
            return RegfileExit::jump(c.r64(1));
        }
        215 => {
            c.logic(c.r8(6) & 0x40u64, 8);
            if c.cc_ne() { blk = 233; continue; }
            blk = 216; continue;
        }
        216 => {
            c.w64(11, c.r64(6));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(6) & 0x100u64, 16);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            let r548 = c.r64(11) & 0x7ffu64;
            c.w64(11, r548);
            c.logic(r548, 64);
            c.logic(c.r8(6) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 226; continue; }
            blk = 217; continue;
        }
        217 => {
            let Some(t549) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f2922); };
            c.w32(3, t549);
            let r550 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r550);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 219; continue; }
            blk = 218; continue;
        }
        218 => {
            let Some(t551) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008f292d); };
            c.w64(3, t551);
            blk = 219; continue;
        }
        219 => {
            let Some(t552) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2934); };
            c.w32(11, t552);
            let r553 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r553);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 223; continue; }
            blk = 220; continue;
        }
        220 => {
            let r554 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r554);
            let Some(t555) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2945); };
            c.w32(0, t555);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 222; continue; }
            blk = 221; continue;
        }
        221 => {
            let Some(t556) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2959); };
            c.w64(1, t556);
            let Some(t557) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f295d); };
            c.w64(5, t557);
            c.w64(0, c.r64(11));
            let r558 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r558);
            return RegfileExit::jump(c.r64(1));
        }
        222 => {
            let r559 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r559);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f296e); }
            let Some(t560) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2976); };
            c.w32(8, t560);
            let Some(t561) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f297b); };
            c.w64(1, t561);
            let Some(t562) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f297f); };
            c.w64(5, t562);
            c.w64(0, c.r64(11));
            let r563 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r563);
            return RegfileExit::jump(c.r64(1));
        }
        223 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 224; continue;
        }
        224 => {
            let Some(t564) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2996); };
            c.w32(0, t564);
            let Some(t565) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f299a); };
            c.cmp(t565, c.r32(0), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 225; continue;
        }
        225 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f29a7); }
            blk = 291; continue;
        }
        226 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 230; continue; }
            blk = 227; continue;
        }
        227 => {
            let r566 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r566);
            let Some(t567) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f29bb); };
            c.w32(0, t567);
            let r568 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r568);
            let Some(t569) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f29c3); };
            c.w32(11, t569);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 229; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t570) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f29d9); };
            c.w64(1, t570);
            let Some(t571) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f29dd); };
            c.w64(5, t571);
            let r572 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r572);
            return RegfileExit::jump(c.r64(1));
        }
        229 => {
            let r573 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r573);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f29eb); }
            let Some(t574) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f29f3); };
            c.w32(8, t574);
            let Some(t575) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f29f8); };
            c.w64(1, t575);
            let Some(t576) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f29fc); };
            c.w64(5, t576);
            let r577 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r577);
            return RegfileExit::jump(c.r64(1));
        }
        230 => {
            let Some(t578) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2a06); };
            c.w32(11, t578);
            let r579 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r579);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 303; continue; }
            blk = 231; continue;
        }
        231 => {
            let Some(t580) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2a18); };
            c.w32(0, t580);
            let Some(t581) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f2a1c); };
            c.cmp(t581, c.r32(0), 32);
            if c.cc_ne() { blk = 303; continue; }
            blk = 232; continue;
        }
        232 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f2a29); }
            blk = 291; continue;
        }
        233 => {
            c.w64(11, c.r64(6));
            let r582 = c.shr(c.r32(11), 0x7u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r582);
            let r583 = c.r32(11) & 0xfu64;
            c.w32(11, r583);
            c.logic(r583, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_l() { blk = 240; continue; }
            blk = 234; continue;
        }
        234 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_l() { blk = 238; continue; }
            blk = 235; continue;
        }
        235 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 301; continue; }
            blk = 236; continue;
        }
        236 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 298; continue; }
            blk = 237; continue;
        }
        237 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 298; continue; }
            blk = 238; continue;
        }
        238 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 289; continue; }
            blk = 239; continue;
        }
        239 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 290; continue; }
            blk = 240; continue;
        }
        240 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 244; continue; }
            blk = 241; continue;
        }
        241 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 253; continue; }
            blk = 242; continue;
        }
        242 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 263; continue; }
            blk = 243; continue;
        }
        243 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 273; continue; }
            blk = 244; continue;
        }
        244 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 247; continue; }
            blk = 245; continue;
        }
        245 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 250; continue; }
            blk = 246; continue;
        }
        246 => {
            return RegfileExit::fault(0x70008f2aad);
        }
        247 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r584 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r584);
            let r585 = c.r64(6) & 0xfffffu64;
            c.w64(6, r585);
            c.logic(r585, 64);
            let Some(t586) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2abd); };
            c.w32(11, t586);
            let Some(t587) = c.ld8(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2ac2); };
            c.w32(0, (t587 as u8 as i8 as i64 as u64));
            c.w64(3, c.r64(11).wrapping_mul(8));
            let r588 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r588);
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 249; continue; }
            blk = 248; continue;
        }
        248 => {
            let Some(t589) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2ada); };
            c.w64(1, t589);
            let Some(t590) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2ade); };
            c.w64(5, t590);
            let r591 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r591);
            return RegfileExit::jump(c.r64(1));
        }
        249 => {
            let r592 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r592);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2aec); }
            let Some(t593) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2af4); };
            c.w32(8, t593);
            let Some(t594) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2af9); };
            c.w64(1, t594);
            let Some(t595) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2afd); };
            c.w64(5, t595);
            let r596 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r596);
            return RegfileExit::jump(c.r64(1));
        }
        250 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r597 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r597);
            let r598 = c.r64(6) & 0xfffffu64;
            c.w64(6, r598);
            c.logic(r598, 64);
            let Some(t599) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2b16); };
            c.w32(11, t599);
            let Some(t600) = c.ld16(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2b1b); };
            c.w32(0, (t600 as u16 as i16 as i64 as u64));
            c.w64(3, c.r64(11).wrapping_mul(8));
            let r601 = c.shl(c.r32(0), 1u64, 32).unwrap_or(c.r32(0));
            c.w32(0, r601);
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 252; continue; }
            blk = 251; continue;
        }
        251 => {
            let Some(t602) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2b33); };
            c.w64(1, t602);
            let Some(t603) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2b37); };
            c.w64(5, t603);
            let r604 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r604);
            return RegfileExit::jump(c.r64(1));
        }
        252 => {
            let r605 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r605);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2b45); }
            let Some(t606) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2b4d); };
            c.w32(8, t606);
            let Some(t607) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2b52); };
            c.w64(1, t607);
            let Some(t608) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2b56); };
            c.w64(5, t608);
            let r609 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r609);
            return RegfileExit::jump(c.r64(1));
        }
        253 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r610 = c.r64(6) & 0xfffffu64;
            c.w64(6, r610);
            c.logic(r610, 64);
            let Some(t611) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2b6b); };
            c.w32(11, t611);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f2b70); }
            c.w64(0, c.r64(11));
            let r612 = c.add(c.r32(0), c.r32(11), 32);
            c.w32(0, r612);
            if c.cc_o() { blk = 255; continue; }
            blk = 254; continue;
        }
        254 => {
            c.w64(11, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(11));
            blk = 260; continue;
        }
        255 => {
            let Some(t613) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f2b84); };
            c.w64(0, t613);
            let Some(t614) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f2b8b); };
            c.w64(3, t614);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            let Some(t615) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f2b92); };
            c.w64(1, t615);
            let Some(t616) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f2b99); };
            c.cmp(t616, c.r64(2), 64);
            if c.cc_a() { blk = 258; continue; }
            blk = 256; continue;
        }
        256 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f2ba8, 0x70008f2ba3);
        }
        257 => {
            let Some(t617) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f2ba8); };
            c.w64(11, t617);
            let Some(t618) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2bac); };
            c.w64(15, t618);
            let Some(t619) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2bb0); };
            c.w64(8, t619);
            let Some(t620) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f2bb4); };
            c.w64(12, t620);
            let Some(t621) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f2bb8); };
            c.w64(9, t621);
            blk = 259; continue;
        }
        258 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008f2bbe); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 259; continue;
        }
        259 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(11) as u32 as i32 as i64));
            let Some(t622) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f2bce); };
            c.w64(11, t622);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f2bd5); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f2bd9); }
            blk = 260; continue;
        }
        260 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t623) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2be5); };
            c.w32(11, t623);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 262; continue; }
            blk = 261; continue;
        }
        261 => {
            let Some(t624) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2bfb); };
            c.w64(1, t624);
            let Some(t625) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2bff); };
            c.w64(5, t625);
            let r626 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r626);
            return RegfileExit::jump(c.r64(1));
        }
        262 => {
            let r627 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r627);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2c0d); }
            let Some(t628) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2c15); };
            c.w32(8, t628);
            let Some(t629) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2c1a); };
            c.w64(1, t629);
            let Some(t630) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2c1e); };
            c.w64(5, t630);
            let r631 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r631);
            return RegfileExit::jump(c.r64(1));
        }
        263 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r632 = c.r64(6) & 0xfffffu64;
            c.w64(6, r632);
            c.logic(r632, 64);
            let Some(t633) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2c33); };
            c.w32(11, t633);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f2c38); }
            c.cmp(c.r32(11), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 265; continue; }
            blk = 264; continue;
        }
        264 => {
            let r634 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r634);
            blk = 270; continue;
        }
        265 => {
            let Some(t635) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f2c4a); };
            c.w64(0, t635);
            let Some(t636) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f2c51); };
            c.w64(3, t636);
            c.w64(2, c.r64(3).wrapping_add(0xc));
            let Some(t637) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f2c58); };
            c.w64(1, t637);
            let Some(t638) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f2c5f); };
            c.cmp(t638, c.r64(2), 64);
            if c.cc_a() { blk = 268; continue; }
            blk = 266; continue;
        }
        266 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f2c6e, 0x70008f2c69);
        }
        267 => {
            let Some(t639) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f2c6e); };
            c.w64(11, t639);
            let Some(t640) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2c72); };
            c.w64(15, t640);
            let Some(t641) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2c76); };
            c.w64(8, t641);
            let Some(t642) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f2c7a); };
            c.w64(12, t642);
            let Some(t643) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f2c7e); };
            c.w64(9, t643);
            blk = 269; continue;
        }
        268 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008f2c84); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 269; continue;
        }
        269 => {
            c.w32(10, c.r32(11));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t644) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f2c97); };
            c.w64(11, t644);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(11)) { return RegfileExit::fault(0x70008f2c9e); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f2ca2); }
            c.w64(11, c.r64(0));
            blk = 270; continue;
        }
        270 => {
            c.w64(0, c.r64(9));
            c.w64(9, c.r64(0).wrapping_add(0x3));
            let Some(t645) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2cb1); };
            c.w32(0, t645);
            c.w64(3, c.r64(0).wrapping_mul(8));
            c.cmp(c.r32(0), 0xb4u64, 32);
            if c.cc_ae() { blk = 272; continue; }
            blk = 271; continue;
        }
        271 => {
            let Some(t646) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2cc5); };
            c.w64(1, t646);
            let Some(t647) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2cc9); };
            c.w64(5, t647);
            c.w64(0, c.r64(11));
            let r648 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r648);
            return RegfileExit::jump(c.r64(1));
        }
        272 => {
            let r649 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r649);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(11)) { return RegfileExit::fault(0x70008f2cda); }
            let Some(t650) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2ce2); };
            c.w32(8, t650);
            let Some(t651) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2ce7); };
            c.w64(1, t651);
            let Some(t652) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2ceb); };
            c.w64(5, t652);
            c.w64(0, c.r64(11));
            let r653 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r653);
            return RegfileExit::jump(c.r64(1));
        }
        273 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r654 = c.r64(6) & 0xfffffu64;
            c.w64(6, r654);
            c.logic(r654, 64);
            let Some(t655) = c.ld64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2d03); };
            c.w64(11, t655);
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f2d08); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_e() { blk = 281; continue; }
            blk = 274; continue;
        }
        274 => {
            let Some(t656) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f2d12); };
            c.w64(0, t656);
            let Some(t657) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f2d19); };
            c.w64(3, t657);
            c.w64(2, c.r64(3).wrapping_add(0x10));
            let Some(t658) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f2d20); };
            c.w64(1, t658);
            let Some(t659) = c.ld64(c.r64(1)) else { return RegfileExit::fault(0x70008f2d27); };
            c.cmp(t659, c.r64(2), 64);
            if c.cc_a() { blk = 277; continue; }
            blk = 275; continue;
        }
        275 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x70008f2d36, 0x70008f2d31);
        }
        276 => {
            let Some(t660) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f2d36); };
            c.w64(11, t660);
            let Some(t661) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2d3a); };
            c.w64(15, t661);
            let Some(t662) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2d3e); };
            c.w64(8, t662);
            let Some(t663) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f2d42); };
            c.w64(12, t663);
            let Some(t664) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f2d46); };
            c.w64(9, t664);
            blk = 278; continue;
        }
        277 => {
            if !c.st64(c.r64(0), c.r64(2)) { return RegfileExit::fault(0x70008f2d4c); }
            c.w64(0, c.r64(3).wrapping_add(0x1));
            blk = 278; continue;
        }
        278 => {
            let Some(t665) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f2d53); };
            c.w64(3, t665);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008f2d5a); }
            c.cmp(c.r64(11), 0x0u64, 64);
            if c.cc_g() { blk = 280; continue; }
            blk = 279; continue;
        }
        279 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008f2d63); }
            let r666 = c.neg(c.r64(11), 64);
            c.w64(11, r666);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x70008f2d6d); }
            blk = 286; continue;
        }
        280 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008f2d73); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(11)) { return RegfileExit::fault(0x70008f2d7a); }
            blk = 286; continue;
        }
        281 => {
            let Some(t667) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f2d80); };
            c.w64(11, t667);
            let Some(t668) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f2d87); };
            c.w64(0, t668);
            c.w64(2, c.r64(0).wrapping_add(0x8));
            let Some(t669) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f2d8e); };
            c.w64(3, t669);
            let Some(t670) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f2d95); };
            c.cmp(t670, c.r64(2), 64);
            if c.cc_a() { blk = 284; continue; }
            blk = 282; continue;
        }
        282 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008f2da4, 0x70008f2d9f);
        }
        283 => {
            blk = 285; continue;
        }
        284 => {
            if !c.st64(c.r64(11), c.r64(2)) { return RegfileExit::fault(0x70008f2da6); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 285; continue;
        }
        285 => {
            let Some(t671) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f2db0); };
            c.w64(8, t671);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f2db7); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008f2dbb); }
            let Some(t672) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f2dc2); };
            c.w64(9, t672);
            let Some(t673) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2dc6); };
            c.w64(15, t673);
            let Some(t674) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2dca); };
            c.w64(8, t674);
            let Some(t675) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f2dce); };
            c.w64(12, t675);
            blk = 286; continue;
        }
        286 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t676) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2dd9); };
            c.w32(11, t676);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 288; continue; }
            blk = 287; continue;
        }
        287 => {
            let Some(t677) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2def); };
            c.w64(1, t677);
            let Some(t678) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2df3); };
            c.w64(5, t678);
            let r679 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r679);
            return RegfileExit::jump(c.r64(1));
        }
        288 => {
            let r680 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r680);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2e01); }
            let Some(t681) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2e09); };
            c.w32(8, t681);
            let Some(t682) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2e0e); };
            c.w64(1, t682);
            let Some(t683) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2e12); };
            c.w64(5, t683);
            let r684 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r684);
            return RegfileExit::jump(c.r64(1));
        }
        289 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r685 = c.r64(6) & 0xfffffu64;
            c.w64(6, r685);
            c.logic(r685, 64);
            if !c.movss_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f2e27); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 291; continue;
        }
        290 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r686 = c.r64(6) & 0xfffffu64;
            c.w64(6, r686);
            c.logic(r686, 64);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f2e3e); }
            blk = 291; continue;
        }
        291 => {
            let Some(t687) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f2e44); };
            c.w64(11, t687);
            let Some(t688) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f2e4b); };
            c.w64(0, t688);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x18), 0) { return RegfileExit::fault(0x70008f2e52); }
            let Some(t689) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f2e57); };
            c.w64(2, t689);
            let Some(t690) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f2e5e); };
            c.cmp(t690, c.r64(3), 64);
            if c.cc_a() { blk = 294; continue; }
            blk = 292; continue;
        }
        292 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f2e6d, 0x70008f2e68);
        }
        293 => {
            blk = 295; continue;
        }
        294 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f2e6f); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 295; continue;
        }
        295 => {
            let Some(t691) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f2e79); };
            c.w64(8, t691);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t692) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f2e81); };
            c.w64(8, t692);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f2e88); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x18)) { return RegfileExit::fault(0x70008f2e8c); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f2e91); }
            let Some(t693) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f2e96); };
            c.w64(12, t693);
            let Some(t694) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2e9a); };
            c.w32(8, t694);
            c.w64(11, c.r64(8).wrapping_mul(8));
            c.cmp(c.r32(8), 0xb4u64, 32);
            if c.cc_ae() { blk = 297; continue; }
            blk = 296; continue;
        }
        296 => {
            let Some(t695) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2eb0); };
            c.w64(15, t695);
            let Some(t696) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2eb4); };
            c.w64(1, t696);
            let Some(t697) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2eb8); };
            c.w64(5, t697);
            let r698 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r698);
            return RegfileExit::jump(c.r64(1));
        }
        297 => {
            let r699 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r699);
            let Some(t700) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2ec6); };
            c.w64(2, t700);
            if !c.st64(c.r64(2).wrapping_add(c.r64(11)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2eca); }
            let Some(t701) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2ed2); };
            c.w32(8, t701);
            let Some(t702) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2ed7); };
            c.w64(15, t702);
            let Some(t703) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2edb); };
            c.w64(1, t703);
            let Some(t704) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2edf); };
            c.w64(5, t704);
            let r705 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r705);
            return RegfileExit::jump(c.r64(1));
        }
        298 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r706 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r706);
            let r707 = c.r64(6) & 0xfffffu64;
            c.w64(6, r707);
            c.logic(r707, 64);
            let Some(t708) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2ef8); };
            c.w32(11, t708);
            let Some(t709) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f2efd); };
            c.w32(0, t709);
            let r710 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r710);
            c.w64(3, c.r64(11).wrapping_mul(8));
            c.cmp(c.r32(11), 0xb4u64, 32);
            if c.cc_ae() { blk = 300; continue; }
            blk = 299; continue;
        }
        299 => {
            let Some(t711) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2f15); };
            c.w64(1, t711);
            let Some(t712) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2f19); };
            c.w64(5, t712);
            let r713 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r713);
            return RegfileExit::jump(c.r64(1));
        }
        300 => {
            let r714 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r714);
            if !c.st64(c.r64(8).wrapping_add(c.r64(3)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2f27); }
            let Some(t715) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f2f2f); };
            c.w32(8, t715);
            let Some(t716) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2f34); };
            c.w64(1, t716);
            let Some(t717) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2f38); };
            c.w64(5, t717);
            let r718 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r718);
            return RegfileExit::jump(c.r64(1));
        }
        301 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f2f46); }
            let Some(t719) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008f2f4a); };
            c.w64(8, t719);
            let Some(t720) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2f51); };
            c.w32(8, t720);
            let r721 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r721);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f2f58); }
            let Some(t722) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x70008f2f5a); };
            c.w64(3, t722);
            c.w32(6, 0);
            c.logic(0, 32);
            c.w32(0, 0x1u64);
            return c.call(0x700086ed74, 0x70008f2f6d, 0x70008f2f68);
        }
        302 => {
            return RegfileExit::fault(0x70008f2f6d);
        }
        303 => {
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t723) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f2f76); };
            c.w32(3, t723);
            let Some(t724) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2f7c); };
            c.w32(2, t724);
            let r725 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r725);
            let Some(t726) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f2f84); };
            c.w64(1, t726);
            let Some(t727) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f2f88); };
            c.w32(3, t727);
            let r728 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r728);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f2f8f); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f2f93); }
            let Some(t729) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f2f94); };
            if !c.push64(t729) { return RegfileExit::fault(0x70008f2f94); }
            let Some(t730) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f2f97); };
            if !c.push64(t730) { return RegfileExit::fault(0x70008f2f97); }
            c.w32(0, 0x2u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f2f9f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f2fa0); }
            let Some(t731) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x70008f2fa4); };
            c.w64(3, t731);
            c.w32(0, 0x4u64);
            c.w64(6, c.r64(1));
            return c.call(0x700086ed74, 0x70008f2fb8, 0x70008f2fb3);
        }
        304 => {
            let Some(t732) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2fb8); };
            c.w64(8, t732);
            let Some(t733) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f2fbc); };
            c.w64(11, t733);
            let Some(t734) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f2fc0); };
            c.w64(9, t734);
            let Some(t735) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f2fc4); };
            c.w32(3, t735);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 306; continue; }
            blk = 305; continue;
        }
        305 => {
            let Some(t736) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2fd9); };
            c.w64(15, t736);
            let Some(t737) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f2fdd); };
            c.w64(1, t737);
            let Some(t738) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f2fe1); };
            c.w64(5, t738);
            c.w64(12, c.r64(11));
            let r739 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r739);
            return RegfileExit::jump(c.r64(1));
        }
        306 => {
            let r740 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r740);
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f2ff2); }
            let Some(t741) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f2ffa); };
            c.w32(8, t741);
            let Some(t742) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f2fff); };
            c.w64(15, t742);
            let Some(t743) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3003); };
            c.w64(1, t743);
            let Some(t744) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3007); };
            c.w64(5, t744);
            c.w64(12, c.r64(11));
            let r745 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r745);
            return RegfileExit::jump(c.r64(1));
        }
        307 => {
            return RegfileExit::fault(0x70008f3014);
        }
        308 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f3015); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f3019); }
            let r746 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r746);
            c.w64(11, c.r64(9).wrapping_add(0x3));
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t747) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f3027); };
            c.w32(3, t747);
            let Some(t748) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f302d); };
            c.w32(2, t748);
            let r749 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r749);
            let Some(t750) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f3035); };
            c.w64(1, t750);
            let Some(t751) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3039); };
            c.w32(3, t751);
            let r752 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r752);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f3040); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f3044); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f3048); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(11)) { return RegfileExit::fault(0x70008f304c); }
            c.w32(0, 0xcu64);
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(3));
            return c.call(0x7000804df4, 0x70008f3060, 0x70008f305b);
        }
        309 => {
            let Some(t753) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3060); };
            c.w64(8, t753);
            let Some(t754) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3064); };
            c.w64(11, t754);
            let Some(t755) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f3068); };
            c.w64(9, t755);
            let Some(t756) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f306c); };
            c.w32(3, t756);
            c.w64(2, c.r64(3).wrapping_mul(8));
            c.cmp(c.r32(3), 0xb4u64, 32);
            if c.cc_ae() { blk = 311; continue; }
            blk = 310; continue;
        }
        310 => {
            let Some(t757) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3081); };
            c.w64(15, t757);
            let Some(t758) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3085); };
            c.w64(1, t758);
            let Some(t759) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3089); };
            c.w64(5, t759);
            c.w64(12, c.r64(11));
            let r760 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r760);
            return RegfileExit::jump(c.r64(1));
        }
        311 => {
            if !c.st64(c.r64(8).wrapping_add(c.r64(2)).wrapping_sub(0x648), c.r64(0)) { return RegfileExit::fault(0x70008f3096); }
            let r761 = c.add(c.r64(9), 0x1u64, 64);
            c.w64(9, r761);
            let Some(t762) = c.ld8(c.r64(9).wrapping_add(c.r64(11))) else { return RegfileExit::fault(0x70008f30a2); };
            c.w32(8, t762);
            let Some(t763) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f30a7); };
            c.w64(15, t763);
            let Some(t764) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f30ab); };
            c.w64(1, t764);
            let Some(t765) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f30af); };
            c.w64(5, t765);
            c.w64(12, c.r64(11));
            let r766 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r766);
            return RegfileExit::jump(c.r64(1));
        }
        312 => {
            return RegfileExit::jump(0x70008f30c0);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f30d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(8, c.r64(5));
            let Some(t0) = c.ld64(c.r64(8).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f30d7); };
            c.w64(11, t0);
            let Some(t1) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f30db); };
            c.w32(11, t1);
            let r2 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r2);
            let Some(t3) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f30e2); };
            c.w32(11, t3);
            let r4 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r4);
            let Some(t5) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f30e9); };
            c.w32(0, t5);
            let Some(t6) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f30ed); };
            c.cmp(t6, c.r32(0), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t7) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f30f6); };
            c.w64(11, t7);
            blk = 2; continue;
        }
        2 => {
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f30fd); };
            c.w64(0, t8);
            let Some(t9) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f3104); };
            c.cmp(t9, c.r32(11), 32);
            if c.cc_e() { blk = 254; continue; }
            blk = 3; continue;
        }
        3 => {
            let Some(t10) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f3111); };
            c.w32(3, t10);
            if let Some(sr) = c.shl(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.w64(2, c.r64(3).wrapping_add(c.r64(3)));
            let Some(t11) = c.ld32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f311e); };
            c.w32(1, t11);
            let r12 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r12);
            c.logic(c.r8(1) & 0x1u64, 8);
            if c.cc_e() { blk = 7; continue; }
            blk = 4; continue;
        }
        4 => {
            c.cmp(c.r32(1), 0x3u64, 32);
            if c.cc_e() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            let r13 = c.r64(1) & 0xfffffffffffffffdu64;
            c.w64(1, r13);
            c.logic(r13, 64);
            let Some(t14) = c.ld32(c.r64(1).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f3138); };
            c.w32(0, t14);
            let r15 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r15);
            let Some(t16) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f313e); };
            c.cmp(t16, c.r32(0), 32);
            if c.cc_e() { blk = 13; continue; }
            blk = 6; continue;
        }
        6 => {
            let r17 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r17);
            let Some(t18) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f314f); };
            c.w32(8, t18);
            let Some(t19) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3154); };
            c.w64(1, t19);
            return RegfileExit::jump(c.r64(1));
        }
        7 => {
            let Some(t20) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f315a); };
            c.w64(8, t20);
            let Some(t21) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f315e); };
            c.w32(8, t21);
            let r22 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r22);
            let Some(t23) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f3165); };
            c.w32(8, t23);
            let r24 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r24);
            let Some(t25) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f316c); };
            c.w32(11, t25);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.cmp(c.r64(11), 0xa7u64, 64);
            if c.cc_a() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008f317f);
        }
        9 => {
            let Some(t26) = c.ld32(c.r64(8).wrapping_add(0x2a3)) else { return RegfileExit::fault(0x70008f3180); };
            c.w32(8, t26);
            let r27 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r27);
            c.w64(11, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t28) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f318d); };
            c.w32(0, t28);
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            c.w64(3, c.r64(11));
            let r29 = c.r64(3) & 0xfffu64;
            c.w64(3, r29);
            c.logic(r29, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.w64(2, c.r64(3).wrapping_add(0x1));
            c.cmp(c.r64(2), c.r64(0), 64);
            if c.cc_ae() { blk = 253; continue; }
            blk = 10; continue;
        }
        10 => {
            let Some(t30) = c.ld32(c.r64(8).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f31b1); };
            c.w32(8, t30);
            let r31 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r31);
            let Some(t32) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f31b9); };
            c.w32(0, t32);
            if let Some(sr) = c.shr(c.r64(11), 0xcu64, 64) { c.w64(11, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r33 = c.r64(11) & 0x3ffffu64;
            c.w64(11, r33);
            c.logic(r33, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(11), c.r64(0), 64);
            if c.cc_b() { blk = 12; continue; }
            blk = 11; continue;
        }
        11 => {
            return RegfileExit::fault(0x70008f31d3);
        }
        12 => {
            let r34 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r34);
            let Some(t35) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f31d8); };
            c.w32(0, t35);
            let Some(t36) = c.ld32(c.r64(8).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f31dd); };
            c.w32(8, t36);
            let r37 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r37);
            let Some(t38) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f31e5); };
            c.w64(1, t38);
            c.w64(0, c.r64(8));
            return RegfileExit::jump(c.r64(1));
        }
        13 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f31ee); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f31f2); }
            let r39 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r39);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f31f8); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f31fc); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008f3200); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008f3204); }
            blk = 251; continue;
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f320d); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f3211); }
            let r40 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r40);
            let Some(t41) = c.ld32(c.r64(11).wrapping_add(c.r64(2)).wrapping_add(0x23)) else { return RegfileExit::fault(0x70008f3217); };
            c.w32(7, t41);
            let r42 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r42);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f321f); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f3223); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f3227); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(12)) { return RegfileExit::fault(0x70008f322b); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(11)) { return RegfileExit::fault(0x70008f322f); }
            if !c.st64(c.r64(5).wrapping_sub(0x40), c.r64(3)) { return RegfileExit::fault(0x70008f3233); }
            let Some(t43) = c.ld32(c.r64(13).wrapping_add(0xca8)) else { return RegfileExit::fault(0x70008f3237); };
            c.cmp(t43, c.r32(7), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 15; continue;
        }
        15 => {
            let Some(t44) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f3244); };
            c.w64(2, t44);
            let Some(t45) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3248); };
            c.w32(1, t45);
            let r46 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r46);
            let Some(t47) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f324e); };
            c.w32(1, t47);
            let r48 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r48);
            let Some(t49) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3254); };
            c.w32(6, t49);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            if !c.st64(c.r64(5).wrapping_sub(0x50), c.r64(2)) { return RegfileExit::fault(0x70008f325d); }
            c.cmp(c.r64(6), 0x3u64, 64);
            if c.cc_a() { blk = 17; continue; }
            blk = 16; continue;
        }
        16 => {
            return RegfileExit::fault(0x70008f3267);
        }
        17 => {
            let Some(t50) = c.ld32(c.r64(1).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f3268); };
            c.w32(11, t50);
            let r51 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r51);
            if !c.st64(c.r64(5).wrapping_sub(0x48), c.r64(11)) { return RegfileExit::fault(0x70008f326f); }
            c.cmp(c.r64(6), 0x2u64, 64);
            if c.cc_a() { blk = 19; continue; }
            blk = 18; continue;
        }
        18 => {
            return RegfileExit::fault(0x70008f3279);
        }
        19 => {
            let Some(t52) = c.ld32(c.r64(1).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f327a); };
            c.w32(1, t52);
            let r53 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r53);
            c.logic(c.r8(7) & 0x1u64, 8);
            if c.cc_e() { blk = 35; continue; }
            blk = 20; continue;
        }
        20 => {
            let Some(t54) = c.ld32(c.r64(7).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f328a); };
            c.w32(6, t54);
            let Some(t55) = c.ld32(c.r64(13).wrapping_add(0x120)) else { return RegfileExit::fault(0x70008f328d); };
            c.cmp(t55, c.r32(6), 32);
            if c.cc_e() { blk = 30; continue; }
            blk = 21; continue;
        }
        21 => {
            let Some(t56) = c.ld32(c.r64(7).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3296); };
            c.w32(6, t56);
            let r57 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r57);
            c.w32(3, 0);
            c.logic(0, 32);
            c.cmp(c.r32(6), c.r32(3), 32);
            if c.cc_e() { blk = 23; continue; }
            blk = 22; continue;
        }
        22 => {
            let Some(t58) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f32a2); };
            c.cmp(t58, c.r32(3), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 23; continue;
        }
        23 => {
            let Some(t59) = c.ld32(c.r64(7).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f32ab); };
            c.w32(3, t59);
            let Some(t60) = c.ld32(c.r64(7).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f32ae); };
            c.w32(6, t60);
            let r61 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r61);
            c.logic(c.r8(6) & 0x1u64, 8);
            if c.cc_e() { blk = 29; continue; }
            blk = 24; continue;
        }
        24 => {
            let Some(t62) = c.ld32(c.r64(13).wrapping_add(0xb0)) else { return RegfileExit::fault(0x70008f32ba); };
            c.cmp(t62, c.r32(6), 32);
            if c.cc_e() { blk = 27; continue; }
            blk = 25; continue;
        }
        25 => {
            c.cmp(c.r32(6), 0x3u64, 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 26; continue;
        }
        26 => {
            let r63 = c.r64(6) & 0xfffffffffffffffdu64;
            c.w64(6, r63);
            c.logic(r63, 64);
            blk = 28; continue;
        }
        27 => {
            c.w64(6, c.r64(1));
            blk = 28; continue;
        }
        28 => {
            c.w64(10, c.r64(3));
            c.w64(3, c.r64(6));
            c.w64(6, c.r64(10));
            blk = 36; continue;
        }
        29 => {
            let r64 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r64);
            let Some(t65) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f32e7); };
            c.w32(8, t65);
            let Some(t66) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f32ec); };
            c.w64(1, t66);
            let Some(t67) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f32f0); };
            c.w64(5, t67);
            c.w64(0, c.r64(6));
            let r68 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r68);
            return RegfileExit::jump(c.r64(1));
        }
        30 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x3));
            let Some(t69) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f3305); };
            c.w32(6, t69);
            let Some(t70) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f330b); };
            c.w32(9, t70);
            let r71 = c.add(c.r64(9), c.r64(14), 64);
            c.w64(9, r71);
            let Some(t72) = c.ld32(c.r64(9).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3313); };
            c.w32(6, t72);
            let r73 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r73);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f331b); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f331f); }
            c.w64(10, c.r64(6));
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(10));
            c.w64(10, c.r64(2));
            c.w64(2, c.r64(6));
            c.w64(6, c.r64(10));
            c.w64(0, c.r64(3));
            let Some(t74) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f3338); };
            c.w64(3, t74);
            let Some(t75) = c.ld32(c.r64(7).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f333c); };
            c.logic(t75 & 0x20000000u64, 32);
            if c.cc_ne() { blk = 32; continue; }
            blk = 31; continue;
        }
        31 => {
            let r76 = c.add(c.r64(7), 0x3fu64, 64);
            c.w64(7, r76);
            blk = 33; continue;
        }
        32 => {
            let Some(t77) = c.ld32(c.r64(7).wrapping_add(0x1f)) else { return RegfileExit::fault(0x70008f334b); };
            c.w32(7, t77);
            let Some(t78) = c.ld64(c.r64(13).wrapping_add(c.r64(7).wrapping_mul(8)).wrapping_add(0x3098)) else { return RegfileExit::fault(0x70008f334e); };
            c.w64(7, t78);
            blk = 33; continue;
        }
        33 => {
            return c.call(c.r64(7), 0x70008f3358, 0x70008f3356);
        }
        34 => {
            let Some(t79) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3358); };
            c.w64(8, t79);
            let Some(t80) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f335c); };
            c.w64(8, t80);
            let Some(t81) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f3360); };
            c.w64(9, t81);
            let Some(t82) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f3364); };
            c.w32(11, t82);
            let Some(t83) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3369); };
            c.w64(15, t83);
            let Some(t84) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f336d); };
            c.w64(1, t84);
            let Some(t85) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3371); };
            c.w64(5, t85);
            c.w64(12, c.r64(8));
            let r86 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r86);
            return RegfileExit::jump(c.r64(1));
        }
        35 => {
            c.w64(3, c.r64(1));
            c.w64(6, c.r64(7));
            blk = 36; continue;
        }
        36 => {
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.w64(2, c.r64(6));
            let r87 = c.r64(2) & 0xfu64;
            c.w64(2, r87);
            c.logic(r87, 64);
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(3)) { return RegfileExit::fault(0x70008f3391); }
            c.cmp(c.r64(2), 0x4u64, 64);
            if c.cc_e() { blk = 181; continue; }
            blk = 37; continue;
        }
        37 => {
            c.cmp(c.r64(2), 0x5u64, 64);
            if c.cc_e() { blk = 180; continue; }
            blk = 38; continue;
        }
        38 => {
            c.cmp(c.r64(2), 0xdu64, 64);
            if c.cc_e() { blk = 179; continue; }
            blk = 39; continue;
        }
        39 => {
            c.cmp(c.r64(2), 0x2u64, 64);
            if c.cc_e() { blk = 118; continue; }
            blk = 40; continue;
        }
        40 => {
            c.cmp(c.r64(2), 0x6u64, 64);
            if c.cc_e() { blk = 116; continue; }
            blk = 41; continue;
        }
        41 => {
            c.cmp(c.r64(2), 0x7u64, 64);
            if c.cc_e() { blk = 111; continue; }
            blk = 42; continue;
        }
        42 => {
            c.cmp(c.r64(2), 0x8u64, 64);
            c.w8(11, (c.cc_e() as u64));
            c.w32(11, c.r8(11));
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 100; continue; }
            blk = 43; continue;
        }
        43 => {
            c.cmp(c.r64(2), 0x9u64, 64);
            if c.cc_e() { blk = 100; continue; }
            blk = 44; continue;
        }
        44 => {
            c.cmp(c.r64(2), 0x3u64, 64);
            if c.cc_e() { blk = 59; continue; }
            blk = 45; continue;
        }
        45 => {
            c.cmp(c.r64(2), 0xbu64, 64);
            if c.cc_e() { blk = 114; continue; }
            blk = 46; continue;
        }
        46 => {
            c.cmp(c.r64(2), 0xcu64, 64);
            if c.cc_e() { blk = 56; continue; }
            blk = 47; continue;
        }
        47 => {
            c.cmp(c.r64(2), 0xeu64, 64);
            if c.cc_e() { blk = 50; continue; }
            blk = 48; continue;
        }
        48 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x3));
            let Some(t88) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f341d); };
            c.w32(2, t88);
            let Some(t89) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3423); };
            c.w32(1, t89);
            let r90 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r90);
            let Some(t91) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f342b); };
            c.w32(2, t91);
            let r92 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r92);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f3432); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f3436); }
            let Some(t93) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f3437); };
            if !c.push64(t93) { return RegfileExit::fault(0x70008f3437); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f343a); }
            let Some(t94) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f343b); };
            if !c.push64(t94) { return RegfileExit::fault(0x70008f343b); }
            let Some(t95) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f343e); };
            if !c.push64(t95) { return RegfileExit::fault(0x70008f343e); }
            if !c.st64(c.r64(5).wrapping_sub(0x58), c.r64(0)) { return RegfileExit::fault(0x70008f3441); }
            let Some(t96) = c.ld64(c.r64(13).wrapping_add(0x2070)) else { return RegfileExit::fault(0x70008f3445); };
            c.w64(3, t96);
            let Some(t97) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f344c); };
            c.w64(6, t97);
            c.w32(0, 0x5u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008f345d, 0x70008f3458);
        }
        49 => {
            let Some(t98) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f345d); };
            c.w64(8, t98);
            let Some(t99) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3461); };
            c.w64(8, t99);
            let Some(t100) = c.ld64(c.r64(5).wrapping_sub(0x58)) else { return RegfileExit::fault(0x70008f3465); };
            c.w64(9, t100);
            let Some(t101) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f3469); };
            c.w32(11, t101);
            let Some(t102) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f346e); };
            c.w64(15, t102);
            let Some(t103) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3472); };
            c.w64(1, t103);
            let Some(t104) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3476); };
            c.w64(5, t104);
            c.w64(12, c.r64(8));
            let r105 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r105);
            return RegfileExit::jump(c.r64(1));
        }
        50 => {
            let Some(t106) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f3483); };
            c.w32(11, t106);
            let r107 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r107);
            let Some(t108) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f348a); };
            c.w32(11, t108);
            let r109 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r109);
            let Some(t110) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3491); };
            c.w32(0, t110);
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w64(0, (c.r32(0) as u32 as i32 as i64 as u64));
            let r111 = c.r64(6) & 0x1ffffffu64;
            c.w64(6, r111);
            c.logic(r111, 64);
            if let Some(sr) = c.sar(c.r64(0), 1u64, 64) { c.w64(0, sr); }
            c.cmp(c.r64(6), c.r64(0), 64);
            if c.cc_b() { blk = 52; continue; }
            blk = 51; continue;
        }
        51 => {
            return RegfileExit::fault(0x70008f34ab);
        }
        52 => {
            let Some(t112) = c.ld32(c.r64(11).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f34ac); };
            c.w32(11, t112);
            let r113 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r113);
            let Some(t114) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f34b4); };
            c.w32(0, t114);
            let r115 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r115);
            c.w64(11, c.r64(9).wrapping_add(0x3));
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(11)) { return RegfileExit::fault(0x70008f34bf); }
            let Some(t116) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f34c3); };
            c.cmp(t116, c.r32(0), 32);
            if c.cc_e() { blk = 54; continue; }
            blk = 53; continue;
        }
        53 => {
            let Some(t117) = c.ld8(c.r64(11).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f34cc); };
            c.w32(8, t117);
            let Some(t118) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f34d1); };
            c.w64(1, t118);
            let Some(t119) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f34d5); };
            c.w64(5, t119);
            c.w64(9, c.r64(11));
            let r120 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r120);
            return RegfileExit::jump(c.r64(1));
        }
        54 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t121) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f34e6); };
            c.w32(3, t121);
            let Some(t122) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f34ec); };
            c.w32(2, t122);
            let r123 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r123);
            let Some(t124) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f34f4); };
            c.w32(3, t124);
            let r125 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r125);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f34fb); }
            c.w32(0, 0x174u64);
            if !c.push64(c.r64(0)) { return RegfileExit::fault(0x70008f3504); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f3505); }
            let Some(t126) = c.ld64(c.r64(13).wrapping_add(0x2220)) else { return RegfileExit::fault(0x70008f3506); };
            c.w64(3, t126);
            c.w32(0, 0x2u64);
            let Some(t127) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f3512); };
            c.w64(6, t127);
            return c.call(0x700086ed74, 0x70008f351b, 0x70008f3516);
        }
        55 => {
            let Some(t128) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f351b); };
            c.w64(8, t128);
            let Some(t129) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f351f); };
            c.w64(8, t129);
            let Some(t130) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f3523); };
            c.w64(9, t130);
            let Some(t131) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f3527); };
            c.w32(11, t131);
            let Some(t132) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f352c); };
            c.w64(15, t132);
            let Some(t133) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3530); };
            c.w64(1, t133);
            let Some(t134) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3534); };
            c.w64(5, t134);
            c.w64(12, c.r64(8));
            let r135 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r135);
            return RegfileExit::jump(c.r64(1));
        }
        56 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x3));
            let Some(t136) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f3549); };
            c.w32(2, t136);
            let Some(t137) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f354f); };
            c.w32(1, t137);
            let r138 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r138);
            let Some(t139) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3557); };
            c.w32(2, t139);
            let r140 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r140);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f355e); }
            if !c.st64(c.r64(5).wrapping_sub(0x38), c.r64(0)) { return RegfileExit::fault(0x70008f3562); }
            let Some(t141) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f3566); };
            c.w64(6, t141);
            c.w64(0, c.r64(3));
            c.w64(3, c.r64(2));
            let Some(t142) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f3570); };
            c.w64(1, t142);
            c.w32(2, 0x2u64);
            c.w64(11, c.r64(2));
            return c.call(0x70008bc174, 0x70008f3581, 0x70008f357c);
        }
        57 => {
            let Some(t143) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3581); };
            c.w64(8, t143);
            let Some(t144) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3585); };
            c.w64(8, t144);
            let Some(t145) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f3589); };
            c.w64(9, t145);
            let Some(t146) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f358d); };
            c.w32(11, t146);
            let Some(t147) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3592); };
            c.w64(15, t147);
            let Some(t148) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3596); };
            c.w64(1, t148);
            let Some(t149) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f359a); };
            c.w64(5, t149);
            c.w64(12, c.r64(8));
            let r150 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r150);
            return RegfileExit::jump(c.r64(1));
        }
        58 => {
            blk = 114; continue;
        }
        59 => {
            let Some(t151) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f35ac); };
            c.w32(11, t151);
            let r152 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r152);
            let Some(t153) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f35b3); };
            c.w32(2, t153);
            let Some(t154) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f35b6); };
            c.cmp(t154, c.r32(11), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 60; continue;
        }
        60 => {
            let Some(t155) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f35c3); };
            c.w32(1, t155);
            let Some(t156) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f35c9); };
            c.w32(6, t156);
            let r157 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r157);
            let Some(t158) = c.ld32(c.r64(6).wrapping_add(c.r64(1).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f35d1); };
            c.w32(1, t158);
            let r159 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r159);
            c.logic(c.r8(2) & 0x2u64, 8);
            if c.cc_e() { blk = 98; continue; }
            blk = 61; continue;
        }
        61 => {
            let Some(t160) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f35e1); };
            c.w32(2, t160);
            let r161 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r161);
            let Some(t162) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f35e8); };
            c.cmp(t162, c.r32(2), 32);
            if c.cc_ne() { blk = 83; continue; }
            blk = 62; continue;
        }
        62 => {
            let Some(t163) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f35f5); };
            c.w32(2, t163);
            let r164 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r164);
            let Some(t165) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f35fc); };
            c.w32(6, t165);
            let r166 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r166);
            let Some(t167) = c.ld8(c.r64(6).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f3602); };
            c.logic(t167 & 0x2u64, 8);
            if c.cc_ne() { blk = 81; continue; }
            blk = 63; continue;
        }
        63 => {
            let Some(t168) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f360c); };
            c.cmp(t168, c.r32(6), 32);
            if c.cc_e() { blk = 65; continue; }
            blk = 64; continue;
        }
        64 => {
            c.w64(11, c.r64(0));
            blk = 98; continue;
        }
        65 => {
            let Some(t169) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f361d); };
            c.w64(0, t169);
            let Some(t170) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x70008f3624); };
            c.cmp(t170, 0x20u64, 8);
            if c.cc_e() { blk = 79; continue; }
            blk = 66; continue;
        }
        66 => {
            let Some(t171) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f362d); };
            c.w32(0, t171);
            let Some(t172) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f3630); };
            c.cmp(t172, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 67; continue;
        }
        67 => {
            let Some(t173) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f363d); };
            c.w32(0, t173);
            let r174 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r174);
            let Some(t175) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3643); };
            c.w32(1, t175);
            let Some(t176) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f3646); };
            c.cmp(t176, c.r32(1), 32);
            if c.cc_ne() { blk = 69; continue; }
            blk = 68; continue;
        }
        68 => {
            let Some(t177) = c.ld32(c.r64(0).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f364f); };
            c.w32(0, t177);
            let r178 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r178);
            let Some(t179) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3655); };
            c.w32(1, t179);
            let Some(t180) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f3658); };
            c.cmp(t180, c.r32(1), 32);
            if c.cc_e() { blk = 68; continue; }
            blk = 69; continue;
        }
        69 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 70; continue;
        }
        70 => {
            let Some(t181) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3669); };
            c.w32(1, t181);
            let r182 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r182);
            let Some(t183) = c.ld16(c.r64(1).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f366f); };
            c.w32(6, t183);
            c.w32(7, c.r64(6).wrapping_sub(0x80c));
            c.cmp(c.r32(7), 0xeu64, 32);
            if c.cc_be() { blk = 75; continue; }
            blk = 71; continue;
        }
        71 => {
            c.w32(0, c.r16(6));
            c.cmp(c.r32(0), 0x82au64, 32);
            if c.cc_e() { blk = 74; continue; }
            blk = 72; continue;
        }
        72 => {
            let Some(t184) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f3688); };
            c.cmp(t184, c.r32(1), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 73; continue;
        }
        73 => {
            blk = 76; continue;
        }
        74 => {
            c.w64(3, c.r64(11));
            blk = 76; continue;
        }
        75 => {
            c.w64(3, c.r64(0));
            blk = 76; continue;
        }
        76 => {
            let Some(t185) = c.ld32(c.r64(3).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f369f); };
            c.w32(11, t185);
            let r186 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r186);
            let Some(t187) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f36a6); };
            c.w32(11, t187);
            let r188 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r188);
            let Some(t189) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f36ad); };
            c.w32(0, t189);
            let r190 = c.sub(c.r32(0), 0x89u64, 32);
            c.w32(0, r190);
            c.cmp(c.r32(0), 0x9u64, 32);
            if c.cc_a() { blk = 251; continue; }
            blk = 77; continue;
        }
        77 => {
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t191) = c.ld32(c.r64(11).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f36c4); };
            c.w32(6, t191);
            let r192 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r192);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f36cb); }
            let Some(t193) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f36cf); };
            if !c.push64(t193) { return RegfileExit::fault(0x70008f36cf); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f36d9, 0x70008f36d4);
        }
        78 => {
            c.w64(11, c.r64(0));
            let Some(t194) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f36dc); };
            c.w64(15, t194);
            let Some(t195) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f36e0); };
            c.w64(9, t195);
            let Some(t196) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f36e4); };
            c.w64(8, t196);
            blk = 98; continue;
        }
        79 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f36f1); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f36f5); }
            if !c.push64(c.r64(1)) { return RegfileExit::fault(0x70008f36f6); }
            let Some(t197) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f36f7); };
            if !c.push64(t197) { return RegfileExit::fault(0x70008f36f7); }
            let Some(t198) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f36fa); };
            c.w64(6, t198);
            let Some(t199) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f36fe); };
            c.w64(3, t199);
            c.w32(0, 0x3u64);
            c.w64(2, c.r64(0));
            return c.call(0x700086ed74, 0x70008f3712, 0x70008f370d);
        }
        80 => {
            c.w64(11, c.r64(0));
            let Some(t200) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3715); };
            c.w64(15, t200);
            let Some(t201) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f3719); };
            c.w64(9, t201);
            let Some(t202) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f371d); };
            c.w64(8, t202);
            blk = 98; continue;
        }
        81 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f372d); }
            let Some(t203) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f3731); };
            if !c.push64(t203) { return RegfileExit::fault(0x70008f3731); }
            let Some(t204) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f3734); };
            c.w64(6, t204);
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            c.w64(1, c.r64(0));
            return c.call(0x70007f2274, 0x70008f3745, 0x70008f3740);
        }
        82 => {
            c.w64(11, c.r64(0));
            let Some(t205) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3748); };
            c.w64(15, t205);
            let Some(t206) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f374c); };
            c.w64(9, t206);
            let Some(t207) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3750); };
            c.w64(8, t207);
            blk = 98; continue;
        }
        83 => {
            let Some(t208) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3759); };
            c.w32(0, t208);
            let r209 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r209);
            let Some(t210) = c.ld16(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f375f); };
            c.w32(1, t210);
            c.cmp(c.r16(1), 0x834u64, 16);
            if c.cc_e() { blk = 97; continue; }
            blk = 84; continue;
        }
        84 => {
            c.w32(6, c.r64(1).wrapping_sub(0x80c));
            c.cmp(c.r32(6), 0xeu64, 32);
            if c.cc_be() { blk = 90; continue; }
            blk = 85; continue;
        }
        85 => {
            c.cmp(c.r16(1), 0x411u64, 16);
            if c.cc_ne() { blk = 251; continue; }
            blk = 86; continue;
        }
        86 => {
            let Some(t211) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3784); };
            c.w32(0, t211);
            let Some(t212) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f3788); };
            c.cmp(t212, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 87; continue;
        }
        87 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 88; continue;
        }
        88 => {
            let Some(t213) = c.ld16(c.r64(2).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f379f); };
            c.cmp(t213, 0x40u64, 16);
            if c.cc_ae() { blk = 251; continue; }
            blk = 89; continue;
        }
        89 => {
            let Some(t214) = c.ld32(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f37aa); };
            c.w32(11, t214);
            let r215 = c.shl(c.r32(11), 1u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r215);
            blk = 98; continue;
        }
        90 => {
            let Some(t216) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f37b6); };
            c.w32(2, t216);
            let Some(t217) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f37ba); };
            c.cmp(t217, c.r32(2), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 91; continue;
        }
        91 => {
            let Some(t218) = c.ld8(c.r64(0).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f37c7); };
            c.w32(0, t218);
            c.w64(2, c.r64(0));
            let r219 = c.r32(2) & 0xc0u64;
            c.w32(2, r219);
            c.logic(r219, 32);
            c.cmp(c.r32(2), 0xc0u64, 32);
            if c.cc_e() { blk = 93; continue; }
            blk = 92; continue;
        }
        92 => {
            let Some(t220) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f37dc); };
            c.w32(11, t220);
            let r221 = c.r32(11) & 0x1fu64;
            c.w32(11, r221);
            c.logic(r221, 32);
            let r222 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r222);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_a() { blk = 251; continue; }
            blk = 93; continue;
        }
        93 => {
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_ne() { blk = 251; continue; }
            blk = 94; continue;
        }
        94 => {
            let Some(t223) = c.ld32(c.r64(3).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f37fa); };
            c.w32(11, t223);
            let r224 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r224);
            let Some(t225) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f3801); };
            c.cmp(t225, c.r32(11), 32);
            if c.cc_e() { blk = 251; continue; }
            blk = 95; continue;
        }
        95 => {
            let Some(t226) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f380e); };
            c.w32(0, t226);
            let Some(t227) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f3812); };
            c.cmp(t227, c.r32(0), 32);
            if c.cc_ne() { blk = 98; continue; }
            blk = 96; continue;
        }
        96 => {
            let Some(t228) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f381b); };
            c.w32(11, t228);
            let r229 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r229);
            blk = 98; continue;
        }
        97 => {
            let Some(t230) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3824); };
            c.w32(0, t230);
            let Some(t231) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f3828); };
            c.cmp(t231, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 98; continue;
        }
        98 => {
            let r232 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r232);
            let Some(t233) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3839); };
            c.w64(8, t233);
            let Some(t234) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f383d); };
            c.w32(0, t234);
            let Some(t235) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3842); };
            c.w64(1, t235);
            let Some(t236) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3846); };
            c.w64(5, t236);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r237 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r237);
            return RegfileExit::jump(c.r64(1));
        }
        99 => {
            blk = 251; continue;
        }
        100 => {
            let Some(t238) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f385b); };
            c.w64(0, t238);
            let Some(t239) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x70008f3862); };
            c.cmp(t239, 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 101; continue;
        }
        101 => {
            c.logic(c.r8(6) & 0x10u64, 8);
            if c.cc_ne() { blk = 103; continue; }
            blk = 102; continue;
        }
        102 => {
            let Some(t240) = c.ld32(c.r64(7).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f3871); };
            c.w32(0, t240);
            let r241 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r241);
            blk = 104; continue;
        }
        103 => {
            let Some(t242) = c.ld32(c.r64(7).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f3879); };
            c.w32(0, t242);
            let r243 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r243);
            blk = 104; continue;
        }
        104 => {
            c.cmp(c.r32(0), 0x3u64, 32);
            if c.cc_ne() { blk = 106; continue; }
            blk = 105; continue;
        }
        105 => {
            return RegfileExit::fault(0x70008f3884);
        }
        106 => {
            let r244 = c.r64(0) & 0xfffffffffffffffdu64;
            c.w64(0, r244);
            c.logic(r244, 64);
            let Some(t245) = c.ld32(c.r64(3).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3889); };
            c.w32(2, t245);
            let r246 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r246);
            let Some(t247) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f388f); };
            c.w32(3, t247);
            let r248 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r248);
            let Some(t249) = c.ld64(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3895); };
            c.w64(2, t249);
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_ne() { blk = 108; continue; }
            blk = 107; continue;
        }
        107 => {
            let Some(t250) = c.ld32(c.r64(1).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f389f); };
            c.w32(11, t250);
            let r251 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r251);
            let Some(t252) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f38a6); };
            c.w32(1, t252);
            let r253 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r253);
            blk = 108; continue;
        }
        108 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let r254 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r254);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f38b5); }
            let Some(t255) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f38b9); };
            if !c.push64(t255) { return RegfileExit::fault(0x70008f38b9); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f38bc); }
            c.w64(6, c.r64(0));
            c.w64(7, c.r64(1));
            c.w32(1, 0);
            c.logic(0, 32);
            c.w64(11, c.r64(1));
            return c.call(0x70007fe8d4, 0x70008f38d0, 0x70008f38cb);
        }
        109 => {
            let Some(t256) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f38d0); };
            c.w64(8, t256);
            let Some(t257) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f38d4); };
            c.w64(8, t257);
            let Some(t258) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f38d8); };
            c.w64(9, t258);
            let Some(t259) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f38dc); };
            c.w32(11, t259);
            let Some(t260) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f38e1); };
            c.w64(15, t260);
            let Some(t261) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f38e5); };
            c.w64(1, t261);
            let Some(t262) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f38e9); };
            c.w64(5, t262);
            c.w64(12, c.r64(8));
            let r263 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r263);
            return RegfileExit::jump(c.r64(1));
        }
        110 => {
            blk = 114; continue;
        }
        111 => {
            let Some(t264) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f38f8); };
            c.w64(0, t264);
            let Some(t265) = c.ld8(c.r64(0)) else { return RegfileExit::fault(0x70008f38ff); };
            c.cmp(t265, 0x20u64, 8);
            if c.cc_e() { blk = 114; continue; }
            blk = 112; continue;
        }
        112 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let r266 = c.r64(6) & 0x3ffu64;
            c.w64(6, r266);
            c.logic(r266, 64);
            let r267 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r267);
            let r268 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r268);
            let Some(t269) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f391b); };
            c.w32(1, t269);
            let r270 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r270);
            let Some(t271) = c.ld32(c.r64(1).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f3921); };
            c.w32(1, t271);
            let r272 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r272);
            let Some(t273) = c.ld32(c.r64(1).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f3927); };
            c.w32(2, t273);
            let r274 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r274);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f392e); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f3932); }
            c.w64(1, c.r64(3));
            c.w64(3, c.r64(2));
            c.w64(2, c.r64(11));
            let Some(t275) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f393f); };
            c.w64(6, t275);
            return c.call(0x70007fea14, 0x70008f3948, 0x70008f3943);
        }
        113 => {
            let Some(t276) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3948); };
            c.w64(8, t276);
            let Some(t277) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f394c); };
            c.w64(8, t277);
            let Some(t278) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f3950); };
            c.w64(9, t278);
            let Some(t279) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f3954); };
            c.w32(11, t279);
            let Some(t280) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3959); };
            c.w64(15, t280);
            let Some(t281) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f395d); };
            c.w64(1, t281);
            let Some(t282) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3961); };
            c.w64(5, t282);
            c.w64(12, c.r64(8));
            let r283 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r283);
            return RegfileExit::jump(c.r64(1));
        }
        114 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x3));
            let Some(t284) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f3976); };
            c.w32(3, t284);
            let Some(t285) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f397c); };
            c.w32(2, t285);
            let r286 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r286);
            let Some(t287) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3984); };
            c.w32(3, t287);
            let r288 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r288);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f398b); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f398f); }
            let Some(t289) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f3990); };
            if !c.push64(t289) { return RegfileExit::fault(0x70008f3990); }
            let Some(t290) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f3993); };
            if !c.push64(t290) { return RegfileExit::fault(0x70008f3993); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f3996); }
            let Some(t291) = c.ld64(c.r64(13).wrapping_add(0x2048)) else { return RegfileExit::fault(0x70008f399a); };
            c.w64(3, t291);
            c.w32(0, 0x3u64);
            let Some(t292) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f39a6); };
            c.w64(6, t292);
            return c.call(0x700086ed74, 0x70008f39af, 0x70008f39aa);
        }
        115 => {
            let Some(t293) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f39af); };
            c.w64(8, t293);
            let Some(t294) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f39b3); };
            c.w64(8, t294);
            let Some(t295) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f39b7); };
            c.w64(9, t295);
            let Some(t296) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f39bb); };
            c.w32(11, t296);
            let Some(t297) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f39c0); };
            c.w64(15, t297);
            let Some(t298) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f39c4); };
            c.w64(1, t298);
            let Some(t299) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f39c8); };
            c.w64(5, t299);
            c.w64(12, c.r64(8));
            let r300 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r300);
            return RegfileExit::jump(c.r64(1));
        }
        116 => {
            if let Some(sr) = c.shr(c.r64(6), 0x6u64, 64) { c.w64(6, sr); }
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let r301 = c.r64(6) & 0x3ffu64;
            c.w64(6, r301);
            c.logic(r301, 64);
            let r302 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r302);
            let r303 = c.imul2(c.r64(6), 0x3u64, 64);
            c.w64(2, r303);
            let Some(t304) = c.ld32(c.r64(3).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f39ec); };
            c.w32(3, t304);
            let r305 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r305);
            let Some(t306) = c.ld32(c.r64(3).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f39f2); };
            c.w32(3, t306);
            let r307 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r307);
            let Some(t308) = c.ld32(c.r64(3).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x17)) else { return RegfileExit::fault(0x70008f39f8); };
            c.w32(3, t308);
            let r309 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r309);
            let Some(t310) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f39ff); };
            c.w32(7, t310);
            let r311 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r311);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f3a05); }
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f3a09); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f3a0b); }
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t312) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f3a11); };
            c.w64(6, t312);
            return c.call(0x70007f2274, 0x70008f3a1a, 0x70008f3a15);
        }
        117 => {
            let Some(t313) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3a1a); };
            c.w64(8, t313);
            let Some(t314) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3a1e); };
            c.w64(8, t314);
            let Some(t315) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f3a22); };
            c.w64(9, t315);
            let Some(t316) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f3a26); };
            c.w32(11, t316);
            let Some(t317) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3a2b); };
            c.w64(15, t317);
            let Some(t318) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3a2f); };
            c.w64(1, t318);
            let Some(t319) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3a33); };
            c.w64(5, t319);
            c.w64(12, c.r64(8));
            let r320 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r320);
            return RegfileExit::jump(c.r64(1));
        }
        118 => {
            let Some(t321) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3a40); };
            c.w32(2, t321);
            let r322 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r322);
            c.logic(c.r8(2) & 0x1u64, 8);
            if c.cc_ne() { blk = 120; continue; }
            blk = 119; continue;
        }
        119 => {
            let Some(t323) = c.ld64(c.r64(13).wrapping_add(0x418)) else { return RegfileExit::fault(0x70008f3a4b); };
            c.w64(2, t323);
            blk = 120; continue;
        }
        120 => {
            let Some(t324) = c.ld32(c.r64(2).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f3a52); };
            c.w32(1, t324);
            let Some(t325) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f3a55); };
            c.w32(6, t325);
            let Some(t326) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3a5b); };
            c.w32(7, t326);
            let r327 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r327);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            let Some(t328) = c.ld32(c.r64(7).wrapping_add(c.r64(6).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3a66); };
            c.w32(6, t328);
            let r329 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r329);
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            let r330 = c.sub(c.r64(1), 0x1u64, 64);
            c.w64(1, r330);
            let Some(t331) = c.ld32(c.r64(6).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3a74); };
            c.w32(11, t331);
            let r332 = c.shr(c.r32(11), 0x2u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r332);
            let r333 = c.r64(11) & c.r64(1);
            c.w64(11, r333);
            c.logic(r333, 64);
            c.w32(15, 0);
            c.logic(0, 32);
            blk = 121; continue;
        }
        121 => {
            c.w64(8, c.r64(11).wrapping_add(c.r64(11).wrapping_mul(2)).wrapping_add(0x5));
            c.w64(3, c.r64(8).wrapping_mul(4));
            let Some(t334) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3a8f); };
            c.w32(0, t334);
            let Some(t335) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f3a93); };
            c.cmp(t335, c.r32(0), 32);
            if c.cc_e() { blk = 178; continue; }
            blk = 122; continue;
        }
        122 => {
            c.cmp(c.r32(6), c.r32(0), 32);
            if c.cc_e() { blk = 124; continue; }
            blk = 123; continue;
        }
        123 => {
            let r336 = c.add(c.r64(15), 0x1u64, 64);
            c.w64(15, r336);
            c.w64(8, c.r64(15).wrapping_add(c.r64(11)));
            c.w64(11, c.r64(1));
            let r337 = c.r64(11) & c.r64(8);
            c.w64(11, r337);
            c.logic(r337, 64);
            blk = 121; continue;
        }
        124 => {
            let Some(t338) = c.ld32(c.r64(2).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3ab4); };
            c.w32(11, t338);
            let Some(t339) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f3ab8); };
            c.w32(0, t339);
            c.w64(11, (c.r32(11) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(11), 1u64, 64) { c.w64(11, sr); }
            let r340 = c.add(c.r64(8), 0x1u64, 64);
            c.w64(8, r340);
            c.cmp(c.r64(8), c.r64(11), 64);
            if c.cc_b() { blk = 126; continue; }
            blk = 125; continue;
        }
        125 => {
            return RegfileExit::fault(0x70008f3acb);
        }
        126 => {
            let Some(t341) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f3acc); };
            c.w32(8, t341);
            let Some(t342) = c.ld32(c.r64(2).wrapping_add(c.r64(3)).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f3ad2); };
            c.w32(11, t342);
            let r343 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r343);
            let Some(t344) = c.ld32(c.r64(7).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3ada); };
            c.w32(8, t344);
            let r345 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r345);
            c.logic(c.r8(0) & 0x2u64, 8);
            if c.cc_e() { blk = 177; continue; }
            blk = 127; continue;
        }
        127 => {
            let Some(t346) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3aea); };
            c.w32(0, t346);
            let Some(t347) = c.ld32(c.r64(13).wrapping_add(0xd68)) else { return RegfileExit::fault(0x70008f3aee); };
            c.cmp(t347, c.r32(0), 32);
            if c.cc_ne() { blk = 152; continue; }
            blk = 128; continue;
        }
        128 => {
            let Some(t348) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3afb); };
            c.w32(2, t348);
            let r349 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r349);
            let Some(t350) = c.ld32(c.r64(2).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3b02); };
            c.w32(11, t350);
            let r351 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r351);
            let Some(t352) = c.ld8(c.r64(11).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f3b09); };
            c.logic(t352 & 0x2u64, 8);
            if c.cc_ne() { blk = 150; continue; }
            blk = 129; continue;
        }
        129 => {
            let Some(t353) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f3b14); };
            c.cmp(t353, c.r32(11), 32);
            if c.cc_e() { blk = 131; continue; }
            blk = 130; continue;
        }
        130 => {
            let Some(t354) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f3b1d); };
            c.w64(11, t354);
            blk = 177; continue;
        }
        131 => {
            let Some(t355) = c.ld64(c.r64(13).wrapping_add(0x2e18)) else { return RegfileExit::fault(0x70008f3b29); };
            c.w64(11, t355);
            let Some(t356) = c.ld8(c.r64(11)) else { return RegfileExit::fault(0x70008f3b30); };
            c.cmp(t356, 0x20u64, 8);
            if c.cc_e() { blk = 148; continue; }
            blk = 132; continue;
        }
        132 => {
            let Some(t357) = c.ld32(c.r64(2).wrapping_add(0x33)) else { return RegfileExit::fault(0x70008f3b3a); };
            c.w32(8, t357);
            let Some(t358) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f3b3e); };
            c.cmp(t358, c.r32(8), 32);
            if c.cc_e() { blk = 134; continue; }
            blk = 133; continue;
        }
        133 => {
            let Some(t359) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3b47); };
            c.w64(8, t359);
            blk = 251; continue;
        }
        134 => {
            let Some(t360) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f3b50); };
            c.w64(0, t360);
            let Some(t361) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3b54); };
            c.w32(8, t361);
            let r362 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r362);
            let Some(t363) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3b5b); };
            c.w32(11, t363);
            let Some(t364) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f3b5f); };
            c.cmp(t364, c.r32(11), 32);
            if c.cc_ne() { blk = 136; continue; }
            blk = 135; continue;
        }
        135 => {
            let Some(t365) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f3b68); };
            c.w32(8, t365);
            let r366 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r366);
            let Some(t367) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3b6f); };
            c.w32(11, t367);
            let Some(t368) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f3b73); };
            c.cmp(t368, c.r32(11), 32);
            if c.cc_e() { blk = 135; continue; }
            blk = 136; continue;
        }
        136 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_e() { blk = 147; continue; }
            blk = 137; continue;
        }
        137 => {
            let Some(t369) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3b86); };
            c.w32(11, t369);
            let r370 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r370);
            let Some(t371) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3b8d); };
            c.w32(3, t371);
            c.w32(1, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(1), 0xeu64, 32);
            if c.cc_be() { blk = 142; continue; }
            blk = 138; continue;
        }
        138 => {
            c.w32(8, c.r16(3));
            c.cmp(c.r32(8), 0x82au64, 32);
            if c.cc_e() { blk = 141; continue; }
            blk = 139; continue;
        }
        139 => {
            let Some(t372) = c.ld32(c.r64(13).wrapping_add(0xd48)) else { return RegfileExit::fault(0x70008f3baa); };
            c.cmp(t372, c.r32(11), 32);
            if c.cc_ne() { blk = 143; continue; }
            blk = 140; continue;
        }
        140 => {
            let Some(t373) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3bb3); };
            c.w64(8, t373);
            blk = 251; continue;
        }
        141 => {
            let Some(t374) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f3bbc); };
            c.w32(6, t374);
            let r375 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r375);
            c.w64(0, c.r64(6));
            blk = 143; continue;
        }
        142 => {
            c.w64(0, c.r64(8));
            blk = 143; continue;
        }
        143 => {
            let Some(t376) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f3bca); };
            c.w32(8, t376);
            let r377 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r377);
            let Some(t378) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3bd1); };
            c.w32(8, t378);
            let r379 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r379);
            let Some(t380) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3bd8); };
            c.w32(11, t380);
            let r381 = c.sub(c.r32(11), 0x89u64, 32);
            c.w32(11, r381);
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_a() { blk = 146; continue; }
            blk = 144; continue;
        }
        144 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t382) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f3bee); };
            c.w32(6, t382);
            let r383 = c.add(c.r64(6), c.r64(14), 64);
            c.w64(6, r383);
            let Some(t384) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3bf5); };
            c.w64(8, t384);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f3bf9); }
            let Some(t385) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f3bfd); };
            if !c.push64(t385) { return RegfileExit::fault(0x70008f3bfd); }
            c.w32(1, 0);
            c.logic(0, 32);
            return c.call(0x70007f6fb4, 0x70008f3c07, 0x70008f3c02);
        }
        145 => {
            c.w64(11, c.r64(0));
            let Some(t386) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f3c0a); };
            c.w64(9, t386);
            blk = 177; continue;
        }
        146 => {
            let Some(t387) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3c13); };
            c.w64(8, t387);
            blk = 251; continue;
        }
        147 => {
            let Some(t388) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3c1c); };
            c.w64(8, t388);
            blk = 251; continue;
        }
        148 => {
            c.w64(11, c.r64(9));
            c.w32(0, c.r64(11).wrapping_add(c.r64(11)));
            let Some(t389) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3c2c); };
            c.w64(3, t389);
            if !c.st64(c.r64(3).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f3c30); }
            let Some(t390) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f3c34); };
            if !c.push64(t390) { return RegfileExit::fault(0x70008f3c34); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f3c37); }
            let Some(t391) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f3c39); };
            if !c.push64(t391) { return RegfileExit::fault(0x70008f3c39); }
            let Some(t392) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f3c3c); };
            c.w64(6, t392);
            let Some(t393) = c.ld64(c.r64(13).wrapping_add(0x23a0)) else { return RegfileExit::fault(0x70008f3c40); };
            c.w64(3, t393);
            c.w32(0, 0x3u64);
            c.w64(8, c.r64(3));
            return c.call(0x700086ed74, 0x70008f3c54, 0x70008f3c4f);
        }
        149 => {
            c.w64(11, c.r64(0));
            let Some(t394) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f3c57); };
            c.w64(9, t394);
            blk = 177; continue;
        }
        150 => {
            c.w64(8, c.r64(9));
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t395) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3c67); };
            c.w64(0, t395);
            if !c.st64(c.r64(0).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f3c6b); }
            let Some(t396) = c.ld64(c.r64(5).wrapping_sub(0x48)) else { return RegfileExit::fault(0x70008f3c6f); };
            if !c.push64(t396) { return RegfileExit::fault(0x70008f3c6f); }
            c.w64(7, c.r64(2));
            c.w32(0, 0);
            c.logic(0, 32);
            let Some(t397) = c.ld64(c.r64(5).wrapping_sub(0x50)) else { return RegfileExit::fault(0x70008f3c77); };
            c.w64(6, t397);
            return c.call(0x70007f2274, 0x70008f3c80, 0x70008f3c7b);
        }
        151 => {
            c.w64(11, c.r64(0));
            let Some(t398) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f3c83); };
            c.w64(9, t398);
            blk = 177; continue;
        }
        152 => {
            let Some(t399) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f3c8c); };
            c.w64(0, t399);
            let Some(t400) = c.ld32(c.r64(0).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3c90); };
            c.w32(8, t400);
            let r401 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r401);
            let Some(t402) = c.ld16(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3c97); };
            c.w32(3, t402);
            c.cmp(c.r16(3), 0x834u64, 16);
            if c.cc_e() { blk = 174; continue; }
            blk = 153; continue;
        }
        153 => {
            c.w32(2, c.r64(3).wrapping_sub(0x80c));
            c.cmp(c.r32(2), 0xeu64, 32);
            if c.cc_be() { blk = 163; continue; }
            blk = 154; continue;
        }
        154 => {
            c.cmp(c.r16(3), 0x411u64, 16);
            if c.cc_e() { blk = 156; continue; }
            blk = 155; continue;
        }
        155 => {
            let Some(t403) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3cb9); };
            c.w64(8, t403);
            blk = 251; continue;
        }
        156 => {
            let Some(t404) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3cc2); };
            c.w32(8, t404);
            let Some(t405) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f3cc6); };
            c.cmp(t405, c.r32(8), 32);
            if c.cc_e() { blk = 158; continue; }
            blk = 157; continue;
        }
        157 => {
            let Some(t406) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3ccf); };
            c.w64(8, t406);
            blk = 251; continue;
        }
        158 => {
            let Some(t407) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f3cd8); };
            c.w32(8, t407);
            let r408 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r408);
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 160; continue; }
            blk = 159; continue;
        }
        159 => {
            let Some(t409) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3ce5); };
            c.w64(8, t409);
            blk = 251; continue;
        }
        160 => {
            let Some(t410) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3cee); };
            c.w32(11, t410);
            let r411 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r411);
            let Some(t412) = c.ld16(c.r64(11).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3cf5); };
            c.cmp(t412, 0x40u64, 16);
            if c.cc_b() { blk = 162; continue; }
            blk = 161; continue;
        }
        161 => {
            let Some(t413) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3cfd); };
            c.w64(8, t413);
            blk = 251; continue;
        }
        162 => {
            let Some(t414) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f3d06); };
            c.w32(8, t414);
            c.w32(11, c.r64(8).wrapping_add(c.r64(8)));
            blk = 177; continue;
        }
        163 => {
            let Some(t415) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3d13); };
            c.w32(11, t415);
            let Some(t416) = c.ld32(c.r64(13).wrapping_add(0x980)) else { return RegfileExit::fault(0x70008f3d17); };
            c.cmp(t416, c.r32(11), 32);
            if c.cc_e() { blk = 165; continue; }
            blk = 164; continue;
        }
        164 => {
            let Some(t417) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3d20); };
            c.w64(8, t417);
            blk = 251; continue;
        }
        165 => {
            let Some(t418) = c.ld8(c.r64(8).wrapping_add(0x9)) else { return RegfileExit::fault(0x70008f3d29); };
            c.w32(8, t418);
            c.w64(11, c.r64(8));
            let r419 = c.r32(11) & 0xc0u64;
            c.w32(11, r419);
            c.logic(r419, 32);
            c.cmp(c.r32(11), 0xc0u64, 32);
            if c.cc_e() { blk = 168; continue; }
            blk = 166; continue;
        }
        166 => {
            let Some(t420) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f3d41); };
            c.w32(11, t420);
            let r421 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r421);
            let Some(t422) = c.ld32(c.r64(11).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f3d48); };
            c.w32(11, t422);
            let r423 = c.r32(11) & 0x1fu64;
            c.w32(11, r423);
            c.logic(r423, 32);
            let r424 = c.sub(c.r32(11), 0x10u64, 32);
            c.w32(11, r424);
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_be() { blk = 168; continue; }
            blk = 167; continue;
        }
        167 => {
            let Some(t425) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3d5a); };
            c.w64(8, t425);
            blk = 251; continue;
        }
        168 => {
            c.logic(c.r8(8) & 0x1u64, 8);
            if c.cc_ne() { blk = 173; continue; }
            blk = 169; continue;
        }
        169 => {
            let Some(t426) = c.ld32(c.r64(0).wrapping_add(0x1b)) else { return RegfileExit::fault(0x70008f3d69); };
            c.w32(11, t426);
            let r427 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r427);
            let Some(t428) = c.ld32(c.r64(13).wrapping_add(0xa8)) else { return RegfileExit::fault(0x70008f3d70); };
            c.cmp(t428, c.r32(11), 32);
            if c.cc_e() { blk = 172; continue; }
            blk = 170; continue;
        }
        170 => {
            let Some(t429) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3d79); };
            c.w32(8, t429);
            let Some(t430) = c.ld32(c.r64(13).wrapping_add(0xd0)) else { return RegfileExit::fault(0x70008f3d7d); };
            c.cmp(t430, c.r32(8), 32);
            if c.cc_ne() { blk = 177; continue; }
            blk = 171; continue;
        }
        171 => {
            let Some(t431) = c.ld32(c.r64(11).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f3d86); };
            c.w32(11, t431);
            let r432 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r432);
            blk = 177; continue;
        }
        172 => {
            let Some(t433) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3d8f); };
            c.w64(8, t433);
            blk = 251; continue;
        }
        173 => {
            let Some(t434) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3d98); };
            c.w64(8, t434);
            blk = 251; continue;
        }
        174 => {
            let Some(t435) = c.ld32(c.r64(11).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3da1); };
            c.w32(8, t435);
            let Some(t436) = c.ld32(c.r64(13).wrapping_add(0x818)) else { return RegfileExit::fault(0x70008f3da5); };
            c.cmp(t436, c.r32(8), 32);
            if c.cc_e() { blk = 176; continue; }
            blk = 175; continue;
        }
        175 => {
            let Some(t437) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3dae); };
            c.w64(8, t437);
            blk = 251; continue;
        }
        176 => {
            let Some(t438) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f3db7); };
            c.w32(11, t438);
            let r439 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r439);
            blk = 177; continue;
        }
        177 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t440) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3dc5); };
            c.w64(3, t440);
            let Some(t441) = c.ld64(c.r64(3).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3dc9); };
            c.w64(8, t441);
            let Some(t442) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f3dcd); };
            c.w32(0, t442);
            let Some(t443) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f3dd2); };
            c.w64(15, t443);
            let Some(t444) = c.ld64(c.r64(15).wrapping_add(c.r64(0).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3dd6); };
            c.w64(1, t444);
            let Some(t445) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3dda); };
            c.w64(5, t445);
            c.w64(12, c.r64(8));
            c.w64(0, c.r64(11));
            let r446 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r446);
            return RegfileExit::jump(c.r64(1));
        }
        178 => {
            let Some(t447) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3dea); };
            c.w64(8, t447);
            blk = 251; continue;
        }
        179 => {
            let r448 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r448);
            let Some(t449) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f3df7); };
            c.w64(8, t449);
            let Some(t450) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f3dfb); };
            c.w32(11, t450);
            let Some(t451) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3e00); };
            c.w64(1, t451);
            let Some(t452) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3e04); };
            c.w64(5, t452);
            c.w64(12, c.r64(8));
            let r453 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r453);
            return RegfileExit::jump(c.r64(1));
        }
        180 => {
            let r454 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r454);
            let Some(t455) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f3e15); };
            c.w32(8, t455);
            let Some(t456) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3e1a); };
            c.w64(1, t456);
            let Some(t457) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3e1e); };
            c.w64(5, t457);
            c.w64(0, c.r64(3));
            let r458 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r458);
            return RegfileExit::jump(c.r64(1));
        }
        181 => {
            c.logic(c.r8(6) & 0x40u64, 8);
            if c.cc_ne() { blk = 195; continue; }
            blk = 182; continue;
        }
        182 => {
            c.w64(11, c.r64(6));
            if let Some(sr) = c.shr(c.r64(11), 0x9u64, 64) { c.w64(11, sr); }
            c.logic(c.r16(6) & 0x100u64, 16);
            c.w8(0, (c.cc_ne() as u64));
            c.w32(0, c.r8(0));
            let r459 = c.r64(11) & 0x7ffu64;
            c.w64(11, r459);
            c.logic(r459, 64);
            c.logic(c.r8(6) & 0xffffffffffffff80u64, 8);
            if c.cc_ne() { blk = 190; continue; }
            blk = 183; continue;
        }
        183 => {
            let Some(t460) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f3e54); };
            c.w32(3, t460);
            let r461 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r461);
            c.logic(c.r8(3) & 0x1u64, 8);
            if c.cc_ne() { blk = 185; continue; }
            blk = 184; continue;
        }
        184 => {
            let Some(t462) = c.ld64(c.r64(13).wrapping_add(0x168)) else { return RegfileExit::fault(0x70008f3e5f); };
            c.w64(3, t462);
            blk = 185; continue;
        }
        185 => {
            let Some(t463) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3e66); };
            c.w32(11, t463);
            let r464 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r464);
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 187; continue; }
            blk = 186; continue;
        }
        186 => {
            let r465 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r465);
            let Some(t466) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f3e77); };
            c.w32(8, t466);
            let Some(t467) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3e7c); };
            c.w64(1, t467);
            let Some(t468) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3e80); };
            c.w64(5, t468);
            c.w64(0, c.r64(11));
            let r469 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r469);
            return RegfileExit::jump(c.r64(1));
        }
        187 => {
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 188; continue;
        }
        188 => {
            let Some(t470) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3e97); };
            c.w32(0, t470);
            let Some(t471) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f3e9b); };
            c.cmp(t471, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 189; continue;
        }
        189 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f3ea8); }
            blk = 243; continue;
        }
        190 => {
            c.cmp(c.r32(0), 0x0u64, 32);
            if c.cc_ne() { blk = 192; continue; }
            blk = 191; continue;
        }
        191 => {
            let r472 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r472);
            let Some(t473) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f3ebc); };
            c.w32(8, t473);
            let Some(t474) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3ec1); };
            c.w32(0, t474);
            let r475 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r475);
            let Some(t476) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3ec9); };
            c.w64(1, t476);
            let Some(t477) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3ecd); };
            c.w64(5, t477);
            let r478 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r478);
            return RegfileExit::jump(c.r64(1));
        }
        192 => {
            let Some(t479) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3ed7); };
            c.w32(11, t479);
            let r480 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r480);
            c.logic(c.r8(11) & 0x1u64, 8);
            if c.cc_e() { blk = 251; continue; }
            blk = 193; continue;
        }
        193 => {
            let Some(t481) = c.ld32(c.r64(11).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3ee9); };
            c.w32(0, t481);
            let Some(t482) = c.ld32(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f3eed); };
            c.cmp(t482, c.r32(0), 32);
            if c.cc_ne() { blk = 251; continue; }
            blk = 194; continue;
        }
        194 => {
            if !c.movsd_rm(0, c.r64(11).wrapping_add(0x3)) { return RegfileExit::fault(0x70008f3efa); }
            blk = 243; continue;
        }
        195 => {
            c.w64(11, c.r64(6));
            let r483 = c.shr(c.r32(11), 0x7u64, 32).unwrap_or(c.r32(11));
            c.w32(11, r483);
            let r484 = c.r32(11) & 0xfu64;
            c.w32(11, r484);
            c.logic(r484, 32);
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_l() { blk = 202; continue; }
            blk = 196; continue;
        }
        196 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_l() { blk = 200; continue; }
            blk = 197; continue;
        }
        197 => {
            c.cmp(c.r32(11), 0x7u64, 32);
            if c.cc_e() { blk = 249; continue; }
            blk = 198; continue;
        }
        198 => {
            c.cmp(c.r32(11), 0x8u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 199; continue;
        }
        199 => {
            c.cmp(c.r32(11), 0x9u64, 32);
            if c.cc_e() { blk = 248; continue; }
            blk = 200; continue;
        }
        200 => {
            c.cmp(c.r32(11), 0x5u64, 32);
            if c.cc_e() { blk = 241; continue; }
            blk = 201; continue;
        }
        201 => {
            c.cmp(c.r32(11), 0x6u64, 32);
            if c.cc_e() { blk = 242; continue; }
            blk = 202; continue;
        }
        202 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_l() { blk = 206; continue; }
            blk = 203; continue;
        }
        203 => {
            c.cmp(c.r32(11), 0x2u64, 32);
            if c.cc_e() { blk = 211; continue; }
            blk = 204; continue;
        }
        204 => {
            c.cmp(c.r32(11), 0x3u64, 32);
            if c.cc_e() { blk = 219; continue; }
            blk = 205; continue;
        }
        205 => {
            c.cmp(c.r32(11), 0x4u64, 32);
            if c.cc_e() { blk = 227; continue; }
            blk = 206; continue;
        }
        206 => {
            c.cmp(c.r32(11), 0x0u64, 32);
            if c.cc_e() { blk = 209; continue; }
            blk = 207; continue;
        }
        207 => {
            c.cmp(c.r32(11), 0x1u64, 32);
            if c.cc_e() { blk = 210; continue; }
            blk = 208; continue;
        }
        208 => {
            return RegfileExit::fault(0x70008f3f7e);
        }
        209 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r485 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r485);
            let r486 = c.r64(6) & 0xfffffu64;
            c.w64(6, r486);
            c.logic(r486, 64);
            let Some(t487) = c.ld8(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3f8e); };
            c.w32(8, (t487 as u8 as i8 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t488) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f3f98); };
            c.w32(8, t488);
            let Some(t489) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3f9d); };
            c.w64(1, t489);
            let Some(t490) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3fa1); };
            c.w64(5, t490);
            let r491 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r491);
            return RegfileExit::jump(c.r64(1));
        }
        210 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r492 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r492);
            let r493 = c.r64(6) & 0xfffffu64;
            c.w64(6, r493);
            c.logic(r493, 64);
            let Some(t494) = c.ld16(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3fba); };
            c.w32(8, (t494 as u16 as i16 as i64 as u64));
            c.w32(0, c.r64(8).wrapping_add(c.r64(8)));
            let Some(t495) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f3fc4); };
            c.w32(8, t495);
            let Some(t496) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f3fc9); };
            c.w64(1, t496);
            let Some(t497) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f3fcd); };
            c.w64(5, t497);
            let r498 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r498);
            return RegfileExit::jump(c.r64(1));
        }
        211 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r499 = c.r64(6) & 0xfffffu64;
            c.w64(6, r499);
            c.logic(r499, 64);
            let Some(t500) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f3fe2); };
            c.w32(8, t500);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f3fe7); }
            c.w64(11, c.r64(8));
            let r501 = c.add(c.r32(11), c.r32(8), 32);
            c.w32(11, r501);
            if c.cc_o() { blk = 213; continue; }
            blk = 212; continue;
        }
        212 => {
            c.w64(8, (c.r32(11) as u32 as i32 as i64 as u64));
            c.w64(0, c.r64(8));
            blk = 218; continue;
        }
        213 => {
            let Some(t502) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f3ffb); };
            c.w64(11, t502);
            let Some(t503) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f4002); };
            c.w64(0, t503);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t504) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f4009); };
            c.w64(2, t504);
            let Some(t505) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f4010); };
            c.cmp(t505, c.r64(3), 64);
            if c.cc_a() { blk = 216; continue; }
            blk = 214; continue;
        }
        214 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f401f, 0x70008f401a);
        }
        215 => {
            let Some(t506) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f401f); };
            c.w64(8, t506);
            let Some(t507) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4023); };
            c.w64(15, t507);
            let Some(t508) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4027); };
            c.w64(9, t508);
            let Some(t509) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f402b); };
            c.w64(12, t509);
            blk = 217; continue;
        }
        216 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f4031); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 217; continue;
        }
        217 => {
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r32(8) as u32 as i32 as i64));
            let Some(t510) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f4044); };
            c.w64(8, t510);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f404b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f404f); }
            blk = 218; continue;
        }
        218 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t511) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f405b); };
            c.w32(8, t511);
            let Some(t512) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4060); };
            c.w64(1, t512);
            let Some(t513) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4064); };
            c.w64(5, t513);
            let r514 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r514);
            return RegfileExit::jump(c.r64(1));
        }
        219 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r515 = c.r64(6) & 0xfffffu64;
            c.w64(6, r515);
            c.logic(r515, 64);
            let Some(t516) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4079); };
            c.w32(8, t516);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f407e); }
            c.cmp(c.r32(8), 0x3fffffffu64, 32);
            if c.cc_a() { blk = 221; continue; }
            blk = 220; continue;
        }
        220 => {
            let r517 = c.shl(c.r32(8), 1u64, 32).unwrap_or(c.r32(8));
            c.w32(8, r517);
            blk = 226; continue;
        }
        221 => {
            let Some(t518) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f4090); };
            c.w64(11, t518);
            let Some(t519) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f4097); };
            c.w64(0, t519);
            c.w64(3, c.r64(0).wrapping_add(0xc));
            let Some(t520) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f409e); };
            c.w64(2, t520);
            let Some(t521) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f40a5); };
            c.cmp(t521, c.r64(3), 64);
            if c.cc_a() { blk = 224; continue; }
            blk = 222; continue;
        }
        222 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f40b4, 0x70008f40af);
        }
        223 => {
            let Some(t522) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f40b4); };
            c.w64(8, t522);
            let Some(t523) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f40b8); };
            c.w64(15, t523);
            let Some(t524) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f40bc); };
            c.w64(9, t524);
            let Some(t525) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f40c0); };
            c.w64(12, t525);
            blk = 225; continue;
        }
        224 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f40c6); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 225; continue;
        }
        225 => {
            c.w32(10, c.r32(8));
            c.xorpd_rr(0, 0);
            c.cvtsi2sd(0, (c.r64(10) as i64));
            let Some(t526) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f40dc); };
            c.w64(8, t526);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f40e3); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f40e7); }
            c.w64(8, c.r64(0));
            blk = 226; continue;
        }
        226 => {
            c.w64(11, c.r64(9));
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t527) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f40f6); };
            c.w32(11, t527);
            let Some(t528) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f40fb); };
            c.w64(1, t528);
            let Some(t529) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f40ff); };
            c.w64(5, t529);
            c.w64(0, c.r64(8));
            let r530 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r530);
            return RegfileExit::jump(c.r64(1));
        }
        227 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r531 = c.r64(6) & 0xfffffu64;
            c.w64(6, r531);
            c.logic(r531, 64);
            let Some(t532) = c.ld64(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4117); };
            c.w64(8, t532);
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f411c); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_e() { blk = 235; continue; }
            blk = 228; continue;
        }
        228 => {
            let Some(t533) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f4126); };
            c.w64(11, t533);
            let Some(t534) = c.ld64(c.r64(11)) else { return RegfileExit::fault(0x70008f412d); };
            c.w64(0, t534);
            c.w64(3, c.r64(0).wrapping_add(0x10));
            let Some(t535) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f4134); };
            c.w64(2, t535);
            let Some(t536) = c.ld64(c.r64(2)) else { return RegfileExit::fault(0x70008f413b); };
            c.cmp(t536, c.r64(3), 64);
            if c.cc_a() { blk = 231; continue; }
            blk = 229; continue;
        }
        229 => {
            c.w32(2, 0x10u64);
            return c.call(0x70007febb4, 0x70008f414a, 0x70008f4145);
        }
        230 => {
            let Some(t537) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f414a); };
            c.w64(8, t537);
            let Some(t538) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f414e); };
            c.w64(15, t538);
            let Some(t539) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4152); };
            c.w64(9, t539);
            let Some(t540) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f4156); };
            c.w64(12, t540);
            blk = 232; continue;
        }
        231 => {
            if !c.st64(c.r64(11), c.r64(3)) { return RegfileExit::fault(0x70008f415c); }
            c.w64(11, c.r64(0).wrapping_add(0x1));
            c.w64(0, c.r64(11));
            blk = 232; continue;
        }
        232 => {
            let Some(t541) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f4166); };
            c.w64(3, t541);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(3)) { return RegfileExit::fault(0x70008f416d); }
            c.cmp(c.r64(8), 0x0u64, 64);
            if c.cc_g() { blk = 234; continue; }
            blk = 233; continue;
        }
        233 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x3u64) { return RegfileExit::fault(0x70008f4176); }
            let r542 = c.neg(c.r64(8), 64);
            c.w64(8, r542);
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008f4180); }
            blk = 240; continue;
        }
        234 => {
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x2u64) { return RegfileExit::fault(0x70008f4186); }
            if !c.st64(c.r64(0).wrapping_add(0x7), c.r64(8)) { return RegfileExit::fault(0x70008f418d); }
            blk = 240; continue;
        }
        235 => {
            let Some(t543) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f4193); };
            c.w64(8, t543);
            let Some(t544) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f419a); };
            c.w64(11, t544);
            c.w64(3, c.r64(11).wrapping_add(0x8));
            let Some(t545) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f41a1); };
            c.w64(0, t545);
            let Some(t546) = c.ld64(c.r64(0)) else { return RegfileExit::fault(0x70008f41a8); };
            c.cmp(t546, c.r64(3), 64);
            if c.cc_a() { blk = 238; continue; }
            blk = 236; continue;
        }
        236 => {
            c.w32(2, 0x8u64);
            return c.call(0x70007febb4, 0x70008f41b7, 0x70008f41b2);
        }
        237 => {
            blk = 239; continue;
        }
        238 => {
            if !c.st64(c.r64(8), c.r64(3)) { return RegfileExit::fault(0x70008f41b9); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 239; continue;
        }
        239 => {
            let Some(t547) = c.ld64(c.r64(13).wrapping_add(0x1c0)) else { return RegfileExit::fault(0x70008f41c3); };
            c.w64(8, t547);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f41ca); }
            if !c.st32(c.r64(0).wrapping_add(0x3), 0x0u64) { return RegfileExit::fault(0x70008f41ce); }
            let Some(t548) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f41d5); };
            c.w64(12, t548);
            let Some(t549) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f41d9); };
            c.w64(15, t549);
            let Some(t550) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f41dd); };
            c.w64(9, t550);
            blk = 240; continue;
        }
        240 => {
            c.w64(8, c.r64(9));
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t551) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f41e8); };
            c.w32(8, t551);
            let Some(t552) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f41ed); };
            c.w64(1, t552);
            let Some(t553) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f41f1); };
            c.w64(5, t553);
            let r554 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r554);
            return RegfileExit::jump(c.r64(1));
        }
        241 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r555 = c.r64(6) & 0xfffffu64;
            c.w64(6, r555);
            c.logic(r555, 64);
            if !c.movss_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f4206); }
            c.cvtss2sd(0, c.xmm_u32(0));
            blk = 243; continue;
        }
        242 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r556 = c.r64(6) & 0xfffffu64;
            c.w64(6, r556);
            c.logic(r556, 64);
            if !c.movsd_rm(0, c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) { return RegfileExit::fault(0x70008f421d); }
            blk = 243; continue;
        }
        243 => {
            let Some(t557) = c.ld64(c.r64(13).wrapping_add(0x2db8)) else { return RegfileExit::fault(0x70008f4223); };
            c.w64(8, t557);
            let Some(t558) = c.ld64(c.r64(8)) else { return RegfileExit::fault(0x70008f422a); };
            c.w64(11, t558);
            c.w64(0, c.r64(11).wrapping_add(0xc));
            if !c.movsd_mr(c.r64(5).wrapping_sub(0x20), 0) { return RegfileExit::fault(0x70008f4231); }
            let Some(t559) = c.ld64(c.r64(13).wrapping_add(0x2dc0)) else { return RegfileExit::fault(0x70008f4236); };
            c.w64(3, t559);
            let Some(t560) = c.ld64(c.r64(3)) else { return RegfileExit::fault(0x70008f423d); };
            c.cmp(t560, c.r64(0), 64);
            if c.cc_a() { blk = 246; continue; }
            blk = 244; continue;
        }
        244 => {
            c.w32(2, 0xcu64);
            return c.call(0x70007febf4, 0x70008f424c, 0x70008f4247);
        }
        245 => {
            blk = 247; continue;
        }
        246 => {
            if !c.st64(c.r64(8), c.r64(0)) { return RegfileExit::fault(0x70008f424e); }
            c.w64(8, c.r64(11).wrapping_add(0x1));
            c.w64(0, c.r64(8));
            blk = 247; continue;
        }
        247 => {
            let Some(t561) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4258); };
            c.w64(11, t561);
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t562) = c.ld64(c.r64(13).wrapping_add(0x140)) else { return RegfileExit::fault(0x70008f4260); };
            c.w64(8, t562);
            if !c.st32(c.r64(0).wrapping_sub(0x1), c.r32(8)) { return RegfileExit::fault(0x70008f4267); }
            if !c.movsd_rm(0, c.r64(5).wrapping_sub(0x20)) { return RegfileExit::fault(0x70008f426b); }
            if !c.movsd_mr(c.r64(0).wrapping_add(0x3), 0) { return RegfileExit::fault(0x70008f4270); }
            let Some(t563) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f4275); };
            c.w64(12, t563);
            let Some(t564) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f4279); };
            c.w32(8, t564);
            let Some(t565) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f427e); };
            c.w64(15, t565);
            let Some(t566) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4282); };
            c.w64(1, t566);
            let Some(t567) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4286); };
            c.w64(5, t567);
            let r568 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r568);
            return RegfileExit::jump(c.r64(1));
        }
        248 => {
            if let Some(sr) = c.shr(c.r64(6), 0xbu64, 64) { c.w64(6, sr); }
            let r569 = c.add(c.r64(9), 0x3u64, 64);
            c.w64(9, r569);
            let r570 = c.r64(6) & 0xfffffu64;
            c.w64(6, r570);
            c.logic(r570, 64);
            let Some(t571) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f429f); };
            c.w32(8, t571);
            let Some(t572) = c.ld32(c.r64(3).wrapping_add(c.r64(6)).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f42a4); };
            c.w32(0, t572);
            let r573 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r573);
            let Some(t574) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f42ab); };
            c.w64(1, t574);
            let Some(t575) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f42af); };
            c.w64(5, t575);
            let r576 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r576);
            return RegfileExit::jump(c.r64(1));
        }
        249 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f42bd); }
            let Some(t577) = c.ld64(c.r64(13).wrapping_add(0x1040)) else { return RegfileExit::fault(0x70008f42c1); };
            c.w64(8, t577);
            let Some(t578) = c.ld32(c.r64(8).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f42c8); };
            c.w32(8, t578);
            let r579 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r579);
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f42cf); }
            let Some(t580) = c.ld64(c.r64(13).wrapping_add(0x28d8)) else { return RegfileExit::fault(0x70008f42d1); };
            c.w64(3, t580);
            c.w32(0, 0x1u64);
            c.w32(6, 0);
            c.logic(0, 32);
            return c.call(0x700086ed74, 0x70008f42e4, 0x70008f42df);
        }
        250 => {
            return RegfileExit::fault(0x70008f42e4);
        }
        251 => {
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(0, c.r64(9).wrapping_add(0x3));
            let Some(t581) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f42ed); };
            c.w32(3, t581);
            let Some(t582) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f42f3); };
            c.w32(2, t582);
            let r583 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r583);
            let Some(t584) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f42fb); };
            c.w64(1, t584);
            let Some(t585) = c.ld32(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f42ff); };
            c.w32(3, t585);
            let r586 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r586);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f4306); }
            if !c.push64(c.r64(3)) { return RegfileExit::fault(0x70008f430a); }
            let Some(t587) = c.ld64(c.r64(5).wrapping_sub(0x40)) else { return RegfileExit::fault(0x70008f430b); };
            if !c.push64(t587) { return RegfileExit::fault(0x70008f430b); }
            let Some(t588) = c.ld64(c.r64(5).wrapping_sub(0x38)) else { return RegfileExit::fault(0x70008f430e); };
            if !c.push64(t588) { return RegfileExit::fault(0x70008f430e); }
            c.w32(11, 0);
            c.logic(0, 32);
            if !c.push64(c.r64(11)) { return RegfileExit::fault(0x70008f4314); }
            if !c.st64(c.r64(5).wrapping_sub(0x30), c.r64(0)) { return RegfileExit::fault(0x70008f4316); }
            let Some(t589) = c.ld64(c.r64(13).wrapping_add(0x2040)) else { return RegfileExit::fault(0x70008f431a); };
            c.w64(3, t589);
            c.w64(6, c.r64(1));
            c.w32(0, 0x4u64);
            c.w64(11, c.r64(0));
            return c.call(0x700086ed74, 0x70008f4331, 0x70008f432c);
        }
        252 => {
            let Some(t590) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4331); };
            c.w64(8, t590);
            let Some(t591) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4335); };
            c.w64(8, t591);
            let Some(t592) = c.ld64(c.r64(5).wrapping_sub(0x30)) else { return RegfileExit::fault(0x70008f4339); };
            c.w64(9, t592);
            let Some(t593) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f433d); };
            c.w32(11, t593);
            let Some(t594) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4342); };
            c.w64(15, t594);
            let Some(t595) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4346); };
            c.w64(1, t595);
            let Some(t596) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f434a); };
            c.w64(5, t596);
            c.w64(12, c.r64(8));
            let r597 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r597);
            return RegfileExit::jump(c.r64(1));
        }
        253 => {
            return RegfileExit::fault(0x70008f4357);
        }
        254 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f4358); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f435c); }
            let r598 = c.sub(c.r64(4), 0x50u64, 64);
            c.w64(4, r598);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t599) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f4366); };
            c.w32(0, t599);
            let Some(t600) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f436c); };
            c.w32(3, t600);
            let r601 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r601);
            let Some(t602) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f4374); };
            c.w64(2, t602);
            let Some(t603) = c.ld32(c.r64(3).wrapping_add(c.r64(0).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4378); };
            c.w32(1, t603);
            let r604 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r604);
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f437f); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f4383); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f4387); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f438b); }
            c.w32(0, 0xeu64);
            c.w64(6, c.r64(2));
            return c.call(0x7000804df4, 0x70008f439c, 0x70008f4397);
        }
        255 => {
            let Some(t605) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f439c); };
            c.w64(8, t605);
            let Some(t606) = c.ld64(c.r64(8).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f43a0); };
            c.w64(8, t606);
            let Some(t607) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f43a4); };
            c.w64(11, t607);
            c.w64(9, c.r64(11).wrapping_add(0x3));
            let Some(t608) = c.ld8(c.r64(9).wrapping_add(c.r64(8))) else { return RegfileExit::fault(0x70008f43ac); };
            c.w32(11, t608);
            let Some(t609) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f43b1); };
            c.w64(15, t609);
            let Some(t610) = c.ld64(c.r64(15).wrapping_add(c.r64(11).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f43b5); };
            c.w64(1, t610);
            let Some(t611) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f43b9); };
            c.w64(5, t611);
            c.w64(12, c.r64(8));
            let r612 = c.add(c.r64(4), 0x60u64, 64);
            c.w64(4, r612);
            return RegfileExit::jump(c.r64(1));
        }
        256 => {
            return RegfileExit::jump(0x70008f43c8);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f43d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f43d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f43d8); }
            let r0 = c.sub(c.r64(4), 0x20u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f43de); };
            c.w64(2, t1);
            let Some(t2) = c.ld64(c.r64(2).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f43e2); };
            c.w64(8, t2);
            let Some(t3) = c.ld32(c.r64(8).wrapping_add(0x13)) else { return RegfileExit::fault(0x70008f43e6); };
            c.w32(8, t3);
            let r4 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r4);
            let Some(t5) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f43ed); };
            c.w32(11, t5);
            let Some(t6) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f43f3); };
            c.w32(3, t6);
            let r7 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r7);
            let Some(t8) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f43fb); };
            c.w64(1, t8);
            let Some(t9) = c.ld32(c.r64(8).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f43ff); };
            c.w32(8, t9);
            let r10 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r10);
            let Some(t11) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f4406); };
            c.w32(6, t11);
            let Some(t12) = c.ld32(c.r64(3).wrapping_add(c.r64(11).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f440c); };
            c.w32(11, t12);
            let r13 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r13);
            let Some(t14) = c.ld32(c.r64(8).wrapping_sub(0x1)) else { return RegfileExit::fault(0x70008f4414); };
            c.w32(3, t14);
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(0)) { return RegfileExit::fault(0x70008f4418); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(15)) { return RegfileExit::fault(0x70008f441c); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(9)) { return RegfileExit::fault(0x70008f4420); }
            if !c.st64(c.r64(5).wrapping_sub(0x28), c.r64(2)) { return RegfileExit::fault(0x70008f4424); }
            let Some(t15) = c.ld32(c.r64(13).wrapping_add(0x158)) else { return RegfileExit::fault(0x70008f4428); };
            c.cmp(t15, c.r32(3), 32);
            if c.cc_e() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t16) = c.ld64(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f4431); };
            c.w64(8, t16);
            blk = 2; continue;
        }
        2 => {
            c.w32(3, c.r64(9).wrapping_add(c.r64(9)));
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            c.w64(7, c.r64(6).wrapping_add(c.r64(6)));
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(3)) { return RegfileExit::fault(0x70008f4443); }
            c.w64(6, c.r64(1));
            c.w64(1, c.r64(11));
            c.w64(3, c.r64(8));
            return c.call(0x700083bef4, 0x70008f4455, 0x70008f4450);
        }
        3 => {
            let Some(t17) = c.ld64(c.r64(5).wrapping_sub(0x28)) else { return RegfileExit::fault(0x70008f4455); };
            c.w64(2, t17);
            let Some(t18) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4459); };
            c.w64(2, t18);
            let Some(t19) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f445d); };
            c.w64(8, t19);
            c.w64(9, c.r64(8).wrapping_add(0x3));
            let Some(t20) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f4465); };
            c.w32(8, t20);
            let Some(t21) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f446a); };
            c.w64(15, t21);
            let Some(t22) = c.ld64(c.r64(15).wrapping_add(c.r64(8).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f446e); };
            c.w64(1, t22);
            let Some(t23) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4472); };
            c.w64(5, t23);
            c.w64(12, c.r64(2));
            let Some(t24) = c.ld64(c.r64(4).wrapping_add(0x18)) else { return RegfileExit::fault(0x70008f4479); };
            c.w64(0, t24);
            let r25 = c.add(c.r64(4), 0x30u64, 64);
            c.w64(4, r25);
            return RegfileExit::jump(c.r64(1));
        }
        4 => {
            return RegfileExit::jump(0x70008f4488);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f4494(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            let Some(t0) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f4494); };
            c.w64(3, (t0 as u8 as i8 as i64 as u64));
            c.w64(2, c.r64(5));
            let Some(t1) = c.ld64(c.r64(2).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f449d); };
            c.w64(3, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f44a1); };
            c.w32(2, t2);
            let Some(t3) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f44a7); };
            c.w32(6, t3);
            c.cmp(c.r8(6), 0x0u64, 8);
            if c.cc_e() { blk = 5; continue; }
            blk = 1; continue;
        }
        1 => {
            let Some(t4) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f44b3); };
            c.w32(7, t4);
            c.w64(7, (c.r32(7) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(7), 1u64, 64) { c.w64(7, sr); }
            c.cmp(c.r64(7), 0x1u64, 64);
            if c.cc_be() { blk = 4; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t5) = c.ld32(c.r64(3).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f44c2); };
            c.w32(3, t5);
            let r6 = c.add(c.r64(3), c.r64(14), 64);
            c.w64(3, r6);
            let r7 = c.add(c.r32(6), 0xffffffffffffffffu64, 32);
            c.w32(6, r7);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            blk = 1; continue;
        }
        4 => {
            return RegfileExit::fault(0x70008f44cf);
        }
        5 => {
            let Some(t8) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f44d0); };
            c.w32(6, t8);
            c.w64(6, (c.r32(6) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(6), 1u64, 64) { c.w64(6, sr); }
            c.cmp(c.r64(2), c.r64(6), 64);
            if c.cc_b() { blk = 7; continue; }
            blk = 6; continue;
        }
        6 => {
            return RegfileExit::fault(0x70008f44de);
        }
        7 => {
            c.w64(2, c.r64(2).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)), c.r32(0)) { return RegfileExit::fault(0x70008f44e7); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 8; continue;
        }
        8 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r9 = c.r64(6) & c.r64(3);
            c.w64(6, r9);
            c.logic(r9, 64);
            let Some(t10) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008f44f8); };
            c.logic(t10 & 0x4u64, 8);
            if c.cc_ne() { blk = 10; continue; }
            blk = 9; continue;
        }
        9 => {
            let r11 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r11);
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f4502); };
            c.w32(3, t12);
            let Some(t13) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4507); };
            c.w64(1, t13);
            return RegfileExit::jump(c.r64(1));
        }
        10 => {
            c.w32(0, c.r32(0));
            let r14 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r14);
            c.w64(6, 0xfffffffffffc0000u64);
            let r15 = c.r64(6) & c.r64(0);
            c.w64(6, r15);
            c.logic(r15, 64);
            let Some(t16) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008f451c); };
            c.logic(t16 & 0x2u64, 8);
            if c.cc_e() { blk = 9; continue; }
            blk = 11; continue;
        }
        11 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008f4526); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f4527); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f1594, 0x70008f4534, 0x70008f452f);
        }
        12 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008f4534); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008f4536); }
            blk = 9; continue;
        }
        13 => {
            return RegfileExit::jump(0x70008f453c);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f4554(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(3, c.r64(5));
            let Some(t0) = c.ld64(c.r64(3).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f4557); };
            c.w64(3, t0);
            let Some(t1) = c.ld32(c.r64(3).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f455b); };
            c.w32(2, t1);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f4561); };
            c.w32(6, t2);
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(6), c.r64(2), 64);
            if c.cc_b() { blk = 2; continue; }
            blk = 1; continue;
        }
        1 => {
            return RegfileExit::fault(0x70008f456f);
        }
        2 => {
            c.w64(2, c.r64(6).wrapping_mul(4).wrapping_add(0x7));
            if !c.st32(c.r64(3).wrapping_add(c.r64(2)), c.r32(0)) { return RegfileExit::fault(0x70008f4578); }
            c.logic(c.r8(0) & 0x1u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 3; continue;
        }
        3 => {
            c.w64(6, 0xfffffffffffc0000u64);
            let r3 = c.r64(6) & c.r64(3);
            c.w64(6, r3);
            c.logic(r3, 64);
            let Some(t4) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008f4589); };
            c.logic(t4 & 0x4u64, 8);
            if c.cc_ne() { blk = 5; continue; }
            blk = 4; continue;
        }
        4 => {
            let r5 = c.add(c.r64(9), 0x2u64, 64);
            c.w64(9, r5);
            let Some(t6) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f4593); };
            c.w32(3, t6);
            let Some(t7) = c.ld64(c.r64(15).wrapping_add(c.r64(3).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4598); };
            c.w64(1, t7);
            return RegfileExit::jump(c.r64(1));
        }
        5 => {
            c.w32(0, c.r32(0));
            let r8 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r8);
            c.w64(6, 0xfffffffffffc0000u64);
            let r9 = c.r64(6) & c.r64(0);
            c.w64(6, r9);
            c.logic(r9, 64);
            let Some(t10) = c.ld8(c.r64(6).wrapping_add(0x8)) else { return RegfileExit::fault(0x70008f45ad); };
            c.logic(t10 & 0x2u64, 8);
            if c.cc_e() { blk = 4; continue; }
            blk = 6; continue;
        }
        6 => {
            c.w64(7, c.r64(3).wrapping_add(c.r64(2)));
            if !c.push64(c.r64(7)) { return RegfileExit::fault(0x70008f45b7); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f45b8); }
            c.w64(8, c.r64(7));
            c.w64(7, c.r64(3));
            return c.call(0x70007f1594, 0x70008f45c5, 0x70008f45c0);
        }
        7 => {
            if !c.pop64_reg(8) { return RegfileExit::fault(0x70008f45c5); }
            if !c.pop64_reg(7) { return RegfileExit::fault(0x70008f45c7); }
            blk = 4; continue;
        }
        8 => {
            return RegfileExit::jump(0x70008f45cc);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f45d4(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f45d4); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f45d8); }
            let r0 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f45de); };
            c.w32(2, t1);
            let Some(t2) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f45e4); };
            c.w32(7, t2);
            let r3 = c.add(c.r64(7), c.r64(14), 64);
            c.w64(7, r3);
            let Some(t4) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f45ec); };
            c.w64(8, t4);
            let Some(t5) = c.ld32(c.r64(7).wrapping_add(c.r64(2).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f45f0); };
            c.w32(2, t5);
            let r6 = c.add(c.r64(2), c.r64(14), 64);
            c.w64(2, r6);
            let Some(t7) = c.ld64(c.r64(8).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f45f7); };
            c.w64(7, t7);
            c.w32(11, c.r64(9).wrapping_add(c.r64(9)));
            if !c.st64(c.r64(8).wrapping_sub(0x28), c.r64(11)) { return RegfileExit::fault(0x70008f45ff); }
            if !c.push64(c.r64(2)) { return RegfileExit::fault(0x70008f4603); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f4604); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f4608); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(8)) { return RegfileExit::fault(0x70008f460c); }
            let Some(t8) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x70008f4610); };
            c.w64(3, t8);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008f4624, 0x70008f461f);
        }
        1 => {
            let Some(t9) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4624); };
            c.w64(2, t9);
            let Some(t10) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f4628); };
            c.w64(2, t10);
            let Some(t11) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f462c); };
            c.w64(7, t11);
            c.w64(9, c.r64(7).wrapping_add(0x2));
            let Some(t12) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f4634); };
            c.w32(7, t12);
            let Some(t13) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f4639); };
            c.w64(15, t13);
            let Some(t14) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f463d); };
            c.w64(1, t14);
            let Some(t15) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4641); };
            c.w64(5, t15);
            c.w64(12, c.r64(2));
            let r16 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r16);
            return RegfileExit::jump(c.r64(1));
        }
        2 => {
            return RegfileExit::jump(0x70008f4650);
        }
            _ => unreachable!(),
        }
    }
}

pub unsafe extern "C" fn rf_70008f4654(raw: *mut u8, mut blk: u32) -> RegfileExit {
    let mut c = ctx(raw);
    loop {
        match blk {
        0 => {
            c.w64(2, c.r64(5));
            let Some(t0) = c.ld64(c.r64(2).wrapping_sub(0x8)) else { return RegfileExit::fault(0x70008f4657); };
            c.w64(7, t0);
            let Some(t1) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x2)) else { return RegfileExit::fault(0x70008f465b); };
            c.w32(8, t1);
            let Some(t2) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f4661); };
            c.w32(11, t2);
            c.w64(0, c.r64(7));
            blk = 1; continue;
        }
        1 => {
            let Some(t3) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f466a); };
            c.w32(3, t3);
            c.w64(3, (c.r32(3) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(3), 1u64, 64) { c.w64(3, sr); }
            c.cmp(c.r64(3), 0x0u64, 64);
            if c.cc_be() { blk = 8; continue; }
            blk = 2; continue;
        }
        2 => {
            let Some(t4) = c.ld32(c.r64(0).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f4679); };
            c.w32(1, t4);
            let r5 = c.add(c.r64(1), c.r64(14), 64);
            c.w64(1, r5);
            let Some(t6) = c.ld32(c.r64(1).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f467f); };
            c.w32(1, t6);
            c.w64(1, (c.r32(1) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(1), 1u64, 64) { c.w64(1, sr); }
            c.logic(c.r64(1) & 0x4000000u64, 64);
            if c.cc_e() { blk = 5; continue; }
            blk = 3; continue;
        }
        3 => {
            c.cmp(c.r64(3), 0x2u64, 64);
            if c.cc_be() { blk = 9; continue; }
            blk = 4; continue;
        }
        4 => {
            let Some(t7) = c.ld32(c.r64(0).wrapping_add(0xf)) else { return RegfileExit::fault(0x70008f4697); };
            c.w32(1, t7);
            let Some(t8) = c.ld32(c.r64(13).wrapping_add(0xa0)) else { return RegfileExit::fault(0x70008f469a); };
            c.cmp(t8, c.r32(1), 32);
            if c.cc_ne() { blk = 14; continue; }
            blk = 5; continue;
        }
        5 => {
            c.cmp(c.r64(3), 0x1u64, 64);
            if c.cc_be() { blk = 10; continue; }
            blk = 6; continue;
        }
        6 => {
            let Some(t9) = c.ld32(c.r64(0).wrapping_add(0xb)) else { return RegfileExit::fault(0x70008f46a9); };
            c.w32(0, t9);
            let r10 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r10);
            let r11 = c.add(c.r32(11), 0xffffffffffffffffu64, 32);
            c.w32(11, r11);
            if c.cc_ne() { blk = 1; continue; }
            blk = 7; continue;
        }
        7 => {
            blk = 11; continue;
        }
        8 => {
            return RegfileExit::fault(0x70008f46b7);
        }
        9 => {
            return RegfileExit::fault(0x70008f46b8);
        }
        10 => {
            return RegfileExit::fault(0x70008f46b9);
        }
        11 => {
            let Some(t12) = c.ld32(c.r64(0).wrapping_add(0x3)) else { return RegfileExit::fault(0x70008f46ba); };
            c.w32(2, t12);
            c.w64(2, (c.r32(2) as u32 as i32 as i64 as u64));
            if let Some(sr) = c.sar(c.r64(2), 1u64, 64) { c.w64(2, sr); }
            c.cmp(c.r64(8), c.r64(2), 64);
            if c.cc_b() { blk = 13; continue; }
            blk = 12; continue;
        }
        12 => {
            return RegfileExit::fault(0x70008f46c8);
        }
        13 => {
            let r13 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r13);
            let Some(t14) = c.ld8(c.r64(9).wrapping_add(c.r64(12))) else { return RegfileExit::fault(0x70008f46cd); };
            c.w32(2, t14);
            let Some(t15) = c.ld32(c.r64(0).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f46d2); };
            c.w32(0, t15);
            let r16 = c.add(c.r64(0), c.r64(14), 64);
            c.w64(0, r16);
            let Some(t17) = c.ld64(c.r64(15).wrapping_add(c.r64(2).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f46da); };
            c.w64(1, t17);
            return RegfileExit::jump(c.r64(1));
        }
        14 => {
            if !c.push64(c.r64(5)) { return RegfileExit::fault(0x70008f46e0); }
            c.w64(5, c.r64(4));
            if !c.push64(0x1cu64) { return RegfileExit::fault(0x70008f46e4); }
            let r18 = c.sub(c.r64(4), 0x18u64, 64);
            c.w64(4, r18);
            let Some(t19) = c.ld8(c.r64(12).wrapping_add(c.r64(9)).wrapping_add(0x1)) else { return RegfileExit::fault(0x70008f46ea); };
            c.w32(8, t19);
            let Some(t20) = c.ld32(c.r64(12).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f46f0); };
            c.w32(11, t20);
            let r21 = c.add(c.r64(11), c.r64(14), 64);
            c.w64(11, r21);
            c.w32(0, c.r64(9).wrapping_add(c.r64(9)));
            let Some(t22) = c.ld32(c.r64(11).wrapping_add(c.r64(8).wrapping_mul(4)).wrapping_add(0x7)) else { return RegfileExit::fault(0x70008f46fc); };
            c.w32(8, t22);
            let r23 = c.add(c.r64(8), c.r64(14), 64);
            c.w64(8, r23);
            if !c.st64(c.r64(2).wrapping_sub(0x28), c.r64(0)) { return RegfileExit::fault(0x70008f4704); }
            if !c.push64(c.r64(8)) { return RegfileExit::fault(0x70008f4708); }
            if !c.st64(c.r64(5).wrapping_sub(0x10), c.r64(15)) { return RegfileExit::fault(0x70008f470a); }
            if !c.st64(c.r64(5).wrapping_sub(0x18), c.r64(9)) { return RegfileExit::fault(0x70008f470e); }
            if !c.st64(c.r64(5).wrapping_sub(0x20), c.r64(2)) { return RegfileExit::fault(0x70008f4712); }
            let Some(t24) = c.ld64(c.r64(13).wrapping_add(0x26c8)) else { return RegfileExit::fault(0x70008f4716); };
            c.w64(3, t24);
            c.w32(0, 0x1u64);
            c.w64(6, c.r64(7));
            return c.call(0x700086ed74, 0x70008f472a, 0x70008f4725);
        }
        15 => {
            let Some(t25) = c.ld64(c.r64(5).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f472a); };
            c.w64(2, t25);
            let Some(t26) = c.ld64(c.r64(2).wrapping_sub(0x20)) else { return RegfileExit::fault(0x70008f472e); };
            c.w64(2, t26);
            let Some(t27) = c.ld64(c.r64(5).wrapping_sub(0x18)) else { return RegfileExit::fault(0x70008f4732); };
            c.w64(9, t27);
            let r28 = c.add(c.r64(9), 0x4u64, 64);
            c.w64(9, r28);
            let Some(t29) = c.ld8(c.r64(9).wrapping_add(c.r64(2))) else { return RegfileExit::fault(0x70008f473a); };
            c.w32(7, t29);
            let Some(t30) = c.ld64(c.r64(5).wrapping_sub(0x10)) else { return RegfileExit::fault(0x70008f473f); };
            c.w64(15, t30);
            let Some(t31) = c.ld64(c.r64(15).wrapping_add(c.r64(7).wrapping_mul(8))) else { return RegfileExit::fault(0x70008f4743); };
            c.w64(1, t31);
            let Some(t32) = c.ld64(c.r64(5)) else { return RegfileExit::fault(0x70008f4747); };
            c.w64(5, t32);
            c.w64(12, c.r64(2));
            let r33 = c.add(c.r64(4), 0x28u64, 64);
            c.w64(4, r33);
            return RegfileExit::jump(c.r64(1));
        }
        16 => {
            return RegfileExit::jump(0x70008f4758);
        }
            _ => unreachable!(),
        }
    }
}
